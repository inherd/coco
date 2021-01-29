use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("Coco Visual")
        .version(VERSION)
        .author("Inherd Group")
        .about("A DevOps Efficiency Analysis and Auto-suggestion Tool.")
        .arg(
            Arg::with_name("export")
                .short("e")
                .long("export")
                .help("export static files"),
        )
        .arg(
            Arg::with_name("server")
                .short("s")
                .long("server")
                .help("run visual server"),
        )
        .get_matches();

    if let Some(i) = matches.value_of("export") {
        println!("Export Static: {}", i);
    }

    if let Some(i) = matches.value_of("server") {
        println!("Run server: {}", i);
    }
}
