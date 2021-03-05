use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "coco")]
pub struct CocoOpt {
    /// Debug mode
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    /// Config file .yml
    #[structopt(short, long, default_value = "coco.yml")]
    pub config_file: String,

    /// With all commits
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub commits: bool,

    /// With all branches
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub branches: bool,

    /// Set git commits scan years, default 1,
    #[structopt(long, short = "y", parse(try_from_str), default_value = "1.0")]
    pub git_years: f64,

    /// Scan file change list from git & cloc
    #[structopt(long, short, short = "f", parse(try_from_str), default_value = "false")]
    pub file_history: bool,

    /// With all tags
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub tags: bool,

    #[structopt(subcommand)]
    pub cmd: Option<CocoCommand>,
}

#[derive(StructOpt, Debug, Clone)]
pub enum CocoCommand {
    /// Create default coco.yml files
    Init,
    /// Download plugins from GitHub
    Plugins,
}

impl Default for CocoOpt {
    fn default() -> Self {
        CocoOpt {
            debug: false,
            config_file: "coco.yml".to_string(),
            commits: false,
            branches: false,
            git_years: 0.0,
            file_history: false,
            tags: false,
            cmd: None,
        }
    }
}
