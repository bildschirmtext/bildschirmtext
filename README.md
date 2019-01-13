# Bildschirmtext

Das *Bildschirmtext* ("BTX") Open Source Projekt hat zum Ziel, den gleichnamigen Online-Dienst (1983-2001) wieder verfügbar zu machen.

Dieses Repository besteht aus

* Server-Software zum Bereitstellen des BTX-Dienstes für bestehende Terminals
* einer Sammlung an historischen BTX-Seiten, die über den neuen Dienst wieder erlebt werden können
* neuen BTX-Inhalten
* Tools zum Arbeiten mit CEPT-Dateien

![Bildschirmtext Login](Bildschirmtext.png?raw=true "Bildschirmtext Login")
![Bildschirmtext Login](Bildschirmtext2.png?raw=true "Bildschirmtext Login")

# Bildschirmtext Server

Der BTX Server `neu-ulm.py` ist in Python 3 geschrieben und hat folgende Features:

* Ausliefern von Seiten
* Navigation
* Eingabe von Daten in Dialoge
* Benutzerverwaltung und Login
	* Der Login-Schirm erwartet die Eingabe von Benutzer-Nummer, Mitbenutzer-Nummer und Kennwort.
	* Läßt man die Felder leer, wird man als Gast angemeldet.
* Mitteilungsdienst
	* Über `*8#` erreicht man den Mitteilungsdienst.
	* `*88#` zeigt eingegangene Mitteilungen an.
	* Über `*810#` kann eine neue Mitteilung verfaßt werden.

Es wurde Wert darauf gelegt, die Architektur des Servers der Original-Architektur nachzuempfinden. Seiten historischer Dumps wurden in ihre Bestandteile zerlegt (Palette, Include, Text, Header und Footer der Post), und der Server baut diese Komponenten zur Laufzeit zusammen. So werden Paletten und Zeichensätze nur dann gesendet, wenn sie nicht schon in den Decoder geladen sind. Des weiteren werden die 1. und 24. Zeile des Bildschirms wie beim Originalserver verwendet: Anbieter-Name und Preis werden aus den Metadaten gewonnen, Warnungen und Fehler besitzen die spezifizierten Codes, und die Nutzdaten dürfen diese Zeilen nicht beschreiben.

## Bedienung

Der Gastbenutzer (Benutzer 0, Mitbenutzer 1) hat ein leeres Paßwort, man kann sich also als Gast einloggen, indem man beim Start 3x "#" drückt (oder einmal DCT). Eigene Benutzer kann man definieren, indem man Dateien in `users/` und `secrets/` erstellt.

* `*00#`: Seite nochmals übertragen (bei Übertragungsfehlern)
* `*09#`: Seite nochmals erstellen (oder neu aus der Datenbank laden)
* `*#`: zur vorherigen Seite zurückkehren
* `*Seitennummer#`: Navigation zu einer bestimmten Seite
* `[0-9]` und `#`: Navigation zu einer Folgeseite
* `**`: löscht die Eingabe
* `*9#`: meldet den Benutzer ab

## Verwendung

`neu-ulm.py` kommuniziert über `stdin` und `stdout`. Über das Tool `socat` kann es mit Terminals oder Decoder-Software in Emulatoren verbunden werden.

Es muß sichergestellt werden, daß die Verzeichnisse `messages` und `stats` für den Server schreibbar sind!

## Befehlszeilenparameter

* `--modem`: wartet vor dem Senden von Daten auf einen "`ATD`" Modem-String
* `--user=`*n*: meldet Benutzer mit der angegebenen Nummer automatisch an (Mitbenutzer ist hier immer 1)
* `--page=`*n*: zeigt statt der Anmeldeseite die angegebene Seite an

## BTX-Hardware-Decoder mit RS232-Anschluß

Manche Hardware-Decoder, wie der LOEWE MultiTel-D, erlauben den Anschluß über RS232. (Auf diesem Gerät: `F9`, `F3`, `F1`, `Shift-F1`, `9600`, `F9`.) Mit `socat` kann der Server folgendermaßen mit dem bestehenden seriellen Port verbunden werden:

	socat /dev/tty.usbserial-FTXP1ANL,clocal=1,nonblock=1,ispeed=9600,ospeed=9600,raw system:"python3 neu-ulm.py"

Der Pfad zum seriellen Port muß entsprechend angepaßt werden. Die Optionen von `socat` beziehen sich auf macOS, auf anderen Betriebssystemen müssen sie eventuell angepaßt werden.

## MS-DOS: "PC online" Decoder (Drews)

	socat -d -d exec:"python3 neu-ulm.py --modem" pty,raw,echo=0

`socat` erzeugt einen virtuellen seriellen Port, der mit der Server-Software verbunden ist. Die Ausgabe beinhaltet den Pfad zu diesem Port.

Im DOSBox-Emulator muß dann folgende Zeile in die Konfigurationsdatei eingetragen werden:

	serial3=directserial realport:ttys005

Der Pfad zum seriellen Port muß entsprechend angepaßt werden. Wichtig: Auf Unix-Systemen muß der Pfad `/dev/` weggelassen werden!

Der Decoder wird bit `BTX1` gestartet, die Einwahl erfolgt mit F10.

In der Konfigurationsdatei kann man im Abschnitt `[autoexec]` das Starten der Software automatisieren:

	mount c: /Pfad/zum/Decoder
	c:
	btx1

Geschwindigkeitsoptimierung:

* Die Einwahl kann beschleunigt werden, indem man das Modem auf "Schnelles Modem mit V.32" stellt und die drei `AT`-Strings löscht.
* Eine schnellere Datenübertragung erhält man, indem in "Option -> Schnittstelle (Alt-S)" die Baudrate auf 9600 gestellt wird.

Um die Einstellungen zu speichern, muß das Programm regulär beendet werden (Alt-Y).

## Commodore 64: 64'er-Decoder (Drews)

`socat` muß wie beim "PC online" Decoder gestartet werden.

Im VICE-C64-Emulator müssen dann folgende Zeilen in die `vicerc`-Konfigurationsdatei eingetragen werden:

	RsDevice1="/dev/ttys001"
	RsDevice1Baud=1200
	RsUserDev=0
	RsUserEnable=1
	RsUserBaud=1200

Der Pfad zum seriellen Port muß entsprechend angepaßt werden.

Dann kann der Decoder (siehe Downloads unten) gestartet werden. Die Einwahl wird mit F7 gestartet. `*` befindet sich auf F1 und `#` auf F3.

## Andere Software-Decoder

Es sollte möglich sein, auch andere Software-Decoder über einen virtuellen seriellen Port mit der Server-Software zu verbinden. Über Erfahrungsberichte sind wir dankbar.

## Software-Decoder Downloads

Es existieren einige Software-Decoder für unterschiedliche Betriebssysteme:

* MS-DOS
	* [TeleComm](https://archive.org/details/TEleComm-KommunikationMitKomfort-BTXDecoderSharewareGerman)
	* [BTX/VTX Manager v1.2 (Drews)](https://archive.org/details/BTXVTXManagerV1FrMS-DOS)
	* [PC online 1&1 BTX-COM Version 4.34 (Drews)](https://www.pagetable.com/docs/btx/decoder/PC%20online%201&1%20BTX-COM%20Version%204.34.img)
* Windows 3.x
	* [T-Online 1.0](https://www.pagetable.com/docs/btx/decoder/T-Online-Software%20Version%201.0%20light.img)
	* [AMARIS](https://www.pagetable.com/docs/btx/decoder/AMARIS.zip)
* Mac OS Classic
	* [MacBTX 1&1](https://archive.org/details/MacBTX11German)
* Amiga 
	* [Amiga BTX Terminal V2.9](https://www.pagetable.com/docs/btx/decoder/Amiga%20BTX%20Terminal%20V2.9.DMS)
	* [Amiga BTX Terminal V3.2b](https://www.pagetable.com/docs/btx/decoder/Amiga%20BTX%20Terminal%20V3.2b.DMS)
* C64
	* [64'er Online BTX v1.52 (Drews)](https://www.pagetable.com/docs/btx/decoder/64er%20Online%20BTX%20v1.52%20(19xx)(-)(de).d64)
	* [64'er Online BTX v1.60 (Drews)](https://www.pagetable.com/docs/btx/decoder/64er%20Online%20BTX%20v1.60%20(19xx)(-)(de).d64)

## Die Datenbank

Die Sammlung an BTX-Seiten befindet sich im Unterverzeichnis `data`. Unterverzeichnisse bezeichnen unterschiedliche BTX-Anbieter. Die meisten Seiten sind dem Dump des Amiga BTX Decoder entnommen und wurden in ihre Bestandteile zerlegt:

* `a.glob` für die globalen Metadaten des Anbieters
* `.meta` für die Metadaten der Seite (kann beliebige `.pal` und `.inc` referenzieren)
* `.pal` für die Palette
* `.inc` für den Include für Zeichensatz und Bildschirmfarben
* und `.cept` für den Seiteninhalt (darf keine Zeichensätze oder Paletten heinhalten)

Der Server verknüpft die unterschiedlichen Dateien wieder und stellt sicher, daß Palette und Include nur wenn notwendig übertragen werden.

Die Startseite des Amiga-BTX-Demos erreicht man mit `*200960#`. Über sie sind alle historischen Seiten erreichbar.

# Tools

Die Tools im `tools` Verzeichnis werden mit `make` gebaut.

## decode_cept

`decode_cept` zeigt den Inhalt einer CEPT-Codierten Datei side-by-side sowohl als Hex als auch mit der Beschreibung der Codes an. Hier ein Ausschnitt aus einer Ausgabe:

	[...]
	1f 2f 44                 # parallel limited mode
	1f 42 41                 # set cursor to line 2, column 1
	1b 2b 20 40              # load DRCs into G3
	1b 7c                    # G3 into right charset
	[...]

## cut_btx

`cut_btx` zerlegt die Dateien des Amiga-Dumps in .glob, .meta, .pal, .inc und .cept-Dateien. Da alle Dateien des Amiga-Dumps bereits konveriert sind, und andere Dumps wohl leicht anderen Code der Post benutzen (weil die Dumps aus einer anderen Zeit stammen), muß es erst für andere Dumps angepasst werden.

# Danksagung und verwandte Projekte

* [Bildschirmtrix](http://www.runningserver.com/?page=rs.thelab.bildschirmtrix) von Philipp Maier
* [Javascript BTX Server](http://members.aon.at/nkehrer/btx_server.html) von Norbert Kehrer
* [Retrotext](https://www.acn.wtf/retrotext.html) von Anna Christina Naß
* [btx_modem](https://github.com/Casandro/btx_modem) von Christian Berger
# Copyright

Der Code dieses Projektes steht unter der MIT-Lizenz. Maintainer ist Michael Steil, E-Mail: mist64@mac.com