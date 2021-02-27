use crate::coco_struct::ClassInfo;

pub trait PlantUml {
    fn render(&self) -> String;
}

pub struct PlantUmlRender;

impl PlantUmlRender {
    pub fn render(_classes: Vec<ClassInfo>) -> String {
        let rendered: Vec<String> = vec![];
        let dep: Vec<String> = vec![];

        format!(
            "@startuml\n\n{}\n{}\n@enduml",
            rendered.join("\n\n"),
            dep.join("\n\n")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::plantuml_render::PlantUmlRender;

    #[test]
    fn should_render_empty() {
        let classes = vec![];
        let str = PlantUmlRender::render(classes);
        assert_eq!("@startuml\n\n\n\n@enduml", str);
    }
}
