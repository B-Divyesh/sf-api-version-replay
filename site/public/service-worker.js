const CACHE = "version-replay-shell-v4";
const SHELL = ["/", "/?demo=1", "/privacy/", "/terms/", "/404.html", "/version-specimen.webp", "/favicon.svg"];

async function cacheShell() {
  const cache = await caches.open(CACHE);
  await cache.addAll(SHELL);
  const documents = await Promise.all(
    SHELL.slice(0, 5).map(async (path) => {
      const response = await cache.match(path);
      return response ? response.clone().text() : "";
    })
  );
  const assets = new Set();
  for (const html of documents) {
    for (const match of html.matchAll(/(?:src|href)="([^"]+)"/g)) {
      const url = new URL(match[1], self.location.origin);
      if (url.origin === self.location.origin && /\.(?:css|js|webp|svg|png|woff2)$/.test(url.pathname)) {
        assets.add(url.pathname);
      }
    }
  }
  await cache.addAll([...assets]);
}

self.addEventListener("install", (event) => {
  event.waitUntil(cacheShell());
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((key) => key !== CACHE).map((key) => caches.delete(key))))
  );
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  if (event.request.method !== "GET" || new URL(event.request.url).origin !== self.location.origin) return;
  event.respondWith(
    caches.match(event.request, { ignoreVary: true }).then((cached) => {
      if (cached) return cached;
      return fetch(event.request).then((response) => {
        if (response.ok) {
          const copy = response.clone();
          caches.open(CACHE).then((cache) => cache.put(event.request, copy));
        }
        return response;
      }).catch(() => {
        if (event.request.mode === "navigate") return caches.match("/");
        return new Response("Offline asset unavailable", { status: 503, statusText: "Service Unavailable" });
      });
    })
  );
});
