import { spawn } from "node:child_process";
import { readFileSync } from "node:fs";
import { chromium } from "playwright";

const claim = process.argv[2];
const port = 4197;
const origin = `http://127.0.0.1:${port}`;
const server = spawn("npx", ["vite", "preview", "--config", "site/vite.config.ts", "--host", "127.0.0.1", "--port", String(port)], { stdio: "ignore" });
const assert = (condition, message) => { if (!condition) throw new Error(message); };

async function ready() {
  for (let attempt = 0; attempt < 80; attempt += 1) {
    try { if ((await fetch(origin)).ok) return; } catch { /* wait */ }
    await new Promise((resolve) => setTimeout(resolve, 100));
  }
  throw new Error("preview server did not start");
}

try {
  await ready();
  const browser = await chromium.launch();
  try {
    if (claim === "primary-demo-workflow") {
      // @claim:primary-demo-workflow
      const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
      const page = await context.newPage();
      await page.goto(`${origin}/?demo=1`, { waitUntil: "networkidle" });
      assert(await page.locator("h1").textContent() === "Replay the complete CLI sample", "primary demo should name the complete CLI job");
      const recording = page.locator(".terminal-recording");
      await recording.waitFor();
      const output = await recording.textContent();
      for (const expected of [
        "Imported 2 redacted fixtures",
        "5 contract changes",
        "Replayed 2024-04-10 → HTTP 204",
        "Replayed 2025-02-24 → HTTP 204",
        "version-replay-report.md"
      ]) assert(output.includes(expected), `primary demo missing: ${expected}`);
      assert(await page.locator("#bench-title").textContent() === "Compare two sample webhook fixtures", "JSON comparator should remain secondary");
      await context.close();
    } else if (claim === "browser-demo-isolation") {
      // @claim:browser-demo-isolation
      const context = await browser.newContext();
      await context.addInitScript(() => {
        localStorage.setItem("vr_theme", "light");
        localStorage.setItem("sb_license:api-version-replay", "must-not-be-read");
        window.__storageReads = [];
        window.__storageWrites = [];
        const get = Storage.prototype.getItem;
        const set = Storage.prototype.setItem;
        Storage.prototype.getItem = function (key) { window.__storageReads.push(key); return get.call(this, key); };
        Storage.prototype.setItem = function (key, value) { window.__storageWrites.push(key); return set.call(this, key, value); };
      });
      const page = await context.newPage();
      const requests = [];
      page.on("request", (request) => requests.push({ url: request.url(), data: request.postData() }));
      await page.goto(`${origin}/?demo=1`, { waitUntil: "networkidle" });
      await page.locator(".result-summary").waitFor();
      assert((await page.locator("h1").allTextContents()).join("") === "Replay the complete CLI sample", "demo needs one useful h1");
      assert(await page.locator("#demo-banner").isVisible(), "demo banner missing");
      assert(await page.locator("#demo-banner").getByText("Reset demo").isVisible(), "reset missing");
      assert(await page.locator("#demo-banner").getByText("Start for real").isVisible(), "exit missing");
      const storage = await page.evaluate(() => ({ reads: window.__storageReads, writes: window.__storageWrites }));
      assert(storage.reads.length === 0 && storage.writes.length === 0, `demo touched storage: ${JSON.stringify(storage)}`);
      assert(requests.every((request) => new URL(request.url).origin === origin), "demo made a cross-origin request");
      const fixtureText = await page.locator("#fixture-old").inputValue();
      assert(requests.every((request) => !request.data?.includes(fixtureText)), "demo uploaded fixture text");
      await page.locator("#fixture-old").fill("{}");
      await page.locator("#demo-reset").click();
      await page.locator(".result-summary").waitFor();
      assert((await page.locator("#fixture-old").inputValue()).includes("payment.failed"), "reset did not restore sample");
      await context.close();
    } else if (claim === "browser-storage-scope") {
      // @claim:browser-storage-scope
      const context = await browser.newContext();
      const page = await context.newPage();
      const requests = [];
      page.on("request", (request) => requests.push(request.url()));
      await page.goto(origin, { waitUntil: "networkidle" });
      await page.locator(".theme-toggle").click();
      const storage = await page.evaluate(() => ({ keys: Object.keys(localStorage), cookies: document.cookie }));
      assert(JSON.stringify(storage.keys) === JSON.stringify(["vr_theme"]), `unexpected storage: ${storage.keys}`);
      assert(storage.cookies === "", `unexpected cookies: ${storage.cookies}`);
      assert(requests.every((request) => new URL(request).origin === origin), "normal browsing made a cross-origin request");
      await context.close();
    } else if (claim === "offline-demo") {
      // @claim:offline-demo
      const context = await browser.newContext();
      const page = await context.newPage();
      await page.goto(`${origin}/?demo=1`, { waitUntil: "networkidle" });
      await page.waitForFunction(() => navigator.serviceWorker?.controller !== null);
      await context.setOffline(true);
      await page.reload({ waitUntil: "domcontentloaded" });
      await page.locator(".result-summary").waitFor();
      assert(await page.locator("#demo-banner").isVisible(), "offline demo banner missing");
      assert((await page.locator(".result-summary > p").textContent()) === "Contract changed", "offline result missing");
      await context.close();
    } else if (claim === "route-metadata") {
      // @claim:route-metadata
      const context = await browser.newContext();
      const page = await context.newPage();
      const routes = [
        ["/", "Version Replay — Test webhook versions locally", "Test old webhook versions against localhost", "https://api-version-replay.sociobot.in/", false],
        ["/?demo=1", "Demo — Version Replay", "Replay the complete CLI sample", "https://api-version-replay.sociobot.in/?demo=1", false],
        ["/privacy/", "Privacy — Version Replay", "Privacy", "https://api-version-replay.sociobot.in/privacy/", false],
        ["/terms/", "Terms — Version Replay", "Terms", "https://api-version-replay.sociobot.in/terms/", false],
        ["/404.html", "Page not found — Version Replay", "This page was not found", "https://api-version-replay.sociobot.in/404.html", true]
      ];
      for (const [path, title, h1, canonical, noindex] of routes) {
        await page.goto(`${origin}${path}`, { waitUntil: "networkidle" });
        assert(await page.title() === title, `${path} title mismatch`);
        assert(await page.locator("h1").count() === 1, `${path} needs one h1`);
        assert(await page.locator("h1").textContent() === h1, `${path} h1 mismatch`);
        assert(await page.locator('link[rel="canonical"]').count() === 1, `${path} canonical missing`);
        assert(await page.locator('link[rel="canonical"]').getAttribute("href") === canonical, `${path} canonical mismatch`);
        assert(await page.locator('meta[property="og:image"]').count() === 1, `${path} OG image missing`);
        assert(await page.locator('meta[property="og:title"]').getAttribute("content") === title, `${path} OG title mismatch`);
        assert(await page.locator('meta[name="twitter:title"]').getAttribute("content") === title, `${path} Twitter title mismatch`);
        assert(await page.locator('meta[name="twitter:description"]').count() === 1, `${path} Twitter description missing`);
        assert(await page.locator('meta[name="twitter:image"]').getAttribute("content") === "https://api-version-replay.sociobot.in/social-card.jpg", `${path} Twitter image mismatch`);
        const robotsMeta = page.locator('meta[name="robots"]');
        const robots = await robotsMeta.count() ? await robotsMeta.getAttribute("content") : null;
        assert(noindex ? robots?.includes("noindex") : !robots?.includes("noindex"), `${path} robots treatment mismatch`);
      }
      const policy = JSON.parse(readFileSync("dist/site/staticwebapp.config.json", "utf8"));
      assert(policy.responseOverrides?.["404"]?.rewrite === "/404.html", "host must rewrite missing routes to the designed 404");
      await context.close();
    } else throw new Error(`Unknown browser claim: ${claim}`);
  } finally { await browser.close(); }
  console.log(`@claim:${claim} passed`);
} finally {
  server.kill("SIGTERM");
}
