import sys

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
	legal_inputs = None
	string = ""
	command_mode = False

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
				if not self.command_mode:
					sys.stderr.write("entering command mode\n")
					editor = Editor()
					editor.line = 24
					editor.column = 1
					editor.height = 1
					editor.width = 40
					editor.string = chr(Cept.ini())
					editor.command_mode = True
					command_mode_val = editor.edit()
					if command_mode_val is None:
						sys.stderr.write("exiting command mode\n")
						self.string = ""
						self.draw_background(False)
						continue
					else:
						#Editor.debug_print(command_mode_val)
						return command_mode_val
					# TODO: handle *021# - *029#
			if ord(c) == Cept.ter():
				#Editor.debug_print(self.string)
				break
			if ord(c) == 8:
				if len(self.string) == 0:
					continue
				sys.stdout.buffer.write(b'\b \b')
				sys.stdout.flush()
				self.string = self.string[:-1] # TODO doesn't work with non-ASCII
			elif (ord(c) >= 0x20 or ord(c) != 0x19) and len(self.string) < self.width:
				is_legal = True
				found = False
				if self.legal_inputs:
					found = False
					is_legal = False
					s = self.string + c
					for legal_input in self.legal_inputs:
						if s == legal_input:
							is_legal = True
							found = True
							break
						if legal_input.startswith(s):
							is_legal = True
							break
				self.string += c
				sys.stdout.write(c)
				sys.stdout.flush()
				if not is_legal:
					break
				if found:
					break
			#sys.stderr.write("self.string = ")
			#Editor.debug_print(self.string)

			if self.command_mode:
				if self.string[-2:] == chr(Cept.ini()) + chr(Cept.ini()):
					# exit command mode, tell parent to clear
					return None
			
		return self.string

	
