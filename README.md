# Bildschirmtext

Das *Bildschirmtext* ("BTX") Open Source Projekt hat zum Ziel, den gleichnamigen Online-Dienst (1983-2001) wieder verfügbar zu machen.

Dieses Repository besteht aus

* Server-Software zum Bereitstellen des BTX-Dienstes für bestehende Terminals
* einer Sammlung an historischen BTX-Seiten, die über den neuen Dienst wieder erlebt werden können
* neuen BTX-Inhalten
* Tools zum Arbeiten mit CEPT-Dateien

## Bildschirmtext Server

Der BTX Server `neu-ulm.py` ist in Python 2 geschrieben und hat folgende Features:

* Ausliefern von Seiten
* Eingabe von Daten in Dialoge
* Navigation
	* `*00`: neu laden
	* `*#`: zur vorherigen Seite zurückkehren
	* `*Seitennummer#`: Navigation zu einer bestimmten Seite
	* `[0-9]` und `#`: Navigation zu einer Folgeseite
	* `**`: löscht die Eingabe
	* `*9#`: meldet den Benutzer ab
* Benutzerverwaltung und Login
	* Der Login-Schirm erwartet die Eingabe von Benutzer-Nummer, Mitbenutzer-Nummer und Kennwort.
	* Läßt man die Felder leer, wird man als Gast angemeldet.
* Mitteilungsdienst (*Work in Progress!*)
	* Über `*8#` erreicht man den Mitteilungsdienst.
	* `*88#` zeigt eingegangene Mitteilunge an.
	* Über `*810#` kann eine neue Mitteilung verfaßt werden. (*Diese wird noch nicht versendet!*)

Es wurde Wert darauf gelegt, die Architektur des Servers der Original-Architektur nachzuempfinden. Seiten historischer Dumps wurden in ihre Bestandteile zerlegt (Palette, Include, Text, Header und Footer der Post), und der Server baut diese Komponenten zur Laufzeit zusammen. So werden Paletten und Zeichensätze nur dann gesendet, wenn sie nicht schon in den Decoder geladen sind. Des weiteren werden die 1. und 24. Zeile des Bildschirms wie beim Originalserver verwendet: Anbieter-Name und Preis werden aus den Metadaten gewonnen, Warnungen und Fehler besitzen die spezifizierten Codes, und die Nutzdaten dürfen diese Zeilen nicht beschreiben.

### Verwendung

`neu-ulm.py` kommuniziert über `stdin` und `stdout`. Über das Tool `socat` kann es mit Terminals oder Decoder-Software in Emulatoren verbunden werden.

Es muß sichergestellt werden, daß die Verzeichnisse `messages` und `stats` für den Server schreibbar sind!

### BTX-Hardware-Decoder mit RS232-Anschluß

Manche Hardware-Decoder, wie der LOEWE MultiTel-D, erlauben den Anschluß über RS232. (Auf diesem Gerät: `F9`, `F3`, `F1`, `Shift-F1`, `9600`, `F9`.) Mit `socat` kann der Server folgendermaßen mit dem bestehenden seriellen Port verbunden werden:

	socat /dev/tty.usbserial-FTXP1ANL,clocal=1,nonblock=1,ispeed=9600,ospeed=9600,raw system:"python neu-ulm.py"

Der Pfad zum seriellen Port muß entsprechend angepaßt werden. Die Optionen von `socat` beziehen sich auf macOS, auf anderen Betriebssystemen müssen sie eventuell angepaßt werden.

### C64-Software-Decoder

	socat -d -d exec:"python neu-ulm.py c64" pty,raw,echo=0

`socat` erzeugt einen virtuellen seriellen Port, der mit der Server-Software verbunden ist. Die Ausgabe beinhaltet den Pfad zu diesem Port.

Im VICE-C64-Emulator müssen dann folgende Zeilen in die `vicerc`-Konfigurationsdatei eingetragen werden:

	RsDevice1="/dev/ttys001"
	RsDevice1Baud=1200
	RsUserDev=0
	RsUserEnable=1
	RsUserBaud=1200

Der Pfad zum seriellen Port muß entsprechend angepaßt werden. Dann kann der Decoder ("64er Online BTX v1.60 (19xx)(-)(de).d64") gestartet werden. Die Einwahl wird mit F7 gestartet. `*` befindet sich auf F1 und `#` auf F3.

### Weitere Software-Decoder

Es existieren noch einige weitere Software-Decoder für unterschiedliche Betriebssysteme:

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

Es sollte möglich sein, auch diese über einen virtuellen seriellen Port mit der Server-Software zu verbinden. Über Erfahrungsberichte sind wir dankbar.

## Die Datenbank

Die Sammlung an BTX-Seiten befindet sich im Unterverzeichnis `data`. Unterverzeichnisse bezeichnen unterschiedliche BTX-Anbieter. Die meisten Seiten sind dem Dump des Amiga BTX Decoder entnommen und wurden in ihre Bestandteile zerlegt:

* `.meta` für die Metadaten
* `.pal` für die Palette
* `.inc` für den Include für Zeichensatz und Bildschirmfarben
* und `.cept` für den Seiteninhalt

Der Server verknüpft die unterschiedlichen Dateien wieder und stellt sicher, daß Palette und Include nur wenn notwendig übertragen werden.

Die Startseite des Amiga-BTX-Demos erreicht man mit `*200960#`. Über sie sind alle historischen Seiten erreichbar.

# Copyright

Der Code dieses Projektes steht unter der MIT-Lizenz. Maintainer ist Michael Steil, E-Mail: mist64@mac.com