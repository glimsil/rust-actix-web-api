# rust-actix-web-api
This is an example of rest api using Rust + actix-web

## Build
Run:
```
cargo build
```

## Run
Run:
```
sudo cargo run
```

Server should be listening at localhost:8080
### Testing
Test calling the health endpoint created in this example. Run:
```
curl localhost:8080/health
```

It should return:

```
{"status":"HEALTHY"}
```
