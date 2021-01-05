// use std::collections::HashMap;

// use chrono::Local;
// use crate::{editor::{InputField, Inputs}, sysmsg::SysMsg};

// use super::user::*;
// use super::cept::*;
// use super::page::*;
// use super::session::*;

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

// fn messaging_create_list(userid: &UserId, is_read: bool) -> (Meta, Cept) {
//     let title = if is_read {
//         "Zurückgelegte Mitteilungen"
//     } else {
//         "Neue Mitteilungen"
//     };
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
//             links.push(Link::new(&(index + 1).to_string(), &(target_prefix.to_owned() + &(index + 1).to_string())));
//         } else {
//             cept.add_raw(b"\r\n\r\n");
//         }
//     }

//     let meta = Meta {
//         publisher_name: Some("!BTX".to_owned()),
//         include: Some("a".to_owned()),
//         clear_screen: Some(true),
//         links: Some(links),
//         publisher_color: Some(7),
//         ..Default::default()
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
//         clear_screen: Some(true),
//         links: Some(vec!(
//             Link::new("0", if is_read { "89" } else { "88"}),
//         )),
//         publisher_color: Some(7),
//         ..Default::default()
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

// pub fn callback_validate_user_id(_: &PageId, input_data: &HashMap<String, String>) -> ActionResult {
//     if User::exists(input_data.user_id) {
//         ActionResult::Ok
//     } else {
//         ActionResult::Error(SysMsg::Custom("Teilnehmerkennung ungültig! -> #"))
//     }
// }

// pub fn callback_validate_ext(_: &PageId, input_data: &HashMap<String, String>) -> ActionResult {
//     if User::exists(input_data.user_id, input_data.ext) {
//         ActionResult::Ok
//     } else {
//         ActionResult::Error(SysMsg::Custom("Mitbenutzernummer ungültig! -> #"))
//     }
// }

// fn messaging_create_compose(user: &User) -> (Meta, Cept) {
//     let meta = Meta {
//         include: Some("a".to_owned()),
//         clear_screen: Some(true),
//         links: Some(vec!(
//             Link::new("0", "8"),
//         )),
//         publisher_color: Some(7),
//         inputs: Some(Inputs {
//             fields: vec!(
//                 InputField {
//                     name: "user_id",
//                     input_type: "user_id",
//                     line: 8,
//                     column: 20,
//                     height: 1,
//                     width: 16,
//                     bgcolor: 4,
//                     fgcolor: 3,
//                     action: callback_validate_user_id,
//                     ..Default::default()
//                 },
//                 InputField {
//                     name: "ext",
//                     input_type: "ext",
//                     line: 8,
//                     column: 37,
//                     height: 1,
//                     width: 1,
//                     bgcolor: 4,
//                     fgcolor: 3,
//                     default: 1,
//                     action: callback_validate_ext,
//                     ..Default::default()
//                 },
//                 InputField {
//                     name: "body",
//                     line: 12,
//                     column: 1,
//                     height: 10,
//                     width: 40,
//                     bgcolor: 4,
//                     fgcolor: 3,
//                     ..Default::default()
//                 }
//             ),
//             action: "send_message",
//             price: 30,
//             // target: "page:8",
//             ..Default::default()
//         }),
//         ..Default::default()
//     };

//     let now = Local::now();
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
//         return messaging_create_list(user, false)
//     } else if pageid == "89a" {
//         return messaging_create_list(user, true)
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
