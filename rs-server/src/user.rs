use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};
use chrono::{Local, DateTime, TimeZone};
use super::staticp::*;
use super::paths::*;

#[derive(Serialize, Deserialize)]
pub struct UserId {
    pub id: String,
    pub ext: String,
}

impl UserId {
	pub fn new(id: &str, ext: &str) -> Self {
        let mut id = id.to_owned();
        let mut ext = ext.to_owned();
        if id == "" {
            id = "0".to_owned();
        }
		if ext == "" {
            ext = "1".to_owned();
        }
        Self { id, ext }
    }

    pub fn to_string(&self) -> String {
        let mut s = self.id.clone();
        s.push('-');
        s += &self.ext;
        s
    }
}

#[derive(Serialize, Deserialize)]
pub enum UserDataPublic {
    Person(UserDataPublicPerson),
    Organization(UserDataPublicOrganization),
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPublicPerson {
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPublicOrganization {
    pub name1: Option<String>,
    pub name2: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserDataPrivate {
	pub street: Option<String>,
	pub zip: Option<String>,
	pub city: Option<String>,
	pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub userid: UserId,
	pub public: UserDataPublic,
	pub private: UserDataPrivate,

	// messaging: None
}

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    password: String,
}

//XXX global_user = None

#[derive(Serialize, Deserialize)]
#[derive(Default)]
struct StatsData {
    last_use: Option<i64>,
}

pub struct Stats {
    filename: String,
    stats_data: StatsData,
}

fn filename(userid: &UserId, path: &str, file_extension: &str) -> String {
    let mut s = String::new();
    s += path;
    s += &userid.to_string();
    s.push('.');
    s += file_extension;
    s
}

impl Stats {
	pub fn new(user: &User) -> Self {
		let filename = filename(&user.userid, PATH_STATS, &"stats");
        if let Ok(f) = File::open(&filename) {
            let stats_data: Result<StatsData, _> = serde_json::from_reader(f);
            if let Ok(stats_data) = stats_data {
                return Stats {
                    filename,
                    stats_data,
                }
            }
        }
        Self {
            filename,
            stats_data: StatsData::default()
        }
    }

	pub fn update(&mut self) {
		// update the last use field with the current time
		self.stats_data.last_use = Some(Local::now().timestamp());
        let json_data = serde_json::to_string(&self.stats_data).unwrap();
        let mut file = File::create(&self.filename).unwrap();
        file.write_all(&json_data.as_bytes());
    }

    pub fn last_use(&self) -> Option<DateTime<Local>> {
        if let Some(last_use) = self.stats_data.last_use {
            Some(Local.timestamp(last_use, 0))
        } else {
            None
        }
    }
}

impl User {
    fn user_filename(userid: &UserId) -> String {
        filename(userid, PATH_USERS, "user")
    }

	fn secrets_filename(userid: &UserId) -> String {
        filename(userid, PATH_SECRETS, "secrets")
    }

    pub fn exists(userid: &UserId) -> bool {
		let filename = Self::user_filename(&userid);
        is_file(&filename)
    }

	fn get(userid: &UserId, personal_data: bool) -> Option<User> {
		let filename = Self::user_filename(&userid);
        let f = File::open(&filename).ok()?;
        let user: User = serde_json::from_reader(f).unwrap();
		// user.messaging = Messaging(user)
        Some(user)
    }

	pub fn create(
        id: &str,
        ext: &str,
        password: &str,
        salutation: &str,
        last_name: &str,
        first_name: &str,
        street: &str,
        zip: &str,
        city: &str,
        country: &str
    ) -> bool {
        let userid = UserId::new(id, ext);
		let user_filename = Self::user_filename(&userid);
		let secrets_filename = Self::secrets_filename(&userid);
		// if the user exists, don't overwrite it!
		if User::exists(&userid) {
			println!("user already exists!");
            return false;
        }
		let user = User {
            userid: userid,
            public: UserDataPublic::Person(UserDataPublicPerson {
                salutation: Some(salutation.to_owned()),
                first_name: Some(first_name.to_owned()),
                last_name: Some(last_name.to_owned()),
            }),
            private: UserDataPrivate {
                street: Some(street.to_owned()),
                zip: Some(zip.to_owned()),
                city: Some(city.to_owned()),
                country: Some(country.to_owned()),
            },
		};
        let json_data = serde_json::to_string(&user).unwrap();
        let mut file = File::create(user_filename).unwrap();
        file.write_all(&json_data.as_bytes());

		let secrets = Secrets {
			password: password.to_owned(),
		};
        let json_data = serde_json::to_string(&secrets).unwrap();
        let mut file = File::create(secrets_filename).unwrap();
        file.write_all(&json_data.as_bytes());

        true
    }

	pub fn login(userid: &UserId, password: &str) -> Option<Self> {
		let filename = Self::secrets_filename(userid);
        if let Ok(f) = File::open(&filename) {
            let secrets: Result<Secrets, _> = serde_json::from_reader(f);
            if let Ok(secrets) = secrets {
                if password == secrets.password {
                    return Self::get(&userid, true)
                }
            }
        }
        None
    }
}
