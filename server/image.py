from PIL import Image
import pprint
import math
import sys
from cept import Cept
import urllib.request

num_dcrs = 46

class Image_UI:

	def cept_from_image(url):
		if url.startswith("http://") or url.startswith("https://"):
			image = Image.open(urllib.request.urlopen(url))
		else:
			image = Image.open(url)
		image.load()
		(width, height) = image.size

		# calculate character resolution
		res_y = math.floor(math.sqrt(num_dcrs * height / width))
		res_x = math.floor(num_dcrs / res_y)
		sys.stderr.write("char resolution: " + str(res_x) + "*" + str(res_y) + "\n")

		# resample
		image = image.resize((res_x * 6, res_y * 10), resample = Image.LANCZOS)

		# convert to 16 custom colors
		try:
			image = image.quantize(colors = 16, method = 1)
		except:
			image = image.quantize(colors = 16, method = 2)

		# create array with palette
		p = image.getpalette()
		palette = []
		for i in range(0, 16):
			r = p[i * 3]
			g = p[i * 3 + 1]
			b = p[i * 3 + 2]
			palette.append("#{:02x}{:02x}{:02x}".format(r,g,b))

		# create drcs
		data_drcs = bytearray()
		data_drcs.extend(b'\x1f\x23\x20\x28\x20\x40\x4b\x44') # start defining 6x10 @ 16c
		data_drcs.extend(b'\x1f\x23\x21') # define starting at char 0x21

		for base_y in range(0, res_y * 10, 10):
			for base_x in range(0, res_x * 6, 6):
				for bitno in range(0, 4):
					data_drcs.extend([0x30 + bitno])
					data_drcs_block = bytearray()
					for y in range(0, 10):
						byte = 0
						for x in range(0, 6):
							byte <<= 1
							byte |= (image.getpixel((base_x + x, base_y + y)) >> bitno) & 1
						byte |= 0x40
						data_drcs_block.append(byte)

					# compression
					if data_drcs_block == bytearray(b'@@@@@@@@@@'):
						data_drcs_block = bytearray(b'\x20')
					elif data_drcs_block == bytearray(b'\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f\x7f'):
						data_drcs_block = bytearray(b'\x2f')
					else:
						y1 = 0
						max = 10
						while True:
							l = 0
							for y2 in range(y1 + 1, max):
								if data_drcs_block[y2] != data_drcs_block[y1]:
									break
								l += 1
							if l:
								data_drcs_block = data_drcs_block[:y1 + 1] + bytes([0x20 + l]) + data_drcs_block[y1 + l + 1:]
								y1 += 1
								max -= l - 1
							y1 += 1
							if y1 == max:
								break

#					sys.stderr.write("data_drcs_block: " + pprint.pformat(data_drcs_block) + "\n")
					data_drcs.extend(data_drcs_block)

		sys.stderr.write("DRCs compressed " + str(40 * res_x * res_y) + " down to " + str(len(data_drcs)) + "\n")

		# create characters to print
		data_chars = []
		for y in range(0, res_y):
			l = bytearray()
			for x in range(0, res_x):
				l.append(0x21 + (y * res_x + x) * 2 )
			data_chars.append(l)

		return (palette, data_drcs, data_chars)

	def create_image_page():
#		filename = "/Users/mist/Desktop/RGB_24bits_palette_sample_image.jpg"
#		filename = "/Users/mist/Desktop/Lenna_(test_image).png"
#		filename = "/Users/mist/Desktop/Wikipedia_logo_593.jpg"
		filename = "/Users/mist/Desktop/220px-C64c_system.jpg"

		(palette, drcs, chars) = Image_UI.cept_from_image(filename)

		data_cept = bytearray()
		data_cept.extend(Cept.define_palette(palette))
		data_cept.extend(drcs)

		data_cept.extend(Cept.set_cursor(3, 1))
		data_cept.extend(Cept.load_g0_drcs())
		for l in chars:
			data_cept.extend(l)
			data_cept.extend(b'\r\n')

		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"publisher_color": 0
		}

		return (meta, data_cept)

	def create_page(pageid):
		if pageid == "666a":
			return Image_UI.create_image_page()
		else:
			return None
