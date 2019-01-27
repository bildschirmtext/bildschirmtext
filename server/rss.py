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

		if not RSS_UI.feed:
			RSS_UI.feed = feedparser.parse("https://www.pagetable.com/?feed=rss2")

		entry = RSS_UI.feed["entries"][6]
		title = entry["title"]
		html = entry["content"][0]["value"]
		soup = BeautifulSoup(html, 'html.parser')

		page = Cept_page()
		page.soup = soup
		page.article_prefix = "XXX"
		page.insert_html_tags(soup.children)

		meta = {
			"clear_screen": True,
			"links": {
				"0": "0"
			},
			"publisher_color": 0
		}

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

		return (meta, data_cept)

	def create_page(pageid, basedir):
		if pageid.startswith("6502"):
			return RSS_UI.create_article_page(ord(pageid[-1]) - ord("a"))

		else:
			return None


