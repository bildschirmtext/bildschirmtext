import sys
import re
import json
import pprint
import urllib.parse
import urllib.request

from bs4 import BeautifulSoup

from cept import Cept
from cept import Cept_page

WIKI_PREFIX = "https://de.wikipedia.org/"
WIKI_PREFIX_W = WIKI_PREFIX + "w/"
WIKI_PREFIX_WIKI = WIKI_PREFIX + "wiki/"

class Wikipedia_UI:
	def insert_toc(soup, w, link_index, last_page, current_page):
		page_and_link_index_for_link = []
		for t1 in soup.contents[0].children:
			if t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				last_page = current_page
				current_page = int(w.y / w.lines_per_page)
				if current_page != last_page:
					link_index = 10

				level = int(t1.name[1])
				# non-breaking space, otherwise it will be filtered at the beginning of lines
				indent = (level - 2) * "\xa0\xa0"
				entry = indent + t1.contents[0].get_text().replace("\n", "")
				padded = entry + ("." * 36)
				padded = padded[:36]
				sys.stderr.write("len(padded): '" + pprint.pformat(len(padded)) + "'\n")
				w.print(padded + "[" + str(link_index) + "]")
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

	def create_wiki_page(wiki_id, subpage):
		sys.stderr.write("wiki_id: " + pprint.pformat(wiki_id) + "\n")
		sys.stderr.write("subpage: " + pprint.pformat(subpage) + "\n")
		url = WIKI_PREFIX_W + "api.php?action=parse&prop=text&pageid=" + str(wiki_id) + "&format=json"
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

		w = Cept_page()
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
						w.print(t2.replace("\n", ""))
					elif t2.name == "span":
						w.print(t2.get_text().replace("\n", ""))
					elif t2.name == "i":
						w.set_italics_on()
						w.print(t2.get_text().replace("\n", ""))
						w.set_italics_off()
					elif t2.name == "b":
						w.set_bold_on()
						w.print(t2.get_text().replace("\n", ""))
						w.set_bold_off()
					elif t2.name == "a":
						if t2["href"].startswith("/wiki/"): # ignore external links
							last_page = current_page
							current_page = int(w.y / w.lines_per_page)
							if current_page != last_page:
								link_index = 10
								# TODO: this breaks if the link
								# goes across two pages!

							while len(wiki_link_targets) < current_page + 1:
								wiki_link_targets.append({})
							wiki_link_targets[current_page][link_index] = t2["href"][6:]

							link_text = t2.get_text().replace("\n", "") + " [" + str(link_index) + "]"
							w.set_link_on()
							w.print(link_text)
							link_index += 1
							w.set_link_off()
					else:
						pass
		#				print("UNKNOWN TAG: " + t2.name)
				w.print("\n")

				if first_paragraph:
					first_paragraph = False
					(link_index, last_page, current_page, page_and_link_index_for_link) = Wikipedia_UI.insert_toc(soup, w, link_index, last_page, current_page)
					sys.stderr.write("page_and_link_index_for_link: " + pprint.pformat(page_and_link_index_for_link) + "\n")
					w.print("\n")

			elif t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				level = int(t1.name[1])
				w.print_heading(level, t1.contents[0].get_text().replace("\n", ""))
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
			meta["links"][str(l)] = "call:Wikipedia_UI.get_wikipedia_pageid_for_name:" + str(links_for_this_page[l])

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
		for line in w.lines_cept[subpage * w.lines_per_page:(subpage + 1) * w.lines_per_page]:
			sys.stderr.write("line " + str(i) + ": " + pprint.pformat(line) + "\n")
			data_cept.extend(line)
			i += 1

		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color(0))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.from_str("0 < Back                        # > Next"))

		return (meta, data_cept)

	def create_search_page():
		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"inputs": {
				"fields": [
					{
						"name": "search",
						"line": 13,
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
		data_cept.extend(Cept.set_cursor(13, 1))
		data_cept.extend(Cept.set_fg_color(0))
		data_cept.extend(Cept.from_str("Wikipedia Search:"))

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

	def create_page(pageid):
		if pageid == "555a":
			return Wikipedia_UI.create_search_page()
		elif pageid.startswith("555"):
			return Wikipedia_UI.create_wiki_page(int(pageid[3:-1]), ord(pageid[-1]) - ord("a"))
		else:
			return None

