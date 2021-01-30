use std::convert::TryFrom;

use mockito::mock;
use serde_json::json;

use super::{RunningConfig, ValidatedSpec, JSON};

fn yaml(expected: &str) -> String {
    let first_line = expected.trim_start_matches('\n').lines().next().unwrap();
    let spaces = first_line.split(|c| c != ' ').next().unwrap();
    expected.replace(spaces, "").trim().to_owned()
}

#[test]
fn test_running_config() {
    let system = oas3::from_path("schema/system.yaml").unwrap();
    let service = oas3::from_path("schema/service.yaml").unwrap();
    let validated_specs =
        [ValidatedSpec::try_from(system).unwrap(), ValidatedSpec::try_from(service).unwrap()];
    let server = mockito::server_url();
    let running_config = RunningConfig::new(&server, &validated_specs[..]);
    let body = json!({"hostname": "UT", "timezone": "Asia/Shanghai"});
    let _m = mock("GET", "/system")
        .with_status(200)
        .with_header("content-type", JSON)
        .with_body(body.to_string())
        .create();
    let body = json!({
        "networking": {"enable": true},
        "rsyslog": {"enable": false}
    });
    let _n = mock("GET", "/services")
        .with_status(200)
        .with_header("content-type", JSON)
        .with_body(body.to_string())
        .create();
    let actual = tokio_test::block_on(running_config.get()).unwrap();
    let expected = yaml(
        r#"
        ---
        system:
          hostname: UT
          timezone: Asia/Shanghai
        ---
        service networking:
          enable: true
        service rsyslog:
          enable: false"#,
    );
    assert_eq!(expected.trim(), actual)
}
