class Cept(bytearray):

	# Constructor 
	def __init__(self): 
		print("Hello Cept")
	

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
	def hide_cursor():
		return b'\x14'


	@staticmethod
	def cursor_home():
		return b'\x1e'

	@staticmethod
	def clear_screen():
		return b'\x0c'

	@staticmethod
	def protect_line():
		return b'\x9b\x31\x50'


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
	def set_fg_color(c):
		if c > 7:
			pal = 1
			c -= 8
		else:
			pal = 0
		return bytes([0x9b, 0x30 + pal, 0x40, 0x80 + c])

	@staticmethod
	def set_bg_color(c):
		if c > 7:
			pal = 1
			c -= 8
		else:
			pal = 0
		return bytes([0x9b, 0x30 + pal, 0x40, 0x90 + c])
		
		
		
		
	@staticmethod
	def set_cursor(y, x):
		return bytes([0x1f, 0x40 + y, 0x40 + x])
	
		
		