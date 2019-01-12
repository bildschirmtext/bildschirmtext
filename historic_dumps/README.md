# Historische BTX-Dumps

## 1988: C64 BTX Demo (Input 64 12/88)

Die Originaldateien beinhalteten mehrere Seite pro Datei. Die 53 Dateien entstanden, indem die Originaldatein an allen Vorkommnissen des Codes 0x1A geschnitten wurden. Die Inhalte deuten auf einen Dump im Oktober 1988 hin.

**Die Originaldumps wurden vom Autor des Demos deutlich modifiziert, so daß sie auf einem C64 optimal dargestellt werden können. Die DRCs wurden alle per Hand auf 8x8-Zeichen angepaßt und sind nicht CEPT-kompatibel, und Umlaute verwenden eine andere Codierung!**

* Nur Einstiegsseiten eines Programms sind vollstandig (DRC, Palette), Dumps von Unterseiten wurden gespeichert, direkt nachdem die Hauptseiten übertragen worden waren, es fehlen also DRCs und Paletten. Diese können aber rekonstruiert werden.
* Die Seiten haben keine Remote-Echo Zeichen am Anfang und enden auf 0x1A.
* Die Dateinamen scheinen keine Bedeutung zu haben.

## 1989: Amiga BTX Terminal

Die 169 Dateien wurden mit der Version 2.9 ausgeliefert und haben Timestamps von Anfang April 1989. In der Version 3.2b sind die Dateien identisch, haben aber spätere Timestamps.

* Nur Einstiegsseiten eines Programms sind vollstandig (DRC, Palette), Dumps von Unterseiten wurden gespeichert, direkt nachdem die Hauptseiten übertragen worden waren, es fehlen also DRCs und Paletten. Diese können aber rekonstruiert werden.
* Alle Dateien beginnen mit Remote-Echo-Zeichen (Zeichen, die auf der vorherigen Seite eingegeben wurden, um auf die aktuelle Seite zu gelangen und vom Server zur Ausgabe zurückgeschickt wurden) und enden auf 0x1A
* Die Dateinamen entsprechen der Nummer der Seite.
* Den Originaldumps wurden offenbar vom Auto des Amiga Decoders Informationen über die Verlinkung hinzugefügt, so daß der Decoder im Demo-Modus die Navigation ermöglichen kann, ohne weitere Metadaten zu benötigen. Diese Verlinkung passiert über die ansonsten undefinierten Codes 0x1F/0x3D/0x30+x.
* Die Seiten des Programms 20096 sind speziell für die Demo angefertigt und nicht Teil von BTX. Dier ermöglichen die Navigation zu den anderen Programmen des Dumps. Einige Dumps von Originalseiten wurden jedoch auch in diesem Programm eingeordnet.

## 1989: C64 BTX Demo (64'er 1/90)

Wie beim Input 64 BTX Demo. Die Inhalte der 127 Dateien deuten auf einen Dump im September 1989 hin.

**Die Originaldumps wurden vom Autor des Demos deutlich modifiziert!** (Siehe oben)

## 1991: BTX-VTX Manager v1.2

Die 11 Dateien haben Timestamps von Mai bzw. Juli 1991

* Alle Dateien sind vollständige Dumps einer Seite (inkl. DRC, Palette).
* Manche Dateien beginnen mit 6 Zeichen, die nicht zum Dump zu gehören scheinen. Manche Dateien enden auf 0x1A, andere nicht.
* Die Dateinamen weisen manchmal auf den Namen des Programms hin, manchmal beinhalten sie Teile der Seitennummer.

## 1993: PC online 1&1

Die 35 Dateien sind entstanden, indem die 21 Originaldateien an allen Vorkommnissen des Codes 0x1A geschnitten und Duplikate entfernt wurden.

Die Originaldateien haben Timestamps von Anfang November 1993.

* Alle Dateien sind vollständige Dumps einer Seite (inkl. DRC, Palette).
* Sie haben keine Remote-Echo Zeichen am Anfang und enden auf 0x1A.
* Die Dateinamen weisen auf den Namen des Programms hin.

## 1994: MacBTX 1&1

Die Originaldatei "Datex-J-Offline-Demo" hat einen Timestamp von Mitte April 1994. Es handelt sich um eine Sitzung, die am Stück aufgezeichnet wurde. Dabei wurden Eingaben gemacht, mit externen Rechnern verbunden, und immer wieder auf *0# navigiert.

Die 128 Dateien sind entstanden, indem die Originaldatei an allen Vorkommnissen des Codes 0x1A geschnitten wurde. Bei den einzelnen Dateien handelt es sich dabei oft nicht um eigenständige Seiten, sondern manchmal auch nur um einzelne Schritte beim Ausfüllen einer Dialogseite.

* Nur Einstiegsseiten eines Programms sind vollstandig (DRC, Palette), Dumps von Unterseiten wurden gespeichert, direkt nachdem die Hauptseiten übertragen worden waren, es fehlen also DRCs und Paletten. Diese können aber rekonstruiert werden.
* Alle Dateien beginnen mit Remote-Echo-Zeichen und enden auf 0x1A.
* Die Dateinamen entsprechen der Reihenfolge der Sitzung.

## 1995: BTXTEST

Die Datei BTXTEST.EXE ist eine Offline-Demo für MS-DOS, die 158 Seiten aus dem Themenfeld Commodore 64 beinhaltet. Die Inhalte weisen auf einen Dump im September 1995 hin.

Die Dateien wurden generiert, indem die ausführbare Datei an allen Vorkommnissen des Codes 0x00 geschnitten wurde.

* Die Dateien haben keine Remote-Echo Zeichen am Anfang und enden nicht auf 0x1A.
* Die Dateinamen entsprechen der Reihenfolge in der Datei.

Beschreibung der Inhalte:

* `*922502#` Dies ist die Titelseite der Brotkasten-Corner in Btx. Mit den Tasten BildAuf und BildAb und den Btx-Befehlen können Sie sich durch diese Demo bewegen.
* `*9225020#` Die Inhaltsseiten des Diskussionsforums. Einzelne Nachrichten sind über ihre Nummer aufzurufen.
* `*92250222#` Die Telesoftware-Rubrik. In der Demo können Sie allerdings keine Software laden. 
* `*92250227#` Wolfgang Grimm ist der Programmierer von Btx-Extra für C64 und C128.
* `*92250228#` Olaf Dzwiza hat sich auf Geos-Programmierung mit GeoCom und GeoBasic spezialisiert.
* `*92250229#` Werbeseiten des Hardware-Anbieters CMD.
* `*9225028#` Bei der regelmäßigen Btx-Hitparade landet die Brotkasten-Corner seit über einem Jahr unter den ersten 20.
* `*92250290#` Impressum: So erreichen Sie uns.
* `*92250240#` Informationen zum DFÜ-Spiel "Trade&War"
* `*92250226#` Performance Peripherals vertreibt Hard- und Software für C64/128.
* `*92250223#` Manfred Frick gibt regelmäßig Geos-Sonderhefte heraus.
* `*9225026#` GUSS-Software ist auf Geos-Programme spezialisiert.
* `*92250225#` Die Clubecke: Hier können alle C64- und Computerclubs zu den Selbstkosten Mitteilungen veröffentlichen.
* `*92250224#` Im Shareware-Registrierservice können Sie Shareware-Programme unkompliziert bezahlen.

## 1996: RUN_ME

Die Datei RUN_ME.EXE ist eine spätere Version von BTXTEST. Die 158 Seiten weisen auf einen Dump im August 1996 hin.

Beschreibung der Inhalte:

* `*922502` Dies ist die Titelseite der Brotkasten-Corner in Btx. Mit den Tasten BildAuf und BildAb und den Btx-Befehlen können Sie sich durch diese Demo bewegen.
* `*9225020002` Verweise auf Btx-Seiten anderer C64-Anbieter
* `*9225020` Die Inhaltsseiten des Diskussionsforums. Einzelne Nachrichten sind über ihre Nummer aufzurufen.
* `*92250221` Hinweise zur aktuellen Ausgabe der Brotkasten-CD-ROM
* `*92250222` Die Telesoftware-Rubrik. In der Demo können Sie allerdings keine Software laden. 
* `*92250227` Wolfgang Grimm ist der Programmierer von Btx-Extra für C64 und C128.
* `*92250228` Olaf Dzwiza hat sich auf Geos-Programmierung mit GeoCom und GeoBasic spezialisiert.
* `*92250229` Werbeseiten des Hardware-Anbieters CMD (u.a. Super64-CPU, 20-MHz-C64!)
* `*92250231` Modem-Datenbank: Wie Modems mit C64-Programmen funktionieren.
* `*92250232` HS-Ware: Homebanking-Software für C64.
* `*9225028` Bei der regelmäßigen Btx-Hitparade in *com# landet die Brotkasten-Corner seit über zwei Jahren unter den ersten 20.
* `*92250290` Impressum: So erreichen Sie uns.
* `*92250291` Spendenseite - sichert die Existenz der Brotkasten-Corner.
* `*92250226` Performance Peripherals vertreibt Hard- und Software für C64/128.
* `*92250223` Manfred Frick gibt regelmäßig Geos-Sonderhefte heraus.
* `*9225026` GUSS-Software ist auf Geos-Programme spezialisiert.
* `*92250225` Die Clubecke: Hier können alle C64- und Computerclubs zu den Selbstkosten Mitteilungen veröffentlichen.
* `*92250224`Im Shareware-Registrierservice können Sie Shareware-Programme unkompliziert bezahlen.
