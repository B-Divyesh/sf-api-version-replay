import { chromium } from "playwright";
import AxeBuilder from "@axe-core/playwright";

const url = process.argv[2] ?? "http://127.0.0.1:4173/";
const origin = new URL(url).origin;

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function runDesktop(browser) {
  const context = await browser.newContext({ viewport: { width: 1366, height: 900 } });
  const page = await context.newPage();
  const errors = [];
  const externalRequests = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  page.on("response", (response) => {
    if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`);
  });
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== origin) externalRequests.push(request.url());
  });

  await page.goto(url, { waitUntil: "networkidle" });
  await page.waitForFunction(() => navigator.serviceWorker?.controller !== null);
  assert(await page.locator("h1").count() === 1, "expected exactly one home h1");
  assert(await page.locator("main").count() === 1, "expected a main landmark");

  await page.locator("#fixture-old").focus();
  await page.keyboard.press("Tab");
  assert(await page.evaluate(() => document.activeElement?.id === "fixture-new"), "Tab should reach the next fixture");
  await page.keyboard.press("Tab");
  assert(await page.evaluate(() => document.activeElement?.id === "compare-button"), "Tab should reach Compare locally");
  await page.keyboard.press("Enter");
  await page.locator(".result-summary").waitFor();
  assert(
    await page.locator(".result-summary > p").evaluate((element) => element.textContent === "Contract changed"),
    "keyboard compare should render a diff"
  );

  await page.locator(".theme-toggle").click();
  assert(await page.locator(".theme-toggle").getAttribute("aria-pressed") === "true", "theme toggle should update its pressed state");
  await page.emulateMedia({ reducedMotion: "reduce" });
  assert(
    await page.evaluate(() => parseFloat(getComputedStyle(document.querySelector(".theme-toggle")).transitionDuration) <= 0.00001),
    "reduced motion should make UI transitions effectively instant"
  );

  const axe = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa"]).analyze();
  const severe = axe.violations.filter((violation) => ["serious", "critical"].includes(violation.impact));
  assert(severe.length === 0, `axe serious/critical: ${severe.map((item) => item.id).join(", ")}`);

  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator("h1").waitFor();
  assert((await page.locator("h1").textContent()).includes("Test old webhook versions"), "offline reload should use the cached shell");
  assert(errors.length === 0, `desktop page errors: ${errors.join(" | ")}`);
  assert(externalRequests.length === 0, `ordinary browsing made external requests: ${externalRequests.join(", ")}`);
  await context.close();
  return { axeViolations: axe.violations.length };
}

async function runMobile(browser) {
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  page.on("console", (message) => {
    if (message.type() === "error") errors.push(message.text());
  });
  page.on("response", (response) => {
    if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`);
  });
  const demoUrl = new URL(url);
  demoUrl.searchParams.set("demo", "1");
  await page.goto(demoUrl.href, { waitUntil: "networkidle" });
  assert(await page.evaluate(() => document.documentElement.scrollWidth === 390), "390px viewport should not overflow horizontally");
  await page.locator(".result-summary").waitFor();
  assert(await page.locator("#demo-banner").isVisible(), "mobile demo banner should remain visible");
  assert(errors.length === 0, `mobile page errors: ${errors.join(" | ")}`);
  await context.close();
}

async function runRouting(browser) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();
  await page.goto(url, { waitUntil: "networkidle" });
  await page.getByRole("link", { name: "How it works" }).click();
  assert(await page.evaluate(() => location.hash === "#workflow"), "section link should use a real history entry");
  assert(await page.evaluate(() => document.activeElement?.id === "workflow-title"), "section route should focus its heading");
  await page.goBack();
  await page.waitForFunction(() => location.hash === "");
  assert(await page.evaluate(() => window.scrollY <= 5), "Back should restore the home position");
  const privacy = new URL("/privacy/", url);
  await page.goto(privacy.href, { waitUntil: "networkidle" });
  await page.waitForFunction(() => document.activeElement?.tagName === "H1");
  assert(await page.locator("h1").textContent() === "Privacy", "privacy route should focus its h1");
  for (const path of ["/privacy/", "/terms/", "/404.html", "/?demo=1"]) {
    await page.goto(new URL(path, url).href, { waitUntil: "networkidle" });
    const axe = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa"]).analyze();
    const severe = axe.violations.filter((violation) => ["serious", "critical"].includes(violation.impact));
    assert(severe.length === 0, `${path} axe serious/critical: ${severe.map((item) => item.id).join(", ")}`);
  }
  await context.close();
}

const browser = await chromium.launch();
try {
  const desktop = await runDesktop(browser);
  await runMobile(browser);
  await runRouting(browser);
  console.log(JSON.stringify({ url, desktop, mobile: "390px no-overflow", result: "passed" }));
} finally {
  await browser.close();
}
