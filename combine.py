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

basedir = "data/20095/"

with open(basedir + "a.glob") as f:
	glob = json.load(f)
	pprint(glob)

with open(basedir + "a.meta") as f:
	meta = json.load(f)
	pprint(meta)

if "palette" in meta:
	palette = meta["palette"]
	filename_palette = basedir + meta["include"] + ".pal"
	with open(filename_palette) as f:
		palette = json.load(f)
		pprint(palette)

if "include" in meta:
	filename_include = basedir + meta["include"] + ".inc"
	with open(filename_include, mode='rb') as f:
		data_include = f.read()
		hexdump(data_include)

filename_cept = basedir + "a.cept"
with open(filename_cept, mode='rb') as f:
	data_cept = f.read()
	hexdump(data_cept)


# combine everything

all_data = chr(0x14) # hide cursor

if "clear_screen" in meta and meta["clear_screen"]:
	all_data += "\x1f\x2f\x43"                 # serial limited mode
	all_data += "\x0c"                         # clear screen

# always show SH291

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

all_data += "\x1f\x26\x20"                     # start defining colors
all_data += "\x1f\x26\x31\x36"                 # define colors 16+

palette_data = ""
for hexcolor in palette["palette"]:
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
	print(byte0, byte1)
	palette_data += chr(byte0) + chr(byte1)

all_data += palette_data

if "set_cursor" in meta and meta["set_cursor"]:
	all_data += "\x1f\x41\x41"                 # set cursor to x=1 y=1


#	all_data += "\x1f\x2f\x43"                 # serial limited mode
#	all_data += "\x0c"                         # clear screen

hexdump(all_data)
