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
						#"default": "64"
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
						"fgcolor": 3,
						"type": "$login_password"
					}
				],
				"confirm": False,
				"target": "page:000001a",
				"no_navigation": True
			}
		}

		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.clear_screen())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_screen_bg_color(12))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Login_UI.btx_logo())
		data_cept.extend(Cept.set_left_g3())
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.set_left_g0())
		data_cept.extend(Cept.set_cursor(18, 8))
		data_cept.extend(Cept.from_str("Teilnehmer"))
		data_cept.extend(Cept.set_cursor(18, 25))
		data_cept.extend(b":")
		data_cept.extend(Cept.set_cursor(18, 36))
		data_cept.extend(Cept.set_fg_color(3))
		data_cept.extend(b'-')
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.set_cursor(20, 8))
		data_cept.extend(Cept.from_str("persönl. Kennwort:"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.set_left_g3())
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		return (meta, data_cept)

	def create_logout():
		meta = {
			"clear_screen": False,
			"links": {
				"#": "00000"
			},
			"publisher_color": 7
		}

		data_cept = bytearray()
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.clear_screen())
		data_cept.extend(Cept.set_cursor(2, 1))
		data_cept.extend(Cept.set_screen_bg_color(12))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Login_UI.btx_logo())
		data_cept.extend(Cept.set_left_g3())
		data_cept.extend(Cept.set_fg_color(15))
		data_cept.extend(Cept.repeat("Q", 40))
		data_cept.extend(Cept.set_fg_color(7))
		data_cept.extend(Cept.set_left_g0())
		data_cept.extend(Cept.set_cursor(19, 8))
		data_cept.extend(Cept.from_str("Vielen Dank für Ihren Anruf!"))
		data_cept.extend(b'\r\n')
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.set_left_g3())
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

		t = datetime.datetime.now()
		current_date = t.strftime("%d.%m.%Y  %H:%M")
		if user.stats.last_login is not None:
			t = datetime.datetime.fromtimestamp(user.stats.last_login)
			last_date = t.strftime("%d.%m.%Y")
			last_time = t.strftime("%H:%M")
		else:
			last_date = "--.--.----"
			last_time = "--:--"

		data_cept = bytearray()
		data_cept.extend(Cept.clear_screen())
		data_cept.extend(Cept.cursor_home())
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.set_screen_bg_color_simple(4))
		data_cept.extend(Cept.load_g0_g0())
		data_cept.extend(Cept.set_left_g0())
		data_cept.extend(Cept.parallel_mode())
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.code_9e())
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(Cept.load_g0_drcs())
		data_cept.extend(Cept.set_left_g0())
		data_cept.extend(b'!"#\r\n$%&')
		data_cept.extend(Cept.cursor_up())
		data_cept.extend(Cept.cursor_right())
		data_cept.extend(Cept.load_g0_g0())
		data_cept.extend(Cept.set_left_g0())
		data_cept.extend(b'\n')
		data_cept.extend(Cept.double_height())
		data_cept.extend(Cept.from_str("Bildschirmtext"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(b'\n')
		data_cept.extend(Cept.set_line_bg_color_simple(4))
		data_cept.extend(Cept.set_palette(1))
		data_cept.extend(Cept.double_height())
		data_cept.extend(b'\r')
		data_cept.extend(Cept.from_str("Deutsche Bundespost"))
		data_cept.extend(b'\n\r')
		data_cept.extend(Cept.set_palette(0))
		data_cept.extend(Cept.normal_size())
		data_cept.extend(Cept.code_9e())
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(Cept.from_str(current_date))
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
		if user.messaging.has_new_messages():
			data_cept.extend(Cept.from_str("Neue Mitteilungen mit 8"))
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(b'\r\n\n\n\n')
		data_cept.extend(Cept.from_str("Sie benutzten Bildschirmtext zuletzt"))
		data_cept.extend(b'\r\n')
		data_cept.extend(Cept.from_str("am "))
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(Cept.from_str(last_date))
		data_cept.extend(Cept.set_fg_color_simple(7))
		data_cept.extend(Cept.from_str(" bis "))
		data_cept.extend(Cept.set_fg_color_simple(3))
		data_cept.extend(Cept.from_str(last_time))
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
		elif pagenumber == "9a":
			return Login_UI.create_logout()
		else:
			return None
