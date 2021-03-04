use std::borrow::Cow;
use std::fs;

use actix_web::body::Body;
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use core_model::Settings;

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
    let project = data.name.clone();
    return lookup_coco_reporter(req, String::from_utf8(project).unwrap().as_str());
}

#[get("/data/{project}/{coco_type}.json")]
pub fn api(req: HttpRequest) -> HttpResponse {
    let project: String = req.match_info().get("project").unwrap().parse().unwrap();
    return lookup_coco_reporter(req, project.as_str());
}

fn lookup_coco_reporter(req: HttpRequest, project: &str) -> HttpResponse {
    let mut coco_type: String = req.match_info().query("coco_type").parse().unwrap();

    let mut project_file = format!("{}.json", project);
    // todo: temp way for pass cases
    if coco_type.ends_with("-commits") {
        coco_type = "git".to_string();
        project_file = format!("{}-commits.json", project);
    } else if coco_type.ends_with("-tags") {
        coco_type = "git".to_string();
        project_file = format!("{}-tags.json", project);
    } else if coco_type.ends_with("-file-history") {
        coco_type = "git".to_string();
        project_file = format!("{}-file-history.json", project);
    }

    let output_path = Settings::reporter(Some(coco_type.as_str())).join(project_file);

    println!("lookup file: {:?}", output_path);
    let content = fs::read_to_string(output_path).unwrap();

    return HttpResponse::Ok()
        .content_type("application/json")
        .body(content.into_bytes());
}

#[derive(Clone)]
pub struct ProjectData {
    pub name: Vec<u8>,
}

pub async fn start(port: &str, project: String) -> std::io::Result<()> {
    return HttpServer::new(move || {
        App::new()
            .data(ProjectData {
                name: project.as_bytes().to_vec(),
            })
            .service(web::resource("/").route(web::get().to(index)))
            // todo: add config api
            .service(data)
            .service(api)
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
            .service(web::resource("/public/{_:.*}").route(web::get().to(dist)))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_index_get() {
        let app = App::new()
            .data(ProjectData {
                name: Vec::from("default"),
            })
            .service(web::resource("/").route(web::get().to(index)))
            .service(data)
            .service(api)
            .service(web::resource("/{_:.*}").route(web::get().to(dist)))
            .service(web::resource("/public/{_:.*}").route(web::get().to(dist)));

        let mut app = test::init_service(app).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }
}
