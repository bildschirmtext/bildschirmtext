import sys
import re
import json
import pprint
import urllib.parse
import urllib.request

from bs4 import BeautifulSoup

from cept import Cept
from cept import Cept_page
from cept import Unscii
from image import Image_UI
from util import Util


mediawiki_from_wiki_url = {}
mediawiki_from_id = []

class MediaWiki:
	wiki_url = None
	title = None
	search_string = None
	pageid_prefix = None
	id = None
	api_prefix = "/wiki/"
	article_prefix = "/wiki/index.php/"

	# maps urls to json
	http_cache = {}

	def __init__(self, wiki_url):
		if wiki_url.endswith("/"):
			wiki_url = wiki_url[:-1]
		self.wiki_url = wiki_url
		self.id = len(mediawiki_from_id)
		mediawiki_from_id.append(self)

	def fetch_json_from_server(self, url):
		j = self.http_cache.get(url)
		if not j :
			sys.stderr.write("URL: " + pprint.pformat(url) + "\n")
			contents = urllib.request.urlopen(url).read()
			j = json.loads(contents)
#			sys.stderr.write("RESPONSE: " + pprint.pformat(j) + "\n")
			self.http_cache[url] = j
		return j

	def title_for_search(self, search):
		sys.stderr.write("search: " + pprint.pformat(search) + "\n")
		j = self.fetch_json_from_server(self.wiki_url + self.api_prefix + "api.php?action=opensearch&search=" + urllib.parse.quote_plus(search) + "&format=json")
		links = j[3]
		if not links:
			return None
		sys.stderr.write("self.wiki_url: " + pprint.pformat(self.wiki_url) + "\n")
		sys.stderr.write("self.article_prefix: " + pprint.pformat(self.article_prefix) + "\n")
		return links[0][len(self.base_url() + self.article_prefix):]

	def wikiid_for_title(self, title):
		title = title.split("#")[0] # we ignore links to sections
		sys.stderr.write("title: " + pprint.pformat(title) + "\n")
		j = self.fetch_json_from_server(self.wiki_url + self.api_prefix + "api.php?action=query&titles=" + title + "&format=json")
		pages = j["query"]["pages"]
		wikiid = list(pages.keys())[0]
		sys.stderr.write("wikiid: " + pprint.pformat(wikiid) + "\n")
		return wikiid

	def pageid_for_title(self, title):
		wikiid = self.wikiid_for_title(title)
		if wikiid:
			sys.stderr.write("self.pageid_prefix: " + pprint.pformat(self.pageid_prefix) + "\n")
			return self.pageid_prefix + str(wikiid)
		else:
			return None

	def html_for_wikiid(self, wikiid):
		j = self.fetch_json_from_server(self.wiki_url + self.api_prefix + "api.php?action=parse&prop=text&pageid=" + str(wikiid) + "&format=json")
		title = j["parse"]["title"]
		html = j["parse"]["text"]["*"]
		return (title, html)

	def base_url(self):
		p = urllib.parse.urlparse(self.wiki_url)
		return '{uri.scheme}://{uri.netloc}'.format(uri=p)

	def base_scheme(self):
		p = urllib.parse.urlparse(self.wiki_url)
		return '{uri.scheme}://'.format(uri=p)

	def get_from_wiki_url(wiki_url):
		mediawiki = mediawiki_from_wiki_url.get(wiki_url)
		if mediawiki:
			return mediawiki
		return MediaWiki(wiki_url)

	def get_from_id(id):
		sys.stderr.write("mediawiki_from_wiki_url: " + pprint.pformat(mediawiki_from_wiki_url) + "\n")
		return mediawiki_from_id[id]

class MediaWiki_UI:

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

		# handle redirects
		for tag in soup.contents[0].findAll('div'):
			if tag.get("class") == ["redirectMsg"]:
				sys.stderr.write("tag: " + pprint.pformat(tag) + "\n")
				for tag in tag.findAll('a'):
					link = tag.get("href")
					title = link[6:]
					sys.stderr.write("a: " + pprint.pformat(title) + "\n")
					wikiid = mediawiki.wikiid_for_title(title)
					sys.stderr.write("wikiid: " + pprint.pformat(wikiid) + "\n")
					return MediaWiki_UI.create_article_page(mediawiki, wikiid, sheet_number)

		# extract URL of first image
		image_url = None
		for tag in soup.contents[0].findAll('img'):
			if tag.get("class") == ["thumbimage"]:
				image_url = tag.get("src")
				if image_url.startswith("//"): # same scheme
					image_url = mediawiki.base_scheme() + image_url[2:]
				if image_url.startswith("/"): # same scheme + host
					image_url = mediawiki.base_url() + image_url
				break

		soup = MediaWiki_UI.simplify_html(soup)

		# try conversion without image to estimate an upper bound
		# on the number of DRCS characters needed on the first page
		page = Cept_page()
		page.article_prefix = mediawiki.article_prefix
		# XXX why is this necessary???
		page.lines_cept = [ [] ]
		page.soup = soup
		page.link_index = 10
		page.pageid_base = mediawiki.pageid_prefix + str(wikiid)
		page.insert_html_tags(soup.contents[0].children)
		# and create the image with the remaining characters
		image = Image_UI(image_url, drcs_start = page.drcs_start_for_first_sheet)

		#
		# conversion
		#
		page = Cept_page()
		page.title = title
		page.article_prefix = mediawiki.article_prefix

		# tell page renderer to leave room for the image in the top right of the first sheet
		if image is not None:
			page.title_image_width = len(image.chars[0])
			page.title_image_height = len(image.chars) - 2 # image draws 2 characters into title area

		# XXX why is this necessary???
		page.lines_cept = [ [] ]

		page.soup = soup
		page.link_index = 10
		page.pageid_base = mediawiki.pageid_prefix + str(wikiid)
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

		meta["links"]["0"] = mediawiki.pageid_prefix

		if len(page.wiki_link_targets) < sheet_number + 1:
			links_for_this_page = {}
		else:
			links_for_this_page = page.wiki_link_targets[sheet_number]

		for l in links_for_this_page.keys():
			meta["links"][str(l)] = "call:MediaWiki_UI.callback_pageid_for_title:" + str(mediawiki.id) + "|" + str(links_for_this_page[l])

		meta["clear_screen"] = is_first_page

		data_cept = page.complete_cept_for_sheet(sheet_number, image)

		return (meta, data_cept)

	def create_search_page(mediawiki, basedir):
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
						"validate": "call:MediaWiki_UI.callback_validate_search:" + str(mediawiki.id)
					}
				],
				"confirm": False,
				"target": "call:MediaWiki_UI.callback_search:" + str(mediawiki.id)
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
		data_cept.extend(Cept.from_str(mediawiki.title))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.normal_size())
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_cursor(18, 1))
		data_cept.extend(Cept.set_fg_color(0))
		data_cept.extend(Cept.from_str(mediawiki.search_string))
		# trick: show cursor now so that user knows they can enter text, even though more
		# data is loading
		data_cept.extend(Cept.show_cursor())

		image = Image_UI(basedir + "wikipedia.png", colors = 4)

		data_cept.extend(Cept.define_palette(image.palette))
		data_cept.extend(image.drcs)

		data_cept.extend(Cept.hide_cursor())

		y = 6
		for l in image.chars:
			data_cept.extend(Cept.set_cursor(y, int((41 - len(image.chars[0])) / 2)))
			data_cept.extend(Cept.load_g0_drcs())
			data_cept.extend(l)
			y += 1

		return (meta, data_cept)

	def callback_pageid_for_title(cls, dummy, id_and_title):
		index = id_and_title.find("|")
		mediawiki = MediaWiki.get_from_id(int(id_and_title[:index]))
		return mediawiki.pageid_for_title(id_and_title[index + 1:])

	def callback_validate_search(cls, input_data, id):
		mediawiki = MediaWiki.get_from_id(int(id))
		title = mediawiki.title_for_search(input_data["search"])
		if not title:
			msg = Util.create_custom_system_message("Suchbegriff nicht gefunden! -> #")
			sys.stdout.buffer.write(msg)
			sys.stdout.flush()
			Util.wait_for_ter()
			return Util.VALIDATE_INPUT_BAD
		else:
			return Util.VALIDATE_INPUT_OK

	def callback_search(cls, s, id):
		mediawiki = MediaWiki.get_from_id(int(id))
		title = mediawiki.title_for_search(s["search"])
		sys.stderr.write("TITLE: " + pprint.pformat(title) + "\n")
		return mediawiki.pageid_for_title(title)

	def lang_from_langdigit(langdigit):
		return

	def create_page(pageid, basedir):
		WIKIPEDIA_PAGEID_PREFIX = "55"
		CONGRESS_PAGEID_PREFIX = "35"
		if re.search("^" + WIKIPEDIA_PAGEID_PREFIX + "\d", pageid):
			lang = { 0: "en", 5: "de", 6: "el" }.get(int(pageid[2]))
			wiki_url = "https://" + lang + ".wikipedia.org/"
			mediawiki = MediaWiki.get_from_wiki_url(wiki_url)
			mediawiki.api_prefix = "/w/"
			mediawiki.article_prefix = "/wiki/"
			mediawiki.pageid_prefix = WIKIPEDIA_PAGEID_PREFIX + pageid[2]
			mediawiki.title = { "en": "Wikipedia - The Free Encyclopedia", "de": "Wikipedia - die freie Enzyklopädie", "el": "Wikipedia - The Free Encyclopedia" }.get(lang)
			mediawiki.search_string = { "en": "Search: ", "de": " Suche: ", "el": "Search: " }.get(lang)
			if len(pageid) == 4:
				return MediaWiki_UI.create_search_page(mediawiki, basedir)
			else:
				return MediaWiki_UI.create_article_page(mediawiki, int(pageid[3:-1]), ord(pageid[-1]) - ord("a"))
		if re.search("^" + CONGRESS_PAGEID_PREFIX, pageid):
			sys.stderr.write("pageid: " + pprint.pformat(pageid) + "\n")
#			wiki_url = "https://events.ccc.de/congress/2018/wiki/index.php"
			wiki_url = "https://events.ccc.de/congress/2018/"
			mediawiki = MediaWiki.get_from_wiki_url(wiki_url)
			mediawiki.article_prefix = "/congress/2018/wiki/index.php/"
			mediawiki.pageid_prefix = CONGRESS_PAGEID_PREFIX
			mediawiki.title = "35C3 Wiki"
			mediawiki.search_string = "Search: "
			if len(pageid) == 3:
				return MediaWiki_UI.create_search_page(mediawiki, basedir)
			else:
				return MediaWiki_UI.create_article_page(mediawiki, int(pageid[2:-1]), ord(pageid[-1]) - ord("a"))
		else:
			return None

