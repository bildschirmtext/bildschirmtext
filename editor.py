import sys

from cept import Cept

class Editor:
	line = None
	column = None
	height = None
	width = None
	fgcolor = None
	bgcolor = None
	allowed_inputs = None
	string = ""
	main_mode = False

	def debug_print(s):	
		sys.stderr.write("'")
		for cc in s:
			if cc == chr(Cept.ini()):
				sys.stderr.write("<INI>")
			if cc == chr(Cept.ter()):
				sys.stderr.write("<TER>")
			else:
				sys.stderr.write(cc)
		sys.stderr.write("'\n")
	
	def draw_background(self, draw_color = False):
		cept_data = bytearray(Cept.parallel_limited_mode())
		cept_data.extend(Cept.set_cursor(self.line, self.column))
		for i in range(0, self.height):
			if draw_color:
				cept_data.extend(Cept.set_fg_color(self.fgcolor))
				cept_data.extend(Cept.set_bg_color(self.bgcolor))
			if self.width == 40:
				cept_data.extend(Cept.clear_line())
			else:
				cept_data.extend(Cept.repeat(" ", self.width))
			if i != self.height - 1:
				cept_data.extend(b'\n')
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()

	def edit(self):
		cept_data = bytearray()
		cept_data.extend(Cept.set_cursor(self.line, self.column))
		if self.fgcolor:
			cept_data.extend(Cept.set_fg_color(self.fgcolor))
		if self.bgcolor:
			cept_data.extend(Cept.set_bg_color(self.bgcolor))
		if self.string:
			s = self.string
			if s[0] == chr(Cept.ini()):
				s = "*" + s[1:]
			cept_data.extend(Cept.from_str(s))
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()

		while True:
			c = sys.stdin.read(1)
			sys.stderr.write("In: 0x" + hex(ord(c)) + "\n")

			if ord(c) == Cept.ini():
				if not self.main_mode:
					sys.stderr.write("escape\n")
					editor = Editor()
					editor.line = 24
					editor.column = 1
					editor.height = 1
					editor.width = 40
					editor.string = chr(Cept.ini())
					editor.main_mode = True
					main_mode_val = editor.edit()
					if main_mode_val is None:
						sys.stderr.write("unescape\n")
						self.string = ""
						self.draw_background(False)
						continue
					else:
						Editor.debug_print(main_mode_val)
						return main_mode_val
					# TODO: handle *021# - *029#
			if ord(c) == Cept.ter():
				Editor.debug_print(self.string)
				break
			if ord(c) == 8:
				if len(self.string) == 0:
					continue
				sys.stdout.buffer.write(b'\b \b')
				sys.stdout.flush()
				self.string = self.string[:-1] # TODO doesn't work with non-ASCII
			elif (ord(c) >= 0x20 or ord(c) != 0x19) and len(self.string) < self.width:
				is_allowed = True
				found = False
				if self.allowed_inputs:
					found = False
					is_allowed = False
					s = self.string + c
					for allowed_input in self.allowed_inputs:
						if s == allowed_input:
							is_allowed = True
							found = True
							break
						if allowed_input.startswith(s):
							is_allowed = True
							break
				self.string += c
				sys.stdout.write(c)
				sys.stdout.flush()
				if not is_allowed:
					break
				if found:
					break
			sys.stderr.write("self.string = ")
			Editor.debug_print(self.string)

			if self.main_mode:
				if self.string[-2:] == chr(Cept.ini()) + chr(Cept.ini()):
					# exit main editor, tell parent to clear
					return None
			
		return self.string

	
