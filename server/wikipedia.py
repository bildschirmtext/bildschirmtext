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
from util import Util

# maps urls to json
http_cache = {}

class MediaWiki:
	wiki_url = None

	def __init__(self, wiki_url):
		if not wiki_url.endswith("/"):
			wiki_url += "/"
		self.wiki_url = wiki_url

	def fetch_json_from_server(self, url):
		j = http_cache.get(url)
		if not j :
			sys.stderr.write("URL: " + pprint.pformat(url) + "\n")
			contents = urllib.request.urlopen(url).read()
			j = json.loads(contents)
#			sys.stderr.write("RESPONSE: " + pprint.pformat(j) + "\n")
			http_cache[url] = j
		return j

	def title_for_search(self, search):
		sys.stderr.write("search: " + pprint.pformat(search) + "\n")
		j = self.fetch_json_from_server(self.wiki_url + "w/api.php?action=opensearch&search=" + urllib.parse.quote_plus(search) + "&format=json")
		links = j[3]
		if not links:
			return None
		return links[0][len(self.wiki_url + "wiki/"):]

	def wikiid_for_title(self, title):
		title = title.split("#")[0] # we ignore links to sections
		sys.stderr.write("title: " + pprint.pformat(title) + "\n")
		j = self.fetch_json_from_server(self.wiki_url + "w/api.php?action=query&titles=" + title + "&format=json")
		pages = j["query"]["pages"]
		wikiid = list(pages.keys())[0]
		sys.stderr.write("wikiid: " + pprint.pformat(wikiid) + "\n")
		return wikiid

	def html_for_wikiid(self, wikiid):
		j = self.fetch_json_from_server(self.wiki_url + "w/api.php?action=parse&prop=text&pageid=" + str(wikiid) + "&format=json")
		title = j["parse"]["title"]
		html = j["parse"]["text"]["*"]
		return (title, html)

wikipedias = {}

class Wikipedia(MediaWiki):
	wiki_url = None
	lang = None

	def __init__(self, lang):
		super(Wikipedia, self).__init__("https://" + lang + ".wikipedia.org/")
		self.lang = lang
		wikipedias[lang] = self

	def get(lang):
		wikipedia = wikipedias.get(lang)
		if wikipedia:
			return wikipedia
		return Wikipedia(lang)

	def title(self):
		return { "en": "Wikipedia - The Free Encyclopedia", "de": "Wikipedia - die freie Enzyklopädie" }.get(self.lang)

	def search_string(self):
		return { "en": "Search: ", "de": " Suche: " }.get(self.lang)


PAGEID_PREFIX = "55"

class Cept_page_from_HTML(Cept_page):
	link_index = None
	wiki_link_targets = []
	page_and_link_index_for_link = []
	first_paragraph = True
	link_count = 0
	links_for_page = []
	pageid_without_sheet = None
	soup = None
	ignore_lf = True

	def insert_toc(self, soup):
		self.page_and_link_index_for_link = []
		for t1 in soup.contents[0].children:
			if t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				if self.current_sheet() != self.prev_sheet:
					self.link_index = 10

				level = int(t1.name[1])
				# non-breaking space, otherwise it will be filtered at the beginning of lines
				indent = (level - 2) * "\xa0\xa0"
				entry = indent + t1.get_text().replace("\n", "")
				padded = entry + ("." * 36)
				padded = padded[:36]
				self.print(padded + "[" + str(self.link_index) + "]")
				self.page_and_link_index_for_link.append((self.current_sheet(), self.link_index))
				self.link_index += 1

	def insert_html_tags(self, tags):
		for t1 in tags:
			if t1.name == "p":
				self.insert_html_tags(t1.children)
				self.print("\n")

				if self.first_paragraph:
					self.first_paragraph = False
					self.insert_toc(self.soup)
#					sys.stderr.write("self.page_and_link_index_for_link: " + pprint.pformat(self.page_and_link_index_for_link) + "\n")
					self.print("\n")

			elif t1.name in ["h2", "h3", "h4", "h5", "h6"]:
				level = int(t1.name[1])
				self.print_heading(level, t1.contents[0].get_text().replace("\n", ""))
#				sys.stderr.write("HEADING page " + str(page.current_sheet()) + ": " + pprint.pformat(t1.contents[0].get_text()) + "\n")
				if self.page_and_link_index_for_link: # only if there is a TOC
					(link_page, link_name) = self.page_and_link_index_for_link[self.link_count]
					self.link_count += 1
					while len(self.links_for_page) < link_page + 1:
						self.links_for_page.append({})
					self.links_for_page[link_page][str(link_name)] = self.pageid_without_sheet + chr(0x61 + self.current_sheet())

			elif t1.name is None:
				self.print(t1, self.ignore_lf)
			elif t1.name == "span":
				self.print(t1.get_text(), self.ignore_lf)
			elif t1.name == "i":
				self.set_italics_on()
				self.print(t1.get_text(), self.ignore_lf)
				self.set_italics_off()
			elif t1.name == "b":
				self.set_bold_on()
				self.print(t1.get_text(), self.ignore_lf)
				self.set_bold_off()
			elif t1.name == "a":
				if t1["href"].startswith("/wiki/"): # links to different article
					if self.current_sheet() != self.prev_sheet:
						self.link_index = 10
						# TODO: this breaks if the link
						# goes across two sheets!

					while len(self.wiki_link_targets) < self.current_sheet() + 1:
						self.wiki_link_targets.append({})
					self.wiki_link_targets[self.current_sheet()][self.link_index] = t1["href"][6:]

					link_text = t1.get_text().replace("\n", "") + " [" + str(self.link_index) + "]"
					self.set_link_on()
					self.print(link_text)
					self.link_index += 1
					self.set_link_off()
				else: # link to section or external link, just print the text
					self.print(t1.get_text(), self.ignore_lf)

			elif t1.name == "ul":
				self.insert_html_tags(t1.children)
			elif t1.name == "ol":
				self.insert_html_tags(t1.children)
			elif t1.name == "code":
				self.set_code_on()
				self.insert_html_tags(t1.children)
				self.set_code_off()
			elif t1.name == "li":
				# TODO indentation
				self.print("* ") # TODO: ordered list
				self.insert_html_tags(t1.children)
				self.print("\n")
			elif t1.name == "pre":
				self.ignore_lf = False
				self.insert_html_tags(t1.children)
				self.ignore_lf = True
			else:
				sys.stderr.write("ignoring tag: " + pprint.pformat(t1.name) + "\n")

#		sys.stderr.write("self.wiki_link_targets: " + pprint.pformat(self.wiki_link_targets) + "\n")


class Wikipedia_UI:
	def pageid_prefix_for_lang(lang):
		return PAGEID_PREFIX + str({ "en": 0, "de": 5 }.get(lang))

	def lang_from_langdigit(langdigit):
		return { 0: "en", 5: "de" }.get(langdigit)

	def get_pageid_for_title(lang, title):
		mediawiki = Wikipedia.get(lang)
		wikiid = mediawiki.wikiid_for_title(title)
		if wikiid:
			return Wikipedia_UI.pageid_prefix_for_lang(lang) + str(wikiid)
		else:
			return None

	def simplify_html(soup):
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

		return soup

	def create_article_page(mediawiki, wikiid, sheet_number):
		is_first_page = sheet_number == 0

		# get HTML from server
		(title, html) = mediawiki.html_for_wikiid(wikiid)

		soup = BeautifulSoup(html, 'html.parser')

		for tag in soup.contents[0].findAll('div'):
			if tag.get("class") == ["redirectMsg"]:
				sys.stderr.write("tag: " + pprint.pformat(tag) + "\n")
				for tag in tag.findAll('a'):
					link = tag.get("href")
					title = link[6:]
					sys.stderr.write("a: " + pprint.pformat(title) + "\n")
					wikiid = Wikipedia_UI.get_pageid_for_title(mediawiki.lang, title)[len(Wikipedia_UI.pageid_prefix_for_lang(mediawiki.lang)):]
					sys.stderr.write("wikiid: " + pprint.pformat(wikiid) + "\n")
					return Wikipedia_UI.create_article_page(mediawiki, wikiid, sheet_number)

		# extract URL of first image
		image_url = None
		for tag in soup.contents[0].findAll('img'):
			if tag.get("class") == ["thumbimage"]:
				image_url = "https:" + tag.get("src")
				(image_palette, image_drcs, image_chars) = Image_UI.cept_from_image(image_url)
				break

		soup = Wikipedia_UI.simplify_html(soup)

		page = Cept_page()


		#
		# conversion
		#
		page = Cept_page_from_HTML()

		# tell page renderer to leave room for the image in the top right of the first sheet
		if image_url is not None:
			page.title_image_width = len(image_chars[0])
			page.title_image_height = len(image_chars) - 2 # image draws 2 characters into title area

		# XXX why is this necessary???
		page.lines_cept = []

		page.soup = soup
		page.link_index = 10
		page.pageid_without_sheet = Wikipedia_UI.pageid_prefix_for_lang(mediawiki.lang) + str(wikiid)
		page.insert_html_tags(soup.contents[0].children)

		# create one page

		if sheet_number > page.number_of_sheets() - 1:
			return None

		meta = {
			"publisher_color": 0
		}

		if len(page.links_for_page) < sheet_number + 1:
			meta["links"] = {}
		else:
			meta["links"] = page.links_for_page[sheet_number]

		meta["links"]["0"] = Wikipedia_UI.pageid_prefix_for_lang(mediawiki.lang)

		if len(page.wiki_link_targets) < sheet_number + 1:
			links_for_this_page = {}
		else:
			links_for_this_page = page.wiki_link_targets[sheet_number]

		for l in links_for_this_page.keys():
			meta["links"][str(l)] = "call:Wikipedia_UI.callback_get_pageid_for_title:" + mediawiki.lang + "/" + str(links_for_this_page[l])

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
#		sys.stderr.write("page.cept_for_sheet(sheet_number): " + pprint.pformat(page.cept_for_sheet(sheet_number)) + "\n")

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

	def create_search_page(wikipedia, basedir):
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
						"column": 9,
						"height": 1,
						"width": 31,
						"bgcolor": 0,
						"fgcolor": 15,
						"validate": "call:Wikipedia_UI.callback_validate_search:" + wikipedia.lang
					}
				],
				"confirm": False,
				"target": "call:Wikipedia_UI.callback_search:" + wikipedia.lang
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
		data_cept.extend(Cept.from_str(wikipedia.title()))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.normal_size())
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_cursor(18, 1))
		data_cept.extend(Cept.set_fg_color(0))
		data_cept.extend(Cept.from_str(wikipedia.search_string()))
		# trick: show cursor now so that user knows they can enter text, even though more
		# data is loading
		data_cept.extend(Cept.show_cursor())

		(palette, drcs, chars) = Image_UI.cept_from_image(basedir + "wikipedia.png", colors = 4)

		data_cept.extend(Cept.define_palette(palette))
		data_cept.extend(drcs)

		data_cept.extend(Cept.hide_cursor())

		y = 6
		for l in chars:
			data_cept.extend(Cept.set_cursor(y, int((41 - len(chars[0])) / 2)))
			data_cept.extend(Cept.load_g0_drcs())
			data_cept.extend(l)
			y += 1

		return (meta, data_cept)

	def callback_get_pageid_for_title(cls, dummy, lang_title):
		index = lang_title.find("/")
		return Wikipedia_UI.get_pageid_for_title(lang_title[:index], lang_title[index + 1:])

	def callback_validate_search(cls, input_data, lang):
		mediawiki = Wikipedia.get(lang)
		pageid = mediawiki.title_for_search(input_data["search"])
		if not pageid:
			msg = Util.create_custom_system_message("Suchbegriff nicht gefunden! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	def callback_search(cls, s, lang):
		mediawiki = Wikipedia.get(lang)
		title = mediawiki.title_for_search(s["search"])
		sys.stderr.write("TITLE: " + pprint.pformat(title) + "\n")
		return Wikipedia_UI.get_pageid_for_title(lang, title)

	def create_page(pageid, basedir):
		if re.search("^" + PAGEID_PREFIX + "\d", pageid):
			wikipedia = Wikipedia.get(Wikipedia_UI.lang_from_langdigit(int(pageid[2])))
			if len(pageid) == 4:
				return Wikipedia_UI.create_search_page(wikipedia, basedir)
			else:
				return Wikipedia_UI.create_article_page(wikipedia, int(pageid[3:-1]), ord(pageid[-1]) - ord("a"))
		else:
			return None

