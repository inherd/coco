use actix_web::{web, App, HttpServer};
use clap::App as ClapApp;
use clap::Arg;
use coco::app::visual::local_server;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = ClapApp::new("Coco Visual")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .arg(
            Arg::with_name("export")
                .short("e")
                .long("export")
                .help("export static files")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .help("run visual server")
                .takes_value(true),
        )
        .subcommand(
            ClapApp::new("server")
                .about("server")
                .version(VERSION)
                .author("Inherd Group")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .help("http server port")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(i) = matches.value_of("export") {
        println!("Export Static: {}", i);
    }

    if let Some(ref matches) = matches.subcommand_matches("server") {
        let mut port = "8000";
        if let Some(input) = matches.value_of("port") {
            port = input
        }

        println!("start server: http://127.0.0.1:{}", port);

        return HttpServer::new(|| {
            App::new()
                .service(web::resource("/").route(web::get().to(local_server::index)))
                .service(web::resource("/{_:.*}").route(web::get().to(local_server::dist)))
        })
        .bind(format!("127.0.0.1:{}", port))?
        .run()
        .await;
    }

    Ok(())
}
