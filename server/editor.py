import sys
import pprint

from cept import Cept
from util import Util

'''
	An Editor object is used for single or multi line text input. Every field on
	a dialog page is backed by one Editor object.
	
	## Features
	
	* An editor has a position, a size, a foreground and a background color. If
	  the color properties are set, it will draw its own background.
	* An editor can be given an list of legal inputs.
	  If end_on_illegal_character is True, as soon as a character is entered
	  that makes the current contents of the editor illegal, the edit() method
	  returns the illegal string.
	  If end_on_illegal_character is False, characters that would make the input
	  illegal are ignored.
	  If end_on_legal_string is True, the edit() method returns as soon as a
	  legal string is completed.
	  

	## Command Mode

	Within any editor, "*" will create a "command mode" child editor in line 24
	that allows entering any global *...# code.

	In command mode, two "*" characters or one "#" character will exit command
	mode and the resulting global code will be sent back to the original
	editor.

	The parent editor will then 
	* interpret editor codes (** to clear editor, *022# for cursor up etc.)
	* instruct the main loop to navigate to the page in case of a page number

	## Main Editor

	The main editor that is presented in line 24 after a non-dialog page is
	shown is just a normal editor that happens to be in line 24, which is
	passed the list of links as legal inputs. "*" will create a command mode
	editor on top of the main editor in line 24.
'''

class Editor:
	line = None
	column = None
	height = None
	width = None
	fgcolor = None
	bgcolor = None
	hint = None
	type = None
	legal_values = None
	end_on_illegal_character = False
	end_on_legal_string = False
	clear_line = True
	cursor_home = False
	echo_ter = False
	command_mode = False
	no_navigation = False
	
	__data = None
	__y = 0
	__x = 0

	@property
	def string(self):
		d = []
		for l in self.__data:
			d.append(l.rstrip())
		ret = '\n'.join(d)
		ret = ret.rstrip('\n')
#		sys.stderr.write("string:\n" + pprint.pformat(ret) + "\n")
		return ret		

	@string.setter
	def string(self, string):
		if string is None:
			string = ""
		self.__data = []
		for line in string.split("\n")[:self.height]:
			self.__data.append((line + " " * self.width)[:self.width])
		while len(self.__data) < self.height:
			self.__data.append(" " * self.width)
#		sys.stderr.write("self.__data:\n" + pprint.pformat(self.__data) + "\n")

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
	
	def set_color(self):
		cept_data = bytearray()
		if self.fgcolor is not None:
			cept_data.extend(Cept.set_fg_color(self.fgcolor))
		if self.bgcolor is not None:
			cept_data.extend(Cept.set_bg_color(self.bgcolor))
		return cept_data

	def draw(self):
		cept_data = bytearray(Cept.parallel_limited_mode())
		cept_data.extend(Cept.hide_cursor())
		cept_data.extend(Cept.set_cursor(self.line, self.column))
		fill_with_clear_line = self.clear_line and self.width == 40# and self.height == 1
		fill_with_spaces = self.clear_line and not fill_with_clear_line
		for i in range(0, self.height):
			l = self.__data[i].rstrip()

			if self.type == "password":
				l = "*" * len(l)
			else:
				if l.startswith(chr(Cept.ini())):
					l = "*" + l[1:]

			if l:
				cept_data.extend(self.set_color())

			if fill_with_clear_line:
				cept_data.extend(Cept.clear_line())
				if self.bgcolor:
					cept_data.extend(Cept.set_line_bg_color(self.bgcolor))

			cept_data.extend(Cept.from_str(l))

			if fill_with_spaces and len(l) > self.width:
				cept_data.extend(Cept.repeat(" ", self.width - len(l)))

			if i != self.height - 1:
				if self.column == 1:
					if self.width != 40 or fill_with_clear_line:
						cept_data.extend(b'\n')
				else:
					cept_data.extend(Cept.set_cursor(self.line + i + 1, self.column))
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()

	def print_hint(self):
		if self.hint:
			cept_data = bytearray(Cept.service_break(24))
			cept_data.extend(Cept.clear_line())
			cept_data.extend(Cept.from_str(self.hint, 1))
			cept_data.extend(Cept.hide_text())
			cept_data.extend(Cept.service_break_back())
			sys.stdout.buffer.write(cept_data)
			sys.stdout.flush()

	def try_insert_character(self, c):
		if self.__x < self.width:
			return self.__data[self.__y][:self.__x] + c + self.__data[self.__y][self.__x + 1:]
		else:
			return self.__data[self.__y]		

	def insert_character(self, c):
		if self.__x < self.width:
			self.__data[self.__y] = self.try_insert_character(c)
			self.__x += 1
			return True
		else:
			return False

	last_c = ""

	def insert_carriage_return(self):
		if self.__x != 0:
			self.__x = 0
			if self.column == 1:
				sys.stdout.write("\r")
			else:
				sys.stdout.buffer.write(Cept.set_cursor(self.line + self.__y, self.column))
			sys.stdout.buffer.write(self.set_color())
			sys.stdout.flush()

	def insert_line_feed(self):
		if self.__y < self.height - 1:
			self.__y += 1
			sys.stdout.write("\n")
			sys.stdout.flush()

	def insert_control_character(self, c):
		if c == "\r": # enter
			# some terminals send CR/LF, others just CR, so we have to do
			# the work on CR, and ignore LF if it was preceded by a CR
			self.insert_carriage_return()
			self.insert_line_feed()
		elif c == "\n": # down
			if self.last_c != "\r": # see above
				self.insert_line_feed()
		elif c == "\b": # left
			if self.__x > 0:
				self.__x -= 1
				sys.stdout.write(c)
				sys.stdout.flush()
		elif c == "\x0b": # up
			if self.__y > 0:
				self.__y -= 1
				sys.stdout.write(c)
				sys.stdout.flush()
		elif c == "\x09": # right
			if self.__x < self.width:
				self.__x += 1
				sys.stdout.write(c)
				sys.stdout.flush()
		self.last_c = c

	def edit(self, skip_entry = False):
		start = True
		dct = False
		prefix = bytearray()
		inject_char = None

		while True:
			if start and not skip_entry:
				start = False
				self.print_hint()
				cept_data = bytearray()
				self.__y = 0
				if self.height > 1 or self.cursor_home:
					cept_data.extend(Cept.set_cursor(self.line, self.column))
					self.__x = 0
				else:
					cept_data.extend(Cept.set_cursor(self.line, self.column + len(self.string)))
					self.__x = len(self.string)
				if self.fgcolor:
					cept_data.extend(Cept.set_fg_color(self.fgcolor))
				if self.bgcolor:
					cept_data.extend(Cept.set_bg_color(self.bgcolor))
				cept_data.extend(Cept.show_cursor())
				sys.stdout.buffer.write(cept_data)
				sys.stdout.flush()

			if skip_entry:
				sys.stderr.write("skipping\n")
				break
		
			if inject_char:
				c = inject_char
				inject_char = None
			else:
				c = Util.readchar()
			sys.stderr.write("In: " + hex(ord(c)) + "\n")

			if self.command_mode and ord(c) == Cept.ini() and self.string[-1:] == chr(Cept.ini()):
				# exit command mode, tell parent to clear
				return (None, False)

			c2 = Cept.code_to_str(prefix + bytes([ord(c)]))
			if c2 is None: # sequence not complete
				prefix.append(ord(c))
				continue
			prefix = bytearray()
			if c2 == "": # we couldn't decode it
				continue
			c = c2
			
			# if c < 0x20
			#     c is a CEPT control code
			# if c >= 0x20
			#     c is Unicode

			if ord(c) < 0x20: #and ord(c) != Cept.ini():
				prefix = bytearray()
				if ord(c) == Cept.ini():
					if not self.command_mode:
						is_editor_code = False
						sys.stderr.write("entering command mode\n")
						editor = Editor()
						editor.line = 24
						editor.column = 1
						editor.height = 1
						editor.width = 20
						editor.string = chr(Cept.ini())
						editor.command_mode = True
						editor.clear_line = True
						editor.echo_ter = True
						editor.draw()
						(val, dct) = editor.edit()
						sys.stderr.write("exited command mode\n")
						if val is None: # "**" in command mode
							self.string = ""
							self.draw()
						else:
							Editor.debug_print(val)
							if val.startswith(chr(Cept.ini())+"02") and len(val) == 4:
								# editor codes *021# etc.
								is_editor_code = True
								code = int(val[3:])
								map = {
									1: "\r",   # CR
									2: "\x0b", # UP
									4: "\x08", # LEFT
									6: "\x09", # RIGHT
									8: "\n",   # DOWN
									9: "\x1a"  # DCT
								}
								c = map.get(code)
								if c is not None:
									inject_char = c
								else:
									sys.stderr.write("ignoring invalid editor code\n")
							else:
								# global code
								if not self.no_navigation or val == chr(Cept.ini())+"00" or val == chr(Cept.ini())+"09":
									return (val, False)
								sys.stderr.write("ignoring navigation\n")
						start = True
						continue
				elif ord(c) == Cept.ter():
					if self.echo_ter:
						sys.stdout.write("#")
						sys.stdout.flush()
					break
				elif ord(c) == Cept.dct():
					dct = True
					break
				self.insert_control_character(c)
			else: # ord(c) >= 0x20
				character_legal = True
				string_legal = False
				# CEPT doesn't have a concept of backspace, so the backspace key
				# sends the sequence CSR_LEFT, SPACE, CSR_LEFT. It is very tricky
				# to detect this properly, so we will just allow spaces in
				# "number" and "alpha" input fields.
				if self.type == "number" and not c.isdigit() and not c == " ":
					character_legal = False
				elif self.type == "alpha" and not c.isalpha() and not c == " ":
					character_legal = False
				elif self.legal_values:
					s = self.try_insert_character(c).rstrip()
					character_legal = False
					for legal_input in self.legal_values:
						if s == legal_input:
							character_legal = True
							string_legal = True
							sys.stderr.write("string_legal!\n")
							break
						elif legal_input.startswith(s):
							character_legal = True
							sys.stderr.write("character_legal!\n")
							break
				if character_legal or self.end_on_illegal_character:
					if self.insert_character(c):
						if self.type == "password":
							sys.stdout.write("*")
						else:
							sys.stdout.buffer.write(Cept.from_str(c))
						sys.stdout.flush()
				if not character_legal and self.end_on_illegal_character:
					break
				if string_legal and self.end_on_legal_string:
					break
			sys.stderr.write("self.__data:\n" + pprint.pformat(self.__data) + "\n")
			sys.stderr.write("self.string:\n" + pprint.pformat(self.string) + "\n")

		return (self.string, dct)

