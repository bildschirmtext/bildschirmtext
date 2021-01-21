use select::{document::Document, node::Children};
use select::node::Node;
use select::predicate::{Class, Name, Predicate};
use std::{collections::HashMap, str::FromStr};
use super::cept::*;

struct Image {
    chars: Vec<Vec<u8>>,
    palette: &'static [String],
    drcs: Cept,
}

pub struct CeptPage {
    title: String,
	x: usize,
	y: usize,
	lines_cept: Vec<Cept>,
	pub data_cept: Cept,
	italics: bool,
	bold: bool,
	link: bool,
	code: bool,
	dirty: bool,
	title_image_width: usize,
	title_image_height: usize,
	lines_per_sheet: usize,
	prev_sheet: usize,
	characterset: usize,
	// drcs_start_for_first_sheet = None
}

impl CeptPage {
    pub fn new() -> CeptPage {
        CeptPage {
            title: "".to_owned(),
            x: 0,
            y: 0, // XXX -1
            lines_cept: vec!(),
            data_cept: Cept::new(),
            italics: false,
            bold: false,
            link: false,
            code: false,
            dirty: false,
            title_image_width: 0,
            title_image_height: 0,
            lines_per_sheet: 17,
            prev_sheet: 0,
            characterset: 0,
        }
    }

	fn init_new_line(&mut self) {
		self.data_cept = Cept::new();
		self.data_cept.clear_line();
// 		sys.stderr.write("self.y: '" + pprint.pformat(self.y) + "'\n")
// 		sys.stderr.write("self.y % lines_per_sheet: '" + pprint.pformat(self.y % lines_per_sheet) + "'\n")
		self.x = 0;
		self.y += 1;

		if (self.y % self.lines_per_sheet) == 0 {
			self.resend_attributes();
			// remember how much of the DRCS space on the first sheet was used
			if self.current_sheet() == 1 {
                // self.drcs_start_for_first_sheet = self.characterset.drcs_code; // XXX
            }
			// new character set for every sheet
            self.characterset = 0;
        }
    }

	fn print_newline(&mut self) {
		if self.x == 0 && self.y % self.lines_per_sheet == 0 {
			// no empty first lines
            return
        }
		self.reset_style();
		self.data_cept.repeat(b' ', 40 - self.x as u8);
		self.resend_attributes();
        self.create_new_line();
    }

    fn reset_style(&mut self) {
		self.data_cept.set_fg_color(15);
		self.data_cept.set_bg_color(7);
        self.data_cept.underline_off();
    }

	fn resend_attributes(&mut self) {
// 		sys.stderr.write("self.italics: " + pprint.pformat(["self.italics: ",self.italics , self.bold , self.link]) + "\n")
		if self.italics {
			self.data_cept.set_fg_color(6);
        } else if self.bold {
            self.data_cept.set_fg_color(0);
        }
		if self.code {
			self.data_cept.set_bg_color(6);
        } else {
            self.data_cept.set_bg_color(7);
        }
		if self.link {
			self.data_cept.underline_on();
            self.data_cept.set_fg_color(4);
        }
		if !self.italics && !self.bold && !self.link && !self.code {
            self.reset_style()
        }
        self.dirty = false;
    }

    fn add_string(&mut self, s: &str) {
        if self.dirty {
            self.resend_attributes();
        }
        self.data_cept.add_str_characterset(s, Some(self.characterset));
    }

    fn print_internal(&mut self, s: &str) {
		let mut s = s;

		while s.len() != 0 {
			// let split = s.splitn(1, |c: char| c.is_whitespace()).collect();


            let index = s.chars().position(char::is_whitespace);
            let ends_in_space = index.is_some();
            let mut index = index.unwrap_or(s.len());

			let line_width = 40 - if self.y < self.title_image_height { self.title_image_width } else { 0 };

            let new_s = if index >= 40 {
				// it wouldn't ever fit, break it
				// at the end of the line
				index = 40 - self.x as usize;
				Some(&s[index..])
            } else {
                None
			};

			let index2 = if index + 1 <= s.len() {
				index + 1
			} else {
				index
			};

			if index == 0 && self.x == 0 {
				// starts with space and we're at the start of a line
				// -> skip space

            } else if index + self.x as usize > line_width as usize {
				// word doesn't fit, print it (plus the space)
				// into a new line
				self.print_newline();
				self.add_string(&s[..index2]);
				self.x += index;
				if ends_in_space {
                    self.x += 1
                }
            } else if ends_in_space && index + self.x as usize + 1 == 40 {
				// space in last column
				// -> just print it, cursor will be in new line
				self.add_string(&s[..index2]);
				self.create_new_line()
			} else if !ends_in_space && index + self.x as usize == 40 {
				// character in last column, not followed by a space
				// -> just print it, cursor will be in new line
				self.add_string(&s[..index]);
				self.create_new_line()
			} else if ends_in_space && index + self.x as usize == 40 {
				// character in last column, followed by space
				// -> omit the space, cursor will be in new line
				self.add_string(&s[..index]);
				self.create_new_line();
            } else {
				self.add_string(&s[..index2]);
				self.x += s[..index2].len();
				if self.x == 40 {
                    self.create_new_line();
                }
            }

			s = if let Some(new_s) = new_s {
				new_s
            } else {
				&s[index2..]
            }
        }
    }

    pub fn create_new_line(&mut self) {
        let mut data_cept = Cept::new();
        std::mem::swap(&mut self.data_cept, &mut data_cept);
        self.lines_cept.push(data_cept);
        self.init_new_line() // XXX overwrites self.data_cept again
    }

    pub fn set_italics_on(&mut self) {
		println!("set_italics_on");
		return;

		self.italics = true;
		self.dirty = true;
    }

    pub fn set_italics_off(&mut self) {
		println!("set_italics_off");
		return;

		self.italics = false;
		self.dirty = true;
    }

    pub fn set_bold_on(&mut self) {
		println!("set_bold_on");
		return;

		self.bold = true;
		self.dirty = true;
    }

    pub fn set_bold_off(&mut self) {
		println!("set_bold_off");
		return;

		self.bold = false;
		self.dirty = true;
    }

    pub fn set_link_on(&mut self) {
		println!("set_link_on");
		return;

		self.link = true;
		self.dirty = true;
    }

    pub fn set_link_off(&mut self) {
		println!("set_link_off");
		return;

		self.link = false;
		self.dirty = true;
    }

    pub fn set_code_on(&mut self) {
		println!("set_code_on");
		return;

		self.code = true;
		self.dirty = true;
    }

    pub fn set_code_off(&mut self) {
		println!("set_code_off");
		return;

		self.code = false;
		self.dirty = true;
    }

	pub fn print(&mut self, s: &str, ignore_lf: bool) {
		println!("print: \"{}\"", s);
		return;

		self.prev_sheet = self.current_sheet();

        if s.contains('\n') {
            for l in s.lines() {
                self.print_internal(l);
                if !ignore_lf {
                    self.print_newline();
                }
            }
        } else {
			self.print_internal(s);
        }
    }

	pub fn print_heading(&mut self, level: i32, s: &str) {
		println!("print_heading: {}, \"{}\"", level, s);
		return;

		self.prev_sheet = self.current_sheet();

		let s = if s.len() > 39 {
			&s[..39]
		} else {
			s
		};
		if level == 2 {
			if (self.y + 1) % self.lines_per_sheet == 0 || (self.y + 2) % self.lines_per_sheet == 0 {
				// don't draw double height title into
				// the last line or the one above
				self.data_cept.add_str("\n");
                self.create_new_line();
            }
			self.data_cept.underline_off();
			self.data_cept.clear_line();
			self.data_cept.add_raw(b"\n");
			self.data_cept.clear_line();
			self.data_cept.set_fg_color(0);
			self.data_cept.double_height();
			self.data_cept.add_str(&s);
			self.data_cept.add_raw(b"\r\n");
			self.data_cept.normal_size();
			self.data_cept.set_fg_color(15);
			self.create_new_line();
			self.create_new_line();
        } else {
			if (self.y + 1) % self.lines_per_sheet == 0 {
				// don't draw title into the last line
				self.data_cept.add_raw(b"\n");
                self.create_new_line();
            }
			self.data_cept.underline_on();
			self.data_cept.set_fg_color(0);
			self.data_cept.add_str(&s);
			self.data_cept.underline_off();
			self.data_cept.set_fg_color(15);
			self.data_cept.add_raw(b"\r\n");
            self.create_new_line();
        }
        return
    }

	pub fn current_sheet(&self) -> usize {
        self.y / self.lines_per_sheet
    }

	pub fn number_of_sheets(&self) -> usize {
		self.lines_cept.len() / self.lines_per_sheet
    }

	fn cept_for_sheet(&mut self, sheet_number: usize) -> Cept {
		let mut cept = Cept::new();
		let lines = &self.lines_cept[sheet_number * self.lines_per_sheet .. (sheet_number + 1) * self.lines_per_sheet];
		if lines.len() == 0 {
            return cept;
        }
		for line in lines {
            cept.extend(line);
        }
		// fill page with blank lines
		for i in 0 .. self.lines_per_sheet - lines.len() {
			cept.add_raw(b"\n");
            cept.clear_line();
        }
        cept
    }

	pub fn complete_cept_for_sheet(&mut self, sheet_number: usize, image: Option<Image>) -> Cept {
		let is_first_page = sheet_number == 0;

		// print the page title (only on the first sheet)
		let mut data_cept = Cept::new();
		data_cept.parallel_mode();

		if is_first_page {
            let title = &self.title;
			data_cept.set_screen_bg_color(7);
			data_cept.set_cursor(2, 1);
			data_cept.set_line_bg_color(0);
			data_cept.add_raw(b"n");
			data_cept.set_line_bg_color(0);
			data_cept.double_height();
			data_cept.set_fg_color(7);
			data_cept.add_str(&self.title[..39]);
			data_cept.add_raw(b"\r\n");
			data_cept.normal_size();
			data_cept.add_raw(b"\n");
        } else {
			// on sheets b+, we need to clear the image area
			if let Some(image) = &image {
                for i in 0..2 {
                    data_cept.set_cursor(3 + i, 41 - image.chars[0].len() as u8);
                    data_cept.repeat(b' ', image.chars[0].len() as u8);
                }
            }
        }

		// print navigation
		// * on sheet 0, so we don't have to print it again on later sheets
		// * on the last sheet, because it doesn't show the "#" text
		// * on the second last sheet, because navigating back from the last one needs to show "#" again
		if sheet_number == 0 || sheet_number >= self.number_of_sheets() - 2 {
			data_cept.set_cursor(23, 1);
			data_cept.set_line_bg_color(0);
			data_cept.set_fg_color(7);
			data_cept.add_str("0 < Back");
			let s = "# > Next";
			data_cept.set_cursor(23, 41 - s.len() as u8);
			if sheet_number == self.number_of_sheets() - 1 {
				data_cept.repeat(b' ', s.len() as u8);
            } else {
                data_cept.add_str(s);
            }
        }

		data_cept.set_cursor(5, 1);

		// add text
		data_cept += self.cept_for_sheet(sheet_number);

		// transfer image on first sheet
		if is_first_page && image.is_some() {
            let image = image.unwrap();
			// placeholder rectangle
			for y in 0..image.chars.len() {
				data_cept.set_cursor(3 + y as u8, 41 - image.chars[0].len() as u8);
				data_cept.set_bg_color(15);
                data_cept.repeat(b' ', image.chars[0].len() as u8);
            }
			// palette
			data_cept.define_palette(image.palette, None);
			// DRCS
			data_cept += image.drcs;
			// draw characters
			let mut i = 0;
			for l in &image.chars {
				data_cept.set_cursor(3 + i as u8, 41 - image.chars[0].len() as u8);
				data_cept.load_g0_drcs();
				data_cept.add_raw(&l);
				data_cept.add_raw(b"\r\n");
                i += 1;
            }
        }

        return data_cept
    }
}

pub struct CeptFromHtmlGenerator {
    pub cept_page: CeptPage,
	link_index: usize,
	wiki_link_targets: HashMap<usize, HashMap<usize, String>>,
	page_and_link_index_for_link: Vec<(usize, usize)>,
	first_paragraph: bool,
	link_count: usize,
	links_for_page: HashMap<usize, HashMap<String, String>>,
	pageid_base: Option<String>,
	ignore_lf: bool,
	article_prefix: Option<String>,
}

impl CeptFromHtmlGenerator {
	pub fn new() -> Self {
		Self {
			cept_page: CeptPage::new(),
			link_index: 0,
			wiki_link_targets: HashMap::new(),
			page_and_link_index_for_link: vec!(),
			first_paragraph: false,
			link_count: 0,
			links_for_page: HashMap::new(),
			pageid_base: None,
			ignore_lf: false,
			article_prefix: None,
		}
	}


	fn insert_toc(&mut self, node: &Node) {
        self.page_and_link_index_for_link = vec!();
		for t1 in node.children() {
			if ["h2", "h3", "h4", "h5", "h6"].contains(&t1.name().unwrap()) {
				if self.cept_page.current_sheet() != self.cept_page.prev_sheet {
					self.link_index = 10;
				}
				let mut xx = t1.name().unwrap().to_owned();
				xx.remove(0);
				let level = usize::from_str(&xx).unwrap();
				// non-breaking space, otherwise it will be filtered at the beginning of lines
				let mut text = "\u{00a0}\u{00a0}".repeat(level - 2);
				text.push_str(&t1.text().replace("\n", ""));
				text.push_str(&(".".repeat(36)));
				let mut text = text[..36].to_owned();
				text.push('[');
				text.push_str(&self.link_index.to_string());
				text.push(']');
				self.cept_page.print(&text, false);
				self.page_and_link_index_for_link.push((self.cept_page.current_sheet(), self.link_index));
                self.link_index += 1;
            }
        }
    }

	// fn insert_html_tags<'a, I>(&mut self, tags: I)
	// where
	// 	I: Iterator<Item = &'a Node<'a>>
	// {
	pub fn insert_html_tags(&mut self, tags: Children) {
		for t1 in tags {
			match t1.name() {
				Some("p") => {
					self.insert_html_tags(t1.children());
					self.cept_page.print("\n", false);

					// if self.first_paragraph {
					// 	self.first_paragraph = false;
					// 	self.insert_toc(self.html);
					// 	self.cept_page.print("\n", false);
					// }
				},
				Some("h2") | Some("h3") | Some("h4") | Some("h5") | Some("h6") => {
					let mut xx = t1.name().unwrap().to_owned();
					xx.remove(0);
					let level = i32::from_str(&xx).unwrap();
					self.cept_page.print_heading(level, &t1.text().replace("\n", ""));
					// if !self.page_and_link_index_for_link.is_empty() { // only if there is a TOC
					// 	let (link_page, link_name) = self.page_and_link_index_for_link[self.link_count];
					// 	self.link_count += 1;
					// 	self.links_for_page[&link_page][&link_name.to_string()] = self.pageid_base.unwrap() + &((b'a' + self.cept_page.current_sheet() as u8) as char).to_string();
					// }
				},
				None => {
					self.cept_page.print(&t1.text(), self.ignore_lf)
				},
				Some("span") => {
					self.cept_page.print(&t1.text(), self.ignore_lf)
				},
				Some("i") => {
					self.cept_page.set_italics_on();
					self.cept_page.print(&t1.text(), self.ignore_lf);
					self.cept_page.set_italics_off();
				},
				Some("b") => {
					self.cept_page.set_bold_on();
					self.cept_page.print(&t1.text(), self.ignore_lf);
					self.cept_page.set_bold_off();
				},
				// Some("a") => {
				// 	if t1.attr("href").unwrap().starts_with(&self.article_prefix.unwrap()) { // links to different article
				// 		if self.cept_page.current_sheet() != self.cept_page.prev_sheet {
				// 			self.link_index = 10;
				// 			// TODO: this breaks if the link
				// 			// goes across two sheets!
				// 		}

				// 		self.wiki_link_targets[&self.cept_page.current_sheet()][&self.link_index] = t1.attr("href").unwrap().to_owned();
				// 		//XXX[self.article_prefix.len()..];

				// 		let link_text = t1.text().replace("\n", "") + " [" + &self.link_index.to_string() + "]";
				// 		self.cept_page.set_link_on();
				// 		self.cept_page.print(&link_text, false);
				// 		self.link_index += 1;
				// 		self.cept_page.set_link_off();
				// 	} else { // link to section or external link, just print the text
				// 		self.cept_page.print(&t1.text(), self.ignore_lf);
				// 	}
				// },
				Some("ul") => {
					self.insert_html_tags(t1.children())
				},
				Some("ol") => {
					self.insert_html_tags(t1.children())
				},
				Some("code") => {
					self.cept_page.set_code_on();
					self.insert_html_tags(t1.children());
					self.cept_page.set_code_off();
				},
				Some("li") => {
					// TODO indentation
					self.cept_page.print("* ", false); // TODO: ordered list
					self.insert_html_tags(t1.children());
					self.cept_page.print("\n", false);
				},
				Some("pre") => {
					self.ignore_lf = false;
					self.insert_html_tags(t1.children());
					self.ignore_lf = true;
				},
				_ => {
					println!("ignoring tag: {:?}", t1.name());
				}
			}
        }
    }
}
