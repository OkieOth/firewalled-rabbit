# TL;DR;
This project creates an rabbitmq contained docker image. In addition
it exposes a small webserver that allows to block and unblock port
5672. 

The aim for a risky thing like this, is to be used in integration test
scenarios.

**Don't use that image in a production environment!!!**

# Additional Commands

```bash
# start rabbitmq
docker run --rm -p 5672:5672 -p 15672:15672 rabbitmq:3.11.19-management-alpine
```

