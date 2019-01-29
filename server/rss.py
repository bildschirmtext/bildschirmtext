import sys
import re
import pprint
import urllib.parse
import urllib.request
import feedparser

from bs4 import BeautifulSoup

from cept import Cept
from cept import Cept_page
from cept import Unscii
from util import Util


class RSS_UI:
	feed = None

	def create_article_page(sheet_number):
		is_first_page = sheet_number == 0

		url = "https://www.pagetable.com/?feed=rss2"

		if not RSS_UI.feed:
			RSS_UI.feed = feedparser.parse(url)

		entry = RSS_UI.feed["entries"][6]
		title = entry["title"]
		html = entry["content"][0]["value"]
		soup = BeautifulSoup(html, 'html.parser')

		page = Cept_page()
		page.soup = soup
		page.title = title
		page.article_prefix = "XXX"
		page.url = url
		page.insert_html_tags(soup.children)

		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"publisher_color": 0
		}

		data_cept = page.complete_cept_for_sheet(sheet_number)

		return (meta, data_cept)

	def create_page(pageid, basedir):
		if pageid.startswith("6502"):
			return RSS_UI.create_article_page(ord(pageid[-1]) - ord("a"))

		else:
			return None


