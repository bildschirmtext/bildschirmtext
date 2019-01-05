import sys

from cept import Cept

class Editor:
	line = None
	column = None
	height = None
	width = None
	fgcolor = None
	bgcolor = None

	def draw_background(self):
		cept_data = bytearray(Cept.parallel_limited_mode())
		for i in range(0, self.height):
			cept_data.extend(Cept.set_cursor(self.line + i, self.column))
			cept_data.extend(Cept.set_fg_color(self.fgcolor))
			cept_data.extend(Cept.set_bg_color(self.bgcolor))
			cept_data.extend(Cept.repeat(" ", self.width))
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()

	def edit(self):
		cept_data = bytearray()
		cept_data.extend(Cept.set_cursor(self.line, self.column))
		cept_data.extend(Cept.set_fg_color(self.fgcolor))
		cept_data.extend(Cept.set_bg_color(self.bgcolor))
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
			elif len(s) < self.width:
				s += c
				sys.stdout.write(c)
				sys.stdout.flush()
			sys.stderr.write("String: '" + s + "'\n")
			
		return s
