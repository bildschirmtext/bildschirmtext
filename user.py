import os
import sys
import json
import time

from cept import Cept

PATH_USERS = "users/"
PATH_SECRETS = "secrets/"
PATH_STATS = "stats/"

# Currently, this only holds the last use
class Stats():
	last_login = None
	user = None

	def __filename(self):
		return PATH_STATS + self.user.user_id + "-" + self.user.ext + ".stats"

	def __init__(self, user):
		self.user = user
		filename = self.__filename()
		if os.path.isfile(filename):
			with open(filename) as f:
				stats = json.load(f)	
			self.last_login = stats.get("last_use")
	
	def update(self):
		# update the last use field with the current time
		stats = { "last_use": time.time() }
		with open(self.__filename(), 'w') as f:
			json.dump(stats, f)
	

class User():
	user_id = None
	ext = None
	personal_data = False

	# public - person
	salutation = None
	first_name = None
	last_name = None
	# public - organization
	org_name = None
	org_add_name = None
	# personal_data
	street = None
	zip = None
	city = None
	country = None

	stats = None
	messaging = None

	@classmethod
	def exists(cls, user_id, ext = "1"):
		filename = PATH_USERS + user_id + "-" + ext + ".user"
		return os.path.isfile(filename)
	
	@classmethod
	def get(cls, user_id, ext, personal_data = False):
		from messaging import Messaging
		filename = PATH_USERS + user_id + "-" + ext + ".user"
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
	def login(cls, user_id, ext, password, force = False):
		filename = PATH_SECRETS + user_id + "-" + ext + ".secrets"
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			dict = json.load(f)

		if password != dict.get("password") and not force:
			return None

		return cls.get(user_id, ext, True)

class User_UI:
	def line():
		data_cept = bytearray()
		data_cept.extend(Cept.set_left_g3())
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.set_left_g0())
		return data_cept

	def create_title(title):
		data_cept = bytearray(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(
			b'\x1b\x28\x40'           # load G0 into G0
			b'\x0f'                   # G0 into left charset
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

	def create_add_user():
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
						"width": 20,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "number"
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
						"width": 20,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "password"
					},
				],
				"confirm": False
#				"target": "page:000001a",
			},
			"publisher_color": 7
		}
		
		data_cept = bytearray()
		data_cept.extend(User_UI.create_title("Neuen Benutzer einrichten"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str("Teilnehmernummer:"))
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

	def create_page(user, pagenumber):
		if pagenumber == "77a":
			return User_UI.create_add_user()














