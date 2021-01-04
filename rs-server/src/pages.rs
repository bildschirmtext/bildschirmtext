use std::{fs::File, io::Read};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::cept::*;
use super::editor::*;
use super::session::*;
use super::stat::*;

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

        // if compress {
        //     page_cept_data_1 = Cept.compress(page_cept_data_1)
        // }

        cept
    }

    //
    fn cept_preamble_from_meta(&self, client_state: &mut ClientState, pageid: &PageId) -> Cept {
        let mut cept = Cept::new();

        cept.hide_cursor();

        if self.meta.clear_screen == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            client_state.include = None;
        }

        let basedir = find_basedir(pageid).unwrap().0;

        // define palette
        if let Some(palette) = &self.meta.palette {
            let mut filename_palette = basedir.to_owned();
            filename_palette += &palette;
            filename_palette += ".pal";
            println!("filename_palette = {}", filename_palette);
            // println!("last_filename_palette = {}", last_filename_palette);
            if Some(filename_palette.clone()) != client_state.palette {
                client_state.palette = Some(filename_palette.clone());
                let f = File::open(&filename_palette).unwrap();
                let palette: Palette = serde_json::from_reader(f).unwrap();
                cept.define_palette(&palette.palette, palette.start_color);
            } else {
                println!("skipping palette");
            }
        } else {
            client_state.palette = None;
        }

        if let Some(include) = &self.meta.include {
            let mut filename_include = basedir.to_owned();
            filename_include += &include;
            filename_include += ".inc";
            // if is_file(filename_include) {
            // 	filename_include_cm = basedir + meta["include"] + ".inc.cm"
            // 	filename_include = basedir + meta["include"] + ".inc"
            // } else {
            // 	filename_include_cm =""
            //     filename_include = basedir + meta["include"] + ".cept"
            // }
            println!("Filename_include = {}", filename_include);

            if Some(filename_include.clone()) != client_state.include || self.meta.clear_screen == Some(true) {
                client_state.include = Some(filename_include.clone());
                // if os.path.isfile(filename_include) {
                    let mut cept_include : Vec<u8> = vec!();
                    let mut f = File::open(&filename_include).unwrap();
                    f.read_to_end(&mut cept_include);
                    println!("loading: {}", filename_include);
                // } else if os.path.isfile(filename_include_cm) {
                // 	data_include = CM.read(filename_include_cm)
                // } else {
                //     sys.stderr.write("include file not found.\n")
                // }
                // palette definition has to end with 0x1f; add one if
                // the include data doesn't start with one
                if cept_include[0] != 0x1f {
                    cept.set_cursor(1, 1)
                }
                cept.add_raw(&cept_include);
            // }
            } else {
                client_state.include = None;
            }

        // b = baud if baud else 1200
        // if len(cept) > (b/9) * SH291_THRESHOLD_SEC {
            // cept = Util.create_system_message(291) + cept
        // }
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

        let color_string = if publisher_color < 8 {
            let mut c = Cept::new();
            c.set_fg_color(publisher_color);
            c
        } else {
            let mut c = Cept::new();
            c.set_fg_color_simple(publisher_color - 8);
            c
        };

        cept.add_raw(color_string.data());

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

        cept.add_raw(color_string.data());

        cept.add_raw(b"\r");


        // TODO: price
        if !hide_header_footer & !hide_price {
            cept.add_str(publisher_name.unwrap());
            cept.set_cursor(1, 31);
            cept.add_raw(b"  ");
            cept.add_str(&format_currency(0));
        }

        cept.cursor_home();
        cept.set_palette(0);
        cept.protect_line();
        cept.add_raw(b"\n");
    }


}

fn format_currency(price: u32) -> String {
    format!("DM  {},{:02}", price / 100, price % 100)
}

pub fn create_system_message(code: usize, price: Option<u32>) -> Cept {
    let mut text = String::new();
    let mut prefix = "SH";
    if code == 0 {
        text = "                               ".to_owned();
    } else if code == 10 {
        text = "Rückblättern nicht möglich     ".to_owned();
    } else if code == 44 {
        text = "Absenden? Ja:19 Nein:2         ".to_owned();
    } else if code == 47 {
        text = format!("Absenden für {}? Ja:19 Nein:2", format_currency(price.unwrap()));
    } else if code == 55 {
        text = "Eingabe wird bearbeitet        ".to_owned();
    } else if code == 73 {
        let current_datetime = Utc::now().format("%d.%m.%Y %H:%M").to_string();
        text = format!("Abgesandt {}, -> #  ", current_datetime);
        prefix = "1B";
    } else if code == 100 || code == 101 {
        text = "Seite nicht vorhanden          ".to_owned();
    } else if code == 291 {
        text = "Seite wird aufgebaut           ".to_owned();
    }

    let mut msg = Cept::new();
    msg.service_break(24);
    msg.clear_line();
    msg.add_str_characterset(&text, Some(1));
    msg.hide_text();
    msg.add_raw(b"\x08");
    msg.add_str(prefix);
    msg.add_str(&format!("{:03}", code));
    msg.service_break_back();
    msg
}
