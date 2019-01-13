#!/usr/bin/python3
# -*- coding: UTF-8 -*-
'''
rtx - RetroText
by Anna Christina Naß <acn@acn.wtf>
released under GPL

tagliste.py: Enthält Ersetzungstabelle für makePage.py
'''

'''
Empfohlener Aufbau einer Seite:
<ResetPar>[DRCS-Definition]<A>...</A>
'''


liste = [
    # Reset-Sequenzen   
    (b"ResetSer", b"\x1f\x2f\x41"),
    (b"ResetPar", b"\x1f\x2f\x42"),
    # Link-Definition: <A>iaabbb..<A>...</A>: i = lfd.Nr. (1,2,3...); aa: "# ", "1 ", "21"; bbb: Zielseite (0a)
    (b"A", b"\x1f\x3d"),
    (b"/A", b"\x1f\x2f"),
    # Cursor-Positionierung: <Cur><24><01> für zeile,spalte => 24,1
    (b"go", b"\x1f"),
    (b"01", b"\x41"), (b"02", b"\x42"), (b"03", b"\x43"), (b"04", b"\x44"),
    (b"05", b"\x45"), (b"06", b"\x46"), (b"07", b"\x47"), (b"08", b"\x48"),
    (b"09", b"\x49"), (b"10", b"\x4a"), (b"11", b"\x4b"), (b"12", b"\x4c"),
    (b"13", b"\x4d"), (b"14", b"\x4e"), (b"15", b"\x4f"), (b"16", b"\x50"),
    (b"17", b"\x51"), (b"18", b"\x52"), (b"19", b"\x53"), (b"20", b"\x54"),
    (b"21", b"\x55"), (b"22", b"\x56"), (b"23", b"\x57"), (b"24", b"\x58"),
    (b"25", b"\x59"), (b"26", b"\x5a"), (b"27", b"\x5b"), (b"28", b"\x5c"),
    (b"29", b"\x5d"), (b"30", b"\x5e"), (b"31", b"\x5f"), (b"32", b"\x60"),
    (b"33", b"\x61"), (b"34", b"\x62"), (b"35", b"\x63"), (b"36", b"\x64"),
    (b"37", b"\x65"), (b"38", b"\x66"), (b"39", b"\x67"), (b"40", b"\x68"),

    # Zeichensätze:
    (b"G0l", b"\x0f"),  # default links
    (b"G1l", b"\x0e"),
    (b"G1r", b"\x1b\x7e"),
    (b"G2l", b"\x1b\x6e"),
    (b"G2r", b"\x1b\x7d"), # default rechts
    (b"G3l", b"\x1b\x6f"),
    (b"G3r", b"\x1b\x7c"),
    (b"G2", b"\x19"), # Folgendes Zeichen aus G2 holen
    (b"G3", b"\x1d"), # Folgendes Zeichen aus G3 holen
    ## Zeichensätze umlegen:
    (b"G0inG0", b"\x1b\x28\x40"), (b"G0inG1", b"\x1b\x29\x40"), (b"G0inG2", b"\x1b\x2a\x40"), (b"G0inG3", b"\x1b\x2b\x40"),
    (b"G1inG0", b"\x1b\x28\x63"), (b"G1inG1", b"\x1b\x29\x63"), (b"G1inG2", b"\x1b\x2a\x63"), (b"G1inG3", b"\x1b\x2b\x63"),
    (b"G2inG0", b"\x1b\x28\x62"), (b"G2inG1", b"\x1b\x29\x62"), (b"G2inG2", b"\x1b\x2a\x62"), (b"G2inG3", b"\x1b\x2b\x62"),
    (b"G3inG0", b"\x1b\x28\x64"), (b"G3inG1", b"\x1b\x29\x64"), (b"G3inG2", b"\x1b\x2a\x64"), (b"G3inG3", b"\x1b\x2b\x64"),
    ## DRCs ablegen:
    (b"DRCinG0", b"\x1b\x28\x20\x40"), (b"DRCinG1", b"\x1b\x29\x20\x40"), (b"DRCinG2", b"\x1b\x2a\x20\x40"), (b"DRCinG3", b"\x1b\x2b\x20\x40"),

    ## Umlaute + Eszett: -- dafür muß G2 in G2 liegen!
    (b"uml", b"\x19\x48"),  # folgendes Zeichen erhält Umlautpunkte <uml>a => ä
    (b"sz", b"\x19\xfb"),   # ß

    # Steuerzeichen:
    (b"CurL", b"\x08"), (b"CurR", b"\x09"), (b"CurU", b"\x0b"), (b"CurD", b"\x0a"), # Cursor bewegen (left/right/up/down)
    (b"cls", b"\x0c"),      # Clear Screen
    (b"CR", b"\x0d"),       # Cursor zum Zeilenanfang (CR)
    (b"br", b"\x0d\x0a"),   # "CRLF"
    (b"Cursor", b"\x11"),   # Cursor sichtbar
    (b"/Cursor", b"\x14"),  # Cursor unsichtbar
    (b"blink", b"\x88"),    # Blinken an
    (b"/blink", b"\x89"),   # Blinken aus
    (b"transp", b"\x8b"),   # Transparenter Bereich an
    (b"/transp", b"\x8a"),  # Transparenter Bereich aus
    (b"norm", b"\x8c"),     # normal breit+hoch
    (b"2h", b"\x8d"),       # doppelt hoch
    (b"2w", b"\x8e"),       # doppelt breit
    (b"2hw", b"\x8f"),      # doppelt hoch u. breit
    (b"hidden", b"\x98"),   # verdeckte Anzeige
    (b"/hidden", b"\x9f"),  # verdeckte Anzeige aus / Mosaikwiederholung aus (ser)
    (b"u", b"\x9a"),        # Unterstreichen ein
    (b"/u", b"\x99"),       # Unterstreichen aus
    # Zeichenwiederholung: x<Rep><03> => wiederholt x 3x => xxxx
    # TODO: evtl. noch mehr Zahlenwerte einfügen - aktuell nur bis 40 (s.o.)
    (b"rep", b"\x12"),

    # für ganze Zeile gültige Attribute:
    (b"Lu", b"\x1b\x23\x21\x5a"),       # unterstreichen
    (b"/Lu", b"\x1b\x23\x21\x59"),
    (b"Lblink", b"\x1b\x23\x21\x48"),   # blinken
    (b"/Lblink", b"\x1b\x23\x21\x49"),
    (b"Lhidden", b"\x1b\x23\x21\x58"),  # verdecken
    (b"/Lhidden", b"\x1b\x23\x21\x5f"),

    # Farbtafel-Selektion:
    (b"ColTab0", b"\x9b\x30\x40"),  # Tafel 0 wählen (normale Farben)
    (b"ColTab1", b"\x9b\x31\x40"),  # Tafel 1 wählen (halbe Intensität; schwarz ist hier transparent)
    (b"ColTab2", b"\x9b\x32\x40"),  # Tafel 2 wählen (default: wie 0)
    (b"ColTab3", b"\x9b\x33\x40"),  # Tafel 3 wählen (default: wie 0)
    # Hinweis: Die Definition eigener Farben und die Zuweisung zu Tafel 2 und 3 ist hier nicht enthalten

    # Farben Vordergrund (par) / G1-Satz (ser)
    (b"black", b"\x80"),  (b"red", b"\x81"),      (b"green", b"\x82"),  (b"yellow", b"\x83"),
    (b"blue", b"\x84"),   (b"magenta", b"\x85"),  (b"cyan", b"\x86"),   (b"white", b"\x87"),
    # Alternativ: Referenz per Nummer:
    (b"c0", b"\x80"), (b"c1", b"\x81"), (b"c2", b"\x82"), (b"c3", b"\x83"),
    (b"c4", b"\x84"), (b"c5", b"\x85"), (b"c6", b"\x86"), (b"c7", b"\x87"),
    # Farben Hintergrund (background; par) / L-Satz (ser)
    (b"Bblack", b"\x90"), (b"Bred", b"\x91"),     (b"Bgreen", b"\x92"), (b"Byellow", b"\x93"),
    (b"Bblue", b"\x94"),  (b"Bmagenta", b"\x95"), (b"Bcyan", b"\x96"),  (b"Bwhite", b"\x97"),
    # Alternativ: Referenz per Nummer:
    (b"Bc0", b"\x90"), (b"Bc1", b"\x91"), (b"Bc2", b"\x92"), (b"Bc3", b"\x93"),
    (b"Bc4", b"\x94"), (b"Bc5", b"\x95"), (b"Bc6", b"\x96"), (b"Bc7", b"\x97"),
    # Hintergrundfarbe für ganzen Bildschirm: (Screen)
    (b"Sblack", b"\x1b\x23\x20\x50"), (b"Sred", b"\x1b\x23\x20\x51"),     (b"Sgreen", b"\x1b\x23\x20\x52"), (b"Syellow", b"\x1b\x23\x20\x53"),
    (b"Sblue", b"\x1b\x23\x20\x54"),  (b"Smagenta", b"\x1b\x23\x20\x55"), (b"Scyan", b"\x1b\x23\x20\x56"),  (b"Swhite", b"\x1b\x23\x20\x57"),
    # Alternativ: Referenz per Nummer:
    (b"Sc0", b"\x1b\x23\x20\x50"), (b"Sc1", b"\x1b\x23\x20\x51"), (b"Sc2", b"\x1b\x23\x20\x52"), (b"Sc3", b"\x1b\x23\x20\x53"),
    (b"Sc4", b"\x1b\x23\x20\x54"), (b"Sc5", b"\x1b\x23\x20\x55"), (b"Sc6", b"\x1b\x23\x20\x56"), (b"Sc7", b"\x1b\x23\x20\x57"),
    # Hintergrundfarbe für ganze Zeile (mit Rand):
    (b"Rblack", b"\x1b\x23\x21\x50"), (b"Rred", b"\x1b\x23\x21\x51"),     (b"Rgreen", b"\x1b\x23\x21\x52"), (b"Ryellow", b"\x1b\x23\x21\x53"),
    (b"Rblue", b"\x1b\x23\x21\x54"),  (b"Rmagenta", b"\x1b\x23\x21\x55"), (b"Rcyan", b"\x1b\x23\x21\x56"),  (b"Rwhite", b"\x1b\x23\x21\x57"),
    # Vordergrundfarbe für ganze Zeile (Line):
    (b"Lblack", b"\x1b\x23\x21\x40"), (b"Lred", b"\x1b\x23\x21\x41"),     (b"Lgreen", b"\x1b\x23\x21\x42"), (b"Lyellow", b"\x1b\x23\x21\x43"),
    (b"Lblue", b"\x1b\x23\x21\x44"),  (b"Lmagenta", b"\x1b\x23\x21\x45"), (b"Lcyan", b"\x1b\x23\x21\x46"),  (b"Lwhite", b"\x1b\x23\x21\x47"),

    # weiteres zu Farben:
    (b"TranspLine", b"\x1b\x23\x21\x5e"),  # ganze Zeile transparent 
    (b"PolNorm", b"\x9c"),  # normale Farbpolarität (par) - schwarzer Hintergrund (ser)
    (b"PolInv", b"\x9d"),   # invers (par) - neuer Hintergrund ist letzte akt. Farbe (ser)
    (b"BTransp", b"\x9e"),  # Transparenter Hintergrund (par) - Mosaikwiederholung bei ser. Steuerz. (ser)

    # Servicesprung zu Zeile x (erste Spalte):
    # setzt dort G0 links + G2 rechts, seriell, Farbtafel 0
    (b"Service", b"\x1f\x2f\x40"),  # Sprung in Zeile 24: <Service><24>
    (b"/Service", b"\x1f\x2f\x4f"), # Zurück vom Servicesprung

    # Seite abschließen: gehe zu 24,01, Cursor an
    (b"/CEPT", b"\x1f\x58\x41\x11\x1a"),
]


