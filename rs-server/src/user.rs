use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use super::cept::*;
use super::pages::*;
use super::stat::*;
use super::session::*;

const PATH_USERS: &str = "../users/";
const PATH_SECRETS: &str = "../secrets/";
const PATH_STATS: &str = "../stats/";

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
    pub ext: String,
}

impl UserId {
	pub fn new(id: &str, ext: &str) -> Self {
        let mut id = id.to_owned();
        let mut ext = ext.to_owned();
        if id == "" {
            id = "0".to_owned();
        }
		if ext == "" {
            ext = "1".to_owned();
        }
        Self { id, ext }
    }

    pub fn to_string(&self) -> String {
        let mut s = self.id.clone();
        s.push('-');
        s += &self.ext;
        s
    }
}

#[derive(Serialize, Deserialize)]
pub enum UserDataPublic {
    Person(UserDataPublicPerson),
    Organization(UserDataPublicOrganization),
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPublicPerson {
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPublicOrganization {
    pub name1: Option<String>,
    pub name2: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPrivate {
	pub street: Option<String>,
	pub zip: Option<String>,
	pub city: Option<String>,
	pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub userid: UserId,
	pub public: UserDataPublic,
	pub private: UserDataPrivate,

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

fn filename(userid: &UserId, path: &str, file_extension: &str) -> String {
    let mut s = String::new();
    s += path;
    s += &userid.to_string();
    s.push('.');
    s += file_extension;
    s
}

impl Stats {
	pub fn new(user: &User) -> Self {
		let filename = filename(&user.userid, PATH_STATS, &".stats");
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
        let mut file = File::create(&self.filename).unwrap();
        file.write_all(&json_data.as_bytes());
    }
}

impl User {
    fn user_filename(userid: &UserId) -> String {
        filename(userid, PATH_USERS, "user")
    }

	fn secrets_filename(userid: &UserId) -> String {
        filename(userid, PATH_SECRETS, "secrets")
    }

    fn exists(userid: &UserId) -> bool {
		let filename = Self::user_filename(&userid);
        is_file(&filename)
    }

	fn get(userid: &UserId, personal_data: bool) -> Option<User> {
		let filename = Self::user_filename(&userid);
        let f = File::open(&filename).ok()?;
        let user: User = serde_json::from_reader(f).unwrap();
		// user.messaging = Messaging(user)
        Some(user)
    }

	fn create(
        id: &str,
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
        let userid = UserId::new(id, ext);
		let user_filename = Self::user_filename(&userid);
		let secrets_filename = Self::secrets_filename(&userid);
		// if the user exists, don't overwrite it!
		if User::exists(&userid) {
			println!("user already exists!");
            return false;
        }
		let user = User {
            userid: userid,
            public: UserDataPublic::Person(UserDataPublicPerson {
                salutation: Some(salutation.to_owned()),
                first_name: Some(first_name.to_owned()),
                last_name: Some(last_name.to_owned()),
            }),
            private: UserDataPrivate {
                street: Some(street.to_owned()),
                zip: Some(zip.to_owned()),
                city: Some(city.to_owned()),
                country: Some(country.to_owned()),
            },
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

	pub fn login(userid: &UserId, password: &str) -> Option<Self> {
		let filename = Self::secrets_filename(userid);
        if let Ok(f) = File::open(&filename) {
            let secrets: Result<Secrets, _> = serde_json::from_reader(f);
            if let Ok(secrets) = secrets {
                if password == secrets.password {
                    return Self::get(&userid, true);
                }
            }
        }
        None
    }
}

fn line() -> Cept {
    let mut cept = Cept::new();
    cept.set_left_g3();
    cept.set_fg_color(15);
    cept.repeat(b'Q', 40);
    cept.set_fg_color(7);
    cept.set_left_g0();
    cept
}

fn create_title(title: &str) -> Cept {
    let mut cept = Cept::new();
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
    let mut cept = Cept::new();
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

fn create_add_user() -> Page {
    let meta_str = r#"
    {
        "clear_screen": true,
        "include": "a",
        "inputs": {
            "confirm": false,
            "fields": [
                {
                    "bgcolor": 12,
                    "column": 19,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Gew\u00fcnschte Nummer oder # eingeben",
                    "line": 6,
                    "name": "user_id",
                    "type": "Numeric",
                    "validate": true,
                    "width": 10
                },
                {
                    "bgcolor": 12,
                    "column": 9,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Anrede oder # eingeben",
                    "line": 7,
                    "name": "salutation",
                    "width": 20
                },
                {
                    "bgcolor": 12,
                    "column": 7,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Nachnamen oder # eingeben",
                    "line": 8,
                    "name": "last_name",
                    "validate": true,
                    "width": 20
                },
                {
                    "bgcolor": 12,
                    "column": 10,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Vornamen oder # eingeben",
                    "line": 9,
                    "name": "first_name",
                    "width": 20
                },
                {
                    "bgcolor": 12,
                    "column": 9,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Stra\u00dfe und Hausnummer oder # eingeben",
                    "line": 10,
                    "name": "street",
                    "width": 20
                },
                {
                    "bgcolor": 12,
                    "column": 6,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Postleitzahl oder # eingeben",
                    "line": 11,
                    "name": "zip",
                    "type": "Numeric",
                    "width": 5
                },
                {
                    "bgcolor": 12,
                    "column": 17,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Ort oder # eingeben",
                    "line": 11,
                    "name": "city",
                    "width": 13
                },
                {
                    "bgcolor": 12,
                    "column": 37,
                    "cursor_home": true,
                    "default": "de",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Land oder # eingeben",
                    "line": 11,
                    "name": "country",
                    "overwrite": true,
                    "type": "alpha",
                    "width": 2
                },
                {
                    "bgcolor": 12,
                    "column": 25,
                    "cursor_home": true,
                    "default": "n",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "j/n oder # eingeben",
                    "legal_values": [
                        "j",
                        "n"
                    ],
                    "line": 13,
                    "name": "block_payments",
                    "width": 1
                },
                {
                    "bgcolor": 12,
                    "column": 25,
                    "cursor_home": true,
                    "default": "n",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "j/n oder # eingeben",
                    "legal_values": [
                        "j",
                        "n"
                    ],
                    "line": 14,
                    "name": "block_fees",
                    "width": 1
                },
                {
                    "bgcolor": 12,
                    "column": 34,
                    "cursor_home": true,
                    "default": "9",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "0-9 oder # eingeben",
                    "line": 15,
                    "name": "pocket_money_major",
                    "overwrite": true,
                    "type": "Numeric",
                    "width": 1
                },
                {
                    "bgcolor": 12,
                    "column": 36,
                    "cursor_home": true,
                    "default": "99",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "00-99 oder # eingeben",
                    "line": 15,
                    "name": "pocket_money_minor",
                    "overwrite": true,
                    "type": "Numeric",
                    "width": 2
                },
                {
                    "bgcolor": 12,
                    "column": 34,
                    "cursor_home": true,
                    "default": "9",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "0-9 oder # eingeben",
                    "line": 16,
                    "name": "max_price_major",
                    "overwrite": true,
                    "type": "Numeric",
                    "width": 1
                },
                {
                    "bgcolor": 12,
                    "column": 36,
                    "cursor_home": true,
                    "default": "99",
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "00-99 oder # eingeben",
                    "line": 16,
                    "name": "max_price_minor",
                    "overwrite": true,
                    "type": "Numeric",
                    "width": 2
                },
                {
                    "bgcolor": 12,
                    "column": 11,
                    "fgcolor": 3,
                    "height": 1,
                    "hint": "Neues Kennwort",
                    "line": 19,
                    "name": "password",
                    "type": "password",
                    "validate": true,
                    "width": 14
                }
            ],
            "target": "call:User_UI.callback_add_user"
        },
        "links": [
            { "code": "0", "target": "0" },
            { "code": "1", "target": "88" },
            { "code": "2", "target": "89" },
            { "code": "5", "target": "810" }
        ],
        "publisher_color": 7,
        "publisher_name": "!BTX"
    }
    "#;
    let meta: Meta = serde_json::from_str(meta_str).unwrap();

    let mut cept = Cept::new();
    cept += create_title("Neuen Benutzer einrichten");
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
    cept.repeat(b' ', 7);
    cept.add_str("Ort:");
    cept.set_cursor(11, 31);
    cept.add_str("Land:");
    cept.add_raw(b"\r\n");
    cept += line();
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
    cept += line();
    cept.add_raw(b"\r\n");
    cept.add_str("Kennwort: ");
    cept.add_raw(b"\r\n\r\n");
    cept += line();

    Page { cept, meta }
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

pub fn create(pageid: &PageId) -> Option<Page> {
    if pageid.page == "77" {
        Some(create_add_user())
    } else {
        None
    }
}