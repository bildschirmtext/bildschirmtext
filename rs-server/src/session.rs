use chrono::Utc;
use std::io::{Read, Write};
use std::fs::File;
use std::collections::HashMap;
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


pub enum Validate {
    Ok,
	Error,
	Restart,
}

pub struct Session {
    user: Option<User>,
    last_filename_palette: Option<String>,
    last_filename_include: Option<String>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            user: None,
            last_filename_palette: None,
            last_filename_include: None,
         }
    }

    pub fn run(&mut self, stream: &mut (impl Write + Read))
    {
        let mut desired_pageid = "00000".to_string();
        let compress = false;

        let mut current_pageid = "".to_string();
        let mut autoplay = false;
        let mut history: Vec<String> = vec!();
        let mut error = 0;

        let mut showing_message = false;

        self.last_filename_palette = None;
        self.last_filename_include = None;

        let mut links = None;

        loop {
            let mut inputs = None;

            // if User.user() is not None:
            // 	User.user().stats.update()

            if desired_pageid.len() > 0 && desired_pageid.chars().last().unwrap().is_ascii_digit() {
                desired_pageid += "a";
            }

            let mut add_to_history = true;
            if error == 0 {
                add_to_history = true;

                // *# back
                if desired_pageid == "" {
                    if history.len() < 2 {
                        println!("ERROR: No history.");
                        error = 10
                    } else {
                        let _ = history.pop();
                        desired_pageid = history.pop().unwrap();
                        // if we're navigating back across page numbers...
                        if desired_pageid.chars().last().unwrap() != current_pageid.chars().last().unwrap() {
                            // if previous page was sub-page, keep going back until "a"
                            while desired_pageid.chars().last().unwrap() != 'a' {
                                desired_pageid = history.pop().unwrap();
                            }
                        }
                    }
                }
                if desired_pageid == "09" { // hard reload
                    println!("hard reload");
                    desired_pageid = history.last().unwrap().to_string();
                    add_to_history = false;
                    // force load palette and include
                    self.last_filename_palette = None;
                    self.last_filename_include = None;
                }
                if desired_pageid == "00" { // re-send CEPT data of current page
                    println!("resend");
                    error = 0;
                    add_to_history = false;
                } else if desired_pageid != "" {
                    println!("showing page: {}", desired_pageid);
                    let page = self.get_page(&desired_pageid);
                    self.show_page(stream, &page, &desired_pageid);
                    links = page.meta.links;
                    inputs = page.meta.inputs;
                    autoplay = page.meta.autoplay == Some(true);
                    // except:
                    //     error=10


                    // # user interrupted palette/charset, so the decoder state is undefined
                    self.last_filename_palette = None;
                    self.last_filename_include = None;


                    error = 0

                    // if success else 100
                } else {
                    error = 100
                }
            }

            if error == 0 {
                current_pageid = desired_pageid;
                if add_to_history {
                    history.push(current_pageid.clone());
                };
            } else {
                if desired_pageid != "" {
                    println!("ERROR: Page not found: {}", desired_pageid);
                    if desired_pageid.chars().last().unwrap() >= 'b' && desired_pageid.chars().last().unwrap() <= 'z' {
                        error = 101;
                    }
                }
                let mut cept = create_system_message(error, None);
                cept.sequence_end_of_page();
            	write_stream(stream, cept.data());
            	showing_message = true;
            }

            desired_pageid = "".to_string();

            let input_data = if autoplay {
                println!("autoplay!");
                hashmap!["$navigation".to_owned() => "".to_owned()]
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
                    inputs = Some(Inputs {
                        fields: vec!(
                            InputField {
                                name: "$navigation".to_string(),
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
                                no_navigation: false,
                                validate: false,
                                default: None,
                            }),
                        confirm: false,
                        no_55: true,
                        target: None,
                        no_navigation: false,
                    });
                }

                Self::handle_inputs(&current_pageid, &mut inputs.unwrap(), stream)
            };
            println!("input_data: {:?}", input_data);

            error = 0;
            if let Some(d) = input_data.get("$command") {
                desired_pageid = d.clone();
            } else {
                let val = input_data.get("$navigation").unwrap();
                let val_or_hash = if val.len() != 0 { val.clone() } else { "#".to_owned() };
                let mut found = false;
                if let Some(links) = &links {
                    for link in links {
                        if val_or_hash == link.code {
                            // link
                            desired_pageid = link.target.clone();
                            // decode = decode_call(desired_pageid, None)
                            // if decode {
                            //     desired_pageid = decode
                            // }
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    if val.len() == 0 {
                        // next sub-page
                        let last_char = current_pageid.chars().last().unwrap();
                        if last_char.is_ascii_digit() {
                            desired_pageid = current_pageid.clone() + "b"
                        } else if last_char >= 'a' && last_char <= 'y' {
                            let mut s = current_pageid.to_owned();
                            s.pop();
                            s.push((last_char as u8 + 1) as char);
                        } else {
                            error = 101;
                            desired_pageid = "".to_owned();
                        }
                    } else {
                        error = 100;
                        desired_pageid = "".to_owned();
                    }
                }
            }

        }
    }

    fn handle_inputs(pageid: &str, inputs: &mut Inputs, stream: &mut (impl Write + Read)) -> HashMap<String, String> {
        // create editors and draw backgrounds
        let mut editors = vec!();
        for input_field in &mut inputs.fields {
            input_field.no_navigation = inputs.no_navigation;
            let editor = Editor::new(input_field);
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
                    return hashmap!["$command".to_string() => val[1..].to_string()];
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
                hashmap!["$command".to_owned() => target[5..].to_owned()]
            } else {
                // XXX we should loop
                let handle_result = Self::handle(pageid, &input_data);
                hashmap!["$command".to_owned() => handle_result]
            }
        } else {
            input_data
        }

    }

    pub fn get_page(&self, pageid: &str) -> Page {
        if pageid.starts_with("00000") || pageid == "9a" {
            super::login::create(pageid, self.user.as_ref()).unwrap()
        } else if pageid == "77a" {
            super::user::create(pageid).unwrap()
        } else if pageid.starts_with('7') {
            super::historic::create(&pageid[1..])
        } else {
            super::stat::create(pageid).unwrap()
        }
    }

    pub fn validate(pageid: &str, input_data: &HashMap<String, String>) -> Validate {
        if pageid.starts_with("00000") || pageid == "9a" {
            super::login::validate(pageid, input_data)
        } else {
            Validate::Ok
        }
    }

    pub fn handle(pageid: &str, input_data: &HashMap<String, String>) -> String {
        panic!();
    }

    pub fn show_page(&mut self, stream: &mut (impl Write + Read), page: &Page, pageid: &str) -> bool {
        let cept1 = self.cept_preamble_from_meta(&page, pageid);
        let cept2 = self.cept_main_from_page(&page, pageid);

        // if compress {
        //     page_cept_data_1 = Cept.compress(page_cept_data_1)
        //     page_cept_data_2 = Cept.compress(page_cept_data_2)
        // }

        println!("Sending pal/char");
        write_stream(stream, cept1.data());
        println!("Sending text");
        write_stream(stream, cept2.data());

        false
    }

    //
    fn cept_preamble_from_meta(&mut self, page: &Page, pageid: &str) -> Cept {
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

    fn cept_main_from_page(&mut self, page: &Page, pageid: &str) -> Cept {
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


pub fn headerfooter(cept: &mut Cept, pageid: &str, publisher_name: Option<&str>, publisher_color: u8) {
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
        cept.add_str(&format!("{pageid:>width$}", pageid=pageid, width=22));
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
