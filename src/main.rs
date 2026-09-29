use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Response, Server, StatusCode};

fn get_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        _ => "application/octet-stream",
    }
}

fn main() {
    let port = env::var("SERVER_PORT")
        .or_else(|_| env::var("PORT"))
        .unwrap_or_else(|_| "8080".to_string());

    let addr = format!("0.0.0.0:{}", port);
    println!("==============================================");
    println!("  🐸 BOUNCY KOEN RUST SERVER ACTIEF!         ");
    println!("  🌐 Luistert op http://{}", addr);
    println!("==============================================");

    let server = Server::http(&addr).expect("Kon server niet starten");

    for request in server.incoming_requests() {
        let url_path = request.url().split('?').next().unwrap_or("/");
        let clean_path = if url_path == "/" || url_path.is_empty() {
            "index.html"
        } else {
            url_path.trim_start_matches('/')
        };

        let file_path = PathBuf::from(clean_path);

        // Prevent path traversal
        if clean_path.contains("..") || !file_path.exists() || file_path.is_dir() {
            let res = Response::from_string("404 Niet Gevonden")
                .with_status_code(StatusCode(404));
            let _ = request.respond(res);
            continue;
        }

        match File::open(&file_path) {
            Ok(file) => {
                let mime = get_mime(&file_path);
                let header = Header::from_bytes(&b"Content-Type"[..], mime.as_bytes()).unwrap();
                let res = Response::from_file(file).with_header(header);
                let _ = request.respond(res);
            }
            Err(_) => {
                let res = Response::from_string("500 Server Fout")
                    .with_status_code(StatusCode(500));
                let _ = request.respond(res);
            }
        }
    }
}
