use std::{fs::File, io::Write};
use serde::{Deserialize, Serialize};
use chrono::{Local, DateTime, TimeZone};
use std::fmt;
use super::staticp::*;
use super::paths::*;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
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

impl fmt::Display for UserId {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.id)?;
        fmt.write_str("-")?;
        fmt.write_str(&self.ext)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub enum UserDataPublic {
    Person(UserDataPublicPerson),
    Organization(UserDataPublicOrganization),
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct UserDataPublicPerson {
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct UserDataPublicOrganization {
    pub name1: Option<String>,
    pub name2: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct UserDataPrivate {
	pub street: Option<String>,
	pub zip: Option<String>,
	pub city: Option<String>,
	pub country: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    pub userid: UserId,
	pub public: UserDataPublic,
	pub private: UserDataPrivate,
    #[serde(skip_serializing, skip_deserializing)]
    pub stats: Stats,
}

#[derive(Serialize, Deserialize)]
pub struct Secrets {
    password: String,
}

//XXX global_user = None

#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Clone)]
pub struct Stats {
    last_use: Option<i64>,
}

fn filename(userid: &UserId, path: &str, file_extension: &str) -> String {
    let mut s = String::new();
    s += path;
    s += &userid.to_string();
    s.push('.');
    s += file_extension;
    s
}

fn stats_filename(userid: &UserId) -> String {
    filename(userid, PATH_STATS, &"stats")
}

impl Stats {
	pub fn for_userid(userid: &UserId) -> Self {
		let filename = stats_filename(userid);
        if let Ok(f) = File::open(&filename) {
            let stats_data: Result<Stats, _> = serde_json::from_reader(f);
            if let Ok(stats_data) = stats_data {
                return stats_data;
            }
        }
        Stats::default()
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            userid: UserId::new("0", "0"),
            public: UserDataPublic::Person(
                UserDataPublicPerson {
                    salutation: None,
                    first_name: None,
                    last_name: Some("Gastbenutzer".to_owned()),
                }
            ),
            private: UserDataPrivate {
                street: None,
                zip: None,
                city: None,
                country: None,
            },
            stats: Stats::default(),
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

	pub fn get(userid: &UserId) -> Option<User> {
		let filename = Self::user_filename(&userid);
        let f = File::open(&filename).ok()?;
        serde_json::from_reader(f).ok()
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
        let stats = Stats::for_userid(&userid);
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
            stats: stats
		};
        let json_data = serde_json::to_string(&user).unwrap();
        if let Ok(mut file) = File::create(user_filename) {
            if let Ok(_) = file.write_all(&json_data.as_bytes()) {
                let secrets = Secrets {
                    password: password.to_owned(),
                };
                let json_data = serde_json::to_string(&secrets).unwrap();
                if let Ok(mut file) = File::create(secrets_filename) {
                    if let Ok(_) = file.write_all(&json_data.as_bytes()) {
                        return true;
                    }
                    println!("ERROR creating user! [1]");
                }
                println!("ERROR creating user! [2]");
            }
            println!("ERROR creating user! [3]");
        }

        false
    }

	pub fn login(userid: &UserId, password: &str) -> Option<Self> {
		let filename = Self::secrets_filename(userid);
        if let Ok(f) = File::open(&filename) {
            let secrets: Result<Secrets, _> = serde_json::from_reader(f);
            if let Ok(secrets) = secrets {
                if password == secrets.password {
                    return Self::get(&userid)
                }
            }
        }
        None
    }

    pub fn name(&self) -> String {
        match &self.public {
            UserDataPublic::Person(person) => {
                let mut name = String::new();
                if let Some(first_name) = &person.first_name {
                    name += &first_name;
                    name.push(' ');
                }
                if let Some(last_name) = &person.last_name {
                    name += last_name;
                }
                name
            },
            UserDataPublic::Organization(organization) => {
                if let Some(name1) = &organization.name1 {
                    name1.clone()
                } else {
                    String::new()
                }
            },
        }
    }

    pub fn is_someone(&self) -> bool {
        self.userid.id != "0"
    }

	pub fn update_stats(&mut self) {
		// update the last use field with the current time
		self.stats.last_use = Some(Local::now().timestamp());
        let json_data = serde_json::to_string(&self.stats).unwrap();
        if let Ok(mut file) = File::create(stats_filename(&self.userid)) {
            if let Ok(_) = file.write_all(&json_data.as_bytes()) {
                return;
            }
            println!("ERROR updating stats! [1]");
        }
        println!("ERROR updating stats! [2]");
    }

    pub fn last_use(&self) -> Option<DateTime<Local>> {
        if let Some(last_use) = self.stats.last_use {
            Some(Local.timestamp(last_use, 0))
        } else {
            None
        }
    }
}
