Bun.serve({
  hostname: "0.0.0.0", // 允许外部访问
  port: 8081,
  fetch(req) {
    return new Response("Hello World");
  },
});
