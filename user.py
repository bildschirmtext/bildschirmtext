import os
import sys
import json
import time

PATH_USERS = "users/"
PATH_SECRETS = "secrets/"
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
	organisation = None
	
	# personal_data
	street = None
	city = None
	country = None
	stats = None

	@classmethod
	def exists(cls, user_id, ext = "1"):
		filename = PATH_USERS + user_id + "-" + ext + ".user"
		return os.path.isfile(filename)
	
	@classmethod
	def get(cls, user_id, ext, personal_data = False):
		filename = PATH_USERS + user_id + "-" + ext + ".user"
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			dict = json.load(f)
	
		user = cls()
		user.user_id = user_id
		user.ext = ext
		user.salutation = dict.get("salutation", "")
		user.first_name = dict.get("first_name", "")
		user.last_name = dict.get("last_name", "")
		user.organisation = dict.get("organisation", "")
		
		if (personal_data):
			user.street = dict.get("street", "")
			user.city = dict.get("city", "")
			user.country = dict.get("country", "")
			user.stats = Stats(user)

		return user

	@classmethod
	def login(cls, user_id, ext, password):
		filename = PATH_SECRETS + user_id + "-" + ext + ".secrets"
		if not os.path.isfile(filename):
			return None
		with open(filename) as f:
			dict = json.load(f)

		if password != dict.get("password"):
			return None

		return cls.get(user_id, ext, True)
