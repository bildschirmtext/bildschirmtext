import sys

from cept import Cept

class Editor:
	line = None
	column = None
	height = None
	width = None
	fgcolor = None
	bgcolor = None

	def read_with_echo(clear_line):
		c = sys.stdin.read(1)
		if clear_line:
			sys.stdout.write('\x18');
		if ord(c) == Cept.ini():
			sys.stdout.write('*')
		elif ord(c) == Cept.ter():
			sys.stdout.write('#')
		else:
			sys.stdout.write(c)
		sys.stdout.flush()
		sys.stderr.write("In: " + str(ord(c)) + "\n")
		return c
	
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
	
	def main_input(current_pageid, links, showing_message):
		# convert "#" to TER in links
		if "#" in links:
			links[chr(Cept.ter())] = links["#"]
			links.pop("#", None)
		# extract link prefix characters
		link_prefixes = set()
		for link in links:
			link_prefixes.add(link[0])
	
		s = ""
		desired_pageid = None
		
		while True:
			c = Editor.read_with_echo(showing_message)
			showing_message = False
	
			# TODO: backspace
			# TODO: only allow alphanumeric
			
			s += c
			sys.stderr.write("s: '")
			Editor.debug_print(s)
		
			if s[0].isdigit() or s[0] == chr(Cept.ter()):
				# potential link
				if s in links:
					# correct link
					desired_pageid = links[s]
				elif len(s) == 1 and s in link_prefixes:
					# prefix of a link
					pass
				elif s == chr(Cept.ter()):
					# next sub-page
					if current_pageid[-1:] >= "a" and current_pageid[-1:] <= "y":
						desired_pageid = current_pageid[:-1] + chr(ord(current_pageid[-1:]) + 1)
					elif current_pageid[-1:] >= '0' and current_pageid[-1:] <= '9':
						desired_pageid = current_pageid + "b"
				else:
					# can't be a valid link
					s = ""
					sys.stdout.buffer.write(create_system_message(100) + Cept.sequence_end_of_page())
					sys.stdout.flush()
					showing_message = True
			elif s[-2:] == chr(Cept.ini()) + chr(Cept.ini()):
				# "**" clears input
				s = ""
				cept_data = bytearray(Cept.set_cursor(24, 1))
				cept_data.extend(Cept.clear_line())
				sys.stdout.buffer.write(cept_data)
				sys.stdout.flush()
				sys.stderr.write("Cleared.\n")
			elif s[0] == chr(Cept.ini()) and s[-1] == chr(Cept.ter()):
				desired_pageid = s[1:-1]
				sys.stderr.write("New page: '" + desired_pageid+ "'.\n")
		
			if desired_pageid is not None:
				break
		return desired_pageid
	

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
