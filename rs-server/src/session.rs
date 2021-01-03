use chrono::Utc;
use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;
use std::str::FromStr;
use super::cept::*;
use super::editor::*;
use super::stat::*;
use super::pages::*;
use super::user::*;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

const INPUT_NAME_NAVIGATION: &'static str = "$navigation";
const INPUT_NAME_COMMAND: &'static str = "$command";

enum InputData {
    Command(String),
    Navigation(String),
    TextFields(HashMap<String, String>),
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

    fn kill_leading(&self, n: usize) -> Self {
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

pub enum Validate {
    Ok,
	Error,
	Restart,
}

pub enum CommandType {
    Goto(PageId, bool),
    SendAgain,
    Error(usize)
}

pub struct Session {
    user: Option<User>,
    last_filename_palette: Option<String>,
    last_filename_include: Option<String>,
    current_pageid: PageId,
    history: Vec<PageId>,
    autoplay: bool,
}

impl Session {
    pub fn new() -> Self {
        Self {
            user: None,
            last_filename_palette: None,
            last_filename_include: None,
            current_pageid: PageId::empty(),
            history: vec!(),
            autoplay: false,
        }
    }

    // Interpret a global command code ("*nnn#").
    // This could be
    // * an explicit page numer
    // * "*00#" to re-send the current page CEPT data (e.g. after a transmission error)
    // * "*09#" to reload the current page (may fetch a newer version of the page)
    // * "*#" to go back
    fn interpret_command(&mut self, command_input: &str) -> CommandType {
        if command_input == "" {
            // *# = back
            println!("command: back");
            if self.history.len() < 2 {
                println!("ERROR: No history.");
                CommandType::Error(10)
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
                CommandType::Goto(target_pageid, false)
            }
        } else if command_input == "09" {
            // hard reload
            println!("command: hard reload");
            // force load palette and include
            self.last_filename_palette = None;
            self.last_filename_include = None;
            CommandType::Goto(self.current_pageid.clone(), false)
        } else if command_input == "00" {
            // re-send CEPT data of current page
            println!("command: resend");
            CommandType::SendAgain
        } else {
            CommandType::Goto(PageId::from_str(command_input).unwrap(), true)
        }
    }

    // Handle page interactivity:
    // * for pages with text fields, draw them and allow editing them
    // * for pages with without text fields, allow entering a link
    // In both cases, it is possible to escape into command mode.
    fn get_inputs(&self, inputs: Option<&Inputs>, links: Option<&Vec<Link>>, stream: &mut (impl Write + Read)) -> InputData {
        if self.autoplay {
            println!("autoplay!");
            // inject "#"
            InputData::Navigation("".to_owned())
        } else {
            if inputs.is_none() {
                let mut legal_values = vec!();
                if let Some(links) = links.clone() {
                    for link in links {
                        if link.code != "#" {
                            legal_values.push(link.code.clone());
                        }
                    }
                }
                let mut inputs = Inputs {
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
                    target: None,
                    no_navigation: false,
                };
                Self::handle_text_fields(&self.current_pageid, &inputs, stream)
            } else {
                let mut inputs = inputs.unwrap();
                Self::handle_text_fields(&self.current_pageid, &inputs, stream)
            }
        }
    }

    pub fn run(&mut self, stream: &mut (impl Write + Read))
    {
        let mut target_pageid = PageId::from_str("00000").unwrap();
        let mut add_to_history = false;

        let compress = false;


        self.last_filename_palette = None;
        self.last_filename_include = None;

        let mut links = None;

        let mut inputs = None;

        let mut current_page_cept = Cept::new();

        'main: loop {
            // if User.user() is not None:
            // 	User.user().stats.update()

            // *** show page
            println!("showing page: {}", target_pageid.to_string());
            if let Some(page) = self.get_page(&target_pageid) {
                current_page_cept = self.construct_page_cept(&page, &target_pageid);
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
                Self::show_error(error, stream);
            }


            'input: loop {
                // *** get user input
                let input_data = self.get_inputs(inputs.as_ref(), links.as_ref(), stream);
                // println!("input_data: {:?}", input_data);

                // *** handle input
                match input_data {
                    InputData::Command(command_input) => {
                        match self.interpret_command(&command_input) {
                            CommandType::Goto(t, a) => {
                                target_pageid = t;
                                add_to_history = a;
                                continue 'main;
                            },
                            CommandType::SendAgain => {
                                write_stream(stream, current_page_cept.data());
                            },
                            CommandType::Error(e) => {
                                Self::show_error(e, stream);
                                continue 'input;
                            }
                        }
                    },
                    InputData::Navigation(val) => {
                        if let Some(links) = &links {
                            for link in links {
                                if (*val == link.code) || (val == "" && link.code == "#") {
                                    target_pageid = PageId::from_str(&link.target).unwrap();
                                    continue 'main;
                                }
                            }
                        }
                        // not found
                        if val.len() == 0 {
                            // next sub-page
                            self.current_pageid.sub += 1;
                            continue 'main;
                        } else {
                            println!("ERROR: Illegal navigation");
                            Self::show_error(100, stream);
                            continue 'input;
                        }
                    }
                    _ => {
                        // XXX TODO
                    }
                }
            }


        }
    }

    fn show_error(error: usize, stream: &mut (impl Write + Read)) {
        let mut cept = create_system_message(error, None);
        cept.sequence_end_of_page();
        write_stream(stream, cept.data());
    }

    fn handle_text_fields(pageid: &PageId, inputs: &Inputs, stream: &mut (impl Write + Read)) -> InputData {
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
                    return InputData::Command(val[1..].to_string());
                }
            }

            input_data.insert(input_field.name.to_string(), val.unwrap().to_string());


            let mut validate_result = Validate::Ok;
            if input_field.validate {
                validate_result = Self::validate(pageid, &input_data);
            }

            match validate_result {
                Validate::Ok => {
                    i += 1;
                },
                Validate::Error => {
                    skip = false;
                    continue;
                },
                Validate::Restart => {
                    i = 0;
                    skip = false;
                    continue;
                }
            }
        }

        // confirmation
        // if inputs.confirm {
        // 	if confirm(inputs) {
        // 		if inputs.action == "send_message" {
        // 			User.user().messaging.send(input_data["user_id"], input_data["ext"], input_data["body"])
        // 			system_message_sent_message()
        //         } else {
        //             pass // TODO we stay on the page, in the navigator?
        //         }
        //     }
        // } else if !inputs.no_55 {
        // 	cept_data = Util.create_system_message(55)
        // 	sys.stdout.buffer.write(cept_data)
        //     sys.stdout.flush()
        // }

        // send "input_data" to "inputs.target"
        if let Some(target) = &inputs.target {
        	if target.starts_with("page:") {
                return InputData::Command(target[5..].to_owned());
            } else {
                // XXX we should loop
                let handle_result = Self::handle(pageid, &input_data);
                return InputData::Command(handle_result);
            }
        } else if let Some(val) = input_data.get(INPUT_NAME_NAVIGATION) {
            return InputData::Navigation(val.to_owned())
        } else {
            return InputData::TextFields(input_data);
        }

    }

    pub fn get_page(&self, pageid: &PageId) -> Option<Page> {
        if pageid.page.starts_with("00000") || pageid.page == "9" {
            super::login::create(pageid, self.user.as_ref())
        } else if pageid.page == "77" {
            super::user::create(pageid)
        } else if pageid.page.starts_with('7') {
            Some(super::historic::create(&pageid.kill_leading(1)))
        } else {
            super::stat::create(pageid)
        }
    }

    pub fn validate(pageid: &PageId, input_data: &HashMap<String, String>) -> Validate {
        if pageid.page.starts_with("00000") || pageid.page == "9" {
            super::login::validate(pageid, input_data)
        } else {
            Validate::Ok
        }
    }

    pub fn handle(pageid: &PageId, input_data: &HashMap<String, String>) -> String {
        panic!();
    }

    pub fn construct_page_cept(&mut self, page: &Page, pageid: &PageId) -> Cept {
        let mut cept = self.cept_preamble_from_meta(&page, pageid);
        cept += self.cept_main_from_page(&page, pageid);

        // if compress {
        //     page_cept_data_1 = Cept.compress(page_cept_data_1)
        // }

        cept
    }

    //
    fn cept_preamble_from_meta(&mut self, page: &Page, pageid: &PageId) -> Cept {
        let mut cept = Cept::new();

        cept.hide_cursor();

        if page.meta.clear_screen == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            self.last_filename_include = None;
        }

        let basedir = find_basedir(pageid).unwrap().0;

        // define palette
        if let Some(palette) = &page.meta.palette {
            let mut filename_palette = basedir.to_owned();
            filename_palette += &palette;
            filename_palette += ".pal";
            println!("filename_palette = {}", filename_palette);
            // println!("last_filename_palette = {}", last_filename_palette);
            if Some(filename_palette.clone()) != self.last_filename_palette {
                self.last_filename_palette = Some(filename_palette.clone());
                let f = File::open(&filename_palette).unwrap();
                let palette: Palette = serde_json::from_reader(f).unwrap();
                cept.define_palette(&palette.palette, palette.start_color);
            } else {
                println!("skipping palette");
            }
        } else {
            self.last_filename_palette = None;
        }

        if let Some(include) = &page.meta.include {
            let mut filename_include = basedir.to_owned();
            filename_include += &include;
            filename_include += ".inc";
            // if is_file(filename_include) {
            // 	filename_include_cm = basedir + meta["include"] + ".inc.cm"
            // 	filename_include = basedir + meta["include"] + ".inc"
            // } else {
            // 	filename_include_cm =""
            //     filename_include = basedir + meta["include"] + ".cept"
            // }
            println!("Filename_include = {}", filename_include);

            if Some(filename_include.clone()) != self.last_filename_include || page.meta.clear_screen == Some(true) {
                self.last_filename_include = Some(filename_include.clone());
                // if os.path.isfile(filename_include) {
                    let mut cept_include : Vec<u8> = vec!();
                    let mut f = File::open(&filename_include).unwrap();
                    f.read_to_end(&mut cept_include);
                    println!("loading: {}", filename_include);
                // } else if os.path.isfile(filename_include_cm) {
                // 	data_include = CM.read(filename_include_cm)
                // } else {
                //     sys.stderr.write("include file not found.\n")
                // }
                // palette definition has to end with 0x1f; add one if
                // the include data doesn't start with one
                if cept_include[0] != 0x1f {
                    cept.set_cursor(1, 1)
                }
                cept.add_raw(&cept_include);
            // }
            } else {
                self.last_filename_include = None;
            }

        // b = baud if baud else 1200
        // if len(cept) > (b/9) * SH291_THRESHOLD_SEC {
            // cept = Util.create_system_message(291) + cept
        // }
        }
        cept
    }

    fn cept_main_from_page(&mut self, page: &Page, pageid: &PageId) -> Cept {
        let mut cept = Cept::new();

        if page.meta.cls2 == Some(true) {
            cept.serial_limited_mode();
            cept.clear_screen();
            self.last_filename_include = None;
        }

        headerfooter(&mut cept, pageid, page.meta.publisher_name.as_deref(), page.meta.publisher_color.unwrap());

        if page.meta.parallel_mode == Some(true) {
            cept.parallel_mode();
        }

        cept.add_raw(page.cept.data());

        cept.serial_limited_mode();

        // XXX ???
        headerfooter(&mut cept, pageid, page.meta.publisher_name.as_deref(), page.meta.publisher_color.unwrap());

        cept.sequence_end_of_page();

        cept
    }
}


pub fn headerfooter(cept: &mut Cept, pageid: &PageId, publisher_name: Option<&str>, publisher_color: u8) {
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
            if p.len() > 30 {
                Some(&p[..30])
            } else {
                Some(p)
            }
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
        cept.add_str(&format!("{pageid:>width$}", pageid=pageid.to_string(), width=22));
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


fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}

fn create_system_message(code: usize, price: Option<f32>) -> Cept {
    let mut text = String::new();
    let mut prefix = "SH";
    if code == 0 {
        text = "                               ".to_owned();
    } else if code == 10 {
        text = "Rückblättern nicht möglich     ".to_owned();
    } else if code == 44 {
        text = "Absenden? Ja:19 Nein:2         ".to_owned();
    } else if code == 47 {
        text = format!("Absenden für {}? Ja:19 Nein:2", format_currency(price.unwrap()));
    } else if code == 55 {
        text = "Eingabe wird bearbeitet        ".to_owned();
    } else if code == 73 {
        let current_datetime = Utc::now().format("%d.%m.%Y %H:%M").to_string();
        text = format!("Abgesandt {}, -> #  ", current_datetime);
        prefix = "1B";
    } else if code == 100 || code == 101 {
        text = "Seite nicht vorhanden          ".to_owned();
    } else if code == 291 {
        text = "Seite wird aufgebaut           ".to_owned();
    }

    let mut msg = Cept::new();
    msg.service_break(24);
    msg.clear_line();
    msg.add_str_characterset(&text, Some(1));
    msg.hide_text();
    msg.add_raw(b"\x08");
    msg.add_str(prefix);
    msg.add_str(&format!("{:03}", code));
    msg.service_break_back();
    msg
}
