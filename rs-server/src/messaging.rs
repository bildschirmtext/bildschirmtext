use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use chrono::{Local, Utc};
use chrono::TimeZone;
use uuid::Uuid;
use crate::paths::*;
use crate::user::*;

use super::staticp::*;

#[derive(Serialize, Deserialize)]
#[derive(Default)]
struct MessageDatabase {
    messages: Vec<Message>,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub body: String,
    pub from_address: String,
    pub from_name: String,
    pub timestamp: i64,
    #[serde(default)]
    pub is_read: bool,
    pub uuid: Uuid,
}

impl Message {
	pub fn from_date(&self) -> String {
        let t = Local.timestamp(self.timestamp, 0);
        t.format("%d.%m.%Y").to_string()
    }

	pub fn from_time(&self) -> String {
        let t = Local.timestamp(self.timestamp, 0);
        t.format("%H:%M").to_string()
    }
}

pub struct MessageBox {
	userid: UserId,
    database: MessageDatabase,
}

// XXX Of course this is full of race conditions! In all functions that update the
// XXX database, if a message comes in between self.database() and self.save(),
// XXX it will be lost.

impl MessageBox {
	pub fn for_userid(userid: &UserId) -> Self {
        Self {
            userid: userid.clone(),
            database: MessageDatabase::default(),
        }
    }

	fn database_filename(&self) -> String {
        let mut s = String::new();
        s += PATH_MESSAGES;
        s += &self.userid.to_string();
        s += ".messages";
        s
    }

	fn database(&mut self) -> &mut MessageDatabase {
		let filename = self.database_filename();
		if !is_file(&filename) {
			println!("messages file not found");
			self.database = MessageDatabase::default();
        } else {
            let f = File::open(&filename).unwrap();
            self.database = serde_json::from_reader(f).unwrap();
        }
        &mut self.database
    }

	fn save(&self) {
        let json_data = serde_json::to_string(&self.database).unwrap();
        let mut file = File::create(self.database_filename()).unwrap();
        file.write_all(&json_data.as_bytes());
    }

	pub fn select(&mut self, is_read: bool, start: usize, count: Option<usize>) -> Vec<&Message> {
        let database = self.database();

        let mut ms = vec!();
        let mut j = 0;
        for i in (0..database.messages.len()).rev() {
            let m = &database.messages[i];
            if m.is_read != is_read {
                continue;
            }
            if j < start {
                continue;
            }
            if let Some(count) = count {
                if j >= start + count {
                    continue;
                }
            }
            ms.push(m);
            j += 1;
        }

        return ms;
    }

	pub fn mark_as_read(&mut self, uuid: Uuid) {
        let database = self.database();
        for message in &mut database.messages {
            if message.uuid == uuid {
                if !message.is_read {
                    message.is_read = true;
                    break;
                }
            }
        }
        self.save();
    }

	pub fn has_new_messages(&mut self) -> bool {
        self.select(false, 0, None).len() != 0
    }
}

pub fn send_message(from_userid: &UserId, to_userid: &UserId, body: &str) {
    let mut to_message_box = MessageBox::for_userid(to_userid);
    let database = to_message_box.database();
    database.messages.push(
        Message {
            from_address: from_userid.to_string(),
            from_name: User::get(&from_userid).unwrap().name(),
            timestamp: Utc::now().timestamp(),
            body: body.to_owned(),
            is_read: false,
            uuid: Uuid::new_v4(),
        },
    );
    to_message_box.save();
}

