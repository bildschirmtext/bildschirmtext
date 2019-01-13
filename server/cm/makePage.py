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

import getopt
import sys
import os

import tagliste
import drcs

# Kommandozeilenparameter verarbeiten:

def OptionsError():
    """ nicht alle Parameter wurden angegeben; zeigt diese an und beendet Programm """
    print("Aufrufparamter:\n -i <Eingabedatei>\n -o <Ausgabedatei>")
    sys.exit()

infilename = None
outfilename = None

try:
    opts, args = getopt.gnu_getopt(sys.argv[1:], 'i:o:')
except getopt.GetoptError as err:
    OptionsError()

for opt, arg in opts:
    if opt == "-i":
        infilename = arg
    if opt == "-o":
        outfilename = arg

if not infilename or not outfilename:
    OptionsError()

# Eingabedaten lesen:
with open(infilename, 'rb') as infile:
    indata = infile.read()

# Daten verarbeiten:
outdata = indata.replace(b'\x0a',b'')

for el in tagliste.liste:
    old = b'<' + el[0] + b'>'
    # 'new' ist el[1]
    outdata = outdata.replace(old, el[1])

for el in drcs.liste:
    old = b'<' + el[0] + b'>'
    # 'new' ist el[1]
    outdata = outdata.replace(old, el[1])

# Ausgabedatei öffnen und Daten schreiben:
outfile = open(outfilename, 'wb')
outfile.write(outdata)

# Wir sind fertig:
outfile.close()

