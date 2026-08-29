import { chromium } from "playwright";
import AxeBuilder from "@axe-core/playwright";

const url = process.argv[2] ?? "http://127.0.0.1:4173/";
const origin = new URL(url).origin;

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function assertNoSevereAxe(page, label) {
  const axe = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa"]).analyze();
  const severe = axe.violations.filter((violation) => ["serious", "critical"].includes(violation.impact));
  assert(severe.length === 0, `${label} axe serious/critical: ${severe.map((item) => item.id).join(", ")}`);
  return axe.violations.length;
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

  const axeViolations = await assertNoSevereAxe(page, "desktop home");

  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator("h1").waitFor();
  assert((await page.locator("h1").textContent()).includes("Test old webhook versions"), "offline reload should use the cached shell");
  assert(errors.length === 0, `desktop page errors: ${errors.join(" | ")}`);
  assert(externalRequests.length === 0, `ordinary browsing made external requests: ${externalRequests.join(", ")}`);
  await context.close();
  return { axeViolations };
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
  const checks = [
    { label: "mobile home", href: new URL("/", url).href, selectors: [".terminal-recording pre"] },
    { label: "mobile demo", href: new URL("/?demo=1", url).href, selectors: [".terminal-recording pre", ".result-table-wrap"] }
  ];
  for (const check of checks) {
    await page.goto(check.href, { waitUntil: "networkidle" });
    if (check.label === "mobile demo") {
      await page.locator(".result-summary").waitFor();
      assert(await page.locator("#demo-banner").isVisible(), "mobile demo banner should remain visible");
    }
    assert(await page.evaluate(() => document.documentElement.scrollWidth === 390), `${check.label} should not overflow horizontally`);
    for (const selector of check.selectors) {
      const region = page.locator(selector);
      assert(await region.getAttribute("tabindex") === "0", `${check.label} ${selector} should be keyboard focusable`);
      assert(Boolean(await region.getAttribute("aria-label")), `${check.label} ${selector} needs an accessible name`);
      await region.focus();
      assert(await region.evaluate((element) => document.activeElement === element), `${check.label} ${selector} should receive focus`);
      const before = await region.evaluate((element) => element.scrollLeft);
      await page.keyboard.press("ArrowRight");
      await page.waitForTimeout(100);
      const after = await region.evaluate((element) => element.scrollLeft);
      assert(after > before, `${check.label} ${selector} should scroll from the keyboard`);
    }
    await assertNoSevereAxe(page, `${check.label} light treatment`);
    await page.locator(".theme-toggle").click();
    await assertNoSevereAxe(page, `${check.label} dark treatment`);
  }
  assert(errors.length === 0, `mobile page errors: ${errors.join(" | ")}`);
  await context.close();
  return { routes: checks.length, themesPerRoute: 2, viewport: "390x844" };
}

async function assertFullPageBack(browser, viewport, targetPath) {
  const context = await browser.newContext({ viewport });
  const page = await context.newPage();
  const home = new URL("/", url).href;
  await page.goto(home, { waitUntil: "networkidle" });
  await page.waitForFunction(() => document.activeElement?.id === "hero-title");
  const startingScroll = await page.evaluate(() => window.scrollY);
  await page.goto(new URL(targetPath, url).href, { waitUntil: "networkidle" });
  await page.waitForFunction(() => document.activeElement?.tagName === "H1");
  await page.goBack();
  await page.waitForFunction(() =>
    location.pathname === "/" &&
    !location.search &&
    document.activeElement?.id === "hero-title" &&
    document.querySelector(".route-status")?.textContent?.includes("Test old webhook versions")
  );
  const restoredScroll = await page.evaluate(() => window.scrollY);
  assert(Math.abs(restoredScroll - startingScroll) <= 5, `${targetPath} Back should preserve the home scroll position at ${viewport.width}px`);
  await context.close();
}

async function runRouting(browser) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const page = await context.newPage();
  await page.goto(url, { waitUntil: "networkidle" });
  for (let attempt = 0; attempt < 3; attempt += 1) {
    await page.getByRole("link", { name: "How it works" }).click();
    await page.waitForFunction(() => location.hash === "#workflow" && document.activeElement?.id === "workflow-title");
    assert(await page.evaluate(() => location.hash === "#workflow"), "section link should use a real history entry");
    await page.goBack();
    await page.waitForFunction(() => location.hash === "" && window.scrollY <= 5 && document.activeElement?.id === "hero-title");
    assert(await page.evaluate(() => window.scrollY <= 5), "Back should restore the home position");
    assert(await page.evaluate(() => document.activeElement?.id === "hero-title"), "Back should focus the home heading");
  }
  const privacy = new URL("/privacy/", url);
  await page.goto(privacy.href, { waitUntil: "networkidle" });
  await page.waitForFunction(() => document.activeElement?.tagName === "H1");
  assert(await page.locator("h1").textContent() === "Privacy", "privacy route should focus its h1");
  for (const path of ["/privacy/", "/terms/", "/404.html", "/?demo=1"]) {
    await page.goto(new URL(path, url).href, { waitUntil: "networkidle" });
    await assertNoSevereAxe(page, path);
  }
  await context.close();

  for (const viewport of [{ width: 1280, height: 800 }, { width: 390, height: 844 }]) {
    for (const path of ["/privacy/", "/?demo=1"]) {
      await assertFullPageBack(browser, viewport, path);
    }
  }
}

async function runExternalFragments(browser) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 } });
  const source = await context.newPage();
  await source.goto(url, { waitUntil: "networkidle" });
  const links = await source.locator('a[href^="https://"][href*="#"]').evaluateAll((anchors) =>
    [...new Set(anchors.map((anchor) => anchor.href))]
  );
  assert(links.length > 0, "expected at least one external fragment link");

  for (const href of links) {
    const target = await context.newPage();
    await target.goto(href, { waitUntil: "domcontentloaded" });
    const hash = new URL(href).hash;
    await target.waitForFunction((fragment) => {
      const id = decodeURIComponent(fragment.slice(1));
      return Boolean(
        document.getElementById(id) ||
        document.getElementById(`user-content-${id}`) ||
        [...document.querySelectorAll("a[href]")].some((anchor) => anchor.getAttribute("href") === fragment)
      );
    }, hash);
    await target.close();
  }
  await context.close();
}

const browser = await chromium.launch();
try {
  const desktop = await runDesktop(browser);
  const mobile = await runMobile(browser);
  await runRouting(browser);
  await runExternalFragments(browser);
  console.log(JSON.stringify({ url, desktop, mobile, routing: "hash and cross-document Back passed", result: "passed" }));
} finally {
  await browser.close();
}
