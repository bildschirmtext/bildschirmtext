use chrono::Utc;

const PATH_MESSAGES: &str = "../messages/";

struct MessageDict {
    messages: Vec<Message>,
}

impl MessageDict {
    fn new() -> Self {
        Self {
            messages: vec!()
        }
    }
}

struct Message {
    body: String,
    from_user_id: String,
    from_ext: String,
    personal_data: bool,
    timestamp: usize,
    is_read: bool,
}

struct XMessage {
	dict: MessageDict,
	from_user: String,
    index: usize,
}

impl XMessage {
	fn new(dict: &MessageDict, index: usize) -> Option<Self> {
		let from_user = User.get(dict.from_user_id, dict.from_ext, dict.personal_data, false);
		if let Some(from_user) = from_user {
            Some(Self {
                dict: dict,
                from_user: String,
                index: index,
            })
        } else {
            println!("from user not found!");
            None
        }
    }

	fn from_date(&self) -> String {
        let t = Utc::timestamp(self.dict.timestamp);
        t.format("%d.%m.%Y").to_string()
    }

	fn from_time(&self) {
        let t = Utc::timestamp(self.dict.timestamp);
        t.format("%H:%M").to_string()
    }

	fn body(&mut self) {
        self.dict.body
    }

struct Messaging {
	user: String,
    dict: MessageDict,
}


impl Messaging {
	fn new(user: &User) -> Self {
        Self {
            user: u,
            dict: MessageDict::new(),
        }
    }

	fn dict_filename(user_id: &str, ext: &str) -> String {
        let mut s = String::new;
        s += PATH_MESSAGES;
        s += user_id;
        s.push('-');
        s += ext;
        s += ".messages";
        s
    }

	fn load_dict(user_id: &str, ext: &str) {
		let filename = Messaging::dict_filename(user_id, ext);
		if !is_file(filename) {
			println!("messages file not found");
			MessageDict::new()
        } else {
            let f = File::open(&filename).unwrap();
            serde_json::from_reader(f).unwrap()
        }
    }

	fn save_dict(user_id: &str, ext: &str, dict: &MessageDict) {
        let json_data = serde_json::to_string(dict).unwrap();
        let mut file = File::create(dict_filename(user_id, ext)).unwrap();
        file.write_all(json_data);
    }

	fn load(&mut self) {
        self.dict = Messaging::load_dict(self.user.user_id, self.user.ext);
    }

	fn save(&mut self) {
        Messaging::save_dict(self.user.user_id, self.user.ext, self.dict);
    }

	fn select(&mut self, is_read: bool, start: usize, count: Option<usize>) -> Vec<Message> {
		self.load();

		let mut ms = vec!();
		let mut j = 0;
		for i in (0..self.dict.messages.len()).rev() {
			let m = self.dict.messages[i];
			if m.is_read != is_read {
                continue;
            }
            if j < start {
                continue;
            }
            if let Some(count) = count {
                if j >= start + count {
                    continue;
                }
            }
            ms.push(Message(m, i));
            j += 1;
        }

        return ms;
    }

	fn mark_as_read(&mut self, index: usize) {
		self.load();
		if !self.dict.messages[index].is_read {
			self.dict.messages[index].is_read = true;
            self.save();
        }
    }

	fn has_new_messages(&mut self) {
		self.load();
        self.select(false, 0, None).len() != 0;
    }

	fn send(&mut self, user_id: &str, ext: &str, body: &str) {
		let dict = Messaging::load_dict(user_id, ext);
		dict.messages.push(
            Message {
				from_user_id: self.user.user_id,
				from_ext: self.user.ext,
				personal_data: False,
				timestamp: time.time(),
                body: body,
                is_read: false,
			},
		);
        Messaging::save_dict(user_id, ext, dict);
    }
}

// ************
// UI
// ************

// private
fn messaging_create_title(title: &str) -> Cept {
    let cept = Cept::new();
    cept.set_cursor(2, 1);
    cept.set_palette(1);
    cept.set_screen_bg_color_simple(4);
    cept.add_raw([
        0x1b, 0x28, 0x40,           // load G0 into G0
        0x0f                   // G0 into left charset
    ]);
    cept.parallel_mode();
    cept.set_palette(0);
    cept.code_9e();
    cept.add_raw(b"\n\r");
    cept.set_line_bg_color_simple(4);
    cept.add_raw(b"\n");
    cept.set_line_bg_color_simple(4);
    cept.set_palette(1);
    cept.double_height();
    cept.add_raw(b"\r");
    cept.add_str(title);
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept
}

// private
fn messaging_create_menu(title: &str, items: &[&str]) -> Cept {
    let cept = messaging_create_title(title);
    cept.add_raw(b"\n\r\n\r");
    let mut i = 1;
    for item in items {
        let s = String::new();
        cept.add_str(i.to_string());
        cept.add_str("  ");
        cept.add_str(item);
        cept.add_raw(b"\r\n\r\n");
        i += 1;
    }

    cept.add_raw(b"\r\n\r\n\r\n\r\n\r\n\r\n");
    cept.set_line_bg_color_simple(4);
    cept.add_raw(b"0\x19\x2b");
    cept.add_str(" Gesamtübersicht");

    cept
}

fn messaging_create_main_menu() -> (Meta, Cept) {
    let meta = Meta {
        publisher_name: Some("!BTX"),
        include: Some("a"),
        clear_screen: True,
        links: Some(vec!(
            Link("0", "0"),
            Link("1", "88"),
            Link("2", "89"),
            Link("5", "810"),
        )),
        publisher_color: Some(7),
    }

    let cept = messaging_create_menu(
        "Mitteilungsdienst",
        &[
            "Neue Mitteilungen",
            "Zurückgelegte Mitteilungen",
            "Abruf Antwortseiten",
            "Ändern Mitteilungsempfang",
            "Mitteilungen mit Alphatastatur"
        ]
    );
    (meta, cept)
}

fn messaging_create_list(user: &User, is_read: bool) -> (Meta, Cept) {
    if is_read {
        title = "Zurückgelegte Mitteilungen"
    } else {
        title = "Neue Mitteilungen"
    }
    let cept = Messaging_UI.messaging_create_title(title);

    let mut links = vec!(
        Link("0", "8"),
    );

    let target_prefix = if is_read {"89" } else { "88" };

    let messages = user.messaging.select(is_read, 0, 9);

    for index in 0..9 {
        cept.add_str((index + 1).to_string());
        cept.add_str("  ");
        if index < messages.len() {
            message = messages[index];
            if message.from_user.org_name {
                cept.add_str(message.from_user.org_name);
            } else {
                cept.add_str(message.from_user.first_name);
                cept.add_raw(b" ");
                cept.add_str(message.from_user.last_name);
                cept.add_raw(b"\r\n   ");
            }
            cept.add_str(message.from_date());
            cept.add_raw(b"   ");
            cept.add_str(message.from_time());
            cept.add_raw(b"\r\n");
            links[str(index + 1)] = target_prefix + (index + 1).to_string();
        } else {
            cept.add_raw(b"\r\n\r\n");
        }

    let meta = Meta {
        publisher_name: Some("!BTX"),
        include: Some("a"),
        clear_screen: true,
        links: links,
        publisher_color: Some(7_),
    };
    (meta, cept)
}

fn messaging_create_message_detail(user: &User, index: usize, is_read: bool) -> (Meta, Cept) {
    messages = user.messaging.select(is_read, index, 1);
    if messages.len() == 0 {
        return None;
    }

    message = messages[0];

    meta = Meta {
        publisher_name: Some("Bildschirmtext"),
        include: Some("11a"),
        palette: Some("11a"),
        clear_screen: True,
        links: Some(vec(
            Link("0", if is_read { "89" } else { "88"}),
        )),
        publisher_color: Some(7),
    };

    let from_date = message.from_date();
    let from_time = message.from_time();
    let from_street;
    let from_zip;
    let from_city;
    if message.from_user.personal_data {
        from_street = message.from_user.street;
        from_zip = message.from_user.zip;
        from_city = message.from_user.city;
    } else {
        from_street = "";
        from_zip = "";
        from_city = "";
    }

    cept = bytearray(Cept.parallel_limited_mode();
    cept.set_cursor(2, 1);
    cept.set_fg_color(3);
    cept.add_str(b"von ");
    cept.add_str(message.from_user.user_id.ljust(12));
    cept.add_str(" ");
    cept.(message.from_user.ext.rjust(5, '0'));
    cept.set_cursor(2, 41 - from_date.len());
    cept.add_str(from_date);
    cept.repeat(" ", 4);
    cept.add_str(message.from_user.org_name);
    cept.set_cursor(3, 41 - from_time.len());
    cept.add_str(from_time);
    cept.repeat(" ", 4);
    cept.set_fg_color_simple(0);
    cept.add_str(message.from_user.first_name);
    cept.add_str(" ");
    cept.add_str(message.from_user.last_name);
    cept.add_raw(b"\r\n");
    cept.repeat(" ", 4);
    cept.add_str(add_street);
    cept.add_raw(b"\r\n");
    cept.repeat(" ", 4);
    cept.add_str(from_zip);
    cept.add_raw(b' ');
    cept.add_str(from_city);
    cept.add_raw(b"\r\n");
    cept.add_str(b"an  ");
    cept.add_str(user.user_id.ljust(12));
    cept.add_str(" ");
    cept.add_str(user.ext.rjust(5, '0'));
    cept.add_raw(b"\r\n");
    cept.repeat(" ", 4);
    cept.add_str(user.first_name);
    cept.add_str(" ");
    cept.add_str(user.last_name);
    cept.add_raw(b"\r\n\n");
    cept.add_str(message.body());
    cept.set_cursor(23, 1);
    cept.add_raw(b'0');
    cept.add_raw(&[
        0x1b, 0x29, 0x20, 0x40,                                    // load DRCs into G1
        0x1b, 0x7e                                            // G1 into right charset
    ]);
    cept.add_str(" Gesamtübersicht");
    cept.repeat(" ", 22);

    user.messaging.mark_as_read(message.index);

    (meta, cept)
}

fn callback_validate_user_id(cls, input_data, dummy):
    if User.exists(input_data["user_id"]):
        return Util.VALIDATE_INPUT_OK
    else:
        msg = Util.create_custom_system_message("Teilnehmerkennung ungültig! -> #")
        sys.stdout.buffer.write(msg)
        sys.stdout.flush()
        Util.wait_for_ter()
        return Util.VALIDATE_INPUT_BAD

fn callback_validate_ext(cls, input_data, dummy):
    if User.exists(input_data["user_id"], input_data["ext"]):
        return Util.VALIDATE_INPUT_OK
    else:
        msg = Util.create_custom_system_message("Mitbenutzernummer ungültig! -> #")
        sys.stdout.buffer.write(msg)
        sys.stdout.flush()
        Util.wait_for_ter()
        return Util.VALIDATE_INPUT_RESTART

fn messaging_create_compose(user):
    meta = {
        "include": "a",
        "clear_screen": True,
        "links": {
            "0": "8"
        },
        "publisher_color": 7,
        "inputs": {
            "fields": [
                {
                    "name": "user_id",
                    "type": "user_id",
                    "line": 8,
                    "column": 20,
                    "height": 1,
                    "width": 16,
                    "bgcolor": 4,
                    "fgcolor": 3,
                    "validate": "call:Messaging_UI.callback_validate_user_id"
                },
                {
                    "name": "ext",
                    "type": "ext",
                    "line": 8,
                    "column": 37,
                    "height": 1,
                    "width": 1,
                    "bgcolor": 4,
                    "fgcolor": 3,
                    "default": "1",
                    "validate": "call:Messaging_UI.callback_validate_ext"
                },
                {
                    "name": "body",
                    "line": 12,
                    "column": 1,
                    "height": 10,
                    "width": 40,
                    "bgcolor": 4,
                    "fgcolor": 3
                }
            ],
            "action": "send_message",
            "price": 30,
            "target": "page:8"
        }
    }

    current_date = datetime.datetime.now().strftime("%d.%m.%Y")
    current_time = datetime.datetime.now().strftime("%H:%M")

    cept = bytearray(Cept.set_cursor(2, 1))
    cept.set_palette(1))
    cept.set_screen_bg_color_simple(4))
    cept.add_raw(
        b'\x1b\x28\x40'                                    // load G0 into G0
    )
    cept.add_raw(
        b'\x0f'                                            // G0 into left charset
    )
    cept.parallel_mode())
    cept.set_palette(0))
    cept.code_9e())
    cept.add_raw(b"\n\r")
    cept.set_line_bg_color_simple(4))
    cept.add_raw(b"\n")
    cept.set_line_bg_color_simple(4))
    cept.set_palette(1))
    cept.double_height())
    cept.add_raw(b"\r")
    cept.add_str("Mitteilungsdienst"))
    cept.add_raw(b"\n\r")
    cept.set_palette(0))
    cept.normal_size())
    cept.code_9e())
    cept.set_fg_color_simple(7))
    cept.add_str("Absender:"))
    cept.add_str(user.user_id))
    cept.set_cursor(5, 25))
    cept.add_str(user.ext))
    cept.set_cursor(6, 10))
    cept.add_str(user.first_name))
    cept.set_cursor(7, 10))
    cept.add_str(user.last_name))
    cept.set_cursor(5, 31))
    cept.add_str(current_date))
    cept.set_cursor(6, 31))
    cept.add_str(current_time))
    cept.add_raw(b"\r\n\n");
    cept.add_str("Tln.-Nr. Empfänger:"))
    cept.set_cursor(8, 36))
    cept.add_raw(
        b'-'
        b'\r\n\n\n'
    )
    cept.add_raw(b'Text:')
    cept.add_raw(b'\r\n\n\n\n\n\n\n\n\n\n\n\n')
    cept.set_line_bg_color_simple(4))
    cept.add_raw(b'0')
    cept.add_raw(
        b'\x19'                                            // switch to G2 for one character
        b'\x2b\xfe\x7f'                                    // "+."
    )
    return (meta, cept)

fn create_page(user, pagenumber):
    if pagenumber == "8a":
        return Messaging_UI.messaging_create_main_menu()
    elif pagenumber == "88a":
        return Messaging_UI.messaging_create_list(user, False)
    elif pagenumber == "89a":
        return Messaging_UI.messaging_create_list(user, True)
    elif re.search("^88\da$", pagenumber):
        return Messaging_UI.messaging_create_message_detail(user, int(pagenumber[2:-1]) - 1, False)
    elif re.search("^89\da$", pagenumber):
        return Messaging_UI.messaging_create_message_detail(user, int(pagenumber[2:-1]) - 1, True)
    elif pagenumber == "810a":
        return Messaging_UI.messaging_create_compose(user)
    else:
        return None
