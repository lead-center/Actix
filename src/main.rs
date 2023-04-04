use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Result, get};
use actix_web::rt::net::UnixListener;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../wasm/dist/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "../wasm/dist/index.html").index_file("index.html"))
    });

    

    let socket = std::env::var("SOCK");

    let server = if let Ok(socket) = socket {
        let listener = UnixListener::bind(socket).unwrap();
        listener
    } else {
        server = server.bind(("127.0.0.1", 8080))?;
        server
    }

    server.run().await
}
