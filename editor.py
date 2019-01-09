import sys
import pprint

from cept import Cept

'''
	An Editor object is used for single or multi line text input. Every field on
	a dialog page is backed by one Editor object.
	
	## Features
	
	* An editor has a position, a size, a foreground and a background color. It
	  can draw its own background.
	* An editor can be supplied an list of legal inputs. As soon as a character
	  is entered that makes the current contents of the editor illegal, the edit()
	  method returns with the illegal string.

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
	legal_values = None
	string = None
	command_mode = False
	no_navigation = False

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
	
	def draw(self):
		if self.string:
			s = self.string
			if s[0] == chr(Cept.ini()):
				s = "*" + s[1:]
		else:
			 s = ""

		cept_data = bytearray(Cept.parallel_limited_mode())
		cept_data.extend(Cept.hide_cursor())
		cept_data.extend(Cept.set_cursor(self.line, self.column))
		for i in range(0, self.height):
			if self.fgcolor is not None and self.bgcolor is not None:
				cept_data.extend(Cept.set_fg_color(self.fgcolor))
				cept_data.extend(Cept.set_bg_color(self.bgcolor))
			if self.width == 40:
				cept_data.extend(Cept.clear_line())
				cept_data.extend(Cept.from_str(s))
			else:
				cept_data.extend(Cept.from_str(s))
				cept_data.extend(Cept.repeat(" ", self.width -  len(s)))
			if i != self.height - 1:
				cept_data.extend(b'\n')
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

	def edit(self, skip = False):
		start = True
		dct = False

		if self.string is None:
			self.string = ""

		sys.stderr.write("starting with self.string = ")
		Editor.debug_print(self.string)
		
		while True:
			if start:
				start = False
				self.print_hint()
				cept_data = bytearray()
				cept_data.extend(Cept.set_cursor(self.line, self.column + len(self.string)))
				if self.fgcolor:
					cept_data.extend(Cept.set_fg_color(self.fgcolor))
				if self.bgcolor:
					cept_data.extend(Cept.set_bg_color(self.bgcolor))
				cept_data.extend(Cept.show_cursor())
				sys.stdout.buffer.write(cept_data)
				sys.stdout.flush()

			if skip:
				sys.stderr.write("skipping\n")
				break
		
			c = sys.stdin.read(1)
			sys.stderr.write("In: 0x" + hex(ord(c)) + "\n")

			if self.command_mode and ord(c) == Cept.ini() and self.string[-1:] == chr(Cept.ini()):
				# exit command mode, tell parent to clear
				return (None, False)

			if ord(c) == Cept.ini():
				if not self.command_mode:
					sys.stderr.write("entering command mode\n")
					editor = Editor()
					editor.line = 24
					editor.column = 1
					editor.height = 1
					editor.width = 40
					editor.string = chr(Cept.ini())
					editor.command_mode = True
					(val, dct) = editor.edit()
					if val is None:
						sys.stderr.write("exiting command mode\n")
					else:
						#Editor.debug_print(val)
						# TODO: handle *021# - *029#
						if not self.no_navigation or val == chr(Cept.ini())+"00" or val == chr(Cept.ini())+"09":
							return (val, False)
						sys.stderr.write("ignoring navigation\n")
					self.string = ""
					self.draw()
					start = True
					continue
			elif ord(c) == Cept.ter():
				#Editor.debug_print(self.string)
				break
			elif ord(c) == Cept.dct():
				dct = True
				break
			elif ord(c) == 8:
				if len(self.string) == 0:
					continue
				sys.stdout.buffer.write(b'\b \b')
				sys.stdout.flush()
				self.string = self.string[:-1] # TODO doesn't work with non-ASCII
			elif (ord(c) >= 0x20 or ord(c) != 0x19) and len(self.string) < self.width:
				is_legal = True
				found = False
				if self.legal_values:
					found = False
					is_legal = False
					s = self.string + c
					for legal_input in self.legal_values:
						if s == legal_input:
							is_legal = True
							found = True
							break
						elif legal_input.startswith(s):
							is_legal = True
							break
				self.string += c
				sys.stdout.write(c)
				sys.stdout.flush()
				if not is_legal:
					break
				if found:
					break
			sys.stderr.write("self.string = ")
			Editor.debug_print(self.string)

		return (self.string, dct)

	
