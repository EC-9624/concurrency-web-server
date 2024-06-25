const cluster = require("cluster");
const http = require("http");

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
  const port = process.env.PORT || 3000;
  // Worker process - start an HTTP server
  const server = http.createServer((req, res) => {
    res.writeHead(200);
    res.end(`Hello from worker process ${process.pid}`);
  });

  server.listen(port, () => {
    console.log(`Worker process ${process.pid} listening on port ${port}`);
  });
}
