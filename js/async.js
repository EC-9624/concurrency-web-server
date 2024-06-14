const http = require("http");

const port = process.env.PORT || 3000;

const requestHandler = async (req, res) => {
  res.writeHead(200);
  res.end(`Hello, World!`);
};

const server = http.createServer(async (req, res) => {
  try {
    await requestHandler(req, res);
  } catch (error) {
    res.writeHead(500);
    res.end("Internal Server Error");
  }
});

const startServer = async () => {
  server.listen(port, () => {
    console.log(`Server listening on port ${port}`);
  });
};

startServer().catch((error) => {
  console.error("Failed to start the server:", error);
});
