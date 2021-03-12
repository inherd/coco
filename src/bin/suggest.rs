use structopt::StructOpt;

use coco::domain::SuggestOpt;

fn main() {
    let opt: SuggestOpt = SuggestOpt::from_args();

    println!("found config file: {:?}", opt);
}
