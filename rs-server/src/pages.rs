use std::io::Write;
use super::cept::*;


pub fn interactive_mode(stream: &mut impl Write)
{
    loop {

        let pageid = "123";

        let page = Page::create_historic_main_page();

        let mut cept1 = Cept::new();
        cept1.hide_cursor();

        if page.meta.clear_screen {
            cept1.serial_limited_mode();
            cept1.clear_screen();
            //last_filename_include = ""
        }

        // cept1.extend(create_preamble(basedir, meta))

        let mut cept2 = Cept::new();

        if page.meta.cls2 {
            cept2.serial_limited_mode();
            cept2.clear_screen();
            // last_filename_include = ""
        }

        headerfooter(&mut cept2, pageid, page.meta.publisher_name.as_deref(), page.meta.publisher_color);


        stream.write_all(cept1.data()).unwrap();
        stream.write_all(cept2.data()).unwrap();
    }
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
            Some(&p[..30])
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

struct Meta {
    publisher_name: Option<String>,
    clear_screen: bool,
    cls2: bool,
    links: Vec<(String, String)>,
    publisher_color: u8,
}

struct Page {
    cept: Cept,
    meta: Meta,
}

impl Page {
    pub fn new(meta: Meta) -> Self {
        Self {
            cept: Cept::new(),
            meta: meta,
        }
    }

    fn create_title(&mut self, title: &str) {
        self.cept.set_cursor(2, 1);
        self.cept.set_palette(1);
        self.cept.set_screen_bg_color_simple(4);
        self.cept.add_raw(
            &[0x1b, 0x28, 0x40,       // load G0 into G0
             0x0f]                   // G0 into left charset
        );
        self.cept.parallel_mode();
        self.cept.set_palette(0);
        self.cept.code_9e();
        self.cept.set_line_bg_color_simple(4);
        self.cept.add_raw(b"\n");
        self.cept.set_line_bg_color_simple(4);
        self.cept.set_palette(1);
        self.cept.double_height();
        self.cept.add_raw(b"\r");
        self.cept.add_str(title);
        self.cept.add_raw(b"\n\r");
        self.cept.set_palette(0);
        self.cept.normal_size();
        self.cept.code_9e();
        self.cept.set_fg_color_simple(7);
    }

	fn footer(&mut self, left: &str, right: Option<&str>) {
		self.cept.set_cursor(23, 1);
		self.cept.set_palette(0);
		self.cept.set_line_bg_color_simple(4);
		self.cept.add_str(left);

		if let Some(right) = right {
            self.cept.set_cursor(23, 41 - right.len() as u8);
            self.cept.add_str(right);
        }
    }

	pub fn create_historic_main_page() -> Self {
        let meta = Meta {
            publisher_name: Some("!BTX".to_owned()),
            clear_screen: true,
            cls2: false,
            links: vec![
        		("0".to_owned(), "0".to_owned()),
				("10".to_owned(), "710".to_owned()),
				("11".to_owned(), "711".to_owned()),
				("#".to_owned(), "711".to_owned()),
            ],
			publisher_color: 7,
		};

        let mut page = Page::new(meta);
		page.create_title("Historische Seiten");
		page.cept.add_raw(b"\r\n");
		page.cept.add_str(
			"Nur wenige hundert der mehreren hundert-\
			tausend BTX-Seiten sind überliefert.\n\
			Die meisten entstammen dem Demomodus von\
			Software-BTX-Decoderprogrammen.\n\
			\n\
			1988: C64 BTX Demo (Input 64 12/88)...--\
			1989: Amiga BTX Terminal..............10\
			1989: C64 BTX Demo (64'er 1/90).......--\
			1991: BTX-VTX Manager v1.2............--\
			1993: PC online 1&1...................11\
			1994: MacBTX 1&1......................--\
			1995: BTXTEST.........................--\
			1996: RUN_ME..........................--\
			\n\
			Da historische Seiten erst angepaßt wer-\
			den müssen, um nutzbar zu sein, sind\n\
			noch nicht alle Sammlungen verfügbar."
			//XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
		);

		page.footer("0 Zurück", None);
        page
    }
}


fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}
