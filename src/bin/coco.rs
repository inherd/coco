use coco::app::git_app::get_repo;

fn main() {
    let output = get_repo("https://github.com/phodal/coco.fixtures");
    println!("{:}", output);
}
