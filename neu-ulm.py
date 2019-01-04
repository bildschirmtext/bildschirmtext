# -*- coding: utf-8 -*-
import json
import sys
import os
import time
import datetime
from pprint import pprint

from cept import Cept

# paths
PATH_DATA = "data/"
PATH_USERS = "users/"
PATH_STATS = "stats/"
PATH_MESSAGES = "messages/"


# session info
session_user = "0"
session_ext = "1"
session_salutation = ""
session_first_name = ""
session_last_name = ""
session_last_date = "01.01.1970"
session_last_time = "0:00"
session_messages = []

# globals

last_filename_include = ""
last_filename_palette = ""
links = {}

def g2code(c, mode):
	if mode == 0:
		return b'\x19' + bytearray([ord(c)])
	else:
		return bytearray([ord(c) + 0x80])

def cept_from_unicode(s1, mode = 0):
	s2 = bytearray()
	for c in s1:
		# TODO: complete conversion!
		if ord(c) == 0xe4:
			s2.extend(g2code('H', mode) + b'a')           # &auml;
		elif ord(c) == 0xf6:
			s2.extend(g2code('H', mode) + b'o')           # &ouml;
		elif ord(c) == 0xfc:
			s2.extend(g2code('H', mode) + b'u')           # &uuml;
		elif ord(c) == 0xc4:
			s2.extend(g2code('H', mode) + b'A')           # &Auml;
		elif ord(c) == 0xd6:
			s2.extend(g2code('H', mode) + b'O')           # &Ouml;
		elif ord(c) == 0xdc:
			s2.extend(g2code('H', mode) + b'U')           # &Uuml;
		elif ord(c) == 0xdf:
			s2.extend(g2code('{', mode))                 # &szlig;
		else:
			s2.append(ord(c))
	return s2

def format_currency(price):
	return "DM  %d" % int(price / 100) + ",%02d" % int(price % 100)

def headerfooter(pagenumber, publisher_name, publisher_color):
	hide_header_footer = len(publisher_name) == 0
	hide_price = False
	# Early screenshots had a two-line publisher name with
	# the BTX logo in it for BTX-internal pages. Some .meta
	# files still reference this, but we should remove this.
	if publisher_name == "!BTX":
#		publisher_name = (
#			b'\x1b\x22\x41'                 # parallel mode
#			b'\x9b\x30\x40'                 # select palette #0
#			b'\x9e'                         # ???
#			b'\x87'                         # set fg color to #7
#			b'\x1b\x28\x20\x40'             # load DRCs into G0
#			b'\x0f'                         # G0 into left charset
#			b'\x21\x22\x23'                 # "!"#"
#			b'\n'
#			b'\r'
#			b'\x24\x25\x26'                 # "$%&"
#			b'\x0b'                         # cursor up
#			b'\x09'                         # cursor right
#			b'\x1b\x28\x40'                 # load G0 into G0
#			b'\x0f'                         # G0 into left charset
#			b'\n'
#			b'\x8d'                         # double height
#			# TODO: this does not draw!! :(
#			b'Bildschirmtext'
#		)
		publisher_name = "Bildschirmtext"
		hide_price = True
	else:
		publisher_name = publisher_name[:30]


	hf = bytearray(Cept.set_res_40_24())
	hf.extend(Cept.set_cursor(23, 1))
	hf.extend(Cept.unprotect_line())
	hf.extend(
		b'\x1b\x23\x21\x4c'                 # set fg color of line to 12
	)
	hf.extend(Cept.parallel_limited_mode())
	hf.extend(Cept.set_cursor(24, 1))
	hf.extend(Cept.unprotect_line())
	hf.extend(b' \b')
	hf.extend(Cept.clear_line())
	hf.extend(Cept.cursor_home())
	hf.extend(Cept.unprotect_line())
	hf.extend(b' \b')
	hf.extend(Cept.clear_line())
	hf.extend(Cept.serial_limited_mode())
	hf.extend(Cept.set_cursor(24, 1))
	hf.extend(Cept.set_fg_color(8))
	hf.extend(
		b'\b'
		b'\x9d'                             # ???
		b'\b'
	)

	if publisher_color < 8:
		color_string = bytearray(b'\x9b\x30\x40') + bytearray([0x80 + publisher_color])
	else:
		color_string = bytearray([0x80 + publisher_color - 8])

	hf.extend(color_string)

	hf.extend(Cept.set_cursor(24, 19))

	if not hide_header_footer:
		hf.extend(cept_from_unicode(pagenumber).rjust(22))

	hf.extend(Cept.cursor_home())
	hf.extend(Cept.set_palette(1))
	hf.extend(Cept.set_fg_color(8))
	hf.extend(
		b'\b'
		b'\x9d'                             # ???
		b'\b'
	)
	
	hf.extend(color_string)

	hf.extend(b'\r')

	hf.extend(cept_from_unicode(publisher_name))

	# TODO: price
	if not hide_header_footer and not hide_price:
		hf.extend(Cept.set_cursor(1, 31))
		hf.extend(b'  ')
		hf.extend(cept_from_unicode(format_currency(0)))

	hf.extend(Cept.cursor_home())
	hf.extend(Cept.set_palette(0))
	hf.extend(Cept.protect_line())
	hf.extend(b'\n')
	return hf



def create_system_message(code, price = 0, hint = ""):
	msg = ""
	if hint != "":
		msg = hint
	elif code == 0:
		msg = "                               "
	elif code == 44:
		msg = "Absenden? Ja:19 Nein:2         "
	elif code == 47:
		msg = "Absenden für " + format_currency(price) + "? Ja:19 Nein:2"
	elif code == 55:
		msg = "Eingabe wird bearbeitet        "
	elif code == 100:
		msg = "Seite nicht vorhanden          "
	elif code == 291:
		msg = "Seite wird aufgebaut           "
	elif code == 998:
		msg = "Ungültiger Teilnehmer oder Kennwort"

	msg = cept_from_unicode(msg, 1)

	msg = (
		b'\x1f\x2f\x40\x58'             # service break to row 24
		b'\x18'                         # clear line
	) + msg + (
		b'\x98'                         # hide
		b'\b'                           # cursor left
	)
	msg += b'SH'
	msg += cept_from_unicode(str(code)).rjust(3, b'0')
	msg += b'\x1f\x2f\x4f'              # service break back
	return msg

def create_preamble(basedir, meta):
	global last_filename_include
	global last_filename_palette

	preamble = b''

	# define palette
	if "palette" in meta:
		palette = meta["palette"]
		filename_palette = basedir + meta["palette"] + ".pal"
		sys.stderr.write("filename_palette = " + filename_palette + "\n")
		sys.stderr.write("last_filename_palette = " + last_filename_palette + "\n")
		if filename_palette != last_filename_palette:
			last_filename_palette = filename_palette
			with open(filename_palette) as f:
				palette = json.load(f)
			palette_data = Cept.define_palette(palette["palette"])
			preamble += palette_data
		else:
			sys.stderr.write("skipping palette\n")
	else:
		last_filename_palette = ""

	if "include" in meta:
		filename_include = basedir + meta["include"] + ".inc"
		if filename_include != last_filename_include:
			last_filename_include = filename_include
			with open(filename_include, mode='rb') as f:
				data_include = f.read()
			# palette definition has to end with 0x1f; add one if
			# the include data doesn't start with one
			if data_include[0] != 0x1f:
				preamble += Cept.set_cursor(1, 1)
			preamble += data_include
	else:
		last_filename_include = ""

	if len(preamble) > 600: # > 4 seconds @ 1200 baud
		preamble = create_system_message(291) + preamble

	return preamble

def replace_placeholders(cept):
	global session_salutation
	global session_first_name
	global session_last_name
	global session_last_date
	global session_last_time

	current_date = datetime.datetime.now().strftime("%d.%m.%Y")
	current_time = datetime.datetime.now().strftime("%H:%M")

	while True:
		found = False
		# TODO: convert into lookup table
		pos = cept.find(b'\x1f\x40\x41')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(current_date) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x42')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_salutation) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x43')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_first_name) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x44')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_last_name) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x45')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_last_date) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x46')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_last_time) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x47')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_user) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x48')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(session_ext) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x49')
		if pos > 0:
			cept = cept[:pos] + cept_from_unicode(current_time) + cept[pos+3:]
			found = True
				
		if not found:
			break

	return cept

def messaging_create_title(title):
	data_cept = bytearray(
		b'\x1f\x42\x41'           # set cursor to line 2, column 1
		b'\x9b\x31\x40'           # select palette #1
		b'\x1b\x23\x20\x54'       # set bg color of screen to 4
		b'\x1b\x28\x40'           # load G0 into G0
		b'\x0f'                   # G0 into left charset
	)
	data_cept.extend(Cept.parallel_mode())
	data_cept.extend(Cept.set_palette(0))
	data_cept.extend(Cept.code_9e())
	data_cept.extend(b'\n\r')
	data_cept.extend(
		b'\x1b\x23\x21\x54'       # set bg color of line to 4
	)
	data_cept.extend(b'\n')
	data_cept.extend(
		b'\x1b\x23\x21\x54'       # set bg color of line to 4
	)	
	data_cept.extend(Cept.set_palette(1))
	data_cept.extend(Cept.double_height())
	data_cept.extend(b'\r')
	data_cept += cept_from_unicode(title)
	data_cept += (b'\n\r')
	data_cept.extend(Cept.set_palette(0))
	data_cept.extend(Cept.normal_size())
	data_cept.extend(Cept.code_9e())
	data_cept.extend(Cept.set_fg_color_simple(7))
	return data_cept

def messaging_create_menu(title, items):
	data_cept = messaging_create_title(title)
	data_cept += (
		b"\n\r\n\r"
	)
	i = 1
	for item in items:
		data_cept += cept_from_unicode(str(i)) + b'  ' + cept_from_unicode(item)
		data_cept += b"\r\n\r\n"
		i +=1

	data_cept += (
		b'\r\n\r\n\r\n\r\n\r\n\r\n'
		b'\x1b\x23\x21\x54'                                     # set bg color of line to 4
		b'0\x19\x2b  Gesamt\x19Hubersicht'
	)

	return data_cept


def messages_load():
	global session_messages
	
	filename = PATH_MESSAGES + session_user + "-" + session_ext + ".messages"
	if not os.path.isfile(filename):
		messages = []
		sys.stderr.write("messages file not found\n")
	else:
		with open(filename) as f:
			session_messages = json.load(f)["messages"]
	
def message_get(index):
	messages_load()
	message = session_messages[index]

	t = datetime.datetime.fromtimestamp(message["timestamp"])
	from_date = t.strftime("%d.%m.%Y")
	from_time = t.strftime("%H:%M")
	from_user = message["from_user"]
	from_ext = message["from_ext"]
	from_org = message["from_org"]
	from_first = message["from_first"]
	from_last = message["from_last"]
	from_street = message["from_street"]
	from_city = message["from_city"]
	message_body = message["body"]
	return (
		from_user,
		from_ext,
		from_date,
		from_time,
		from_org,
		from_first,
		from_last,
		from_street,
		from_city,
		message_body
	)

def messaging_create_page(pagenumber):
	sys.stderr.write("pagenumber[:2] " + pagenumber[:2] + "\n")
	if pagenumber == "8a":
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
		
		data_cept = messaging_create_menu(
			"Mitteilungsdienst",
			[
				"Neue Mitteilungen",
				"Zur\x19Huckgelegte Mitteilungen",
				"Abruf Antwortseiten",
				"\x19HAndern Mitteilungsempfang",
				"Mitteilungen mit Alphatastatur"
			]
		)
	elif pagenumber == "88a":
		meta = {
			"publisher_name": "!BTX",
			"include": "a",
			"clear_screen": True,
			"publisher_color": 7
		}
		data_cept = messaging_create_title("Neue Mitteilungen")

		links = {
			"0": "8"
		}
		
		messages_load()
		
		for index in range(0, 9):
			#sys.stderr.write("message #" + str(index) + "/" + str(len(session_messages)) + "\n")
			data_cept += cept_from_unicode(str(index + 1)) + b'  '
			if len(session_messages) > index:
				message = session_messages[index]
				data_cept += cept_from_unicode(message["from_first"]) + b' ' + cept_from_unicode(message["from_last"])
				data_cept += b'\r\n   '
				t = datetime.datetime.fromtimestamp(message["timestamp"])
				data_cept += cept_from_unicode(t.strftime("%d.%m.%Y   %H:%M"))
				data_cept += b'\r\n'
				links[str(index + 1)] = "88" + str(index + 1)
			else:
				data_cept += b'\r\n\r\n'

		meta["links"] = links
	
	elif pagenumber[:2] == "88":
		index = int(pagenumber[2:-1]) - 1
		sys.stderr.write("pagenumber " + pagenumber + "\n")
		sys.stderr.write("index " + str(index) + "\n")
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

		(
			from_user,
			from_ext,
			from_date,
			from_time,
			from_org,
			from_first,
			from_last,
			from_street,
			from_city,
			message_body
		) = message_get(index)


		data_cept = bytearray(Cept.parallel_limited_mode())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_fg_color(3))
		data_cept.extend(b'von ')
		data_cept.extend(cept_from_unicode(from_user.ljust(12)) + b' ' + cept_from_unicode(from_ext.rjust(5, '0')))
		data_cept.extend(Cept.set_cursor(2, 41 - len(from_date)))
		data_cept.extend(cept_from_unicode(from_date))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(cept_from_unicode(from_org))
		data_cept.extend(Cept.set_cursor(3, 41 - len(from_time)))
		data_cept.extend(cept_from_unicode(from_time))
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(Cept.set_fg_color_simple(0))
		data_cept.extend(cept_from_unicode(from_first) + b' ' + cept_from_unicode(from_last))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(cept_from_unicode(from_street))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(cept_from_unicode(from_city))
		data_cept.extend(b'\r\n')
		data_cept.extend(b'an  ')
		data_cept.extend(cept_from_unicode(session_user.ljust(12)) + b' ' + cept_from_unicode(session_ext.rjust(5, '0')))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.repeat(" ", 4))
		data_cept.extend(cept_from_unicode(session_first_name) + b' ' + cept_from_unicode(session_last_name))
		data_cept.extend(b'\r\n\n')
		data_cept.extend(cept_from_unicode(message_body))
		data_cept.extend(Cept.set_cursor(23, 1))
		data_cept.extend(b'0')
		data_cept.extend(
			b'\x1b\x29\x20\x40'                                    # load DRCs into G1
			b'\x1b\x7e'                                            # G1 into right charset
		)
		data_cept.extend(cept_from_unicode(" Gesamtübersicht"))
		data_cept.extend(Cept.repeat(" ", 22))

	elif pagenumber == "810a":
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
		data_cept.extend(
			b'\x9b\x31\x40'                                    # select palette #1
			b'\x1b\x23\x20\x54'                                # set bg color of screen to 4
			b'\x1b\x28\x40'                                    # load G0 into G0
			b'\x0f'                                            # G0 into left charset
		)
		data_cept.extend(
			b'\x1b\x22\x41'                                    # parallel mode
		)
		data_cept.extend(
			b'\x9b\x30\x40'                                    # select palette #0
			b'\x9e'                                            # ???
			b'\n\r'
			b'\x1b\x23\x21\x54'                                # set bg color of line to 4
			b'\n'
			b'\x1b\x23\x21\x54'                                # set bg color of line to 4
			b'\x9b\x31\x40'                                    # select palette #1
			b'\x8d'                                            # double height
			b'\r'
		)
		data_cept.extend(cept_from_unicode("Mitteilungsdienst"))
		data_cept.extend(
			b'\n\r'
			b'\x9b\x30\x40'                                    # select palette #0
			b'\x8c'                                            # normal size
			b'\x9e'                                            # ???
			b'\x87'                                            # set fg color to #7
		)
		data_cept.extend(cept_from_unicode("Absender:"))

		data_cept.extend(cept_from_unicode(session_user))
		data_cept.extend(Cept.set_cursor(5, 25))
		data_cept.extend(cept_from_unicode(session_ext))
		data_cept.extend(Cept.set_cursor(6, 10))
		data_cept.extend(cept_from_unicode(session_first_name))
		data_cept.extend(Cept.set_cursor(7, 10))
		data_cept.extend(cept_from_unicode(session_last_name))
		data_cept.extend(Cept.set_cursor(5, 31))
		data_cept.extend(cept_from_unicode(current_date))
		data_cept.extend(Cept.set_cursor(6, 31))
		data_cept.extend(cept_from_unicode(current_time))
		data_cept.extend(b'\r\n\n')
		data_cept.extend(cept_from_unicode("Tln.-Nr. Empfänger:"))
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
			b'\x1b\x23\x21\x54'                                # set bg color of line to 4
		)
		data_cept.extend(
			b'0'
		)
		data_cept.extend(
			b'\x19'                                            # switch to G2 for one character
			b'\x2b\xfe\x7f'                                    # "+."
		)

	else:
		return None
	
	return (meta, data_cept)


def create_page(basepath, pagenumber):
	if pagenumber[-1:].isdigit():
		pagenumber += "a"

	basedir = None

	for i in reversed(range(0, len(pagenumber))):
		testdir = basepath + pagenumber[:i+1]
		if os.path.isdir(testdir):
			sys.stderr.write("testdir: '" + testdir + "'\n")
			filename = pagenumber[i+1:]
			sys.stderr.write("filename: '" + filename + "'\n")
			basedir = testdir + "/"
			break

	if basedir is None:
		return None

	# generated pages
	sys.stderr.write("pagenumber[0]: '" + pagenumber[0] + "'\n")
	if pagenumber[0] == '8':
		ret = messaging_create_page(pagenumber)
		if ret is None:
			return None
		(meta, data_cept) = ret
	else:
		filename_meta = basedir + filename + ".meta"
		filename_cept = basedir + filename + ".cept"

		if not os.path.isfile(filename_meta):
			return None

		with open(filename_meta) as f:
			meta = json.load(f)
		
		with open(filename_cept, mode='rb') as f:
			data_cept = f.read()
	
		data_cept = replace_placeholders(data_cept)

	with open(basedir + "a.glob") as f:
		glob = json.load(f)
	meta.update(glob) # combine dicts, glob overrides meta

	all_data = bytearray(Cept.hide_cursor())

	if "clear_screen" in meta and meta["clear_screen"]:
		all_data.extend(Cept.serial_limited_mode())
		all_data.extend(Cept.clear_screen())

	all_data.extend(create_preamble(basedir, meta))

	if "cls2" in meta and meta["cls2"]:
		all_data.extend(Cept.serial_limited_mode())
		all_data.extend(Cept.clear_screen())

	# header
	hf = headerfooter(pagenumber, meta["publisher_name"], meta["publisher_color"])
	all_data.extend(hf)

	# payload
	all_data.extend(data_cept)

	all_data.extend(Cept.serial_limited_mode())

	# footer
	all_data.extend(hf)

	all_data.extend(Cept.sequence_end_of_page())

	inputs = meta.get("inputs")
	return (all_data, meta["links"], inputs)


def read_with_echo(clear_line):
	c = sys.stdin.read(1)
	if clear_line:
		sys.stdout.write('\x18');
	if ord(c) == Cept.ini():
		sys.stdout.write('*')
	elif ord(c) == Cept.ter():
		sys.stdout.write('#')
	else:
		sys.stdout.write(c)
	sys.stdout.flush()
	sys.stderr.write("In: " + str(ord(c)) + "\n")
	return c

def stats_filename():
	global session_user
	global session_ext
	return PATH_STATS + session_user + "-" + session_ext + ".stats"

def stats_read():
	global session_last_date
	global session_last_time
	filename = stats_filename()
	if os.path.isfile(filename):
		with open(filename) as f:
			stats = json.load(f)	
		t = datetime.datetime.fromtimestamp(stats["last_login"])
		session_last_date = t.strftime("%d.%m.%Y")
		session_last_time = t.strftime("%H:%M")

def state_update():
	stats = { "last_login": time.time() }
	with open(stats_filename(), 'w') as f:
		json.dump(stats, f)

def login(input_data):
	global session_user
	global session_ext
	global session_salutation
	global session_first_name
	global session_last_name
	global session_last_date
	global session_last_time

	session_user = input_data["user"]
	session_ext = input_data["ext"]
	password = input_data["password"]
	if session_user == "":
		session_user = "0"
	if session_ext == "":
		session_ext = "1"
	filename = PATH_USERS + session_user + "-" + session_ext + ".user"
	if not os.path.isfile(filename):
		return False
	with open(filename) as f:
		user_data = json.load(f)
	session_salutation = user_data["salutation"]
	session_first_name = user_data["first_name"]
	session_last_name = user_data["last_name"]
	success = password == user_data["password"]
	if success:
		stats_read()
		
	return success

def handle_inputs(inputs):
	while True:
		cept_data = bytearray(Cept.parallel_limited_mode())
		for input in inputs["fields"]:
			l = input["line"]
			c = input["column"]
			h = input["height"]
			w = input["width"]
			for i in range(0, h):
				cept_data.extend(Cept.set_cursor(l + i, c))
				cept_data.extend(Cept.set_fg_color(input["fgcolor"]))
				cept_data.extend(Cept.set_bg_color(input["bgcolor"]))
				cept_data.extend(Cept.repeat(" ", w))
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
	
		input_data = {}
	
		for input in inputs["fields"]:
			l = input["line"]
			c = input["column"]
			h = input["height"]
			w = input["width"]

			if "hint" in input:
				hint = input["hint"]
			else:
				hint = ""
		
			cept_data  = create_system_message(0, 0, hint)
			cept_data += Cept.set_cursor(l, c)
			cept_data += Cept.set_fg_color(input["fgcolor"])
			cept_data += Cept.set_bg_color(input["bgcolor"])
			sys.stdout.buffer.write(cept_data)
			sys.stdout.flush()
		
			s = ""
			while True:
				c = sys.stdin.read(1)
				sys.stderr.write("Input In: " + str(ord(c)) + "\n")
				if ord(c) == Cept.ter():
					break
				if ord(c) == 8:
					if len(s) == 0:
						continue
					sys.stdout.buffer.write(b'\b \b')
					sys.stdout.flush()
					s = s[:-1]
				elif ord(c) < 0x20 and ord(c) != 0x19:
					continue
				elif len(s) < w:
					s += c
					sys.stdout.write(c)
					sys.stdout.flush()
				sys.stderr.write("String: '" + s + "'\n")
				
			input_data[input["name"]] = s
	
		if not "confirm" in inputs or inputs["confirm"]:
			if "price" in inputs:
				price = inputs["price"]
				cept_data  = create_system_message(47, price)
			else:
				price = 0
				cept_data  = create_system_message(44)
			cept_data += Cept.set_cursor(24, 1)
			sys.stdout.buffer.write(cept_data)
			sys.stdout.flush()
		
			seen_a_one = False
			while True:
				c = sys.stdin.read(1)
				if c == "2":
					doit = False
					sys.stdout.buffer.write(c)
					sys.stdout.flush()
					break
				elif c == "1" and not seen_a_one:
					seen_a_one = True
					sys.stdout.buffer.write(c)
					sys.stdout.flush()
				elif c == "9" and seen_a_one:
					doit = True
					sys.stdout.buffer.write(c)
					sys.stdout.flush()
					break
				elif ord(c) == 8 and seen_a_one:
					seen_a_one = False
					sys.stdout.buffer.write(b'\b \b')
					sys.stdout.flush()
				
		cept_data = create_system_message(55)
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
	
		# login page
		if "is_login" in inputs and inputs["is_login"]:
			if not login(input_data):
				sys.stderr.write("login incorrect\n")
				cept_data = create_system_message(998)
				sys.stdout.buffer.write(cept_data)
				sys.stdout.flush()
				continue
			else:
				sys.stderr.write("login ok\n")
		#else:
			# send "input_data" to "inputs["target"]"
			
		cept_data = create_system_message(0)
		cept_data += Cept.sequence_end_of_page()
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
		
		if inputs["target"][:5] == "page:":
			return inputs["target"][5:]
		else:
			return ""


def show_page(pagenumber):
	global links
	
	while True:
		sys.stderr.write("showing page: '" + pagenumber + "'\n")
		ret = create_page(PATH_DATA, pagenumber)

		if ret is None:
			cept_data = create_system_message(100) + Cept.sequence_end_of_page()
			sys.stdout.buffer.write(cept_data)
			sys.stdout.flush()
			showing_message = True
			sys.stderr.write("page not found\n")
			return False

		(cept_data, links, inputs) = ret
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
		
		if inputs is None:
			return True

		new_pagenumber = handle_inputs(inputs)
		if new_pagenumber != "*00":
			pagenumber = new_pagenumber
		

# MAIN

sys.stderr.write("Neu-Ulm running.\n")

if len(sys.argv) > 1 and sys.argv[1] == "c64":
	num_crs = 0
	while True:
		c = read_with_echo(False);
		if ord(c) == 13:
			num_crs += 1
			if num_crs == 4:
				break
			
new_pagenumber = "00000" # login page

show_page(new_pagenumber)

current_pagenumber = new_pagenumber

MODE_NONE = 0
MODE_INI  = 1

mode = MODE_NONE
new_pagenumber = ""
showing_message = False

while True:
	gotopage = False;
	c = read_with_echo(showing_message);
	showing_message = False
	if mode == MODE_NONE:
		lookuplink = False
		if ord(c) == Cept.ini():
			mode = MODE_INI
			new_pagenumber = ""
			sys.stderr.write("mode = MODE_INI\n")
		elif ord(c) == Cept.ter():
			if len(new_pagenumber) > 0:
				sys.stderr.write("error: TER not expected here!\n")
			else:
				new_pagenumber = '#'
				lookuplink = True
				sys.stderr.write("local link: -> '" + new_pagenumber + "'\n")
		elif (c >= '0' and c <= '9'):
			new_pagenumber += c
			lookuplink = True
			sys.stderr.write("local link: '" + c + "' -> '" + new_pagenumber + "'\n")

		if lookuplink:
			if new_pagenumber in links:
				new_pagenumber = links[new_pagenumber]
				sys.stderr.write("found: -> '" + new_pagenumber + "'\n")
				gotopage = True;
			elif new_pagenumber == '#':
				if current_pagenumber[-1:] >= 'a' and current_pagenumber[-1:] <= 'y':
					new_pagenumber = current_pagenumber[:-1] + chr(ord(current_pagenumber[-1:]) + 1)
				elif current_pagenumber[-1:] >= '0' and current_pagenumber[-1:] <= '9':
					new_pagenumber = current_pagenumber + "b"
				gotopage = True;
			sys.stderr.write("new_pagenumber: '" + new_pagenumber + "'\n")
				
	elif mode == MODE_INI:
		if ord(c) == Cept.ini():
			# '**' resets mode
			mode = MODE_NONE
			new_pagenumber = ""
			cept_data  = b'\x1f\x58\x41'
			cept_data += b'\x18'
			sys.stdout.buffer.write(cept_data)
			sys.stdout.flush()
			sys.stderr.write("mode = MODE_NONE\n")
		elif c >= '0' and c <= '9':
			new_pagenumber += c
			sys.stderr.write("global link: '" + c + "' -> '" + new_pagenumber + "'\n")
			if new_pagenumber == "00" or new_pagenumber == "09":
				# TODO: 09 is a *hard* refresh
				new_pagenumber = current_pagenumber
				gotopage = True;
				mode = MODE_NONE
			sys.stderr.write("mode = MODE_NONE\n")
		elif ord(c) == Cept.ter():
			if new_pagenumber == "":
				new_pagenumber = previous_pagenumber
			sys.stderr.write("TERM global link: '" + new_pagenumber + "'\n")
			gotopage = True;
			mode = MODE_NONE
			sys.stderr.write("mode = MODE_NONE\n")
		
	if gotopage:
		if show_page(new_pagenumber):
			previous_pagenumber = current_pagenumber
			current_pagenumber = new_pagenumber
		new_pagenumber = ""
		state_update()

