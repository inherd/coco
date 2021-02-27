use crate::coco_struct::ClassInfo;

pub trait PlantUml {
    fn render(&self) -> String;
}

pub struct PlantUmlRender;

impl PlantUmlRender {
    pub fn render(classes: &Vec<ClassInfo>) -> String {
        let mut rendered: Vec<String> = vec![];
        for clazz in classes {
            let mut members = vec![];
            for member in &clazz.members {
                members.push(format!("  {}{}\n", member.access, member.name))
            }
            let mut methods = vec![];
            let mut content = format!("{}", members.join(""));
            for method in &clazz.methods {
                methods.push(format!("  {}{}()\n", method.access, method.name))
            }
            content = format!("{}{}", content, methods.join(""));

            rendered.push(format!("class {{\n{}}}", content));
        }

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
    use crate::coco_struct::{ClassInfo, MemberInfo, MethodInfo};
    use crate::plantuml_render::PlantUmlRender;

    #[test]
    fn should_render_empty() {
        let classes = vec![];
        let str = PlantUmlRender::render(&classes);
        assert_eq!("@startuml\n\n\n\n@enduml", str);
    }

    #[test]
    fn should_render_single_empty_class() {
        let mut classes = vec![];
        let demo = ClassInfo::new("Demo");
        classes.push(demo);

        let str = PlantUmlRender::render(&classes);
        assert_eq!("@startuml\n\nclass {\n}\n\n@enduml", str);
    }

    #[test]
    fn should_render_member_method() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");

        let member = MemberInfo::new("demo", "-", "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-", "String".to_string());
        demo.methods.push(method);

        classes.push(demo);

        let str = PlantUmlRender::render(&classes);
        assert_eq!(
            "@startuml\n\nclass {\n  -demo\n  -method()\n}\n\n@enduml",
            str
        );
    }
}
