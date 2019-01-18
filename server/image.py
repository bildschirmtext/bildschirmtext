from PIL import Image
import pprint
import math
import sys
from cept import Cept
import urllib.request

NUM_DRCS = 46

PIXEL_ASPECT_RATIO = 0.92

class Image_UI:



	def cept_from_image(url):
		sys.stderr.write("URL: " + pprint.pformat(url) + "\n")
		if url.startswith("http://") or url.startswith("https://"):
			image = Image.open(urllib.request.urlopen(url))
		else:
			image = Image.open(url)
		image.load()
		(width, height) = image.size
		sys.stderr.write("resolution: " + str(width) + "*" + str(height) + "\n")

		# calculate character resolution
		exact_res_x = math.sqrt(NUM_DRCS * width / height)
		exact_res_y = math.sqrt(NUM_DRCS * height / width)
		aspect_ratio = width / height / PIXEL_ASPECT_RATIO

#		sys.stderr.write("exact char resolution: " + str(exact_res_x) + "*" + str(exact_res_y) + "\n")

		res_x_1 = math.floor(exact_res_x)
		res_y_1 = math.floor(NUM_DRCS / res_x_1)
		error_1 = abs(1 - (aspect_ratio / (res_x_1 / res_y_1)))
		res_x_2 = math.ceil(exact_res_x)
		res_y_2 = math.floor(NUM_DRCS / res_x_2)
		error_2 = abs(1 - (aspect_ratio / (res_x_2 / res_y_2)))
		res_y_3 = math.floor(exact_res_y)
		res_x_3 = math.floor(NUM_DRCS / res_y_3)
		error_3 = abs(1 - (aspect_ratio / (res_x_3 / res_y_3)))
		res_y_4 = math.ceil(exact_res_y)
		res_x_4 = math.floor(NUM_DRCS / res_y_4)
		error_4 = abs(1 - (aspect_ratio / (res_x_4 / res_y_4)))

#		sys.stderr.write("char resolution 1: " + str(res_x_1) + "*" + str(res_y_1) + ", error: " + str(error_1) + "\n")
#		sys.stderr.write("char resolution 2: " + str(res_x_2) + "*" + str(res_y_2) + ", error: " + str(error_2) + "\n")
#		sys.stderr.write("char resolution 3: " + str(res_x_3) + "*" + str(res_y_3) + ", error: " + str(error_3) + "\n")
#		sys.stderr.write("char resolution 4: " + str(res_x_4) + "*" + str(res_y_4) + ", error: " + str(error_4) + "\n")

		res_x = res_x_1
		res_y = res_y_1
		error = error_1
		if error_2 < error:
			res_x = res_x_2
			res_y = res_y_2
			error = error_2
		if error_3 < error:
			res_x = res_x_3
			res_y = res_y_3
			error = error_3
		if error_4 < error:
			res_x = res_x_4
			res_y = res_y_4
			error = error_4

		sys.stderr.write("char resolution:   " + str(res_x) + "*" + str(res_y) + ", error: " + str(error) + "\n")

		# remove alpha
		if image.mode == "RGBA" or image.mode == "LA":
			background = Image.new("RGB", image.size, (255, 255, 255))
			index = 3 if image.mode == "RGBA" else 1
			background.paste(image, mask=image.split()[index])
			image = background

		# resample
		image = image.resize((res_x * 6, res_y * 10), resample = Image.ANTIALIAS)

		# convert to 16 custom colors
		image = image.quantize(colors = 16, method = 1)

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
