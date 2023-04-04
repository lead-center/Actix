use actix_web::{get, web, HttpRequest};
#[cfg(unix)]
use actix_web::{middleware, App, Error, HttpResponse, HttpServer};
use actix_files::Files;

#[cfg(unix)]
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

#[cfg(not(unix))]
fn main() {}