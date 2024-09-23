import { createServer } from "node:http";

const server = createServer((req, res) => {
  const body = [];

  req.on("data", (chunk) => {
    body.push(chunk);
  });
  req.on("end", () => {
    console.log("request body: ", body.flat());
    res.writeHead(200, {
      "Content-Type": "application/json",
      "Access-Control-Allow-Origin": "*",
      "Access-Control-Allow-Methods": "GET, POST, PUT, DELETE",
      "Access-Control-Allow-Headers": "*",
    });
    res.end(
      JSON.stringify({
        data: "Hello World!",
      }),
    );
  });
});

server.listen(8123);
console.log("Server running at https://localhost:8123/");
