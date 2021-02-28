use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
    pub pure_data_type: String,
}

impl MemberInfo {
    pub fn new(name: &str, access: &str, data_type: String) -> Self {
        MemberInfo {
            name: name.to_string(),
            access: access.to_string(),
            data_type,
            pure_data_type: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub return_type: String,
    pub pure_return_type: String,
}

impl MethodInfo {
    pub fn new(name: &str, access: &str, return_type: String) -> Self {
        MethodInfo {
            name: name.to_string(),
            access: access.to_string(),
            return_type,
            pure_return_type: "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub id: i32,
    pub file: String,
    pub lang: String,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub methods: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(class_name: &str) -> Self {
        ClassInfo {
            name: class_name.to_string(),
            id: 0,
            file: "".to_string(),
            lang: "".to_string(),
            parents: vec![],
            members: vec![],
            methods: vec![],
        }
    }
}
