# CEPT-Codes für BTX

Peter Marschat, MCCA, *2550#

*Bildschirmtext bietet eine große Palette an textorientierten Darstellungsformen. Zur Seite benötigt ein ambitionierter BTX-Anbieter ein geeignetes Werkzeug, das ihm die Umsetzung des Bildschirminhalts in CEPT-Codes abnimmt. Derzeit ist das Angebot an solchen Werkzeugen nicht sehr umfangreich. Neben sehr teuren Workstations und der Editiersoftware für Mupid gibt es für PCs (mit Decodix) nur 2 Produkte: "EVE" von der Firma Infonova und den "BTX-Publisher" der Firma Verbis.*

*Dieser Beitrag beschreibt alle CEPT-Befehle der Alphamosaik-Ebene (C0), um einerseits jedem Interessierten die Möglichkeit zu geben, eigene CEPT-Seiten ohne teure Editiersoftware zu erstellen und andererseits Softwareentwicklern einen Anstoß zu geben, neue Editiersoftware zu schreiben.*

## Leistungsumfang von BTX (C0)

* 5 fixe Zeichensätze
* 1 frei definierbarer Zeichensatz (DRCs)
* 32 Farben aus einer Palette von 4096
* Attribute: 18 Blinkmodi, Unterstreichen, 4 Größen, Verdecken, 2 Bildschirmformate, Scrolling, etc.

In den 5 Zeichensätzen sind neben den ASCII-Zeichen nationale Sonderzeichen, mathematische Zeichen und Symbole, sowie Mosaikzeichen integriert. Bis zu 94 Zeichen können frei definiert werden. Alle Steuercodes sind durch CEPT-Normen festgelegt (CEPT ist die Vereinigung der europäischen Postverwaltungen).

Im Folgenden werden alle Steuercodes und Parameterwerte in hexadezimaler (sedezimaler) Form dargestellt.

### 1.) Das Ebenenmodell

Die Darstellung eines Bildes erfolgt in 5 übereinanderliegenden Ebenen:

1. (unterste) Ebene: schwarz (bzw. Video, TV)
2. Ebene: Schirmhintergrund
3. Ebene: Vektorgrafik (C1 und C2)
4. Ebene: Zeichenhintergrund
5. Ebene: Zeichenvordergrund

Die Ebenen 2, 4 und 5 sind mit den Kontrollcodes von C0 frei manipulierbar.

### 2.) Das Bildschirmformat

Die Ebenen 4 und 5 benutzen ein Raster von 24 Zeilen zu je 40 Zeichen. Dieses Format kann geändert werden auf 20 Zeilen zu je 40 Zeichen (kommt in der Praxis kaum vor). Zusätzlich kann "wraparound" (= automatisches Linefeed bei Zeilenende) ein- und ausgeschaltet werden. Hier die entsprechenden Code-Sequenzen:

							wraparound:
	1F 2D        24 x 40       ein
	1F 2D 71     24 x 40       aus
	1F 2D 42     20 x 40       ein
	1F 2D 42 71  20 x 40       aus

### 3.) Reset - Sequenzen

Wir unterscheiden den seriellen und den parallelen Modus für das Setzen von Attributen. Serielle Attribute gelten bis zum Zeilenende und besetzen eine Zeichenposition am Schirm. Parallele Attribute werden sozusagen dem Cursor angeheftet und gelten bis zur nächsten Änderung dieses Attributs.

	1F 2F 41   serieller Grundzustand
	1F 2F 42   paralleler Grundzustand
	1F 2F 43   ser. eingeschränkter Grundzustand
	1F 2F 44   par. eingeschränkter Grundzustand

Im Grundzustand wird der Bildschirm gelöscht und es werden die Default-Werte für die Zeichensätze und die Attribute gesetzt. Der eingeschränkte Grundzustand setzt nur die Zeichensätze zurück.

### 4.) Zeichensätze

Es stehen 5 fixe Zeichensätze und ein freidefinierbarer Zeichensatz (DRCs) zur Verfügung.

	Standard - G0 - Satz:  ASCII-Zeichensatz
	Standard - G1 - Satz:  Mosaikzeichen
	Standard - G2 - Satz:  Sonderzeichen
	Standard - G3 - Satz:  Balken, Pfeile, Punkte, Mosaik
	L - Satz:  Kombination aus G0 und G1
	DRCs - Satz:  max. 94 freidefinierbare Zeichen unterschiedlicher Auflösung

Im Grundzustand wird mit den Zeichencodes 20 bis 7F flinker Zeichenbereich) der Standard-G0-Satz benutzt und mit den Codes A0 bis FF (rechter Zeichenbereich) der Standard-G2-Satz angezeigt.

Mit den folgenden Sequenzen können die anderen Zeichensätze aktiviert werden:

	0E     G1 im linken  Zeichenbereich
	1B 7E  G1 im rechten Zeichenbereich
	1B 6F  G3 im linken
	1B 7C  G3 im rechten
	1B 6E  G2 im linken
	1B 7D  G2 im rechten (default)
	0F     G0 im linken  (default)

Wenn nur ein einzelnes Zeichen aus dem G2- oder G3-Satz gebraucht wird, kann es mit einem einfachen Steuerzeichen in den linken Zeichenbereich geholt werden:

	19   1 Zeichen aus dem G2 - Satz
	1D   1 Zeichen aus dem G3 - Satz

Der L-Satz stellt eine Ausnahme dar. Er ist nur im seriellen Modus durch die Codes 90 bis 97 aktivierbar (im linken Zeichenbereich). Ausgeschaltet wird er durch Verlassen der Zeile, durch Wechsel in den parallelen Modus, durch setzen des linken Zeichenbereichs oder durch Löschen des Bildschirms.

Weiters können die Standard-G-Sätze und der DRCs-Satz frei auf die Plätze G0 bis G3 aufgeteilt werden:

	1B 28 40  Standard-G0-Satz im G0-Satz (default)
	1B 29 40  Standard-G0-Satz im G1-Satz
	1B 2A 40  Standard-G0-Satz im G2-Satz
	1B 2B 40  Standard-G0-Satz im G3-Satz

	1B 28 63  Standard-G1-Satz im G0-Satz
	1B 29 63  Standard-G1-Satz im G1-Satz (default)
	1B 2A 63  Standard-G1-Satz im G2-Satz
	1B 2B 63  Standard-G1-Satz im G3-Satz

	1B 28 62  Standard-G2-Satz im G0-Satz
	1B 29 62  Standard-G2-Satz im G1-Satz
	1B 2A 62  Standard-G2-Satz im G2-Satz (default)
	1B 2B 62  Standard-G2-Satz im G3-Satz

	1B 28 64  Standard-G3-Satz im G0-Satz
	1B 29 64  Standard-G3-Satz im G1-Satz
	1B 2A 64  Standard-G3-Satz im G2-Satz
	1B 2B 64  Standard-G3-Satz im G3-Satz (default)

	1B 28 20 40  DRCs-Satz im G0-Satz
	1B 29 20 40  DRCs-Satz im G1-Satz
	1B 2A 20 40  DRCs-Satz im G2-Satz
	1B 2B 20 40  DRCs-Satz im G3-Satz

### 5.) Steuerzeichen

Die Codes 00 bis 1F und 80 bis 9F werden als Steuerzeichen interpretiert, wobei einige Zeichen "verboten" sind. Sie sind für die Steuerung der Kommunikation zwischen BTX-Zentrale und Endgerät reserviert (Link-Level-Protokoll).

	00 - 07 Protokollzeichen
	08      Cursor links
	09      Cursor rechts
	0A      Cursor hinunter
	0B      Cursor hinauf
	0C      Schirn löschen
	0D      Cursor zun Zeilenanfang (CR)
	0E      G1-Satz in linken Zeichenbereich
	0F      G0-Satz in linken Zeichenbereich
	10      Protokollzeichen
	11      Cursor sichtbar
	12      Zeichenwiederholung (siehe Text)
	13      INI (* für Seitenaufruf)
	14      Cursor unsichtbar
	15 - 17 Protokollzeichen
	18      Zeichen löschen (cancel)
	19      1 Zeichen aus G2-Satz
	1A      Protokollzeichen
	1B      ESCAPE: Einleitung einer Code-Sequenz
	1C      TER (# für Seitenaufruf)
	1D      1 Zeichen aus G3-Satz
	1E      Cursor linka oben (home)
	1F      US: Einleitung einer Code-Sequenz
			bzw. APA: Cursor-Positionierung

Der Code 12 (repeat) wiederholt das zuletzt angezeigte Zeichen bis zu 63 mal. Die genaue Anzahl der Wiederholungen gibt das Zeichen nach dem Code 12 an. Es kann Werte von 41 (1 mal) bis 7F (63 mal) annehmen.

APA (Code 1F) ermöglicht die direkte Positionierung des Cursors am Bildschirm, wobei das folgende Zeichen die Zeile (Code 41 bis 58) und das darauffolgende Zeichen die Spalte (Code 41 bis 68) angibt.

Die Steuerzeichen des rechten Zeichenbereichs (80 bis 9F) werden im seriellen Modus anders interpretiert als im parallelen Modus:

		  serieller Modus:          paralleler Modus:
	80    G1-Satz schwarz           Zeichenvordergrund schwarz
	81    G1-Satz rot               Zeichenvordergrund rot
	82    G1-Satz grün              Zeichenvordergrund grün
	83    G1-Satz gelb              Zeichenvordergrund gelb
	84    G1-Satz blau              Zeichenvordergrund blau
	85    G1-Satz magenta           Zeichenvordergrund magenta
	86    G1-Satz cyan              Zeichenvordergrund cyan
	87    G1-Satz weiß              Zeichenvordergrund weiß
	88    Blinken ein               Blinken ein
	89    Blinken aus               Blinken aus
	8A    transparenter Bereich aus transparenter Bereich aus
	8B    transparenter Bereich ein transparenter Bereich ein
	8C    normale Größe             normale Größe
	8D    doppelte Höhe             doppelte Höhe
	8E    doppelte Breite           doppelte Breite
	8F    doppelt hoch und breit    doppelt hoch und breit
	90    L-Satz schwarz            Zeichenhintergrund schwarz
	91    L-Satz rot                Zeichenhintergrund rot
	92    L-Satz grün               Zeichenhintergrund grün
	93    L-Satz gelb               Zeichenhintergrund gelb
	94    L-Satz blau               Zeichenhintergrund blau
	95    L-Satz magenta            Zeichenhintergrund magenta
	96    L-Satz cyan               Zeichenhintergrund cyan
	97    L-Satz weiß               Zeichenhintergrund weiß
	98    verdeckte Anzeige         verdeckte Anzeige
	99    Unterstreichen aus        Unterstreichen aus
	9A    Unterstreichen ein        Unterstreichen ein
	9B    CSI: Einleitung einer     CSI: Einleitung einer
			   Code-Sequenz              Code-Sequenz
	9C    schwarzer Hintergrund     normale Farbpolarität
	9D    neue Hintergrundfarbe     inverse Polarität (Vorder-
		  ist letzte aktuelle Farbe grund/Hintergr. vertauscht)
	9E    wiederhole Mosaikzeichen  transparenter Hintergrund
		  bei ser. Steuerzeichen
	9F    Mosaikwiederholung aus    verdeckte Anzeige aus

### 6.) Attribute für ganze Reihe und ganzen Schirm

#### a) Farbe: i=0 bis 7 (8 Farben aus aktueller Farbtafel)

	1B 23 20 5i   Farbe für ganzen Schirm (Ebene 2)
	1B 23 20 5E   transparenter Schirmhintergrund
				  Ebene 1 wird sichtbar (Video, TV oder schwarz)
	1B 23 21 4i   Farbe für Zeichenvordergrund der angezeigten
				  Zeichen in der aktuellen Zeile
	1B 23 21 5i   Farbe für Zeichenhintergrund der angezeigten
				  Zeichen in der aktuellen Zeile, inklusive der
				  beiden Bereiche links und rechts neben der
				  Zeile (Ebene 2).
	1B 23 21 5E   transparenter Zeilenhintergrund

#### b) Unterstreichen der ganzen Reihe

	1B 23 21 5A   Unterstreichen ein
	1B 23 21 59   Unterstreichen aus

#### c) Zeichengröße

	1B 23 21 4C   normale Größe in der ganzen Reihe

#### d) Blinken in ganzer Reihe

	1B 23 21 48   Blinken ein
	1B 23 21 49   Blinken aus

#### e) Verdecken in ganzer Reihe

	1B 23 21 58   ein
	1B 23 21 5F   aus

Zeichen, die dieses Attribut tragen, werden erst sichtbar, wenn die "REVEAL"-Taste gedrückt wird.

#### f) Window

	1B 23 21 4B   Anfang
	1B 23 21 4A   Ende

An allen Zeichenpositionen, die mit diesem Attribut belegt sind, wird der Schirmhintergrund (Ebene 2) transparent. Die Farbe des Zeichenvordergrunds und Zeichenhintergrunds bleiben erhalten. Das bedeutet, daß die Ebene 1 (Video, TV oder schwarz) nur dort sichtbar wird, wo auch Vordergrund und/oder Hintergrund transparent ist!

#### g) geschützte Zeile (wird nicht überschrieben)

	9B 31 50      Anfang
	9B 31 51      Ende

#### h) Polarität (Vordergrundfarbe/Hintergrundfarbe)

	1B 23 21 5C   normal
	1B 23 21 5D   vertauscht

### 7.) Farbtafeln

Insgesamt stehen 4 Farbtafeln mit je 8 Farben (0 bis 7) zur Verfügung. Tafel 0 enthält die Ganztonfarben:

	0   schwarz
	1   rot
	2   grün
	3   gelb
	4   blau
	5   magenta (violett)
	6   cyan (hellblau)
	7   weiß

Die Tafel 1 enthält dieselbe Farbpalette mit halber Intensität. Die Tafeln 2 und 3 enthalten im Grundzustand die Farben der Tafel 0. Durch spezielle Code-Sequenzen können diese 16 Farben aber neu definiert werden (siehe Punkt 8).

Farbtafel-Selektierung:

	9B 3i 40   i = 0 bis 3 (für Tafel 0 bis 3)

Die Farbe schwarz in der Tafel 1 (Halbton) hat eine Sonderfunktion. Sie bedeutet "transparent", das heißt, die darunterliegende Ebene wird sichtbar.

### 8.) Farbdefinition

Die Farben der Tafeln 2 und 3 können frei definiert werden, Im Grundzustand enthalten sie die Ganztonfarben fTafel 0). Dieser Zustand kann durch folgende Reset-Sequenz erzwungen werden:

	1F 26 21      Reset

Die Definition eines neuen Farbtons wird eingeleitet durch die Sequenz

	1F 26 20

Nun folgt die Selektierung der Farbposition:

	1F 26 3t 3u    t = 1 bis 3 (Zehner)  u = 0 bis 9 (Einer)

"t" und "u" repräsentieren die Zehner- und Einerstelle einer dezimalen Farbnummer, wobei die Farbpositionen aufsteigende Nummern erhalten, beginnend mit der Nummer 16 (dezimal). Die Farbe 0 der Tafel 2 hat also die Nummer 16 und die Farbe 7 der Tafel 3 erhält die Nummer 31.

Ein Beispiel: Die Farbe 3 (default gelb) der Farbtafel 3 soll geändert werden. Sie besitzt die Nummer 27.

	Code: 1F 26 20   1F 26 32 37

Daran schließen zwei Datenbytes an, die die RGB-Intensitätsanteile festlegen. Es gibt 16 Helligkeitsstufen (0 bis F) für jede der drei Farbanteile (rot, grün, blau), wobei 0 dunkel bedeutet und F die hellste Einstellung ergibt. Die Codierung erfolgt durch folgende Bitverteilung (beginnend mit dem höchstwertigen Bit des 1. Datenbytes):

	0  1  R3 G3 B3 R2 G2 B2      0  1 R1 G1 B1 R0 G0 B0

Ein Beispiel: Die Farbe 3 der Tafel 2 soll folgenden neuen Farbton erhalten:

	Rotanteil:  F      ( 1 1 1 1 )
	Grünanteil: D      ( 1 1 0 1 )
	Blauanteil: 0      ( 0 0 0 0 )

	Codierung der beiden Datenbytes: 
		0 1 1 1 0 1 1 0
		0 1 1 0 0 1 1 0

	gesamter Code: 1F 26 20    1F 26 31 39 76 66

An die beiden Datenbytes können weitere Bytepaare anschließen, die automatisch den nächsten Farbpositionen zugeordnet werden. Abgeschlossen wird die Definition durch eine beliebige Codesequenz, die mit 1F beginnt (zB. Cursorpositionierung).

### 9.) Blinken (im ser. und par. Modus)

	89          kein Blinken
	88          normales Blinken (50% ein / 50% aus)
	9B 30 41    invertiertes Blinken (aus / ein)
	9B 31 41    Blinken zwischen den Farbtafeln 0/1 bzw 2/3
	9B 32 41    schnelles Blinken (ein / aus / aus)
	9B 33 41    schnelles Blinken (aus / ein / aus)
	9B 34 41    schnelles Blinken (aus / aus / ein)
	9B 35 41    Blinkbewegung nach rechts
	9B 36 41    Blinkbewegung nach links

Die letzten beiden Blinkmodi setzen für aufeinanderfolgende Zeichen automatisch die entsprechenden Blinkphasen, sodaß der Eindruck einer Bewegung entsteht.

### 10.) Markierter Bereich

	9B 32 53    Anfang
	9B 32 54    Ende

Dieses Attribut hat keinerlei Einfluß auf die Anzeige. Die CEPT-Verantwortlichen wollten bei der Definition dieses Attributs eine Option auf zukünftige Applikationen schaffen (zB. interne Weiterverarbeitung von markierten Bereichen im Endgerät oder über Peripheriegeräte wie zB. Drucker).

### 11.) Scrolling

Definition des Scrollbereichs:

	9B 3i 3j 3B 3k 3l 55    
	    i = Anfangszeile (Zehner)
	       j = Anfangszeile (Einer)
	             k = Endzeile (Zehner)
	                l = Endzeile (Einer)

Führende Nullen (Zehnerstelle) können weggelassen werden.

Löschen des Scrollbereichs:

	9B 31 3B 31 56
	9B 32 60    Scrolling einschalten
	9B 33 60    Scrolling ausschalten
	9B 30 60    erzwungenes Scrolling nach oben
	9B 31 60    erzwungenes Scrolling nach unten

### 12.) DRCs

Der frei definierbare Zeichensatz basiert auf einer Matrix von 12 Punkten waagrecht und 10 Punkten senkrecht (bzw. 12 Punkten senkrecht beim Bildschirmformat 20x40). Es gibt sieben Typen von DRCs, die sich durch den Grad der Auflösung und durch die Anzahl der möglichen Farben unterscheiden.

	12 x 10 (12) zweifärbig ohne Farbwahl
	 6 x 10 (12) zweifärbig ohne Farbwahl
				 (verwendet aktuelle Vordergrund- und
				  Hintergrundfarbe)
	12 x 10 (12) vierfärbig mit freier Farbwahl
	 6 x 10 (12) vierfärbig mit freier Farbwahl
	 6 x 5  ( 6) vierfärbig mit freier Farbwahl
	 6 x 10 (12) sechzehnfärbig (Farben aus Tafel 2 und 3)
	 6 x 5  ( 6) sechzehnfärbig (Farben aus Tafel 2 und 3)

Die DRCs-Definition wird eingeleitet durch die Sequenz:

	1F 23 20 4p 4q

wobei p die Auflösung und q die Farbanzahl angibt.

	Erlaubte Werte für p: 6 - 12x12
						  7 - 12x10
						  A -  6x12
						  B =  6x10
						  C -  6x 5
						  F -  6x 6

	Erlaubte Werte für q: 1 = zwei Farben
						  2 = vier Farben
						  4 = sechzehn Farben

In einem DRCs-Satz können verschiedene DRC-Formate eingesetzt werden! Soll vor der Definition der DRCs ein vorher geladener DRCs-Satz vollständig gelöscht werden, muß folgende Anfangssequenz verwendet werden:

	1F 23 20 28 20 40 4p 4q

Die Übertragung der DRC-Daten beginnt mit der Sequenz

	1F 23  X  30

Für X ist jener ASCII-Wert einzusetzen, unter dem das neue DRC gespeichert werden soll. X kann Werte zwischen 21 und 7E annehmen. Das darauffolgende Byte 30 ist ein Datenblockbezeichner (Block 0). Daran schließen sich die Datenbytes an, die die Bildpunkte des DRC zeilenweise beschreiben. Jedes Datenbyte kann 6 Bildpunkte aufnehmen (Bit 0 bis Bit 5, Bit 6 ist immer 1, Bit 7 ist immer 0). Je nach gewählter Auflösung (p) werden 1 oder 2 Datenbytes pro Zeile benötigt.

#### a) 2-Farben-DRC

Im 2-Farb-Modus (q=1) entspricht jeder gesetzte Bildpunkt im Datenbyte einem Farbpunkt (Vordergrundfarbe) am Bildschirm. Jedes 0-Bit im Datenbyte entspricht einem Farbpunkt in der Hintergrundfarbe am Bildschirm.

Bei den 4- und 16-Farben-DRCs wird es etwas komplizierter. Hier sind mehrere Datenblöcke für die Beschreibung nötig:

#### b) 4-Farben-DRC

Für jeden Bildpunkt wird eine 2-Bit-Farbinformation benötigt (Farbe 0 bis 3). Es werden 2 Datenblöcke gebildet, die mit den Blockbezeichnern 30 und 31 beginnen. Der erste Datenblock enthält das höherwertige Farbbit für jeden Bildpunkt, der zweite Block das niederwertige Bit.

#### c) 16-Farben-DRC

Hier benötigt jeder Bildpunkt eine 4-Bit-Farbinformation für die 16 Farben (0 bis F). Daher werden 4 Datenblöcke gebildet mit den Blockbezeichnern 30 bis 33. Jeder Block enthält pro Bildpunkt ein Farbbit, beginnend mit dem höchstwertigen Bit.

Nach dem letzten Datenblock eines DRCs können weitere Datenblöcke folgen, die automatisch den folgenden ASCII-Codepositionen zugeordnet werden. Zu beachten ist, daß die hochauflösenden Farb-DRCs (12x10-4-Farben und 6x10-16-Farben) zwei ASCII-Codepositionen belegen! Das bedeutet, daß diese DRCs zB. unter den ASCII-Codes 21, 23, 25, ... gespeichert werden. Zur Vereinfachung der Codierung von DRCs und um mit möglichst wenigen Datenbytes auszukommen, wurden folgende Sonderbytes eingeführt, die statt eines Datenbytes eingesetzt werden dürfen:

	20    Rest des Datenblocks mit "0" füllen
	21    ganze Zeile wiederholen
	22    ganze Zeile 2x wiederholen
	23    ganze Zeile 3x wiederholen
	24    ganze Zeile 4x wiederholen
	25    ganze Zeile 5x wiederholen
	26    ganze Zeile 6x wiederholen
	27    ganze Zeile 7x wiederholen
	28    ganze Zeile 8x wiederholen
	29    ganze Zeile 9x wiederholen
	2A    ganze Zeile 10x wiederholen
	2C    ganze Zeile mit *0* füllen
	2D    ganze Zeile mit *1* füllen
	2E    Rest des Blocks mit letzter ganzer Zeile füllen
	2F    Rest des Datenblocks mit *1* füllen

### 13.) Farbwahl für 4-Farben-DRCs

Während die Farben für 2-Farben-DRCs und 16-Farben-DRCs bereits vorgegeben sind (2-Farben-DRCs werden in den aktuellen Vorder- und Hintergrundfarben dargestellt, 16-Farben-DRCs benutzen die 16 Farben der Farbtafeln 2 und 3), können die Farben der 4-Farben-DRCs frei zugeordnet werden. Im Grundzustand erhalten sie die ersten 4 Farben der Farbtafel 0 (schwarz, rot, grün, gelb). Eine Änderung der DRCs-Farben wird eingeleitet durch die Sequenz

	1F 26 20 22 20 35 40

Nun folgen die Sequenzen für die vier Farben:

	1F 26 3i    i - 0 bis 3 für die Farben 0 bis 3

Daran schließt ein Datenbyte an, das in den Bits 0 bis 2 die Nummer der gewünschten Farbe enthält und in den Bits 3 und 4 die Farbtafel beinhaltet. Bit 5 ist immer 0, Bit 6 ist immer 1, Bit 7 ist immer 0.

	Datenbyte:      0  1  0  T1 T0 F2 F1 F0

Wie alle anderen Sequenzen, die mit 1F beginnen, muß auch hier mit einer 1F-Sequenz fortgesetzt werden, zB. mit einer direkten Cursor-Positionierung 1F 41 41.

### 14.) Servicesprung zur Zeile X

Diese Funktion wird im BTX-System für die Systemmeldungen in der Zeile 24 verwendet. Sie kann aber auch im "normalen" Seitenaufbau verwendet werden.

	Sequenz: 1F 2F 40 X

Für X können Werte von 41 bis 58 eingesetzt werden, was den Zeilen 1 bis 24 entspricht. Der Decoder wird veranlaßt, den gegenwärtigen Zustand mit allen Attributen zwischenzuspeichern und den Cursor in die erste Spalte der angegebenen Zeile zu setzen. Weiters wird der G0-Satz im linken und der G2-Satz im rechten Zeichenbereich aktiviert, der serielle Modus und die Farbtafel 0 wird gesetzt. Mit der Sequenz

	1F 2F 4F    Rückkehr vom Servicesprung

kehrt der Decoder wieder in den zwischengespeicherten Zustand zurück (auch die alte Cursorposition wird wiederhergestellt).

### 15.) Sondersequenzen (Mupid-spezifisch)

#### a) Warten:

	9B 75          Die Anzeige hält an bis eine
				   beliebige Taste gedrückt wird.
	9B 3i 3j 74    eine bestimmte Zeit warten:
				   i = Zehnerstelle
				   j = Einerstelle

"i" und "j" geben die Wartezeit in 1/10 Sekunden (dezimal!) an, wobei führende Nullen entfallen können.

#### b) Automatische Seitennachforderunq (<K> A muß aktiviert sein!)

	9B 2F 73    TER senden an BTX
	9B 2F 72    TER senden an BTX, ein folgendes
				clear screen wird unterdrückt.
	9B 2i 73    "i" senden an BTX
	9B 2i 72    "i" senden, clear screen unterdrückt
	1F 3D 30     defaults für "i" setzen:
				 i = 0 bis 9  Ziffern "0" bis "9"
				 i = A        leer
				 i = B bis F  entspricht TER
	1F 3D 2i "Text"   neue Zeichen für "i" setzen.

"Text" kann 0 bis 15 Zeichen lang sein. Für INI und TER im Text muß 2A und 23 verwendet werden. Beendet wird die Definition durch eine 1F-Sequenz.

## CEPT-Codes numerisch geordnet

	Zeichenerklärung: ..  00 bis FF
					  n   41 bis 7F
					  m   40 bis 5F
					  z   30 bis 39
					  i   0 bis F

### 1.) Steuerzeichen

	08        Cursor links
	09        Cursor rechts
	0A        Cursor nach unten
	0B        Cursor nach oben
	0C        Schirm löschen (CLS)
	0D        Cursor zum Zeilenanfang (CR)
	0E        G1-Satz im linken Zeichenbereich
	0F        G0-Satz im linken Zeichenbereich
	11        Cursor sichtbar
	12 n      Zeichenwiederholung (Kapitel 5)
	13        INI (* für Seitenaufruf)
	14        Cursor unsichtbar
	18        Zeile ab Cursor löschen (Cancel)
	19        1 Zeichen aus G2-Satz
	1B .. ..  ESCAPE: Einleitung einer Code-Sequenz
	1C        TER (# für Seitenaufruf)
	1D        1 Zeichen aus G3-Satz
	1E        Cursor links oben (home)
	1F        OS: Einleitung einer Codesequenz
	1F n n    APA (Kapitel 5)
	80 bis 87 Zeichenfarben (Vordergrund bzw. G1-Satz)
	88        Blinken ein
	89        Blinken aus
	8A        transparenter Bereich aus
	8B        transparenter Bereich ein
	8C        normale Größe
	8D        doppelt hoch
	8E        doppelt breit
	8F        doppelt hoch und breit
	90 bis 97 Zeichenfarben (Hintergrund bzw. L-Satz)
	98        verdeckte Anzeige
	99        Unterstreichen aus
	9A        Unterstreichen ein
	9B        CSI: Einleitung einer Codesequenz
	9C        Hintergrund schwarz bzw. normale Polarität
	9D        Hintergrundfarbe setzen bzw. inverse Polarität
	9E        Mosaikzeichenwiederholung bzw.
			  Hintergrund transparent

### 2.) 1B - Sequenzen

	1B 22 40     serieller Modus
	1B 22 41     paralleler Modus
	1B 23 20 m   Attribute ganzer Schirm (Kapitel 6)
	1B 23 21 m   Attribute ganze Reihe (Kapitel 6)
	1B 28 .. ..  G0-Satz laden (Kapitel 4)
	1B 29 .. ..  G1-Satz laden (Kapitel 4)
	1B 2A .. ..  G2-Satz laden (Kapitel 4)
	1B 2B .. ..  G3-Satz laden (Kapitel 4)
	1B 6E        G2-Satz im linken Zeichenbereich
	1B 6F        G3-Satz im linken Zeichenbereich
	1B 7C        G3-Satz im rechten Zeichensatz
	1B 7D        G2-Satz im rechten Zeichensatz
	1B 7E        G1-Satz im rechten Zeichensatz

### 3.) 1F - Sequenzen

	1F 23 .. ..  DRCs-Definition (Kapitel 12)
	1F 26 .. ..  Farb-Definition (Kapitel 8 und 13)
	1F 2D ..     Format-Definition (Kapitel 2)
	1F 2F ..     Reset, Servicesprung (Kapitel 3 und 14)
	1F 30 ..     Grafik (C1)
	1F 31 ..     Grafik (C2)
	1F 34 ..     Photografischer Modus
	1F 3C ..     Mupid-Teleprogramm-Format
	1F 3D ..     KEY-Definition (Kapitel 15b)
	1F 3E ..     allgemeines Teleprogramm-Format (TSW51)
	1F n  n      direkte Cursor-Positionierung (Kapitel 5)

### 4.) 9B - Sequenzen

	9B 2i ..     Seitennachforderung (Kapitel 15)
	9B z 3B z 55 Scrollbereich-Definition (Kapitel 11)
	9B z 3B z 56 Scrollbereich löschen (Kapitel 11)
	9B z (z) 74  warten (Kapitel 15)
	9B 30 40     Farbtafel 0 selektieren (Kapitel 7)
	9B 30 41     Blinken invertiert (Kapitel 9)
	9B 30 60     1 Zeile nach oben scrollen (Kapitel 11)
	9B 31 40     Farbtafel 1 selektieren (Kapitel 7)
	9B 31 41     Blinken zwischen Farbtafeln (Kapitel 9)
	9B 31 50     geschützte Zeile (Kapitel 6g)
	9B 31 51     Aufhebung der geschützten Zeile (Kapitel 6g)
	9B 31 60     1 Zeile nach unten scrollen (Kapitel 11)
	9B 32 40     Farbtafel 2 selektieren (Kapitel 7)
	9B 32 41     3-Phasen-Blinken l.Phase (Kapitel 9)
	9B 32 53     Bereich markieren Anfang (Kapitel 10)
	9B 32 54     Bereich markieren Ende (Kapitel 10)
	9B 32 60     Scrolling einschalten (Kapitel 11)
	9B 33 40     Farbtafel 3 selektieren (Kapitel 7)
	9B 33 41     3-Phasen-Blinken 2.Phase (Kapitel 9)
	9B 33 60     Scrolling ausschalten (Kapitel 11)
	9B 34 41     3-Phasen-Blinken 3.Phase (Kapitel 9)
	9B 35 41     Blinken rechts (Kapitel 9)
	9B 36 41     Blinken links (Kapitel 9)
	9B 42        verdeckte Anzeige Ende im seriellen Modus
	9B 75        warten auf Tastendruck (Kapitel 15)

## Literatur-Hinweise

* Deutsche Bundespost Telekom Sektion T25
* Functional Specification for BTX-Terminals (1990)
* B.I. Wissenschaftsverlag: Ainhirn / Fellner
* Bildschirmtext und Editieren mit Mupid
* B.I. Wissenschaftsverlag: H. Mülner
* Mupid - Fibel
* Signum-Verlag: Mupid II - Handbuch
