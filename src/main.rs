use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Result, get};
#[cfg(unix)]
use actix_web::rt::net::UnixListener;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../../wasm/dist/index.html")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "../../wasm/dist/index.html").index_file("index.html"))
    });

    let socket = std::env::var("SOCK");

    if let Ok(socket) = socket {
        #[cfg(unix)]
        let listener = UnixListener::bind(socket).unwrap();
        server = server.bind_uds(socket)?;
        return server.run().await
    } else {
        server = server.bind(("127.0.0.1", 8080))?;
        return server.run().await
    }
}
