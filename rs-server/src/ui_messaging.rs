use std::collections::HashMap;

use chrono::Local;
use std::str::FromStr;

use super::cept::*;
use super::dispatch::*;
use super::messaging::*;
use super::page::*;
use super::session::*;
use super::sysmsg::*;
use super::user::*;
use super::editor::*;

pub struct MessagingPageSession {
    pageid: PageId,
    user: User,
}

pub fn new<'a>(arg: &str, pageid: PageId, user: User) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(MessagingPageSession { pageid, user })
}

impl<'a> PageSession<'a> for MessagingPageSession {

    fn create(&self) -> Option<Page> {
        let user = &self.user;

        if user.is_anonymous() {
            if self.pageid.page == "8" {
                Some(messaging_create_main_menu())
            } else if self.pageid.page == "88" {
                Some(messaging_create_list(&user.userid, false))
            } else if self.pageid.page == "89" {
                Some(messaging_create_list(&user.userid, true))
            } else if self.pageid.page.starts_with("88") {
                return messaging_create_message_detail(&user.userid, usize::from_str(&self.pageid.page[2..]).unwrap() - 1, false)
            } else if self.pageid.page.starts_with("89") {
                return messaging_create_message_detail(&user.userid, usize::from_str(&self.pageid.page[2..]).unwrap() - 1, true)
            } else if self.pageid.page == "810" {
                Some(messaging_create_compose(&user))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult {
        if self.pageid.page == "810" {
            match name {
                "user_id" => {
                    if User::exists(&UserId::new(input_data.get("user_id").unwrap(), "1")) { // XXX
                        ValidateResult::Ok
                    } else {
                        ValidateResult::Error(SysMsg::Custom("Teilnehmerkennung ungültig! -> #".to_owned()))
                    }
                },
                "ext" => {
                    if User::exists(&UserId::new(input_data.get("user_id").unwrap(), input_data.get("ext").unwrap())) {
                        ValidateResult::Ok
                    } else {
                        ValidateResult::Error(SysMsg::Custom("Mitbenutzernummer ungültig! -> #".to_owned()))
                    }
                }
                _ => unreachable!()
            }
        } else {
            unreachable!()
        }
    }

    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest {
        if self.pageid.page == "810" {
            let from_userid = &self.user.userid;
            let to_userid = UserId::new(input_data.get("user_id").unwrap(), input_data.get("ext").unwrap());
            send_message(&from_userid, &to_userid, input_data.get("body").unwrap());
            UserRequest::MessageGoto(SysMsg::Code(SysMsgCode::Sent, None), PageId::from_str("8").unwrap(), true)
        } else {
            unreachable!()
       }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn messaging_create_title(title: &str) -> Cept {
    let mut cept = Cept::new();
    cept.set_cursor(2, 1);
    cept.set_palette(1);
    cept.set_screen_bg_color_simple(4);
    cept.load_g0_g0();
    cept.set_left_g0();
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

fn messaging_create_menu(title: &str, items: &[&str]) -> Cept {
    let mut cept = messaging_create_title(title);
    cept.add_raw(b"\n\r\n\r");
    let mut i = 1;
    for item in items {
        cept.add_str(&i.to_string());
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

fn messaging_create_main_menu() -> Page {
    let meta = Meta {
        publisher_name: Some("!BTX".to_owned()),
        include: Some("a".to_owned()),
        clear_screen: Some(true),
        links: Some(vec!(
            Link::new("0", "0"),
            Link::new("1", "88"),
            Link::new("2", "89"),
            Link::new("5", "810"),
        )),
        publisher_color: Some(7),
        ..Default::default()
    };

    let cept = messaging_create_menu(
        "Mitteilungsdienst",
        &[
            "Neue Mitteilungen",
            "Zurückgelegte Mitteilungen",
            "Abruf Antwortseiten",
            "Ändern Mitteilungsempfang",
            "Mitteilungen mit Alphatastatur"
        ]
    );
    Page {
        meta,
        cept_palette: None,
        cept_include: None,
        cept
    }
}

fn messaging_create_list(userid: &UserId, is_read: bool) -> Page {
    let mut messaging = MessageBox::for_userid(&userid);

    let title = if is_read {
        "Zurückgelegte Mitteilungen"
    } else {
        "Neue Mitteilungen"
    };
    let mut cept = messaging_create_title(title);

    let mut links = vec!(
        Link::new("0", "8"),
    );

    let target_prefix = if is_read {"89" } else { "88" };

    let messages = messaging.select(is_read, 0, Some(9));

    for index in 0..9 {
        cept.add_str(&(index + 1).to_string());
        cept.add_str("  ");
        if index < messages.len() {
            let message = messages[index];
            cept.add_str(&message.from_name);
            cept.add_raw(b"\r\n   ");
            cept.add_str(&message.from_date());
            cept.add_raw(b"   ");
            cept.add_str(&message.from_time());
            cept.add_raw(b"\r\n");
            links.push(Link::new(&(index + 1).to_string(), &(target_prefix.to_owned() + &(index + 1).to_string())));
        } else {
            cept.add_raw(b"\r\n\r\n");
        }
    }

    let meta = Meta {
        publisher_name: Some("!BTX".to_owned()),
        include: Some("a".to_owned()),
        clear_screen: Some(true),
        links: Some(links),
        publisher_color: Some(7),
        ..Default::default()
    };
    Page {
        meta,
        cept_palette: None,
        cept_include: None,
        cept
    }
}

fn messaging_create_message_detail(userid: &UserId, index: usize, is_read: bool) -> Option<Page> {
    let mut messaging = MessageBox::for_userid(&userid);

    let messages = messaging.select(is_read, index, Some(1));
    if messages.len() == 0 {
        return None;
    }

    let message = messages[0];

    let meta = Meta {
        publisher_name: Some("Bildschirmtext".to_owned()),
        include: Some("11a".to_owned()),
        palette: Some("11a".to_owned()),
        clear_screen: Some(true),
        links: Some(vec!(
            Link::new("0", if is_read { "89" } else { "88"}),
        )),
        publisher_color: Some(7),
        ..Default::default()
    };

    let from_date = message.from_date();
    let from_time = message.from_time();

    let mut cept = Cept::new();
    cept.parallel_limited_mode();
    cept.set_cursor(2, 1);
    cept.set_fg_color(3);
    cept.add_str("von ");
    cept.add_str(&message.from_address);
    cept.set_cursor(2, 41 - from_date.len() as u8);
    cept.add_str(&from_date);
    cept.repeat(b' ', 4);
    // cept.add_str(message.from_user.org_name);
    cept.set_cursor(3, 41 - from_time.len() as u8);
    cept.add_str(&from_time);
    cept.repeat(b' ', 4);
    cept.set_fg_color_simple(0);
    cept.add_str(&message.from_name);
    cept.add_raw(b"\r\n");
    cept.repeat(b' ', 4);
    // cept.add_str(&from_street);
    cept.add_raw(b"\r\n");
    cept.repeat(b' ', 4);
    // cept.add_str(&from_zip);
    cept.add_raw(b" ");
    // cept.add_str(&from_city);
    cept.add_raw(b"\r\n");
    cept.add_str("an  ");
    cept.add_str(&userid.to_string());
    cept.add_raw(b"\r\n");
    cept.repeat(b' ', 4);
    cept.add_str(&User::get(&userid).unwrap().name());
    cept.add_raw(b"\r\n\n");
    cept.add_str(&message.body);
    cept.set_cursor(23, 1);
    cept.add_raw(b"0");
    cept.add_raw(&[
        0x1b, 0x29, 0x20, 0x40,                                    // load DRCs into G1
        0x1b, 0x7e                                            // G1 into right charset
    ]);
    cept.add_str(" Gesamtübersicht");
    cept.repeat(b' ', 22);

    let uuid = message.uuid.clone();

    messaging.mark_as_read(uuid);

    Some(Page {
        meta,
        cept_palette: None,
        cept_include: None,
        cept
    })
}

fn messaging_create_compose(user: &User) -> Page {
    let userid = &user.userid;

    let meta = Meta {
        include: Some("a".to_owned()),
        clear_screen: Some(true),
        links: Some(vec!(
            Link::new("0", "8"),
        )),
        publisher_color: Some(7),
        inputs: Some(Inputs {
            fields: vec!(
                InputField {
                    name: "user_id".to_owned(),
                    input_type: InputType::Numeric,
                    line: 8,
                    column: 20,
                    height: 1,
                    width: 16,
                    bgcolor: Some(4),
                    fgcolor: Some(3),
                    validate: true,
                    ..Default::default()
                },
                InputField {
                    name: "ext".to_owned(),
                    input_type: InputType::Numeric,
                    line: 8,
                    column: 37,
                    height: 1,
                    width: 1,
                    bgcolor: Some(4),
                    fgcolor: Some(3),
                    default: Some("1".to_owned()),
                    validate: true,
                    ..Default::default()
                },
                InputField {
                    name: "body".to_owned(),
                    line: 12,
                    column: 1,
                    height: 10,
                    width: 40,
                    bgcolor: Some(4),
                    fgcolor: Some(3),
                    ..Default::default()
                }
            ),
            confirm: true,
            price: Some(30),
            ..Default::default()
        }),
        ..Default::default()
    };

    let now = Local::now();
    let current_date = now.format("%d.%m.%Y").to_string();
    let current_time = now.format("%H:%M").to_string();

    let mut cept = Cept::new();
    cept.set_cursor(2, 1);
    cept.set_palette(1);
    cept.set_screen_bg_color_simple(4);
    cept.load_g0_g0();
    cept.set_left_g0();
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
    cept.add_str("Mitteilungsdienst");
    cept.add_raw(b"\n\r");
    cept.set_palette(0);
    cept.normal_size();
    cept.code_9e();
    cept.set_fg_color_simple(7);
    cept.add_str("Absender:");
    cept.add_str(&userid.id);
    cept.set_cursor(5, 25);
    cept.add_str(&userid.ext);
    cept.set_cursor(6, 10);
    cept.add_str(&user.name());
    cept.set_cursor(5, 31);
    cept.add_str(&current_date);
    cept.set_cursor(6, 31);
    cept.add_str(&current_time);
    cept.add_raw(b"\r\n\n");
    cept.add_str("Tln.-Nr. Empfänger:");
    cept.set_cursor(8, 36);
    cept.add_str("-");
    cept.add_raw(b"\r\n\n\n");
    cept.add_str("Text:");
    cept.add_raw(b"\r\n\n\n\n\n\n\n\n\n\n\n\n");
    cept.set_line_bg_color_simple(4);
    cept.add_str("0");
    cept.add_raw(&[
        0x19,             // switch to G2 for one character
        0x2b, 0xfe, 0x7f, // "+."
    ]);
    Page {
        meta,
        cept_palette: None,
        cept_include: None,
        cept
    }
}
