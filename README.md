# Artifact Lab

[English](README.md) | [中文](README.zh-CN.md)

Artifact Lab is a fully local Tauri desktop app for simulating Genshin-style
artifacts and Honkai: Star Rail relics. The Vue interface runs inside the
desktop shell, while Rust provides the local probability model for item
generation and enhancement.

## Features

- Generate and enhance individual 5-star artifacts or relics.
- Simulate build graduation runs with configurable goals.
- Run average graduation statistics in the local Rust model.
- Work offline without FastAPI, Docker, Nginx, or a separate server process.

## Project Structure

- `frontend/` contains the Vue 3 + Vite interface.
- `src-tauri/` contains the Tauri desktop shell and local commands.
- `crates/artifact_core/` contains the Rust probability model.
- `Cargo.toml` defines the Rust workspace.
- `package.json` defines the root Node/Tauri scripts.

## Local Development

Install JavaScript dependencies:

```powershell
npm install
```

Start the desktop app in development mode:

```powershell
npm run tauri:dev
```

Build the Vue frontend only:

```powershell
npm run frontend:build
```

Run the full verification path:

```powershell
npm run check
```

Build the desktop app:

```powershell
npm run build
```

Build and collect the Windows MSI plus release executable:

```powershell
npm run package:msi
```

## Verification

Run the Rust test suite:

```powershell
npm run rust:test
```

Run the Rust compile check for the Tauri app:

```powershell
cargo check -p app
```

## What It Models

- Genshin artifact set and slot selection.
- Genshin weighted main-stat sampling.
- Honkai: Star Rail cavern relic and planar ornament generation.
- Initial 3/4 substat sampling.
- Substat value rolls.
- Enhancement events up to +20 for artifacts and +15 for relics.
- Build graduation simulation and average graduation statistics.
