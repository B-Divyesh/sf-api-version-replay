import "./styles.css";

const themeButton = document.querySelector<HTMLButtonElement>(".theme-toggle")!;
const themeLabel = themeButton.querySelector<HTMLElement>(".theme-label")!;
function setTheme(theme: "light" | "dark", persist = true): void {
  document.documentElement.dataset.theme = theme;
  themeButton.setAttribute("aria-pressed", String(theme === "dark"));
  themeButton.setAttribute("aria-label", `Use ${theme === "dark" ? "light" : "dark"} theme`);
  themeLabel.textContent = theme === "dark" ? "Light" : "Dark";
  if (persist) try { localStorage.setItem("vr_theme", theme); } catch { /* optional */ }
}
let theme: "light" | "dark" = matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
try { const stored = localStorage.getItem("vr_theme"); if (stored === "light" || stored === "dark") theme = stored; } catch { /* optional */ }
setTheme(theme, false);
themeButton.addEventListener("click", () => setTheme(document.documentElement.dataset.theme === "dark" ? "light" : "dark"));

function focusPageHeading(): void {
  const heading = document.querySelector<HTMLElement>("main h1");
  if (!heading) return;
  heading.tabIndex = -1;
  heading.focus({ preventScroll: true });
  const status = document.querySelector<HTMLElement>(".route-status")!;
  status.textContent = "";
  requestAnimationFrame(() => {
    status.textContent = heading.textContent ?? "Page loaded";
  });
}

addEventListener("pageshow", () => requestAnimationFrame(focusPageHeading));
