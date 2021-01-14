pub struct ClocLanguage {
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    pub reports: Vec<ClocDetail>,
}

pub struct ClocDetail {
    pub blanks: usize,
    pub code: usize,
    pub comments: usize,
    pub name: String,
}
