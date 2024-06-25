use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (stream, _) = listener.accept().await?;

        task::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    Ok(())
}
