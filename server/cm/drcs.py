#!/usr/bin/python3
# -*- coding: UTF-8 -*-
'''
rtx - RetroText
by Anna Christina Naß <acn@acn.wtf>
released under GPL

drcs.py: Enthält eine DRC-Sammlung
'''

pyramide = ( # 4 Zeichen, 4farbig
    b"\x1f\x23\x30\x30" b"\x40\x40\x2e"
    b"\x31" b"\x40\x41\x40\x43\x21\x40\x47\x21\x40\x4f\x21\x40\x5f\x21\x40\x7f"
    b"\x30" b"\x40\x40\x27\x42\x40\x47\x40"
    b"\x31" b"\x60\x40\x70\x40\x21\x78\x40\x21\x7c\x40\x21\x78\x40\x21\x70\x40"
    b"\x30" b"\x40\x40\x28\x40\x41"
    b"\x31" b"\x40\x7f\x41\x7f\x21\x43\x7f\x21\x47\x7e\x21\x4f\x7c\x4f\x7d\x5f\x78"
    b"\x30" b"\x47\x40\x4E\x40\x4E\x60\x5D\x70\x5B\x70\x77\x70\x6E\x48\x51\x7C\x6F\x7C\x7F\x7E"
    b"\x31" b"\x70\x40\x60\x40\x60\x60\x41\x70\x43\x70\x47\x70\x4E\x40\x50\x40\x60\x40\x40\x40"
)

colorpyramide = (
    b"\x1f\x26\x30\x47"
    b"\x1f\x26\x31\x49"
    b"\x1f\x26\x32\x4f"
    b"\x1f\x26\x33\x43"
)

btxlogo = (
    b"\x1F\x23\x21"
    b"\x30\x40\x40\x40\x40\x43\x7F\x4C\x40\x70\x40\x70\x4F\x70\x7F\x71\x7F\x70\x7C\x70\x40"
    b"\x30\x40\x40\x40\x40\x7F\x7F\x40\x40\x40\x40\x7F\x7F\x7F\x7F\x40\x40\x43\x70\x4F\x7C"
    b"\x30\x40\x40\x40\x40\x7F\x70\x40\x4C\x40\x43\x7C\x43\x7F\x43\x7F\x63\x4F\x43\x40\x43"
    b"\x30\x70\x4F\x70\x7F\x70\x7F\x70\x7F\x70\x7F\x70\x40\x4C\x40\x43\x7F\x40\x40\x40\x40"
    b"\x30\x4F\x7C\x4F\x7C\x73\x73\x7C\x4F\x7F\x7F\x40\x40\x40\x40\x7F\x7F\x40\x40\x40\x40"
    b"\x30\x7C\x43\x7F\x43\x7F\x43\x7F\x43\x7F\x43\x40\x43\x40\x4C\x7F\x70\x40\x40\x40\x40"
)

# "..." auf "." (2e)
ellipse = ( b"\x1f\x23\x2e" b"\x30\x27\x59\x66\x20" )

# Farbtafel 2 (Farbe Nr. 16-19):
alternativefarben = (
    b"\x1F\x26\x20\x1F\x26\x31\x36"  # init + Start ab 16
    b"\x7F\x40\x78\x78\x78\x40\x60\x60" # Farben 0-3
)
 
liste = [
    (b"reset_12x10_4c", b"\x1f\x23\x20\x28\x20\x40\x47\x42"),
    (b"reset_12x10_2c", b"\x1f\x23\x20\x28\x20\x40\x47\x41"),
    (b"Dstart_12x10_4c", b"\x1f\x23\x20\x47\x42"),
    (b"Dstart_12x10_2c", b"\x1f\x23\x20\x47\x41"),
    (b"Dcolorstart", b"\x1f\x26\x20\x22\x20\x35\x40"), # Änderung der Farbtafel folgt
    (b"Dcolorpyramide", colorpyramide),                # ...hiermit
    (b"Dpyramide", pyramide),
    (b"Dbtxlogo", btxlogo),
    (b"Dellipse", ellipse),

    (b"Creset", b"\x1f\x26\x21"),
    (b"Calt2", alternativefarben),
]
