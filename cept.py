class Cept(bytearray):

	# Constructor 
	def __init__(self): 
		print("Hello Cept")
	
	# private
	def g2code(c, mode):
		if mode == 0:
			return b'\x19' + bytearray([ord(c)])
		else:
			return bytearray([ord(c) + 0x80])
	
	def from_str(s1, mode = 0):
		s2 = bytearray()
		for c in s1:
			# TODO: complete conversion!
			if ord(c) == 0xe4:
				s2.extend(Cept.g2code('H', mode) + b'a')           # &auml;
			elif ord(c) == 0xf6:
				s2.extend(Cept.g2code('H', mode) + b'o')           # &ouml;
			elif ord(c) == 0xfc:
				s2.extend(Cept.g2code('H', mode) + b'u')           # &uuml;
			elif ord(c) == 0xc4:
				s2.extend(Cept.g2code('H', mode) + b'A')           # &Auml;
			elif ord(c) == 0xd6:
				s2.extend(Cept.g2code('H', mode) + b'O')           # &Ouml;
			elif ord(c) == 0xdc:
				s2.extend(Cept.g2code('H', mode) + b'U')           # &Uuml;
			elif ord(c) == 0xdf:
				s2.extend(Cept.g2code('{', mode))                 # &szlig;
			else:
				s2.append(ord(c))
		return s2

	def from_aa(aa, indent):
		dict = { "": 0x20, "1": 0x21, "2": 0x22, "12": 0x23, "3": 0x24, "13": 0x25, "23": 0x26, "123": 0x27, "4": 0x28, "14": 0x29, "24": 0x2a, "124": 0x2b, "34": 0x2c, "134": 0x2d, "234": 0x2e, "1234": 0x2f, "5": 0x30, "15": 0x31, "25": 0x32, "125": 0x33, "35": 0x34, "135": 0x35, "235": 0x36, "1235": 0x37, "45": 0x38, "145": 0x39, "245": 0x3a, "1245": 0x3b, "345": 0x3c, "1345": 0x3d, "2345": 0x3e, "12345": 0x3f, "123456": 0x5f, "6": 0x60, "16": 0x61, "26": 0x62, "126": 0x63, "36": 0x64, "136": 0x65, "236": 0x66, "1236": 0x67, "46": 0x68, "146": 0x69, "246": 0x6a, "1246": 0x6b, "346": 0x6c, "1346": 0x6d, "2346": 0x6e, "12346": 0x6f, "56": 0x70, "156": 0x71, "256": 0x72, "1256": 0x73, "356": 0x74, "1356": 0x75, "2356": 0x76, "12356": 0x77, "456": 0x78, "1456": 0x79, "2456": 0x7a, "12456": 0x7b, "3456": 0x7c, "13456": 0x7d, "23456": 0x7e }
		while len(aa) % 3 != 0:
			aa.append(" " * len(aa[0]))
		data_cept = bytearray()
		data_cept.extend(b'\x0e')                      # G1 into left charset
		for y in range(0, len(aa), 3):
			if indent < 4:
				for i in range(0, indent):
					data_cept.extend(b'\x09')
			else:
				data_cept.extend(bytes([0x09, 0x12, 0x40 + indent - 1]))
			for x in range(0, len(aa[0]), 2):
				s = ""
				next_column_exists = x+1 < len(aa[y])
				if aa[y][x] != " ":
					s += "1"
				if next_column_exists and aa[y][x+1] != " ":
					s += "2"
				if aa[y+1][x] != " ":
					s += "3"
				if next_column_exists and aa[y+1][x+1] != " ":
					s += "4"
				if aa[y+2][x] != " ":
					s += "5"
				if next_column_exists and aa[y+2][x+1] != " ":
					s += "6"
				data_cept.append(dict[s])
			data_cept.extend(b'\r\n')
		data_cept.extend(b'\x0f')                       # G0 into left charset
		return data_cept

	# CEPT sequences	

	@staticmethod
	def sequence_end_of_page():
		return (
			b'\x1f\x58\x41'		 # set cursor to line 24, column 1
			b'\x11'				 # show cursor
			b'\x1a'				 # end of page
		)
	

	# CEPT codes
	
	@staticmethod
	def ini():
		return 0x13 # cept init - 19 - prints *


	@staticmethod
	def ter():
		return 0x1c # cept terminate - 28 - prints #

	@staticmethod
	def set_res_40_24():
		return b'\x1f\x2d'

	@staticmethod
	def hide_cursor():
		return b'\x14'

	@staticmethod
	def cursor_home():
		return b'\x1e'

	@staticmethod
	def clear_screen():
		return b'\x0c'

	@staticmethod
	def clear_line():
		return b'\x18'

	@staticmethod
	def protect_line():
		return b'\x9b\x31\x50'

	@staticmethod
	def unprotect_line():
		return b'\x9b\x31\x51'

	@staticmethod
	def parallel_mode():
		return b'\x1b\x22\x41'
		
	@staticmethod
	def serial_limited_mode():
		return b'\x1f\x2f\x43'
		
	@staticmethod
	def parallel_limited_mode():
		return b'\x1f\x2f\x44'

	@staticmethod
	def repeat(c, n):
		return bytes([ord(c), 0x12, 0x40 + n - 1])

	@staticmethod
	def define_palette(palette):
		cept = bytearray(
			b'\x1f\x26\x20'			  # start defining colors
			b'\x1f\x26\x31\x36'		  # define colors 16+
		)
	
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
			cept.append(byte0)
			cept.append(byte1)
		return cept

	
	@staticmethod
	def set_palette(pal):
		return bytes([0x9b, 0x30 + pal, 0x40])

	@staticmethod
	def set_fg_color_simple(c):
		return bytes([0x80 + c])

	@staticmethod
	def set_bg_color_simple(c):
		return bytes([0x90 + c])

	@staticmethod
	def set_fg_color(c):
		return Cept.set_palette(c >> 3) + Cept.set_fg_color_simple(c & 7)

	@staticmethod
	def set_bg_color(c):
		return Cept.set_palette(c >> 3) + Cept.set_bg_color_simple(c & 7)

	@staticmethod
	def set_line_bg_color_simple(c):
		return bytes([0x1b, 0x23, 0x21, 0x50 + c])

	@staticmethod
	def set_screen_bg_color_simple(c):
		return bytes([0x1b, 0x23, 0x20, 0x50 + c])

	@staticmethod
	def set_screen_bg_color(c):
		return Cept.set_palette(c >> 3) + Cept.set_screen_bg_color_simple(c & 7)

	@staticmethod
	def set_line_fg_color_simple(c):
		return bytes([0x1b, 0x23, 0x21, 0x40 + c])

	@staticmethod
	def set_cursor(y, x):
		return bytes([0x1f, 0x40 + y, 0x40 + x])

	@staticmethod
	def service_break(y):
		return bytes([0x1f, 0x2f, 0x40, 0x40 + y])
	
	@staticmethod
	def service_break_back():
		return b'\x1f\x2f\x4f'
	
	@staticmethod
	def normal_size():
		return b'\x8c'

	@staticmethod
	def double_height():
		return b'\x8d'
		
	@staticmethod
	def hide_text():
		return b'\x98'
		
	@staticmethod
	def code_9d():
		return b'\x9d'
		
	@staticmethod
	def code_9e():
		return b'\x9e'
		
		