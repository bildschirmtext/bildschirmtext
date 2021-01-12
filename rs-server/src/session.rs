use super::user::*;
use std::io::{Read, Write};
use std::collections::HashMap;
use std::str::FromStr;
use super::cept::*;
use super::editor::*;
use super::page::*;
use super::dispatch::*;
use super::sysmsg::*;

const INPUT_NAME_NAVIGATION: &'static str = "$navigation";

enum InputEvent {
    Command(String),                     // user entered a command ("*nnn#")
    Navigation(String),                  // user entered a navigation link
    TextFields(HashMap<String, String>), // user finished filling the page's text fields
}

#[derive(Clone)]
#[derive(Debug)]
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

    // XXX implement Display trait instead!
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

pub enum ValidateResult {
    Ok,
	Error(SysMsg),
	Restart,
}

pub enum UserRequest {
    Login(UserId, String),
    MessageGoto(SysMsg, PageId, bool),
    Goto(PageId, bool),
    SendAgain,
    Error(SysMsg)
}

pub struct ClientState {
    pub cept_palette: Option<Cept>,
    pub cept_include: Option<Cept>,
}

pub struct Session {
    user: User,           // the actual logged-in user
    anonymous_user: User, // the user's anonymous alter ego
    client_state: ClientState,
    current_pageid: PageId,
    history: Vec<PageId>,
    autoplay: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            user: User::anonymous(),
            anonymous_user: User::anonymous(),
            client_state:ClientState {
                cept_palette: None,
                cept_include: None,
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
            // dispatch page
            let (publisher_name, page_session) = super::dispatch::dispatch_pageid(&target_pageid, &self.user, &self.anonymous_user);

            // *** show page
            println!("showing page: {}", target_pageid.to_string());
            let page = page_session.create();
            if let Some(mut page) = page {
                page.meta.publisher_name = Some(publisher_name.to_owned());
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
                    SysMsgCode::SubPageDoesNotExist
                } else {
                    SysMsgCode::PageDoesNotExist
                };
                show_sysmsg(&SysMsg::new(error), stream);
            }

            self.user.update_stats();

            'input: loop {
                // *** get user input
                let input_event = self.get_inputs(inputs.as_mut(), links.as_ref(), &page_session, stream);

                // *** handle input
                let req = match input_event {
                    InputEvent::Command(command_input) => {
                        self.decode_command(&command_input)
                    },
                    InputEvent::Navigation(val) => {
                        self.decode_link(links.as_ref(), &val)
                    },
                    InputEvent::TextFields(input_data) => {
                        page_session.send(&input_data)
                    },
                };
                match req {
                    UserRequest::Login(userid, password) => {
                        if let Some(user) = User::login(&userid, &password) {
                            println!("login ok");
                            self.user = user;
                            target_pageid = PageId::from_str("000001").unwrap();
                            add_to_history = false;
                            continue 'main;
                        } else {
                            println!("login incorrect");
                            show_sysmsg(&SysMsg::Custom("Ungültiger Teilnehmer/Kennwort -> #".to_owned()), stream);
                            continue 'input;
                        }
                    },
                    UserRequest::Goto(t, a) => {
                        target_pageid = t;
                        add_to_history = a;
                        continue 'main;
                    },
                    UserRequest::MessageGoto(e, t, a) => {
                        show_sysmsg(&e, stream);
                        target_pageid = t;
                        add_to_history = a;
                        continue 'main;
                    },
                    UserRequest::SendAgain => {
                        write_stream(stream, current_page_cept.data());
                    },
                    UserRequest::Error(sysmsg) => {
                        show_sysmsg(&sysmsg, stream);
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
    fn get_inputs(&self, inputs: Option<&mut Inputs>, links: Option<&Vec<Link>>, page_session: &Box<dyn PageSession<'static>>, stream: &mut (impl Write + Read)) -> InputEvent {
        if self.autoplay {
            println!("autoplay!");
            InputEvent::Navigation("".to_owned()) // inject "#"
        } else {
            let mut i;
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
                            validate: false,
                            default: None,
                        }),
                    confirm: false,
                    no_55: true,
                    prohibit_command_mode: false,
                    price: None,
                };
                &mut i
            } else {
                inputs.unwrap()
            };

            // create editors and draw backgrounds
            let mut editors = vec!();
            for input_field in &inputs.fields {
                let mut editor = Editor::new(input_field);
                editor.prohibit_command_mode = inputs.prohibit_command_mode;
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
                    if val.starts_with(CEPT_INI as char) {
                        return InputEvent::Command(val[1..].to_string());
                    }
                }

                input_data.insert(input_field.name.to_string(), val.unwrap().to_string());

                let validate_result = if input_field.validate {
                    page_session.validate(&input_field.name, &input_data)
                } else {
                    ValidateResult::Ok
                };
                match validate_result {
                    ValidateResult::Ok => {
                        i += 1;
                    },
                    ValidateResult::Error(sysmsg) => {
                        show_sysmsg(&sysmsg, stream);
                        skip = false;
                        continue;
                    },
                    ValidateResult::Restart => {
                        i = 0;
                        skip = false;
                        continue;
                    }
                }
            }

            // ask for confirmation
            if inputs.confirm {
                if Self::confirm(&inputs, stream) {
                    // XXX
                }
            } else if !inputs.no_55 {
                show_sysmsg(&SysMsg::new(SysMsgCode::Processing), stream);
            }

            // send "input_data" to "inputs.target"
            if let Some(val) = input_data.get(INPUT_NAME_NAVIGATION) {
                return InputEvent::Navigation(val.to_owned())
            } else {
                // fill defaults with current state, so in case we get called again,
                // the user can keep editing
                let mut vec = vec!();
                for (i, field) in inputs.fields.iter().enumerate() {
                    vec.push((i, Some(input_data.get(&field.name).unwrap().clone())));
                }
                for (i, v) in vec {
                    inputs.fields[i].default = v;
                }
                return InputEvent::TextFields(input_data);
            }

        }
    }

    fn confirm(inputs: &Inputs, stream: &mut (impl Write + Read)) -> bool { // "send?" message
        let price = inputs.price;
        if price.is_some() && price != Some(0) {
            show_sysmsg(&SysMsg::Code(SysMsgCode::ConfirmSendPay, price), stream);
        } else {
            show_sysmsg(&SysMsg::Code(SysMsgCode::ConfirmSend, None), stream);
        };
        let mut cept = Cept::new();
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
            println!("command: back {:?}", self.history);
            if self.history.len() < 2 {
                println!("ERROR: No history.");
                UserRequest::Error(SysMsg::new(SysMsgCode::CantGoBack))
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
                UserRequest::Goto(target_pageid, true)
            }
        } else if command_input == "09" {
            // hard reload
            println!("command: hard reload");
            // invalidate palette and include
            self.client_state.cept_palette = None;
            self.client_state.cept_include = None;
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
            UserRequest::Error(SysMsg::new(SysMsgCode::PageDoesNotExist))
        }
    }
}