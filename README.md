# Artifact Lab

[English](README.md) | [中文](README.zh-CN.md)

Artifact Lab is a Vue 3 + FastAPI application for simulating Genshin-style
artifacts and Honkai: Star Rail relics. It includes interactive item
generation, enhancement, single graduation runs, and average graduation
statistics.

## Project Structure

- `frontend/` contains the Vue 3 + Vite single page app.
- `backend/` exposes the probability models through FastAPI.
- `server/` contains Docker Compose, Nginx, and Dockerfile deployment assets.
- `artifact_model.py` and `hsr_relic_model.py` are the shared simulation models
  used by the API.

## Local Development

Start the API:

```powershell
python -m pip install -r .\backend\requirements.txt
python -m uvicorn backend.app.main:app --reload --port 8000
```

Start the Vue app in another terminal:

```powershell
cd .\frontend
npm install
npm run dev
```

Open:

```text
http://127.0.0.1:5173
```

## What It Models

- Genshin artifact set and slot selection
- Genshin weighted main-stat sampling
- Honkai: Star Rail cavern relic and planar ornament generation
- Initial 3/4 substat sampling
- Substat value rolls
- Enhancement events up to +20 for artifacts and +15 for relics
- Build graduation simulation and average graduation statistics in the Vue app

The model follows a two-step generation path: weighted single-item generation
first, then independent enhancement events. That keeps the mechanics suitable
for an interactive UI while leaving room for longer-term farming analysis.

## Docker Deployment

From the `server/` folder:

```powershell
docker compose up --build
```

The app listens on:

```text
http://127.0.0.1:18080
```

See `server/README.md` for 1Panel and reverse proxy notes.

## Archive Policy

The GitHub `main` branch contains only the current Vue + FastAPI architecture.
Older local HTML prototypes are intentionally kept out of the GitHub main
branch and preserved locally in the `local-html-archive` branch.
