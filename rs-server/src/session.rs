use std::io::{Read, Write};
use std::collections::HashMap;
use std::str::FromStr;
use super::cept::*;
use super::editor::*;
use super::pages::*;
use super::user::*;

const INPUT_NAME_NAVIGATION: &'static str = "$navigation";

enum InputEvent {
    Command(String),                     // user entered a command ("*nnn#")
    Navigation(String),                  // user entered a navigation link
    TextFields(HashMap<String, String>), // user finished filling the page's text fields
}

#[derive(Clone)]
pub struct PageId {
    pub page: String,
    pub sub: usize,
}

impl PageId {
    fn empty() -> Self {
        PageId {
            page: "".to_owned(),
            sub: 0
        }
    }

    pub fn reduced_by(&self, n: usize) -> Self {
        PageId {
            page: self.page[n..].to_owned(),
            sub: self.sub
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = self.page.clone();
        s.push((b'a' + self.sub as u8) as char);
        s
    }
}

impl FromStr for PageId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let last_char = s.chars().last().unwrap().to_ascii_lowercase();
        if last_char.is_alphabetic() {
            Ok(PageId {
                page: s[0..s.len() - 1].to_owned(),
                sub: (last_char as u8 - b'a') as usize
            })
        } else {
            Ok(PageId {
                page: s.to_owned(),
                sub: 0
            })
        }
    }
}

pub enum Error {
    None,
    Code(usize),
    Custom(String),
}

pub enum ActionResult {
    Ok,
	Error,
	Restart,
}

pub enum UserRequest {
    Goto(PageId, bool),
    SendAgain,
    Error(Error)
}

pub struct ClientState {
    pub palette: Option<String>,
    pub include: Option<String>,
}

pub struct Session {
    user: Option<User>,
    client_state: ClientState,
    current_pageid: PageId,
    history: Vec<PageId>,
    autoplay: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            user: None,
            client_state:ClientState {
                palette: None,
                include: None,
            },
            current_pageid: PageId::empty(),
            history: vec!(),
            autoplay: false,
        }
    }

    // Main loop: show page, get user input, loop.
    // This is only called once and loops forever
    pub fn run(&mut self, stream: &mut (impl Write + Read))
    {
        let mut target_pageid = PageId::from_str("00000").unwrap();
        let mut add_to_history = false;
        let mut links = None;
        let mut inputs = None;
        let mut current_page_cept = Cept::new();

        'main: loop {
            // XXX if User.user() is not None:
            // 	User.user().stats.update()

            // *** show page
            println!("showing page: {}", target_pageid.to_string());
            if let Some(page) = super::dispatch::get_page(&target_pageid, self.user.as_ref()) {
                current_page_cept = page.construct_page_cept(&mut self.client_state, &target_pageid);
                write_stream(stream, current_page_cept.data());
                links = page.meta.links;
                inputs = page.meta.inputs;
                self.autoplay = page.meta.autoplay == Some(true);
                self.current_pageid = target_pageid.clone();
                if add_to_history {
                    self.history.push(self.current_pageid.clone());
                };
            } else {
                println!("ERROR: Page not found: {}", target_pageid.to_string());
                let error = if target_pageid.sub > 0 {
                    101
                } else {
                    100
                };
                show_error(&Error::Code(error), stream);
            }

            'input: loop {
                // *** get user input
                let input_data = self.get_inputs(&self.current_pageid, inputs.as_ref(), links.as_ref(), stream);

                // *** handle input
                let req = match input_data {
                    InputEvent::Command(command_input) => {
                        self.decode_command(&command_input)
                    },
                    InputEvent::Navigation(val) => {
                        self.decode_link(links.as_ref(), &val)
                    },
                    InputEvent::TextFields(input_data) => {
                        self.decode_text_fields(&self.current_pageid, inputs.as_ref(), &input_data)
                    },
                };
                match req {
                    UserRequest::Goto(t, a) => {
                        target_pageid = t;
                        add_to_history = a;
                        continue 'main;
                    },
                    UserRequest::SendAgain => {
                        write_stream(stream, current_page_cept.data());
                    },
                    UserRequest::Error(e) => {
                        show_error(&e, stream);
                        continue 'input;
                    }
                }

            }
        }
    }

    // Handle page interactivity:
    // * for pages with text fields, draw them and allow editing them
    // * for pages with without text fields, allow entering a link
    // In both cases, it is possible to escape into command mode.
    fn get_inputs(&self, pageid: &PageId, inputs: Option<&Inputs>, links: Option<&Vec<Link>>, stream: &mut (impl Write + Read)) -> InputEvent {
        if self.autoplay {
            println!("autoplay!");
            InputEvent::Navigation("".to_owned()) // inject "#"
        } else {
            let i;
            let inputs = if inputs.is_none() {
                // for pages without text fields, create a single text
                // field at the bottom of the screen to input a link
                let mut legal_values = vec!();
                if let Some(links) = links.clone() {
                    for link in links {
                        if link.code != "#" {
                            legal_values.push(link.code.clone());
                        }
                    }
                }
                i = Inputs {
                    fields: vec!(
                        InputField {
                            name: INPUT_NAME_NAVIGATION.to_string(),
                            line: 24,
                            column: 1,
                            height: 1,
                            width: 20,
                            fgcolor: None,
                            bgcolor: None,
                            hint: None,
                            input_type: InputType::Normal,
                            cursor_home: false,
                            clear_line: false,
                            legal_values: Some(legal_values),
                            end_on_illegal_character: true,
                            end_on_legal_string: true,
                            echo_ter: true,
                            command_mode: false,
                            action: None,
                            default: None,
                        }),
                    confirm: false,
                    no_55: true,
                    no_navigation: false,
                    price: None,
                    action: None,
                };
                &i
            } else {
                inputs.unwrap()
            };

            // create editors and draw backgrounds
            let mut editors = vec!();
            for input_field in &inputs.fields {
                let mut editor = Editor::new(input_field);
                editor.no_navigation = inputs.no_navigation;
                editor.draw(stream);
                editors.push(editor);
            }

            // get all inputs
            let mut input_data = HashMap::new();
            let mut i = 0;
            let mut skip = false;
            while i < inputs.fields.len() {
                let input_field = &inputs.fields[i];

                let (val, dct) = editors[i].edit(skip, stream);

                if dct {
                    skip = true;
                }

                if let Some(val) = &val {
                    if val.starts_with(0x13 as char) { // XXX Cept.ini()
                        return InputEvent::Command(val[1..].to_string());
                    }
                }

                input_data.insert(input_field.name.to_string(), val.unwrap().to_string());


                let action_result = if let Some(action) = input_field.action {
                    action(&pageid, &input_data)
                } else {
                    ActionResult::Ok
                };

                match action_result {
                    ActionResult::Ok => {
                        i += 1;
                    },
                    ActionResult::Error => {
                        skip = false;
                        continue;
                    },
                    ActionResult::Restart => {
                        i = 0;
                        skip = false;
                        continue;
                    }
                }
            }

            // ask for confirmation
            if inputs.confirm {
                if Self::confirm(&inputs, stream) {
                    // if inputs.action == "send_message" {
                    // 	User.user().messaging.send(input_data["user_id"], input_data["ext"], input_data["body"])
                    // 	system_message_sent_message()
                    // } else {
                    //     // TODO we stay on the page, in the navigator?
                    // }
                }
            } else if !inputs.no_55 {
                let cept = create_system_message(&Error::Code(55), None);
                write_stream(stream, cept.data());
            }

            // send "input_data" to "inputs.target"
            if let Some(val) = input_data.get(INPUT_NAME_NAVIGATION) {
                return InputEvent::Navigation(val.to_owned())
            } else {
                return InputEvent::TextFields(input_data);
            }

        }
    }

    fn confirm(inputs: &Inputs, stream: &mut (impl Write + Read)) -> bool { // "send?" message
        let price = inputs.price;
        let mut cept = if price.is_some() && price != Some(0) {
            create_system_message(&Error::Code(47), price)
        } else {
            create_system_message(&Error::Code(44), None)
        };
        cept.set_cursor(24, 1);
        cept.sequence_end_of_page();
        write_stream(stream, cept.data());

        // TODO: use an editor for this, too!
        let mut seen_a_one = false;
        loop {
            let c = readchar(stream);
            if c == b'2' {
                write_stream(stream, &[c]);
                return false;
            } else if c == b'1' && !seen_a_one {
                write_stream(stream, &[c]);
                seen_a_one = true;
            } else if c == b'9' && seen_a_one {
                write_stream(stream, &[c]);
                return true;
            } else if c == 8 && seen_a_one {
                write_stream(stream, &[c]);
                seen_a_one = false;
            }
        }
    }

    // Decode a global command code ("*nnn#").
    // This could be
    // * an explicit page numer
    // * "*00#" to re-send the current page CEPT data (e.g. after a transmission error)
    // * "*09#" to reload the current page (may fetch a newer version of the page)
    // * "*#" to go back
    fn decode_command(&mut self, command_input: &str) -> UserRequest {
        if command_input == "" {
            // *# = back
            println!("command: back");
            if self.history.len() < 2 {
                println!("ERROR: No history.");
                UserRequest::Error(Error::Code(10))
            } else {
                let _ = self.history.pop();
                let mut target_pageid = self.history.pop().unwrap();
                // if we're navigating back across page numbers...
                if target_pageid.sub != self.current_pageid.sub {
                    // if previous page was sub-page, keep going back until "a"
                    while target_pageid.sub != 0 {
                        target_pageid = self.history.pop().unwrap();
                    }
                }
                UserRequest::Goto(target_pageid, false)
            }
        } else if command_input == "09" {
            // hard reload
            println!("command: hard reload");
            // invalidate palette and include
            self.client_state.palette = None;
            self.client_state.include = None;
            UserRequest::Goto(self.current_pageid.clone(), false)
        } else if command_input == "00" {
            // re-send CEPT data of current page
            println!("command: resend");
            UserRequest::SendAgain
        } else {
            UserRequest::Goto(PageId::from_str(command_input).unwrap(), true)
        }
    }

    fn decode_link(&self, links: Option<&Vec<Link>>, val: &str) -> UserRequest {
        if let Some(links) = links {
            for link in links {
                if (*val == link.code) || (val == "" && link.code == "#") {
                    return UserRequest::Goto(PageId::from_str(&link.target).unwrap(), true);
                }
            }
        }
        // not found
        if val == "" {
            // next sub-page
            let pageid = PageId { page: self.current_pageid.page.clone(), sub: self.current_pageid.sub + 1 };
            UserRequest::Goto(pageid, true)
        } else {
            println!("ERROR: Illegal navigation");
            UserRequest::Error(Error::Code(100))
        }
    }

    fn decode_text_fields(&self, pageid: &PageId, inputs: Option<&Inputs>, input_data: &HashMap<String, String>) -> UserRequest {
        let action_result = if let Some(action) = inputs.unwrap().action {
            action(&pageid, &input_data)
        } else {
            UserRequest::SendAgain // XXX
        };
        action_result
    }
}

fn show_error(error: &Error, stream: &mut (impl Write + Read)) {
    let mut cept = create_system_message(error, None);
    cept.sequence_end_of_page();
    write_stream(stream, cept.data());
    wait_for_ter(stream);
}
