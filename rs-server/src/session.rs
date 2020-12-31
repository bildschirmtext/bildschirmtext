use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::fs::File;
use super::cept::*;
use super::editor::*;
use super::historic::*;
use super::stat::*;
use super::pages::*;

pub struct Session {

}

impl Session {
    pub fn new() -> Self {
        Self { }
    }

    pub fn run(&mut self, stream: &mut (impl Write + Read))
    {
        let mut desired_pageid = "78a".to_string(); // login page
        let compress = false;

        let mut current_pageid = "".to_string();
        let autoplay = false;
        let mut history: Vec<String> = vec!();
        let mut error = 0;

        let showing_message = false;

        // let mut last_filename_palette = "";
        // let mut last_filename_include = "";

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
                    // last_filename_palette = "";
                    // last_filename_include = "";
                }
                if desired_pageid == "00" { // re-send CEPT data of current page
                    println!("resend");
                    error = 0;
                    add_to_history = false;
                } else if desired_pageid != "" {
                    println!("showing page: {}", desired_pageid);
                    let (cept1, cept2, l, i, autoplay) = create_page(&desired_pageid);
                    links = l;
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
                    for link in &links.clone().unwrap() {
                        legal_values.push(link.code.clone());
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

                Self::handle_inputs(&inputs.unwrap(), stream)
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
                for link in links.unwrap() {
                    if val_or_hash == link.code {
                        // link
                        desired_pageid = link.target;
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
}
