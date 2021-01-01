use serde::{Deserialize, Serialize};
use super::cept::*;
use super::editor::*;





#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Link {
    pub code: String,
    pub target: String,
}

impl Link {
    pub fn new(code: &str, target: &str) -> Self {
        Self { code: code.to_owned(), target: target.to_owned() }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Meta {
    pub publisher_name: Option<String>,
    pub clear_screen: Option<bool>,
    pub cls2: Option<bool>,
    pub parallel_mode: Option<bool>,
    pub links: Option<Vec<Link>>,
    pub publisher_color: Option<u8>,
    pub inputs: Option<Inputs>,
    pub palette: Option<String>,
    pub include: Option<String>,
    pub autoplay: Option<bool>,
}

impl Meta {
    pub fn merge(&mut self, other: Meta) {
        if other.publisher_name.is_some() {
            self.publisher_name = other.publisher_name;
        }
        if other.clear_screen.is_some() {
            self.clear_screen = other.clear_screen;
        }
        if other.cls2.is_some() {
            self.cls2 = other.cls2;
        }
        if other.parallel_mode.is_some() {
            self.parallel_mode = other.parallel_mode;
        }
        if other.links.is_some() {
            self.links = other.links;
        }
        if other.publisher_color.is_some() {
            self.publisher_color = other.publisher_color;
        }
        if other.inputs.is_some() {
            self.inputs = other.inputs;
        }
    }
}

pub struct Page {
    pub cept: Cept,
    pub meta: Meta,
}

impl Page {
    pub fn new(meta: Meta) -> Self {
        Self {
            cept: Cept::new(),
            meta: meta,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Palette {
    pub palette: Vec<String>,
    pub start_color: Option<u8>,
}

