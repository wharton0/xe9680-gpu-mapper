# Dell XE9680 GPU 映射查询程序

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows-blue.svg)](https://www.microsoft.com/windows)

🖥️ 用于 Dell XE9680 服务器的 GPU 映射查询桌面应用程序，帮助快速定位操作系统 ID 与物理 GPU 位置的对应关系。

## 📸 预览

![应用截图](docs/screenshot.png)

## ✨ 功能特性

- 🔍 **实时搜索**: 支持按 Minor Number、Bus ID、Module ID、Slot、PSB 查询
- 🎯 **可视化定位**: 红色高亮显示匹配的物理 GPU 位置
- 📊 **详细信息**: 表格形式展示完整的 GPU 映射信息
- 🎨 **精美界面**: 拟物化 GPU 外观设计，专业美观
- 💻 **桌面应用**: 双击即用，无需浏览器或服务器
- 📦 **单文件部署**: 仅需一个 exe 文件

## 🚀 快速开始

### 下载使用

1. 前往 [Releases](https://github.com/wharton0/xe9680-gpu-mapper/releases) 页面
2. 下载最新版本的 `xe9680-gpu-mapper.exe`
3. 双击运行即可

### 从源码编译

#### 前置要求

- Rust 1.70+ ([安装 Rust](https://rustup.rs/))
- Windows 10/11

#### 编译步骤

```bash
# 克隆仓库
git clone https://github.com/wharton0/xe9680-gpu-mapper.git
cd xe9680-gpu-mapper

# 编译 Release 版本
cargo build --release

# 运行
.\target\release\xe9680-gpu-mapper.exe
```

或者使用便捷脚本：
```bash
# 双击运行
run.bat
```

## 📖 使用说明

1. **启动程序**: 双击 `xe9680-gpu-mapper.exe`
2. **输入查询**: 在搜索框输入任意字段值，例如：
   - `0` - 查找 Minor Number 为 0 的 GPU
   - `3B` - 查找 Bus ID 为 3B 的 GPU
   - `gpu1` - 查找 Module ID 为 gpu1 的 GPU
   - `s28` - 查找位于 Slot 28 的 GPU
   - `psb1` - 查找所有属于 PSB1 的 GPU
3. **查看结果**: 
   - 左侧示意图红色高亮显示匹配的 GPU
   - 右侧表格显示详细信息

## 📊 GPU 数据映射

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

## 🛠️ 技术栈

- **核心语言**: Rust 🦀
- **GUI 框架**: [wry](https://github.com/tauri-apps/wry) (跨平台 WebView)
- **前端技术**: HTML5 + CSS3 + JavaScript
- **样式框架**: TailwindCSS
- **序列化**: serde + serde_json

## 📁 项目结构

```
xe9680-gpu-mapper/
├── src/
│   └── main.rs           # 主程序代码
├── target/
│   └── release/
│       └── xe9680-gpu-mapper.exe  # 可执行文件
├── Cargo.toml            # Rust 项目配置
├── .gitignore            # Git 忽略规则
├── README.md             # 项目说明
├── 快速开始.md           # 中文快速指南
├── run.bat               # 编译运行脚本
└── gupids.html           # 原版 HTML（参考）
```

## 🔧 自定义配置

### 修改 GPU 数据

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

在 `main.rs` 中调整：

```rust
.with_inner_size(wry::application::dpi::LogicalSize::new(1400.0, 900.0))
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建您的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交您的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📝 更新日志

### v1.7.0 (2025-11-21)

- ✨ 初始发布
- 🎨 拟物化 GPU 界面设计
- 🔍 完整的搜索和高亮功能
- 💻 纯净的桌面应用体验（无控制台窗口）
- 📦 单文件部署

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 👤 作者

**Wharton Wang**

- GitHub: [@wharton0](https://github.com/wharton0)

## 🙏 致谢

- [wry](https://github.com/tauri-apps/wry) - 优秀的跨平台 WebView 库
- [TailwindCSS](https://tailwindcss.com/) - 实用的 CSS 框架

---

⭐ 如果这个项目对您有帮助，请给个 Star！
