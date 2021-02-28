use crate::coco_struct::ClassInfo;
use std::collections::HashMap;

pub trait PlantUml {
    fn render(&self) -> String;
}

pub struct PlantUmlRender;

impl PlantUmlRender {
    pub fn render(classes: &Vec<ClassInfo>) -> String {
        let mut rendered: Vec<String> = vec![];
        let mut deps: Vec<String> = vec![];

        let mut class_map: HashMap<String, bool> = HashMap::default();
        for clazz in classes {
            class_map.insert(clazz.name.clone(), true);
        }

        for clazz in classes {
            let mut dep_map: HashMap<String, String> = HashMap::default();

            let members = PlantUmlRender::render_member(&clazz, &mut dep_map);
            let mut content = format!("{}", members.join(""));

            let methods = PlantUmlRender::render_method(&clazz, &mut dep_map);
            content = format!("{}{}", content, methods.join(""));

            rendered.push(format!("class {} {{\n{}}}", clazz.name, content));

            for (callee, current_clz) in dep_map {
                if callee == current_clz {
                    continue;
                }

                if class_map.get(&callee).is_none() {
                    continue;
                }

                deps.push(format!("{} --> {}\n", current_clz, callee));
            }
        }

        format!(
            "@startuml\n\n{}\n{}\n@enduml",
            rendered.join("\n\n"),
            deps.join("")
        )
    }

    fn render_method(clazz: &&ClassInfo, dep_map: &mut HashMap<String, String>) -> Vec<String> {
        let mut methods = vec![];
        for method in &clazz.methods {
            if method.return_type.is_empty() {
                methods.push(format!("  {}{}()\n", method.access, method.name))
            } else {
                methods.push(format!(
                    "  {} {} {}()\n",
                    method.access, method.return_type, method.name
                ));

                dep_map.insert(method.return_type.clone(), clazz.name.clone());
            }
        }
        methods
    }

    fn render_member(clazz: &&ClassInfo, dep_map: &mut HashMap<String, String>) -> Vec<String> {
        let mut members = vec![];
        for member in &clazz.members {
            if member.data_type.is_empty() {
                members.push(format!("  {}{}\n", member.access, member.name))
            } else {
                members.push(format!(
                    "  {} {} {}\n",
                    member.access, member.data_type, member.name
                ));

                dep_map.insert(member.data_type.clone(), clazz.name.clone());
            }
        }
        members
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
        assert_eq!("@startuml\n\nclass Demo {\n}\n\n@enduml", str);
    }

    #[test]
    fn should_render_member_method() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");

        let member = MemberInfo::new("demo", "-", "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-", "Demo".to_string());
        demo.methods.push(method);

        classes.push(demo);

        let str = PlantUmlRender::render(&classes);
        assert_eq!(
            "@startuml\n\nclass Demo {\n  - String demo\n  - Demo method()\n}\n\n@enduml",
            str
        );
    }

    #[test]
    fn should_render_deps() {
        let mut classes = vec![];
        let mut demo = ClassInfo::new("Demo");
        let demo2 = ClassInfo::new("Demo2");

        let member = MemberInfo::new("demo", "-", "String".to_string());
        demo.members.push(member);

        let method = MethodInfo::new("method", "-", "Demo2".to_string());
        demo.methods.push(method);

        classes.push(demo);
        classes.push(demo2);

        let str = PlantUmlRender::render(&classes);
        assert_eq!(true, str.contains("Demo --> Demo2"));
        assert_eq!(false, str.contains("Demo --> String"));
    }
}
