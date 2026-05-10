# Artifact Lab

[English](README.md) | [中文](README.zh-CN.md)

Artifact Lab 是一个基于 Vue 3 + FastAPI 的圣遗物/遗器模拟应用，用于模拟原神风格五星圣遗物与崩坏：星穹铁道五星遗器的生成、强化、单次毕业和平均毕业统计。

## 项目结构

- `frontend/`：Vue 3 + Vite 单页应用。
- `backend/`：通过 FastAPI 暴露概率模型接口。
- `server/`：Docker Compose、Nginx 和 Dockerfile 部署配置。
- `artifact_model.py` 与 `hsr_relic_model.py`：后端 API 使用的共享模拟模型。

## 本地开发

启动 API：

```powershell
python -m pip install -r .\backend\requirements.txt
python -m uvicorn backend.app.main:app --reload --port 8000
```

另开一个终端启动 Vue 应用：

```powershell
cd .\frontend
npm install
npm run dev
```

访问：

```text
http://127.0.0.1:5173
```

## 模型能力

- 原神圣遗物套装与部位选择
- 原神主词条按权重抽样
- 星铁隧洞遗器与位面饰品生成
- 初始 3/4 个副词条抽样
- 副词条档位抽样
- 圣遗物最高 +20、遗器最高 +15 的强化事件
- Vue 应用内的毕业模拟与平均毕业统计

模型采用两阶段生成思路：先按权重生成单件物品，再进行独立强化事件。这样既适合交互式界面，也为后续更长周期的刷取分析留下空间。

## Docker 部署

进入 `server/` 目录：

```powershell
docker compose up --build
```

默认访问地址：

```text
http://127.0.0.1:18080
```

1Panel 与反向代理说明见 `server/README.md`。

## 归档说明

GitHub 的 `main` 分支只保留当前 Vue + FastAPI 架构。过去用于本地运行的 HTML 原型不会上传到 GitHub 主分支，并已保留在本地 `local-html-archive` 分支中。
