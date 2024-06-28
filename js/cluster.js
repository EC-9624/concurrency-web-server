const cluster = require("cluster");
const http = require("http");
const fs = require("fs");
const path = require("path");

const port = process.env.PORT || 3000;
const distPath = path.join(__dirname, "..", "dist");

// If we're the master process, fork worker processes
if (cluster.isMaster) {
  const numWorkers = require("os").cpus().length - 1; // 10 cores

  console.log(
    `Master process ${process.pid} is running and spawning ${numWorkers} worker processes`
  );

  // Fork worker processes for each CPU core
  for (let i = 0; i < numWorkers; i++) {
    cluster.fork();
  }

  // Handle worker process exit events
  cluster.on("exit", (worker, code, signal) => {
    console.log(
      `Worker ${worker.process.pid} died with code ${code} and signal ${signal}`
    );
    console.log("Starting a new worker process");
    cluster.fork();
  });
} else {
  // Worker process - start an HTTP server
  const server = http.createServer((req, res) => {
    // Resolve the file path based on the request URL
    let filePath = path.join(
      distPath,
      req.url === "/" ? "index.html" : req.url
    );

    // Set the content type based on the file extension
    const extname = String(path.extname(filePath)).toLowerCase();
    const mimeTypes = {
      ".html": "text/html",
      ".js": "application/javascript",
      ".css": "text/css",
      ".json": "application/json",
      ".png": "image/png",
      ".jpg": "image/jpg",
      ".gif": "image/gif",
      ".svg": "image/svg+xml",
      ".wav": "audio/wav",
      ".mp4": "video/mp4",
      ".woff": "application/font-woff",
      ".ttf": "application/font-ttf",
      ".eot": "application/vnd.ms-fontobject",
      ".otf": "application/font-otf",
      ".wasm": "application/wasm",
    };

    const contentType = mimeTypes[extname] || "application/octet-stream";

    fs.readFile(filePath, (err, data) => {
      if (err) {
        if (err.code === "ENOENT") {
          // If file not found, serve the index.html file (for client-side routing)
          fs.readFile(path.join(distPath, "index.html"), (error, indexData) => {
            if (error) {
              res.writeHead(500);
              res.end(`Error reading file: ${error.message}`);
            } else {
              res.writeHead(200, { "Content-Type": "text/html" });
              res.end(indexData);
            }
          });
        } else {
          res.writeHead(500);
          res.end(`Error reading file: ${err.message}`);
        }
      } else {
        res.writeHead(200, { "Content-Type": contentType });
        res.end(data);
      }
    });
  });

  server.listen(port, () => {
    console.log(`Worker process ${process.pid}  listening on port ${port}`);
  });
}
