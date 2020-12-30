use std::io::{Read, Write};
use super::cept::*;
use super::editor::*;



pub fn interactive_mode(stream: &mut (impl Write + Read))
{
    let mut desired_pageid = "78a".to_string(); // login page
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

        let mut links = None;

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
                let (cept1, cept2, l, inputs, autoplay) = create_page(&desired_pageid);
                links = Some(l);
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

        let input_data = if autoplay {
            println!("autoplay!");
            vec!(( "$navigation".to_owned(), "".to_owned() ))
        } else {
            if inputs.fields.is_empty() {
                let mut legal_values = vec!();
                for (value, _) in &links.clone().unwrap() {
                    legal_values.push(value.clone());
                }
                // legal_values = list(links.keys())
                // if "#" in legal_values:
                //     legal_values.remove("#")
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
                };
            }

            handle_inputs(&inputs, stream)
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
                if val == key {
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


pub fn create_page(pageid: &str) -> (Cept, Cept, Vec<(String, String)>, &'static str, bool) {
    let page = if pageid == "78a" {
        Page::create_historic_main_page()
    } else if pageid == "711a" || pageid == "712a" {
        create_historic_overview(pageid[1..3].parse().unwrap(), 0).unwrap()
    } else {
        panic!();
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

    fn historic_line(&mut self, page: (&str, &str), index: i32) {
        let link = historic_pretty_link_from_str(page.0);
        let mut s = page.1.to_string();
        s += " ";
        s += &link;
        while s.len() < 38 {
            s.push('.');
        }
        self.cept.add_str(&s);
        self.cept.add_str(&index.to_string());
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


fn create_historic_overview(collection: i32, index: i32) -> Option<Page> {
    let name;
    let description;
    let distribution;
    let start_page;
    let pages;

    if collection == 10 {
        name = "Amiga Demo";
        description =
            "Der Amiga BTX Software-Decoder wurde mit\
            Dumps von 113 BTX-Seiten aus 32\n\
            Programmen ausgeliefert, sowie 56 eigens\
            gestalteten Seiten zum Thema BTX.\n\
            Die Seiten stammen vom April 1989.";
        distribution = vec!(9, 17);

        start_page = Some(("20096/1", "Amiga Demo Startseite"));

        pages = vec!(
            ("1050", "Btx-Telex"),
            ("1188", "Teleauskunft"),
            ("1692", "Cityruf"),
            ("20000", "Deutsche Bundespost"),
            ("20096", "Commodore"),
            ("20511/223", "Kölner Stadtanzeiger"),
            ("21212", "Verbraucher-Zentrale NRW"),
            ("25800/0000", "Deutsche Bundesbahn"),
            ("30003", "Formel Eins"),
            ("30711", "Btx Südwest Datenbank GmbH"),
            ("33033", "Eden"),
            ("34034", "Frankfurter Allg. Zeitung"),
            ("34344", "Neue Mediengesellschaft Ulm"),
            ("35853", "ABIDA GmbH"),
            ("40040/200", "Axel Springer Verlag"),
            ("44479", "DIMDI"),
            ("50257", "Computerwelt Btx-Info-Dienst"),
            ("54004/04", "ÖVA Versicherungen"),
            ("57575", "Lotto Toto"),
            ("64064", "Markt & Technik"),
            ("65432/0", "ADAC"),
            ("67007", "Rheinpfalz Verlag/Druckerei"),
            ("201474/75", "Rhein-Neckar-Zeitung"),
//			("208585", "eba Pressebüro und Verlag [BROKEN]"),
            ("208888", "Neue Mediengesellschaft Ulm"),
            ("402060", "AUTO & BTX WOLFSBURG"),
            ("50707545", "CHIP Magazin"),
            ("86553222", "Chaos Computer Club"),
            ("505050035", "Steinfels Sprachreisen"),
            ("920492040092", "Wolfgang Fritsch (BHP)"),
        );
    } else if collection == 11 {
        name = "PC online 1&1";
        description =
            "Der PC online 1&1 Decoder wurde mit\n\
            von 25 BTX-Seiten aus 15 Programmen\n\
            ausgeliefert. Die Seiten stammen vom\n\
            November 1993.";
        distribution = vec!(12);

        start_page = None;

        pages = vec!(
            ("00000/88", "Teleauskunft"),
            ("00000/1188", "Mitteilungsdienst"),
            ("20111/1", "Vobis Microcomputer AG"),
            ("20111/11020", "- Übersicht 486"),
            ("20111/1102030", "- 486 DX-50 "),
            ("20111/110203010", "- 486 DX-50 Details"),
            ("21199", "Microsoft"),
            ("21199/1362", "- Produkte"),
            ("25800", "Deutsche Bundesbahn"),
            ("28000/101", "Postbank"),
            ("34561/10", "1&1 Telekommunkation"),
            ("34561/99", "- Forum [a-b]"),
            ("37107/2154", "WDR Computer-Club"),
            ("46801/8149999999", "Handelsblatt"),
            ("49498/0004902", "bhv Computerbücher"),
            ("49498/000490201", "- Neuheiten"),
            ("50000", "Deutsche Lufthansa"),
            ("52800", "IBM Deutschland"),
            ("52800/03", "- IBM Personal Systeme"),
            ("52800/31", "- HelpClubShop [a-c]"),
            ("58587/003", " ITZ Schulungen"),
            ("69010", "Deutscher Ind. Handelstag"),
            ("353535/00", "START Tourismus"),
            ("353535/01240", "- Veranstalter"),
            ("353535/01640", "- Reiseinformationen"),
        );
    } else {
        return None;
    }

    let mut start_with = 0;
    if index != 0 {
        for i in 0..index as usize {
            if i >= distribution.len() {
                return None;
            }
            start_with += distribution[i];
        }
    }


    let mut links = vec!(
        ("0".to_owned(), "78".to_owned()),
    );
    if let Some(start_page) = start_page {
        links.push(("10".to_owned(), historic_link_from_str(start_page.0)));
    }
    let mut i = 20;
    for page in &pages {
        links.push((i.to_string(), historic_link_from_str(page.0)));
        i += 1
    }

    let meta = Meta {
        publisher_name: Some("!BTX".to_owned()),
        clear_screen: true,
        cls2: false,
        parallel_mode: false,
        links: links,
        publisher_color: 7,
    };

    let mut page = Page::new(meta);

    // sys.stderr.write("meta: " + pprint.pformat(meta) + "\n")

    let mut cept = Cept::new();
    let mut t = "Historische Seiten: ".to_owned();
    t += name;
    page.create_title(&t);
    cept.add_str("\r\n");

    if index == 0 {
        cept.add_str(description);
        cept.add_str("\r\n\n");
        if let Some(start_page) = start_page {
            page.historic_line(start_page, 10);
            cept.add_str("\n")
        }
    }

    let end = if index as usize >= distribution.len() {
        pages.len()
    } else {
        start_with + distribution[index as usize]
    };
    for i in start_with..end {
        page.historic_line(pages[i], i as i32 + 20);
    }

    let right = if (index as usize) < distribution.len() { Some("Weiter #") } else { None };
    page.footer("0 Zurück", right);
    // cept.compress();

    Some(page)
}

fn historic_link_from_str(s: &str) -> String {
    s.replace("/", "")
}

fn historic_pretty_link_from_str(s: &str) -> String {
    let split: Vec<&str> = s.split("/").collect();
    let s = if split[0] == "00000" {
        split[1]
    } else {
        split[0]
    };
    let mut res = "(*".to_owned();
    res += s;
    res += "#)";
    res
}

fn format_currency(price: f32) -> String {
    format!("DM  {},{:02}", (price / 100.0).floor(), (price % 100.0).floor())
}

