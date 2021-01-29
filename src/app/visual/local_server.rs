use std::borrow::Cow;
use std::fs;

use actix_web::body::Body;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::settings::Settings;
use actix_web::dev::Service;

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

#[get("/data/{project}/{coco_type}.json")] // <- define path parameters
pub fn data(req: HttpRequest) -> HttpResponse {
    let project: String = req.match_info().get("project").unwrap().parse().unwrap();
    let coco_type: String = req.match_info().query("coco_type").parse().unwrap();

    let project_file = format!("{}.json", project);
    let output_path = Settings::reporter_dir(Some(coco_type.as_str())).join(project_file);

    let content = fs::read_to_string(output_path.clone()).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(content.into_bytes());
}

pub async fn start(port: &str, project: &str) -> std::io::Result<()> {
    return HttpServer::new(|| {
        App::new()
            .service(web::resource("/").route(web::get().to(index)))
            .service(data)
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
            .service(web::resource("/public/{_:.*}").route(web::get().to(dist)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}
