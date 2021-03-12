use dialoguer::{theme::ColorfulTheme, Select};
use webbrowser;

use structopt::StructOpt;

use coco::app::visual::{local_server, output_static};
use coco::domain::visual_opt::{SubVisualCommand, VisualOpt};
use coco::infrastructure::file_scanner;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt: VisualOpt = VisualOpt::from_args();

    let project = match opt.name {
        Some(proj) => proj.to_string(),
        None => select_project_prompt(),
    };

    if let Some(sub_cmd) = &opt.cmd {
        match sub_cmd {
            SubVisualCommand::Export { output } => {
                start_export_reporter(output, project.clone());
                return Ok(());
            }
        }
    }

    return start_local_server(project, opt.port.as_str()).await;
}

fn start_export_reporter(output: &String, project_name: String) {
    output_static::run(output, project_name);
}

async fn start_local_server(project: String, port: &str) -> std::io::Result<()> {
    let url = format!("http://127.0.0.1:{}", port);
    println!("start server: {}", url);

    open_url(&url);

    println!("project: {}", project);
    return local_server::start(port, project).await;
}

pub fn open_url(url: &str) {
    if let Err(err) = webbrowser::open(url) {
        println!("failure to open in browser: {}", err);
    }
}

pub fn select_project_prompt() -> String {
    let selections = file_scanner::lookup_projects();
    if selections.len() == 0 {
        panic!("Please run coco first!");
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("pick project")
        .default(0)
        .items(&selections[..])
        .interact()
        .expect("1. Windows Users need to run with Windows Shell, such as PowerShell");

    let project = selections[selection].clone();
    project
}
