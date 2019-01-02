import json
import sys
import os
import time
import datetime
from pprint import pprint

# constants

CEPT_INI = 19
CEPT_TER = 28

CEPT_END_OF_PAGE = (
	"\x1f\x58\x41"      # set cursor to line 24, column 1
	"\x11"              # show cursor
	"\x1a"              # end of page
)

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

reload(sys)  
sys.setdefaultencoding('latin-1')

def hexdump(src, length=16, sep='.'):
	FILTER = ''.join([(len(repr(chr(x))) == 3) and chr(x) or sep for x in range(256)])
	lines = []
	for c in xrange(0, len(src), length):
		chars = src[c:c+length]
		hex = ' '.join(["%02x" % ord(x) for x in chars])
		if len(hex) > 24:
			hex = "%s %s" % (hex[:24], hex[24:])
		printable = ''.join(["%s" % ((ord(x) <= 127 and FILTER[ord(x)]) or sep) for x in chars])
		lines.append("%08x  %-*s  |%s|\n" % (c, length*3, hex, printable))
	print ''.join(lines)

def encode_string(s1):
	s2 = ""
	for c in s1:
		# TODO: complete conversion!
		if ord(c) == 0xfc:
			s2 += "\x19Hu"           # &uuml;
		elif ord(c) == 0xd6:
			s2 += "\x19HO"           # &Ouml;
		else:
			s2 += chr(ord(c))
	return s2

def headerfooter(pagenumber, meta):
	publisher_name = meta["publisher_name"]
	hide_header_footer = len(meta["publisher_name"]) == 0
	hide_price = False
	if publisher_name == "!BTX":
		publisher_name = (
			"\x1b\x22\x41"                 # parallel mode
			"\x9b\x30\x40"                 # select palette #0
			"\x9e"                         # ???
			"\x87"                         # set fg color to #7
			"\x1b\x28\x20\x40"             # load DRCs into G0
			"\x0f"                         # G0 into left charset
			"\x21\x22\x23"                 # "!"#"
			"\x0a"                         # cursor down
			"\x0d"                         # cursor to beginning of line
			"\x24\x25\x26"                 # "$%&"
			"\x0b"                         # cursor up
			"\x09"                         # cursor right
			"\x1b\x28\x40"                 # load G0 into G0
			"\x0f"                         # G0 into left charset
			"\x0a"                         # cursor down
			"\x8d"                         # double height
			# TODO: this does not draw!! :(
			"Bildschirmtext"
		)
		hide_price = True
	else:
		publisher_name = publisher_name[:30]


	hf = (
		"\x1f\x2d"                         # set resolution to 40x24
		"\x1f\x57\x41"                     # set cursor to line 23, column 1
		"\x9b\x31\x51"                     # unprotect line
		"\x1b\x23\x21\x4c"                 # set fg color of line to 12
		"\x1f\x2f\x44"                     # parallel limited mode
		"\x1f\x58\x41"                     # set cursor to line 24, column 1
		"\x9b\x31\x51"                     # unprotect line
		"\x20"                             # " "
		"\x08"                             # cursor left
		"\x18"                             # clear line
		"\x1e"                             # cursor home
		"\x9b\x31\x51"                     # unprotect line
		"\x20"                             # " "
		"\x08"                             # cursor left
		"\x18"                             # clear line
		"\x1f\x2f\x43"                     # serial limited mode
		"\x1f\x58\x41"                     # set cursor to line 24, column 1
		"\x9b\x31\x40"                     # select palette #1
		"\x80"                             # set fg color to #0
		"\x08"                             # cursor left
		"\x9d"                             # ???
		"\x08"                             # cursor left
	)
	
	publisher_color = meta["publisher_color"]

	if publisher_color < 8:
		color_string = "\x9b\x30\x40" + chr(0x80 + publisher_color)
	else:
		color_string = chr(0x80 + publisher_color - 8)

	hf += color_string

	hf += "\x1f\x58\x53"                   # set cursor to line 24, column 19

	if not hide_header_footer:
		hf += pagenumber.rjust(22)

	hf += (
		"\x1e"                             # cursor home
		"\x9b\x31\x40"                     # select palette #1
		"\x80"                             # set fg color to #0
		"\x08"                             # cursor left
		"\x9d"                             # ???
		"\x08"                             # cursor left
	)
	
	hf += color_string

	hf += "\x0d"                           # cursor to beginning of line

	hf += encode_string(publisher_name)

	# TODO: price
	if not hide_header_footer and not hide_price:
		hf += "\x1f\x41\x5f"                   # set cursor to line 1, column 31
		hf += "   0,00 DM"

	hf += (
		"\x1e"                             # cursor home
		"\x9b\x30\x40"                     # select palette #0
		"\x9b\x31\x50"                     # protect line
		"\x0a"                             # cursor down
	)
	return hf

def encode_palette(palette):
	palette_data = ""
	for hexcolor in palette:
		r = int(hexcolor[1:3], 16)
		g = int(hexcolor[3:5], 16)
		b = int(hexcolor[5:7], 16)
		r0 = (r >> 4) & 1
		r1 = (r >> 5) & 1
		r2 = (r >> 6) & 1
		r3 = (r >> 7) & 1
		g0 = (g >> 4) & 1
		g1 = (g >> 5) & 1
		g2 = (g >> 6) & 1
		g3 = (g >> 7) & 1
		b0 = (b >> 4) & 1
		b1 = (b >> 5) & 1
		b2 = (b >> 6) & 1
		b3 = (b >> 7) & 1
		byte0 = 0x40 | r3 << 5 | g3 << 4 | b3 << 3 | r2 << 2 | g2 << 1 | b2
		byte1 = 0x40 | r1 << 5 | g1 << 4 | b1 << 3 | r0 << 2 | g0 << 1 | b0
		palette_data += chr(byte0) + chr(byte1)
	return palette_data

def format_currency(price):
	return "DM  %d" % int(price / 100) + ".%02d" % int(price % 100)

def create_system_message(code, price = 0):
	msg = (
		"\x1f\x2f\x40\x58"             # service break to row 24
		"\x18"                         # clear line
	)
	if code == 0:
		msg += "                               "
	elif code == 44:
		msg += "Absenden? Ja:19 Nein:2         "
	elif code == 47:
		msg += "Absenden f\x19Hur " + format_currency(price) + "? Ja:19 Nein:2"
	elif code == 55:
		msg += "Eingabe wird bearbeitet        "
	elif code == 100:
		msg += "Seite nicht vorhanden          "
	elif code == 291:
		msg += "Seite wird aufgebaut           "
	elif code == 998:
		msg += "Ung\x19Hultiger Teilnehmer oder Kennwort"
	elif code == 999:
		msg += "# eingeben um fortzufahren     "
	msg += (
		"\x98"                         # hide
		"\x08"                         # cursor left
	)
	msg += "SH"
	msg += str(code).rjust(3, '0')
	msg += "\x1f\x2f\x4f"              # service break back
	return msg

def create_preamble(basedir, meta):
	global last_filename_include
	global last_filename_palette

	preamble = ""

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
			palette_data = encode_palette(palette["palette"])
			preamble += (
				"\x1f\x26\x20"           # start defining colors
				"\x1f\x26\x31\x36"       # define colors 16+
			)
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
			if ord(data_include[0]) != 0x1f:
				preamble += "\x1f\x41\x41"           # set cursor to x=1 y=1
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
		pos = cept.find("\x1f\x40\x41")
		if pos > 0:
			cept = cept[:pos] + current_date + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x42")
		if pos > 0:
			cept = cept[:pos] + session_salutation + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x43")
		if pos > 0:
			cept = cept[:pos] + session_first_name + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x44")
		if pos > 0:
			cept = cept[:pos] + session_last_name + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x45")
		if pos > 0:
			cept = cept[:pos] + session_last_date + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x46")
		if pos > 0:
			cept = cept[:pos] + session_last_time + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x47")
		if pos > 0:
			cept = cept[:pos] + session_user + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x48")
		if pos > 0:
			cept = cept[:pos] + session_ext + cept[pos+3:]
			found = True
	
		pos = cept.find("\x1f\x40\x49")
		if pos > 0:
			cept = cept[:pos] + current_time + cept[pos+3:]
			found = True
				
		if not found:
			break

	return cept

def messaging_create_title(title):
	data_cept = (
		"\x1f\x42\x41"           # set cursor to line 2, column 1
		"\x9b\x31\x40"           # select palette #1
		"\x1b\x23\x20\x54"       # set bg color of screen to 4
		"\x1b\x28\x40"           # load G0 into G0
		"\x0f"                   # G0 into left charset
		"\x1b\x22\x41"           # parallel mode
		"\x9b\x30\x40"           # select palette #0
		"\x9e"                   # ???
		"\x0a"                   # cursor down
		"\x0d"                   # cursor to beginning of line
		"\x1b\x23\x21\x54"       # set bg color of line to 4
		"\x0a"                   # cursor down
		"\x1b\x23\x21\x54"       # set bg color of line to 4
		"\x9b\x31\x40"           # select palette #1
		"\x8d"                   # double height
		"\x0d"                   # cursor to beginning of line
	)
	data_cept += title
	data_cept += (
		"\n\r"
		"\x9b\x30\x40"           # select palette #0
		"\x8c"                   # normal size
		"\x9e"                   # ???
		"\x87"                   # set fg color to #7
	)
	return data_cept

def messaging_create_menu(title, items):
	data_cept = messaging_create_title(title)
	data_cept += (
		"\n\r\n\r"
	)
	i = 1
	for item in items:
		data_cept += str(i) + "  " + item
		data_cept += "\r\n\r\n"
		i +=1

	data_cept += (
		"\r\n\r\n\r\n\r\n\r\n\r\n"
		"\x1b\x23\x21\x54"                                     # set bg color of line to 4
		"0\x19\x2b  Gesamt\x19Hubersicht"
	)

	return data_cept


def messages_load():
	global session_messages
	
	filename = "messages/" + session_user + "-" + session_ext + ".messages"
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
			data_cept += str(index + 1) + "  "
			if len(session_messages) > index:
				message = session_messages[index]
				data_cept += message["from_first"] + " " + message["from_last"]
				data_cept += "\r\n   "
				t = datetime.datetime.fromtimestamp(message["timestamp"])
				data_cept += t.strftime("%d.%m.%Y   %H:%M")
				data_cept += "\r\n"
				links[str(index + 1)] = "88" + str(index + 1)
			else:
				data_cept += "\r\n\r\n"

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

		data_cept = (
			"\x1f\x2f\x44"                                        # parallel limited mode
			"\x1f\x42\x41"                                        # set cursor to line 2, column 1
			"\x9b\x30\x40"                                        # select palette #0
			"\x83"                                                # set fg color to #3
		)
		data_cept += "von " + from_user.ljust(12) + " " + from_ext.rjust(5, '0')

		data_cept += "\x1f\x42" + chr(0x40 + 41 - len(from_date))
		data_cept += from_date

		data_cept += (
			" \x12\x43"
		)
		data_cept += from_org

		data_cept += "\x1f\x43" + chr(0x40 + 41 - len(from_time))
		data_cept += from_time

		data_cept += (
			" \x12\x43"
			"\x80"                                                # set fg color to #0
		)
		data_cept += from_first + " " + from_last
		data_cept += (
			"\r\n \x12\x43"
		)
		data_cept += from_street
		data_cept += (
			"\r\n \x12\x43"
		)
		data_cept += from_city
		data_cept += (
			"\r\n"
		)
		data_cept += "an  " + session_user.ljust(12) + " " + session_ext.rjust(5, '0')
		data_cept += (
			"\r\n \x12\x43"
		)
		data_cept += session_first_name + " " + session_last_name
		data_cept += (
			"\r\n\n"
		)
		data_cept += message_body
		data_cept += (
			"\x1f\x57\x41"
			"0"
			"\x1b\x29\x20\x40"                                    # load DRCs into G1
			"\x1b\x7e"                                            # G1 into right charset
			" Gesamt\x19Hubersicht"
			"\x20\x12\x56"                                        # repeat ' ' 22 times
		)

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

		data_cept = (
			"\x1f\x42\x41"                                    # set cursor to line 2, column 1
			"\x9b\x31\x40"                                    # select palette #1
			"\x1b\x23\x20\x54"                                # set bg color of screen to 4
			"\x1b\x28\x40"                                    # load G0 into G0
			"\x0f"                                            # G0 into left charset
			"\x1b\x22\x41"                                    # parallel mode
			"\x9b\x30\x40"                                    # select palette #0
			"\x9e"                                            # ???
			"\n"                                              # cursor down
			"\r"                                              # cursor to beginning of line
			"\x1b\x23\x21\x54"                                # set bg color of line to 4
			"\n"                                              # cursor down
			"\x1b\x23\x21\x54"                                # set bg color of line to 4
			"\x9b\x31\x40"                                    # select palette #1
			"\x8d"                                            # double height
			"\r"                                              # cursor to beginning of line
			"Mitteilungsdienst"
			"\n"                                              # cursor down
			"\r"                                              # cursor to beginning of line
			"\x9b\x30\x40"                                    # select palette #0
			"\x8c"                                            # normal size
			"\x9e"                                            # ???
			"\x87"                                            # set fg color to #7
			"Absender:"
		)
		data_cept += session_user
		data_cept += (
			"\x1f\x45\x59"                                    # set cursor to line 5, column 25
		)
		data_cept += session_ext
		data_cept += (
			"\x1f\x46\x4a"                                    # set cursor to line 6, column 10
		)
		data_cept += session_first_name
		data_cept += (
			"\x1f\x47\x4a"                                    # set cursor to line 7, column 10
		)
		data_cept += session_last_name
		data_cept += (
			"\x1f\x45\x5f"                                    # set cursor to line 5, column 31
		)
		data_cept += current_date
		data_cept += (
			"\x1f\x46\x5f"                                    # set cursor to line 6, column 31
		)
		data_cept += current_time
		data_cept += (
			"\r"                                              # cursor to beginning of line
			"\n"                                              # cursor down
			"\n"                                              # cursor down
			"Tln.-Nr. Empf\x19Hanger:"
			"\x1f\x48\x64"                                    # set cursor to line 8, column 36
			"\x2d"                                            # "-"
			"\r"                                              # cursor to beginning of line
			"\n\n\n"
			"Text:"
			"\r\n\n\n\n\n\n\n\n\n\n\n\n"
			"\x1b\x23\x21\x54"                                # set bg color of line to 4
			"0"
			"\x19"                                            # switch to G2 for one character
			"\x2b\xfe\x7f"                                    # "+."
		)

	else:
		return ({}, "")
	
	return (meta, data_cept)


def create_page(basepath, pagenumber):
	if pagenumber[-1:] >= '0' and pagenumber[-1:] <= '9':
		pagenumber += "a"

	basedir = ""

	for i in reversed(range(0, len(pagenumber))):
		testdir = basepath + pagenumber[0:i+1]
		if os.path.isdir(testdir):
			sys.stderr.write("testdir: '" + testdir + "'\n")
			filename = pagenumber[i+1:]
			sys.stderr.write("filename: '" + filename + "'\n")
			basedir = testdir + "/"
			break

	if basedir == "":
		return ("", {}, [])

	# generated pages
	sys.stderr.write("pagenumber[0]: '" + pagenumber[0] + "'\n")
	if pagenumber[0] == '8':
		(meta, data_cept) = messaging_create_page(pagenumber)
		if data_cept == "":
			return ("", {}, [])
	else:
		if not os.path.isfile(testdir + "/" + filename + ".meta"):
			return ("", {}, [])

		sys.stderr.write("reading: '" + basedir + filename + "'.meta\n")
		with open(basedir + filename + ".meta") as f:
			meta = json.load(f)
		
		filename_cept = basedir + filename + ".cept"
		with open(filename_cept, mode='rb') as f:
			data_cept = f.read()
	
		data_cept = replace_placeholders(data_cept)

	sys.stderr.write("reading: '" + basedir + "'.glob\n")
	with open(basedir + "a.glob") as f:
		glob = json.load(f)
	meta.update(glob) # combine dicts, glob overrides meta

	all_data = chr(0x14) # hide cursor

	if "clear_screen" in meta and meta["clear_screen"]:
		all_data += (
			"\x1f\x2f\x43"                 # serial limited mode
			"\x0c"                         # clear screen
		)

	all_data += create_preamble(basedir, meta)

	if "cls2" in meta and meta["cls2"]:
		all_data += (
			"\x1f\x2f\x43"                 # serial limited mode
			"\x0c"                         # clear screen
		)

	# header + footer
	all_data += headerfooter(pagenumber, meta)

#	# links
#	all_data += "\x1f\x3d\x30"
#	i = 0x31
#	for key, value in meta["links"].iteritems():
#		all_data +=	"\x1f\x3d"
#		all_data += chr(i)
#		all_data += key.encode('utf-8').ljust(2)
#		all_data += value.encode('utf-8')
#		i += 1

	# payload
	all_data += data_cept

	all_data += "\x1f\x2f\x43" # serial limited mode

	# header + footer
	all_data += headerfooter(pagenumber, meta)

	all_data += CEPT_END_OF_PAGE

	if "inputs" in meta:
		inputs = meta["inputs"]
	else:
		inputs = []
	return (all_data, meta["links"], inputs)


def read_with_echo(clear_line):
	c = sys.stdin.read(1)
	if clear_line:
		sys.stdout.write("\x18");
	if ord(c) == CEPT_INI:
		sys.stdout.write('*')
	elif ord(c) == CEPT_TER:
		sys.stdout.write('#')
	else:
		sys.stdout.write(c)
	sys.stdout.flush()
	sys.stderr.write("In: " + str(ord(c)) + "\n")
	return c

def set_fg_color(c):
	if c > 7:
		pal = 1
		c -= 8
	else:
		pal = 0
	return "\x9b" + chr(0x30 + pal) + "\x40" + chr(0x80 + c)

def set_bg_color(c):
	if c > 7:
		pal = 1
		c -= 8
	else:
		pal = 0
	return "\x9b" + chr(0x30 + pal) + "\x40" + chr(0x90 + c)

def update_stats():
	global session_user
	global session_ext
	filename = "stats/" + session_user + "-" + session_ext + ".stats"
	stats = { "last_login": time.time() }
	with open(filename, 'w') as f:
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
	filename = "users/" + session_user + "-" + session_ext + ".user"
	if not os.path.isfile(filename):
		return False
	with open(filename) as f:
		user_data = json.load(f)
	session_salutation = user_data["salutation"]
	session_first_name = user_data["first_name"]
	session_last_name = user_data["last_name"]
	success = password == user_data["password"]
	if success:
		filename = "stats/" + session_user + "-" + session_ext + ".stats"
		if os.path.isfile(filename):
			with open(filename) as f:
				stats = json.load(f)	
			t = datetime.datetime.fromtimestamp(stats["last_login"])
			session_last_date = t.strftime("%d.%m.%Y")
			session_last_time = t.strftime("%H:%M")
		
	return success

def handle_inputs(inputs):
	while True:
		cept_data = (
			"\x1f\x2f\x44"                     # parallel limited mode
		)
		for input in inputs["fields"]:
			l = input["line"]
			c = input["column"]
			h = input["height"]
			w = input["width"]
			for i in range(0, h):
				cept_data += "\x1f" + chr(0x40 + l + i) + chr(0x40 + c)      # set cursor
				cept_data += set_fg_color(input["fgcolor"])
				cept_data += set_bg_color(input["bgcolor"])
				cept_data += " \x12" + chr(0x40 + w - 1)
		sys.stdout.write(cept_data)
		sys.stdout.flush()
	
		input_data = {}
	
		for input in inputs["fields"]:
			l = input["line"]
			c = input["column"]
			h = input["height"]
			w = input["width"]
		
			cept_data  = create_system_message(999)
			cept_data += "\x1f" + chr(0x40 + l) + chr(0x40 + c)      # set cursor
			cept_data += set_fg_color(input["fgcolor"])
			cept_data += set_bg_color(input["bgcolor"])
			sys.stdout.write(cept_data)
			sys.stdout.flush()
		
			s = ""
			while True:
				c = sys.stdin.read(1)
				sys.stderr.write("Input In: " + str(ord(c)) + "\n")
				if ord(c) == CEPT_TER:
					break
				if ord(c) == 8:
					if len(s) == 0:
						continue
					sys.stdout.write("\x08 \x08")
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
			cept_data += "\x1f" + chr(0x40 + 24) + chr(0x40 + 24)      # set cursor
			sys.stdout.write(cept_data)
			sys.stdout.flush()
		
			seen_a_one = False
			while True:
				c = sys.stdin.read(1)
				if c == "2":
					doit = False
					sys.stdout.write(c)
					sys.stdout.flush()
					break
				elif c == "1" and not seen_a_one:
					seen_a_one = True
					sys.stdout.write(c)
					sys.stdout.flush()
				elif c == "9" and seen_a_one:
					doit = True
					sys.stdout.write(c)
					sys.stdout.flush()
					break
				elif ord(c) == 8 and seen_a_one:
					seen_a_one = False
					sys.stdout.write("\x08 \x08")
					sys.stdout.flush()
				
		cept_data = create_system_message(55)
		sys.stdout.write(cept_data)
		sys.stdout.flush()
	
		# login page
		if "is_login" in inputs and inputs["is_login"]:
			if not login(input_data):
				sys.stderr.write("login incorrect\n")
				cept_data = create_system_message(998)
				sys.stdout.write(cept_data)
				sys.stdout.flush()
				continue
			else:
				sys.stderr.write("login ok\n")
		#else:
			# send "input_data" to "inputs["target"]"
			
		cept_data = create_system_message(0)
		cept_data += CEPT_END_OF_PAGE
		sys.stdout.write(cept_data)
		sys.stdout.flush()
		
		if inputs["target"][:5] == "page:":
			return inputs["target"][5:]
		else:
			return ""


def show_page(pagenumber):
	global links
	
	success = True
	while True:
		sys.stderr.write("showing page: '" + pagenumber + "'\n")
		(cept_data, new_links, inputs) = create_page("data/", pagenumber)
		if cept_data == "":
			sh100 = create_system_message(100)
			cept_data = sh100 + CEPT_END_OF_PAGE
			showing_message = True
			success = False
			sys.stderr.write("page not found\n")
		else:
			links = new_links
		sys.stdout.write(cept_data)
		sys.stdout.flush()
		
		if len(inputs):
			new_pagenumber = handle_inputs(inputs)
			if new_pagenumber != "*00":
				pagenumber = new_pagenumber
		else:
			pagenumber = ""
			
		if len(pagenumber) == 0:
			break

	return success
		

# MAIN

sys.stderr.write("running!!\n")

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
		if ord(c) == CEPT_INI:
			mode = MODE_INI
			new_pagenumber = ""
			sys.stderr.write("mode = MODE_INI\n")
		elif ord(c) == CEPT_TER:
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
		if ord(c) == CEPT_INI:
			# '**' resets mode
			mode = MODE_NONE
			new_pagenumber = ""
			cept_data  = "\x1f\x58\x41"
			cept_data += "\x18"
			sys.stdout.write(cept_data)
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
		elif ord(c) == CEPT_TER:
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
		update_stats()

