use std::io::{Read, Write};
use super::cept::*;
use super::editor::*;



pub fn interactive_mode(stream: &mut (impl Write + Read))
{
    let mut desired_pageid = "00000".to_string(); // login page
    let compress = false;

    let mut current_pageid = "".to_string();
    let autoplay = false;
    let mut inputs = Inputs::default(); // XXX no need to init
    let mut history: Vec<String> = vec!();
    let mut error = 0;

    let showing_message = false;

    let mut last_filename_palette = "";
    let mut last_filename_include = "";

    loop {
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
                    // sys.stderr.write("ERROR: No history.\n")
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
                // sys.stderr.write("hard reload\n")
                desired_pageid = history.last().unwrap().to_string();
                add_to_history = false;
                // force load palette and include
                last_filename_palette = "";
                last_filename_include = "";
            }
            if desired_pageid == "00" { // re-send CEPT data of current page
                // sys.stderr.write("resend\n")
                error = 0;
                add_to_history = false;
            } else if desired_pageid != "" {
                // sys.stderr.write("showing page: '" + desired_pageid + "'\n")
                let (cept1, cept2, links, inputs, autoplay) = create_page();
                // except:
                //     error=10

                // if (compress):
                //     page_cept_data_1 = Cept.compress(page_cept_data_1)
                //     page_cept_data_2 = Cept.compress(page_cept_data_2)

                // sys.stderr.write("Sending pal/char: ")
                stream.write_all(cept1.data()).unwrap();
                // sys.stderr.write("Sending text: ")
                stream.write_all(cept2.data()).unwrap();

                // # user interrupted palette/charset, so the decoder state is undefined
                // last_filename_palette = ""
                // last_filename_include = ""


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
        //     if desired_pageid:
    	// 		sys.stderr.write("ERROR: Page not found: " + desired_pageid + "\n")
		// 	if (desired_pageid[-1] >= "b" and desired_pageid[-1] <= "z"):
		// 		code = 101
    	// 	cept_data = Util.create_system_message(error) + Cept.sequence_end_of_page()
    	// 	sys.stdout.buffer.write(cept_data)
    	// 	sys.stdout.flush()
    	// 	showing_message = True
        }

        desired_pageid = "".to_string();

        if autoplay {
            // sys.stderr.write("autoplay!\n")
            // input_data = { "$navigation" : "" }
        } else {
            if inputs.fields.is_empty() {
                // legal_values = list(links.keys())
                // if "#" in legal_values:
                //     legal_values.remove("#")
                let legal_values = Some(vec!()); // XXX
                inputs = Inputs {
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
                            typ: InputType::Normal,
                            cursor_home: false,
                            clear_line: false,
                            legal_values: legal_values,
                            end_on_illegal_character: true,
                            end_on_legal_string: true,
                            echo_ter: true,
                            command_mode: false,
                            no_navigation: false,
                            default: None,
                        }),
                    confirm: false,
                    no_55: true,
                };
            }

            let input_data = handle_inputs(&inputs, stream);

            // sys.stderr.write("input_data: " + pprint.pformat(input_data) + "\n")
        }

        // error = 0;
        // desired_pageid = input_data.get("$command")

        // if desired_pageid is None:
        //     val = input_data["$navigation"]
        //     val_or_hash = val if val else "#"
        //     if val_or_hash in links:
        //         # link
        //         desired_pageid = links[val_or_hash]
        //         decode = decode_call(desired_pageid, None)
        //         if decode:
        //             desired_pageid = decode
        //     elif not val:
        //         # next sub-page
        //         if current_pageid[-1:].isdigit():
        //             desired_pageid = current_pageid + "b"
        //         elif current_pageid[-1:] >= "a" and current_pageid[-1:] <= "y":
        //             desired_pageid = current_pageid[:-1] + chr(ord(current_pageid[-1:]) + 1)
        //         else:
        //             error = 101
        //             desired_pageid = None
        //     else:
        //         error = 100
        //         desired_pageid = None


    }
}

fn handle_inputs(inputs: &Inputs, stream: &mut (impl Write + Read)) -> Vec<(String, String)> {
	// create editors and draw backgrounds
	let mut editors = vec!();
	for input_field in &inputs.fields {
		let editor = Editor::new(input_field);
        editor.draw(stream);
		editors.push(editor);
    }

	// get all inputs
	let mut input_data = vec!();
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
                return vec!(("$command".to_string(), val[1..].to_string()));
            }
        }

		input_data.push((input_field.name.to_string(), val.unwrap().to_string()));

		// ret = decode_call(input_field.validate), input_data);

		// if not ret or ret == Util.VALIDATE_INPUT_OK {
            i += 1;
        // }
		// if ret == Util.VALIDATE_INPUT_BAD {
			// skip = False
            // continue
        // } else if ret == Util.VALIDATE_INPUT_RESTART {
			// i = 0
			// skip = False
            // continue
        // }
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

	// send "input_data" to "inputs["target"]"

	// if "target" in inputs:
	// 	if inputs["target"].startswith("page:"):
	// 		return { "$command": inputs["target"][5:] }

	// 	ret = decode_call(inputs["target"], input_data)
	// 	if ret:
	// 		return { "$command": ret }
	// 	else:
	// 		return None // error
	// else:
		return input_data;
}


pub fn create_page() -> (Cept, Cept, &'static str, &'static str, bool) {
    let pageid = "123";

    let page = Page::create_historic_main_page();

    let mut cept1 = Cept::new();
    cept1.hide_cursor();

    if page.meta.clear_screen {
        cept1.serial_limited_mode();
        cept1.clear_screen();
        //last_filename_include = ""
    }

    // cept1.extend(create_preamble(basedir, meta))

    let mut cept2 = Cept::new();

    if page.meta.cls2 {
        cept2.serial_limited_mode();
        cept2.clear_screen();
        // last_filename_include = ""
    }

    headerfooter(&mut cept2, pageid, page.meta.publisher_name.as_deref(), page.meta.publisher_color);

    if page.meta.parallel_mode {
        cept2.parallel_mode();
    }

    cept2.add_raw(page.cept.data());

    cept2.serial_limited_mode();

    // cept_2.extend(hf) //???

    cept2.sequence_end_of_page();

    // XXX
    let links = "";
    let inputs = "";
    let autoplay = false;

    (cept1, cept2, links, inputs, autoplay)
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
            Some(&p[..30])
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

struct Meta {
    publisher_name: Option<String>,
    clear_screen: bool,
    cls2: bool,
    parallel_mode: bool,
    links: Vec<(String, String)>,
    publisher_color: u8,
}

struct Page {
    cept: Cept,
    meta: Meta,
}

impl Page {
    pub fn new(meta: Meta) -> Self {
        Self {
            cept: Cept::new(),
            meta: meta,
        }
    }

    fn create_title(&mut self, title: &str) {
        self.cept.set_cursor(2, 1);
        self.cept.set_palette(1);
        self.cept.set_screen_bg_color_simple(4);
        self.cept.add_raw(
            &[0x1b, 0x28, 0x40,       // load G0 into G0
             0x0f]                   // G0 into left charset
        );
        self.cept.parallel_mode();
        self.cept.set_palette(0);
        self.cept.code_9e();
        self.cept.set_line_bg_color_simple(4);
        self.cept.add_raw(b"\n");
        self.cept.set_line_bg_color_simple(4);
        self.cept.set_palette(1);
        self.cept.double_height();
        self.cept.add_raw(b"\r");
        self.cept.add_str(title);
        self.cept.add_raw(b"\n\r");
        self.cept.set_palette(0);
        self.cept.normal_size();
        self.cept.code_9e();
        self.cept.set_fg_color_simple(7);
    }

	fn footer(&mut self, left: &str, right: Option<&str>) {
		self.cept.set_cursor(23, 1);
		self.cept.set_palette(0);
		self.cept.set_line_bg_color_simple(4);
		self.cept.add_str(left);

		if let Some(right) = right {
            self.cept.set_cursor(23, 41 - right.len() as u8);
            self.cept.add_str(right);
        }
    }

	pub fn create_historic_main_page() -> Self {
        let meta = Meta {
            publisher_name: Some("!BTX".to_owned()),
            clear_screen: true,
            cls2: false,
            parallel_mode: false,
            links: vec![
        		("0".to_owned(), "0".to_owned()),
				("10".to_owned(), "710".to_owned()),
				("11".to_owned(), "711".to_owned()),
				("#".to_owned(), "711".to_owned()),
            ],
			publisher_color: 7,
		};

        let mut page = Page::new(meta);
		page.create_title("Historische Seiten");
		page.cept.add_raw(b"\r\n");
		page.cept.add_str(
			"Nur wenige hundert der mehreren hundert-\
			tausend BTX-Seiten sind überliefert.\n\
			Die meisten entstammen dem Demomodus von\
			Software-BTX-Decoderprogrammen.\n\
			\n\
			1988: C64 BTX Demo (Input 64 12/88)...--\
			1989: Amiga BTX Terminal..............10\
			1989: C64 BTX Demo (64'er 1/90).......--\
			1991: BTX-VTX Manager v1.2............--\
			1993: PC online 1&1...................11\
			1994: MacBTX 1&1......................--\
			1995: BTXTEST.........................--\
			1996: RUN_ME..........................--\
			\n\
			Da historische Seiten erst angepaßt wer-\
			den müssen, um nutzbar zu sein, sind\n\
			noch nicht alle Sammlungen verfügbar."
			//XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
		);

		page.footer("0 Zurück", None);
        page
    }
}


fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}

