# qr-url-uuid4

[![CI](https://github.com/kookyleo/qr-url-uuid4/actions/workflows/ci.yml/badge.svg)](https://github.com/kookyleo/qr-url-uuid4/actions/workflows/ci.yml)

在线示例（GitHub Pages）：https://kookyleo.github.io/qr-url-uuid4/

使用 Base44 将 UUID v4 编码为紧凑的 QR 友好 URL。移除 6 个固定位（版本 + 变体）以优化 QR 码字母数字模式编码。

## 概述

本库实现了 UUID v4 标识符的紧凑编码方案：

- **输入**: 标准 UUID v4（128 位）
- **优化**: 移除 6 个确定性位（4 位版本 + 2 位变体）→ 122 位熵
- **编码**: Base44（QR 字母数字字符集，不含空格）
- **输出**: 紧凑的 URL 安全字符串（通常 24-25 字符）

### 为什么选择 Base44 而不是 Base45？

Base45（[RFC 9285](https://datatracker.ietf.org/doc/html/rfc9285)）使用完整的 QR 码字母数字字符集：`0-9A-Z $%*+-./:`（45 个字符）。但是，**空格字符**在 URL 嵌入时会造成问题：

- ❌ **需要 URL 编码**: 空格必须编码为 `%20` 或 `+`，增加长度
- ❌ **代理问题**: 某些 HTTP 代理和服务器会删除首尾空格
- ❌ **复制粘贴问题**: 用户从浏览器或日志复制 URL 时可能丢失空格
- ❌ **处理不一致**: 不同系统对空格处理不同（百分号编码 vs 加号编码）

**Base44** 从字母表中移除了空格字符，提供：

- ✅ **真正的 URL 安全**: 任何字符都不需要百分号编码
- ✅ **QR 最优**: 仍使用 QR 字母数字模式（平均 5.5 位/字符）
- ✅ **可靠**: URL 处理在不同系统间无歧义
- ✅ **紧凑**: 由于字母表略小，仅比 Base45 稍长

详细的英文文档请参阅 [README.md](README.md)

## 安装

- 构建 CLI：`cargo install --path .`
- 以库形式使用：在你的项目中添加依赖 `qr-url-uuid4 = { git = "https://github.com/kookyleo/qr-url-uuid4.git" }`，或使用本地路径依赖。

## 命令行用法

```
qr-url-uuid4

命令：
  gen                       生成随机 UUID v4，并打印 Base44 与 UUID
  encode <UUID|HEX|@->     将 UUID 编码为 Base44。支持：
                           - 标准 UUID 字符串（xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx）
                           - 32 位十六进制（无连字符）
                           - 通过 @- 从 stdin 读入 16 字节原始数据
  decode <BASE44|@->       将 Base44 字符串解码回 UUID 字符串与字节（hex）

选项：
  -q, --quiet              仅输出主要结果
  -h, --help               显示帮助
```

示例：
- `qr-url-uuid4 gen`
- `qr-url-uuid4 encode 550e8400-e29b-41d4-a716-446655440000`
- `qr-url-uuid4 decode <base44-string>`

## GitHub Pages 示例

项目会自动发布一个在线示例到 GitHub Pages：
- https://kookyleo.github.io/qr-url-uuid4/

## 为什么是 122 位？

UUID v4 固定 4 位版本号与 2 位变体位，分别为 0100 与 10。移除它们后剩余 128-6=122 位熵。我们将其打包为 16 字节，其中最后一个字节只有低 2 位有效，高 6 位为 0 填充，然后使用 Base44 编码。

## 许可证

Apache-2.0
