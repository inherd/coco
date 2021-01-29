use std::borrow::Cow;

use crate::settings::Settings;
use actix_web::body::Body;
use actix_web::{web, App, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::fs;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok()
                .content_type(from_path(path).first_or_octet_stream().as_ref())
                .body(body)
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

pub fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

pub fn dist(path: web::Path<String>) -> HttpResponse {
    handle_embedded_file(&path.0)
}

pub fn data(_path: web::Path<String>, project: String) -> HttpResponse {
    let file_name = format!("{}.json", project);
    let output_path = Settings::reporter_dir(Some("framework")).join(file_name);

    let content = fs::read_to_string(output_path.clone()).unwrap();
    HttpResponse::Ok()
        .content_type(from_path(output_path).first_or_octet_stream().as_ref())
        .body(content.into_bytes())
}

pub async fn start(port: &str, project: String) -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            // .service(
            //     web::resource("/data/{_:.*}").route(
            //         web::get().to(|path: web::Path<String>| data(path, project.to_string())),
            //     ),
            // )
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
            .service(web::resource("/public/{_:.*}").route(web::get().to(dist)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}
