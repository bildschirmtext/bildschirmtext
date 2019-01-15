import sys
import re
import json
import pprint
import urllib.request

from bs4 import BeautifulSoup

from cept import Cept

LINES_PER_PAGE = 17

class wikipedia_cept_page:
	x = None
	y = None
	lines_cept = []
	data_cept = None
	italics = False
	bold = False
	link = False
	dirty = False

	def __init__(self):
		self.x = 0
		self.y = -1
		self.init_new_line()

	def init_new_line(self):
		self.data_cept = bytearray()
		self.data_cept.extend(Cept.clear_line())
#		sys.stderr.write("self.y: '" + pprint.pformat(self.y) + "'\n")
#		sys.stderr.write("self.y % LINES_PER_PAGE: '" + pprint.pformat(self.y % LINES_PER_PAGE) + "'\n")
		self.x = 0
		self.y += 1

		if (self.y % LINES_PER_PAGE) == 0:
			self.resend_attributes()

#		s = str(self.y) + " "
#		self.data_cept.extend(Cept.from_str(s))
#		self.x += len(s)

	def create_new_line(self):
		self.lines_cept.append(self.data_cept)
		self.init_new_line()

	def set_italics_on(self):
		self.italics = True
		self.dirty = True
		return

	def set_italics_off(self):
		self.italics = False
		self.dirty = True
		return

	def set_bold_on(self):
		self.bold = True
		self.dirty = True
		return

	def set_bold_off(self):
		self.bold = False
		self.dirty = True
		return

	def set_link_on(self):
		self.link = True
		self.dirty = True
		return

	def set_link_off(self):
		self.link = False
		self.dirty = True
		return

	def resend_attributes(self):
#		sys.stderr.write("self.italics: " + pprint.pformat(["self.italics: ",self.italics , self.bold , self.link]) + "\n")
		if self.italics:
			self.data_cept.extend(Cept.set_fg_color(6))
		elif self.bold:
			self.data_cept.extend(Cept.set_fg_color(0))
		if self.link:
			self.data_cept.extend(Cept.underline_on())
			self.data_cept.extend(Cept.set_fg_color(4))
		if not self.italics and not self.bold and not self.link:
			self.data_cept.extend(Cept.set_fg_color(15))
			self.data_cept.extend(Cept.underline_off())
		self.dirty = False

	def newline(self):
		if self.x == 0 and self.y % LINES_PER_PAGE == 0:
			# no empty first lines
			return
		self.data_cept.extend(Cept.repeat(" ", 40 - self.x))
		self.create_new_line()

	def add_string(self, s):
		if self.dirty:
			self.resend_attributes()
		self.data_cept.extend(Cept.from_str(s))
#		sys.stderr.write("before self.x: " + pprint.pformat(self.x) + "\n")
		sys.stderr.write("adding: '" + pprint.pformat(s) + "'\n")
		sys.stderr.write("self.data_cept: " + pprint.pformat(self.data_cept) + "\n")

	def print(self, s):
		s = s.replace("\n", "")
		sys.stderr.write("s: " + pprint.pformat(s) + "\n")
		while s:
			index = s.find(" ")
			if index < 0:
				index = len(s)
				ends_in_space = False
			else:
				ends_in_space = True

			sys.stderr.write("decide self.x: " + pprint.pformat(self.x) + "\n")
			sys.stderr.write("decide index: " + pprint.pformat(index) + "\n")
			if index == 0 and self.x == 0:
				sys.stderr.write("A\n")
				# starts with space and we're at the start of a line
				# -> skip space
				pass
			elif index + self.x > 40:
				sys.stderr.write("B\n")
				# word doesn't fit, print it (plus the space)
				# into a new line
				if self.link:
					self.data_cept.extend(Cept.underline_off())
				self.data_cept.extend(Cept.repeat(" ", 40 - self.x))
				if self.link:
					self.data_cept.extend(Cept.underline_on())
				self.create_new_line()
				self.add_string(s[:index + 1])
				self.x += index
				if ends_in_space:
					self.x += 1
			elif ends_in_space and index + self.x + 1 == 40:
				sys.stderr.write("C\n")
				# space in last column
				# -> just print it, cursor will be in new line
				self.add_string(s[:index + 1])
				self.create_new_line()
			elif not ends_in_space and index + self.x == 40:
				sys.stderr.write("D\n")
				# character in last column, not followed by a space
				# -> just print it, cursor will be in new line
				self.add_string(s[:index])
				self.create_new_line()
			elif ends_in_space and index + self.x == 40:
				sys.stderr.write("E\n")
				# character in last column, followed by space
				# -> omit the space, cursor will be in new line
				self.add_string(s[:index])
				self.create_new_line()
			else:
				sys.stderr.write("F\n")
				self.add_string(s[:index + 1])
				self.x += len(s[:index + 1])
				if self.x == 40:
					self.create_new_line()
			s = s[index + 1:]

	def add_line(self, s):
		self.data_cept.extend(Cept.from_str(s))
		self.create_new_line()

	def print_heading(self, level, s):
		if level == 2:
			if (self.y + 1) % LINES_PER_PAGE == 0 or (self.y + 2) % LINES_PER_PAGE == 0:
				# don't draw double height title into
				# the last line or the one above
				self.data_cept.extend(b'\n')
				self.create_new_line()
			self.data_cept.extend(Cept.underline_off())
			self.data_cept.extend(Cept.clear_line())
			self.data_cept.extend(b'\n')
			self.data_cept.extend(Cept.clear_line())
			self.data_cept.extend(Cept.set_fg_color(0))
			self.data_cept.extend(Cept.double_height())
			self.data_cept.extend(Cept.from_str(s[:39]))
			self.data_cept.extend(b'\r\n')
			self.data_cept.extend(Cept.normal_size())
			self.data_cept.extend(Cept.set_fg_color(15))
			self.create_new_line()
			self.create_new_line()
		else:
			if (self.y + 1) % LINES_PER_PAGE == 0:
				# don't draw title into the last line
				self.data_cept.extend(b'\n')
				self.create_new_line()
			self.data_cept.extend(Cept.underline_on())
			self.data_cept.extend(Cept.set_fg_color(0))
			self.data_cept.extend(Cept.from_str(s[:39]))
			self.data_cept.extend(Cept.underline_off())
			self.data_cept.extend(Cept.set_fg_color(15))
			self.data_cept.extend(b'\r\n')
			self.create_new_line()
		return


class Wikipedia_UI:
	def insert_toc(soup, w, link_index, last_page, current_page):
		page_and_link_index_for_link = []
		for t1 in soup.contents[0].children:
			if t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				last_page = current_page
				current_page = int(w.y / LINES_PER_PAGE)
				if current_page != last_page:
					link_index = 10

				level = int(t1.name[1])
				indent = (level - 2) * "  "
				entry = indent + t1.contents[0].get_text()
				padded = entry + ("." * 36)
				padded = padded[:36]
				sys.stderr.write("len(padded): '" + pprint.pformat(len(padded)) + "'\n")
				w.add_line(padded + "[" + str(link_index) + "]")
				page_and_link_index_for_link.append((current_page, link_index))
				link_index += 1
		return (link_index, last_page, current_page, page_and_link_index_for_link)
		pageid

	def get_wikipedia_pageid_for_name(target_name):
		sys.stderr.write("NAME: " + pprint.pformat(target_name) + "\n")
		url = "https://de.wikipedia.org/w/api.php?action=query&titles=" + target_name + "&format=json"
		sys.stderr.write("URL: " + pprint.pformat(url) + "\n")
		contents = urllib.request.urlopen(url).read()
		j = json.loads(contents)
		sys.stderr.write("LINK: " + pprint.pformat(j) + "\n")
		pages = j["query"]["pages"]
#		pageid = None
#		for id in pages.keys():
#			if pages[id]["title"] == target_name:
#				pageid = id
#				break
#		if not pageid:
		pageid = list(pages.keys())[0]
		sys.stderr.write("pageid: " + pprint.pformat(pageid) + "\n")
		return pageid

	def create_wiki_page(wiki_id, subpage):
		sys.stderr.write("wiki_id: " + pprint.pformat(wiki_id) + "\n")
		sys.stderr.write("subpage: " + pprint.pformat(subpage) + "\n")
		url = "https://de.wikipedia.org/w/api.php?action=parse&prop=text&pageid=" + str(wiki_id) + "&format=json"
		sys.stderr.write("LINK: " + pprint.pformat(url) + "\n")
		contents = urllib.request.urlopen(url).read()
		j = json.loads(contents)
#		sys.stderr.write("LINK: " + pprint.pformat(j) + "\n")


		is_first_page = subpage == 0

		meta = {
			"publisher_color": 0
		}

		meta["clear_screen"] = is_first_page

		links_for_page = []

		#pprint.pprint(j)

		title = j["parse"]["title"]
		html = j["parse"]["text"]["*"]
		#html = html.replace("\n", "")
		#print(html)
		#exit(1)

		soup = BeautifulSoup(html, 'html.parser')

		# div are usually boxes
		[x.extract() for x in soup.contents[0].findAll('div')]
		# tables are usually boxes, (but not always!)
		[x.extract() for x in soup.contents[0].findAll('table')]

		# remove "[edit]" links
		for x in soup.contents[0].findAll('span'):
			if x.get("class") in [["mw-editsection"], ["mw-editsection-bracket"]]:
				x.extract()

		# remove citations
		for x in soup.findAll("a"):
			if x.get("href").startswith("#cite_note"):
				x.extract()

		# remove everything subscript: citation text, citation needed...
		for x in soup.findAll("sup"):
			x.extract()

		#print(soup.prettify())
		#exit(1)

		w = wikipedia_cept_page()
		w.lines_cept = []

#		sys.stderr.write("SOUP : " + pprint.pformat(soup.prettify()) + "\n")

		link_index = 10
		current_page = 0
		last_page = 0
		link_count = 0
		wiki_link_targets = []

		first_paragraph = True

		for t1 in soup.contents[0].children:
		#	pprint.pprint("XXXXX" + str(t1) + "\n\n\n")
		#	pprint.pprint(["name: ", t1.name])
			if t1.name == "p":
				# XXX move this up with the filters
				if t1.get_text().replace("\n", "") == "":
					continue
#				print("<p>")
				for t2 in t1.children:
					if t2.name is None:
						w.print(t2)
					elif t2.name == "span":
						w.print(t2.get_text())
					elif t2.name == "i":
						w.set_italics_on()
						w.print(t2.get_text())
						w.set_italics_off()
					elif t2.name == "b":
						w.set_bold_on()
						w.print(t2.get_text())
						w.set_bold_off()
					elif t2.name == "a":
						if t2["href"].startswith("/wiki/"): # ignore external links
							last_page = current_page
							current_page = int(w.y / LINES_PER_PAGE)
							if current_page != last_page:
								link_index = 10
								# TODO: this breaks if the link
								# goes across two pages!

							while len(wiki_link_targets) < current_page + 1:
								wiki_link_targets.append({})
							wiki_link_targets[current_page][link_index] = t2["href"][6:]

							link_text = t2.get_text() + " [" + str(link_index) + "]"
							w.set_link_on()
							w.print(link_text)
							link_index += 1
							w.set_link_off()
					else:
						pass
		#				print("UNKNOWN TAG: " + t2.name)
				w.newline()
				w.newline()

				if first_paragraph:
					first_paragraph = False
					(link_index, last_page, current_page, page_and_link_index_for_link) = Wikipedia_UI.insert_toc(soup, w, link_index, last_page, current_page)
					sys.stderr.write("page_and_link_index_for_link: " + pprint.pformat(page_and_link_index_for_link) + "\n")
					w.newline()

			elif t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				level = int(t1.name[1])
				w.print_heading(level, t1.contents[0].get_text())
				sys.stderr.write("HEADING page " + str(current_page) + ": " + pprint.pformat(t1.contents[0].get_text()) + "\n")
				if page_and_link_index_for_link: # only if there is a TOC
					(link_page, link_name) = page_and_link_index_for_link[link_count]
					link_count += 1
					while len(links_for_page) < link_page + 1:
						links_for_page.append({})
					links_for_page[link_page][str(link_name)] = "555" + str(wiki_id) + chr(0x61 + current_page)


		sys.stderr.write("wiki_link_targets: " + pprint.pformat(wiki_link_targets) + "\n")


		# create one page

		if len(links_for_page) < subpage + 1:
			meta["links"] = {}
		else:
			meta["links"] = links_for_page[subpage]


		links_for_this_page = wiki_link_targets[subpage]

		for l in links_for_this_page.keys():
			wikipedia_pageid = Wikipedia_UI.get_wikipedia_pageid_for_name(links_for_this_page[l])
			sys.stderr.write("wikipedia_pageid: " + pprint.pformat(wikipedia_pageid) + "\n")
			sys.stderr.write("str(l): " + pprint.pformat(str(l)) + "\n")
			if wikipedia_pageid: # ignore wiki pages that can't be found
				meta["links"][str(l)] = "555" + str(wikipedia_pageid)

		str_wikipedia = "Wikipedia"

		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_screen_bg_color(7))
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(Cept.double_height())
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.from_str(title[:39]))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.normal_size())
		data_cept.extend(b'\n')

		i = 2
		for line in w.lines_cept[subpage * LINES_PER_PAGE:(subpage + 1) * LINES_PER_PAGE]:
			sys.stderr.write("line " + str(i) + ": " + pprint.pformat(line) + "\n")
			data_cept.extend(line)
			i += 1

		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.from_str("0 < Back                        # > Next"))

		return (meta, data_cept)

	def create_page(pageid):
		if pageid.startswith("555"):
			return Wikipedia_UI.create_wiki_page(int(pageid[3:-1]), ord(pageid[-1]) - ord("a"))
		else:
			return None

