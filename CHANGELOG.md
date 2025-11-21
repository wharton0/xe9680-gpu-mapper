# 更新日志

本项目的所有重要更改都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵守 [语义化版本](https://semver.org/lang/zh-CN/)。

## [1.7.0] - 2025-11-21

### ✨ 新增

- 🎨 **拟物化 GPU 界面设计**
  - 黑色散热器背景效果
  - 金属盖板样式（香槟金色调）
  - 螺丝钉装饰细节
  - 红色高亮选中效果

- 🔍 **强大的搜索功能**
  - 支持 Minor Number 查询
  - 支持 Bus ID 查询
  - 支持 Module ID 查询
  - 支持 Slot 查询
  - 支持 PSB 查询
  - 实时搜索，即时反馈

- 🖥️ **桌面应用体验**
  - 双击即可运行，无需安装
  - 无控制台窗口（纯 GUI 模式）
  - 单一可执行文件（~720 KB）
  - 原生窗口体验

- 📊 **可视化功能**
  - 2×4 GPU 物理布局图
  - 前后方向标识
  - 红色高亮匹配的 GPU
  - 放大动画效果

- 📋 **详细信息展示**
  - 表格形式展示 GPU 信息
  - 匹配字段高亮显示
  - 完整的字段说明

### 🛠️ 技术细节

- Rust 1.70+ 编写
- 基于 wry WebView 框架
- 使用 TailwindCSS 美化界面
- serde/serde_json 数据序列化
- Windows subsystem 配置（无控制台）

### 📦 构建优化

- 启用 LTO (Link Time Optimization)
- 启用 strip（移除调试符号）
- 优化文件大小

### 📝 文档

- 完整的 README.md
- 中文快速开始指南
- MIT 开源许可证
- 项目结构说明

### 🎯 GPU 数据映射

支持 Dell XE9680 8 块 GPU 的完整映射：
- GPU 1-8 的 Minor Number、Bus ID、Slot、PSB 映射
- PSB1-PSB4 的分组信息

---

## [未发布]

### 计划中的功能

- [ ] 支持自定义 GPU 数据配置文件
- [ ] 添加导出功能（PDF/图片）
- [ ] 多语言支持（English）
- [ ] 深色/浅色主题切换
- [ ] 系统托盘功能
- [ ] 快捷键支持

---

[1.7.0]: https://github.com/wharton0/xe9680-gpu-mapper/releases/tag/v1.7.0
