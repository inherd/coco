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

        fs::write(file_path, content).expect("cannot write file");
    }

    export_reporter(&path, project);
}

fn export_reporter<P: AsRef<Path>>(path: &P, project: String) {
    let data_dir = path.as_ref().join("data");
    let _ = fs::create_dir_all(&data_dir);

    // git
    let git = Settings::git().join(format!("{}.json", project));
    let _ = fs::copy(git, &data_dir.join("git.json"));

    let commits = Settings::git().join(format!("{}-commits.json", project).as_str());
    let _ = fs::copy(commits, &data_dir.join("git-commits.json"));

    let tags = Settings::git().join(format!("{}-tags.json", project));
    let _ = fs::copy(tags, &data_dir.join("git-tags.json"));

    // file_history
    let file_history = Settings::git().join(format!("{}-file-history.json", project));
    let _ = fs::copy(file_history, &data_dir.join("git-file-history.json"));

    // cloc
    let cloc = Settings::cloc().join(format!("{}.json", project));
    let _ = fs::copy(cloc, &data_dir.join("cloc.json"));

    // struct analysis
    let structs = Settings::struct_dir().join(format!("{}.json", project));
    let _ = fs::copy(structs, &data_dir.join("struct-analysis.json"));
}
