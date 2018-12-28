import json
import sys
import os
from pprint import pprint

CEPT_INI = 19
CEPT_TER = 28

CEPT_END_OF_PAGE = (
	"\x1f\x58\x41"      # set cursor to line 24, column 1
	"\x11"              # show cursor
	"\x1a"              # end of page
)

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
			s2 += "\x19\x48u"           # &uuml;
		elif ord(c) == 0xd6:
			s2 += "\x19\x48O"           # &uuml;
		else:
			s2 += chr(ord(c))
	return s2

def headerfooter(pagenumber, meta):
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

	hf += encode_string(meta["publisher_name"][:30])

	hf += "\x1f\x41\x5f"                   # set cursor to line 1, column 31

	# TODO: price
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

def create_system_message(code):
	msg = (
		"\x1f\x2f\x40\x58"             # service break to row 24
		"\x18"                         # clear line
	)
	if code == 100:
		msg += "Seite nicht vorhanden          "
	elif code == 291:
		msg += "Seite wird aufgebaut           "
	elif code == 999:
		msg += "Absenden? Ja: 19 Nein: 2       "
	msg += (
		"\x98"                         # hide
		"\x08"                         # cursor left
	)
	msg += "SH" + str(code).rjust(3, '0')
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

def create_page(basepath, pagenumber):
	if pagenumber[-1:] >= '0' and pagenumber[-1:] <= '9':
		pagenumber += "a"

	basedir = ""

	for i in reversed(range(0, len(pagenumber))):
		testdir = basepath + pagenumber[0:i+1]
		if os.path.isdir(testdir):
			sys.stderr.write("testdir: '" + testdir + "'\n")
			filename = pagenumber[i+1:]
			if os.path.isfile(testdir + "/" + filename + ".meta"):
				sys.stderr.write("filename: '" + filename + "'\n")
				basedir = testdir + "/"
				break

	if basedir == "":
		return ("", {})

	with open(basedir + "a.glob") as f:
		glob = json.load(f)

	with open(basedir + filename + ".meta") as f:
		meta = json.load(f)

	meta.update(glob) # combine dicts, glob overrides meta

	filename_cept = basedir + filename + ".cept"
	with open(filename_cept, mode='rb') as f:
		data_cept = f.read()

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

def show_page(pagenumber):
	global links
	sys.stderr.write("showing page: '" + pagenumber + "'\n")
	(cept_data, new_links, inputs) = create_page("data/", pagenumber)
	if cept_data == "":
		sh100 = create_system_message(100)
		cept_data = sh100 + CEPT_END_OF_PAGE
		showing_message = True
		sys.stderr.write("page not found\n")
	else:
		links = new_links
	sys.stdout.write(cept_data)
	sys.stdout.flush()
	
	for input in inputs:
		l = input["line"]
		c = input["column"]
		h = input["height"]
		w = input["width"]

		cept_data = (
			"\x1f\x2f\x44"                     # parallel limited mode
			"\x90" # black background
		)
		for i in range(1, h):
			cept_data += "\x1f" + chr(0x40 + l + i) + chr(0x40 + c)      # set cursor
			cept_data += " \x12" + chr(0x40 + w)
			sys.stdout.write(cept_data)
			sys.stdout.flush()
	
		cept_data += "\x1f" + chr(0x40 + l) + chr(0x40 + c)      # set cursor
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
			
	cept_data = create_system_message(999)
	sys.stdout.write(cept_data)
	sys.stdout.flush()


# MAIN

sys.stderr.write("running!!\n")

num_crs = 0
while True:
	c = read_with_echo(False);
	if ord(c) == 13:
		num_crs += 1
#		sys.stderr.write("num_crs: " + num_crs + "\n")
		if num_crs == 4:
			break
			
show_page("0")

MODE_NONE = 0
MODE_INI  = 1

mode = MODE_NONE
pagenumber = ""
showing_message = False

while True:
	gotopage = False;
	c = read_with_echo(showing_message);
	showing_message = False
	if mode == MODE_NONE:
		lookuplink = False
		if ord(c) == CEPT_INI:
			mode = MODE_INI
			pagenumber = ""
			sys.stderr.write("mode = MODE_INI\n")
		elif ord(c) == CEPT_TER:
			if len(pagenumber) > 0:
				sys.stderr.write("error: TER not expected here!\n")
			else:
				pagenumber = '#'
				lookuplink = True
				sys.stderr.write("local link: -> '" + pagenumber + "'\n")
		elif (c >= '0' and c <= '9'):
			pagenumber += c
			lookuplink = True
			sys.stderr.write("local link: '" + c + "' -> '" + pagenumber + "'\n")

		if lookuplink:
			if pagenumber in links:
				pagenumber = links[pagenumber]
				sys.stderr.write("found: -> '" + pagenumber + "'\n")
				gotopage = True;
			else:
				if pagenumber == '#' and pagenumber[-1:] >= 'a' and pagenumber[-1:] <= 'y':
					pagenumber = pagenumber[:-1] + chr(ord(pagenumber[-1:]) + 1)
				
	elif mode == MODE_INI:
		if ord(c) == CEPT_INI:
			# '**' resets mode
			mode = MODE_NONE
			pagenumber = ""
			cept_data  = "\x1f\x58\x41"
			cept_data += "\x18"
			sys.stdout.write(cept_data)
			sys.stdout.flush()
			sys.stderr.write("mode = MODE_NONE\n")
		elif c >= '0' and c <= '9':
			pagenumber += c
			sys.stderr.write("global link: '" + c + "' -> '" + pagenumber + "'\n")
		elif ord(c) == CEPT_TER:
			sys.stderr.write("TERM global link: '" + pagenumber + "'\n")
			gotopage = True;
			mode = MODE_NONE
			sys.stderr.write("mode = MODE_NONE\n")
		
	if gotopage:
		show_page(pagenumber)
		pagenumber = ""

