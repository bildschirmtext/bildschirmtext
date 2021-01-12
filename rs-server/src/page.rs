use serde::{Deserialize, Serialize};

use super::cept::*;
use super::editor::*;
use super::session::*;
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
    pub autoplay: Option<bool>,

    // these are only uses by static pages
    pub palette: Option<String>,
    pub include: Option<String>,
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
    pub meta: Meta,
    pub cept_palette: Option<Cept>,
    pub cept_include: Option<Cept>,
    pub cept: Cept,
}

impl Page {
    pub fn new(meta: Meta) -> Self {
        Self {
            meta: meta,
            cept_palette: None,
            cept_include: None,
            cept: Cept::new(),
        }
    }

    pub fn construct_page_cept(&self, client_state: &mut ClientState, pageid: &PageId) -> Cept {
        let mut cept;
        cept = self.cept_preamble_from_meta(client_state);
        cept += self.cept_main_from_page(client_state, pageid);
        cept
    }

    //
    fn cept_preamble_from_meta(&self, client_state: &mut ClientState) -> Cept {
        let mut cept = Cept::new();

        cept.hide_cursor();

        let clear_screen = self.meta.clear_screen;

        if clear_screen == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            client_state.cept_include = None;
        }

        if let Some(cept_palette) = &self.cept_palette {
            if Some(cept_palette) != client_state.cept_palette.as_ref() {
                cept.add_raw(cept_palette.data());
            }
            client_state.cept_palette = Some(cept_palette.clone());
        }
        if let Some(cept_include) = &self.cept_include {
            if Some(cept_include) != client_state.cept_include.as_ref() {
                cept.add_raw(cept_include.data());
            }
            client_state.cept_include = Some(cept_include.clone());
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
            client_state.cept_include = None;
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

