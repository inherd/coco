use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

pub fn run() {
    let index_html = Asset::get("index.html").unwrap();
    println!("{:?}", std::str::from_utf8(index_html.as_ref()));

    for file in Asset::iter() {
        println!("{}", file.as_ref());
    }
}
