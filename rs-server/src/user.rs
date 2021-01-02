use serde::{Deserialize, Serialize};

const PATH_USERS: &str = "../users/";
const PATH_SECRETS: &str = "../secrets/";
const PATH_STATS: &str = "../stats/";

pub struct User {
    pub user_id: String,
	pub ext: String,
	pub personal_data: bool,

	// public - person
	pub salutation: Option<String>,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	// public - organization
	pub org_name: Option<String>,
	pub org_add_name: Option<String>,
	// personal_data
	pub street: Option<String>,
	pub zip: Option<String>,
	pub city: Option<String>,
	pub country: Option<String>,

	// stats: None
	// messaging: None
}

impl User {
    pub fn get(user_id: &str, ext: &str, personal_data: bool) -> Option<User> {
        Some(User {
            user_id: "0".to_owned(),
            ext: "0".to_owned(),
            personal_data: false,

            salutation: None,
            first_name: None,
            last_name: None,
            org_name: None,
            org_add_name: None,

            street: None,
            zip: None,
            city: None,
            country: None,
        })
    }
}

//XXX global_user = None

#[derive(Serialize, Deserialize)]
struct StatsData {
    last_login: Some(timestamp),
}

struct Stats {
    filename: String,
    stats_data: StatsData,
}

impl Stats {
    fn filename(user: &User) -> String {
        let mut s = String::new();
        s += PATH_MESSAGES;
        s += user_id;
        s.push('-');
        s += ext;
        s += ".stats";
        s
    }

	pub fn new(user: &User) {
		let filename = self.filename(user);
        let f = File::open(&filename).unwrap();
        let stats_data: StatsData = serde_json::from_reader(f).unwrap();
        Stats {
            filename,
            stats_data,
        }
    }

	pub fn update(self) {
		// update the last use field with the current time
		let stats = Stats { last_use: Utc::now().timestamp() };
        let json_data = serde_json::to_string(stats).unwrap();
        let mut file = File::create(self.filename).unwrap();
        file.write_all(&json_data.as_bytes());
    }
}

impl User {
	// fn sanitize(user_id: &str, ext: &str) {
	// 	if user_id == "" {
    //         user_id = "0"
    //     }
	// 	if ext is None or ext == "":
	// 		ext = "1"
    //     return (user_id, ext)
    // }

	fn user_filename(user_id, ext):
		return PATH_USERS + user_id + "-" + ext + ".user"

	fn secrets_filename(user_id, ext):
		return PATH_SECRETS + user_id + "-" + ext + ".secrets"

	@classmethod
	fn exists(user_id, ext = "1"):
		(user_id, ext) = cls.sanitize(user_id, ext)
		filename = User.user_filename(user_id, ext)
		return os.path.isfile(filename)

	@classmethod
	fn get(user_id, ext, personal_data = False):
		(user_id, ext) = cls.sanitize(user_id, ext)
		from messaging import Messaging
		filename = User.user_filename(user_id, ext)
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			dict = json.load(f)

		user = cls()
		user.user_id = user_id
		user.ext = ext
		user.salutation = dict.get("salutation", "")
		user.first_name = dict.get("first_name", "")
		user.last_name = dict.get("last_name", "")
		user.org_name = dict.get("org_name", "")
		user.org_add_name = dict.get("org_add_name", "")

		user.personal_data = personal_data
		if (personal_data):
			user.street = dict.get("street", "")
			user.zip = dict.get("zip", "")
			user.city = dict.get("city", "")
			user.country = dict.get("country", "")
			user.stats = Stats(user)

		user.messaging = Messaging(user)

		return user

	@classmethod
	fn create(user_id, ext, password, salutation, last_name, first_name, street, zip, city, country):
		user_filename = User.user_filename(user_id, ext)
		secrets_filename = User.secrets_filename(user_id, ext)
		// if the user exists, don't overwrite it!
		if os.path.isfile(user_filename) or os.path.isfile(secrets_filename):
			sys.stderr.write("already exists: " + pprint.pformat(user_filename, secrets_filename) + "\n")
			return False
		user_dict = {
			"salutation": salutation,
			"first_name": first_name,
			"last_name": last_name,
			"street": street,
			"zip": zip,
			"city": city,
			"country": country
		}
		with open(user_filename, 'w') as f:
			json.dump(user_dict, f)
		secrets_dict = {
			"password": password
		}
		with open(secrets_filename, 'w') as f:
			json.dump(secrets_dict, f)
		return True

	@classmethod
	fn login(user_id, ext, password, force = False):
		global global_user

		(user_id, ext) = cls.sanitize(user_id, ext)
		filename = User.secrets_filename(user_id, ext)
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			dict = json.load(f)

		if password != dict.get("password") and not force:
			return None

		global_user = cls.get(user_id, ext, True)
		return True if global_user else False

class User_UI:
	fn line():
		data_cept = bytearray()
		data_cept.extend(Cept.set_left_g3())
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.set_left_g0())
		return data_cept

	fn create_title(title):
		data_cept = bytearray(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(
			b'\x1b\x28\x40'           // load G0 into G0
			b'\x0f'                   // G0 into left charset
		)
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.code_9e())
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.double_height())
		data_cept.extend(b'\r')
		data_cept.extend(Cept.from_str(title))
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.normal_size())
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_fg_color_simple(7))
		return data_cept

	fn create_title2(title):
		data_cept = bytearray(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(
			b'\x1b\x28\x40'           // load G0 into G0
			b'\x0f'                   // G0 into left charset
		)
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.double_height())
		data_cept.extend(b'\r')
		data_cept.extend(Cept.from_str(title))
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.normal_size())
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_fg_color_simple(7))
		return data_cept

	fn create_add_user():
		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"links": {
				"0": "0",
				"1": "88",
				"2": "89",
				"5": "810"
			},
			"inputs": {
				"fields": [
					{
						"name": "user_id",
						"hint": "Gewünschte Nummer oder # eingeben",
						"line": 6,
						"column": 19,
						"height": 1,
						"width": 10,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "number",
						"validate": "call:User_UI.callback_validate_user_id"
					},
					{
						"name": "salutation",
						"hint": "Anrede oder # eingeben",
						"line": 7,
						"column": 9,
						"height": 1,
						"width": 20,
						"bgcolor": 12,
						"fgcolor": 3
					},
					{
						"name": "last_name",
						"hint": "Nachnamen oder # eingeben",
						"line": 8,
						"column": 7,
						"height": 1,
						"width": 20,
						"bgcolor": 12,
						"validate": "call:User_UI.callback_validate_last_name",
						"fgcolor": 3
					},
					{
						"name": "first_name",
						"hint": "Vornamen oder # eingeben",
						"line": 9,
						"column": 10,
						"height": 1,
						"width": 20,
						"bgcolor": 12,
						"fgcolor": 3
					},
					{
						"name": "street",
						"hint": "Straße und Hausnummer oder # eingeben",
						"line": 10,
						"column": 9,
						"height": 1,
						"width": 20,
						"bgcolor": 12,
						"fgcolor": 3
					},
					{
						"name": "zip",
						"hint": "Postleitzahl oder # eingeben",
						"line": 11,
						"column": 6,
						"height": 1,
						"width": 5,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "number"
					},
					{
						"name": "city",
						"hint": "Ort oder # eingeben",
						"line": 11,
						"column": 17,
						"height": 1,
						"width": 13,
						"bgcolor": 12,
						"fgcolor": 3
					},
					{
						"name": "country",
						"hint": "Land oder # eingeben",
						"line": 11,
						"column": 37,
						"height": 1,
						"width": 2,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "de",
						"type": "alpha",
						"cursor_home": True,
						"overwrite": True
					},
					{
						"name": "block_payments",
						"hint": "j/n oder # eingeben",
						"line": 13,
						"column": 25,
						"height": 1,
						"width": 1,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "n",
						"cursor_home": True,
						"legal_values": [ "j", "n" ]
					},
					{
						"name": "block_fees",
						"hint": "j/n oder # eingeben",
						"line": 14,
						"column": 25,
						"height": 1,
						"width": 1,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "n",
						"cursor_home": True,
						"legal_values": [ "j", "n" ]
					},
					{
						"name": "pocket_money_major",
						"hint": "0-9 oder # eingeben",
						"line": 15,
						"column": 34,
						"height": 1,
						"width": 1,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "9",
						"type": "number",
						"cursor_home": True,
						"overwrite": True
					},
					{
						"name": "pocket_money_minor",
						"hint": "00-99 oder # eingeben",
						"line": 15,
						"column": 36,
						"height": 1,
						"width": 2,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "99",
						"type": "number",
						"cursor_home": True,
						"overwrite": True
					},
					{
						"name": "max_price_major",
						"hint": "0-9 oder # eingeben",
						"line": 16,
						"column": 34,
						"height": 1,
						"width": 1,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "9",
						"type": "number",
						"cursor_home": True,
						"overwrite": True
					},
					{
						"name": "max_price_minor",
						"hint": "00-99 oder # eingeben",
						"line": 16,
						"column": 36,
						"height": 1,
						"width": 2,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "99",
						"type": "number",
						"cursor_home": True,
						"overwrite": True
					},
					{
						"name": "password",
						"hint": "Neues Kennwort",
						"line": 19,
						"column": 11,
						"height": 1,
						"width": 14,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "password",
						"validate": "call:User_UI.callback_validate_password",
					},
				],
				"confirm": False,
				"target": "call:User_UI.callback_add_user",
			},
			"publisher_color": 7
		}

		data_cept = bytearray()
		data_cept.extend(User_UI.create_title("Neuen Benutzer einrichten"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Teilnehmernummer:"))
		data_cept.extend(Cept.set_cursor(6, 29))
		data_cept.extend(Cept.from_str("-1"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Anrede:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Name:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Vorname:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Straße:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("PLZ:"))
		data_cept.extend(Cept.repeat(" ", 7))
		data_cept.extend(Cept.from_str("Ort:"))
		data_cept.extend(Cept.set_cursor(11, 31))
		data_cept.extend(Cept.from_str("Land:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(User_UI.line())
		data_cept.extend(Cept.from_str("Vergütungssperre aktiv:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Gebührensperre   aktiv:"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Taschengeldkonto      :"))
		data_cept.extend(Cept.set_cursor(15, 35))
		data_cept.extend(Cept.from_str(",   DM"))
		data_cept.extend(Cept.from_str("Max. Vergütung/Seite  :"))
		data_cept.extend(Cept.set_cursor(16, 35))
		data_cept.extend(Cept.from_str(",   DM"))
		data_cept.extend(User_UI.line())
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Kennwort: "))
		data_cept.extend(b"\r\n\r\n")
		data_cept.extend(User_UI.line())
		return (meta, data_cept)

	fn callback_validate_user_id(input_data, dummy):
		if User.exists(input_data["user_id"]):
			msg = Util.create_custom_system_message("Teilnehmernummer bereits vergeben! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	fn callback_validate_last_name(input_data, dummy):
		if not input_data["last_name"]:
			msg = Util.create_custom_system_message("Name darf nicht leer sein! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	fn callback_validate_password(input_data, dummy):
		if len(input_data["password"]) < 4:
			msg = Util.create_custom_system_message("Kennwort muß mind. 4-stellig sein! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	fn callback_add_user(input_data, dummy):
		sys.stderr.write("input_data: " + pprint.pformat(input_data) + "\n")
		if User.create(
			input_data["user_id"],
			"1", # ext
			input_data["password"],
			input_data["salutation"],
			input_data["last_name"],
			input_data["first_name"],
			input_data["street"],
			input_data["zip"],
			input_data["city"],
			input_data["country"]
		):
			msg = Util.create_custom_system_message("Benutzer angelegt. Bitte neu anmelden. -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return "00000"
		else:
			msg = Util.create_custom_system_message("Benutzer konnte nicht angelegt werden. -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return "77"

	fn create_page(user, pagenumber):
		if pagenumber == "77a":
			return User_UI.create_add_user()
		else:
			return None














