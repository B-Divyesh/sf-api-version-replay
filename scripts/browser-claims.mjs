import { spawn } from "node:child_process";
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
    if (claim === "browser-demo-isolation") {
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
      assert((await page.locator("h1").allTextContents()).join("") === "Compare sample webhook versions", "demo needs one useful h1");
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
      for (const [path, title, h1] of [["/", "Version Replay — Test webhook versions locally", "Test old webhook versions against localhost"], ["/?demo=1", "Demo — Version Replay", "Compare sample webhook versions"], ["/privacy/", "Privacy — Version Replay", "Privacy"], ["/terms/", "Terms — Version Replay", "Terms"]]) {
        await page.goto(`${origin}${path}`, { waitUntil: "networkidle" });
        assert(await page.title() === title, `${path} title mismatch`);
        assert(await page.locator("h1").count() === 1, `${path} needs one h1`);
        assert(await page.locator("h1").textContent() === h1, `${path} h1 mismatch`);
        assert(await page.locator('link[rel="canonical"]').count() === 1, `${path} canonical missing`);
        assert(await page.locator('meta[property="og:image"]').count() === 1, `${path} OG image missing`);
        assert(await page.locator('meta[property="og:title"]').getAttribute("content") === title, `${path} OG title mismatch`);
        assert(await page.locator('meta[name="twitter:title"]').getAttribute("content") === title, `${path} Twitter title mismatch`);
      }
      await context.close();
    } else throw new Error(`Unknown browser claim: ${claim}`);
  } finally { await browser.close(); }
  console.log(`@claim:${claim} passed`);
} finally {
  server.kill("SIGTERM");
}
