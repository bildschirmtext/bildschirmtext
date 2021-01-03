// An Editor object is used for single or multi line text input. Every field on
// a dialog page is backed by one Editor object.
//
// ## Features
//
// * An editor has a position, a size, a foreground and a background color. If
//   the color properties are set, it will draw its own background.
// * An editor can be given a list of legal inputs.
//   If end_on_illegal_character is true, as soon as a character is entered
//   that makes the current contents of the editor illegal, the edit() method
//   returns the illegal string.
//   If end_on_illegal_character is false, characters that would make the input
//   illegal are ignored.
//   If end_on_legal_string is true, the edit() method returns as soon as a
//   legal string is completed.
//
// ## Command Mode
//
// Within any editor, "*" will create a "command mode" child editor in line 24
// that allows entering any global *...# code.
//
// In command mode, two "*" characters or one "#" character will exit command
// mode and the resulting global code will be sent back to the original
// editor.
//
// The parent editor will then
// * interpret editor codes (** to clear editor, *022# for cursor up etc.)
// * instruct the main loop to navigate to the page in case of a page number
//
// ## Main Editor
//
// The main editor that is presented in line 24 after a non-dialog page is
// shown is just a normal editor that happens to be in line 24, which is
// passed the list of links as legal inputs. "*" will create a command mode
// editor on top of the main editor in line 24.

use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use super::cept::*;

#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum InputType {
    Normal,
    Number,
    Alpha,
    Password,
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Normal
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct InputField {
    pub name: String,
    pub line: u8,
    pub column: u8,
    pub height: u8,
    pub width: u8,
    pub fgcolor: Option<u8>,
    pub bgcolor: Option<u8>,
    pub hint: Option<String>,
    pub legal_values: Option<Vec<String>>,
    pub default: Option<String>,
    #[serde(default)]
    pub input_type: InputType,
    #[serde(default)]
    pub cursor_home: bool,
    #[serde(default)]
    pub clear_line: bool,
    #[serde(default)]
    pub end_on_illegal_character: bool,
    #[serde(default)]
    pub end_on_legal_string: bool,
    #[serde(default)]
    pub echo_ter: bool,
    #[serde(default)]
    pub command_mode: bool,
    #[serde(default)]
    pub no_navigation: bool,
    #[serde(default)]
    pub validate: bool,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Default)]
pub struct Inputs {
    pub fields: Vec<InputField>,
    pub target: Option<String>,
    #[serde(default)]
    pub confirm: bool,
    #[serde(default)]
    pub no_55: bool,
    #[serde(default)]
    pub no_navigation: bool,
}

pub struct Editor {
    input_field: InputField,
    data: Vec<String>,
    x: u8,
    y: u8,
    last_c: char,
}

impl Editor {
    pub fn new(input_field: &InputField) -> Self {
        let data = vec!(input_field.default.clone().unwrap_or_default());
        Editor { input_field: input_field.clone(), data, x: 0, y: 0, last_c: '\0' }
    }

	pub fn string(&self) -> String {
		let mut string = String::new();
		for l in &self.data {
            string += l.trim_end();
            string.push('\n');
        }
        while string.ends_with('\n') {
            string.pop();
        }
        string
    }

	pub fn set_string(&mut self, string: &str) {
		self.data = vec!();
		for line in string.lines().take(self.input_field.height as usize) {
            let mut line = line.to_owned();
            while line.len() < self.input_field.width as usize {
                line.push(' ');
            }
            self.data.push(line);
        }
        let mut empty_line = String::new();
        while empty_line.len() < self.input_field.width as usize {
            empty_line.push(' ');
        }
        while self.data.len() < self.input_field.height as usize {
            self.data.push(empty_line.clone());
        }
//		sys.stderr.write("self.__data:\n" + pprint.pformat(self.__data) + "\n")
    }

	pub fn set_color(&self) -> Cept {
		let mut cept = Cept::new();
		if let Some(fgcolor) = self.input_field.fgcolor {
            cept.set_fg_color(fgcolor);
        }
		if let Some(bgcolor) = self.input_field.bgcolor {
            cept.set_bg_color(bgcolor);
        }
        cept
    }

	pub fn draw(&self, stream: &mut impl Write) {
        let mut cept = Cept::new();
		cept.parallel_limited_mode();
		cept.hide_cursor();
		cept.set_cursor(self.input_field.line, self.input_field.column);
		let fill_with_clear_line = self.input_field.clear_line && self.input_field.width == 40;
		let fill_with_spaces = self.input_field.clear_line && !fill_with_clear_line;
		for i in 0..self.input_field.height as usize {
			let l = self.data[i].trim_end();

            let l = match self.input_field.input_type {
                InputType::Password => "*".repeat(l.len()),
			    _ => {
                    if l.starts_with("\x13") { // XXX cept_ini()
                        "*".to_owned() + &l[1..]
                    } else {
                        l.to_owned()
                    }
                }
            };

			if l.len() != 0 {
                cept.extend(&self.set_color());
            }

			if fill_with_clear_line {
                cept.clear_line();
                if let Some(bgcolor) = self.input_field.bgcolor {
                    cept.set_line_bg_color(bgcolor);
                }
            }

            cept.add_str(&l);

			if fill_with_spaces && l.len() > self.input_field.width as usize {
                cept.add_str(&" ".repeat(self.input_field.width as usize - l.len()));
            }

			if i != self.input_field.height as usize - 1 {
				if self.input_field.column == 1 {
					if self.input_field.width != 40 || fill_with_clear_line {
                        cept.add_str("\n");
                    }
                } else {
                    cept.set_cursor(self.input_field.line + i as u8 + 1, self.input_field.column);
                }
            }
        }
        write(stream, cept.data());
    }

	pub fn print_hint(&self, stream: &mut impl Write) {
		if let Some(hint) = &self.input_field.hint {
            let mut cept = Cept::new();
            cept.set_mode(1);
			cept.service_break(24);
			cept.clear_line();
			cept.add_str(&hint);
			cept.hide_text();
			cept.service_break_back();
            write(stream, cept.data());
        }
    }

	pub fn try_insert_character(&mut self, c: char) -> String {
        let mut s = self.data[self.y as usize].clone();
		if self.x < self.input_field.width {
            s.insert(self.x as usize, c);
        }
        s
    }

    pub fn insert_character(&mut self, c: char) -> bool {
		if self.x < self.input_field.width {
            self.data[self.y as usize].insert(self.x as usize, c);
            self.x += 1;
            true
        } else {
            false
        }
    }

	pub fn insert_carriage_return(&mut self, stream: &mut impl Write) {
		if self.x != 0 {
			self.x = 0;
            let mut cept = Cept::new();
			if self.input_field.column == 1 {
				cept.add_str("\r");
            } else {
                cept.set_cursor(self.input_field.line + self.y, self.input_field.column);
            }
			cept.extend(&self.set_color());
            write(stream, cept.data());
        }
    }

	pub fn insert_line_feed(&mut self, stream: &mut impl Write) {
		if self.y < self.input_field.height - 1 {
			self.y += 1;
            let mut cept = Cept::new();
            cept.add_str("\r");
            write(stream, cept.data());
        }
    }

	pub fn insert_control_character(&mut self, c: char, stream: &mut impl Write) {
		match c {
            '\r' => { // enter
                // some terminals send CR/LF, others just CR, so we have to do
                // the work on CR, and ignore LF if it was preceded by a CR
                self.insert_carriage_return(stream);
                self.insert_line_feed(stream);
            },
            '\n' => { // down
                if self.last_c != '\r' { // see above
                    self.insert_line_feed(stream);
                }
            },
            '\x08' => { // left
                if self.x > 0 {
                    self.x -= 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    write(stream, cept.data());
                }
            },
            '\x0b' => { // up
                if self.y > 0 {
                    self.y -= 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    write(stream, cept.data());
                }
            },
            '\x09' => { // right
                if self.x < self.input_field.width {
                    self.x += 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    write(stream, cept.data());
                }
            },
            _ => {},
        }
        self.last_c = c
    }

	pub fn edit(&mut self, skip_entry: bool, stream: &mut (impl Write + Read)) -> (Option<String>, bool) {
		let mut start = true;
		let mut dct = false;
		let mut prefix = vec!();
		let mut inject_char: Option<u8> = None;

		loop {
			if start && !skip_entry {
				start = false;
				self.print_hint(stream);
				let mut cept = Cept::new();
				self.y = 0;
				if self.input_field.height > 1 || self.input_field.cursor_home {
					cept.set_cursor(self.input_field.line, self.input_field.column);
                    self.x = 0;
                } else {
                    let string_len = self.string().len() as u8;
                    cept.set_cursor(self.input_field.line, self.input_field.column + string_len);
                    self.x = string_len;
                }
				if let Some(fgcolor) = self.input_field.fgcolor {
                    cept.set_fg_color(fgcolor);
                }
				if let Some(bgcolor) = self.input_field.bgcolor {
                    cept.set_bg_color(bgcolor);
                }
				cept.show_cursor();
                write(stream, cept.data());
            }

			if skip_entry {
				println!("skipping");
                break;
            }

            let mut c;
			if let Some(i) = inject_char {
				c = i;
				inject_char = None;
            } else {
                c = readchar(stream);
            }
			println!("In: {:x}", c);

			if self.input_field.command_mode && c == cept_ini() && self.string().chars().last().unwrap() == cept_ini() as char {
				// exit command mode, tell parent to clear
                return (None, false);
            }

            let mut x = prefix.clone();
            x.push(c);
			let c2 = Cept::code_to_char(&x);
			if !c2.is_some() { // sequence not complete
				prefix.push(c);
                continue;
            }
			prefix = vec!();
			if c2.unwrap() == '\0' { // we couldn't decode it
                continue
            }
			c = c2.unwrap() as u8; // XXX

			// if c < 0x20
			//     c is a CEPT control code
			// if c >= 0x20
			//     c is Unicode

			if c < 0x20 { //and c != cept_ini():
				prefix = vec!();
				if c == cept_ini() {
					if !self.input_field.command_mode {
                        println!("entering command mode");
                        let input_field = InputField {
                            name: "".to_owned(),
                            line: 24,
                            column: 1,
                            height: 1,
                            width: 20,
                            fgcolor: None,
                            bgcolor: None,
                            hint: None,
                            input_type: InputType::Normal,
                            clear_line: true,
                            legal_values: None,
                            echo_ter: true,
                            command_mode: true,

                            default: None,
                            cursor_home: false,
                            end_on_illegal_character: false,
                            end_on_legal_string: false,
                            no_navigation: false,
                            validate: false,
                        };
                        let mut editor = Editor::new(&input_field);
                        editor.set_string(&cept_ini_str());
						editor.draw(stream);
						let (val, dct) = editor.edit(false, stream);
                        println!("exited command mode");
                        if let Some(val) = val {
                            // Editor.debug_print(val);
                            let mut x = cept_ini_str().to_owned();
                            x += "02";
							if val.starts_with(&x) && val.len() == 4 {
								// editor codes *021# etc.
                                let code = val[3..].parse().unwrap();
                                let c = match code {
									1 => Some('\r'),   // CR
									2 => Some('\x0b'), // UP
									4 => Some('\x08'), // LEFT
									6 => Some('\x09'), // RIGHT
									8 => Some('\n'),   // DOWN
                                    9 => Some('\x1a'),  // DCT
                                    _ => None,
								};
                                if let Some(c) = c {
                                    inject_char = Some(c as u8);
                                } else {
                                    println!("ignoring invalid editor code");
                                }
                            } else {
								// global code
                                let mut x1 = cept_ini_str().to_owned();
                                x1 += "00";
                                let mut x2 = cept_ini_str().to_owned();
                                x2 += "09";
                                    if !self.input_field.no_navigation || val == x1 || val == x2 {
                                    return (Some(val), false);
                                }
                                println!("ignoring navigation");
                            }
                        } else { // "**" in command mode
                            self.set_string("");
                            self.draw(stream);
                        }
						start = true;
                        continue;
                    }
                } else if c == cept_ter() {
					if self.input_field.echo_ter {
                        write(stream, &[b'#']);
                    }
                    break
                } else if c == cept_dct() {
					dct = true;
                    break;
                }
				self.insert_control_character(c as char, stream)
            } else { // c >= 0x20
				let mut character_legal = true;
				let mut string_legal = false;
				// CEPT doesn't have a concept of backspace, so the backspace key
				// sends the sequence CSR_LEFT, SPACE, CSR_LEFT. It is very tricky
				// to detect this properly, so we will just allow spaces in
				// "number" and "alpha" input fields.
				if self.input_field.input_type == InputType::Number && !c.is_ascii_digit() && c != b' ' {
					character_legal = false;
                } else if self.input_field.input_type == InputType::Alpha && !c.is_ascii_alphabetic() && c != b' ' {
					character_legal = false;
				} else {
                    let x = self.try_insert_character(c as char);
                    let s = x.trim_end();
                    if let Some(legal_values) = &self.input_field.legal_values {
                        character_legal = false;
                        for legal_input in legal_values {
                            if s == legal_input {
                                character_legal = true;
                                string_legal = true;
                                println!("string_legal!");
                                break
                            } else if legal_input.starts_with(s) {
                                character_legal = true;
                                println!("character_legal!");
                                break;
                            }
                        }
                    }
                }
				if character_legal || self.input_field.end_on_illegal_character {
					if self.insert_character(c as char) {
						if self.input_field.input_type == InputType::Password {
                            write(stream, &[b'*']);
                        } else {
                            let mut cept = Cept::new();
                            let c = c as char;
                            cept.add_str(&c.to_string());
                            write(stream, cept.data());
                        }
                    }
                }
				if !character_legal && self.input_field.end_on_illegal_character {
                    break;
                }
				if string_legal && self.input_field.end_on_legal_string {
                    break;
                }
            }
        }
			// sys.stderr.write("self.data:\n" + pprint.pformat(selfdata) + "\n")
			// sys.stderr.write("self.string:\n" + pprint.pformat(self.string) + "\n")


        return (Some(self.string()), dct);
    }
}

fn readchar(stream: &mut impl Read) -> u8 {
    let mut buf = [0];
    stream.read(&mut buf);
    buf[0]
}

fn cept_ini() -> u8 {
    0x13
}

fn cept_ter() -> u8 {
    0x1c
}

fn cept_dct() -> u8 {
    0x1a
}

fn cept_ini_str() -> &'static str {
    "\x13"
}

fn write(stream: &mut impl Write, data: &[u8]) {
    stream.write_all(data).unwrap();
    stream.flush();
}