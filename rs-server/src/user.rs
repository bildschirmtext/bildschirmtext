use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::cept::*;
use super::stat::*;

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

//XXX global_user = None

#[derive(Serialize, Deserialize)]
struct StatsData {
    last_use: Option<i64>,
}

struct Stats {
    filename: String,
    stats_data: StatsData,
}

fn filename(user_id: &str, ext: &str, path: &str, file_extension: &str) -> String {
    let mut s = String::new();
    s += path;
    s += user_id;
    s.push('-');
    s += ext;
    s.push('.');
    s += file_extension;
    s
}

impl Stats {
	pub fn new(user: &User) -> Self {
		let filename = filename(&user.user_id, &user.ext, PATH_STATS, &".stats");
        let f = File::open(&filename).unwrap();
        let stats_data: StatsData = serde_json::from_reader(f).unwrap();
        Stats {
            filename,
            stats_data,
        }
    }

	pub fn update(&mut self) {
		// update the last use field with the current time
		self.stats_data.last_use = Some(Utc::now().timestamp());
        let json_data = serde_json::to_string(&self.stats_data).unwrap();
        let mut file = File::create(self.filename).unwrap();
        file.write_all(&json_data.as_bytes());
    }
}

impl User {
	fn sanitize(user_id: &str, ext: &str) -> (String, String) {
        let mut user_id = user_id.to_owned();
        let mut ext = ext.to_owned();
        if user_id == "" {
            user_id = "0".to_owned();
        }
		if ext == "" {
            ext = "1".to_owned();
        }
        (user_id, ext)
    }

    fn user_filename(user_id: &str, ext: &str) -> String {
        filename(user_id, ext, PATH_USERS, "user")
    }

	fn secrets_filename(user_id: &str, ext: &str) -> String {
        filename(user_id, ext, PATH_SECRETS, "secrets")
    }

    fn exists(user_id: &str, ext: Option<&str>) -> bool {
        let ext = ext.unwrap_or(&"");
		let (user_id, ext) = Self::sanitize(&user_id, &ext);
		let filename = Self::user_filename(&user_id, &ext);
        is_file(&filename)
    }

	fn get(user_id: &str, ext: &str, personal_data: bool) -> Option<User> {
		let (user_id, ext) = Self::sanitize(&user_id, &ext);
		let filename = Self::user_filename(&user_id, &ext);
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
		if User::exists(user_id, Some(ext)) {
			println!("user already exists!");
            return false;
        }
		let user = User {
            user_id: user_id.to_owned(),
            ext: ext.to_owned(),
			salutation: Some(salutation.to_owned()),
			first_name: Some(first_name.to_owned()),
			last_name: Some(last_name.to_owned()),
			street: Some(street.to_owned()),
			zip: Some(zip.to_owned()),
			city: Some(city.to_owned()),
            country: Some(country.to_owned()),

            personal_data: false,
            org_name: None,
            org_add_name: None,
		};
        let json_data = serde_json::to_string(&user).unwrap();
        let mut file = File::create(user_filename).unwrap();
        file.write_all(&json_data.as_bytes());

		let secrets = Secrets {
			password: password.to_owned(),
		};
        let json_data = serde_json::to_string(&secrets).unwrap();
        let mut file = File::create(secrets_filename).unwrap();
        file.write_all(&json_data.as_bytes());

        true
    }

	fn login(user_id: &str, ext: &str, password: &str, force: bool) -> bool {
		let (user_id, ext) = Self::sanitize(&user_id, &ext);
		let filename = Self::secrets_filename(&user_id, &ext);
        if let Ok(f) = File::open(&filename) {
            let secrets: Result<Secrets, Error> = serde_json::from_reader(f);
            if let Ok(secrets) = secrets {
                password == secrets.password || force
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn line() -> Cept {
    let cept = Cept::new();
    cept.set_left_g3();
    cept.set_fg_color(15);
    cept.repeat(b'Q', 40);
    cept.set_fg_color(7);
    cept.set_left_g0();
    cept
}

fn create_title(title: &str) -> Cept {
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
    cept.add_str(title);
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept
}

fn create_title2(title: &str) -> Cept {
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
    cept.add_str(title);
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept
}

fn create_add_user() {
    let meta = Meta {
        publisher_name: "!BTX",
        include: "a",
        clear_screen: true,
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
                    cursor_home: true,
                    overwrite: true
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
                    cursor_home: true,
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
                    cursor_home: true,
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
                    cursor_home: true,
                    overwrite: true
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
                    cursor_home: true,
                    overwrite: true
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
                    cursor_home: true,
                    overwrite: true
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
                    cursor_home: true,
                    overwrite: true
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
    cept.add_str("Teilnehmernummer:");
    cept.set_cursor(6, 29);
    cept.add_str("-1");
    cept.add_raw(b"\r\n");
    cept.add_str("Anrede:");
    cept.add_raw(b"\r\n");
    cept.add_str("Name:");
    cept.add_raw(b"\r\n");
    cept.add_str("Vorname:");
    cept.add_raw(b"\r\n");
    cept.add_str("Straße:");
    cept.add_raw(b"\r\n");
    cept.add_str("PLZ:");
    cept.repeat(" ", 7);
    cept.add_str("Ort:");
    cept.set_cursor(11, 31);
    cept.add_str("Land:");
    cept.add_raw(b"\r\n");
    cept.add_raw(line());
    cept.add_str("Vergütungssperre aktiv:");
    cept.add_raw(b"\r\n");
    cept.add_str("Gebührensperre   aktiv:");
    cept.add_raw(b"\r\n");
    cept.add_str("Taschengeldkonto      :");
    cept.set_cursor(15, 35);
    cept.add_str(",   DM");
    cept.add_str("Max. Vergütung/Seite  :");
    cept.set_cursor(16, 35);
    cept.add_str(",   DM");
    cept.add_raw(line());
    cept.add_raw(b"\r\n");
    cept.add_str("Kennwort: ");
    cept.add_raw(b"\r\n\r\n");
    cept.add_raw(line());
    return (meta, cept)
}

// fn callback_validate_user_id(input_data, dummy) {
//     if User::exists(input_data["user_id"]):
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
//     if User::create(
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