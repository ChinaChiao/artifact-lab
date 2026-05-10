const API_BASE = import.meta.env.VITE_API_BASE ?? "";

async function request(path, options = {}) {
  const response = await fetch(`${API_BASE}${path}`, {
    headers: { "Content-Type": "application/json", ...(options.headers ?? {}) },
    ...options,
  });
  if (!response.ok) {
    const detail = await response.json().catch(() => ({}));
    throw new Error(detail.detail || `HTTP ${response.status}`);
  }
  return response.json();
}

export function getMeta() {
  return request("/api/meta");
}

export function generateArtifact(payload) {
  return request("/api/genshin/artifacts/generate", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export function enhanceArtifact(payload) {
  return request("/api/genshin/artifacts/enhance", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export function generateRelic(payload) {
  return request("/api/hsr/relics/generate", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}

export function enhanceRelic(payload) {
  return request("/api/hsr/relics/enhance", {
    method: "POST",
    body: JSON.stringify(payload),
  });
}
