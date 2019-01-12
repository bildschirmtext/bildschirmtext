import os
import sys
import json
import time
import re
import pprint

from cept import Cept
from util import Util

class Historic_UI:
	def create_title(title):
		data_cept = bytearray(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(
			b'\x1b\x28\x40'           # load G0 into G0
			b'\x0f'                   # G0 into left charset
		)
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.double_height())
		data_cept.extend(b'\r')
		data_cept.extend(Cept.from_str(title))
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.normal_size())
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_fg_color_simple(7))
		return data_cept

	def footer(left, right):
		data_cept = bytearray()
		data_cept.extend(Cept.set_cursor(23, 1))
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.from_str(left))
		
		if right:
			data_cept.extend(Cept.set_cursor(23, 41 - len(right)))
			data_cept.extend(Cept.from_str(right))

		return data_cept

	def create_historic_main_page():
		meta = {
			"publisher_name": "!BTX",
			"clear_screen": True,
			"links": {
				"0": "0",
				"11": "711",
				"12": "711",
				"#": "711"
			},
			"publisher_color": 7
		}

		data_cept = bytearray()
		data_cept.extend(Historic_UI.create_title("Historische Seiten"))
		data_cept.extend(b"\r\n")
		data_cept.extend(Cept.from_str(
			"Nur wenige hundert der mehreren hundert-"
			"tausend BTX-Seiten sind überliefert.\n"
			"Die meisten entstammen dem Demomodus von"
			"Software-BTX-Decoderprogrammen.\n"
			"\n"
			"1988: C64 BTX Demo (Input 64 12/88)...--"
			"1989: Amiga BTX Terminal..............11"
			"1989: C64 BTX Demo (64'er 1/90).......--"
			"1991: BTX-VTX Manager v1.2............--"
			"1993: PC online 1&1...................12"
			"1994: MacBTX 1&1......................--"
			"1995: BTXTEST.........................--"
			"1996: RUN_ME..........................--"
			"\n"
			"Da historische Seiten erst angepaßt wer-"
			"den müssen, um nutzbar zu sein, sind\n"
			"noch nicht alle Sammlungen verfügbar."
			#XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
		))

		data_cept.extend(Historic_UI.footer("0 Zurück", None))
		return (meta, Cept.compress(data_cept))

	def historic_link_from_str(s):
		return s.replace("/", "")

	def historic_pretty_link_from_str(s):
		s = "*" + s.split("/")[0] + "#"
		if len(s) >= 8:
			return s + " "
		else:
			return (s + " " * 5)[:8]

	def historic_line(page, index):
		link = Historic_UI.historic_pretty_link_from_str(page[0])
		data_cept = bytearray()
		data_cept.extend(Cept.from_str(link))
		data_cept.extend(Cept.from_str((page[1] + "." * 29)[:38 - len(link)]))
		data_cept.extend(Cept.from_str(str(index)))
		return data_cept

	def create_historic_overview(collection, index):
		if collection == 10:
			name = "Amiga Demo"
			description = (
				"Der Amiga BTX Software-Decoder wurde mit"
				"Dumps von 113 BTX-Seiten aus 32\n"
				"Programmen ausgeliefert, sowie 56 eigens"
				"gestalteten Seiten zum Thema BTX.\n"
				"Die Seiten stammen vom April 1989."
			)
			distribution = [ 9, 17 ]
	
			start_page = [ "20096/1", "Amiga Demo Startseite" ]
	
			pages = [
				[ "1050", "Btx-Telex" ],
				[ "1188", "Teleauskunft" ],
				[ "1692", "Cityruf" ],
				[ "20000", "Deutsche Bundespost" ],
				[ "20096", "Commodore" ],
				[ "20511/223", "Kölner Stadtanzeiger" ],
				[ "21212", "Verbraucher-Zentrale NRW" ],
				[ "25800/0000", "Deutsche Bundesbahn" ],
				[ "30003", "Formel Eins" ],
				[ "30711", "Btx Südwest Datenbank GmbH" ],
				[ "33033", "Eden" ],
				[ "34034", "Frankfurter Allg. Zeitung" ],
				[ "34344", "Neue Mediengesellschaft Ulm" ],
				[ "35853", "ABIDA GmbH" ],
				[ "40040/200", "Axel Springer Verlag" ],
				[ "44479", "DIMDI" ],
				[ "50257", "Computerwelt Btx-Info-Dienst" ],
				[ "54004/04", "ÖVA Versicherungen" ],
				[ "57575", "Lotto Toto" ],
				[ "64064", "Markt & Technik" ],
				[ "65432/0", "ADAC" ],
				[ "67007", "Rheinpfalz Verlag/Druckerei" ],
				[ "201474/75", "Rhein-Neckar-Zeitung" ],
	#			[ "208585", "eba Pressebüro und Verlag [BROKEN]" ],
				[ "208888", "Neue Mediengesellschaft Ulm" ],
				[ "402060", "AUTO & BTX WOLFSBURG" ],
				[ "50707545", "CHIP Magazin" ],
				[ "86553222", "Chaos Computer Club" ],
				[ "505050035", "Steinfels Sprachreisen" ],
				[ "920492040092", "Wolfgang Fritsch (BHP)" ]
			]
		elif collection == 11:
			name = "PC online 1&1"
			description = (
				"Der PC online 1&1 Decoder wurde mit\n"
				"von 35 BTX-Seiten aus 15 Programmen\n"
				"ausgeliefert. Die Seiten stammen vom\n"
				"November 1993."
			)
			distribution = [ ]
	
			start_page = None
	
			pages = [
				[ "25800", "Deutsche Bundesbahn" ],
			]
		else:
			return None

		links = {
			"0": "78",
		}
		if start_page:
			links["10"] = Historic_UI.historic_link_from_str(start_page[0])
		i = 20
		for page in pages:
			links[str(i)] = Historic_UI.historic_link_from_str(page[0])
			i += 1

		meta = {
			"publisher_name": "!BTX",
			"clear_screen": True,
			"links": links,
			"publisher_color": 7
		}
		sys.stderr.write("meta: " + pprint.pformat(meta) + "\n")
		
		data_cept = bytearray()
		data_cept.extend(Historic_UI.create_title("Historische Seiten: " + name))
		data_cept.extend(b"\r\n")

		if not index:
			data_cept.extend(Cept.from_str(description))
			data_cept.extend(b"\r\n\n")
		if start_page:
			data_cept.extend(Historic_UI.historic_line(start_page, 10))
			data_cept.extend(b"\n")

		start_with = 0
		if index:
			for i in range(0, index):
				start_with += distribution[i]

		if index >= len(distribution):
			end = len(pages)
		else:
			end = start_with + distribution[index]
		for i in range(start_with, end):
			data_cept.extend(Historic_UI.historic_line(pages[i], i + 20))
	
		right = "Weiter #" if index < len(distribution) else None
		data_cept.extend(Historic_UI.footer("0 Zurück", right))

		return (meta, Cept.compress(data_cept))

	def create_page(user, pagenumber):
		if pagenumber == "78a":
			return Historic_UI.create_historic_main_page()
		elif re.search("^7\d\d\w$", pagenumber):
			return Historic_UI.create_historic_overview(int(pagenumber[1:3]), ord(pagenumber[3]) - ord('a'))
		else:
			return None
