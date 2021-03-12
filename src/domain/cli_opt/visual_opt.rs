use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "visual")]
pub struct VisualOpt {
    /// Debug mode
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    /// http server port
    #[structopt(long, short, parse(try_from_str), default_value = "8000")]
    pub port: String,

    /// project name
    #[structopt(long, short, parse(try_from_str))]
    pub name: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<SubVisualCommand>,
}

#[derive(StructOpt, Debug, Clone)]
pub enum SubVisualCommand {
    Export {
        /// output path
        #[structopt(long, short, parse(try_from_str), default_value = "coco_static")]
        output: String,

        #[structopt(long, short, parse(try_from_str))]
        name: Option<String>,
    },
}
