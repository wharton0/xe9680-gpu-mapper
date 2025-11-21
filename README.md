# XE9680 GPU 映射查询程序 - Rust 桌面应用

🦀 使用 Rust 开发的 Dell XE9680 GPU 映射查询桌面应用程序

## 版本信息

- **版本**: v1.7
- **作者**: Wharton Wang
- **技术栈**: Rust + wry (WebView) + HTML/CSS/JavaScript

## 功能特性

✨ **核心功能**:
- 🖥️ **桌面应用**: 双击 exe 文件直接打开窗口，就像打开 HTML 一样简单
- 🚫 **无需服务器**: 所有界面和数据都内嵌在应用中
- 🔍 实时搜索 GPU 信息（Minor Number, Bus ID, Module ID, Slot, PSB）
- 🎯 可视化物理位置示意图，红色高亮匹配的 GPU
- 📊 详细信息展示面板
- 🎨 拟物化 GPU 外观设计（黑色散热器 + 金属盖板）

🚀 **Rust 优势**:
- ⚡ 轻量级桌面应用（基于 WebView）
- 🛡️ 内存安全保证
- 📦 单一可执行文件，无需安装依赖
- 🪟 原生窗口体验

## 快速开始

### 方式一：使用预编译版本（推荐）

1. 编译一次（首次使用）:
```bash
cargo build --release
```

2. 之后直接双击运行:
```
target\release\xe9680-gpu-mapper.exe
```

### 方式二：使用批处理脚本

双击 `run.bat` 即可自动编译并启动程序。

### 方式三：命令行编译运行

```bash
# 编译并运行
cargo run --release

# 或者先编译
cargo build --release

# 然后运行
.\target\release\xe9680-gpu-mapper.exe
```

## 使用说明

1. **启动**: 双击 `xe9680-gpu-mapper.exe` 
2. **查询**: 在搜索框输入任意字段值（如 `0`, `gpu1`, `3B`）
3. **查看结果**: 
   - 左侧示意图会红色高亮显示匹配的物理 GPU 位置
   - 右侧显示详细信息

## GPU 数据映射

| Minor Number | Bus ID | Module ID | Slot | PSB  |
|--------------|--------|-----------|------|------|
| 0            | 19     | gpu2      | s28  | psb1 |
| 1            | 3B     | gpu4      | s24  | psb1 |
| 2            | 4C     | gpu1      | s23  | psb2 |
| 3            | 5D     | gpu3      | s27  | psb2 |
| 4            | 9B     | gpu7      | s25  | psb4 |
| 5            | BB     | gpu5      | s21  | psb4 |
| 6            | CB     | gpu6      | s26  | psb3 |
| 7            | DB     | gpu8      | s22  | psb3 |

## 项目结构

```
gpuids/
├── Cargo.toml              # Rust 项目配置
├── src/
│   └── main.rs             # 桌面应用主程序
├── target/
│   └── release/
│       └── xe9680-gpu-mapper.exe  # 可执行文件
├── gupids.html             # 原版 HTML（参考）
├── run.bat                 # 一键编译运行脚本
└── README.md               # 本文件
```

## 技术细节

### 后端 (Rust)
- **GUI 框架**: wry (跨平台 WebView 库)
- **序列化**: serde + serde_json
- **窗口大小**: 1400x900 像素

### 前端
- **样式**: TailwindCSS (CDN)
- **交互**: 原生 JavaScript
- **数据**: 内嵌在 HTML 中（编译时注入）

## 与原版 HTML 的对比

| 特性 | 原版 HTML | Rust 桌面应用 |
|------|-----------|---------------|
| 启动方式 | 双击 HTML 文件 | 双击 exe 文件 |
| 浏览器依赖 | 需要浏览器 | 内嵌 WebView，无需浏览器 |
| 数据存储 | 硬编码在 HTML | Rust 结构体，编译时注入 |
| 窗口体验 | 浏览器标签页 | 原生应用窗口 |
| 部署 | 单个 HTML 文件 | 单个 exe 文件 |
| 文件大小 | ~25 KB | ~3-5 MB (含 WebView) |

## 开发说明

### 添加新 GPU

编辑 `src/main.rs` 中的 `gpu_data` 向量：

```rust
GpuInfo {
    minor_number: "8".to_string(),
    bus_id: "XX".to_string(),
    module_id: "gpuX".to_string(),
    slot: "sXX".to_string(),
    psb: "psbX".to_string(),
},
```

### 修改窗口大小

在 `main.rs` 中修改:

```rust
.with_inner_size(wry::application::dpi::LogicalSize::new(1400.0, 900.0))
```

### 修改样式

HTML 和 CSS 都在 `main.rs` 的 `html` 字符串中，可以直接修改。

## 系统要求

- **操作系统**: Windows 10/11 (或 Linux/macOS，需重新编译)
- **依赖**: WebView2 Runtime (Windows 10/11 通常已预装)

## 常见问题

### Q: 为什么 exe 文件这么大？
A: 因为内嵌了 WebView 运行时和 Rust 标准库。这样的好处是无需任何外部依赖。

### Q: 我想修改 GPU 数据怎么办？
A: 修改 `src/main.rs` 中的数据后，重新运行 `cargo build --release` 编译即可。

### Q: 能否不用 Rust，直接使用 HTML？
A: 可以！直接使用原版的 `gupids.html` 文件即可。Rust 版本的优势在于提供了原生应用体验。

## 许可证

由 Wharton Wang 创建，用于 Dell XE9680 系统管理。

---

💡 **提示**: 如果你只需要简单的查询工具，可以直接使用原版 `gupids.html`。Rust 桌面版适合需要独立应用、无浏览器环境或更专业外观的场景。
