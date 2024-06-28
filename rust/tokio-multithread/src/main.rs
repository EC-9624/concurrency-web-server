use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current directory and construct the dist path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dist_path = current_dir.join("../../dist");
    let dist_path = dist_path
        .canonicalize()
        .expect("Failed to canonicalize path");

    println!("The 'dist' directory path is {}", dist_path.display());

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("Server listening on port 3000...");

    loop {
        let (stream, _) = listener.accept().await?;
        let dist_path = dist_path.clone();

        // Spawn a new task for each connection
        task::spawn(async move {
            if let Err(e) = handle_connection(stream, dist_path).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream, base_path: PathBuf) -> Result<(), std::io::Error> {
    let mut buffer = vec![0; 1024];
    stream.readable().await?;
    let n = stream.read(&mut buffer).await?; // Use `AsyncReadExt` to read from stream

    // Parse the HTTP request (simple parsing)
    let request_line = String::from_utf8_lossy(&buffer[..n]);
    let request_path = request_line
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    // Determine the file to serve
    let mut file_path = base_path.join(&request_path[1..]);
    if file_path.is_dir() {
        file_path = file_path.join("index.html");
    }
    if !file_path.exists() {
        file_path = base_path.join("index.html");
    }

    // Read the file content asynchronously
    let file_path_clone = file_path.clone(); // Clone file_path before moving it into the closure
    let contents = task::spawn_blocking(move || read_file(&file_path_clone)).await??;

    // Get the content type
    let content_type = get_content_type(file_path.extension().unwrap_or_default().to_str().unwrap_or(""));

    // Send the HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type,
        contents.len()
    );
    stream.write_all(response.as_bytes()).await?;
    stream.write_all(&contents).await?;
    stream.flush().await?;
    Ok(())
}

fn read_file(file_path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn get_content_type(extension: &str) -> &str {
    match extension {
        "html" => "text/html",
        "js" => "application/javascript",
        "css" => "text/css",
        "json" => "application/json",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "wav" => "audio/wav",
        "mp4" => "video/mp4",
        "woff" => "application/font-woff",
        "ttf" => "application/font-ttf",
        "eot" => "application/vnd.ms-fontobject",
        "otf" => "application/font-otf",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}
