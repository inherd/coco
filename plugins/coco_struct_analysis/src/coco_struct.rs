pub struct MemberInfo {
    pub name: String,
    pub access: String,
    pub data_type: String,
}

pub struct MethodInfo {
    pub name: String,
    pub access: String,
    pub return_type: String,
}

pub struct ClassInfo {
    pub name: String,
    pub id: i32,
    pub parents: Vec<String>,
    pub members: Vec<MemberInfo>,
    pub method: Vec<MethodInfo>,
}
