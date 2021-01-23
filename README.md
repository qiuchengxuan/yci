YAML-based Configuration Interface
==================================

This project aims at describe any configuration as OpenAPI specs.
therefore we can easily convert between text-based configuration and RESTFUL API call.

e.g. we have an schema like this:

```yaml
openapi: '3.0.3'
info:
  title: Describe system schema
  version: 0.1.0
components:
  schemas:
    System:
      type: object
      properties:
        hostname:
          type: string
        timezone:
          type: string
      required:
        - hostname
        - timezone
paths:
  /system:
    get:
      tags:
        - config
      summary: Get system config
      responses:
        '200':
          description: Success
          content:
            application/json:
              schema:
                type: object
                additionalProperties:
                  $ref: '#/components/schemas/System'
  /system:
    put:
      summary: Replace system config
      tags:
        - config
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/System'
      responses:
        '200':
          description: Success
```

Then we got a YAML config like this:

```yaml
system:
  hostname: localhost
  timezone: Asia/Shanghai
```

The rule applied to it as following:
* top level is PUT method url
* replace top level key space to slash
* underlying attributes are request json body

So we got a RESTFUL API call like this:

* method: PUT
* url: /system
* body:

  ```json
  {
    "hostname": "localhost",
    "timezone": "Asia/Shanghai"
  }
  ```

Objective
---------

* YAML-based configuration schema validation
* Convert between YAML config and REST data
