# Vert

**v0.2.0** — 万能文件格式转换工具 — 桌面应用 + 命令行。基于 Tauri 2 + Vue 3 + Rust。

[English](README.md) | [Apache 2.0 许可证](LICENSE)

## 支持的格式（21种）

| 类别 | 格式 |
|---|---|
| **配置** (6) | `.properties` `.yaml` `.yml` `.json` `.toml` `.xml` `.ini` `.cfg` `.conf` — 任意互转 |
| **数据** (1) | `.csv` ↔ `.json` `.xml`（表格数据） |
| **文档** (6) | `.epub` `.docx` `.pdf` `.md` `.markdown` `.html` `.htm` `.txt` `.text` |
| **图片** (7) | `.svg` `.png` `.jpg` `.jpeg` `.gif` `.webp` `.bmp` `.tiff` — 任意互转 |

**图片转换**：SVG 通过 resvg 栅格化为任意图片格式；PNG/JPEG/GIF/WebP/BMP/TIFF 使用纯 Rust `image` crate。

## 安装

### 桌面应用

从 [Releases](https://github.com/jean3690/vert/releases) 下载。

或从源码构建：

```bash
pnpm install
pnpm tauri build
```

### 命令行

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/jean3690/vert/master/install.ps1 | iex
```

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/jean3690/vert/master/install.sh | bash
```

或通过 cargo 安装：

```bash
cd src-tauri
cargo build --bin vert-cli --no-default-features
./target/debug/vert-cli --help
```

## CLI 用法

```bash
# 转换
vert data.json yaml
vert config.xml json -o output.json
vert document.docx pdf
vert logo.svg png

# 查看
vert input.csv              # 显示可转换的目标格式
vert --list                 # 列出所有支持的格式
```

## 开发

```bash
pnpm install                # 前端依赖

pnpm dev                    # Vite 开发服务器（端口 1420）
pnpm tauri dev              # Tauri 桌面应用（开发模式）
pnpm build                  # 类型检查 + 生产构建

cd src-tauri
cargo test                  # Rust 单元测试（48+ 测试）
cargo build --bin vert-cli --no-default-features  # 仅编译 CLI
```

DOCX/MD/HTML → PDF 转换需要将 Liberation Sans 字体放入 `src-tauri/fonts/` 目录。

## 许可证

Apache 2.0 — 见 [LICENSE](LICENSE)。
