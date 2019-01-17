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
	def insert_toc(soup, page, link_index, last_page, current_page):
		page_and_link_index_for_link = []
		for t1 in soup.contents[0].children:
			if t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				last_page = current_page
				current_page = int(page.y / page.lines_per_page)
				if current_page != last_page:
					link_index = 10

				level = int(t1.name[1])
				# non-breaking space, otherwise it will be filtered at the beginning of lines
				indent = (level - 2) * "\xa0\xa0"
				entry = indent + t1.contents[0].get_text().replace("\n", "")
				padded = entry + ("." * 36)
				padded = padded[:36]
				page.print(padded + "[" + str(link_index) + "]")
				page_and_link_index_for_link.append((current_page, link_index))
				link_index += 1
		return (link_index, last_page, current_page, page_and_link_index_for_link)
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
		for x in soup.contents[0].findAll('img'):
			if x.get("class") == ["thumbimage"]:
				image_url = "https:" + x.get("src")
				(image_palette, image_drcs, image_chars) = Image_UI.cept_from_image(image_url)
				break

		# div are usually boxes -> remove
		[x.extract() for x in soup.contents[0].findAll('div')]
		# tables are usually boxes, (but not always!) -> remove
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

		page = Cept_page()

		# tell page renderer to leave room for the image in the top right of the first sheet
		if image_url is not None:
			page.title_image_width = len(image_chars[0])
			page.title_image_height = len(image_chars) - 2 # image draws 2 characters into title area

		# XXX why is this necessary???
		page.lines_cept = []

		link_index = 10
		current_page = 0
		last_page = 0
		link_count = 0
		wiki_link_targets = []
		links_for_page = []

		first_paragraph = True

		for t1 in soup.contents[0].children:
			if t1.name == "p":
				# XXX move this up with the filters
				if t1.get_text().replace("\n", "") == "":
					continue
#				print("<p>")
				for t2 in t1.children:
					if t2.name is None:
						page.print(t2.replace("\n", ""))
					elif t2.name == "span":
						page.print(t2.get_text().replace("\n", ""))
					elif t2.name == "i":
						page.set_italics_on()
						page.print(t2.get_text().replace("\n", ""))
						page.set_italics_off()
					elif t2.name == "b":
						page.set_bold_on()
						page.print(t2.get_text().replace("\n", ""))
						page.set_bold_off()
					elif t2.name == "a":
						if t2["href"].startswith("/wiki/"): # ignore external links
							last_page = current_page
							current_page = int(page.y / page.lines_per_page)
							if current_page != last_page:
								link_index = 10
								# TODO: this breaks if the link
								# goes across two pages!

							while len(wiki_link_targets) < current_page + 1:
								wiki_link_targets.append({})
							wiki_link_targets[current_page][link_index] = t2["href"][6:]

							link_text = t2.get_text().replace("\n", "") + " [" + str(link_index) + "]"
							page.set_link_on()
							page.print(link_text)
							link_index += 1
							page.set_link_off()
					else:
						pass
		#				print("UNKNOWN TAG: " + t2.name)
				page.print("\n")

				if first_paragraph:
					first_paragraph = False
					(link_index, last_page, current_page, page_and_link_index_for_link) = Wikipedia_UI.insert_toc(soup, page, link_index, last_page, current_page)
					sys.stderr.write("page_and_link_index_for_link: " + pprint.pformat(page_and_link_index_for_link) + "\n")
					page.print("\n")

			elif t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				level = int(t1.name[1])
				page.print_heading(level, t1.contents[0].get_text().replace("\n", ""))
				sys.stderr.write("HEADING page " + str(current_page) + ": " + pprint.pformat(t1.contents[0].get_text()) + "\n")
				if page_and_link_index_for_link: # only if there is a TOC
					(link_page, link_name) = page_and_link_index_for_link[link_count]
					link_count += 1
					while len(links_for_page) < link_page + 1:
						links_for_page.append({})
					links_for_page[link_page][str(link_name)] = "555" + str(wiki_id) + chr(0x61 + current_page)


#		sys.stderr.write("wiki_link_targets: " + pprint.pformat(wiki_link_targets) + "\n")


		# create one page

		meta = {
			"publisher_color": 0
		}

		if len(links_for_page) < sheet_number + 1:
			meta["links"] = {}
		else:
			meta["links"] = links_for_page[sheet_number]


		links_for_this_page = wiki_link_targets[sheet_number]

		for l in links_for_this_page.keys():
			meta["links"][str(l)] = "call:Wikipedia_UI.get_wikipedia_pageid_for_name:" + str(links_for_this_page[l])

		meta["clear_screen"] = is_first_page

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

		if not is_first_page and image_url:
			# clear image
			for i in range(0, 2):
				data_cept.extend(Cept.set_cursor(3 + i, 41 - len(image_chars[0])))
				data_cept.extend(Cept.repeat(" ", len(image_chars[0])))

		i = 2
		for line in page.lines_cept[sheet_number * page.lines_per_page:(sheet_number + 1) * page.lines_per_page]:
			sys.stderr.write("line " + str(i) + ": " + pprint.pformat(line) + "\n")
			data_cept.extend(line)
			i += 1

		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.from_str("0 < Back                        # > Next"))

		if is_first_page and image_url:
			for y in range(0, len(image_chars)):
				data_cept.extend(Cept.set_cursor(3 + y, 41 - len(image_chars[0])))
				data_cept.extend(Cept.set_bg_color(15))
				data_cept.extend(Cept.repeat(" ", len(image_chars[0])))

			data_cept.extend(Cept.define_palette(image_palette))
			data_cept.extend(image_drcs)

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

