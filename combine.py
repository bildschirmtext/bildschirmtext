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
		lines.append("%08x:  %-*s  |%s|\n" % (c, length*3, hex, printable))
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





