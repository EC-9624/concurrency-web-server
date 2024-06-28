use std::{

    env,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

fn main() {

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dist_path = current_dir.join("../../dist");
    let dist_path = dist_path
        .canonicalize()
        .expect("Failed to canonicalize path");

    println!("The 'dist' directory path is {}", dist_path.display());

    let listener = TcpListener::bind("127.0.0.1:3000").expect("Could not bind to port 3000");
    println!("Server listening on port 3000...");


    // Accept connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                let dist_path = dist_path.clone();
                // Spawn a new thread for each connection
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, dist_path) {

                        eprintln!("Failed to handle connection: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}


fn handle_connection(mut stream: TcpStream, base_path: PathBuf) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    // Parse the HTTP request (simple parsing)
    let request_line = String::from_utf8_lossy(&buffer[..]);
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

    // Read the file content
    let mut file = match File::open(&file_path) {
        Ok(file) => file,
        Err(_) => {
            send_404(&mut stream)?;
            return Ok(());
        }
    };
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    // Get the content type
    let content_type = get_content_type(
        file_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or(""),
    );

    // Send the HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
        content_type,
        contents.len()
    );
    stream.write_all(response.as_bytes())?;
    stream.write_all(&contents)?;
    stream.flush()?;
    Ok(())
}

fn send_404(stream: &mut TcpStream) -> std::io::Result<()> {
    let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
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
