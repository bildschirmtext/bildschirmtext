import json
import sys
from pprint import pprint

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


def headerfooter(pagenumber, glob, meta):
	hf = ""
	hf += "\x1f\x2d"                         # set resolution to 40x24
	hf += "\x1f\x57\x41"                     # set cursor to line 23, column 1
	hf += "\x9b\x31\x51"                     # unprotect line
	hf += "\x1b\x23\x21\x4c"                 # set fg color of line to 12
	hf += "\x1f\x2f\x44"                     # parallel limited mode
	hf += "\x1f\x58\x41"                     # set cursor to line 24, column 1
	hf += "\x9b\x31\x51"                     # unprotect line
	hf += "\x20"                             # " "
	hf += "\x08"                             # cursor left
	hf += "\x18"                             # clear line
	hf += "\x1e"                             # cursor home
	hf += "\x9b\x31\x51"                     # unprotect line
	hf += "\x20"                             # " "
	hf += "\x08"                             # cursor left
	hf += "\x18"                             # clear line
	hf += "\x1f\x2f\x43"                     # serial limited mode
	hf += "\x1f\x58\x41"                     # set cursor to line 24, column 1
	hf += "\x9b\x31\x40"                     # select palette #1
	hf += "\x80"                             # set fg color to #0
	hf += "\x08"                             # cursor left
	hf += "\x9d"                             # ???
	hf += "\x08"                             # cursor left
		
	publisher_color = meta["publisher_color"]
	
	hf += chr(0x80 + publisher_color)
	
	hf += "\x1f\x58\x53"                     # set cursor to line 24, column 19
	
	hf += pagenumber.rjust(22)
	
	hf += "\x1e"                             # cursor home
	hf += "\x9b\x31\x40"                     # select palette #1
	hf += "\x80"                             # set fg color to #0
	hf += "\x08"                             # cursor left
	hf += "\x9d"                             # ???
	hf += "\x08"                             # cursor left
	
	hf += chr(0x80 + publisher_color)
	
	hf += "\x0d"                             # cursor to beginning of line
	
	# TODO: clip!
	
	for c in glob["publisher_name"] :
		if ord(c) == 0xfc:
			hf += "\x19\x48\x75"             # &uuml;
		else:
			hf += chr(ord(c))
	
	hf += "\x1f\x41\x5f"                     # set cursor to line 1, column 31
	
	# TODO: price	
	hf += "   0,00 DM"
	
	hf += "\x1e"                             # cursor home
	hf += "\x9b\x30\x40"                     # select palette #0
	hf += "\x9b\x31\x50"                     # protect line
	hf += "\x0a"                             # cursor down
	return hf

def create_page(pagenumber, basedir):
	with open(basedir + "a.glob") as f:
		glob = json.load(f)
	
	with open(basedir + "a.meta") as f:
		meta = json.load(f)
	
	if "palette" in meta:
		palette = meta["palette"]
		filename_palette = basedir + meta["include"] + ".pal"
		with open(filename_palette) as f:
			palette = json.load(f)
	
	filename_cept = basedir + "a.cept"
	with open(filename_cept, mode='rb') as f:
		data_cept = f.read()
	
	if "include" in meta:
		filename_include = basedir + meta["include"] + ".inc"
		with open(filename_include, mode='rb') as f:
			data_include = f.read()
	
	#pprint(glob)
	#pprint(meta)
	#if (palette):
	#	pprint(palette)
	
	# combine everything
	
	sh291 = len(data_include) > 500
	
	all_data = chr(0x14) # hide cursor
	
	if "clear_screen" in meta and meta["clear_screen"]:
		all_data += "\x1f\x2f\x43"                 # serial limited mode
		all_data += "\x0c"                         # clear screen
	
	if sh291:
		all_data += "\x1f\x2f\x40\x58"                 # service break to row 24
		all_data += "\x18"                             # clear line
		all_data += "\x53\x65\x69\x74\x65\x20\x77\x69" # "Seite wi"
		all_data += "\x72\x64\x20\x61\x75\x66\x67\x65" # "rd aufge"
		all_data += "\x62\x61\x75\x74\x20\x20\x20\x20" # "baut    "
		all_data += "\x20\x20\x20\x20\x20\x20\x20\x20" # "        "
		all_data += "\x20\x20\x20"                     # "   "
		all_data += "\x98"                             # hide
		all_data += "\x08"                             # cursor left
		all_data += "\x53\x48\x32\x39\x31"             # "SH291"
		all_data += "\x1f\x2f\x4f"                     # service break back
	
	# define palette
	if palette:
		all_data += "\x1f\x26\x20"                     # start defining colors
		all_data += "\x1f\x26\x31\x36"                 # define colors 16+
		palette_data = encode_palette(palette["palette"])
		all_data += palette_data
	
	if "set_cursor" in meta and meta["set_cursor"]:
		all_data += "\x1f\x41\x41"                 # set cursor to x=1 y=1
	
	if data_include:
		all_data += data_include
	
	if sh291:
		all_data += "\x1f\x2f\x43"                 # serial limited mode
		all_data += "\x0c"                         # clear screen
	
	# header + footer
	all_data += headerfooter(pagenumber, glob, meta)
	
	# links
	all_data += "\x1f\x3d\x30"
	i = 0x31
	for key, value in meta["links"].iteritems():
		all_data +=	"\x1f\x3d"
		all_data += chr(i)
		all_data += key.encode('utf-8').ljust(2)
		all_data += value.encode('utf-8')
		i += 1
		
	# payload
	all_data += data_cept
	
	all_data += "\x1f\x2f\x43" # serial limited mode
	
	# header + footer
	all_data += headerfooter(pagenumber, glob, meta)
	
	all_data += "\x1f\x58\x41"                           # set cursor to x=24 y=1
	all_data += "\x11"                                   # show cursor
	all_data += "\x1a"                                   # end of page
	
	return all_data	
	
	
cept_data = create_page("20095a", "data/20095/")
hexdump(cept_data)
	
	
	
	
	
	
	
	
	
	
