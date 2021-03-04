#[macro_use]
extern crate lazy_static;

pub use coco_config::{CocoConfig, RepoConfig};
pub mod coco_config;

pub use coco_settings::Settings;
pub mod coco_settings;

pub use interface::PluginInterface;
pub mod interface;

pub use support::url_format;
pub mod support;
