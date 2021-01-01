use chrono::Utc;
use super::editor::*;
use super::pages::*;
use super::session::*;

pub fn create(pageid: &str, user: Option<&User>) -> Option<Page> {
    if pageid == "00000a" {
        Some(create_login())
    } else if pageid == "000001a" {
        Some(create_start(user)) // XXX user
    } else if pageid == "9a" {
        Some(create_logout())
    } else {
         None
    }
}

pub fn validate(pageid: &str, input_data: &[(String, String)]) -> Validate {
    // if !User.login(input_data["user_id"], input_data["ext"], input_data["password"]):
    //     sys.stderr.write("login incorrect\n")
    //     msg = Util.create_custom_system_message("Ungültiger Teilnehmer/Kennwort -> #")
    //     sys.stdout.buffer.write(msg)
    //     sys.stdout.flush()
    //     Util.wait_for_ter()
    //     return Util.VALIDATE_INPUT_RESTART
    // else:
    //     sys.stderr.write("login ok\n")
    Validate::Ok
}

fn create_login() -> Page {
    let meta = Meta {
        clear_screen: Some(false),
        links: None,
        publisher_color: Some(7),
        inputs: Some(Inputs {
            fields: vec!(
                InputField {
                    name: "user_id".to_owned(),
                    hint: Some("Teilnehmernummer oder # eingeben".to_owned()),
                    line: 18,
                    column: 26,
                    height: 1,
                    width: 10,
                    bgcolor: Some(12),
                    fgcolor: Some(3),

                    typ: InputType::Normal,
                    cursor_home: false,
                    clear_line: false,
                    legal_values: None,
                    end_on_illegal_character: false,
                    end_on_legal_string: false,
                    echo_ter: false,
                    command_mode: false,
                    no_navigation: false,
                    default: None,
                    validate: None,
                },
                InputField {
                    name: "ext".to_owned(),
                    hint: Some("Mitbenutzer oder # eingeben".to_owned()),
                    line: 18,
                    column: 37,
                    height: 1,
                    width: 1,
                    bgcolor: Some(12),
                    fgcolor: Some(3),
                    typ: InputType::Number,
                    cursor_home: true,
                    default: Some("1".to_owned()),

                    clear_line: false,
                    legal_values: None,
                    end_on_illegal_character: false,
                    end_on_legal_string: false,
                    echo_ter: false,
                    command_mode: false,
                    no_navigation: false,
                    validate: None,
                },
                InputField {
                    name: "password".to_owned(),
                    hint: Some("Nächstes Feld mit #; Leer für Gast".to_owned()),
                    line: 20,
                    column: 26,
                    height: 1,
                    width: 14,
                    bgcolor: Some(12),
                    fgcolor: Some(3),
                    typ: InputType::Password,
                    validate: Some(true),

                    cursor_home: false,
                    clear_line: false,
                    legal_values: None,
                    end_on_illegal_character: false,
                    end_on_legal_string: false,
                    echo_ter: false,
                    command_mode: false,
                    no_navigation: false,
                    default: None,
                }
            ),
            confirm: false,
            no_55: false,
            target: Some("page:000001a".to_owned()),
            // no_navigation: true
        }),
        publisher_name: None,
        cls2: None,
        parallel_mode: None,
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
    page.cept.add_str("\n");
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
    page.cept.add_str("\n");
    page.cept.add_str("\n");
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
    page.cept.add_str("\n");
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
    page.cept.add_str("!\"#\n$%&");
    page.cept.cursor_up();
    page.cept.cursor_right();
    page.cept.load_g0_g0();
    page.cept.set_left_g0();
    page.cept.add_str("\n");
    page.cept.double_height();
    page.cept.add_str("Bildschirmtext");
    page.cept.add_str("\n");
    page.cept.set_line_bg_color_simple(4);
    page.cept.add_str("\n");
    page.cept.set_line_bg_color_simple(4);
    page.cept.set_palette(1);
    page.cept.double_height();
    page.cept.add_str("\n");
    page.cept.add_str("Deutsche Bundespost");
    page.cept.add_str("\n");
    page.cept.set_palette(0);
    page.cept.normal_size();
    page.cept.code_9e();
    page.cept.add_str("\n");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&current_date);
    page.cept.set_fg_color_simple(7);
    page.cept.add_str("\n\n");
    page.cept.add_str("Guten Tag");
    page.cept.add_str("\n");
    page.cept.add_str(&user_name);
    page.cept.add_str("\n");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&notifications);
    page.cept.set_fg_color_simple(7);
    page.cept.set_cursor(19, 1);
    page.cept.add_str("Sie benutzten Bildschirmtext zuletzt");
    page.cept.add_str("\n");
    page.cept.add_str("am ");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&last_date);
    page.cept.set_fg_color_simple(7);
    page.cept.add_str(" bis ");
    page.cept.set_fg_color_simple(3);
    page.cept.add_str(&last_time);
    page.cept.set_fg_color_simple(7);
    page.cept.add_str("\n\r\n\r\n");
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

