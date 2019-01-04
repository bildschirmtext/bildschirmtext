import os
import json

PATH_USERS = "users/"

class Session():
	user = None
	ext = None
	salutation = None
	first_name = None
	last_name = None
	last_login = None
	messages = None

	@classmethod
	def login(cls, user, ext, password):
		if user is None or user == "":
			user = "0"
		if ext is None or ext == "":
			ext = "1"
		filename = PATH_USERS + user + "-" + ext + ".user"
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			user_data = json.load(f)

		if password != user_data["password"]:
			return None

		session = cls()
		session.user = user
		session.ext = ext
		session.salutation = user_data["salutation"]
		session.first_name = user_data["first_name"]
		session.last_name = user_data["last_name"]
		return session
