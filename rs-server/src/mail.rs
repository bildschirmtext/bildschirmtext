use serde::{Deserialize, Serialize};
use std::{fs::File, io::Write};
use chrono::{Local, Utc};
use chrono::TimeZone;
use uuid::Uuid;
use crate::paths::*;
use crate::user::*;

use super::staticp::*;

#[derive(Serialize, Deserialize)]
struct MessageDatabase {
    messages: Vec<Message>,
}

impl MessageDatabase {
    fn new() -> Self {
        Self {
            messages: vec!()
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
struct Message {
    body: String,
    from_userid: UserId,
    personal_data: bool,
    timestamp: i64,
    is_read: bool,
    uuid: Uuid,
}

impl Message {
	fn from_date(&self) -> String {
        let t = Local.timestamp(self.timestamp, 0);
        t.format("%d.%m.%Y").to_string()
    }

	fn from_time(&self) -> String {
        let t = Local.timestamp(self.timestamp, 0);
        t.format("%H:%M").to_string()
    }
}

struct Messaging {
	userid: UserId,
    database: MessageDatabase,
}


impl Messaging {
	fn new(userid: &UserId) -> Self {
        Self {
            userid: userid.clone(),
            database: MessageDatabase::new(),
        }
    }

	fn database_filename(&self) -> String {
        let mut s = String::new();
        s += PATH_MESSAGES;
        s += &self.userid.to_string();
        s += ".messages";
        s
    }

	fn load_database(&mut self) -> MessageDatabase {
		let filename = self.database_filename();
		if !is_file(&filename) {
			println!("messages file not found");
			MessageDatabase::new()
        } else {
            let f = File::open(&filename).unwrap();
            let database: MessageDatabase = serde_json::from_reader(f).unwrap();
            database
        }
    }

	fn save_database(&self) {
        let json_data = serde_json::to_string(&self.database).unwrap();
        let mut file = File::create(self.database_filename()).unwrap();
        file.write_all(&json_data.as_bytes());
    }

	fn load(&mut self) {
        self.database = self.load_database();
    }

	fn save(&mut self) {
        self.save_database();
    }

	fn select(&mut self, is_read: bool, start: usize, count: Option<usize>) -> Vec<&Message> {
		self.load();

		let mut ms = vec!();
		let mut j = 0;
		for i in (0..self.database.messages.len()).rev() {
			let m = &self.database.messages[i];
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

	fn mark_as_read(&mut self, index: usize) {
		self.load();
		if !self.database.messages[index].is_read {
			self.database.messages[index].is_read = true;
            self.save();
        }
    }

	fn has_new_messages(&mut self) {
		self.load();
        self.select(false, 0, None).len() != 0;
    }

	fn send(&mut self, user_id: &str, ext: &str, body: &str) {
		let mut database = self.load_database();
		database.messages.push(
            Message {
				from_userid: self.userid.clone(),
				personal_data: false,
				timestamp: Utc::now().timestamp(),
                body: body.to_owned(),
                is_read: false,
                uuid: Uuid::new_v4(),
			},
		);
        self.save_database();
    }
}

