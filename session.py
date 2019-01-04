import os
import json
import time

PATH_USERS = "users/"
PATH_STATS = "stats/"

class Stats():
	last_login = None
	session = None

	def __filename(self):
		return PATH_STATS + self.session.user + "-" + self.session.ext + ".stats"

	def __init__(self, session):
		self.session = session
		filename = self.__filename()
		if os.path.isfile(filename):
			with open(filename) as f:
				stats = json.load(f)	
			self.last_login = stats["last_login"]
	
	def update(self):
		# update the last login field with the current time
		stats = { "last_login": time.time() }
		with open(self.__filename(), 'w') as f:
			json.dump(stats, f)
	

class Session():
	user = None
	ext = None
	salutation = None
	first_name = None
	last_name = None
	last_login = None
	messages = None
	stats = None
	
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
		session.stats = Stats(session)

		return session
