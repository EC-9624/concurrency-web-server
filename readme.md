# Concurrency Web Server Benchmarks

Benchmark result for Conncurrency web server in Go, Java, NodeJs ,Rust using `wrk`

## Serving String "Hello, World!"

| Language | Test Type         | Requests/sec | Transfer/sec (MB) | Data Read (MB) | Total Requests |
| -------- | ----------------- | ------------ | ----------------- | -------------- | -------------- |
| GO       | net-http          | 121591.8     | 15.07             | 453.19         | 3655453        |
| GO       | tcp               | 5242.11      | 0.26              | 7.81           | 157407         |
| Java     | Threads           | 74117.93     | 6.29              | 189.08         | 2227572        |
| Java     | VirtualThreads    | 73121.78     | 6.21              | 186.53         | 2197579        |
| Java     | SingleThread      | 47538.18     | 4.03              | 121.16         | 1427437        |
| NodeJs   | Cluster           | 111067.4     | 18.32             | 550.25         | 3335124        |
| NodeJs   | SingleThread      | 97411.22     | 14.31             | 429.38         | 2923588        |
| Rust     | actix web         | 194266.05    | 24.08             | 725.07         | 5848375        |
| Rust     | multithreaded tcp | 35381.47     | 1.75              | 17.63          | 355470         |
| Rust     | tcp-singleThread  | 6456.7       | 0.32              | 9.63           | 194149         |

## Serving Static Files

| Language | Test Type      | Max Concurent | Requests/sec | Transfer/sec (MB) | Data Read (MB) | Total Requests |
| -------- | -------------- | ------------- | ------------ | ----------------- | -------------- | -------------- |
| Go       | Net-Http       | 1900          | 68129.48     | 36.9              | 1080           | 2047231        |
| Java     | Single Thread  | 1500          | 25482.41     | 13.8              | 414.6          | 765378         |
| Java     | multithreads   | 1400          | 62900.53     | 34.07             | 1024           | 1893371        |
| Java     | Virtual Thread | 1600          | 53881.12     | 29.19             | 860            | 1621701        |
| NodeJs   | single thread  | 1200          | 42484.87     | 25.69             | 771.48         | 1275960        |
| NodeJs   | Cluster        | 1800          | 44604.86     | 26.97             | 810.31         | 1340177        |
| Rust     | axtic-web      | 2800          | 36508.35     | 25.8              | 776.75         | 1099075        |
