const http = require("http");

const port = process.env.PORT || 3000;

const server = http.createServer((req, res) => {
  res.writeHead(200);
  res.end(`Hello from process ID ${process.pid}`);
});

server.listen(port, () => {
  console.log(`Server process ID ${process.pid} listening on port ${port}`);
});
