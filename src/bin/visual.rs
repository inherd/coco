use clap::{App, Arg};
// use dialoguer::{theme::ColorfulTheme, Select};

use coco::app::visual::{local_server, output_static};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Coco Visual")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .subcommand(
            App::new("export")
                .about("export")
                .version(VERSION)
                .author("Inherd Group")
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .help("output path")
                        .takes_value(true),
                ),
        )
        .subcommand(
            App::new("server")
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

    // let selections = &["default", "coco.fixtures", "coco.fixtures2"];
    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .with_prompt("pick project")
    //     .default(0)
    //     .items(&selections[..])
    //     .interact()
    //     .unwrap();
    //
    // let project = selections[selection];

    if let Some(ref matches) = matches.subcommand_matches("export") {
        let mut path = "coco_static";
        if let Some(input) = matches.value_of("path") {
            path = input
        }

        // todo: make really output
        output_static::run(path);
    }

    if let Some(ref matches) = matches.subcommand_matches("server") {
        let mut port = "8000";
        if let Some(input) = matches.value_of("port") {
            port = input
        }

        println!("start server: http://127.0.0.1:{}", port);
        return local_server::start(port, "default").await;
    }

    Ok(())
}
