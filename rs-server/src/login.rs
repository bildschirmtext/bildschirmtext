use std::collections::HashMap;
use chrono::Utc;
use std::str::FromStr;
use super::editor::*;
use super::pages::*;
use super::session::*;
use super::user::*;

pub fn create(pageid: &PageId, user: Option<&User>) -> Option<Page> {
    if pageid.page == "00000" {
        Some(create_login())
    } else if pageid.page == "000001" {
        Some(create_start(user)) // XXX user
    } else if pageid.page == "9" {
        Some(create_logout())
    } else {
         None
    }
}

pub fn action_login(_: &PageId, input_data: &HashMap<String, String>) -> UserRequest {
    if User::login(
        input_data.get("user_id").unwrap(),
        input_data.get("ext").unwrap(),
        input_data.get("password").unwrap(),
        false
    ) {
        println!("login ok");
        UserRequest::Goto(PageId::from_str("000001").unwrap(), true)
    } else {
        println!("login incorrect");
        UserRequest::Error(Error::Custom("Ungültiger Teilnehmer/Kennwort -> #".to_owned()))
    }
}

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
                    // action: Some(validate_login),
                    ..Default::default()
                }
            ),
            no_navigation: true,
            action: Some(action_login),
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

fn create_start(user: Option<&User>) -> Page {
    let mut links = vec!(Link::new("#", "0"));

    // if user.messaging.has_new_messages():
    //     links["8"] = "88"

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

    let now = Utc::now();
    let current_date = now.format("%d.%m.%Y  %H:%M").to_string();
    let last_date;
    let last_time;
    // if user.stats.last_login is not None:
    //     t = datetime.datetime.fromtimestamp(user.stats.last_login)
    //     last_date = t.strftime("%d.%m.%Y")
    //     last_time = t.strftime("%H:%M")
    // else:
        last_date = "--.--.----".to_owned();
        last_time = "--:--".to_owned();

    let mut user_name;
    if let Some(user) = user {
        user_name = String::new();
        if let Some(salutation) = &user.salutation {
            user_name += &salutation;
            user_name.push('\n');
        }
        if let Some(first_name) = &user.first_name {
            user_name += &first_name;
            user_name.push('\n');
        }
        if let Some(last_name) = &user.last_name {
            user_name += &last_name;
            user_name.push('\n');
        }
    } else {
        user_name = "".to_owned();
    }

    // notifications = Login_UI.notifications(user) //XXX
    let notifications = "";

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
    page.cept.add_str(&notifications);
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

