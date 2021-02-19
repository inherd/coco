#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate serde_derive;
extern crate regex;
extern crate serde;

pub mod dependency;
pub mod facet;
pub mod framework_detector;
pub mod lang;
