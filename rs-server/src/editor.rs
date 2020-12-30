// An Editor object is used for single or multi line text input. Every field on
// a dialog page is backed by one Editor object.
//
// ## Features
//
// * An editor has a position, a size, a foreground and a background color. If
//   the color properties are set, it will draw its own background.
// * An editor can be given a list of legal inputs.
//   If end_on_illegal_character is True, as soon as a character is entered
//   that makes the current contents of the editor illegal, the edit() method
//   returns the illegal string.
//   If end_on_illegal_character is False, characters that would make the input
//   illegal are ignored.
//   If end_on_legal_string is True, the edit() method returns as soon as a
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

use std::io::Write;
use super::cept::*;

#[derive(Clone)]
pub enum InputType {
    Normal,
    Password,
}

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
    pub typ: InputType,
    pub cursor_home: bool,
    pub clear_line: bool,
    pub legal_values: Vec<String>,
    pub end_on_illegal_character: bool,
    pub end_on_legal_string: bool,
    pub echo_ter: bool,
    pub no_navigation: bool,
    pub default: Option<String>,
}

#[derive(Default)]
pub struct Inputs {
    pub fields: Vec<InputField>,
    pub confirm: bool,
    pub no_55: bool,
}

struct Editor {
    input_field: InputField,
    data: Vec<String>,
    x: u8,
    y: u8,
    last_c: char,
}

impl Editor {
    fn new(input_field: InputField) -> Self {
        let data = vec!(input_field.default.clone().unwrap_or_default());
        Editor { input_field: input_field.clone(), data, x: 0, y: 0, last_c: '\0' }
    }

	pub fn string(self) -> String {
		let mut string = String::new();
		for l in self.data {
            string += l.trim_end();
            string.push('\n');
        }
        while string.ends_with('\n') {
            string.pop();
        }
        string
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

            let l = match self.input_field.typ {
                InputType::Password => "*".repeat(l.len()),
			    _ => {
                    if l.starts_with("\x13") { // XXX Cept.ini()
                        "*".to_string() + &l[1..]
                    } else {
                        l.to_string()
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
        stream.write_all(cept.data()).unwrap();
        stream.flush();
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
            stream.write_all(cept.data()).unwrap();
            stream.flush();
        }
    }

	pub fn try_insert_character(&mut self, c: char) {
		if self.x < self.input_field.width {
            let y = self.y as usize;
            self.data[y].insert(self.x as usize, c);
        }
    }

    pub fn insert_character(&mut self, s: &mut String, c: char) -> bool {
		if self.x < self.input_field.width {
            self.try_insert_character(c);
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
            stream.write_all(cept.data()).unwrap();
            stream.flush();
        }
    }

	pub fn insert_line_feed(&mut self, stream: &mut impl Write) {
		if self.y < self.input_field.height - 1 {
			self.y += 1;
            let mut cept = Cept::new();
            cept.add_str("\r");
            stream.write_all(cept.data()).unwrap();
            stream.flush();
        }
    }

	pub fn insert_control_character(&mut self, c: char, stream: &mut impl Write) {
		match c {
            '\r' => { // enter
                // some terminals send CR/LF, others just CR, so we have to do
                // the work on CR, and ignore LF if it was preceded by a CR
                self.insert_carriage_return();
                self.insert_line_feed();
            },
            '\n' => { // down
                if self.last_c != '\r' { // see above
                    self.insert_line_feed();
                }
            },
            '\x08' => { // left
                if self.x > 0 {
                    self.x -= 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    stream.write_all(cept.data()).unwrap();
                    stream.flush();
                }
            },
            '\x0b' => { // up
                if self.y > 0 {
                    self.y -= 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    stream.write_all(cept.data()).unwrap();
                    stream.flush();
                }
            },
            '\x09' => { // right
                if self.x < self.input_field.width {
                    self.x += 1;
                    let cept = Cept::from_raw(&[c as u8]);
                    stream.write_all(cept.data()).unwrap();
                    stream.flush();
                }
            },
        }
        self.last_c = c
    }
}
