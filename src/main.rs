use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, HttpRequest, Result, get, web};
#[cfg(not(windows))]
use actix_web::rt::net::UnixListener;
#[cfg(windows)]
use actix_web::rt::net::TcpListener;


#[cfg(not(windows))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    match std::env::var("SOCK") {
        Ok(s) => {
            dbg!(&s);
            HttpServer::new(|| {
                App::new()
                    .service(Files::new("/", "/home/l/le/lead/wasm/dist").index_file("index.html"))
            })
            .bind_uds(s)?
            .workers(1)
            .run()
            .await;
        },
        Err(e) => {
            dbg!(e);
        }
    };

    Ok(())
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
        .service(Files::new("/", "../signatorytraining/dist").index_file("index.html"))
    });

    let address = ("127.0.0.1", 8081);
    println!("Server started - listening on http://{}:{}", address.0, address.1);
    server.bind(address)?.run().await
    
}