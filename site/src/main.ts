import "./styles.css";
import { compareJson, type BrowserChange } from "./compare";

const byId = <T extends HTMLElement>(id: string): T => {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Missing #${id}`);
  return element as T;
};

const isDemo = new URL(location.href).searchParams.get("demo") === "1";
const oldInput = byId<HTMLTextAreaElement>("fixture-old");
const newInput = byId<HTMLTextAreaElement>("fixture-new");
const originalOld = oldInput.value;
const originalNew = newInput.value;
const resultPanel = byId<HTMLDivElement>("result-panel");
const compareButton = byId<HTMLButtonElement>("compare-button");

function changeRow(category: string, change: BrowserChange): HTMLTableRowElement {
  const row = document.createElement("tr");
  [category, change.kind, change.path, change.before ?? "—", change.after ?? "—"].forEach((value, index) => {
    const cell = document.createElement("td");
    cell.textContent = value;
    if (index === 1) cell.className = `change-${change.kind}`;
    row.append(cell);
  });
  return row;
}

function renderDiff(focusResult = false): void {
  resultPanel.setAttribute("aria-busy", "true");
  compareButton.disabled = true;
  compareButton.firstChild!.textContent = "Comparing… ";
  const delay = matchMedia("(prefers-reduced-motion: reduce)").matches ? 0 : 160;
  window.setTimeout(() => {
    try {
      if (!oldInput.value.trim() || !newInput.value.trim()) throw new Error("Both webhook fixtures need JSON.");
      const diff = compareJson(oldInput.value, newInput.value);
      const changes = [...diff.schema.map((change) => ["type", change] as const), ...diff.values.map((change) => ["value", change] as const)];
      resultPanel.replaceChildren();
      if (changes.length === 0) {
        resultPanel.innerHTML = '<div class="clean-result"><span aria-hidden="true">✓</span><p><strong>Fixtures match.</strong> No type or value changes were found.</p></div>';
      } else {
        const summary = document.createElement("div");
        summary.className = "result-summary";
        const headline = document.createElement("p");
        headline.textContent = "Contract changed";
        const count = document.createElement("span");
        count.textContent = `${diff.schema.length} type / ${diff.values.length} value`;
        summary.append(headline, count);
        const wrapper = document.createElement("div");
        wrapper.className = "result-table-wrap";
        const table = document.createElement("table");
        table.className = "result-table";
        table.innerHTML = "<thead><tr><th>Layer</th><th>Change</th><th>Path</th><th>Before</th><th>After</th></tr></thead>";
        const body = document.createElement("tbody");
        changes.forEach(([category, change]) => body.append(changeRow(category, change)));
        table.append(body);
        wrapper.append(table);
        resultPanel.append(summary, wrapper);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : "The fixtures could not be compared.";
      resultPanel.innerHTML = `<div class="error-result"><span aria-hidden="true">!</span><p><strong>Comparison stopped.</strong> ${message} Check commas, quotes, and closing braces.</p></div>`;
    } finally {
      resultPanel.setAttribute("aria-busy", "false");
      compareButton.disabled = false;
      compareButton.firstChild!.textContent = "Compare sample fixtures ";
      if (focusResult) {
        resultPanel.tabIndex = -1;
        resultPanel.focus();
      }
    }
  }, delay);
}

function resetDemo(focus = true): void {
  oldInput.value = originalOld;
  newInput.value = originalNew;
  if (isDemo) renderDiff(focus);
  else {
    resultPanel.innerHTML = '<div class="empty-result"><span aria-hidden="true">↳</span><p><strong>No comparison yet.</strong> Compare the fixtures to list type and value changes.</p></div>';
    if (focus) oldInput.focus();
  }
}

compareButton.addEventListener("click", () => renderDiff(true));
byId<HTMLButtonElement>("reset-button").addEventListener("click", () => resetDemo());

document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const value = button.dataset.copy ?? "";
    const label = button.querySelector<HTMLElement>(".copy-label");
    try {
      await navigator.clipboard.writeText(value);
      if (label) label.textContent = "Copied";
      byId("copy-status").textContent = "Install command copied to the clipboard.";
    } catch {
      if (label) label.textContent = "Select command";
      byId("copy-status").textContent = `Copy this command: ${value}`;
    }
    window.setTimeout(() => { if (label) label.textContent = "Copy install command"; }, 1800);
  });
});

const themeButton = document.querySelector<HTMLButtonElement>(".theme-toggle")!;
const themeLabel = themeButton.querySelector<HTMLElement>(".theme-label")!;
function setTheme(theme: "light" | "dark", persist: boolean): void {
  document.documentElement.dataset.theme = theme;
  themeButton.setAttribute("aria-pressed", String(theme === "dark"));
  themeButton.setAttribute("aria-label", `Use ${theme === "dark" ? "light" : "dark"} theme`);
  themeLabel.textContent = theme === "dark" ? "Light" : "Dark";
  if (persist) try { localStorage.setItem("vr_theme", theme); } catch { /* optional */ }
}

let initialTheme: "light" | "dark" = matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
if (!isDemo) try {
  const stored = localStorage.getItem("vr_theme");
  if (stored === "light" || stored === "dark") initialTheme = stored;
} catch { /* optional */ }
setTheme(initialTheme, false);
themeButton.addEventListener("click", () => setTheme(document.documentElement.dataset.theme === "dark" ? "light" : "dark", !isDemo));

const offlineLabel = document.querySelector<HTMLElement>(".offline-label")!;
function updateConnection(): void {
  offlineLabel.textContent = navigator.onLine ? "Available after this page loads" : "Offline · sample still works";
}
addEventListener("online", updateConnection);
addEventListener("offline", updateConnection);
updateConnection();

function focusHeading(target: HTMLElement): void {
  const heading = target.matches("h1, h2") ? target : target.querySelector<HTMLElement>("h1, h2");
  if (!heading) return;
  heading.tabIndex = -1;
  heading.focus({ preventScroll: true });
  document.querySelector<HTMLElement>(".route-status")!.textContent = heading.textContent ?? "Page section changed";
}

history.scrollRestoration = "manual";
history.replaceState({ scrollY: window.scrollY }, "");
document.querySelectorAll<HTMLAnchorElement>('a[href^="#"]').forEach((link) => {
  link.addEventListener("click", (event) => {
    const target = document.querySelector<HTMLElement>(link.hash);
    if (!target) return;
    event.preventDefault();
    history.replaceState({ scrollY: window.scrollY }, "");
    history.pushState({ scrollY: target.offsetTop }, "", link.hash);
    target.scrollIntoView({ behavior: matchMedia("(prefers-reduced-motion: reduce)").matches ? "auto" : "smooth" });
    focusHeading(target);
  });
});
addEventListener("popstate", (event) => {
  const y = typeof event.state?.scrollY === "number" ? event.state.scrollY : 0;
  document.documentElement.style.scrollBehavior = "auto";
  scrollTo(0, y);
  requestAnimationFrame(() => document.documentElement.style.removeProperty("scroll-behavior"));
  const target = location.hash ? document.querySelector<HTMLElement>(location.hash) : byId("hero-title");
  if (target) focusHeading(target);
});

if (isDemo) {
  document.body.classList.add("demo-mode");
  byId("demo-banner").hidden = false;
  document.querySelector<HTMLAnchorElement>('a[href="#workflow"]')!.href = "/#workflow";
  const hero = document.querySelector<HTMLElement>(".hero")!;
  hero.remove();
  document.querySelector<HTMLElement>(".proof-strip")!.remove();
  document.querySelector<HTMLElement>(".workflow")!.remove();
  const terminal = byId<HTMLElement>("cli-demo");
  terminal.classList.add("demo-primary");
  const oldTerminalHeading = byId<HTMLElement>("terminal-title");
  const heading = document.createElement("h1");
  heading.id = oldTerminalHeading.id;
  heading.textContent = "Replay the complete CLI sample";
  oldTerminalHeading.replaceWith(heading);
  terminal.setAttribute("aria-labelledby", heading.id);
  const bench = byId<HTMLElement>("bench");
  byId("main").prepend(terminal);
  bench.querySelector<HTMLElement>(".fixture-grid")!.before(resultPanel);
  document.title = "Demo — Version Replay";
  const demoDescription = "Inspect the completed CLI workflow and compare isolated sample webhook fixtures.";
  document.querySelector<HTMLMetaElement>('meta[name="description"]')!.content = demoDescription;
  document.querySelector<HTMLLinkElement>('link[rel="canonical"]')!.href = "https://api-version-replay.sociobot.in/?demo=1";
  document.querySelector<HTMLMetaElement>('meta[property="og:title"]')!.content = "Demo — Version Replay";
  document.querySelector<HTMLMetaElement>('meta[property="og:description"]')!.content = demoDescription;
  document.querySelector<HTMLMetaElement>('meta[name="twitter:title"]')!.content = "Demo — Version Replay";
  document.querySelector<HTMLMetaElement>('meta[name="twitter:description"]')!.content = demoDescription;
  byId<HTMLButtonElement>("demo-reset").addEventListener("click", () => resetDemo());
  renderDiff(false);
  requestAnimationFrame(() => focusHeading(heading));
}

if ("serviceWorker" in navigator) addEventListener("load", () => { void navigator.serviceWorker.register("/service-worker.js").catch(() => undefined); });
