# rust-ipa

## 开发
自动重载
```bash
cargo install systemfd
# 以下可复用端口
systemfd --no-pid -s http::8080 -- cargo watch -x run
```