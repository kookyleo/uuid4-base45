# uuid45

[![CI](https://github.com/kookyleo/uuid4-base45/actions/workflows/ci.yml/badge.svg)](https://github.com/kookyleo/uuid4-base45/actions/workflows/ci.yml)

基于 UUID v4 的紧凑 Base45 编解码：移除 6 位固定位（版本与变体），提供 CLI、Rust 库以及 WASM 绑定。

特性：
- 将 UUID v4（128 位）去除版本（4 位，0100）与变体（2 位，10）的固定位后，得到 122 位有效内容。将其打包成 16 字节（最后一个字节仅低 2 位有效，高 6 位为 0 填充），再使用 Base45 编码为字符串。
- 可将上述 Base45 字符串完整还原回原始 UUID v4（字节或字符串）。
- 同时提供库 API、命令行工具、WASM 绑定和一个 HTML 示例。

## 安装

- 构建 CLI：`cargo install --path .`
- 以库形式使用：在你的项目中添加依赖 `uuid45 = { git = "https://github.com/kookyleo/uuid4-base45.git" }`，或使用本地路径依赖。

## 命令行用法

```
uuid45

命令：
  gen                       生成随机 UUID v4，并打印 Base45 与 UUID
  encode <UUID|HEX|@->     将 UUID 编码为 Base45。支持：
                           - 标准 UUID 字符串（xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx）
                           - 32 位十六进制（无连字符）
                           - 通过 @- 从 stdin 读入 16 字节原始数据
  decode <BASE45|@->       将 Base45 字符串解码回 UUID 字符串与字节（hex）

选项：
  -q, --quiet              仅输出主要结果
  -h, --help               显示帮助
```

示例：
- `uuid45 gen`
- `uuid45 encode 550e8400-e29b-41d4-a716-446655440000`
- `uuid45 decode <base45>`

## 库 API

- `generate_v4() -> Uuid`
- `encode_uuid(uuid: Uuid) -> String`
- `encode_uuid_str(s: &str) -> Result<String, Uuid45Error>`
- `encode_uuid_bytes(bytes: &[u8; 16]) -> String`
- `decode_to_uuid(s: &str) -> Result<Uuid, Uuid45Error>`
- `decode_to_bytes(s: &str) -> Result<[u8; 16], Uuid45Error>`
- `decode_to_string(s: &str) -> Result<String, Uuid45Error>`

## WASM 使用

当为 `wasm32-unknown-unknown` 目标并配合 `wasm-bindgen` 编译时，提供以下绑定：
- `wasm_gen_v4() -> String`
- `wasm_encode_uuid_str(s: &str) -> Result<String, JsValue>`
- `wasm_decode_to_uuid_str(s: &str) -> Result<String, JsValue>`
- `wasm_decode_to_bytes(s: &str) -> Result<Uint8Array, JsValue>`

示例页面位于 `examples/wasm/index.html`。

### 本地构建 WASM（同时提供 wasm-pack 方式）

- 安装目标与 wasm-bindgen-cli：

```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --no-typescript \
  --out-dir examples/wasm/pkg \
  --out-name uuid45 \
  target/wasm32-unknown-unknown/release/uuid45.wasm

# 或使用 wasm-pack
# cargo install wasm-pack
# wasm-pack build --target web --out-dir examples/wasm/pkg --out-name uuid45
```

- 使用静态服务器打开 `examples/wasm/index.html`（例如 `python3 -m http.server`）。

## 下载产物

- 来自 CI（最新构建）：进入 Actions 页面，选择最近一次成功的 CI 任务，下载名为 "wasm-demo" 的 artifact。链接：https://github.com/kookyleo/uuid4-base45/actions
- 来自 Release：对于打了标签（v*）的版本，可在 Releases 页面下载附带的 wasm-demo.tar.gz。链接：https://github.com/kookyleo/uuid4-base45/releases

## 测试

`cargo test` 覆盖：
- 随机 UUID 回环测试
- 固定 UUID 回环测试
- 填充位校验

## 为什么是 122 位？

UUID v4 固定 4 位版本号与 2 位变体位，分别为 0100 与 10。移除它们后剩余 128-6=122 位熵。我们将其打包为 16 字节，其中最后一个字节只有低 2 位有效，高 6 位为 0 填充，然后使用 Base45 编码。

## 许可证

Apache-2.0
