use std::io::{Read,Write};
use chrono::Local;
use super::editor::*;

use super::cept::*;
use super::page::*;

#[derive(PartialEq, Clone, Copy)]
pub enum MsgCode {
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
pub enum Msg {
    None,
    Code(MsgCode, Option<u32>),
    Custom(String),
}

impl Msg {
    pub fn new(code: MsgCode) -> Self {
        Self::Code(code, None)
    }
}

pub fn create_system_message(error: &Msg) -> Cept {
    let mut msg = Cept::new();
    msg.service_break(24);
    msg.clear_line();

    match error {
        Msg::None => {
        }
        Msg::Code(code, price) => {
            let mut text;
            let mut prefix = "SH";
            match code {
                MsgCode::CantGoBack => text = "Rückblättern nicht möglich".to_owned(),
                MsgCode::ConfirmSend => text = "Absenden? Ja:19 Nein:2".to_owned(),
                MsgCode::ConfirmSendPay => text = format!("Absenden für {}? Ja:19 Nein:2", format_currency(price.unwrap())),
                MsgCode::Processing => text = "Eingabe wird bearbeitet".to_owned(),
                MsgCode::Sent => {
                    let current_datetime = Local::now().format("%d.%m.%Y %H:%M").to_string();
                    text = format!("Abgesandt {}, -> #", current_datetime);
                    prefix = "1B";
                },
                MsgCode::PageDoesNotExist => text = "Seite nicht vorhanden".to_owned(),
                MsgCode::SubPageDoesNotExist => text = "Seite nicht vorhanden".to_owned(),
                MsgCode::TransferringPage => text = "Seite wird aufgebaut".to_owned(),
                _ => text = "".to_owned(),
            }
            while text.len() < 31 {
                text.push(' ');
            }
            msg.add_str_characterset(&text, Some(1));
            msg.hide_text();
            msg.add_raw(b"\x08");
            msg.add_str(prefix);
            msg.add_str(&format!("{:03}", *code as u32));
        },
        Msg::Custom(text) => {
            msg.add_str_characterset(text, Some(1));
        }
    }
    msg.service_break_back();
    msg
}

pub fn show_msg(error: &Msg, stream: &mut (impl Write + Read)) {
    let mut cept = create_system_message(error);
    if *error != Msg::new(MsgCode::Processing) {
        // XXX test this somewhere else
        cept.sequence_end_of_page();
    }
    write_stream(stream, cept.data());
    if let Msg::Custom(_) = error {
        wait_for_ter(stream);
    }
}

