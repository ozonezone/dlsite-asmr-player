self.addEventListener("fetch", (event) => {
  if (event.request.url.indexOf("/stream/") !== -1) {
    return false;
  }
  event.respondWith(fetch(event.request));
});
