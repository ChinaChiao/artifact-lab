import { simulateGraduation } from "../sim/graduationCore";

function runAverage(config, startIndex, sampleCount) {
  const sampleResults = [];
  const chunkSize = Math.max(5, Math.min(60, Math.floor(160000 / Math.max(1, config.maxRuns))));

  for (let done = 0; done < sampleCount; done += 1) {
    const result = simulateGraduation(config, startIndex + done);
    sampleResults.push(result);

    if (done % chunkSize === 0 || done + 1 === sampleCount) {
      self.postMessage({
        type: done + 1 === sampleCount ? "done" : "progress",
        done: done + 1,
        sampleResults,
      });
    }
  }
}

self.addEventListener("message", (event) => {
  try {
    const { config, startIndex = 0, sampleCount = config.sampleCount } = event.data;
    runAverage(config, startIndex, sampleCount);
  } catch (error) {
    self.postMessage({
      type: "error",
      message: error instanceof Error ? error.message : String(error),
    });
  }
});
