use super::cept::*;

struct CharacterSet {
}

struct CeptPage {
    title: Option<String>,
	x: u8,
	y: i8,
	lines_cept: Vec<String>,
	data_cept: Cept,
	italics: bool,
	bold: bool,
	link: bool,
	code: bool,
	dirty: bool,
	title_image_width: usize,
	title_image_height: usize,
	lines_per_sheet: u8,
	// prev_sheet = None
	characterset: Option<CharacterSet>,
	// drcs_start_for_first_sheet = None
}

impl CeptPage {
    pub fn new() -> CeptPage {
        CeptPage {
            title: None,
            x: 0,
            y: -1,
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
        }
    }

	fn init_new_line(&mut self) {
		self.data_cept = Cept::new();
		self.data_ceptclear_line();
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
		self.data_cept.repeat(" ", 40 - self.x);
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
        self.dirty = False
    }

    fn add_string(&mut self, s: &str) {
        if self.dirty {
            self.resend_attributes();
        }
        self.data_cept.add_str(s, self.characterset);
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

			if self.y < self.title_image_height {
				line_width = 40 - self.title_image_width
            } else {
                line_width = 40
            }

            let new_s = if index >= 40 {
				// it wouldn't ever fit, break it
				// at the end of the line
				index = 40 - self.x;
				Some(s[index..])
            } else {
                None
            };

			if index == 0 && self.x == 0 {
				// starts with space and we're at the start of a line
				// -> skip space

            } else if index + self.x > line_width {
				// word doesn't fit, print it (plus the space)
				// into a new line
				self.print_newline();
				self.add_string(s[..index + 1]);
				self.x += index;
				if ends_in_space {
                    self.x += 1
                }
            } else if ends_in_space && index + self.x + 1 == 40 {
				// space in last column
				// -> just print it, cursor will be in new line
				self.add_string(s[..index + 1]);
				self.create_new_line()
			} else if !ends_in_space && index + self.x == 40 {
				// character in last column, not followed by a space
				// -> just print it, cursor will be in new line
				self.add_string(s[..index]);
				self.create_new_line()
			} else if ends_in_space && index + self.x == 40 {
				// character in last column, followed by space
				// -> omit the space, cursor will be in new line
				self.add_string(s[..index]);
				self.create_new_line();
            } else {
				self.add_string(s[..index + 1]);
				self.x += len(s[..index + 1]);
				if self.x == 40 {
                    self.create_new_line();
                }
            }

			s = if let Some(new_s) = new_s {
				new_s
            } else {
                s[index + 1..]
            }
        }
    }

    pub fn create_new_line(&self) {
		self.lines_cept.append(self.data_cept);
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

	pub fn current_sheet(&self) -> i32 {
        int(self.y / self.lines_per_sheet)
    }

	pub fn number_of_sheets(&self) -> i32 {
		(self.lines_cept.len() as f32 / self.lines_per_sheet as f32).ceil
    }

	fn cept_for_sheet(&mut self, sheet_number: i32) -> Option<Cept> {
		let lines = self.lines_cept[sheet_number * self.lines_per_sheet .. (sheet_number + 1) * self.lines_per_sheet];
		if lines.len() == 0 {
            return None;
        }
		let mut data_cept = Cept::new();
		for line in lines {
            data_cept.extend(line);
        }
		// fill page with blank lines
		for i in 0 .. self.lines_per_sheet - lines.len() {
			data_cept.add_raw(b"\n");
            data_cept.clear_line();
        }
        return Some(data_cept);
    }

	pub fn complete_cept_for_sheet(&mut self, sheet_number: i32, image: Option<Image>) {
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
			data_cept.from_str(self.title[..39]);
			data_cept.add_raw(b"\r\n");
			data_cept.normal_size();
			data_cept.add_raw(b"\n");
        } else {
			// on sheets b+, we need to clear the image area
			if let Some(image) = image {
                for i in 0..2 {
                    data_cept.set_cursor(3 + i, 41 - len(image.chars[0]));
                    data_cept.repeat(" ", len(image.chars[0]));
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
			data_cept.from_str("0 < Back");
			s = "# > Next";
			data_cept.set_cursor(23, 41 - len(s));
			if sheet_number == self.number_of_sheets() - 1 {
				data_cept.repeat(" ", len(s));
            } else {
                data_cept.from_str(s);
            }
        }

		data_cept.set_cursor(5, 1);

		// add text
		data_cept.add_raw(self.cept_for_sheet(sheet_number));

		// transfer image on first sheet
		if is_first_page && image.is_some() {
			// placeholder rectangle
			for y in range(0, len(image.chars)) {
				data_cept.set_cursor(3 + y, 41 - len(image.chars[0]));
				data_cept.set_bg_color(15);
                data_cept.repeat(" ", len(image.chars[0]));
            }
			// palette
			data_cept.define_palette(image.palette);
			// DRCS
			data_cept.add_raw(image.drcs);
			// draw characters
			i = 0;
			for l in image.chars {
				data_cept.set_cursor(3 + i, 41 - len(image.chars[0]));
				data_cept.load_g0_drcs();
				data_cept.add_raw(l);
				data_cept.add_raw(b"\r\n");
                i += 1;
            }
        }

        return data_cept
    }

}
