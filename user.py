import os
import json
import time

PATH_USERS = "users/"
PATH_STATS = "stats/"

# Currently, this only holds the last use
class Stats():
	last_login = None
	user = None

	def __filename(self):
		return PATH_STATS + self.user.user_id + "-" + self.user.ext + ".stats"

	def __init__(self, user):
		self.user = user
		filename = self.__filename()
		if os.path.isfile(filename):
			with open(filename) as f:
				stats = json.load(f)	
			self.last_login = stats.get("last_use")
	
	def update(self):
		# update the last use field with the current time
		stats = { "last_use": time.time() }
		with open(self.__filename(), 'w') as f:
			json.dump(stats, f)
	

class User():
	user_id = None
	ext = None
	salutation = None
	first_name = None
	last_name = None
	stats = None
	
	@classmethod
	def login(cls, user_id, ext, password):
		if user_id is None or user_id == "":
			user_id = "0"
		if ext is None or ext == "":
			ext = "1"
		filename = PATH_USERS + user_id + "-" + ext + ".user"
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			user_data = json.load(f)

		if password != user_data["password"]:
			return None

		user = cls()
		user.user_id = user_id
		user.ext = ext
		user.salutation = user_data["salutation"]
		user.first_name = user_data["first_name"]
		user.last_name = user_data["last_name"]
		user.stats = Stats(user)

		return user
