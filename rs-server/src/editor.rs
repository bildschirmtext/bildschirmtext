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

use super::cept::*;
use super::pages::*;

struct Editor {
    input_field: InputField,
    data: Vec<String>,
}

impl Editor {
    fn new(input_field: InputField) -> Self {
        let data = vec!(input_field.default.clone().unwrap_or_default());
        Editor { input_field: input_field.clone(), data }
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
}
