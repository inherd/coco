use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "coco")]
pub struct CocoOpt {
    /// Activate debug mode
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    #[structopt(short, long, default_value = "coco.yml")]
    pub config_file: String,

    /// with Git Commits
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub commits: bool,

    /// with Git Branches
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub branches: bool,

    /// Set git commits scan years, default 1,
    #[structopt(long, short = "y", default_value = "1")]
    pub git_years: u64,

    /// with file history
    #[structopt(long, short, short = "f", parse(try_from_str), default_value = "false")]
    pub file_history: bool,

    /// with tags features
    #[structopt(short, long, parse(try_from_str), default_value = "false")]
    pub tags: bool,

    #[structopt(subcommand)] // Note that we mark a field as a subcommand
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
            git_years: 0,
            file_history: false,
            tags: false,
            cmd: None,
        }
    }
}
