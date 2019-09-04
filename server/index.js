addEventListener('fetch', event => {
  event.respondWith(handle(event.request));
})

async function handle(request) {
  try {
    const {pathname} = new URL(request.url);
    const server = await import("./pkg");
    const res = new Response(server.handle(pathname));
    res.headers.set("Access-Control-Allow-Origin", "*");
    return res;
  } catch (e) {
    return new Response(e.stack);
  }
}
