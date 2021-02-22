#[derive(Debug, Clone)]
pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
}

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub return_type: String,
}

impl MethodInfo {
    pub fn new(name: &str) -> Self {
        MethodInfo {
            name: name.to_string(),
            access: "".to_string(),
            return_type: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub id: i32,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub method: Vec<MethodInfo>,
}

impl ClassInfo {
    pub fn new(class_name: &str) -> Self {
        ClassInfo {
            name: class_name.to_string(),
            id: 0,
            parents: vec![],
            members: vec![],
            method: vec![],
        }
    }
}
