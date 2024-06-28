use actix_files as fs;
use actix_web::{web, App, HttpServer};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let dist_path = current_dir.join("../../dist");
    let dist_path = dist_path
        .canonicalize()
        .expect("Failed to canonicalize path");

    println!("The 'dist' directory path is {}", dist_path.display());
    println!("Server listening on port 3000...");
    HttpServer::new(move || {
        App::new().service(
            fs::Files::new("/", &dist_path)
                .index_file("index.html")
                .default_handler(fs::NamedFile::open(dist_path.join("index.html")).unwrap()),
        )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
