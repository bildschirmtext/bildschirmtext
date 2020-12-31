use std::io::{Read, Write};
use super::cept::*;
use super::editor::*;
use super::historic::*;
use super::stat::*;

pub fn interactive_mode(stream: &mut (impl Write + Read))
{
    let mut desired_pageid = "78a".to_string(); // login page
    let compress = false;

    let mut current_pageid = "".to_string();
    let autoplay = false;
    let mut history: Vec<String> = vec!();
    let mut error = 0;

    let showing_message = false;

    let mut last_filename_palette = "";
    let mut last_filename_include = "";

    loop {
        let mut inputs = None;

        // if User.user() is not None:
    	// 	User.user().stats.update()

        if desired_pageid.len() > 0 && desired_pageid.chars().last().unwrap().is_ascii_digit() {
            desired_pageid += "a";
        }

        let mut links = None;

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
                last_filename_palette = "";
                last_filename_include = "";
            }
            if desired_pageid == "00" { // re-send CEPT data of current page
                println!("resend");
                error = 0;
                add_to_history = false;
            } else if desired_pageid != "" {
                println!("showing page: {}", desired_pageid);
                let (cept1, cept2, l, i, autoplay) = create_page(&desired_pageid);
                links = Some(l);
                inputs = i;
                // except:
                //     error=10

                // if (compress):
                //     page_cept_data_1 = Cept.compress(page_cept_data_1)
                //     page_cept_data_2 = Cept.compress(page_cept_data_2)

                println!("Sending pal/char");
                stream.write_all(cept1.data()).unwrap();
                println!("Sending text");
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

        let input_data = if autoplay {
            println!("autoplay!");
            vec!(( "$navigation".to_owned(), "".to_owned() ))
        } else {
            if inputs.is_none() {
                let mut legal_values = vec!();
                for (value, _) in &links.clone().unwrap() {
                    legal_values.push(value.clone());
                }
                // legal_values = list(links.keys())
                // if "#" in legal_values:
                //     legal_values.remove("#")
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
                            typ: InputType::Normal,
                            cursor_home: false,
                            clear_line: false,
                            legal_values: Some(legal_values),
                            end_on_illegal_character: true,
                            end_on_legal_string: true,
                            echo_ter: true,
                            command_mode: false,
                            no_navigation: false,
                            default: None,
                        }),
                    confirm: false,
                    no_55: true,
                });
            }

            handle_inputs(&inputs.unwrap(), stream)
        };
        println!("input_data: {:?}", input_data);

        error = 0;
        if input_data[0].0 == "$command" {
            desired_pageid = input_data[0].1.clone();
        } else {
            assert_eq!(input_data[0].0, "$navigation");
            let val = input_data[0].1.clone();
            let val_or_hash = if val.len() != 0 { val.clone() } else { "#".to_owned() };
            let mut found = false;
            for (key, target) in links.unwrap() {
                if val_or_hash == key {
                    // link
                    desired_pageid = target;
                    // decode = decode_call(desired_pageid, None)
                    // if decode {
                    //     desired_pageid = decode
                    // }
                    found = true;
                    break;
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


pub fn create_page(pageid: &str) -> (Cept, Cept, Vec<(String, String)>, Option<Inputs>, bool) {
    let page = match pageid.chars().next().unwrap() {
        '7' => super::historic::create(&pageid[1..]),
        _ => super::stat::create(pageid),
    };

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
    let links = page.meta.links;
    let inputs = page.meta.inputs;
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

pub struct Meta {
    pub publisher_name: Option<String>,
    pub clear_screen: bool,
    pub cls2: bool,
    pub parallel_mode: bool,
    pub links: Vec<(String, String)>,
    pub publisher_color: u8,
    pub inputs: Option<Inputs>,
}

pub struct Page {
    pub cept: Cept,
    pub meta: Meta,
}

impl Page {
    pub fn new(meta: Meta) -> Self {
        Self {
            cept: Cept::new(),
            meta: meta,
        }
    }
}

fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}

