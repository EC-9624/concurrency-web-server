use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
extern crate futures;
use futures::executor::ThreadPoolBuilder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // bind listener

    let mut pool_builder = ThreadPoolBuilder::new();
    pool_builder.pool_size(64);
    let pool = pool_builder.create().expect("couldn't create threadpool");

    // Listen for an incoming connection.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        // spawning each connection in a new thread asynchronously
        pool.spawn_ok(async {
            handle_connection(stream).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";

    stream.write(response.as_bytes()).unwrap(); // write response
    stream.flush().unwrap();
}
