import sys

from cept import Cept

class Util:
	VALIDATE_INPUT_OK = 0
	VALIDATE_INPUT_BAD = 1
	VALIDATE_INPUT_RESTART = 2

	def create_system_message(code, price = 0):
		text = ""
		prefix = "SH"
		if code == 0:
			text = "                               "
		elif code == 10:
			text = "Rückblättern nicht möglich     "
		elif code == 44:
			text = "Absenden? Ja:19 Nein:2         "
		elif code == 47:
			text = "Absenden für " + format_currency(price) + "? Ja:19 Nein:2"
		elif code == 55:
			text = "Eingabe wird bearbeitet        "
		elif code == 73:
			current_datetime = datetime.datetime.now().strftime("%d.%m.%Y %H:%M")
			text = "Abgesandt " + current_datetime + ", -> #  "
			prefix = "1B"
		elif code == 100 or code == 101:
			text = "Seite nicht vorhanden          "
		elif code == 291:
			text = "Seite wird aufgebaut           "
	
		msg = bytearray(Cept.service_break(24))
		msg.extend(Cept.clear_line())
		msg.extend(Cept.from_str(text, 1))
		msg.extend(Cept.hide_text())
		msg.extend(b'\b')
		msg.extend(Cept.from_str(prefix))
		msg.extend(Cept.from_str(str(code)).rjust(3, b'0'))
		msg.extend(Cept.service_break_back())
		return msg
	
	def create_custom_system_message(text):
		msg = bytearray(Cept.service_break(24))
		msg.extend(Cept.clear_line())
		msg.extend(Cept.from_str(text, 1))
		msg.extend(Cept.service_break_back())
		return msg

	def wait_for_ter():
		# TODO: use an editor for this, too!
		sys.stdout.buffer.write(Cept.sequence_end_of_page())
		sys.stdout.flush()
		while True:
			c = sys.stdin.read(1)
			if ord(c) == Cept.ter():
				sys.stdout.write(c)
				sys.stdout.flush()
				break
		cept_data = bytearray(Util.create_system_message(0))
		cept_data.extend(Cept.sequence_end_of_page())
		sys.stdout.buffer.write(cept_data)
		sys.stdout.flush()
