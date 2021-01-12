use std::collections::HashMap;
use std::str::FromStr;
use crate::session::*;
use crate::user::*;
use super::cept::*;
use super::page::*;
use super::sysmsg::*;
use super::dispatch::*;

pub struct UsersPageSession {
    pageid: PageId,
}

pub fn new<'a>(arg: &str, pageid: PageId, _: User) -> Box<dyn PageSession<'a> + 'a> {
    Box::new(UsersPageSession { pageid })
}

impl<'a> PageSession<'a> for UsersPageSession {
    fn create(&self) -> Option<Page> {
        if self.pageid.page == "77" {
            Some(create_add_user())
        } else {
            None
        }
    }

    fn validate(&self, name: &str, input_data: &HashMap<String, String>) -> ValidateResult {
        match name {
            "user_id" => callback_validate_user_id(&self.pageid, input_data),
            "last_name" => callback_validate_last_name(&self.pageid, input_data),
            "password" => callback_validate_password(&self.pageid, input_data),
            _ => unreachable!()
        }
    }

    fn send(&self, input_data: &HashMap<String, String>) -> UserRequest {
        if User::create(
            input_data.get("user_id").unwrap(),
            "1", // ext
            input_data.get("password").unwrap(),
            input_data.get("salutation").unwrap(),
            input_data.get("last_name").unwrap(),
            input_data.get("first_name").unwrap(),
            input_data.get("street").unwrap(),
            input_data.get("zip").unwrap(),
            input_data.get("city").unwrap(),
            input_data.get("country").unwrap()
        ) {
            UserRequest::MessageGoto(SysMsg::Custom("Benutzer angelegt. Bitte neu anmelden. -> #".to_string()), PageId::from_str("00000").unwrap(), true)
        } else {
            UserRequest::Error(SysMsg::Custom("Benutzer konnte nicht angelegt werden. -> #".to_string()))
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

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
            ]
        },
        "links": [
            { "code": "0", "target": "0" },
            { "code": "1", "target": "88" },
            { "code": "2", "target": "89" },
            { "code": "5", "target": "810" }
        ],
        "publisher_color": 7
    }
    "#;
    let meta: Meta = serde_json::from_str(meta_str).unwrap();
    let cept = Cept::from_ceptml(
        "<csr:2,1><pal:1><sbgs:4><g0:g0><left:g0><mode:p><pal:0><9e><n><r>\
        <lbgs:4><n>\
        <lbgs:4><pal:1><height:2><r>\
        Neuen Benutzer einrichten<n><r>\
        <pal:0><size:1><9e><fgs:7><r><n>\
        Teilnehmernummer:<csr:6,29>-1<r><n>\
        Anrede:<r><n>\
        Name:<r><n>\
        Vorname:<r><n>\
        Straße:<r><n>\
        PLZ: <rep:6>Ort: <rep:14>Land:<r><n>\
        <line>\
        Vergütungssperre aktiv:<r><n>\
        Gebührensperre   aktiv:<r><n>\
        Taschengeldkonto      :<csr:15,35>,   DM\
        Max. Vergütung/Seite  :<csr:16,35>,   DM\
        <line>\
        <r><n>\
        Kennwort:\
        <r><n><r><n>\
        <line>\
        "
    );
    hexdump::hexdump(cept.data());

    Page {
        meta,
        cept_palette: None,
        cept_include: None,
        cept
    }
}

fn callback_validate_user_id(_: &PageId, input_data: &HashMap<String, String>) -> ValidateResult {
    if User::exists(&UserId::new(input_data.get("user_id").unwrap(), "1")) {
        ValidateResult::Error(SysMsg::Custom("Teilnehmernummer bereits vergeben! -> #".to_string()))
    } else {
        ValidateResult::Ok
    }
}

fn callback_validate_last_name(_: &PageId, input_data: &HashMap<String, String>) -> ValidateResult {
    if input_data.get("last_name").unwrap() == "" {
        ValidateResult::Error(SysMsg::Custom("Name darf nicht leer sein! -> #".to_string()))
    } else {
        ValidateResult::Ok
    }
}

fn callback_validate_password(_: &PageId, input_data: &HashMap<String, String>) -> ValidateResult {
    if input_data.get("password").unwrap().len() < 4 {
        ValidateResult::Error(SysMsg::Custom("Kennwort muß mind. 4-stellig sein! -> #".to_string()))
    } else {
        ValidateResult::Ok
    }
}

