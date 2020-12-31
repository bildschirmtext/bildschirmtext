#![allow(dead_code)]

use std::ops;

#[derive(Clone)]
pub struct Cept {
    data: Vec<u8>,
    mode: i32,
    // charset: Option<i32>,
}


impl Cept {
    pub fn new() -> Self {
        Self {
            data: vec!(),
            mode: 0,
        }
    }

    pub fn from_str(s: &str) -> Self {
        let mut cept = Cept::new();
        cept.add_str(s);
        cept
    }

    pub fn from_raw(s: &[u8]) -> Self {
        let mut cept = Cept::new();
        cept.add_raw(s);
        cept
    }

    pub fn set_mode(&mut self, mode: i32) {
        self.mode = mode;
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn add_raw(&mut self, data: &[u8]) {
        self.data.extend(data);
    }

    fn g2code(&mut self, s: &[u8]) {
        debug_assert!(s.len() != 0);
        debug_assert!(s.len() <= 2);
        if self.mode == 0 {
            self.data.push(0x19);
            self.data.push(s[0]);
        } else {
            self.data.push(s[0] | 0x80);
        }
        if s.len() > 1 {
            self.data.push(s[1]);
        }
    }

	pub fn add_str(&mut self, s_in: &str) {
        for c in s_in.chars() {
            match c {
                '¤' => self.data.push(b'$'),         // $ and ¤ are swapped
                '$' => self.g2code(b"$"), // $ and ¤ are swapped
    			// '¦' => b"?",      // not available
    			// '¨' => b"?",      // not available
                '©' => self.g2code(b"S"),
    			// "ª" => b"?",      // not available
                '\u{00ad}' => {},    // soft hyphen
                '®' => self.g2code(b"R"),
    			// '¯' => b"?",      // not available
    			// '´' => b"?",      // not available
    			// '¸' => b"?",      // not available
    			// '¹' => b"?",      // not available
    			// 'º' => b"?",      // not available
                'À' => self.g2code(b"AA"),
                'Á' => self.g2code(b"BA"),
                'Â' => self.g2code(b"CA"),
                'Ã' => self.g2code(b"DA"),
                'Ä' => self.g2code(b"HA"),
                'Å' => self.g2code(b"JA"),
                'Æ' => self.g2code(b"a"),
                'Ç' => self.g2code(b"KC"),
                'È' => self.g2code(b"AE"),
                'É' => self.g2code(b"BE"),
                'Ê' => self.g2code(b"CE"),
                'Ë' => self.g2code(b"HE"),
                'Ì' => self.g2code(b"AI"),
                'Í' => self.g2code(b"BI"),
                'Î' => self.g2code(b"CI"),
                'Ï' => self.g2code(b"HI"),
                'Ð' => self.g2code(b"b"),
                'Ñ' => self.g2code(b"DN"),
                'Ò' => self.g2code(b"AO"),
                'Ó' => self.g2code(b"BO"),
                'Ô' => self.g2code(b"CO"),
                'Õ' => self.g2code(b"DO"),
                'Ö' => self.g2code(b"HO"),
                '×' => self.g2code(b"4"),
                'Ø' => self.g2code(b"i"),
                'Ù' => self.g2code(b"AU"),
                'Ú' => self.g2code(b"BU"),
                'Û' => self.g2code(b"CU"),
                'Ü' => self.g2code(b"HU"),
                'Ý' => self.g2code(b"BY"),
                'Þ' => self.g2code(b"l"),
                'ß' => self.g2code(b"{"),
                'à' => self.g2code(b"Aa"),
                'á' => self.g2code(b"Ba"),
                'â' => self.g2code(b"Ca"),
                'ã' => self.g2code(b"Da"),
                'ä' => self.g2code(b"Ha"),
                'å' => self.g2code(b"Ja"),
                'æ' => self.g2code(b"q"),
                'ç' => self.g2code(b"Kc"),
                'è' => self.g2code(b"Ae"),
                'é' => self.g2code(b"Be"),
                'ê' => self.g2code(b"Ce"),
                'ë' => self.g2code(b"He"),
                'ì' => self.g2code(b"Ai"),
                'í' => self.g2code(b"Bi"),
                'î' => self.g2code(b"Ci"),
                'ï' => self.g2code(b"Hi"),
                'ð' => self.g2code(b"s"),
                'ñ' => self.g2code(b"Dn"),
                'ò' => self.g2code(b"Ao"),
                'ó' => self.g2code(b"Bo"),
                'ô' => self.g2code(b"Co"),
                'õ' => self.g2code(b"Do"),
                'ö' => self.g2code(b"Ho"),
                '÷' => self.g2code(b"8"),
                'ø' => self.g2code(b"u"),
                'ù' => self.g2code(b"Au"),
                'ú' => self.g2code(b"Bu"),
                'û' => self.g2code(b"Cu"),
                'ü' => self.g2code(b"Hu"),
                'ý' => self.g2code(b"Ay"),
                'þ' => self.g2code(b"|"),
                'ÿ' => self.g2code(b"Hy"),

                // arrows
                '←' => self.g2code(b","),
                '↑' => self.g2code(b"-"),
                '→' => self.g2code(b"."),
                '↓' => self.g2code(b"/"),

                // math
                '⋅' => self.g2code(b"7"),

                // line feed
                '\n' => self.data.extend(b"\r\n"),

                // latin other
                'š' => self.g2code(b"Os"),
                'Œ' => self.g2code(b"j"),
                'œ' => self.g2code(b"z"),
                'ł' => self.g2code(b"x"),
                'č' => self.g2code(b"Oc"),
                'ć' => self.g2code(b"Bc"),

                // greek
                'ŋ' => self.g2code(b"\x7e"),
                'μ' => self.g2code(b"5"),
                'Ω' => self.g2code(b"`"),

                // punctuation
                '‚' => self.g2code(b")"),
                '’' => self.g2code(b"9"),
                '‘' => self.g2code(b"9"),
                '„' => self.g2code(b"*"),
                '“' => self.g2code(b":"),
                '″' => self.g2code(b":"),
                '”' => self.g2code(b":"),
                '–' => self.g2code(b"P"),

                // look-alikes
                '†' => self.data.push(b'+'),
                '−' => self.data.push(b'-'), // MINUS SIGN
                '⟨' => self.data.push(b'<'),
                '⟩' => self.data.push(b'>'),
                '∗' => self.data.push(b'*'),
                '‐' => self.data.push(b'-'),
                '—' => self.data.push(b'-'),

                // spaces
                ' ' => self.data.push(b' '), // NARROW NO-BREAK SPACE
                ' ' => self.data.push(b' '), // THIN SPACE
                ' ' => self.data.push(b' '), // ZERO WIDTH SPACE
                ' ' => self.data.push(b' '), // EN SPACE

                // used in phonetic alphabet
                'ˈ' => self.data.push(b'\''),
                'ː' => self.data.push(b':'),

                // XXX these change the length!!
                '€' => self.data.extend(b"EUR"),
                '…' => self.data.extend(b"..."),

                // ASCII
                #[allow(overlapping_patterns)]
                ' '..='~' => self.data.push(c as u8),

                _ => {
                    // sys.stderr.write("unknown character: '" + c + "' (" + hex(ord(c)) + ")n '" + s_in + "'\n")
                    // if charset:
                        // data_cept = charset.get(c)
                        // if data_cept:
                            // s2.extend(data_cept)
                        // else:
                            // s2.append(ord('?'))
                    // else:
                    self.data.push(b'?');
                },
            };
        }
    }

    pub fn code_to_char(s1: &[u8]) -> Option<char> {
        // returns a unicode string of the single-char CEPT sequence
        // - '\0': there's is nothing we could decode in the string
        // - None: the sequence is incomplete
        if s1.len() == 0 {
            Some('\0')
        } else if s1.len() == 1 && s1[0] <= 0x7f && s1[0] != 0x19 {
            Some(s1[0] as char) // CEPT == ASCII for 0x00-0x7F (except 0x19)
        } else if s1[0] == 0x19 {
            if s1.len() == 1 {
                None
    //			sys.stderr.write("s1[1]: " + pprint.pformat(s1[1]) + "\n")
    //			sys.stderr.write("len(s1): " + pprint.pformat(len(s1)) + "\n")
            } else if s1[1] == b'H' { // "¨" prefix
                if s1.len() == 2 { // not complete
                    None
                } else {
                    if s1[2] == b'a' {
                        Some('ä')
                    } else if s1[2] == b'o' {
                        Some('ö')
                    } else if s1[2] == b'u' {
                        Some('ü')
                    } else if s1[2] == b'A' {
                        Some('Ä')
                    } else if s1[2] == b'O' {
                        Some('Ö')
                    } else if s1[2] == b'U' {
                        Some('Ü')
                    } else {
                        Some('\0')
                    }
                }
            } else if s1[1] == b'{' { // &szlig
                Some('ß')
            } else {
                Some('\0')
            }
        } else {
            Some('\0')
        }
    }

    pub fn sequence_end_of_page(&mut self) {
        self.data.extend(&[
        0x1f, 0x58, 0x41, // set cursor to line 24, column 1
        0x11,             // show cursor
        0x1a,             // end of page
        ]);
    }

	pub fn ini(&mut self) {
        self.data.push(0x13);
    }

	pub fn ter(&mut self) {
		self.data.push(0x1c);
    }

	pub fn dct(&mut self) {
		self.data.push(0x1a);
    }

	pub fn set_res_40_24(&mut self) {
		self.data.extend(&[0x1f, 0x2d]);
    }

	pub fn show_cursor(&mut self) {
		self.data.push(0x11);
    }

	pub fn hide_cursor(&mut self) {
		self.data.push(0x14);
    }

	pub fn cursor_home(&mut self) {
		self.data.push(0x1e);
    }

	pub fn cursor_left(&mut self) {
		self.data.push(0x08);
    }

	pub fn cursor_right(&mut self) {
		self.data.push(0x09);
    }

	pub fn cursor_down(&mut self) {
		self.data.push(0x0a);
    }

	pub fn cursor_up(&mut self) {
		self.data.push(0x0b);
    }

	pub fn set_cursor(&mut self, y: u8, x: u8) {
        self.data.push(0x1f);
        self.data.push(0x40 + y);
        self.data.push(0x40 + x);
    }

	pub fn clear_screen(&mut self) {
		self.data.push(0x0c);
    }

	pub fn clear_line(&mut self) {
		self.data.push(0x18);
    }

	pub fn protect_line(&mut self) {
		self.data.extend(&[0x9b, 0x31, 0x50]);
    }

	pub fn unprotect_line(&mut self) {
		self.data.extend(&[0x9b, 0x31, 0x51]);
    }

	pub fn parallel_mode(&mut self) {
		self.data.extend(&[0x1b, 0x22, 0x41]);
    }

	pub fn serial_limited_mode(&mut self) {
		self.data.extend(&[0x1f, 0x2f, 0x43]);
    }

	pub fn parallel_limited_mode(&mut self) {
		self.data.extend(&[0x1f, 0x2f, 0x44]);
    }

	pub fn repeat(&mut self, c: u8, n: u8) {
        self.data.push(c);
        self.data.push(0x12);
        self.data.push(0x40 + n - 1);
    }

    pub fn define_palette<T: AsRef<str>>(&mut self, palette: &[T], start_color: Option<u8>) {
        // let palette: &[&str] = palette.iter().collect();
        let start_color = start_color.unwrap_or(16);
		self.data.extend(&[
			0x1f, 0x26, 0x20,		  // start defining colors
            0x1f, 0x26,		          // define colors
        ]);
		self.data.push(0x30 + (start_color / 10));
		self.data.push(0x30 + (start_color % 10));

		for hexcolor in palette {
            let hexcolor = hexcolor.as_ref();
            let (r, g, b) = if hexcolor.len() == 7 {
                (
                    u8::from_str_radix(&hexcolor[1..3], 16).unwrap_or(0),
				    u8::from_str_radix(&hexcolor[3..5], 16).unwrap_or(0),
                    u8::from_str_radix(&hexcolor[5..7], 16).unwrap_or(0),
                )
            } else if hexcolor.len() == 4 {
                (
				    u8::from_str_radix(&hexcolor[1..2], 16).unwrap_or(0) << 4,
				    u8::from_str_radix(&hexcolor[2..3], 16).unwrap_or(0) << 4,
                    u8::from_str_radix(&hexcolor[3..4], 16).unwrap_or(0) << 4,
                )
            } else {
                println!("incorrect palette encoding.");
                ( 0, 0, 0)
            };
			let r0 = (r >> 4) & 1;
			let r1 = (r >> 5) & 1;
			let r2 = (r >> 6) & 1;
			let r3 = (r >> 7) & 1;
			let g0 = (g >> 4) & 1;
			let g1 = (g >> 5) & 1;
			let g2 = (g >> 6) & 1;
			let g3 = (g >> 7) & 1;
			let b0 = (b >> 4) & 1;
			let b1 = (b >> 5) & 1;
			let b2 = (b >> 6) & 1;
			let b3 = (b >> 7) & 1;
			let byte0 = 0x40 | r3 << 5 | g3 << 4 | b3 << 3 | r2 << 2 | g2 << 1 | b2;
			let byte1 = 0x40 | r1 << 5 | g1 << 4 | b1 << 3 | r0 << 2 | g0 << 1 | b0;
			self.data.push(byte0);
            self.data.push(byte1);
        }
    }

	pub fn set_palette(&mut self, pal: u8) {
        self.data.push(0x9b);
        self.data.push(0x30 + pal);
        self.data.push(0x40);
    }

	pub fn set_fg_color_simple(&mut self, c: u8) {
        self.data.push(0x80 + c);
    }

	pub fn set_bg_color_simple(&mut self, c: u8) {
        self.data.push(0x90 + c);
    }

	pub fn set_fg_color(&mut self, c: u8) {
        self.set_palette(c >> 3);
        self.set_fg_color_simple(c & 7);
    }

	pub fn set_bg_color(&mut self, c: u8) {
        self.set_palette(c >> 3);
        self.set_bg_color_simple(c & 7);
    }

	pub fn set_line_bg_color_simple(&mut self, c: u8) {
        self.data.extend(&[0x1b, 0x23, 0x21]);
        self.data.push(0x50 + c);
    }

	pub fn set_line_bg_color(&mut self, c: u8) {
        self.set_palette(c >> 3);
        self.set_line_bg_color_simple(c & 7);
    }

	pub fn set_screen_bg_color_simple(&mut self, c: u8) {
        self.data.extend(&[0x1b, 0x23, 0x20]);
        self.data.push(0x50 + c);
    }

	pub fn set_screen_bg_color(&mut self, c: u8) {
        self.set_palette(c >> 3);
        self.set_screen_bg_color_simple(c & 7);
    }

	pub fn set_line_fg_color_simple(&mut self, c: u8) {
        self.data.extend(&[0x1b, 0x23, 0x21]);
        self.data.push(0x40 + c);
    }

	pub fn set_left_g0(&mut self) {
		self.data.push(0x0f);
    }

	pub fn set_left_g3(&mut self) {
		self.data.extend(&[0x1b, 0x6f]);
    }

	pub fn load_g0_drcs(&mut self) {
		self.data.extend(&[0x1b, 0x28, 0x20, 0x40]);
    }

	pub fn load_g0_g0(&mut self) {
		self.data.extend(&[0x1b, 0x28, 0x40]);
    }

	pub fn service_break(&mut self, y: u8) {
        self.data.extend(&[0x1f, 0x2f, 0x40]);
        self.data.push(0x40 + y);
    }

	pub fn service_break_back(&mut self) {
		self.data.extend(&[0x1f, 0x2f, 0x4f]);
    }

	pub fn normal_size(&mut self) {
		self.data.push(0x8c);
    }

	pub fn double_height(&mut self) {
		self.data.push(0x8d);
    }

	pub fn double_width(&mut self) {
		self.data.push(0x8e);
    }

	pub fn double_size(&mut self) {
		self.data.push(0x8f);
    }

	pub fn underline_off(&mut self) {
		self.data.push(0x99);
    }

	pub fn underline_on(&mut self) {
		self.data.push(0x9a);
    }

	pub fn hide_text(&mut self) {
		self.data.push(0x98);
    }

	pub fn code_9d(&mut self) {
		self.data.push(0x9d);
    }

	pub fn code_9e(&mut self) {
		self.data.push(0x9e);
    }

    pub fn extend(&mut self, other: &Cept) {
        self.data.extend(&other.data);
    }
}

impl ops::Add<Cept> for Cept {
    type Output = Cept;

    fn add(self, rhs: Cept) -> Cept {
        let mut cept = self.clone();
        cept.add_raw(&rhs.data);
        cept
    }
}