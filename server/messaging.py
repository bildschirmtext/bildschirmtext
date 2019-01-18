import sys
import os
import re
import json
import time
import datetime

from cept import Cept
from user import User
from util import Util

PATH_MESSAGES = "../messages/"

class Message:
	dict = None
	from_user = None
	index = None

	def __init__(self, dict, index):
		self.dict = dict
		self.index = index
		self.from_user = User.get(self.dict["from_user_id"], self.dict["from_ext"], self.dict.get("personal_data", False))
		if self.from_user is None:
			sys.stderr.write("from user not found!\n")

	def from_date(self):
		t = datetime.datetime.fromtimestamp(self.dict["timestamp"])
		return t.strftime("%d.%m.%Y")

	def from_time(self):
		t = datetime.datetime.fromtimestamp(self.dict["timestamp"])
		return t.strftime("%H:%M")
		
	def body(self):
		return self.dict["body"]

class Messaging:
	user = None
	dict = None # this are the exact contents from the JSON file on disk

	def __init__(self, u):
		self.user = u

	def dict_filename(user_id, ext):
		return PATH_MESSAGES + user_id + "-" + ext + ".messages"

	def load_dict(user_id, ext):
		filename = Messaging.dict_filename(user_id, ext)
		if not os.path.isfile(filename):
			sys.stderr.write("messages file not found\n")
			dict = { "messages": [] }
		else:
			with open(filename) as f:
				dict = json.load(f)
		return dict
	
	def save_dict(user_id, ext, dict):
		with open(Messaging.dict_filename(user_id, ext), 'w') as f:
			json.dump(dict, f)

	def load(self):
		self.dict = Messaging.load_dict(self.user.user_id, self.user.ext)
		
	def save(self):
		Messaging.save_dict(self.user.user_id, self.user.ext, self.dict)

	def select(self, is_read, start, count):
		self.load()

		ms = []
		j = 0
		for i in reversed(range(0, len(self.dict["messages"]))):
			m = self.dict["messages"][i]
			if m.get("read", False) == is_read:
				if j >= start and (True if count is None else j < start + count):
					ms.append(Message(m, i))
				j += 1

		return ms

	def mark_as_read(self, index):
		self.load()
		if not self.dict["messages"][index].get("read", False):
			self.dict["messages"][index]["read"] = True
			self.save()

	def has_new_messages(self):
		self.load()
		return len(self.select(False, 0, None)) > 0

	def send(self, user_id, ext, body):
		dict = Messaging.load_dict(user_id, ext)
		dict["messages"].append(
			{
				"from_user_id": self.user.user_id,
				"from_ext": self.user.ext,
				"personal_data": False,
				"timestamp": time.time(),
				"body": body
			},
		)
		Messaging.save_dict(user_id, ext, dict)


class Messaging_UI:

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
				"2": "89",
				"5": "810"
			},
			"publisher_color": 7
		}
		
		data_cept = Messaging_UI.messaging_create_menu(
			"Mitteilungsdienst",
			[
				"Neue Mitteilungen",
				"Zurückgelegte Mitteilungen",
				"Abruf Antwortseiten",
				"Ändern Mitteilungsempfang",
				"Mitteilungen mit Alphatastatur"
			]
		)
		return (meta, data_cept)

	def messaging_create_list(user, is_read):
		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"publisher_color": 7
		}
		if is_read:
			title = "Zurückgelegte Mitteilungen"
		else:
			title = "Neue Mitteilungen"
		data_cept = bytearray(Messaging_UI.messaging_create_title(title))

		links = {
			"0": "8"
		}

		target_prefix = "89" if is_read else "88"

		messages = user.messaging.select(is_read, 0, 9)

		for index in range(0, 9):
			data_cept.extend(Cept.from_str(str(index + 1)) + b'  ')
			if index < len(messages):
				message = messages[index]
				if message.from_user.org_name:
					data_cept.extend(Cept.from_str(message.from_user.org_name))
				else:
					data_cept.extend(Cept.from_str(message.from_user.first_name))
					data_cept.extend(b' ')
					data_cept.extend(Cept.from_str(message.from_user.last_name))
				data_cept.extend(b'\r\n   ')
				data_cept.extend(Cept.from_str(message.from_date()))
				data_cept.extend(b'   ')
				data_cept.extend(Cept.from_str(message.from_time()))
				data_cept.extend(b'\r\n')
				links[str(index + 1)] = target_prefix + str(index + 1)
			else:
				data_cept.extend(b'\r\n\r\n')

		meta["links"] = links
		return (meta, data_cept)

	def messaging_create_message_detail(user, index, is_read):
		messages = user.messaging.select(is_read, index, 1)
		if len(messages) == 0:
			return None

		message = messages[0]

		meta = {
			"publisher_name": "Bildschirmtext",
			"include": "11a",
			"palette": "11a",
			"clear_screen": True,
			"links": {
				"0": "89" if is_read else "88",
			},
			"publisher_color": 7
		}

		from_date = message.from_date()
		from_time = message.from_time()
		if message.from_user.personal_data:
			from_street = message.from_user.street
			from_zip = message.from_user.zip
			from_city = message.from_user.city
		else:
			from_street = ""
			from_zip = ""
			from_city = ""

		data_cept = bytearray(Cept.parallel_limited_mode())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_fg_color(3))
		data_cept.extend(b'von ')
		data_cept.extend(Cept.from_str(message.from_user.user_id.ljust(12)) + b' ' + Cept.from_str(message.from_user.ext.rjust(5, '0')))
		data_cept.extend(Cept.set_cursor(2, 41 - len(from_date)))
		data_cept.extend(Cept.from_str(from_date))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(message.from_user.org_name))
		data_cept.extend(Cept.set_cursor(3, 41 - len(from_time)))
		data_cept.extend(Cept.from_str(from_time))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.set_fg_color_simple(0))
		data_cept.extend(Cept.from_str(message.from_user.first_name) + b' ' + Cept.from_str(message.from_user.last_name))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(from_street))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.from_str(from_zip))
		data_cept.extend(b' ')
		data_cept.extend(Cept.from_str(from_city))
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

		user.messaging.mark_as_read(message.index)

		return (meta, data_cept)

	def callback_validate_user_id(cls, input_data, dummy):
		if User.exists(input_data["user_id"]):
			return Util.VALIDATE_INPUT_OK
		else:
			msg = Util.create_custom_system_message("Teilnehmerkennung ungültig! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD

	def callback_validate_ext(cls, input_data, dummy):
		if User.exists(input_data["user_id"], input_data["ext"]):
			return Util.VALIDATE_INPUT_OK
		else:
			msg = Util.create_custom_system_message("Mitbenutzernummer ungültig! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_RESTART

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
						"name": "user_id",
						"type": "user_id",
						"line": 8,
						"column": 20,
						"height": 1,
						"width": 16,
						"bgcolor": 4,
						"fgcolor": 3,
						"validate": "call:Messaging_UI.callback_validate_user_id"
					},
					{
						"name": "ext",
						"type": "ext",
						"line": 8,
						"column": 37,
						"height": 1,
						"width": 1,
						"bgcolor": 4,
						"fgcolor": 3,
						"default": "1",
						"validate": "call:Messaging_UI.callback_validate_ext"
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
				"action": "send_message",
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
		data_cept.extend(b'Text:')
		data_cept.extend(b'\r\n\n\n\n\n\n\n\n\n\n\n\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'0')
		data_cept.extend(
			b'\x19'                                            # switch to G2 for one character
			b'\x2b\xfe\x7f'                                    # "+."
		)
		return (meta, data_cept)

	def create_page(user, pagenumber):
		if pagenumber == "8a":
			return Messaging_UI.messaging_create_main_menu()
		elif pagenumber == "88a":
			return Messaging_UI.messaging_create_list(user, False)
		elif pagenumber == "89a":
			return Messaging_UI.messaging_create_list(user, True)
		elif re.search("^88\da$", pagenumber):
			return Messaging_UI.messaging_create_message_detail(user, int(pagenumber[2:-1]) - 1, False)
		elif re.search("^89\da$", pagenumber):
			return Messaging_UI.messaging_create_message_detail(user, int(pagenumber[2:-1]) - 1, True)
		elif pagenumber == "810a":
			return Messaging_UI.messaging_create_compose(user)
		else:
			return None
