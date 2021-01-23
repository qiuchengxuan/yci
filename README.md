YAML-based Configuration Interface
==================================

This project aims at describe any configuration as OpenAPI compactible YAML,
therefore we can easily convert between text-based configuration and RESTFUL API call.

e.g. we have an openvpn schema like this:

```yaml
openapi: '3.0.3'
info:
  title: Describe OpenVPN tunnel schema
  version: 0.1.0
components:
  schemas:
    OpenVPN:
      type: object
      properties:
        source:
          type: string
          format: ipv4
        destination:
          type: string
          format: ipv4
        address:
          type: string
          format: ipv4
        remote:
          type: string
          format: ipv4
        mode:
          type: string
          enum: [site-to-site, client, server]
      required:
        - souce
        - destination
        - address
        - remote
        - mode
paths:
  /interface/openvpn/{name}:
    put:
      summary: Create openvpn instance
      parameters:
        - in: path
          name: name
          schema:
            type: string
          required: true
          description: OpenVPN instance name
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/OpenVPN'
      responses:
        '200':
          description: Success
```

Then we got a YAML config like this:

```yaml
interface tun0:
  type: openvpn
  destination: 8.8.8.8
  local: 10.0.0.1
  remote: 10.0.0.7
  mode: site-to-site
```

The rule applied to it as following:
* top level is PUT method url
* replace top level key space to slash
* underlying attributes are request json body

So we got a RESTFUL API call like this:

* method: PUT
* url: /interface/tun0
* body:

  ```json
  {
    "type": "openvpn",
    "destination": "8.8.8.8",
    "local": "10.0.0.1",
    "remote": "10.0.0.7",
    "mode": "site-to-site"
  }
  ```
