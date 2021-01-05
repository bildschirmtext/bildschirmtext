// use std::collections::HashMap;

// use chrono::Local;
// use crate::{editor::{InputField, Inputs}, sysmsg::SysMsg};

// use super::cept::*;
// use super::dispatch::*;
// use super::messaging::*;
// use super::page::*;
// use super::session::*;
// use super::user::*;

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

// fn messaging_create_main_menu() -> Page {
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
//     Page { meta, cept }
// }

// fn messaging_create_list(userid: &UserId, is_read: bool) -> Page {
//     let messaging = Messaging::for_userid(&userid);

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

//     let messages = messaging.select(is_read, 0, Some(9));

//     for index in 0..9 {
//         cept.add_str(&(index + 1).to_string());
//         cept.add_str("  ");
//         if index < messages.len() {
//             let message = messages[index];
//             let from_user = User::get(&message.from_userid);
//             if let Some(from_user) = from_user {
//                 match from_user.public {
//                     UserDataPublic::Person(person) => {
//                         if let Some(first_name) = &person.first_name {
//                             cept.add_str(&first_name);
//                             cept.add_raw(b" ");
//                         }
//                         if let Some(last_name) = &person.last_name {
//                             cept.add_str(last_name);
//                             cept.add_raw(b"\r\n   ");
//                         }
//                     },
//                     UserDataPublic::Organization(organization) => {
//                         if let Some(name1) = &organization.name1 {
//                             cept.add_str(name1);
//                         }
//                     },
//                 }
//             } else {

//             }
//             cept.add_str(&message.from_date());
//             cept.add_raw(b"   ");
//             cept.add_str(&message.from_time());
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
//     Page { meta, cept }
// }

// fn messaging_create_message_detail(userid: &UserId, index: usize, is_read: bool) -> Option<Page> {
//     let messaging = Messaging::for_userid(&userid);

//     let messages = messaging.select(is_read, index, Some(1));
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
//     let from_user = User::get(&message.from_userid);
//     if let Some(from_user) = from_user {
//         from_street = from_user.private.street.unwrap_or("".to_owned());
//         from_zip = from_user.private.zip.unwrap_or("".to_owned());
//         from_city = from_user.private.city.unwrap_or("".to_owned());
//     } else {
//         // XXX user not found
//         from_street = "".to_owned();
//         from_zip = "".to_owned();
//         from_city = "".to_owned();
//     }

//     let cept = Cept::new();
//     cept.parallel_limited_mode();
//     cept.set_cursor(2, 1);
//     cept.set_fg_color(3);
//     cept.add_str("von ");
//     cept.add_str(message.from_userid.id.ljust(12));
//     cept.add_str(" ");
//     cept.add_raw(message.from_userid.ext.rjust(5, '0'));
//     cept.set_cursor(2, 41 - from_date.len() as u8);
//     cept.add_str(&from_date);
//     cept.repeat(b' ', 4);
//     cept.add_str(message.from_user.org_name);
//     cept.set_cursor(3, 41 - from_time.len() as u8);
//     cept.add_str(&from_time);
//     cept.repeat(b' ', 4);
//     cept.set_fg_color_simple(0);
//     cept.add_str(message.from_user.first_name);
//     cept.add_str(" ");
//     cept.add_str(message.from_user.last_name);
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(&from_street);
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(&from_zip);
//     cept.add_raw(b' ');
//     cept.add_str(&from_city);
//     cept.add_raw(b"\r\n");
//     cept.add_str("an  ");
//     cept.add_str(userid.id.ljust(12));
//     cept.add_str(" ");
//     cept.add_str(user.ext.rjust(5, '0'));
//     cept.add_raw(b"\r\n");
//     cept.repeat(b' ', 4);
//     cept.add_str(&user.first_name.unwrap());
//     cept.add_str(" ");
//     cept.add_str(&user.last_name.unwrap());
//     cept.add_raw(b"\r\n\n");
//     cept.add_str(&message.body());
//     cept.set_cursor(23, 1);
//     cept.add_raw(b"0");
//     cept.add_raw(&[
//         0x1b, 0x29, 0x20, 0x40,                                    // load DRCs into G1
//         0x1b, 0x7e                                            // G1 into right charset
//     ]);
//     cept.add_str(" Gesamtübersicht");
//     cept.repeat(b' ', 22);

//     messaging.mark_as_read(message.index);

//     Some(Page { meta, cept })
// }

// pub fn callback_validate_user_id(_: &PageId, input_data: &HashMap<String, String>) -> ActionResult {
//     if User::exists(&UserId::new(input_data.get("user_id").unwrap(), "1")) { // XXX
//         ActionResult::Ok
//     } else {
//         ActionResult::Error(SysMsg::Custom("Teilnehmerkennung ungültig! -> #".to_owned()))
//     }
// }

// pub fn callback_validate_ext(_: &PageId, input_data: &HashMap<String, String>) -> ActionResult {
//     if User::exists(&UserId::new(input_data.get("user_id").unwrap(), input_data.get("ext").unwrap())) {
//         ActionResult::Ok
//     } else {
//         ActionResult::Error(SysMsg::Custom("Mitbenutzernummer ungültig! -> #".to_owned()))
//     }
// }

// fn messaging_create_compose(userid: &UserId) -> Page {
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
//                     name: "user_id".to_owned(),
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
//                     name: "ext".to_owned(),
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
//                     name: "body".to_owned(),
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
//     Page { meta, cept }
// }

// fn create_page(pageid: &PageId, private_context: PrivateContext) -> Option<Page> {
//     let user = private_context.user;

//     if let Some(user) = user {
//         if pageid.page == "8" {
//             Some(messaging_create_main_menu())
//         } else if pageid.page == "88" {
//             Some(messaging_create_list(&user.userid, false))
//         } else if pageid.page == "89" {
//             Some(messaging_create_list(&user.userid, true))
//         // } else if re.search("^88\da$", pageid.page) {
//         //     return messaging_create_message_detail(user, int(pageid.page[2..-1]) - 1, False)
//         // } else if re.search("^89\da$", pageid.page) {
//         //     return messaging_create_message_detail(user, int(pageid.page[2..-1]) - 1, True)
//         } else if pageid.page == "810" {
//             Some(messaging_create_compose(&user.userid))
//         } else {
//             None
//         }
//     } else {
//         None
//     }
// }


// // if pageid.page == "00000" {
// //     Some(create_login())
// // } else if pageid.page == "000001" {
// //     Some(create_start(private_context)) // XXX user
// // } else if pageid.page == "9" {
// //     Some(create_logout())
// // } else {
// //      None
// // }
