#!/usr/bin/python3
# -*- coding: UTF-8 -*-
'''
rtx - RetroText
by Anna Christina Naß <acn@acn.wtf>
released under GPL

makePage.py: konvertiert eine Pseudo-CEPT-Seite mit "Tags" in das CEPT-Format
und ersetzt diese "Tags" durch CEPT-Bytes.
Damit sollte es einfacher sein, eigene CEPT-Seiten zu erstellen.
Dabei kommt ein relativ einfaches Suchen-und-Ersetzen-System zum Einsatz,
keine komplexen Tags im Stil von HTML mit Beginn und Ende
'''

import sys
import os

import cm.tagliste
import cm.drcs

class CM:
	def read(infilename):
		# Eingabedaten lesen:
		with open(infilename, 'rb') as infile:
			indata = infile.read()

		# Daten verarbeiten:
		outdata = indata.replace(b'\x0a',b'')

		for el in cm.tagliste.liste:
			old = b'<' + el[0] + b'>'
			# 'new' ist el[1]
			outdata = outdata.replace(old, el[1])

		for el in cm.drcs.liste:
			old = b'<' + el[0] + b'>'
			# 'new' ist el[1]
			outdata = outdata.replace(old, el[1])

		return bytearray(outdata)
