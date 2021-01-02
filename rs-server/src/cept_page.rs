use super::cept::*;


struct Image {
    chars: Vec<Vec<u8>>,
}

struct CeptPage {
    title: Option<String>,
	x: usize,
	y: usize,
	lines_cept: Vec<Cept>,
	data_cept: Cept,
	italics: bool,
	bold: bool,
	link: bool,
	code: bool,
	dirty: bool,
	title_image_width: usize,
	title_image_height: usize,
	lines_per_sheet: usize,
	prev_sheet: usize,
	characterset: CharacterSet,
	// drcs_start_for_first_sheet = None
}

impl CeptPage {
    pub fn new() -> CeptPage {
        CeptPage {
            title: None,
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
            characterset: CharacterSet {},
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
                // self.drcs_start_for_first_sheet = self.characterset.drcs_code;
            }
			// new character set for every sheet
            self.characterset = CharacterSet {};
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

	fn resend_attributes(self) {
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
        self.data_cept.add_str_characterset(s, Some(&self.characterset));
    }

    fn print_internal(&mut self, s: &str) {
		if s == "" {
            return
        }

        let mut s = s;

		while s.len() != 0 {
            let index = s.chars().position(|c| c == ' ');
            let mut ends_in_space = index.is_some();
            let mut index = index.unwrap_or(s.len());

			let line_width = if self.y < self.title_image_height {
				40 - self.title_image_width
            } else {
                40
            };

            let new_s = if index >= 40 {
				// it wouldn't ever fit, break it
				// at the end of the line
				index = 40 - self.x as usize;
				Some(&s[index..])
            } else {
                None
            };

			if index == 0 && self.x == 0 {
				// starts with space and we're at the start of a line
				// -> skip space

            } else if index + self.x as usize > line_width as usize {
				// word doesn't fit, print it (plus the space)
				// into a new line
				self.print_newline();
				self.add_string(&s[..index + 1]);
				self.x += index;
				if ends_in_space {
                    self.x += 1
                }
            } else if ends_in_space && index + self.x as usize + 1 == 40 {
				// space in last column
				// -> just print it, cursor will be in new line
				self.add_string(&s[..index + 1]);
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
				self.add_string(&s[..index + 1]);
				self.x += s[..index + 1].len();
				if self.x == 40 {
                    self.create_new_line();
                }
            }

			s = if let Some(new_s) = new_s {
				new_s
            } else {
                &s[index + 1..]
            }
        }
    }

    pub fn create_new_line(&self) {
		self.lines_cept.push(self.data_cept);
        self.init_new_line()
    }

    pub fn set_italics_on(&self) {
		self.italics = true;
		self.dirty = true;
    }

    pub fn set_italics_off(&self) {
		self.italics = false;
		self.dirty = true;
    }

    pub fn set_bold_on(&self) {
		self.bold = true;
		self.dirty = true;
    }

    pub fn set_bold_off(&self) {
		self.bold = false;
		self.dirty = true;
    }

    pub fn set_link_on(&self) {
		self.link = true;
		self.dirty = true;
    }

    pub fn set_link_off(&self) {
		self.link = false;
		self.dirty = true;
    }

    pub fn set_code_on(&self) {
		self.code = true;
		self.dirty = true;
    }

    pub fn set_code_off(&self) {
		self.code = false;
		self.dirty = true;
    }

	pub fn print(&mut self, s: &str, ignore_lf: bool) {
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
		self.prev_sheet = self.current_sheet();

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
			self.data_cept.add_str(&s[..39]);
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
			self.data_cept.add_str(&s[..39]);
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
			data_cept.set_screen_bg_color(7);
			data_cept.set_cursor(2, 1);
			data_cept.set_line_bg_color(0);
			data_cept.add_raw(b"n");
			data_cept.set_line_bg_color(0);
			data_cept.double_height();
			data_cept.set_fg_color(7);
			data_cept.add_str(&self.title.unwrap()[..39]);
			data_cept.add_raw(b"\r\n");
			data_cept.normal_size();
			data_cept.add_raw(b"\n");
        } else {
			// on sheets b+, we need to clear the image area
			if let Some(image) = image {
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
			data_cept.define_palette(image.palette);
			// DRCS
			data_cept.add_raw(image.drcs);
			// draw characters
			let i = 0;
			for l in image.chars {
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

struct CeptFromHtmlGenerator {
    cept_page: CeptPage,
	link_index: Option<u32>,
	wiki_link_targets: Vector<String>,
	page_and_link_index_for_link: Vector<String>,
	first_paragraph: bool,
	link_count: u32,
	links_for_page: Vector<String>,
	pageid_base: Option<String>,
	ignore_lf: bool,
	article_prefix: Option<String>,
}

impl CeptFromHtmlGenerator {
	fn insert_toc(&mut self) {
		self.page_and_link_index_for_link = vec!();
		for t1 in soup.contents[0].children {
			if ["h2", "h3", "h4", "h5", "h6"].contains(t1.name) {
				if self.current_sheet() != self.prev_sheet {
					self.link_index = 10;
                }
				level = int(t1.name[1]);
				// non-breaking space, otherwise it will be filtered at the beginning of lines
				indent = (level - 2) * b"\xa0\xa0";
				entry = indent + t1.get_text().replace("\n", "");
				padded = entry + ("." * 36);
				padded = padded[..36];
				self.print(padded + "[" + str(self.link_index) + "]");
				self.page_and_link_index_for_link.append((self.current_sheet(), self.link_index));
                self.link_index += 1;
            }
        }
    }

	fn insert_html_tags(&mut self, tags: Vec<String>) {
		for t1 in tags {
			if t1.name == "p" {
				self.insert_html_tags(t1.children);
				self.print("\n");

				if self.first_paragraph {
					self.first_paragraph = false;
					self.insert_toc(self.soup);
//					sys.stderr.write("self.page_and_link_index_for_link: " + pprint.pformat(self.page_and_link_index_for_link) + "\n")
                    self.print("\n");
                }
            } else if ["h2", "h3", "h4", "h5", "h6"].contains(t1.name) {
				level = int(t1.name[1]);
				self.print_heading(level, t1.contents[0].get_text().replace("\n", ""));
				if self.page_and_link_index_for_link { // only if there is a TOC
					(link_page, link_name) = self.page_and_link_index_for_link[self.link_count];
					self.link_count += 1;
					while len(self.links_for_page) < link_page + 1 {
                        self.links_for_page.append({})
                    }
                    self.links_for_page[link_page][str(link_name)] = self.pageid_base + chr(0x61 + self.current_sheet())
                }
            } else if t1.name.is_none() {
				self.print(t1, self.ignore_lf)
			} else if t1.name == "span" {
				self.print(t1.get_text(), self.ignore_lf)
			} else if t1.name == "i" {
				self.set_italics_on();
				self.print(t1.get_text(), self.ignore_lf);
				self.set_italics_off();
			} else if t1.name == "b" {
				self.set_bold_on();
				self.print(t1.get_text(), self.ignore_lf);
				self.set_bold_off();
			} else if t1.name == "a" {
				if t1["href"].startswith(self.article_prefix) { // links to different article
					if self.current_sheet() != self.prev_sheet {
						self.link_index = 10;
						// TODO: this breaks if the link
                        // goes across two sheets!
                    }

					while len(self.wiki_link_targets) < self.current_sheet() + 1 {
                        self.wiki_link_targets.append({});
                    }
					self.wiki_link_targets[self.current_sheet()][self.link_index] = t1["href"][len(self.article_prefix)..];

					link_text = t1.get_text().replace("\n", "") + " [" + str(self.link_index) + "]";
					self.set_link_on();
					self.print(link_text);
					self.link_index += 1;
                    self.set_link_off();
                } else { // link to section or external link, just print the text
                    self.print(t1.get_text(), self.ignore_lf);
            }
            } else if t1.name == "ul" {
            self.insert_html_tags(t1.children)
            } else if t1.name == "ol" {
                self.insert_html_tags(t1.children)
            } else if t1.name == "code" {
                self.set_code_on();
                self.insert_html_tags(t1.children);
                self.set_code_off();
            } else if t1.name == "li" {
                // TODO indentation
                self.print("* "); // TODO: ordered list
                self.insert_html_tags(t1.children);
                self.print("\n");
            } else if t1.name == "pre" {
                self.ignore_lf = false;
                self.insert_html_tags(t1.children);
                self.ignore_lf = true;
            } else {
                sys.stderr.write("ignoring tag: " + pprint.pformat(t1.name) + "\n")
            }
        }
    }

}
