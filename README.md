codedays-rs
===

[![Coverage Status](https://coveralls.io/repos/github/overfive/codedays-rs/badge.svg?branch=develop)](https://coveralls.io/github/overfive/codedays-rs?branch=develop)

## 使用

```shell
cp .env.example .env
# 安装 diesel
cargo install diesel_cli --features="barrel-migrations,barrel/sqlite3,sqlite"
diesel migration run

cargo run
```

## 自动加载

参见 https://actix.rs/docs/autoreload/ 进行配置

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
