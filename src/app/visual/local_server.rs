use std::borrow::Cow;
use std::fs;

use actix_web::body::Body;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use crate::settings::Settings;

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

#[get("/data/{coco_type}.json")]
pub fn data(req: HttpRequest, data: web::Data<ProjectData>) -> HttpResponse {
    let project = data.name;
    let coco_type: String = req.match_info().query("coco_type").parse().unwrap();

    let project_file = format!("{}.json", project);
    let output_path = Settings::reporter_dir(Some(coco_type.as_str())).join(project_file);

    println!("lookup file: {:?}", output_path.clone());
    let content = fs::read_to_string(output_path.clone()).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(content.into_bytes());
}

#[derive(Clone, Copy)]
pub struct ProjectData {
    pub name: &'static str,
}

pub async fn start(port: &str, project: &'static str) -> std::io::Result<()> {
    return HttpServer::new(move || {
        App::new()
            .data(ProjectData { name: project })
            .service(web::resource("/").route(web::get().to(index)))
            .service(data)
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
            .service(web::resource("/public/{_:.*}").route(web::get().to(dist)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}
