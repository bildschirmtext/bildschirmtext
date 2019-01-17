import sys
import re
import json
import pprint
import urllib.parse
import urllib.request

from bs4 import BeautifulSoup

from cept import Cept
from cept import Cept_page
from image import Image_UI

#WIKI_PREFIX = "https://en.wikipedia.org/"
WIKI_PREFIX = "https://de.wikipedia.org/"
WIKI_PREFIX_W = WIKI_PREFIX + "w/"
WIKI_PREFIX_WIKI = WIKI_PREFIX + "wiki/"

class Wikipedia_UI:
	def insert_toc(soup, page, link_index):
		page_and_link_index_for_link = []
		for t1 in soup.contents[0].children:
			if t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				if page.current_sheet() != page.prev_sheet:
					link_index = 10

				level = int(t1.name[1])
				# non-breaking space, otherwise it will be filtered at the beginning of lines
				indent = (level - 2) * "\xa0\xa0"
				entry = indent + t1.contents[0].get_text().replace("\n", "")
				padded = entry + ("." * 36)
				padded = padded[:36]
				page.print(padded + "[" + str(link_index) + "]")
				page_and_link_index_for_link.append((page.current_sheet(), link_index))
				link_index += 1
		return (link_index, page_and_link_index_for_link)
		pageid

	def get_wikipedia_pageid_for_name(cls, target_name):
		sys.stderr.write("NAME: " + pprint.pformat(target_name) + "\n")
		url = WIKI_PREFIX_W + "api.php?action=query&titles=" + target_name + "&format=json"
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
		return "555" + str(pageid)

	def create_wiki_page(wiki_id, sheet_number):
		is_first_page = sheet_number == 0

		# get HTML from server
		url = WIKI_PREFIX_W + "api.php?action=parse&prop=text&pageid=" + str(wiki_id) + "&format=json"
		contents = urllib.request.urlopen(url).read()
		j = json.loads(contents)

		title = j["parse"]["title"]
		html = j["parse"]["text"]["*"]

		soup = BeautifulSoup(html, 'html.parser')

		# extract URL of first image
		image_url = None
		for tag in soup.contents[0].findAll('img'):
			if tag.get("class") == ["thumbimage"]:
				image_url = "https:" + tag.get("src")
				(image_palette, image_drcs, image_chars) = Image_UI.cept_from_image(image_url)
				break

		# div are usually boxes -> remove
		[tag.extract() for tag in soup.contents[0].findAll('div')]
		# tables are usually boxes, (but not always!) -> remove
		[tag.extract() for tag in soup.contents[0].findAll('table')]

		# remove "[edit]" links
		for tag in soup.contents[0].findAll('span'):
			if tag.get("class") in [["mw-editsection"], ["mw-editsection-bracket"]]:
				tag.extract()

		# remove citations
		for tag in soup.findAll("a"):
			if tag.get("href").startswith("#cite_note"):
				tag.extract()

		# remove everything subscript: citation text, citation needed...
		for tag in soup.findAll("sup"):
			tag.extract()

		for tag in soup.findAll("p"):
			if tag.get_text().replace("\n", "") == "":
				tag.extract()

		page = Cept_page()

		# tell page renderer to leave room for the image in the top right of the first sheet
		if image_url is not None:
			page.title_image_width = len(image_chars[0])
			page.title_image_height = len(image_chars) - 2 # image draws 2 characters into title area

		# XXX why is this necessary???
		page.lines_cept = []

		link_index = 10
		link_count = 0
		wiki_link_targets = []
		links_for_page = []

		first_paragraph = True

		for t1 in soup.contents[0].children:
			if t1.name == "p":
				for t2 in t1.children:
					if t2.name is None:
						page.print(t2, True)
					elif t2.name == "span":
						page.print(t2.get_text(), True)
					elif t2.name == "i":
						page.set_italics_on()
						page.print(t2.get_text(), True)
						page.set_italics_off()
					elif t2.name == "b":
						page.set_bold_on()
						page.print(t2.get_text(), True)
						page.set_bold_off()
					elif t2.name == "a" and t2["href"].startswith("/wiki/"): # ignore external links
						if page.current_sheet() != page.prev_sheet:
							link_index = 10
							# TODO: this breaks if the link
							# goes across two pages!

						while len(wiki_link_targets) < page.current_sheet() + 1:
							wiki_link_targets.append({})
						wiki_link_targets[page.current_sheet()][link_index] = t2["href"][6:]

						link_text = t2.get_text().replace("\n", "") + " [" + str(link_index) + "]"
						page.set_link_on()
						page.print(link_text)
						link_index += 1
						page.set_link_off()
					else:
						pass
				page.print("\n")

				if first_paragraph:
					first_paragraph = False
					(link_index, page_and_link_index_for_link) = Wikipedia_UI.insert_toc(soup, page, link_index)
#					sys.stderr.write("page_and_link_index_for_link: " + pprint.pformat(page_and_link_index_for_link) + "\n")
					page.print("\n")

			elif t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				level = int(t1.name[1])
				page.print_heading(level, t1.contents[0].get_text().replace("\n", ""))
#				sys.stderr.write("HEADING page " + str(page.current_sheet()) + ": " + pprint.pformat(t1.contents[0].get_text()) + "\n")
				if page_and_link_index_for_link: # only if there is a TOC
					(link_page, link_name) = page_and_link_index_for_link[link_count]
					link_count += 1
					while len(links_for_page) < link_page + 1:
						links_for_page.append({})
					links_for_page[link_page][str(link_name)] = "555" + str(wiki_id) + chr(0x61 + page.current_sheet())


#		sys.stderr.write("wiki_link_targets: " + pprint.pformat(wiki_link_targets) + "\n")


		# create one page

		if sheet_number > page.number_of_sheets() - 1:
			return None

		meta = {
			"publisher_color": 0
		}

		if len(links_for_page) < sheet_number + 1:
			meta["links"] = {}
		else:
			meta["links"] = links_for_page[sheet_number]

		meta["links"]["0"] = "555"

		if len(wiki_link_targets) < sheet_number + 1:
			links_for_this_page = {}
		else:
			links_for_this_page = wiki_link_targets[sheet_number]

		for l in links_for_this_page.keys():
			meta["links"][str(l)] = "call:Wikipedia_UI.get_wikipedia_pageid_for_name:" + str(links_for_this_page[l])

		meta["clear_screen"] = is_first_page

		# print the page title (only on the first sheet)
		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())

		if is_first_page:
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
		else:
			# on sheets b+, we need to clear the image area
			if image_url:
				for i in range(0, 2):
					data_cept.extend(Cept.set_cursor(3 + i, 41 - len(image_chars[0])))
					data_cept.extend(Cept.repeat(" ", len(image_chars[0])))

		# print navigation
		# * on sheet 0, so we don't have to print it again on later sheets
		# * on the last sheet, because it doesn't show the "#" text
		# * on the second last sheet, because navigating back from the last one needs to show "#" again
		if sheet_number == 0 or sheet_number >= page.number_of_sheets() - 2:
			data_cept.extend(Cept.set_cursor(23, 1))
			data_cept.extend(Cept.set_line_bg_color(0))
			data_cept.extend(Cept.set_fg_color(7))
			data_cept.extend(Cept.from_str("0 < Back"))
			s = "# > Next"
			data_cept.extend(Cept.set_cursor(23, 41 - len(s)))
			if sheet_number == page.number_of_sheets() - 1:
				data_cept.extend(Cept.repeat(" ", len(s)))
			else:
				data_cept.extend(Cept.from_str(s))

		data_cept.extend(Cept.set_cursor(5, 1))

		# add text
		data_cept.extend(page.cept_for_sheet(sheet_number))
		sys.stderr.write("page.cept_for_sheet(sheet_number): " + pprint.pformat(page.cept_for_sheet(sheet_number)) + "\n")

		# transfer image on first sheet
		if is_first_page and image_url:
			# placeholder rectangle
			for y in range(0, len(image_chars)):
				data_cept.extend(Cept.set_cursor(3 + y, 41 - len(image_chars[0])))
				data_cept.extend(Cept.set_bg_color(15))
				data_cept.extend(Cept.repeat(" ", len(image_chars[0])))
			# palette
			data_cept.extend(Cept.define_palette(image_palette))
			# DRCS
			data_cept.extend(image_drcs)
			# draw characters
			i = 0
			for l in image_chars:
				data_cept.extend(Cept.set_cursor(3 + i, 41 - len(image_chars[0])))
				data_cept.extend(Cept.load_g0_drcs())
				data_cept.extend(l)
				data_cept.extend(b'\r\n')
				i += 1

		return (meta, data_cept)

	def create_search_page(basedir):
		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"inputs": {
				"fields": [
					{
						"name": "search",
						"line": 18,
						"column": 19,
						"height": 1,
						"width": 20,
						"bgcolor": 0,
						"fgcolor": 15
					}
				],
				"confirm": False,
				"target": "call:Wikipedia_UI.search"
			},
			"publisher_color": 0
		}

		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_screen_bg_color(7))
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(Cept.double_height())
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.from_str("Wikipedia - The Free Encyclopedia"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.normal_size())
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_cursor(18, 1))
		data_cept.extend(Cept.set_fg_color(0))
		data_cept.extend(Cept.from_str("Wikipedia Search: "))
		# trick: show cursor now so that user knows they can enter text, even though more
		# data is loading
		data_cept.extend(Cept.show_cursor())

		(palette, drcs, chars) = Image_UI.cept_from_image(basedir + "wikipedia.png")

		data_cept.extend(Cept.define_palette(palette))
		data_cept.extend(drcs)

		y = 6
		for l in chars:
			data_cept.extend(Cept.set_cursor(y, int((41 - 2 * len(chars[0])) / 2)))
			data_cept.extend(Cept.load_g0_drcs())
			data_cept.extend(Cept.double_size())
			data_cept.extend(l)
			y += 2

		return (meta, data_cept)

	def search(cls, s):
		sys.stderr.write("s: " + pprint.pformat(s) + "\n")
		url = WIKI_PREFIX_W + "api.php?action=opensearch&search=" + urllib.parse.quote_plus(s["search"]) + "&format=json"
		sys.stderr.write("URL: " + pprint.pformat(url) + "\n")
		contents = urllib.request.urlopen(url).read()
		j = json.loads(contents)
#		sys.stderr.write("RESPONSE: " + pprint.pformat(j) + "\n")
		links = j[3]
		first_link = j[3][0]
		first_name = first_link[len(WIKI_PREFIX_WIKI):]
		sys.stderr.write("LINK: " + pprint.pformat(first_name) + "\n")
		return Wikipedia_UI.get_wikipedia_pageid_for_name(None, first_name)

	def create_page(pageid, basedir):
		if pageid == "555a":
			return Wikipedia_UI.create_search_page(basedir)
		elif pageid.startswith("555"):
			return Wikipedia_UI.create_wiki_page(int(pageid[3:-1]), ord(pageid[-1]) - ord("a"))
		else:
			return None

