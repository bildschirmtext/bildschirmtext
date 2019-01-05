import sys
import os
import json
import datetime

from cept import Cept

PATH_MESSAGES = "messages/"

class Message:
	dict = None

	def from_date(self):
		t = datetime.datetime.fromtimestamp(self.dict["timestamp"])
		return t.strftime("%d.%m.%Y")

	def from_time(self):
		t = datetime.datetime.fromtimestamp(self.dict["timestamp"])
		return t.strftime("%H:%M")
		
	def from_first(self):
		return self.dict["from_first"]
	
	def from_last(self):
		return self.dict["from_last"]
	
	def from_org(self):
		return self.dict["from_org"]
	
	def from_street(self):
		return self.dict["from_street"]
	
	def from_city(self):
		return self.dict["from_city"]

	def from_user(self):
		return self.dict["from_user"]

	def from_ext(self):
		return self.dict["from_ext"]

	def body(self):
		return self.dict["body"]

class Messaging:
	user = None
	messages = None

	def __init__(self, u):
		self.user = u

	def load(self):
		filename = PATH_MESSAGES + self.user.user_id + "-" + self.user.ext + ".messages"
		if not os.path.isfile(filename):
			self.messages = []
			sys.stderr.write("messages file not found\n")
		else:
			with open(filename) as f:
				self.messages = json.load(f)["messages"]
		
	def get(self, index):
		self.load()

		if len(self.messages) <= index:
			return None

		message = Message()
		message.dict = self.messages[index]
		return message

class Messaging_UI():

	# private
	def messaging_create_title(title):
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
	
	# private
	def messaging_create_menu(title, items):
		data_cept = bytearray(Messaging_UI.messaging_create_title(title))
		data_cept.extend(b"\n\r\n\r")
		i = 1
		for item in items:
			data_cept.extend(Cept.from_str(str(i)) + b'  ' + Cept.from_str(item))
			data_cept.extend(b"\r\n\r\n")
			i +=1
	
		data_cept.extend(b'\r\n\r\n\r\n\r\n\r\n\r\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'0\x19\x2b')
		data_cept.extend(Cept.from_str(" Gesamtübersicht"))
	
		return data_cept
	
	def messaging_create_main_menu():
		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"links": {
				"0": "0",
				"1": "88",
				"5": "810"
			},
			"publisher_color": 7
		}
		
		data_cept = Messaging_UI.messaging_create_menu(
			"Mitteilungsdienst",
			[
				"Neue Mitteilungen",
				"Zur\x19Huckgelegte Mitteilungen",
				"Abruf Antwortseiten",
				"\x19HAndern Mitteilungsempfang",
				"Mitteilungen mit Alphatastatur"
			]
		)
		return (meta, data_cept)

	def messaging_create_list_new(user):
		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"publisher_color": 7
		}
		data_cept = bytearray(Messaging_UI.messaging_create_title("Neue Mitteilungen"))

		links = {
			"0": "8"
		}
		
		messaging = Messaging(user)
		
		for index in range(0, 9):
			data_cept.extend(Cept.from_str(str(index + 1)) + b'  ')
			message = messaging.get(index)
			if message is not None:
				data_cept.extend(Cept.from_str(message.from_first()) + b' ' + Cept.from_str(message.from_last()))
				data_cept.extend(b'\r\n   ')
				data_cept.extend(Cept.from_str(message.from_date()))
				data_cept.extend(b'   ')
				data_cept.extend(Cept.from_str(message.from_time()))
				data_cept.extend(b'\r\n')
				links[str(index + 1)] = "88" + str(index + 1)
			else:
				data_cept.extend(b'\r\n\r\n')

		meta["links"] = links
		return (meta, data_cept)

	def messaging_create_message_detail(user, index):
		meta = {
			"publisher_name": "Bildschirmtext",
			"include": "11a",
			"palette": "11a",
			"clear_screen": True,
			"links": {
				"0": "88",
			},
			"publisher_color": 7
		}

		messaging = Messaging(user)
		message = messaging.get(index)

		from_date = message.from_date()
		from_time = message.from_time()

		data_cept = bytearray(Cept.parallel_limited_mode())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_fg_color(3))
		data_cept.extend(b'von ')
		data_cept.extend(Cept.from_str(message.from_user().ljust(12)) + b' ' + Cept.from_str(message.from_ext().rjust(5, '0')))
		data_cept.extend(Cept.set_cursor(2, 41 - len(from_date)))
		data_cept.extend(Cept.from_str(from_date))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(message.from_org()))
		data_cept.extend(Cept.set_cursor(3, 41 - len(from_time)))
		data_cept.extend(Cept.from_str(from_time))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.set_fg_color_simple(0))
		data_cept.extend(Cept.from_str(message.from_first()) + b' ' + Cept.from_str(message.from_last()))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(message.from_street()))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(message.from_city()))
		data_cept.extend(b'\r\n')
		data_cept.extend(b'an  ')
		data_cept.extend(Cept.from_str(user.user_id.ljust(12)) + b' ' + Cept.from_str(user.ext.rjust(5, '0')))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(user.first_name) + b' ' + Cept.from_str(user.last_name))
		data_cept.extend(b'\r\n\n')
		data_cept.extend(Cept.from_str(message.body()))
		data_cept.extend(Cept.set_cursor(23, 1))
		data_cept.extend(b'0')
		data_cept.extend(
			b'\x1b\x29\x20\x40'                                    # load DRCs into G1
			b'\x1b\x7e'                                            # G1 into right charset
		)
		data_cept.extend(Cept.from_str(" Gesamtübersicht"))
		data_cept.extend(Cept.repeat(" ", 22))

		return (meta, data_cept)

	def messaging_create_compose(user):
		meta = {
			"include": "a",
			"clear_screen": True,
			"links": {
				"0": "8"
			},
			"publisher_color": 7,
			"inputs": {
				"fields": [
					{
						"name": "user",
						"line": 8,
						"column": 20,
						"height": 1,
						"width": 16,
						"bgcolor": 4,
						"fgcolor": 3
					},
					{
						"name": "ext",
						"line": 8,
						"column": 37,
						"height": 1,
						"width": 1,
						"bgcolor": 4,
						"fgcolor": 3
					},
					{
						"name": "body",
						"line": 12,
						"column": 1,
						"height": 10,
						"width": 40,
						"bgcolor": 4,
						"fgcolor": 3
					}
				],
				"price": 30,
				"target": "page:8"
			}
		}

		current_date = datetime.datetime.now().strftime("%d.%m.%Y")
		current_time = datetime.datetime.now().strftime("%H:%M")

		data_cept = bytearray(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(
			b'\x1b\x28\x40'                                    # load G0 into G0
		)
		data_cept.extend(
			b'\x0f'                                            # G0 into left charset
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
		data_cept.extend(Cept.from_str("Mitteilungsdienst"))
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.normal_size())
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(Cept.from_str("Absender:"))

		data_cept.extend(Cept.from_str(user.user_id))
		data_cept.extend(Cept.set_cursor(5, 25))
		data_cept.extend(Cept.from_str(user.ext))
		data_cept.extend(Cept.set_cursor(6, 10))
		data_cept.extend(Cept.from_str(user.first_name))
		data_cept.extend(Cept.set_cursor(7, 10))
		data_cept.extend(Cept.from_str(user.last_name))
		data_cept.extend(Cept.set_cursor(5, 31))
		data_cept.extend(Cept.from_str(current_date))
		data_cept.extend(Cept.set_cursor(6, 31))
		data_cept.extend(Cept.from_str(current_time))
		data_cept.extend(b'\r\n\n')
		data_cept.extend(Cept.from_str("Tln.-Nr. Empfänger:"))
		data_cept.extend(Cept.set_cursor(8, 36))
		data_cept.extend(
			b'-'
			b'\r\n\n\n'
		)
		data_cept.extend(
			b'Text:'
		)
		data_cept.extend(
			b'\r\n\n\n\n\n\n\n\n\n\n\n\n'
		)
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(
			b'0'
		)
		data_cept.extend(
			b'\x19'                                            # switch to G2 for one character
			b'\x2b\xfe\x7f'                                    # "+."
		)
		return (meta, data_cept)

	def messaging_create_page(user, pagenumber):
		sys.stderr.write("pagenumber[:2] " + pagenumber[:2] + "\n")
		if pagenumber == "8a":
			return Messaging_UI.messaging_create_main_menu()
		elif pagenumber == "88a":
			return Messaging_UI.messaging_create_list_new(user)
		elif pagenumber[:2] == "88":
			return Messaging_UI.messaging_create_message_detail(user, int(pagenumber[2:-1]) - 1)
		elif pagenumber == "810a":
			return Messaging_UI.messaging_create_compose(user)
		else:
			return None
