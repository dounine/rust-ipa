# rust-ipa

## 开发
自动重载
```bash
cargo install systemfd
# 以下可复用端口
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
删除数据库
```bash
sea-orm-cli migrate down
```
创建数据库
```bash
sea-orm-cli migrate up
```
自动删除并创
```bash
sea-orm-cli migrate refresh
```
