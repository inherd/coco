use handlebars::*;
use handlebars_fluent::*;
use serde_json::*;

simple_loader!(create_loader, "./locales/translate", "zh-CN");

pub struct Translator;

impl Translator {
    pub fn init(handlebars: &mut Handlebars) {
        let loader = create_loader();
        let helper = FluentHelper::new(loader);
        handlebars.register_helper("fluent", Box::new(helper));
    }

    pub fn render_page(handlebars: &Handlebars) -> String {
        let data = json!({"lang": "zh-CN"});
        handlebars
            .render_template("{{fluent \"physical-design\"}}", &data)
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::infrastructure::translator::Translator;
    use handlebars::*;
    use handlebars_fluent::*;

    simple_loader!(loader, "./locales/translate", "zh-CN");

    #[test]
    fn should_render_translator() {
        let mut handlebars = Handlebars::new();
        Translator::init(&mut handlebars);
        let string = Translator::render_page(&handlebars);
        assert_eq!("物理设计", string);
    }
}
