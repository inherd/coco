use fluent::{FluentBundle, FluentResource};
use rust_embed::RustEmbed;
use unic_langid::LanguageIdentifier;

use std::borrow::Cow;
use std::path::PathBuf;

pub struct Translator {
    lang: String,
    content: String,
}

#[derive(RustEmbed)]
#[folder = "locales/"]
struct LocalesAsset;

impl Translator {
    pub fn new(lang: &str) -> Translator {
        let path = PathBuf::from("translate").join(lang).join("suggest.ftl");
        let path_str = format!("{}", path.display());
        let file_content: Cow<'static, [u8]> = LocalesAsset::get(&path_str).unwrap();
        let content = std::str::from_utf8(file_content.as_ref()).expect("cannot read file");

        return Translator {
            lang: lang.to_string(),
            content: content.to_string(),
        };
    }

    pub fn translate(self, content: &str) -> String {
        let res = FluentResource::try_new(self.content).expect("Failed to parse an FTL string.");

        let langid: LanguageIdentifier = self.lang.parse().expect("Parsing language failed.");
        let mut bundle = FluentBundle::new(vec![langid]);

        bundle
            .add_resource(&res)
            .expect("Failed to add FTL resources to the bundle.");

        let msg = bundle.get_message(content).expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(&pattern, None, &mut errors);

        return value.to_string();
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::translator::Translator;

    #[test]
    pub fn should_translate_physical_design() {
        let translator = Translator::new("zh-CN");
        let string = translator.translate("physical-design");
        assert_eq!("物理设计", string);
    }
}
