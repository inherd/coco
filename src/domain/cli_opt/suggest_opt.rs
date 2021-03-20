use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "visual")]
pub struct SuggestOpt {
    /// Debug mode
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    /// Config file .yml
    #[structopt(short, long, default_value = "coco.yml")]
    pub config_file: String,
}
