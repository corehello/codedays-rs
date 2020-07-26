codedays-rs
===

## 使用

```shell
echo "DATABASE_URL=postgres://postgres:dayscode@localhost/codedays?sslmode=disable" > .env

cargo install diesel_cli --features "postgres" --no-default-features
diesel migration run

cargo run
```

## 自动加载

参见 https://actix.rs/docs/autoreload/ 进行配置

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
