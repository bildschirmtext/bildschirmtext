import os
import sys
import json
import time
import re
import pprint

from cept import Cept
from util import Util

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
	def sanitize(cls, user_id, ext):
		if user_id is None or user_id == "":
			user_id = "0"
		if ext is None or ext == "":
			ext = "1"
		return (user_id, ext)

	def user_filename(user_id, ext):
		return PATH_USERS + user_id + "-" + ext + ".user"

	def secrets_filename(user_id, ext):
		return PATH_SECRETS + user_id + "-" + ext + ".secrets"

	@classmethod
	def exists(cls, user_id, ext = "1"):
		(user_id, ext) = cls.sanitize(user_id, ext)
		filename = User.user_filename(user_id, ext)
		return os.path.isfile(filename)
	
	@classmethod
	def get(cls, user_id, ext, personal_data = False):
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
	def create(cls, user_id, ext, password, salutation, last_name, first_name, street, zip, city, country):
		user_filename = User.user_filename(user_id, ext)
		secrets_filename = User.secrets_filename(user_id, ext)
		# if the user exists, don't overwrite it!
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
	def login(cls, user_id, ext, password, force = False):
		(user_id, ext) = cls.sanitize(user_id, ext)
		filename = User.secrets_filename(user_id, ext)
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

	def create_title2(title):
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
						"width": 10,
						"bgcolor": 12,
						"fgcolor": 3,
						"type": "number",
						"validate": "call:User_UI.validate_user_id"
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
						"validate": "call:User_UI.validate_last_name",
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
						"validate": "call:User_UI.validate_password",
					},
				],
				"confirm": False,
				"target": "call:User_UI.add_user_callback",
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

	def validate_user_id(cls, input_data):
		if User.exists(input_data["user_id"]):
			msg = Util.create_custom_system_message("Teilnehmernummer bereits vergeben! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	def validate_last_name(cls, input_data):
		if not input_data["last_name"]:
			msg = Util.create_custom_system_message("Name darf nicht leer sein! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	def validate_password(cls, input_data):
		if len(input_data["password"]) < 4:
			msg = Util.create_custom_system_message("Kennwort muß mind. 4-stellig sein! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	def add_user_callback(cls, input_data):
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

	def historic_link_from_str(s):
		return s.replace("/", "")

	def historic_pretty_link_from_str(s):
		s = "*" + s.split("/")[0] + "#"
		if len(s) >= 8:
			return s + " "
		else:
			return (s + " " * 5)[:8]

	def historic_line(page, index):
		link = User_UI.historic_pretty_link_from_str(page[0])
		data_cept = bytearray()
		data_cept.extend(Cept.from_str(link))
		data_cept.extend(Cept.from_str((page[1] + "." * 29)[:38 - len(link)]))
		data_cept.extend(Cept.from_str(str(index)))
		return data_cept

	def create_historic_overview(index):
		name = "Amiga Demo"
		description = (
			"Der Amiga BTX Software-Decoder wurde mit"
			"Dumps von 113 BTX-Seiten aus 32\n"
			"Programmen ausgeliefert, sowie 56 eigens"
			"gestalteten Seiten zum Thema BTX.\n"
			"Die Seiten stammen vom April 1989."
		)
		distribution = [ 9, 17 ]

		start_page = [ "25096/0", "Amiga Demo Startseite" ]

		pages = [
			[ "1050", "Btx-Telex" ],
			[ "1188", "Teleauskunft" ],
			[ "1692", "Cityruf" ],
			[ "20000", "Deutsche Bundespost" ],
			[ "20096", "Commodore" ],
			[ "20511/223", "Kölner Stadtanzeiger" ],
			[ "21212", "Verbraucher-Zentrale NRW" ],
			[ "25800/0000", "Deutsche Bundesbahn" ],
			[ "30003", "Formel Eins" ],
			[ "30711", "Btx Südwest Datenbank GmbH" ],
			[ "33033", "Eden" ],
			[ "34034", "Frankfurter Allg. Zeitung" ],
			[ "34344", "Neue Mediengesellschaft Ulm" ],
			[ "35853", "ABIDA GmbH" ],
			[ "40040/200", "Axel Springer Verlag" ],
			[ "44479", "DIMDI" ],
			[ "50257", "Computerwelt Btx-Info-Dienst" ],
			[ "54004/04", "ÖVA Versicherungen" ],
			[ "57575", "Lotto Toto" ],
			[ "64064", "Markt & Technik" ],
			[ "65432/0", "ADAC" ],
			[ "67007", "Rheinpfalz Verlag/Druckerei" ],
			[ "201474/75", "Rhein-Neckar-Zeitung" ],
#			[ "208585", "eba Pressebüro und Verlag [BROKEN]" ],
			[ "208888", "Neue Mediengesellschaft Ulm" ],
			[ "402060", "AUTO & BTX WOLFSBURG" ],
			[ "50707545", "CHIP Magazin" ],
			[ "86553222", "Chaos Computer Club" ],
			[ "505050035", "Steinfels Sprachreisen" ],
			[ "920492040092", "Wolfgang Fritsch (BHP)" ]
		]

		links = { "10": User_UI.historic_link_from_str(start_page[0])}
		i = 10
		for page in pages:
			links[str(i)] = User_UI.historic_link_from_str(page[0])
			i += 1

		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"links": links,
			"publisher_color": 7
		}
		sys.stderr.write("meta: " + pprint.pformat(meta) + "\n")
		
		data_cept = bytearray()
		data_cept.extend(User_UI.create_title2("Historische Seiten: " + name))
		data_cept.extend(b"\r\n")

		if not index:
			data_cept.extend(Cept.from_str(description))
			data_cept.extend(b"\r\n\n")
			data_cept.extend(User_UI.historic_line(start_page, 10))
			data_cept.extend(b"\n")

		start_with = 0
		if index:
			for i in range(0, index):
				start_with += distribution[i]

		if index >= len(distribution):
			end = len(pages)
		else:
			end = start_with + distribution[index]
		for i in range(start_with, end):
			data_cept.extend(User_UI.historic_line(pages[i], i + 20))

		data_cept.extend(Cept.set_cursor(23, 1))
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.from_str("0 Zurück"))
		
		if index < len(distribution):
			data_cept.extend(Cept.set_cursor(23, 33))
			data_cept.extend(Cept.from_str("Weiter #"))

		return (meta, Cept.compress(data_cept))

	def create_page(user, pagenumber):
		if pagenumber == "77a":
			return User_UI.create_add_user()
		else:
			return None














