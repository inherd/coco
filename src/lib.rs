#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate tempdir;

pub mod app;
pub mod domain;
pub mod infrastructure;

use std::collections::HashMap;

lazy_static! {
    static ref COCO_CONFIG: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("dir", ".coco");
        m
    };
}

pub fn global_config(key: &str) -> &'static str {
    return COCO_CONFIG.get(key).unwrap();
}