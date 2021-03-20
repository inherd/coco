use structopt::StructOpt;

use coco::app::suggest::suggester::Suggester;
use coco::domain::SuggestOpt;
use coco::infrastructure::file_scanner;
use core_model::CocoConfig;

fn main() {
    let opt: SuggestOpt = SuggestOpt::from_args();

    let config_file = &opt.config_file;
    let _config = CocoConfig::from_file(config_file);

    let projects = file_scanner::lookup_projects();
    for project in projects {
        Suggester::run(project);
    }
}
