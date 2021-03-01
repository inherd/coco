use core_model::Settings;
use rust_embed::RustEmbed;
use std::fs;
use std::path::Path;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

pub fn run<P: AsRef<Path>>(path: P, project: String) {
    for file in Asset::iter() {
        let file_name = format!("{}", file.as_ref());
        let file_path = &path.as_ref().join(file.as_ref());

        let content = Asset::get(&file_name).unwrap();
        let _ = fs::create_dir_all(&file_path.parent().unwrap());

        println!("write to file: {}", file_path.display());
        fs::write(file_path, content).expect("cannot write file");
    }

    let git = Settings::git().join(format!("{}.json", project));
    let _ = fs::copy(git, &path.as_ref().join("data").join("git.json"));

    let commits = Settings::git().join(format!("{}-commits.json", project).as_str());
    let _ = fs::copy(
        commits,
        &path.as_ref().join("data").join("git-commits.json"),
    );

    let tags = Settings::git().join(format!("{}-tags.json", project));
    let _ = fs::copy(tags, &path.as_ref().join("data").join("git-tags.json"));

    let cloc = Settings::cloc().join(format!("{}.json", project));
    let _ = fs::copy(cloc, &path.as_ref().join("data").join("cloc.json"));

    let structs = Settings::struct_analysis().join(format!("{}.json", project));
    let _ = fs::copy(
        structs,
        &path.as_ref().join("data").join("struct-analysis.json"),
    );
}
