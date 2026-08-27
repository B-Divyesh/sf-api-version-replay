import "./styles.css";
import { compareJson, type BrowserChange } from "./compare";

const byId = <T extends HTMLElement>(id: string): T => {
  const element = document.getElementById(id);
  if (!element) throw new Error(`Missing #${id}`);
  return element as T;
};

const originalOld = byId<HTMLTextAreaElement>("fixture-old").value;
const originalNew = byId<HTMLTextAreaElement>("fixture-new").value;
const resultPanel = byId<HTMLDivElement>("result-panel");
const compareButton = byId<HTMLButtonElement>("compare-button");

function changeRow(category: string, change: BrowserChange): HTMLTableRowElement {
  const row = document.createElement("tr");
  const values = [category, change.kind, change.path, change.before ?? "—", change.after ?? "—"];
  values.forEach((value, index) => {
    const cell = document.createElement("td");
    cell.textContent = value;
    if (index === 1) cell.className = `change-${change.kind}`;
    row.append(cell);
  });
  return row;
}

function renderDiff(): void {
  resultPanel.setAttribute("aria-busy", "true");
  compareButton.disabled = true;
  compareButton.firstChild!.textContent = "Comparing… ";
  const delay = window.matchMedia("(prefers-reduced-motion: reduce)").matches ? 0 : 160;
  window.setTimeout(() => {
    try {
      const oldText = byId<HTMLTextAreaElement>("fixture-old").value.trim();
      const newText = byId<HTMLTextAreaElement>("fixture-new").value.trim();
      if (!oldText || !newText) throw new Error("Both fixture specimens need JSON before they can be compared.");
      const diff = compareJson(oldText, newText);
      const changes = [
        ...diff.schema.map((change) => ["schema", change] as const),
        ...diff.values.map((change) => ["value", change] as const)
      ];
      resultPanel.replaceChildren();
      if (changes.length === 0) {
        const clean = document.createElement("div");
        clean.className = "clean-result";
        const icon = document.createElement("span");
        icon.setAttribute("aria-hidden", "true");
        icon.textContent = "✓";
        const copy = document.createElement("p");
        const strong = document.createElement("strong");
        strong.textContent = "Contracts match.";
        copy.append(strong, " No schema or primitive value differences found.");
        clean.append(icon, copy);
        resultPanel.append(clean);
        return;
      }

      const summary = document.createElement("div");
      summary.className = "result-summary";
      const headline = document.createElement("p");
      headline.textContent = "Contract changed";
      const count = document.createElement("span");
      count.textContent = `${diff.schema.length} schema / ${diff.values.length} value`;
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
    } catch (error) {
      const message = error instanceof Error ? error.message : "The fixture could not be compared.";
      const errorBox = document.createElement("div");
      errorBox.className = "error-result";
      const icon = document.createElement("span");
      icon.setAttribute("aria-hidden", "true");
      icon.textContent = "!";
      const copy = document.createElement("p");
      const strong = document.createElement("strong");
      strong.textContent = "Comparison stopped. ";
      copy.append(strong, `${message} Check commas, quotes, and closing braces, then try again.`);
      errorBox.append(icon, copy);
      resultPanel.replaceChildren(errorBox);
    } finally {
      resultPanel.setAttribute("aria-busy", "false");
      compareButton.disabled = false;
      compareButton.firstChild!.textContent = "Compare locally ";
    }
  }, delay);
}

compareButton.addEventListener("click", renderDiff);
byId<HTMLButtonElement>("reset-button").addEventListener("click", () => {
  byId<HTMLTextAreaElement>("fixture-old").value = originalOld;
  byId<HTMLTextAreaElement>("fixture-new").value = originalNew;
  resultPanel.innerHTML = '<div class="empty-result"><span aria-hidden="true">↳</span><p><strong>Bench reset.</strong> The original specimens are ready to compare.</p></div>';
  byId<HTMLTextAreaElement>("fixture-old").focus();
});

document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const value = button.dataset.copy ?? "";
    const label = button.querySelector<HTMLElement>(".copy-label");
    try {
      await navigator.clipboard.writeText(value);
      if (label) label.textContent = "Copied";
      byId("copy-status").textContent = "Install command copied to the clipboard.";
    } catch {
      if (label) label.textContent = "Select";
      byId("copy-status").textContent = `Copy this command: ${value}`;
    }
    window.setTimeout(() => {
      if (label) label.textContent = "Copy";
    }, 1800);
  });
});

const themeButton = document.querySelector<HTMLButtonElement>(".theme-toggle")!;
const themeLabel = themeButton.querySelector<HTMLElement>(".theme-label")!;

function setTheme(theme: "light" | "dark"): void {
  document.documentElement.dataset.theme = theme;
  themeButton.setAttribute("aria-pressed", String(theme === "dark"));
  themeButton.setAttribute("aria-label", `Use ${theme === "dark" ? "light" : "dark"} theme`);
  themeLabel.textContent = theme === "dark" ? "Light" : "Dark";
  try {
    localStorage.setItem("vr_theme", theme);
  } catch {
    // Theme persistence is optional when storage is unavailable.
  }
}

let storedTheme: string | null = null;
try {
  storedTheme = localStorage.getItem("vr_theme");
} catch {
  storedTheme = null;
}
setTheme(
  storedTheme === "light" || storedTheme === "dark"
    ? storedTheme
    : window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light"
);
themeButton.addEventListener("click", () => {
  setTheme(document.documentElement.dataset.theme === "dark" ? "light" : "dark");
});

const offlineLabel = document.querySelector<HTMLElement>(".offline-label")!;
function updateConnection(): void {
  offlineLabel.textContent = navigator.onLine
    ? "Ready offline after first visit"
    : "Offline · comparison bench still works";
}
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
updateConnection();

const PRODUCT = "api-version-replay";
const LICENSE_KEY = `sb_license:${PRODUCT}`;
const VERDICT_KEY = `sb_license_verdict:${PRODUCT}`;
const BILLING_BASE = "https://api.sociobot.in";
const licenseStatus = byId<HTMLParagraphElement>("license-status");
const tokenInput = byId<HTMLInputElement>("license-token");

interface LicenseVerdict {
  valid: boolean;
  reason: string;
  expires_at?: string | null;
  checkedAt?: number;
}

function showLicense(verdict: LicenseVerdict): void {
  licenseStatus.replaceChildren();
  if (verdict.valid) {
    licenseStatus.textContent = "Pro license active on this browser. Use the same token with `vr license activate`.";
    licenseStatus.dataset.state = "valid";
    return;
  }
  licenseStatus.textContent = `License no longer active (${verdict.reason}). `;
  const link = document.createElement("a");
  link.href = `${BILLING_BASE}/api/v1/products/${PRODUCT}/checkout`;
  link.textContent = "Buy a new license.";
  licenseStatus.append(link);
  licenseStatus.dataset.state = "invalid";
}

async function verifyLicense(token: string): Promise<void> {
  licenseStatus.textContent = "Verifying license…";
  try {
    const response = await fetch(
      `${BILLING_BASE}/api/v1/products/${PRODUCT}/verify?license=${encodeURIComponent(token)}`,
      { headers: { accept: "application/json" } }
    );
    if (!response.ok) throw new Error(`verification returned ${response.status}`);
    const verdict = (await response.json()) as LicenseVerdict;
    const cached = { ...verdict, checkedAt: Date.now() };
    localStorage.setItem(VERDICT_KEY, JSON.stringify(cached));
    showLicense(cached);
  } catch {
    licenseStatus.textContent = navigator.onLine
      ? "License verification is unavailable. The free tools remain ready; try again shortly."
      : "You are offline. The free tools remain ready; reconnect to verify a new license.";
  }
}

let savedToken: string | null = null;
let cachedVerdict: LicenseVerdict | null = null;
try {
  const currentUrl = new URL(window.location.href);
  const returnedToken = currentUrl.searchParams.get("license");
  if (returnedToken) {
    localStorage.setItem(LICENSE_KEY, returnedToken);
    currentUrl.searchParams.delete("license");
    history.replaceState({}, "", `${currentUrl.pathname}${currentUrl.search}${currentUrl.hash}`);
  }
  savedToken = returnedToken ?? localStorage.getItem(LICENSE_KEY);
  const rawVerdict = localStorage.getItem(VERDICT_KEY);
  cachedVerdict = rawVerdict ? (JSON.parse(rawVerdict) as LicenseVerdict) : null;
  if (cachedVerdict?.valid) showLicense(cachedVerdict);
  const verdictAge = Date.now() - (cachedVerdict?.checkedAt ?? 0);
  if (savedToken && (returnedToken || verdictAge >= 86_400_000)) void verifyLicense(savedToken);
} catch {
  licenseStatus.textContent = "Browser storage is unavailable. Paste a token to verify it for this visit.";
}

byId<HTMLFormElement>("restore-form").addEventListener("submit", (event) => {
  event.preventDefault();
  const token = tokenInput.value.trim();
  if (!token) return;
  try {
    localStorage.setItem(LICENSE_KEY, token);
  } catch {
    // Verification still works for this visit.
  }
  tokenInput.value = "";
  void verifyLicense(token);
});

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    void navigator.serviceWorker.register("/service-worker.js").catch(() => undefined);
  });
}
