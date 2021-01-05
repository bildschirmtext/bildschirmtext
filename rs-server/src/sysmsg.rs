use std::io::{Read,Write};
use chrono::Local;
use super::editor::*;

use super::cept::*;
use super::page::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SysMsgCode {
    CantGoBack = 10,
    ConfirmSend = 44,
    ConfirmSendPay = 47,
    Processing = 55,
    Sent = 73,
    PageDoesNotExist = 100,
    SubPageDoesNotExist = 101,
    TransferringPage = 291,
}

#[derive(PartialEq)]
pub enum SysMsg {
    None,
    Code(SysMsgCode, Option<u32>),
    Custom(String),
}

impl SysMsg {
    pub fn new(code: SysMsgCode) -> Self {
        Self::Code(code, None)
    }
}

pub fn create_sysmsg(error: &SysMsg) -> Cept {
    let mut cept = Cept::new();
    cept.service_break(24);
    cept.clear_line();

    match error {
        SysMsg::None => {}
        SysMsg::Code(code, price) => {
            let mut text;
            let mut prefix = "SH";
            match code {
                SysMsgCode::CantGoBack => text = "Rückblättern nicht möglich".to_owned(),
                SysMsgCode::ConfirmSend => text = "Absenden? Ja:19 Nein:2".to_owned(),
                SysMsgCode::ConfirmSendPay => text = format!("Absenden für {}? Ja:19 Nein:2", format_currency(price.unwrap())),
                SysMsgCode::Processing => text = "Eingabe wird bearbeitet".to_owned(),
                SysMsgCode::Sent => {
                    let current_datetime = Local::now().format("%d.%m.%Y %H:%M").to_string();
                    text = format!("Abgesandt {}, -> #", current_datetime);
                    prefix = "1B";
                },
                SysMsgCode::PageDoesNotExist => text = "Seite nicht vorhanden".to_owned(),
                SysMsgCode::SubPageDoesNotExist => text = "Seite nicht vorhanden".to_owned(),
                SysMsgCode::TransferringPage => text = "Seite wird aufgebaut".to_owned(),
                _ => text = "".to_owned(),
            }
            while text.len() < 31 {
                text.push(' ');
            }
            cept.add_str_characterset(&text, Some(1));
            cept.hide_text();
            cept.add_raw(b"\x08");
            cept.add_str(prefix);
            cept.add_str(&format!("{:03}", *code as u32));
        },
        SysMsg::Custom(text) => {
            cept.add_str_characterset(text, Some(1));
        }
    }
    cept.service_break_back();
    cept
}

pub fn show_sysmsg(error: &SysMsg, stream: &mut (impl Write + Read)) {
    let mut cept = create_sysmsg(error);
    if *error != SysMsg::new(SysMsgCode::Processing) {
        // XXX test this somewhere else
        cept.sequence_end_of_page();
    }
    write_stream(stream, cept.data());
    if let SysMsg::Custom(_) = error {
        wait_for_ter(stream);
    }
}

