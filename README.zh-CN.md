# Artifact Lab

[English](README.md) | [中文](README.zh-CN.md)

Artifact Lab 是一个完全本地运行的 Tauri 桌面应用，用于模拟原神风格五星圣遗物与崩坏：星穹铁道五星遗器。Vue 负责界面，Rust 负责本地概率模型、生成和强化逻辑。

## 功能特性

- 单件 5 星圣遗物/遗器生成与强化。
- 可配置目标的毕业模拟。
- 使用本地 Rust 模型运行平均毕业统计。
- 离线运行，不需要 FastAPI、Docker、Nginx 或独立服务器进程。

## 项目结构

- `frontend/`：Vue 3 + Vite 界面。
- `src-tauri/`：Tauri 桌面壳与本地命令。
- `crates/artifact_core/`：Rust 概率模型。
- `Cargo.toml`：Rust workspace 配置。
- `package.json`：根目录 Node/Tauri 脚本。

## 本地开发

安装 JavaScript 依赖：

```powershell
npm install
```

启动桌面开发模式：

```powershell
npm run tauri:dev
```

只构建 Vue 前端：

```powershell
npm run frontend:build
```

运行完整验证：

```powershell
npm run check
```

构建桌面应用：

```powershell
npm run build
```

构建并收集 Windows MSI 与 release 可执行文件：

```powershell
npm run package:msi
```

## 验证

运行 Rust 测试：

```powershell
npm run rust:test
```

检查 Tauri 应用编译：

```powershell
cargo check -p app
```

## 模型能力

- 原神圣遗物套装与部位选择。
- 原神主词条按权重抽样。
- 星铁隧洞遗器与位面饰品生成。
- 初始 3/4 个副词条抽样。
- 副词条档位抽样。
- 圣遗物最高 +20、遗器最高 +15 的强化事件。
- 毕业模拟与平均毕业统计。
