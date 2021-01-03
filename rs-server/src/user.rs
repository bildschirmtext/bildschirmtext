use serde::{Deserialize, Serialize};

const PATH_USERS: &str = "../users/";
const PATH_SECRETS: &str = "../secrets/";
const PATH_STATS: &str = "../stats/";

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
	pub ext: String,
	pub personal_data: bool,

	// public - person
	pub salutation: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	// public - organization
	pub org_name: Option<String>,
	pub org_add_name: Option<String>,
	// personal_data
	pub street: Option<String>,
	pub zip: Option<String>,
	pub city: Option<String>,
	pub country: Option<String>,

	// stats: None
	// messaging: None
}

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    password: String,
}

impl User {
    pub fn get(user_id: &str, ext: &str, personal_data: bool) -> Option<User> {
        Some(User {
            user_id: "0".to_owned(),
            ext: "0".to_owned(),
            personal_data: false,

            salutation: None,
            first_name: None,
            last_name: None,
            org_name: None,
            org_add_name: None,

            street: None,
            zip: None,
            city: None,
            country: None,
        })
    }
}

//XXX global_user = None

#[derive(Serialize, Deserialize)]
struct StatsData {
    last_login: Some(timestamp),
}

struct Stats {
    filename: String,
    stats_data: StatsData,
}

fn filename(userid: &str, ext: &str, file_extension: &str) -> String {
    let mut s = String::new();
    s += PATH_MESSAGES;
    s += user_id;
    s.push('-');
    s += ext;
    s.push('.');
    s += file_extension;
    s
}

impl Stats {
	pub fn new(user: &User) {
		let filename = filename(user.user_id, user.ext, &".stats");
        let f = File::open(&filename).unwrap();
        let stats_data: StatsData = serde_json::from_reader(f).unwrap();
        Stats {
            filename,
            stats_data,
        }
    }

	pub fn update(self) {
		// update the last use field with the current time
		let stats = Stats { last_use: Utc::now().timestamp() };
        let json_data = serde_json::to_string(stats).unwrap();
        let mut file = File::create(self.filename).unwrap();
        file.write_all(&json_data.as_bytes());
    }
}

impl User {
	// fn sanitize(user_id: &str, ext: &str) {
	// 	if user_id == "" {
    //         user_id = "0"
    //     }
	// 	if ext is None or ext == "":
	// 		ext = "1"
    //     return (user_id, ext)
    // }

    fn user_filename(user_id: &str, ext: &str) {
        filename(user_id, ext, "user");
    }

	fn secrets_filename(user_id: &str, ext: &str) {
        filename(user_id, ext, "secrets");
    }

    fn exists(user_id: &str, ext: Option<&str>) {
        let ext = ext.unwrap_or(&"1");
		let (user_id, ext) = Self::sanitize(user_id, ext);
		let filename = User.user_filename(user_id, ext);
        return is_file(filename);
    }

	fn get(user_id: &str, ext: &str, personal_data: bool) -> User {
		(user_id, ext) = Self::sanitize(user_id, ext);
		filename = User.user_filename(user_id, ext);
        let f = File::open(&filename).ok()?;
        let user: User = serde_json::from_reader(f).ok()?;
		// user.messaging = Messaging(user)
        Some(user)
    }

	fn create(
        user_id: &str,
        ext: &str,
        password: &str,
        salutation: &str,
        last_name: &str,
        first_name: &str,
        street: &str,
        zip: &str,
        city: &str,
        country: &str
    ) -> bool {
		let user_filename = Self::user_filename(user_id, ext);
		let secrets_filename = Self::secrets_filename(user_id, ext);
		// if the user exists, don't overwrite it!
		if exists(user_id, ext) {
			println!("user already exists!");
            return false;
        }
		let user = User {
			salutation,
			first_name,
			last_name,
			street,
			zip,
			city,
			country
		};
        let json_data = serde_json::to_string(user).unwrap();
        let mut file = File::create(user_filename).unwrap();
        file.write_all(&json_data.as_bytes());

		let secrets = Secrets {
			password
		};
        let json_data = serde_json::to_string(user).unwrap();
        let mut file = File::create(secrets_filename).unwrap();
        file.write_all(&json_data.as_bytes());

        true;
    }

	fn login(user_id: &str, ext: &str, password: &str, force: bool) {
		let (user_id, ext) = cls.sanitize(user_id, ext);
		let filename = Self::secrets_filename(user_id, ext);
        let f = File::open(&filename).ok()?;
        let secrets: Secrets = serde_json::from_reader(f).ok()?;
        password == secrets.password || force
    }
}

fn line() -> Cept {
    let cept = Cept::new();
    cept.set_left_g3();
    cept.set_fg_color(15);
    cept.repeat('Q', 40);
    cept.set_fg_color(7);
    cept.set_left_g0();
    cept
}

fn create_title(title: &str) {
    let cept = Cept::new();
    cept.set_cursor(2, 1);
    cept.set_palette(1);
    cept.set_screen_bg_color_simple(4);
    cept.add_raw(&[
        0x1b, 0x28, 0x40,           // load G0 into G0
        0x0f,                   // G0 into left charset
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
    cept.from_str(title);
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept
}

fn create_title2(title: &str) {
    let cept = Cept::new();
    cept.set_cursor(2, 1);
    cept.set_palette(1);
    cept.set_screen_bg_color_simple(4);
    cept.add_raw(&[
        0x1b, 0x28, 0x40,           // load G0 into G0
        0x0f,                   // G0 into left charset
    ]);
    cept.parallel_mode();
    cept.set_palette(0);
    cept.code_9e();
    cept.set_line_bg_color_simple(4);
    cept.add_raw(b"\n");
    cept.set_line_bg_color_simple(4);
    cept.set_palette(1);
    cept.double_height();
    cept.add_raw(b"\r");
    cept.from_str(title);
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept
}

fn create_add_user() {
    meta = Meta {
        publisher_name: "!BTX",
        include: "a",
        clear_screen: True,
        links: Some(vec!(
            Links::new("0", "0"),
            Links::new("1", "88"),
            Links::new("2", "89"),
            Links::new("5", "810"),
        )),
        inputs: Inputs {
            fields: [
                InputField {
                    name: "user_id",
                    hint: "Gewünschte Nummer oder # eingeben",
                    line: 6,
                    column: 19,
                    height: 1,
                    width: 10,
                    bgcolor: 12,
                    fgcolor: 3,
                    typ: "number",
                    validate: "call:User_UI.callback_validate_user_id"
                },
                InputField {
                    name: "salutation",
                    hint: "Anrede oder # eingeben",
                    line: 7,
                    column: 9,
                    height: 1,
                    width: 20,
                    bgcolor: 12,
                    fgcolor: 3
                },
                InputField {
                    name: "last_name",
                    hint: "Nachnamen oder # eingeben",
                    line: 8,
                    column: 7,
                    height: 1,
                    width: 20,
                    bgcolor: 12,
                    validate: "call:User_UI.callback_validate_last_name",
                    fgcolor: 3
                },
                InputField {
                    name: "first_name",
                    hint: "Vornamen oder # eingeben",
                    line: 9,
                    column: 10,
                    height: 1,
                    width: 20,
                    bgcolor: 12,
                    fgcolor: 3
                },
                InputField {
                    name: "street",
                    hint: "Straße und Hausnummer oder # eingeben",
                    line: 10,
                    column: 9,
                    height: 1,
                    width: 20,
                    bgcolor: 12,
                    fgcolor: 3
                },
                InputField {
                    name: "zip",
                    hint: "Postleitzahl oder # eingeben",
                    line: 11,
                    column: 6,
                    height: 1,
                    width: 5,
                    bgcolor: 12,
                    fgcolor: 3,
                    typ: "number"
                },
                InputField {
                    name: "city",
                    hint: "Ort oder # eingeben",
                    line: 11,
                    column: 17,
                    height: 1,
                    width: 13,
                    bgcolor: 12,
                    fgcolor: 3
                },
                InputField {
                    name: "country",
                    hint: "Land oder # eingeben",
                    line: 11,
                    column: 37,
                    height: 1,
                    width: 2,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "de",
                    typ: "alpha",
                    cursor_home: True,
                    overwrite: True
                },
                InputField {
                    name: "block_payments",
                    hint: "j/n oder # eingeben",
                    line: 13,
                    column: 25,
                    height: 1,
                    width: 1,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "n",
                    cursor_home: True,
                    legal_values: [ "j", "n" ]
                },
                InputField {
                    name: "block_fees",
                    hint: "j/n oder # eingeben",
                    line: 14,
                    column: 25,
                    height: 1,
                    width: 1,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "n",
                    cursor_home: True,
                    legal_values: [ "j", "n" ]
                },
                InputField {
                    name: "pocket_money_major",
                    hint: "0-9 oder # eingeben",
                    line: 15,
                    column: 34,
                    height: 1,
                    width: 1,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "9",
                    typ: "number",
                    cursor_home: True,
                    overwrite: True
                },
                InputField {
                    name: "pocket_money_minor",
                    hint: "00-99 oder # eingeben",
                    line: 15,
                    column: 36,
                    height: 1,
                    width: 2,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "99",
                    typ: "number",
                    cursor_home: True,
                    overwrite: True
                },
                InputField {
                    name: "max_price_major",
                    hint: "0-9 oder # eingeben",
                    line: 16,
                    column: 34,
                    height: 1,
                    width: 1,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "9",
                    typ: "number",
                    cursor_home: True,
                    overwrite: True
                },
                InputField {
                    name: "max_price_minor",
                    hint: "00-99 oder # eingeben",
                    line: 16,
                    column: 36,
                    height: 1,
                    width: 2,
                    bgcolor: 12,
                    fgcolor: 3,
                    default: "99",
                    typ: "number",
                    cursor_home: True,
                    overwrite: True
                },
                InputField {
                    name: "password",
                    hint: "Neues Kennwort",
                    line: 19,
                    column: 11,
                    height: 1,
                    width: 14,
                    bgcolor: 12,
                    fgcolor: 3,
                    typ: "password",
                    validate: "call:User_UI.callback_validate_password",
                },
            ],
            confirm: false,
            target: "call:User_UI.callback_add_user",
        },
        publisher_color: 7
    };

    let cept = Cept::new();
    cept.add_raw(create_title("Neuen Benutzer einrichten"));
    cept.add_raw(b"\r\n");
    cept.from_str("Teilnehmernummer:");
    cept.set_cursor(6, 29);
    cept.from_str("-1");
    cept.add_raw(b"\r\n");
    cept.from_str("Anrede:");
    cept.add_raw(b"\r\n");
    cept.from_str("Name:");
    cept.add_raw(b"\r\n");
    cept.from_str("Vorname:");
    cept.add_raw(b"\r\n");
    cept.from_str("Straße:");
    cept.add_raw(b"\r\n");
    cept.from_str("PLZ:");
    cept.repeat(" ", 7);
    cept.from_str("Ort:");
    cept.set_cursor(11, 31);
    cept.from_str("Land:");
    cept.add_raw(b"\r\n");
    cept.add_raw(line());
    cept.from_str("Vergütungssperre aktiv:");
    cept.add_raw(b"\r\n");
    cept.from_str("Gebührensperre   aktiv:");
    cept.add_raw(b"\r\n");
    cept.from_str("Taschengeldkonto      :");
    cept.set_cursor(15, 35);
    cept.from_str(",   DM");
    cept.from_str("Max. Vergütung/Seite  :");
    cept.set_cursor(16, 35);
    cept.from_str(",   DM");
    cept.add_raw(line());
    cept.add_raw(b"\r\n");
    cept.from_str("Kennwort: ");
    cept.add_raw(b"\r\n\r\n");
    cept.add_raw(line());
    return (meta, cept)
}

// fn callback_validate_user_id(input_data, dummy) {
//     if User.exists(input_data["user_id"]):
//         msg = Util.create_custom_system_message("Teilnehmernummer bereits vergeben! -> #")
//         sys.stdout.buffer.write(msg)
//         sys.stdout.flush()
//         Util.wait_for_ter()
//         return Util.VALIDATE_INPUT_BAD
//     else:
//         return Util.VALIDATE_INPUT_OK
// }

// fn callback_validate_last_name(input_data, dummy) {
//     if not input_data["last_name"]:
//         msg = Util.create_custom_system_message("Name darf nicht leer sein! -> #")
//         sys.stdout.buffer.write(msg)
//         sys.stdout.flush()
//         Util.wait_for_ter()
//         return Util.VALIDATE_INPUT_BAD
//     else:
//         return Util.VALIDATE_INPUT_OK
// }

// fn callback_validate_password(input_data, dummy) {
//     if len(input_data["password"]) < 4:
//         msg = Util.create_custom_system_message("Kennwort muß mind. 4-stellig sein! -> #")
//         sys.stdout.buffer.write(msg)
//         sys.stdout.flush()
//         Util.wait_for_ter()
//         return Util.VALIDATE_INPUT_BAD
//     else:
//         return Util.VALIDATE_INPUT_OK
// }

// fn callback_add_user(input_data: Vec<(String, String)>) {
//     println!("input_data: {}", input_data);
//     if User.create(
//         input_data["user_id"],
//         "1", // ext
//         input_data["password"],
//         input_data["salutation"],
//         input_data["last_name"],
//         input_data["first_name"],
//         input_data["street"],
//         input_data["zip"],
//         input_data["city"],
//         input_data["country"]
//     ):
//         msg = Util.create_custom_system_message("Benutzer angelegt. Bitte neu anmelden. -> #")
//         sys.stdout.buffer.write(msg)
//         sys.stdout.flush()
//         Util.wait_for_ter()
//         return "00000"
//     else:
//         msg = Util.create_custom_system_message("Benutzer konnte nicht angelegt werden. -> #")
//         sys.stdout.buffer.write(msg)
//         sys.stdout.flush()
//         Util.wait_for_ter()
//         return "77"
// }

fn create(user: &User, pageid: &str) -> Option<Page> {
    if pageid == "77a" {
        Some(create_add_user())
    } else {
        None
    }
}