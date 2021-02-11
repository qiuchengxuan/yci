YAML-based Configuration Interface
==================================

This project aims at describe any configuration as OpenAPI specs.
therefore we can easily convert between text-based configuration and RESTFUL API call.

Introduction
------------

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
version: 0.1
format: standard
date: 2021-02-11
---
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
  
Standard and compact format
---------------------------

e.g. we have a configuration like this:

```yaml
version: 0.1
format: standard
date: 2021-02-11
---
interfaces eth1:
  address: 192.168.1.2/24
# a lot of configuration ...
interfaces eth1 ip ospf:
  cost: 100
```

It's not that convenient to lookup ospf configuration
with a lot configuration between then, and `interfaces eth1` looks weired, 
so a compact format would be more friendly to read like this:

```yaml
version: 0.1
format: compact
date: 2021-02-11
---
interface eth1:
  address: 192.168.1.2/24
  ip ospf:
    cost: 100
# ...
```

But we no longer able to treat a whole top level entry as a valid
put method, and we have no idea whether it's attribute is part of 
it's schema, so a compact configuration must be restored to standard 
configuration with schemas involved, or call put methods with aid of 
schemas.
  
Objective
---------

* YAML-based configuration schema validation
* Convert between YAML config and REST data
