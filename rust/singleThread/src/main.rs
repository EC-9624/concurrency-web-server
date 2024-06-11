use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("server listening on port 7878....");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        hello_handler(stream);
    }
}

fn hello_handler(mut stream: TcpStream) {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
