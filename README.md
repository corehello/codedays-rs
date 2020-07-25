codedays-rs
===

## 使用

```shell
echo "DATABASE_URL=postgres://postgres:dayscode@localhost/codedays?sslmode=disable" > .env

cargo install diesel_cli --features "postgres" --no-default-features
diesel migration run

cargo build
```
