# Server Deployment

This folder contains the deployment project for 1Panel or Docker Compose.
The application source remains in the repository root, `backend/`, and
`frontend/` so local development has a single source of truth.

## Run with Docker Compose

From this folder:

```bash
docker compose up -d --build
```

The app listens on:

```text
http://127.0.0.1:18080
```

For 1Panel, create a Compose project from `server/compose.yaml`, then create an
OpenResty reverse proxy site to:

```text
http://host-ip:18080
```

Do not expose port `18080` directly to the public internet. Expose only `80`
and `443` through 1Panel/OpenResty.
