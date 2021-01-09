use std::collections::HashMap;
use chrono::Local;

use super::editor::*;
use super::page::*;
use super::session::*;
use super::user::*;
use super::dispatch::*;
use super::messaging::*;

pub struct LoginPageSession<'a> {
    pageid: &'a PageId,
    user: Option<&'a User>,
    stats: Option<&'a Stats>,
}

pub fn new<'a>(pageid: &'a PageId, user: Option<&'a User>, stats: Option<&'a Stats>) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(LoginPageSession { pageid, user, stats })
}

impl<'a> PageSession<'a> for LoginPageSession<'a> {
    fn create(&self) -> Option<Page> {
        if self.pageid.page == "00000" {
            Some(create_login())
        } else if self.pageid.page == "000001" {
            Some(create_start(self.user, self.stats)) // XXX user
        } else if self.pageid.page == "9" {
            Some(create_logout())
        } else {
             None
        }
    }

    fn validate(&self, _: &str, _: &HashMap<String, String>) -> ValidateResult {
        unreachable!()
    }

    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest {
        if self.pageid.page == "00000" {
            UserRequest::Login(
                UserId::new(
                    input_data.get("user_id").unwrap(),
                    input_data.get("ext").unwrap(),
                ),
                input_data.get("password").unwrap().clone(),
            )
        } else {
            unreachable!()
       }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn create_login() -> Page {
    let meta = Meta {
        clear_screen: Some(false),
        inputs: Some(Inputs {
            confirm: false,
            fields: vec!(
                InputField {
                    bgcolor: Some(12),
                    column: 26,
                    fgcolor: Some(3),
                    height: 1,
                    hint: Some("Teilnehmernummer oder # eingeben".to_owned()),
                    line: 18,
                    name: "user_id".to_owned(),
                    width: 10,
                    ..Default::default()
                },
                InputField {
                    bgcolor: Some(12),
                    column: 37,
                    cursor_home: true,
                    default: Some("1".to_owned()),
                    fgcolor: Some(3),
                    height: 1,
                    hint: Some("Mitbenutzer oder # eingeben".to_owned()),
                    line: 18,
                    name: "ext".to_owned(),
                    input_type: InputType::Numeric,
                    width: 1,
                    ..Default::default()
                },
                InputField {
                    bgcolor: Some(12),
                    column: 26,
                    fgcolor: Some(3),
                    height: 1,
                    hint: Some("Nächstes Feld mit #; Leer für Gast".to_owned()),
                    line: 20,
                    name: "password".to_owned(),
                    input_type: InputType::Password,
                    width: 14,
                    ..Default::default()
                }
            ),
            prohibit_command_mode: true,
            ..Default::default()
        }),
        links: None,
        publisher_color: Some(7),
        ..Default::default()
    };

    let mut page = Page::new(meta);
    page.cept.parallel_mode();
    page.cept.clear_screen();
    page.cept.set_cursor(2, 1);
    page.cept.set_screen_bg_color(12);
    page.cept.set_fg_color(7);
    btx_logo(&mut page);
    page.cept.set_left_g3();
    page.cept.set_fg_color(15);
    page.cept.add_raw(&std::iter::repeat(b'Q').take(40).collect::<Vec<u8>>());
    page.cept.set_fg_color(7);
    page.cept.set_left_g0();
    page.cept.set_cursor(18, 8);
    page.cept.add_str("Teilnehmer");
    page.cept.set_cursor(18, 25);
    page.cept.add_str(":");
    page.cept.set_cursor(18, 36);
    page.cept.set_fg_color(3);
    page.cept.add_str("-");
    page.cept.set_fg_color(7);
    page.cept.set_cursor(20, 8);
    page.cept.add_str("persönl. Kennwort:");
    page.cept.add_raw(b"\r\n");
    page.cept.set_left_g3();
    page.cept.set_fg_color(15);
    page.cept.add_raw(&std::iter::repeat(b'Q').take(40).collect::<Vec<u8>>());

    page
}

fn create_logout() -> Page {
    let meta = Meta {
        clear_screen: Some(false),
        links: Some(vec!(
            Link::new("#", "00000"),
        )),
        publisher_color: Some(7),

        publisher_name: None,
        cls2: None,
        parallel_mode: None,
        inputs: None,
        palette: None,
        include: None,
        autoplay: None,
    };

    let mut page = Page::new(meta);
    page.cept.parallel_mode();
    page.cept.clear_screen();
    page.cept.set_cursor(2, 1);
    page.cept.set_screen_bg_color(12);
    page.cept.set_fg_color(7);
    btx_logo(&mut page);
    page.cept.set_left_g3();
    page.cept.set_fg_color(15);
    page.cept.add_raw(&std::iter::repeat(b'Q').take(40).collect::<Vec<u8>>());
    page.cept.set_fg_color(7);
    page.cept.set_left_g0();
    page.cept.set_cursor(19, 8);
    page.cept.add_str("Vielen Dank für Ihren Anruf!");
    page.cept.add_raw(b"\r\n");
    page.cept.add_raw(b"\r\n");
    page.cept.set_left_g3();
    page.cept.set_fg_color(15);
    page.cept.add_raw(&std::iter::repeat(b'Q').take(40).collect::<Vec<u8>>());
    page
}

fn last_use(stats: Option<&Stats>) -> (String, String) {
    if let Some(stats) = &stats {
        if let Some(t) = stats.last_use() {
            return (t.format("%d.%m.%Y").to_string(), t.format("%H:%M").to_string());
        }
    }
    ("--.--.----".to_owned(), "--:--".to_owned())
}

fn has_new_messages(user: Option<&User>) -> bool {
     if let Some(user) = user {
        MessageBox::for_userid(&user.userid).has_new_messages()
    } else {
        false
    }
}

fn create_start(user: Option<&User>, stats: Option<&Stats>) -> Page {
    let mut links = vec!(Link::new("#", "0"));

    if has_new_messages(user) {
        links.push(Link::new("8", "88"));
    }

    if user.is_none() {
        links.push(Link::new("7", "77"));
    }

    let meta = Meta {
        include: Some("a".to_owned()),
        clear_screen: Some(true),
        links: Some(links),
        publisher_color: Some(7),

        publisher_name: None,
        cls2: None,
        parallel_mode: None,
        inputs: None,
        palette: None,
        autoplay: None,
    };

    let now = Local::now();
    let current_date = now.format("%d.%m.%Y  %H:%M").to_string();
    let (last_date, last_time) = last_use(stats);

    let mut user_name;
    if let Some(user) = &user {
        user_name = String::new();
        match &user.public {
            UserDataPublic::Person(person) => {
                if let Some(salutation) = &person.salutation {
                    user_name += &salutation;
                    user_name.push('\n');
                }
                if let Some(first_name) = &person.first_name {
                    user_name += &first_name;
                    user_name.push('\n');
                }
                if let Some(last_name) = &person.last_name {
                    user_name += &last_name;
                    user_name.push('\n');
                }
            },
            UserDataPublic::Organization(organization) => {

            }
        }
    } else {
        user_name = "".to_owned();
    }

    let mut page = Page::new(meta);

    page.cept.clear_screen();
    page.cept.cursor_home();
    page.cept.add_raw(b"\n");
    page.cept.set_palette(1);
    page.cept.set_screen_bg_color_simple(4);
    page.cept.load_g0_g0();
    page.cept.set_left_g0();
    page.cept.parallel_mode();
    page.cept.set_palette(0);
    page.cept.code_9e();
    page.cept.set_fg_color_simple(7);
    page.cept.load_g0_drcs();
    page.cept.set_left_g0();
    page.cept.add_raw(b"!\"#\r\n$%&");
    page.cept.cursor_up();
    page.cept.cursor_right();
    page.cept.load_g0_g0();
    page.cept.set_left_g0();
    page.cept.add_raw(b"\n");
    page.cept.double_height();
    page.cept.add_str("Bildschirmtext");
    page.cept.add_raw(b"\r\n");
    page.cept.set_line_bg_color_simple(4);
    page.cept.add_raw(b"\n");
    page.cept.set_line_bg_color_simple(4);
    page.cept.set_palette(1);
    page.cept.double_height();
    page.cept.add_raw(b"\n");
    page.cept.add_str("Deutsche Bundespost");
    page.cept.add_raw(b"\r\n");
    page.cept.set_palette(0);
    page.cept.normal_size();
    page.cept.code_9e();
    page.cept.add_raw(b"\r\n");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&current_date);
    page.cept.set_fg_color_simple(7);
    page.cept.add_raw(b"\r\n\n");
    page.cept.add_str("Guten Tag");
    page.cept.add_raw(b"\r\n");
    page.cept.add_str(&user_name);
    page.cept.add_raw(b"\r\n");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&notifications(user));
    page.cept.set_fg_color_simple(7);
    page.cept.set_cursor(19, 1);
    page.cept.add_str("Sie benutzten Bildschirmtext zuletzt");
    page.cept.add_raw(b"\r\n");
    page.cept.add_str("am ");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&last_date);
    page.cept.set_fg_color_simple(7);
    page.cept.add_str(" bis ");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&last_time);
    page.cept.set_fg_color_simple(7);
    page.cept.add_raw(b"\r\n\r\n\r\n");
    page.cept.set_line_bg_color_simple(4);
    page.cept.add_str("Weiter mit #  oder  *Seitennummer#");
    page
}

fn notifications(user: Option<&User>) -> String {
    if let Some(user) = user {
        if has_new_messages(Some(user)) {
            "Neue Mitteilungen mit 8".to_owned()
        } else {
            "".to_owned()
        }
    } else {
        "Als Gastbenutzer können Sie beliebige\n\
        Bildschirmtext-Inhalte abrufen.\n\
        Um Ihren eigenen Zugang einzurichten,\n\
        mit dem Sie auch Mitteilungen versenden\n\
        und empfangen können, drücken Sie jetzt\n\
        bitte die 7.".to_owned()
    }
}

fn btx_logo(page: &mut Page) {
    page.cept.from_aa(
        &[
            "    ████████████████████████████████████████████████   ",
            "   █                                                █  ",
            "  █                                                  █ ",
            " █                                                    █",
            " █                                                    █",
            " █                                                    █",
            " █                ████████████████████                █",
            " █             ██████████████████████████             █",
            " █           ██████████████████████████████           █",
            " █          ████████████████████████████████          █",
            " █         ███████████            ███████████         █",
            " █         ██████████              ██████████         █",
            " █         ██████████     ████     ██████████         █",
            " █         █████████    ████████    █████████         █",
            " █          ██████     ██████████     ██████          █",
            " █           ███   ███ ██████████ ███   ███           █",
            " █               █████ ██████████ █████               █",
            " █             ███████ ██████████ ███████             █",
            " █            ████████ ██████████ ████████            █",
            " █            ████████ ██████████ ████████            █",
            " █            ████████ ██████████ ████████            █",
            " █            █████████ ████████ █████████            █",
            " █            ██████████  ████  ██████████            █",
            " █            ████████████    ████████████            █",
            " █            ████████████████████████████            █",
            " █            ████████████████████████████            █",
            " █            ████████████████████████████            █",
            " █                                                    █",
            " █                                                    █",
            " █                                                    █",
            " █   ███ █ █   █        █   █                         █",
            " █   █ █   █   █        █              █          █   █",
            " █   █ █ █ █ ███ ███ ██ ███ █ ██ █████ ██ ███ █ █ ██  █",
            " █   ██  █ █ █ █ █   █  █ █ █ █  █ █ █ █  █ █ █ █ █   █",
            " █   █ █ █ █ █ █ ███ █  █ █ █ █  █ █ █ █  ███  █  █   █",
            " █   █ █ █ █ █ █   █ █  █ █ █ █  █ █ █ █  █   █ █ █   █",
            " █   ███ █ █ ███ ███ ██ █ █ █ █  █ █ █ ██ ███ █ █ ██  █",
            " █                                                    █",
            " █                                                    █",
            " █                                                    █",
            "  █                                                  █ ",
            "   █                                                █  ",
            "    ████████████████████████████████████████████████   "
        ], 6);
}

