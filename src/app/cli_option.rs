#[derive(Debug, Clone)]
pub struct CocoCliOption {
    pub branches: bool,
}

impl Default for CocoCliOption {
    fn default() -> Self {
        CocoCliOption { branches: false }
    }
}
