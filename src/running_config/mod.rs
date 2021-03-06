use std::convert::TryFrom;

use hyper::body::HttpBody;
#[cfg(unix_socket)]
use hyperlocal::{UnixClientExt, Uri};
#[cfg(validate)]
use oas3::validation::ValidationTree;
use oas3::Spec;
use serde_yaml::{Mapping, Value};

const JSON: &str = "application/json";

pub struct ValidatedSpec(Spec);

impl TryFrom<Spec> for ValidatedSpec {
    type Error = ();

    fn try_from(spec: Spec) -> Result<ValidatedSpec, Self::Error> {
        // TODO: implement
        Ok(Self(spec))
    }
}

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    HTTP(http::StatusCode, String),
    UTF8(std::str::Utf8Error),
    JSON(serde_json::Error),
    Yaml(serde_yaml::Error),
    Validation(oas3::validation::Error),
}

fn transform_mapping(path: &str, mapping: Mapping) -> Mapping {
    let prefix = path.trim_matches('/').trim_end_matches('s').replace('/', " ");
    let mut new_mapping = Mapping::new();
    for (key, value) in mapping.into_iter() {
        let new_key = match key {
            Value::String(s) => Value::String(format!("{} {}", prefix, s)),
            Value::Number(n) => Value::String(format!("{} {}", prefix, n)),
            _ => key,
        };
        new_mapping.insert(new_key, value);
    }
    new_mapping
}

pub struct RunningConfig<'a> {
    server: &'a str,
    specs: Vec<&'a Spec>,
    path: String,
    #[cfg(unix_socket)]
    socket: String,
}

impl<'a> RunningConfig<'a> {
    pub fn new(server: &'a str, specs: &'a [ValidatedSpec]) -> Self {
        Self { server, specs: specs.iter().map(|spec| &spec.0).collect(), path: "".to_owned() }
    }

    pub fn set_path_prefix(&mut self, path: String) {
        self.path = path
    }

    async fn request(&self, path: &str) -> Result<String, Error> {
        let url = format!("{}{}", self.server, path);
        let (client, uri) = match () {
            #[cfg(not(unix_socket))]
            _ => (hyper::Client::new(), url.parse().unwrap()),
            #[cfg(unix_socket)]
            _ => (hyper::Client::unix(), Uri::new(self.socket, url)),
        };
        let mut response = client.get(uri).await.map_err(|e| Error::Hyper(e))?;
        let status = response.status();
        let mut body = String::new();
        while let Some(chunk) = response.body_mut().data().await {
            let bytes = chunk.map_err(|e| Error::Hyper(e))?;
            let string = std::str::from_utf8(&bytes[..]).map_err(|e| Error::UTF8(e))?;
            body.push_str(string);
        }
        if !status.is_success() {
            return Err(Error::HTTP(status, body));
        }
        Ok(body)
    }

    pub async fn get(&self) -> Result<String, Error> {
        let mut docs = Vec::new();
        for &spec in self.specs.iter() {
            for (path, entry) in spec.paths.iter() {
                let get = match entry.get {
                    Some(ref method) => method,
                    None => continue,
                };
                if get.parameters.len() > 0 {
                    continue;
                }
                if get.tags.iter().find(|tag| tag.as_str() == "config").is_none() {
                    continue;
                }
                if !path.starts_with(&self.path) {
                    continue;
                }
                let schema = get.responses(&spec)["200"].content[JSON].schema(&spec).unwrap();
                let body = self.request(path).await?;
                #[cfg(validate)]
                {
                    let validator = ValidationTree::from_schema(&schema, spec).unwrap();
                    let json = serde_json::from_str(&body).map_err(|e| Error::JSON(e))?;
                    validator.validate(&json).map_err(|e| Error::Validation(e))?;
                }
                let mut yaml = serde_json::from_str::<Value>(&body).map_err(|e| Error::JSON(e))?;
                let plural = schema.additional_properties.is_some() || schema.items.is_some();
                if plural {
                    match yaml {
                        Value::Mapping(mapping) => {
                            yaml = Value::Mapping(transform_mapping(path, mapping))
                        }
                        _ => (),
                    }
                } else {
                    let mut new_mapping = Mapping::new();
                    let key = Value::String(path.trim_matches('/').replace('/', " "));
                    new_mapping.insert(key, yaml);
                    yaml = Value::Mapping(new_mapping);
                }
                docs.push(serde_yaml::to_string(&yaml).map_err(|e| Error::Yaml(e))?);
            }
        }
        Ok(docs.join("\n"))
    }
}

#[cfg(test)]
mod test;
