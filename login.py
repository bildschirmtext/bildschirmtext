import datetime

from cept import Cept

class Login_UI:
	def btx_logo():
		return Cept.from_aa(
			[
				"    ████████████████████████████████████████████████   ",
				"   █                                                █  ",
				"  █                                                  █ ",
				" █                                                    █",
				" █                                                    █",
				" █                                                    █",
				" █                ████████████████████                █",
				" █             ██████████████████████████             █",
				" █           ██████████████████████████████           █",
				" █          ████████████████████████████████          █",
				" █         ███████████            ███████████         █",
				" █         ██████████              ██████████         █",
				" █         ██████████     ████     ██████████         █",
				" █         █████████    ████████    █████████         █",
				" █          ██████     ██████████     ██████          █",
				" █           ███   ███ ██████████ ███   ███           █",
				" █               █████ ██████████ █████               █",
				" █             ███████ ██████████ ███████             █",
				" █            ████████ ██████████ ████████            █",
				" █            ████████ ██████████ ████████            █",
				" █            ████████ ██████████ ████████            █",
				" █            █████████ ████████ █████████            █",
				" █            ██████████  ████  ██████████            █",
				" █            ████████████    ████████████            █",
				" █            ████████████████████████████            █",
				" █            ████████████████████████████            █",
				" █            ████████████████████████████            █",
				" █                                                    █",
				" █                                                    █",
				" █                                                    █",
				" █   ███ █ █   █        █   █                         █",
				" █   █ █   █   █        █              █          █   █",
				" █   █ █ █ █ ███ ███ ██ ███ █ ██ █████ ██ ███ █ █ ██  █",
				" █   ██  █ █ █ █ █   █  █ █ █ █  █ █ █ █  █ █ █ █ █   █",
				" █   █ █ █ █ █ █ ███ █  █ █ █ █  █ █ █ █  ███  █  █   █",
				" █   █ █ █ █ █ █   █ █  █ █ █ █  █ █ █ █  █   █ █ █   █",
				" █   ███ █ █ ███ ███ ██ █ █ █ █  █ █ █ ██ ███ █ █ ██  █",
				" █                                                    █",
				" █                                                    █",
				" █                                                    █",
				"  █                                                  █ ",
				"   █                                                █  ",
				"    ████████████████████████████████████████████████   "
			], 6
		)

	def create_login():
		meta = {
			"clear_screen": False,
			"links": {
			},
			"publisher_color": 7,
			"inputs": {
				"fields": [
					{
						"name": "user_id",
						"hint": "Teilnehmernummer oder # eingeben",
						"line": 18,
						"column": 26,
						"height": 1,
						"width": 10,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "64"
					},
					{
						"name": "ext",
						"hint": "Mitbenutzer oder # eingeben",
						"line": 18,
						"column": 37,
						"height": 1,
						"width": 1,
						"bgcolor": 12,
						"fgcolor": 3,
						"default": "1"
					},
					{
						"name": "password",
						"hint": "Nächstes Feld mit #; Leer für Gast",
						"line": 20,
						"column": 26,
						"height": 1,
						"width": 7,
						"bgcolor": 12,
						"fgcolor": 3
					}
				],
				"confirm": False,
				"target": "page:000001a",
				"is_login": True
			}
		}

		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.clear_screen())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_screen_bg_color(12))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Login_UI.btx_logo())
		data_cept.extend(
			b'\x1b\x6f'                   # G3 into left charset
		)
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(
			b'\x0f'                       # G0 into left charset
			b'\x20\x12\x46'               # repeat ' ' 6 times
		)
		data_cept.extend(Cept.from_str("Teilnehmer"))
		data_cept.extend(
			b'\x20\x12\x46'               # repeat ' ' 6 times
			b":"
			b'\x20\x12\x49'               # repeat ' ' 9 times
		)
		data_cept.extend(Cept.set_fg_color(3))
		data_cept.extend(
			b"-"
		)
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(
			b'\r\n\n'
			b'\x20\x12\x46'               # repeat ' ' 6 times
		)
		data_cept.extend(Cept.from_str("persönl. Kennwort:"))
		data_cept.extend(b'\r\n')
		data_cept.extend(
			b'\x1b\x6f'                   # G3 into left charset
		)
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		return (meta, data_cept)

	def create_start(user):
		meta = {
			"include": "a",
			"clear_screen": True,
			"links": {
				"#": "0",
				"8": "8"
			},
			"publisher_color": 7
		}

		current_date = datetime.datetime.now().strftime("%d.%m.%Y")
		current_time = datetime.datetime.now().strftime("%H:%M")
		if user.stats.last_login is not None:
			t = datetime.datetime.fromtimestamp(user.stats.last_login)
			last_date = Cept.from_str(t.strftime("%d.%m.%Y"))
			last_time = Cept.from_str(t.strftime("%H:%M"))
		else:
			last_date = Cept.from_str("--.--.----")
			last_time = Cept.from_str("--:--")

		data_cept = bytearray()
		data_cept.extend(Cept.clear_screen())
		data_cept.extend(Cept.cursor_home())
		data_cept.extend(
			b'\n'
		)
		data_cept.extend(
			b'\x9b\x31\x40'                           # select palette #1
			b'\x1b\x23\x20\x54'                        # set bg color of screen to 4
			b'\x1b\x28\x40'                           # load G0 into G0
			b'\x0f'                                 # G0 into left charset
			b'\x1b\x22\x41'                           # parallel mode
			b'\x9b\x30\x40'                           # select palette #0
			b'\x9e'                                 # ???
			b'\x87'                                 # set fg color to #7
			b'\x1b\x28\x20\x40'                        # load DRCs into G0
			b'\x0f'                                 # G0 into left charset
			b'!"#\r\n$%&'
			b'\x0b'                                 # cursor up
			b'\x09'                                 # cursor right
			b'\x1b\x28\x40'                           # load G0 into G0
			b'\x0f'                                 # G0 into left charset
		)
		data_cept.extend(b'\n')
		data_cept.extend(
			b'\x8d'                                 # double height
		)
		data_cept.extend(
			Cept.from_str("Bildschirmtext")
		)
		data_cept.extend(
			b'\r\n'
			b'\x1b\x23\x21\x54'                        # set bg color of line to 4
			b'\n'
			b'\x1b\x23\x21\x54'                        # set bg color of line to 4
			b'\x9b\x31\x40'                           # select palette #1
			b'\x8d'                                 # double height
			b'\r'
		)
		data_cept.extend(Cept.from_str("Deutsche Bundespost"))
		data_cept.extend(b'\n\r')
		data_cept.extend(
			b'\x9b\x30\x40'                           # select palette #0
			b'\x8c'                                 # normal size
			b'\x9e'                                 # ???
			b'\x87'                                 # set fg color to #7
			b'\r\n'
			b'\x8c'                                 # normal size
			b'\x83'                                 # set fg color to #3
		)
		data_cept.extend(Cept.from_str(current_date))
		data_cept.extend(b'  ')
		data_cept.extend(Cept.from_str(current_time))
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(b'\r\n\n')
		data_cept.extend(Cept.from_str("Guten Tag"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.from_str(user.salutation))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.from_str(user.first_name))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.from_str(user.last_name))
		data_cept.extend(b'\r\n\n\n')
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(Cept.from_str("Neue Mitteilungen mit 8"))
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(b'\r\n\n\n\n')
		data_cept.extend(Cept.from_str("Sie benutzten Bildschirmtext zuletzt"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.from_str("am "))
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(last_date)
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(Cept.from_str(" bis "))
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(last_time)
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(b'\r\n\r\n\r\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.from_str("Weiter mit #  oder  *Seitennummer#"))
		return (meta, data_cept)

	def create_page(user, pagenumber):
		if pagenumber == "00000a":
			return Login_UI.create_login()
		elif pagenumber == "000001a":
			return Login_UI.create_start(user)
		else:
			return None
