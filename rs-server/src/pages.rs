use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::fs::File;
use super::cept::*;
use super::editor::*;
use super::historic::*;
use super::stat::*;




pub fn create_page(pageid: &str) -> (Cept, Cept, Option<Vec<Link>>, Option<Inputs>, bool) {
    let page = match pageid.chars().next().unwrap() {
        '7' => super::historic::create(&pageid[1..]),
        _ => super::stat::create(pageid).unwrap(),
    };

    let mut cept1 = Cept::new();
    cept1.hide_cursor();

    if page.meta.clear_screen == Some(true) {
        cept1.serial_limited_mode();
        cept1.clear_screen();
        //last_filename_include = ""
    }

    let basedir = find_basedir(pageid).unwrap().0;
    cept1.extend(&create_preamble(&basedir, &page.meta));

    let mut cept2 = Cept::new();

    if page.meta.cls2 == Some(true) {
        cept2.serial_limited_mode();
        cept2.clear_screen();
        // last_filename_include = ""
    }

    headerfooter(&mut cept2, pageid, page.meta.publisher_name.as_deref(), page.meta.publisher_color.unwrap());

    if page.meta.parallel_mode == Some(true) {
        cept2.parallel_mode();
    }

    cept2.add_raw(page.cept.data());

    cept2.serial_limited_mode();

    // cept_2.extend(hf) //???

    cept2.sequence_end_of_page();

    // XXX
    let links = page.meta.links;
    let inputs = page.meta.inputs;
    let autoplay = false;

    (cept1, cept2, links, inputs, autoplay)
}

pub fn headerfooter(cept: &mut Cept, pageid: &str, publisher_name: Option<&str>, publisher_color: u8) {
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
        cept.add_str(&format!("{pageid:>width$}", pageid=pageid, width=22));
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
        cept.add_str(&format_currency(0.0));
    }

	cept.cursor_home();
	cept.set_palette(0);
	cept.protect_line();
	cept.add_raw(b"\n");
}

fn create_preamble(basedir: &str, meta: &Meta) -> Cept {
	// global last_filename_include
	// global last_filename_palette

	let mut cept = Cept::new();

	// define palette
	if let Some(palette) = &meta.palette {
        let mut filename_palette = basedir.to_owned();
        filename_palette += &palette;
        filename_palette += ".pal";
		println!("filename_palette = {}", filename_palette);
		// println!("last_filename_palette = {}", last_filename_palette);
		// if filename_palette != last_filename_palette {
			// last_filename_palette = filename_palette
            let mut f = File::open(&filename_palette).unwrap();
            let mut palette: Palette = serde_json::from_reader(f).unwrap();
			cept.define_palette(&palette.palette, palette.start_color);
        // } else {
            // sys.stderr.write("skipping palette\n")
        // }
    } else {
        // last_filename_palette = ""
    }

	if let Some(include) = &meta.include {
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

		// if ((filename_include != last_filename_include) or meta.get("clear_screen", False)) {
			// last_filename_include = filename_include;
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
    // } else {
        // last_filename_include = ""
    // }

	// b = baud if baud else 1200
	// if len(cept) > (b/9) * SH291_THRESHOLD_SEC {
        // cept = Util.create_system_message(291) + cept
    // }
    }
    cept
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
    palette: Vec<String>,
    start_color: Option<u8>,
}

fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}

