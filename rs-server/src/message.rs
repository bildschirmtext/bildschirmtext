// use serde::{Deserialize, Serialize};
// use std::{fs::File, io::Write};
// use chrono::Utc;
// use chrono::TimeZone;
// use uuid::Uuid;
// use super::cept::*;
// use super::pages::*;
// use super::session::*;
// use super::staticp::*;

// const PATH_MESSAGES: &str = "../messages/";

// #[derive(Serialize, Deserialize)]
// struct MessageDatabase {
//     messages: Vec<Message>,
// }

// impl MessageDatabase {
//     fn new() -> Self {
//         Self {
//             messages: vec!()
//         }
//     }
// }

// #[derive(Clone)]
// #[derive(Serialize, Deserialize)]
// struct Message {
//     body: String,
//     from_user_id: String,
//     from_ext: String,
//     personal_data: bool,
//     timestamp: i64,
//     is_read: bool,
//     uuid: Uuid,
// }

// impl Message {
// 	fn from_date(&self) -> String {
//         let t = Utc.timestamp(self.timestamp, 0);
//         t.format("%d.%m.%Y").to_string()
//     }

// 	fn from_time(&self) -> String {
//         let t = Utc.timestamp(self.timestamp, 0);
//         t.format("%H:%M").to_string()
//     }
// }

// struct Messaging<'a> {
// 	user: &'a User,
//     database: MessageDatabase,
// }


// impl Messaging<'_> {
// 	fn new(user: &User) -> Self {
//         Self {
//             user: user,
//             database: MessageDatabase::new(),
//         }
//     }

// 	fn database_filename(user_id: &str, ext: &str) -> String {
//         let mut s = String::new();
//         s += PATH_MESSAGES;
//         s += user_id;
//         s.push('-');
//         s += ext;
//         s += ".messages";
//         s
//     }

// 	fn load_database(user_id: &str, ext: &str) -> MessageDatabase {
// 		let filename = Self::database_filename(user_id, ext);
// 		if !is_file(&filename) {
// 			println!("messages file not found");
// 			MessageDatabase::new()
//         } else {
//             let f = File::open(&filename).unwrap();
//             let database: MessageDatabase = serde_json::from_reader(f).unwrap();
//             database
//         }
//     }

// 	fn save_database(user_id: &str, ext: &str, database: &MessageDatabase) {
//         let json_data = serde_json::to_string(database).unwrap();
//         let mut file = File::create(Self::database_filename(user_id, ext)).unwrap();
//         file.write_all(&json_data.as_bytes());
//     }

// 	fn load(&mut self) {
//         self.database = Messaging::load_database(&self.user.user_id, &self.user.ext);
//     }

// 	fn save(&mut self) {
//         Messaging::save_database(&self.user.user_id, &self.user.ext, &self.database);
//     }

// 	fn select(&mut self, is_read: bool, start: usize, count: Option<usize>) -> Vec<Message> {
// 		self.load();

// 		let mut ms = vec!();
// 		let mut j = 0;
// 		for i in (0..self.database.messages.len()).rev() {
// 			let m = self.database.messages[i];
// 			if m.is_read != is_read {
//                 continue;
//             }
//             if j < start {
//                 continue;
//             }
//             if let Some(count) = count {
//                 if j >= start + count {
//                     continue;
//                 }
//             }
//             ms.push(m);
//             j += 1;
//         }

//         return ms;
//     }

// 	fn mark_as_read(&mut self, index: usize) {
// 		self.load();
// 		if !self.database.messages[index].is_read {
// 			self.database.messages[index].is_read = true;
//             self.save();
//         }
//     }

// 	fn has_new_messages(&mut self) {
// 		self.load();
//         self.select(false, 0, None).len() != 0;
//     }

// 	fn send(&mut self, user_id: &str, ext: &str, body: &str) {
// 		let database = Messaging::load_database(user_id, ext);
// 		database.messages.push(
//             Message {
// 				from_user_id: self.user.user_id,
// 				from_ext: self.user.ext,
// 				personal_data: false,
// 				timestamp: Utc::now().timestamp(),
//                 body: body.to_owned(),
//                 is_read: false,
// 			},
// 		);
//         Messaging::save_database(user_id, ext, &database);
//     }
// }

// // ************
// // UI
// // ************

// // private
// fn messaging_create_title(title: &str) -> Cept {
//     let cept = Cept::new();
//     cept.set_cursor(2, 1);
//     cept.set_palette(1);
//     cept.set_screen_bg_color_simple(4);
//     cept.add_raw(&[
//         0x1b, 0x28, 0x40,           // load G0 into G0
//         0x0f                   // G0 into left charset
//     ]);
//     cept.parallel_mode();
//     cept.set_palette(0);
//     cept.code_9e();
//     cept.add_raw(b"\n\r");
//     cept.set_line_bg_color_simple(4);
//     cept.add_raw(b"\n");
//     cept.set_line_bg_color_simple(4);
//     cept.set_palette(1);
//     cept.double_height();
//     cept.add_raw(b"\r");
//     cept.add_str(title);
//     cept.add_raw(b"\n\r");
//     cept.set_palette(0);
//     cept.normal_size();
//     cept.code_9e();
//     cept.set_fg_color_simple(7);
//     cept
// }

// // private
// fn messaging_create_menu(title: &str, items: &[&str]) -> Cept {
//     let cept = messaging_create_title(title);
//     cept.add_raw(b"\n\r\n\r");
//     let mut i = 1;
//     for item in items {
//         let s = String::new();
//         cept.add_str(&i.to_string());
//         cept.add_str("  ");
//         cept.add_str(item);
//         cept.add_raw(b"\r\n\r\n");
//         i += 1;
//     }

//     cept.add_raw(b"\r\n\r\n\r\n\r\n\r\n\r\n");
//     cept.set_line_bg_color_simple(4);
//     cept.add_raw(b"0\x19\x2b");
//     cept.add_str(" Gesamtübersicht");

//     cept
// }

// fn messaging_create_main_menu() -> (Meta, Cept) {
//     let meta = Meta {
//         publisher_name: Some("!BTX".to_owned()),
//         include: Some("a".to_owned()),
//         clear_screen: Some(true),
//         links: Some(vec!(
//             Link::new("0", "0"),
//             Link::new("1", "88"),
//             Link::new("2", "89"),
//             Link::new("5", "810"),
//         )),
//         publisher_color: Some(7),

//         cls2: None,
//         parallel_mode: None,
//         inputs: None,
//         palette: None,
//         autoplay: None,
//     };

//     let cept = messaging_create_menu(
//         "Mitteilungsdienst",
//         &[
//             "Neue Mitteilungen",
//             "Zurückgelegte Mitteilungen",
//             "Abruf Antwortseiten",
//             "Ändern Mitteilungsempfang",
//             "Mitteilungen mit Alphatastatur"
//         ]
//     );
//     (meta, cept)
// }

// fn messaging_create_list(user: &User, is_read: bool) -> (Meta, Cept) {
//     let title = if is_read {
//         "Zurückgelegte Mitteilungen"
//     } else {
//         "Neue Mitteilungen"
//     }
//     let cept = messaging_create_title(title);

//     let mut links = vec!(
//         Link::new("0", "8"),
//     );

//     let target_prefix = if is_read {"89" } else { "88" };

//     let messages = user.messaging.select(is_read, 0, 9);

//     for index in 0..9 {
//         cept.add_str(&(index + 1).to_string());
//         cept.add_str("  ");
//         if index < messages.len() {
//             let message = messages[index];
//             if message.from_user.org_name {
//                 cept.add_str(message.from_user.org_name);
//             } else {
//                 cept.add_str(message.from_user.first_name);
//                 cept.add_raw(b" ");
//                 cept.add_str(message.from_user.last_name);
//                 cept.add_raw(b"\r\n   ");
//             }
//             cept.add_str(message.from_date());
//             cept.add_raw(b"   ");
//             cept.add_str(message.from_time());
//             cept.add_raw(b"\r\n");
//             links[(index + 1).to_string()] = target_prefix + (index + 1).to_string();
//         } else {
//             cept.add_raw(b"\r\n\r\n");
//         }
//     }

//     let meta = Meta {
//         publisher_name: Some("!BTX".to_owned()),
//         include: Some("a".to_owned()),
//         clear_screen: Some(true),
//         links: links,
//         publisher_color: Some(7_),
//     };
//     (meta, cept)
// }

// fn messaging_create_message_detail(user: &User, index: usize, is_read: bool) -> Option<(Meta, Cept)> {
//     let messages = user.messaging.select(is_read, index, 1);
//     if messages.len() == 0 {
//         return None;
//     }

//     let message = messages[0];

//     let meta = Meta {
//         publisher_name: Some("Bildschirmtext".to_owned()),
//         include: Some("11a".to_owned()),
//         palette: Some("11a".to_owned()),
//         clear_screen: true,
//         links: Some(vec!(
//             Link::new("0", if is_read { "89" } else { "88"}),
//         )),
//         publisher_color: Some(7),
//     };

//     let from_date = message.from_date();
//     let from_time = message.from_time();
//     let from_street;
//     let from_zip;
//     let from_city;
//     if message.from_user.personal_data {
//         from_street = message.from_user.street;
//         from_zip = message.from_user.zip;
//         from_city = message.from_user.city;
//     } else {
//         from_street = "";
//         from_zip = "";
//         from_city = "";
//     }

//     let cept = Cept::new();
//     cept.parallel_limited_mode();
//     cept.set_cursor(2, 1);
//     cept.set_fg_color(3);
//     cept.add_str("von ");
//     cept.add_str(message.from_user.user_id.ljust(12));
//     cept.add_str(" ");
//     cept.add_raw(message.from_user.ext.rjust(5, '0'));
//     cept.set_cursor(2, 41 - from_date.len());
//     cept.add_str(from_date);
//     cept.repeat(b' ', 4);
//     cept.add_str(message.from_user.org_name);
//     cept.set_cursor(3, 41 - from_time.len());
//     cept.add_str(from_time);
//     cept.repeat(b' ', 4);
//     cept.set_fg_color_simple(0);
//     cept.add_str(message.from_user.first_name);
//     cept.add_str(" ");
//     cept.add_str(message.from_user.last_name);
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(from_street);
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(from_zip);
//     cept.add_raw(b' ');
//     cept.add_str(from_city);
//     cept.add_raw(b"\r\n");
//     cept.add_str("an  ");
//     cept.add_str(user.user_id.ljust(12));
//     cept.add_str(" ");
//     cept.add_str(user.ext.rjust(5, '0'));
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(&user.first_name.unwrap());
//     cept.add_str(" ");
//     cept.add_str(&user.last_name.unwrap());
//     cept.add_raw(b"\r\n\n");
//     cept.add_str(message.body());
//     cept.set_cursor(23, 1);
//     cept.add_raw(b"0");
//     cept.add_raw(&[
//         0x1b, 0x29, 0x20, 0x40,                                    // load DRCs into G1
//         0x1b, 0x7e                                            // G1 into right charset
//     ]);
//     cept.add_str(" Gesamtübersicht");
//     cept.repeat(' ', 22);

//     user.messaging.mark_as_read(message.index);

//     Some((meta, cept))
// }

// // fn callback_validate_user_id(input_data: &[(String, String)]) {
// //     if User.exists(input_data.user_id) {
// //         return util::VALIDATE_INPUT_OK;
// //     } else {
// //         let msg = util::create_custom_system_message("Teilnehmerkennung ungültig! -> #");
// //         write(stream, msg);
// //         util::wait_for_ter();
// //         return util::VALIDATE_INPUT_BAD;
// //     }
// // }

// // fn callback_validate_ext(input_data: &[(String, String)]) {
// //     if User.exists(input_data.user_id, input_data.ext) {
// //         return util::VALIDATE_INPUT_OK;
// //     } else {
// //         let msg = util::create_custom_system_message("Mitbenutzernummer ungültig! -> #");
// //         write(stream, msg);
// //         util::wait_for_ter();
// //         return util::VALIDATE_INPUT_RESTART;
// //     }
// // }

// fn messaging_create_compose(user: &User) -> (Meta, Cept) {
//     let meta = Meta {
//         include: Some("a".to_owned()),
//         clear_screen: Some(true),
//         links: Some(vec!(
//             Link::new("0", "8"),
//         )),
//         publisher_color: Some(7,)
//         inputs: Inputs {
//             fields: vec!(
//                 InputField {
//                     name: "user_id",
//                     type: "user_id",
//                     line: 8,
//                     column: 20,
//                     height: 1,
//                     width: 16,
//                     bgcolor: 4,
//                     fgcolor: 3,
//                     validate: "call:Messaging_UI.callback_validate_user_id"
//                 },
//                 InputField {
//                     name: "ext",
//                     type: "ext",
//                     line: 8,
//                     column: 37,
//                     height: 1,
//                     width: 1,
//                     bgcolor: 4,
//                     fgcolor: 3,
//                     default: 1,
//                     validate: "call:Messaging_UI.callback_validate_ext"
//                 },
//                 InputField {
//                     name: "body",
//                     line: 12,
//                     column: 1,
//                     height: 10,
//                     width: 40,
//                     bgcolor: 4,
//                     fgcolor: 3
//                 }
//             ),
//             action: "send_message",
//             price: 30,
//             target: "page:8"
//         }
//     };

//     let now = Utc::now();
//     let current_date = now.format("%d.%m.%Y").to_string();
//     let current_time = now.format("%H:%M").to_string();

//     let mut cept = Cept::new();
//     cept.set_cursor(2, 1);
//     cept.set_palette(1);
//     cept.set_screen_bg_color_simple(4);
//     cept.add_raw(&[
//         0x1b, 0x28, 0x40                                    // load G0 into G0
//     ]);
//     cept.add_raw(&[
//         0x0f                                            // G0 into left charset
//     ]);
//     cept.parallel_mode();
//     cept.set_palette(0);
//     cept.code_9e();
//     cept.add_raw(b"\n\r");
//     cept.set_line_bg_color_simple(4);
//     cept.add_raw(b"\n");
//     cept.set_line_bg_color_simple(4);
//     cept.set_palette(1);
//     cept.double_height();
//     cept.add_raw(b"\r");
//     cept.add_str("Mitteilungsdienst");
//     cept.add_raw(b"\n\r");
//     cept.set_palette(0);
//     cept.normal_size();
//     cept.code_9e();
//     cept.set_fg_color_simple(7);
//     cept.add_str("Absender:");
//     cept.add_str(&user.user_id);
//     cept.set_cursor(5, 25);
//     cept.add_str(&user.ext);
//     cept.set_cursor(6, 10);
//     cept.add_str(&user.first_name.unwrap());
//     cept.set_cursor(7, 10);
//     cept.add_str(&user.last_name.unwrap());
//     cept.set_cursor(5, 31);
//     cept.add_str(&current_date);
//     cept.set_cursor(6, 31);
//     cept.add_str(&current_time);
//     cept.add_raw(b"\r\n\n");
//     cept.add_str("Tln.-Nr. Empfänger:");
//     cept.set_cursor(8, 36);
//     cept.add_str("-");
//     cept.add_raw(b"\r\n\n\n");
//     cept.add_str("Text:");
//     cept.add_raw(b"\r\n\n\n\n\n\n\n\n\n\n\n\n");
//     cept.set_line_bg_color_simple(4);
//     cept.add_str("0");
//     cept.add_raw(&[
//         0x19,                                            // switch to G2 for one character
//         0x2b, 0xfe, 0x7f,                                    // "+."
//     ]);
//     (meta, cept)
// }

// fn create_page(user: &User, pageid: &PageId) -> Option<Page> {
//     if pageid == "8a" {
//         return messaging_create_main_menu()
//     } else if pageid == "88a" {
//         return messaging_create_list(user, False)
//     } else if pageid == "89a" {
//         return messaging_create_list(user, True)
//     // } else if re.search("^88\da$", pageid) {
//     //     return messaging_create_message_detail(user, int(pageid[2..-1]) - 1, False)
//     // } else if re.search("^89\da$", pageid) {
//     //     return messaging_create_message_detail(user, int(pageid[2..-1]) - 1, True)
//     } else if pageid == "810a" {
//         return messaging_create_compose(user)
//     } else {
//         return None
//     }
// }
