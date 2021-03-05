#[derive(Debug, Clone)]
pub struct CocoOpt {
    pub branches: bool,
    pub years: u64,
    pub file_history: bool,
    pub tags: bool,
}

impl Default for CocoOpt {
    fn default() -> Self {
        CocoOpt {
            branches: false,
            years: 0,
            file_history: false,
            tags: false,
        }
    }
}
