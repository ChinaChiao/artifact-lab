import { invoke } from '@tauri-apps/api/core';

export function getMeta() {
  return invoke('get_meta');
}

export function generateArtifact(payload) {
  return invoke('generate_artifact', { payload });
}

export function enhanceArtifact(payload) {
  return invoke('enhance_artifact', { payload });
}

export function generateRelic(payload) {
  return invoke('generate_relic', { payload });
}

export function enhanceRelic(payload) {
  return invoke('enhance_relic', { payload });
}

export function simulateGraduation(payload) {
  return invoke('simulate_graduation', { payload });
}

export function summarizeGraduationAverage(payload) {
  return invoke('summarize_graduation_average', { payload });
}
