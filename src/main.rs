use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, Result, get, web};
#[cfg(not(windows))]
use actix_web::rt::net::UnixListener;
#[cfg(windows)]
use actix_web::rt::net::TcpListener;

#[get("/")]
#[cfg(not(windows))]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../wasm/dist/index.html")))
}

#[get("/")]
#[cfg(windows)]
async fn index() -> Result<HttpResponse> {
    dbg!("index hit");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("E:/dev/lead_center/signatorytraining/dist/index.html")))
}

#[cfg(not(windows))]
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
    };

    server.run().await
}

#[get("/article/{article_name}")]
async fn article(path: web::Path<String>) -> Result<HttpResponse> {
    let article_name = path.into_inner();

    
    let article_path = String::from("articles/") + &article_name;
    dbg!(&article_path);
    
    let md = markdown::file_to_html(&std::path::Path::new(&article_path)).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(md))
}

#[cfg(windows)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new(|| {
        App::new()
        .service(actix_web::web::scope("/signatorytraining").service(Files::new("/", "E:/dev/lead_center/signatorytraining/dist/index.html").index_file("index.html")))
        .service(article)
        .service(Files::new("articles", "articles").index_file("index.html"))
        .service(Files::new("/", "E:/dev/lead_center/signatorytraining/dist").index_file("index.html"))
    });

    let address = ("127.0.0.1", 8081);
    println!("Server started - listening on {}:{}", address.0, address.1);
    server.bind(address)?.run().await
    
}