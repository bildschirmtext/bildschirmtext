use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};
use crate::cept_page;

use super::cept::*;
use super::editor::*;
use super::session::*;
use super::staticp::*;
use super::sysmsg::*;

// how many seconds does pal/char transmission have to take
// until we show the SH291 message
const SH291_THRESHOLD_SEC: usize = 2;
const BAUD_RATE: usize = 1200;

#[derive(Serialize, Deserialize)]
pub struct Palette {
    pub palette: Vec<String>,
    pub start_color: Option<u8>,
}

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

#[derive(Default)]
#[derive(Serialize, Deserialize)]
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

    pub fn construct_page_cept(&self, client_state: &mut ClientState, pageid: &PageId) -> Cept {
        let mut cept;
        cept = self.cept_preamble_from_meta(client_state, pageid);
        cept += self.cept_main_from_page(client_state, pageid);
        cept
    }

    //
    fn cept_preamble_from_meta(&self, client_state: &mut ClientState, pageid: &PageId) -> Cept {
        let mut cept = Cept::new();

        cept.hide_cursor();

        let clear_screen = self.meta.clear_screen;

        if clear_screen == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            client_state.include = None;
        }

        let mut cept_palette = None;
        let mut cept_include = None;
        if let Some(basedir) = find_basedir(pageid) {
            let basedir = basedir.0;
            cept_palette = load_palette(self.meta.palette.as_ref(), &basedir, client_state);
            cept_include = load_include(self.meta.include.as_ref(), clear_screen, &basedir, client_state);
        } else {
            println!("ERROR: basedir not found!");
        }

        if let Some(cept_palette) = cept_palette {
            cept.add_raw(cept_palette.data());
        }
        if let Some(cept_include) = cept_include {
            cept.add_raw(cept_include.data());
        }

        // If the include data is large and the connection is slow, the system may
        // appear frozen, so in this case, we show a message indicating that something
        // is going on.
        if cept.data().len() > (BAUD_RATE / 9) * SH291_THRESHOLD_SEC {
            cept = create_sysmsg(&SysMsg::new(SysMsgCode::TransferringPage)) + cept;
        }
        cept
    }

    fn cept_main_from_page(&self, client_state: &mut ClientState, pageid: &PageId) -> Cept {
        let mut cept = Cept::new();

        if self.meta.cls2 == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            client_state.include = None;
        }

        Self::headerfooter(&mut cept, pageid, self.meta.publisher_name.as_deref(), self.meta.publisher_color.unwrap());

        if self.meta.parallel_mode == Some(true) {
            cept.parallel_mode();
        }

        cept.add_raw(self.cept.data());
        cept.serial_limited_mode();

        Self::headerfooter(&mut cept, pageid, self.meta.publisher_name.as_deref(), self.meta.publisher_color.unwrap());

        cept.sequence_end_of_page();

        cept
    }


    pub fn headerfooter(cept: &mut Cept, pageid: &PageId, publisher_name: Option<&str>, publisher_color: u8) {
        let mut hide_price = false;
        let mut publisher_name = publisher_name;

        let hide_header_footer = if let Some(p) = publisher_name {
            // Early screenshots had a two-line publisher name with
            // the BTX logo in it for BTX-internal pages. Some .meta
            // files still reference this, but we should remove this.
            publisher_name = if p == "!BTX" {
                hide_price = true;
                Some("Bildschirmtext")
            } else {
                if p.len() > 30 {
                    Some(&p[..30])
                } else {
                    Some(p)
                }
            };
            false
        } else {
            true
        };

        cept.set_res_40_24();
        cept.set_cursor(23, 1);
        cept.unprotect_line();
        cept.set_line_fg_color_simple(12);
        cept.parallel_limited_mode();
        cept.set_cursor(24, 1);
        cept.unprotect_line();
        cept.add_raw(b" \x08");
        cept.clear_line();
        cept.cursor_home();
        cept.unprotect_line();
        cept.add_raw(b" \x08");
        cept.clear_line();
        cept.serial_limited_mode();
        cept.set_cursor(24, 1);
        cept.set_fg_color(8);
        cept.add_raw(b"\x08");
        cept.code_9d();
        cept.add_raw(b"\x08");
        cept.set_fg_color_optimized(publisher_color);
        cept.set_cursor(24, 19);
        if !hide_header_footer {
            cept.add_str(&format!("{pageid:>width$}", pageid=pageid.to_string(), width=22));
        }
        cept.cursor_home();
        cept.set_palette(1);
        cept.set_fg_color(8);
        cept.add_raw(b"\x08");
        cept.code_9d();
        cept.add_raw(b"\x08");
        cept.set_fg_color_optimized(publisher_color);
        cept.add_raw(b"\r");
        if !hide_header_footer & !hide_price {
            cept.add_str(publisher_name.unwrap());
            cept.set_cursor(1, 31);
            cept.add_raw(b"  ");
            cept.add_str(&format_currency(0)); // TODO: price
        }
        cept.cursor_home();
        cept.set_palette(0);
        cept.protect_line();
        cept.add_raw(b"\n");
    }
}

pub fn format_currency(price: u32) -> String {
    format!("DM  {},{:02}", price / 100, price % 100)
}

fn resource_filename(basedir: &str, resource_name: &str, extention: &str) -> String {
    let mut filename = basedir.to_owned();
    filename += resource_name;
    filename.push('.');
    filename += extention;
    filename
}

fn load_palette(palette_name: Option<&String>, basedir: &str, client_state: &mut ClientState) -> Option<Cept> {
    if let Some(palette_name) = palette_name {
        let filename = resource_filename(basedir, palette_name, "pal");
        if Some(filename.clone()) != client_state.palette {
            client_state.palette = Some(filename.clone());
            println!("loading: {}", filename);
            if let Ok(f) = File::open(&filename) {
                let palette: Result<Palette, _> = serde_json::from_reader(f);
                if let Ok(palette) = palette {
                    let mut cept = Cept::new();
                    cept.define_palette(&palette.palette, palette.start_color);
                    return Some(cept);
                } else {
                    println!("ERROR reading palette file! [1]");
                    return None;
                }
            } else {
                println!("ERROR reading palette file! [2]");
                return None;
            }
        } else {
            println!("skipping palette");
            return None;
        }
    } else {
        None
        // client_state.palette = None;
    }
}

fn load_include(include_name: Option<&String>, clear_screen: Option<bool>, basedir: &str, client_state: &mut ClientState) -> Option<Cept> {
    if let Some(include_name) = include_name {
        let filename = resource_filename(basedir, include_name, "inc");
        if Some(filename.clone()) != client_state.include || clear_screen == Some(true) {
            client_state.include = Some(filename.clone());
            let mut cept_include : Vec<u8> = vec!();
            println!("loading: {}", filename);
            if let Ok(mut f) = File::open(&filename) {
                if let Ok(_) = f.read_to_end(&mut cept_include) {
                    // ok
                } else {
                    println!("ERROR reading include file! [1]");
                }
            } else {
                println!("ERROR creating user! [1]");
            }
            let mut cept = Cept::new();
            // palette definition has to end with 0x1f; add one if
            // the include data doesn't start with one
            if cept_include[0] != 0x1f {
                cept.set_cursor(1, 1)
            }
            cept.add_raw(&cept_include);
            return Some(cept);
        } else {
            client_state.include = None;
        }
    }
    None
}