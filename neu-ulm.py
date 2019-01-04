# -*- coding: utf-8 -*-
import sys
import os
import json
import time
import datetime

from cept import Cept
from session import Session
from messaging import Messaging

# paths
PATH_DATA = "data/"

session = None

# globals

last_filename_include = ""
last_filename_palette = ""
links = {}

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
	hf.extend(Cept.set_line_fg_color_simple(12))
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
	hf.extend(b'\b')
	hf.extend(Cept.code_9d())
	hf.extend(b'\b')

	if publisher_color < 8:
		color_string = Cept.set_fg_color(publisher_color)
	else:
		color_string = Cept.set_fg_color_simple(publisher_color - 8)

	hf.extend(color_string)

	hf.extend(Cept.set_cursor(24, 19))

	if not hide_header_footer:
		hf.extend(Cept.from_str(pagenumber).rjust(22))

	hf.extend(Cept.cursor_home())
	hf.extend(Cept.set_palette(1))
	hf.extend(Cept.set_fg_color(8))
	hf.extend(b'\b')
	hf.extend(Cept.code_9d())
	hf.extend(b'\b')
	
	hf.extend(color_string)

	hf.extend(b'\r')

	hf.extend(Cept.from_str(publisher_name))

	# TODO: price
	if not hide_header_footer and not hide_price:
		hf.extend(Cept.set_cursor(1, 31))
		hf.extend(b'  ')
		hf.extend(Cept.from_str(format_currency(0)))

	hf.extend(Cept.cursor_home())
	hf.extend(Cept.set_palette(0))
	hf.extend(Cept.protect_line())
	hf.extend(b'\n')
	return hf

def create_system_message(code, price = 0, hint = ""):
	text = ""
	if hint != "":
		text = hint
	elif code == 0:
		text = "                               "
	elif code == 44:
		text = "Absenden? Ja:19 Nein:2         "
	elif code == 47:
		text = "Absenden für " + format_currency(price) + "? Ja:19 Nein:2"
	elif code == 55:
		text = "Eingabe wird bearbeitet        "
	elif code == 100:
		text = "Seite nicht vorhanden          "
	elif code == 291:
		text = "Seite wird aufgebaut           "
	elif code == 998:
		text = "Ungültiger Teilnehmer oder Kennwort"

	msg = bytearray(Cept.service_break(24))
	msg.extend(Cept.clear_line())
	msg.extend(Cept.from_str(text, 1))
	msg.extend(Cept.hide_text())
	msg.extend(b'\b')
	msg.extend(b'SH')
	msg.extend(Cept.from_str(str(code)).rjust(3, b'0'))
	msg.extend(Cept.service_break_back())
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
	global session

	current_date = datetime.datetime.now().strftime("%d.%m.%Y")
	current_time = datetime.datetime.now().strftime("%H:%M")

	while True:
		found = False
		# TODO: convert into lookup table
		pos = cept.find(b'\x1f\x40\x41')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(current_date) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x42')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(session.salutation) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x43')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(session.first_name) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x44')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(session.last_name) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x45')
		if pos > 0:
			t = datetime.datetime.fromtimestamp(session.stats.last_login)
			cept = cept[:pos] + Cept.from_str(t.strftime("%d.%m.%Y")) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x46')
		if pos > 0:
			t = datetime.datetime.fromtimestamp(session.stats.last_login)
			cept = cept[:pos] + Cept.from_str(t.strftime("%H:%M")) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x47')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(session.user) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x48')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(session.ext) + cept[pos+3:]
			found = True
	
		pos = cept.find(b'\x1f\x40\x49')
		if pos > 0:
			cept = cept[:pos] + Cept.from_str(current_time) + cept[pos+3:]
			found = True
				
		if not found:
			break

	return cept

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
		ret = Messaging.messaging_create_page(session, pagenumber)
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

def login(input_data):
	global session
	
	session = Session.login(input_data["user"], input_data["ext"], input_data["password"])
	
	return not session is None

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
		
			cept_data = bytearray(create_system_message(0, 0, hint))
			cept_data.extend(Cept.set_cursor(l, c))
			cept_data.extend(Cept.set_fg_color(input["fgcolor"]))
			cept_data.extend(Cept.set_bg_color(input["bgcolor"]))
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
				cept_data = bytearray(create_system_message(47, price))
			else:
				price = 0
				cept_data = bytearray(create_system_message(44))
			cept_data.extend(Cept.set_cursor(24, 1))
			sys.stdout.buffer.write(cept_data)
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
			
		cept_data = bytearray(create_system_message(0))
		cept_data.extend(Cept.sequence_end_of_page())
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
		
		if inputs["target"][:5] == "page:":
			return inputs["target"][5:]
		else:
			return ""


def show_page(pagenumber):
	global links
	
	while True:
		if session is not None:
			session.stats.update()

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
			cept_data = bytearray(Cept.set_cursor(24, 1))
			cept_data.extend(Cept.clear_line())
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

