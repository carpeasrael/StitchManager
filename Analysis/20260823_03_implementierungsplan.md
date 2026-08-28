# Implementierungsplan IMP-STM-001 — Analyse vor der Erstellung

**Datum:** 23.08.2026 **Auslöser:** Auftrag, auf Grundlage von URS-STM-001, DES-STM-001 und
TEC-STM-001 einen Implementierungsplan der Version 1.0 zu erstellen. **Nicht Gegenstand:**
Änderungen an den drei Fachdokumenten, am Mockup und an den Gate-Skripten. Ebenso nicht die
Klärung der offenen Punkte selbst — der Plan bereitet sie vor, er entscheidet sie nicht.
**Änderungsklasse:** G (Begründung in Abschnitt 7).

---

## 1. Problembeschreibung

Der Bestand beschreibt vollständig, **was** das System leisten muss (URS-STM-001, 222
Anforderungen), **wie** es aussieht und sich verhält (DES-STM-001) und **womit** es gebaut wird
(TEC-STM-001). Was fehlt, ist die Verbindung dazwischen: eine verplante Reihenfolge, in der die
Anforderungen umgesetzt werden, und die Zuordnung, wo jede einzelne nachgewiesen wird.

Ohne diese Verbindung bleiben drei Festlegungen des Lastenhefts unerfüllbar:

| Stelle | Forderung | Heutiger Zustand |
|---|---|---|
| URS-STM-001 Abschnitt 13.2 | Jede Muss-Anforderung braucht mindestens einen zugeordneten Nachweis | Keine Zuordnung vorhanden |
| URS-STM-001 Abschnitt 13.3 | Rückverfolgbarkeit Anforderung → Umsetzung → Prüffall → Ergebnis | Matrix ist als Vorlage angelegt, aber leer |
| CLAUDE.md Abschnitt 10, Phase 4 | Rückverfolgbarkeit ist je Änderung fortzuschreiben | Es gibt nichts, worin fortgeschrieben würde |

Hinzu kommt ein zweites, praktisches Problem. TEC-STM-001 Abschnitt 8 nennt zehn
Vorgehensschritte, aber ohne Bezug auf einzelne Anforderungen. Wer damit zu bauen beginnt, kann zu
keinem Zeitpunkt sagen, welcher Anteil des geforderten Umfangs erreicht ist. Der Plan schließt
diese Lücke, indem er die Schritte in Arbeitspakete überführt und jedes Paket an Kennungen bindet.

## 2. Betroffene Komponenten

> **Nachgeführt am 24.08.2026.** Der ursprüngliche Ansatz war rein additiv. Aus den Befunden
> der Stufe-1-Prüfung (Abschnitt 8) sind Änderungen an vier bestehenden Dateien geworden. Der
> Endzustand steht hier; die Entwicklung dorthin in Abschnitt 8.
>
> **Erneut nachgeführt am 26.08.2026.** Die Stufe-1-Runde vom 25.08.2026 hat drei Blocker
> gemeldet, die alle denselben Kern treffen: Die zwischenzeitlich eingeführte
> Projektregelprüfung legte den offenen Oberflächenweg fest. Die Tabelle unten führt den
> Endzustand **nach** deren Behebung; die Entwicklung dorthin steht in
> [`20260825_01_anwendungsbau.md`](20260825_01_anwendungsbau.md) und im Protokoll
> `Reviews/20260825-214545_main.md`.

| Datei | Art der Änderung |
|---|---|
| `Analysis/20260823_03_implementierungsplan.md` | **neu** — dieses Dokument, Nachweis der Phase 1 |
| `Implementation/StitchManager_Implementierungsplan.md` | **neu** — IMP-STM-001 v1.0, das Ergebnis |
| `Requirements/StitchManager_Lastenheft.md` | **inhaltlich geändert** — OP-14 bis OP-21 in Kapitel 14, IMP-STM-001 in Abschnitt 1.5, Version 1.2 → **1.3**, Kopfdatum und Historieneintrag |
| `Design/StitchManager_Design_Beschreibung.md` | **inhaltlich geändert** — Abschnitt 13 um OP-15 bis OP-17, OP-19 und OP-20 ergänzt, Version 1.2 → **1.3**, Historieneintrag |
| `TechStack/StitchManager_TechStack.md` | **Kopfzeile nachgezogen** auf URS v1.3 — Metadatenpflege ohne Versionserhöhung (CLAUDE.md Abschnitt 4) |
| `scripts/check-plan.sh` | **neu** — Konsistenzprüfungen des Plans, Teil der Stufe 0c |
| `scripts/check-plan.test.sh` | **neu** — deren Selbsttest, je Bedingung mindestens ein Negativfall, läuft in Stufe 0b mit |
| `scripts/check-projektregeln.sh` | **neu** — Projektregeln am Quellbaum (D-05/SM-DES-003, SM-SEC-004, SM-SEC-005, SM-PLT-007), Teil der Stufe 0c. Legt **keinen** Oberflächenweg fest: Die Gestaltungsquelle deklariert sich selbst, die Pfadzuordnung steht in `.projektregeln.conf` |
| `scripts/check-projektregeln.test.sh` | **neu** — deren Selbsttest, je Bedingung mindestens ein Negativfall, läuft in Stufe 0b mit |
| `.projektregeln.conf` | **neu** — Zuordnung der logischen Modulnamen aus IMP-STM-001 Kapitel 3 auf Pfade. Entscheidet keinen offenen Punkt; sie sagt, wo etwas liegt, nicht womit es gebaut wird |
| `.npmrc` | **neu** — `ignore-scripts=true`; die Installationsskripte der Fremdpakete laufen nicht mit den Rechten der Nutzerin (SM-OSS-011) |
| `scripts/check-docs.sh` | **geändert** — ruft die Planprüfungen auf, trennt ENTFÄLLT von PASS, erfasst mit `sources()` auch `*.sh`, findet die Gestaltungsquelle über deren Markierung statt über einen Pfad und führt den lokalen Linter nur aus, wenn er **nicht** versioniert ist |
| `scripts/review-gate.sh` | **geändert** — die Gate-Signatur bindet die neuen Skripte und die Zuordnungsdatei; Rückgabewert 3 wird als **ENTFÄLLT** protokolliert, nie als PASS, und nicht zwischengespeichert |
| `scripts/review-gate.test.sh` | **geändert** — Stufe 0b zieht den Planprüfer-Selbsttest mit (Fall I1) und den der Projektregeln; Fälle K1 bis K3 für die ENTFÄLLT-Auswertung; die Prüfrepos ignorieren `node_modules/` |
| `CLAUDE.md` | **geändert** — Prüfbereiche, Bestandsbeschreibung, Dokumentenhierarchie, OP-Register und dessen Aufzählung in Abschnitt 8, Klassentabelle, Zeile „Farbliterale“ der Prüfkette, Umgang mit roten Protokollen |

**Gelesen, aber nicht geändert:** `Design/stitchmanager-mockup.html`,
`scripts/install-hooks.sh`.

### 2.1 Warum das Lastenheft am Ende doch geändert wurde

Ursprünglich sollte IMP-STM-001 **nicht** in Abschnitt 1.5 des Lastenhefts eingetragen werden. Die
Begründung: Der Eintrag wäre eine inhaltliche Änderung mit Versionserhöhung, Historieneintrag und
Nachzug der Kopfzeilen in DES-STM-001 und TEC-STM-001 — hoher Preis für einen Rückverweis, der für
die Widerspruchsauflösung nichts leistet.

**Diese Begründung trägt seit v1.3 nicht mehr.** Die sieben offenen Punkte OP-14 bis OP-20 haben
dieselben Kosten ohnehin ausgelöst; der Eintrag kostet nun nichts zusätzlich. Er ist deshalb
erfolgt, ausdrücklich mit dem Zusatz „nachgeordnet — begründet und beschränkt keine Anforderung",
damit die Rangfolge eindeutig bleibt. `CLAUDE.md` Abschnitt 3 führt IMP-STM-001 seither als Rang
5, hinter dem Mockup; der Plan schreibt dieselbe Rangfolge in Abschnitt 0.1.

### 2.2 Ablageort und Kennung

Ablage in einem neuen Verzeichnis `Implementation/`, Kennung **IMP-STM-001** nach dem Muster der
bestehenden Kennungen (URS, DES, TEC, ANA, ABG). Das Verzeichnis ist neu und damit ein unbekannter
Pfad im Sinne der Klassentabelle — siehe Abschnitt 7.

Die Kennung IMP-STM-001 ist **keine Anforderungskennung** im Sinne von `SM-<Bereich>-<Nr>` und
berührt die Regel der Nichtwiederverwendung nicht. Sie folgt dem Muster der Dokumentkennungen und
ist bisher unbelegt.

## 3. Betroffene Anforderungen

Der Plan **verplant** Anforderungen, er ändert keine. Zwei Ebenen sind zu unterscheiden:

**Erfüllt werden durch die Existenz des Plans:** URS-STM-001 Abschnitt 13.2 und 13.3
(Nachweiszuordnung und Rückverfolgbarkeit). Beides sind Festlegungen des Verifikationskapitels,
keine nummerierten Anforderungen — der Plan ist ihr erster tatsächlicher Träger.

**Gegenstand der Verplanung:** die 126 Muss-Anforderungen. Ausgezählt am Lastenheft:

| Priorität | Anzahl | Behandlung in Version 1.0 |
|---|---|---|
| M — Muss | 126 | vollständig in Arbeitspakete verplant, je eine Zeile in der Matrix |
| S — Soll | 79 | elf nachgezogen (Abschnitte 8.1, 8.5, 8.7, 8.9, 8.11), 68 begründet zurückgestellt |
| K — Kann | 17 | benannt, keine Zusage (URS-STM-001 Abschnitt 1.4) |

Die Beschränkung auf Muss ist eine Entscheidung des Auftraggebers. URS-STM-001 Abschnitt 1.4
verlangt für die Streichung von Soll-Anforderungen eine Begründung und deren Dokumentation; der
Plan erbringt sie im eigenen Kapitel „Zurückgestellter Umfang". Ohne dieses Kapitel wäre die
Umfangsentscheidung ein Verstoß gegen 1.4 — mit ihm ist sie eine dokumentierte Umfangsfestlegung.

### 3.1 Eine Reibungsstelle, die zu benennen ist

DES-STM-001 beschreibt an mehreren Stellen Oberflächenverhalten, dessen zugehörige Anforderung im
Lastenheft nur **Soll** ist. Im Muss-Umfang entsteht dieses Verhalten nicht:

| Vorgabe in DES-STM-001 | Zugehörige Anforderung | Wirkung |
|---|---|---|
| Kopfzeile der Musterauswahl mit Filter-Chips und Entfernen-Kreuz (Abschnitt 6.3) | SM-SRC-009 (S) | Filter wirken, sind aber nicht einzeln entfernbar dargestellt |
| Umschalter Kachel-/Listenansicht (Abschnitt 6.3, Werkzeugleiste) | SM-PRV-006 (S), zusätzlich OP-10 offen | Nur Kachelansicht |
| Rahmenprüfung im Detailbereich, Prüfpunkt D-09 (Abschnitt 6.4) | SM-MAC-002 (S) | Box entfällt; D-09 ist in Version 1.0 nicht abnehmbar |

Das ist kein Widerspruch zwischen den Dokumenten — DES-STM-001 beschreibt den Zielzustand, nicht
den ersten Stand. Es ist aber eine Stelle, an der die Abnahmeliste von DES-STM-001 nicht
vollständig bedient wird, und sie gehört sichtbar in den Plan statt in eine spätere Überraschung.
Die betroffenen Prüfpunkte werden im Plan namentlich als „in Version 1.0 nicht anwendbar"
ausgewiesen — nicht als bestanden, nicht als offen.

## 4. Berührte offene Punkte

Einordnung nach CLAUDE.md Abschnitt 8. Maßgeblich ist die Wirkung des Plans, nicht die bloße
Erwähnung eines Punktes.

| Nr | Frage in Kurzform | Einordnung |
|---|---|---|
| OP-13 | Wiederverwendung des vorhandenen Rust-Kerns oder Neuentwicklung? | **neutral** — die Arbeitspakete sind nach Fachinhalt geschnitten, nicht nach Herkunft des Codes. Ein Paket lautet „Formatparser lesend, gehärtet, gefuzzt", nicht „Parser übernehmen" oder „Parser schreiben". Die Antwort ändert Aufwand und Reihenfolge innerhalb eines Pakets, nicht dessen Gegenstand. |
| OP-01 | Einzelplatz oder Synchronisation? | **neutral** — der Plan verplant ausschließlich die Einzelplatzfunktionen des Lastenhefts. Eine spätere Bejahung von Synchronisation erzeugt neue Anforderungen; sie gehören dann ins Lastenheft, nicht in diesen Plan. |
| OP-02 | Gewerblicher Bereich im ersten Stand? | **neutral** — Kapitel 6.13 ist durchgehend Kann und liegt außerhalb des Muss-Umfangs. Die Antwort wirkt auf einen späteren Stand. |
| OP-03 | Zielformate des Schreibpfads? | **neutral** — verplant ist SM-EXP-001 (Konvertierung in ein wählbares Zielformat, Muss). SM-EXP-002, das die konkrete Formatliste festlegt, ist Soll und zurückgestellt. Der Plan nennt die Formatliste als Eingang des Arbeitspakets, nicht als Inhalt. |
| OP-05 | Entfernte KI beibehalten, entfernen oder hinter einen Bau-Schalter? | **neutral** — verplant ist ausschließlich die lokale Verarbeitung (SM-KIA-002, Muss) samt Schlüsselablage und Abschaltbarkeit. SM-KIA-003 ist Kann. Der Plan baut die Schnittstelle so, dass beide Antworten offenbleiben, und schreibt keine entfernte Anbindung fest. |
| — | Weg A (cxx-qt) gegen Weg B (PySide6 und PyO3) | **grundlagenschaffend** — der Plan stellt den Prototypvergleich als erstes Arbeitspaket voran, mit den beiden Messpunkten aus TEC-STM-001 Abschnitt 2.3. Der Modulschnitt ist wegunabhängig formuliert. |
| OP-07, OP-09 | Festbreitenschrift freigegeben? Farbwerte abgeglichen? | **neutral, aber terminiert** — beide sind Vorbedingung des Arbeitspakets „Gestaltungsgrundlage". Der Plan entwickelt gegen die Bezeichner, nicht gegen die Werte, und führt die Klärung als Eingangsbedingung. |
| OP-08 | Referenzgerät für die Messungen? | **neutral, aber terminiert** — Vorbedingung des Nachweisplans. Ohne Referenzgerät sind SM-NFR-001 und SM-SRC-007 nicht abnehmbar (SM-NFR-004 ist Soll und zurückgestellt); der Plan benennt das als Eingang, setzt aber kein Gerät fest. |
| OP-04, OP-06, OP-11 | Paketsignatur, Produktname, Symbol im Dunkelmodus | **neutral, aber terminiert** — Vorbedingungen des Auslieferungspakets. SM-PLT-002 und SM-SEC-012 stehen selbst unter dem Vorbehalt „sofern die Signaturinfrastruktur bereitsteht"; der Plan gibt diesen Vorbehalt unverändert weiter. |
| OP-10, OP-12 | Listenansicht, Kennzahlen der Übersichtskarte | **neutral** — beide betreffen Soll-Umfang außerhalb von Version 1.0. |

**Kein offener Punkt wird durch den Plan entschieden.** Wo eine Antwort gebraucht wird, steht sie
als Vorbedingung eines Arbeitspakets, und das Paket startet nicht, bevor sie vorliegt.

## 5. Begründung

Der Plan wird gebraucht, weil vier Festlegungen des Bestands sonst wirkungslos bleiben:

1. **Die Nachweispflicht braucht einen Träger.** „Eine Anforderung ohne zugeordneten Prüffall gilt
   als nicht abgenommen" (URS-STM-001 Abschnitt 13.3) ist ohne Matrix eine Aussage ohne Adressat.
2. **Das Gate wächst mit dem Code, nicht von selbst.** CLAUDE.md Abschnitt 11 teilt die Prüfkette
   in „heute anwendbar" und „mit dem ersten Quellcode verbindlich". Wann welche Prüfung scharf
   wird, steht nirgends. Der Plan bindet das an Arbeitspakete: Mit dem ersten `Cargo.toml`
   wechseln die Rust-Gates von ENTFÄLLT auf prüfpflichtig (S3), und die Projektregelprüfungen zu
   SM-DES-003 und SM-NFR-012 müssen zu diesem Zeitpunkt stehen.
3. **Die Reihenfolge ist nicht beliebig.** Die Pfadprüfung muss vor dem Import stehen, die
   Kernfassade vor der Oberfläche, die Lizenzprüfung vor der ersten fremden Abhängigkeit. Wird das
   nachgeholt, ist es teurer — und im Fall der Fassade (SM-SEC-004) praktisch nicht mehr
   durchsetzbar.
4. **Die teuerste Unsicherheit gehört an den Anfang.** TEC-STM-001 stellt den Prototypvergleich
   bewusst vor alles Bauen; der maßhaltige Druck (SM-PRN-006) ist die härteste Anforderung des
   Vorhabens. Ein Plan, der damit endet statt zu beginnen, verschiebt das Risiko nach hinten.

## 6. Vorgeschlagener Ansatz

Erstellung von `Implementation/StitchManager_Implementierungsplan.md` als IMP-STM-001 v1.0 in
folgender Gliederung:

| Kap. | Inhalt |
|---|---|
| 0 | Zweck, Rangfolge, ausdrückliche Feststellung: begründet keine Anforderung, führt kein eigenes Register offener Punkte |
| 1 | Umfang von Version 1.0 — 126 Muss und elf nachgezogene Soll verplant, die übrigen begründet zurückgestellt, Reibungsstellen nach Abschnitt 3.1 dieser Analyse benannt |
| 2 | Entscheidungsgatter: je offenem Punkt die wartenden Arbeitspakete und der Klärungsweg |
| 3 | Zielarchitektur und Modulschnitt aus TEC-STM-001 Abschnitt 3, wegunabhängig formuliert, je Modul die verantworteten Kennungen |
| 4 | Arbeitspakete AP-00 bis AP-22, je mit Ziel, Kennungen, Vorbedingungen, Ergebnis, Nachweis, Risiko |
| 5 | Meilensteine M0 bis M7 mit der Bedingung, die den Meilenstein schließt |
| 6 | Prüf- und Nachweisplan, Prüffallschema, Messaufbauten |
| 7 | Rückverfolgbarkeitsmatrix, 126 Zeilen |
| 8 | Mitwachsende Prüfkette — welche Gate-Stufe mit welchem Arbeitspaket scharf wird |
| 9 | Risiken |
| 10 | Zurückgestellter Umfang mit Begründung |
| 11 | Offene Punkte — Verweis auf URS-STM-001 Kapitel 14 |
| 12 | Änderungshistorie |

**Schnittprinzip der Arbeitspakete:** fachlich, nicht technologisch. Jedes Paket ist unter beiden
Antworten auf OP-13 und unter beiden Anbindungswegen dasselbe Paket. Ein Paket ist abgeschlossen,
wenn seine Kennungen einen Prüffall haben und dieser bestanden ist.

**Prüffallschema:** `PF-<Bereich>-<nn>`, wobei `<Bereich>` das Kürzel der Anforderung übernimmt.
Der Plan vergibt die Kennungen, führt die Ergebnisspalte aber durchgehend als offen — er ist
Planung, kein Nachweis.

### 6.1 Regeln, an denen das Dokument sonst scheitert

Aus `scripts/check-docs.sh` gelesen. Beide betreffenden Prüfungen laufen über **alle**
Markdown-Dateien außer `Reviews/`, also auch über ein neues Verzeichnis:

- **Kein Farbliteral** außerhalb von DES-STM-001 Abschnitt 3 (Prüfung 6, Vorwegnahme von D-05).
  Der Plan nennt ausschließlich Bezeichner, nie Werte — was ohnehin der Regel aus DES-STM-001
  Abschnitt 1 entspricht, gegen Namen zu entwickeln.
- **Keine tote Kennung** (Prüfungen 1 und 4). Jede genannte Anforderungs- und Punktkennung muss im
  Lastenheft definiert beziehungsweise geführt sein.
- **Deutsch**, Sachsprache, Tabellenform (SM-SET-006 gilt der Oberfläche, RB-05 und CLAUDE.md
  Abschnitt 1 dem Dokumentbestand).

### 6.2 Beobachtung ohne Blockerwirkung

`CLAUDE.md` Abschnitt 11 beschreibt `.markdownlint-cli2.jsonc` im Wurzelverzeichnis als vorhanden
und begründet damit die Ausnahme der Zeilenlängenregel für Tabellen. Die Datei existiert nicht;
`markdownlint-cli2` ist zudem nicht installiert, weshalb Prüfung 7 der Stufe 0c heute als
entfallen gemeldet wird. Das ist eine Altlast im Sinne von S4 — von dieser Änderung weder
verursacht noch verschlimmert. Sie **bleibt unbehoben und ohne Registrierung**: Ein offener Punkt
des Lastenhefts wäre der falsche Ort, weil die Sache keine Anforderung berührt, sondern die
Werkzeugkette dieses Repositorys. Wer sie beheben will, legt `.markdownlint-cli2.jsonc` an und
installiert den Linter — beides Klasse G. Der Plan hält seine Fließtextzeilen unabhängig davon bei
höchstens 100 Zeichen.

## 7. Änderungsklasse

**Klasse G** — im Endzustand aus zwei Gründen, von denen jeder allein trägt:

1. Die Änderung legt mit `Implementation/` ein neues Verzeichnis an. Zum Zeitpunkt der Analyse war
   es ein im Klassenschema nicht geführter Pfad; `CLAUDE.md` Abschnitt 9 ordnet unbekannte Pfade
   der Klasse G zu. Inzwischen führt die Klassentabelle `Implementation/` ausdrücklich unter
   Klasse D — für **künftige** Änderungen am Plan gilt damit D, nicht G.
2. Die Änderung ändert `CLAUDE.md` selbst. Das ist unabhängig vom ersten Grund Klasse G und bleibt
   es auch nach der Ergänzung der Klassentabelle.

Klasse D scheidet für **diesen** Änderungssatz damit aus, obwohl er auch ein Fachdokument ändert
(URS-STM-001 v1.3): Im Zweifel gilt die höhere Klasse. Klasse T scheidet aus, weil der
Änderungssatz weit über zwanzig Zeilen umfasst und Kennungen vergibt.

Daraus folgt: Analysepflicht (dieses Dokument), Gates 0, 0b und 0c, Stufe 1 mit vollständigem
Konsens aller vier Reviewer. Die Rust-Gates stehen mangels `Cargo.toml` als ENTFÄLLT im Protokoll
(S3) — nicht als bestanden.

## 8. Nachführung des Ansatzes nach der Stufe-1-Prüfung

`CLAUDE.md` Abschnitt 10, Phase 2 verlangt, den Ansatz nachzuführen statt ihn stillschweigend zu
ersetzen. Die erste Stufe-1-Runde endete 0/4 mit 31 Befunden — 0 blocker, 13 major, 18 minor, alle
mit Änderungsbezug. Das zugehörige Protokoll ist rot und wird nach Abschnitt 15 der `CLAUDE.md`
nicht committet; die Befunde sind deshalb hier zusammengefasst, damit sie nicht mit ihm
verschwinden. Vier Festlegungen der Abschnitte 3 bis 6 haben sich geändert:

**8.1 Vier Soll-Anforderungen sind nachgezogen.** Der ursprüngliche Ansatz verplante
ausschließlich Muss. Die Prüfung wies nach, dass dadurch **Muss**-Zusagen ins Leere laufen: Ohne
SM-SRC-009 gibt es keinen Bedienweg, einen Filter wieder aufzuheben, obwohl SM-SRC-003 und
SM-SRC-010 Filterung zusagen; ohne SM-BAT-005 kein Bedienelement, das den Abbruch auslöst, dessen
Folgenlosigkeit SM-BAT-007 zusichert. SM-NFR-009 und SM-NFR-013 tragen zwei verbindliche
Gestaltungsgrundsätze aus DES-STM-001 (Abschnitt 2 und 8) und entstehen einmalig in der
Gestaltungsgrundlage. Der Umfang der Version 1.0 beträgt damit **130 Anforderungen**. Entscheidung
des Nutzers vom 24.08.2026.

**8.2 Zwei Lücken des Lastenhefts sind als offene Punkte aufgenommen.** Für zwei bei der Prüfung
gefundene Mängel existiert im Lastenheft keine Kennung: die Härtung der Anzeigekomponente für
Fremddokumente (SM-FMT-012 erfasst ausdrücklich nur die Stickformatparser) und die ausformulierten
Zustände aus DES-STM-001 Abschnitt 10. Da der Plan keine Anforderung begründen darf, sind sie als
**OP-14** und **OP-15** in Kapitel 14 des Lastenhefts eingetragen; URS-STM-001 steigt dadurch auf
**v1.3**, die Kopfzeilen von DES-STM-001 und TEC-STM-001 werden im selben Commit nachgezogen
(Metadatenpflege, keine Versionserhöhung dort). Entscheidung des Nutzers vom 24.08.2026.

**8.3 Der Nachweisbegriff ist geschärft.** Der ursprüngliche Ansatz führte alle messenden
Nachweise in AP-22 und erzeugte damit einen Zirkelschluss: AP-22 setzte AP-21 voraus, AP-21 alle
vorstehenden Pakete, und kein Paket mit einer Anforderung der Prüfmethode A hätte je schließen
können. Der Plan unterscheidet nun **Erstnachweis** (im Fachpaket, gegen einen in AP-02 erzeugten
Prüfbestand von 100.000 Einträgen), **Wiederholung** (in AP-22, je Veröffentlichung) und
**Regressionslauf** (für die drei querschnittlichen Kennungen).

**8.4 Eine Regel der `CLAUDE.md` ist berichtigt.** `CLAUDE.md` Abschnitt 11 nannte die
Prüfbereiche `Requirements/`, `Design/`, `TechStack/`, `CLAUDE.md` und `scripts/`. Mit
`Implementation/` entstand ein Bereich, den der Regeltext nicht führte, während
`scripts/check-docs.sh` ihn faktisch prüft. Die Deckung beruhte auf einer Eigenschaft der
Umsetzung, nicht auf der Regel; die Liste ist ergänzt (S1, Befund gegen die Datei).

**8.5 Zweite Runde: weitere fünf Festlegungen geändert.** Auch die zweite Stufe-1-Runde endete
0/4, mit 37 Befunden (0 blocker, 15 major, 22 minor). Die Befunde lagen tiefer als in Runde 1 und
trafen überwiegend Folgen der Änderungen aus 8.1 bis 8.4:

- **SM-SEC-010 nachgezogen** (fünfte Soll-Anforderung, Umfang jetzt 131). Die Zurückstellung hatte
  auf einer engen Lesart von „exportierbar" beruht — eine Auslegung, die dem Plan nach Abschnitt
  0.1 nicht zusteht.
- **Drei weitere offene Punkte:** OP-16 (Zustandstabelle und Mindestgrößen aus DES-STM-001
  Abschnitt 7), OP-17 (Übersichtskarte aus Abschnitt 6.2), OP-18 (fehlende Schwellenwerte für
  SM-NFR-002 und SM-SRC-008). OP-14 wurde neu gefasst: Die ursprüngliche Formulierung enthielt die
  Behauptung, SM-FMT-012 erfasse „ausdrücklich nur die Stickformatparser" — der Wortlaut sagt
  „alle Formatparser". Der Plan hatte damit eine Muss-Anforderung enger ausgelegt, als sie
  geschrieben ist.
- **Fristen des Lastenhefts eingehalten:** OP-07, OP-09 und OP-15 bis OP-17 tragen die Frist „vor
  Umsetzungsbeginn der Oberfläche". Sie standen in Abschnitt 2.2 des Plans, der nur den
  *Abschluss* sperrt — ein nachgeordnetes Dokument hätte damit eine Festlegung des führenden
  aufgeweicht. Sie stehen jetzt in Abschnitt 2.1 und sperren den Beginn von AP-11 und AP-12.
- **Zwei weitere Zirkelschlüsse aufgelöst:** SM-NFR-002 und SM-NFR-008 waren Paketen zugeordnet,
  die vor der Oberfläche beziehungsweise vor dem Dialoggerüst schließen; beide Prüffälle sind
  jetzt geteilt. Ebenso die Prüfpunkte D-03, D-04, D-06 und D-11.
- **`CLAUDE.md` vollständig nachgezogen:** Die Ergänzung der Prüfbereiche aus 8.4 hatte an vier
  weiteren Stellen Widersprüche erzeugt (Bestandsbeschreibung, Dokumentenhierarchie, OP-Register,
  Klassentabelle). Alle sind behoben; IMP-STM-001 steht jetzt als Rang 5, und `Implementation/`
  fällt künftig unter Klasse D statt G.

### 8.6 Befunde der nicht committeten roten Protokolle

Nach `CLAUDE.md` Abschnitt 15 werden rote Protokolle nicht committet; ihre blocker- und
major-Befunde stehen deshalb hier. Alle Befunde beider Runden hatten Änderungsbezug (S4).

**Runde 1** — 31 Befunde, 0 blocker, 13 major, 18 minor:

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | AP-22, AP-21, Abschnitt 4.1 | major | Zirkelschluss: AP-22 trägt alle Messnachweise, setzt aber AP-21 und damit alle Fachpakete voraus | SM-PRN-006, SM-SRC-007 u. a. | behoben (8.3) |
| N-2 | AP-02/AP-07 | major | Prüfbestand von 100.000 Einträgen in keiner Vorbedingungskette der messenden Pakete | SM-LIB-009, SM-NFR-001 | behoben |
| N-3 | Kapitel 6.4 | major | SM-PRV-007 und SM-SRC-008 ohne Messaufbau | SM-PRV-007, SM-SRC-008 | behoben |
| N-4 | AP-10 | major | Oberflächenkennungen SM-SRC-008 und SM-NFR-003 an AP-10 (Kernpaket) | SM-SRC-008, SM-NFR-003 | behoben |
| T-1 | Abschnitt 1.2 | major | Reibungsstellenliste behauptet Vollständigkeit („an drei Stellen"), war unvollständig | SM-DES-008 u. a. | behoben |
| Te-1 | AP-03 | major | Schreibziele außerhalb der Bibliothekswurzel ungeprüft; Fremddatennamen unbereinigt | SM-SEC-001, SM-SEC-002 | behoben |
| Te-2 | AP-14 | major | PDF-Anzeige als zweiter Fremddatenparser mit „Risiko: gering" abgetan | SM-DOC-004, OP-14 | behoben |
| C-1 | AP-12 | major | Die sechs Zustände aus DES-STM-001 Abschnitt 10 in keinem Paket verplant | OP-15 | behoben |
| C-2 | Abschnitt 1.2 | major | Reibungsstellenliste unvollständig (deckungsgleich mit T-1) | SM-DES-008 | behoben |
| C-3 | Kapitel 10.1 | major | SM-NFR-009 und SM-NFR-013 ohne Begründung zurückgestellt | SM-NFR-009, SM-NFR-013 | behoben (nachgezogen, 8.1) |
| C-4 | Kapitel 3/AP-12 | major | Dialoge und Navigationsspalte ohne Bauort; `ui/navigation` und `ui/dialoge` fehlen in der Modultabelle | SM-DAT-003, SM-LIB-002 | behoben |
| C-5 | Abschnitt 4.2 | major | Querschnittsnachweise fallen vor der Mehrzahl der Dialoge | SM-NFR-008, SM-SET-006, SM-NFR-006 | behoben (Abschnitt 4.2) |
| C-6 | AP-17 | major | Abbruchzusage ohne Abbruch-Bedienelement | SM-BAT-007, SM-BAT-005 | behoben (nachgezogen, 8.1) |

**Runde 2** — 37 Befunde, 0 blocker, 15 major, 22 minor:

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | AP-22/AP-21 | major | SM-NFR-002 wird in AP-08 erstnachgewiesen, der Messaufbau verlangt eine Oberfläche | SM-NFR-002, AK-01 | behoben (PF-NFR-02.1/.2) |
| N-2 | AP-02/AP-07 | major | Prüfbestand in M0 nicht erzeugbar — Schema entsteht erst in AP-04 | SM-LIB-009, SM-SRC-007 | behoben (Zweiteilung) |
| N-3 | Kapitel 6.4 | major | Fassade liefert Treffermengen nicht ausschnittweise — Virtualisierung endet an der Schnittstelle | SM-PRV-007, SM-SRC-007 | behoben (Schnittregel 3) |
| N-4 | AP-10 | major | SM-PRV-003 ohne Ordnerüberwachung nur beim Importlauf ausgelöst | SM-PRV-003 | behoben (zweiter Auslöser) |
| N-5 | Implementierungsplan | major | Zwei A-Nachweise mit Messgröße, aber ohne Abnahmeschwelle | SM-NFR-002, SM-SRC-008 | behoben (OP-18) |
| T-1 | Abschnitt 1.2 | major | D-03, D-04, D-06, D-11 und AK-07 in AP-11 erstnachgewiesen, setzen aber AP-12 voraus | SM-NFR-007 | behoben (Abschnitt 4.2) |
| T-2 | Implementierungsplan | major | Änderung an `CLAUDE.md` erzeugte vier neue Widersprüche | `CLAUDE.md` Abs. 3, 4, 8, 9 | behoben |
| T-3 | Implementierungsplan | major | Analyse behauptete im Endzustand, keine bestehende Datei zu ändern | `CLAUDE.md` Abs. 10 | behoben (Abschnitt 2) |
| Te-1 | AP-03 | major | Plan legte SM-FMT-012 und SM-SEC-011 enger aus als ihr Wortlaut; PDF-Pfad ohne Fuzzing-Ziel | SM-SEC-011, SM-FMT-012 | behoben (PF-SEC-11.2, OP-14 neu gefasst) |
| Te-2 | AP-14 | major | SM-SEC-008 nur `ui/detail` zugeordnet; Kachel, Chips, Stapelvorschau und Analysewerte ungebunden | SM-SEC-008 | behoben (querschnittlich) |
| C-1 | AP-12 | major | Fokusfang und Fokusrückgabe ohne Träger — AP-11 kann den Nachweis nicht führen | SM-NFR-008 | behoben (PF-NFR-08.1/.2) |
| C-2 | Abschnitt 1.2 | major | SM-NFR-002 an einer Oberfläche gemessen, die es zum Messzeitpunkt nicht gibt | SM-NFR-002 | behoben (deckungsgleich N-1) |
| C-3 | Kapitel 10.1 | major | Zustandstrio nur für einen einzigen Bildschirm verplant | OP-15 | behoben (querschnittlich) |
| C-4 | Kapitel 3/AP-12 | major | Mindestgrößen von Bedienelementen kommen im Plan nicht vor | OP-16 | behoben |
| C-5 | Abschnitt 4.2 | major | Gatterregel weichte die Frist des Lastenhefts für OP-07, OP-09 und OP-15 auf | `CLAUDE.md` Abs. 3 | behoben (Abschnitt 2.1) |

Die minor-Befunde beider Runden sind ebenfalls abgearbeitet; sie betrafen überwiegend
Querverweise, Zeilenlängen, fehlende Prüfbedingungen und Begründungen, die nur den Gegenstand
statt den Grund nannten.

**8.7 Dritte Runde: drei weitere Soll-Anforderungen nachgezogen, Prüffälle entkoppelt.** Auch die
dritte Runde endete 0/4, mit 38 Befunden (0 blocker, 12 major, 26 minor). Die Schweregrade sanken,
die Befunde trafen jetzt überwiegend Folgefehler der Korrekturen aus Runde 2:

- **Drei Logikfehler in meinen eigenen Korrekturen.** Der Vorfilter der Duplikaterkennung war mit
  „oder" statt „und" formuliert — die zweite Bedingung ist für jede bereits importierte Datei
  wahr, ein inkrementeller Lauf hätte den Gesamtbestand gehasht. Die Gültigkeitsprüfung des
  Vorschau-Zwischenspeichers erlaubte „ersatzweise" den Inhaltshash, was im Lesepfad die
  vollständige Quelldatei je Kachel gelesen hätte. Beides berichtigt.
- **SM-FMT-010, SM-MET-010 und SM-SET-003 nachgezogen** (Umfang jetzt 134). SM-DES-008 (Muss)
  verlangt den Detailabschnitt „Farben"; ohne die ersten beiden Kennungen wäre eine
  Muss-Anforderung dauerhaft teilweise unerfüllt geblieben — für Soll-Streichungen sieht
  URS-STM-001 Abschnitt 1.4 diesen Weg vor, für Muss-Anforderungen nicht. SM-SET-003 trägt die
  Persistenz der Modus-Wahl, die der Plan zuvor unter SM-SET-001 geführt hatte, deren Wortlaut sie
  nicht deckt.
- **Prüffälle für Leistungen ohne Kennung entkoppelt.** Zustandstrio und Mindestgrößen hingen als
  Unterfälle an SM-DES-003 und SM-DES-008 — dieselbe Fehlerklasse wie Te-1 aus Runde 2. Sie tragen
  jetzt eigene, vorläufige Kennungen `PF-OP15-*` und `PF-OP16-*` und stehen bewusst nicht in der
  Matrix.
- **AP-19 hatte keinen prüfbaren Gegenstand.** SM-MIG-001 bis 003 legen fest, woraus übernommen
  wird, und sind zurückgestellt. Der Plan legt jetzt ausdrücklich fest, dass SM-MIG-005 am
  Importweg nachgewiesen wird und AK-10 gegenstandslos ist.
- **Zwei weitere Zirkelschlüsse:** SM-SEC-004 konnte in AP-05 nur leer bestehen — dort existiert
  keine Oberfläche; SM-SET-006 und SM-NFR-006 hatten ihren Regressionslauf vor dem Erstnachweis.
  Beide Erstnachweise liegen jetzt in AP-12.
- **Die Zählwerte des Plans waren auseinandergelaufen** (130 gegen 131, „vier nachgezogene" gegen
  fünf, „drei Schnittregeln" bei vier Einträgen). Die Matrix wird seither **aus den
  Zuordnungslisten des Kapitels 4 erzeugt**, nicht daneben gepflegt; beide können nicht mehr
  auseinanderlaufen.

### 8.8 Befunde der Runde 3

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | AP-22/AP-21 | major | Vorfilter der Duplikaterkennung mit „oder" kehrt seine Wirkung um | SM-IMP-003, SM-IMP-005 | behoben |
| N-2 | AP-02/AP-07 | major | Inhaltshash im Lesepfad des Vorschau-Zwischenspeichers | SM-PRV-002, SM-PRV-007 | behoben |
| N-3 | Kapitel 6.4 | major | OP-08 blockiert Erstnachweise, war aber nur AP-22 zugeordnet | SM-SRC-007, SM-NFR-001 | behoben (Regressionsschwelle) |
| N-4 | AP-10 | major | PF-PRV-07 misst die Zeichenpfad-E/A nicht | SM-PRV-007 | behoben |
| T-1 | Abschnitt 1.2 | major | `CLAUDE.md` nennt das OP-Register mit fester Endnummer | `CLAUDE.md` Abs. 4 | behoben (offen formuliert) |
| T-2 | Implementierungsplan | major | Prüffälle ohne Kennung als Unterfälle fremder Muss-Anforderungen geführt | SM-DES-003, SM-DES-008 | behoben (`PF-OP15-*`, `PF-OP16-*`) |
| T-3 | Implementierungsplan | major | Sechs Unterfälle fehlten in Matrix und Unterfalltabelle | URS-STM-001 Abs. 13.3 | behoben |
| T-4 | Implementierungsplan | major | Persistenz der Modus-Wahl ohne tragende Anforderung | SM-SET-001, SM-SET-003 | behoben (nachgezogen) |
| Te-1 | AP-03 | major | SM-SEC-004 konnte in AP-05 nur leer bestehen | SM-SEC-004 | behoben (querschnittlich) |
| C-1 | AP-12 | major | Regressionslauf vor dem Erstnachweis bei SM-SET-006 und SM-NFR-006 | SM-SET-006, SM-NFR-006 | behoben |
| C-2 | Abschnitt 1.2 | major | SM-DES-008 dauerhaft nur teilweise abnehmbar, ohne Entscheidungsweg | SM-DES-008 | behoben (nachgezogen) |
| C-3 | Kapitel 10.1 | major | PF-MIG-05 ohne prüfbaren Gegenstand | SM-MIG-005, AK-10 | behoben |

Die minor-Befunde der Runde 3 sind ebenfalls abgearbeitet; sie betrafen Zählwerte,
Registerangaben, fehlende Prüfbedingungen und einen Anglizismus.

**8.9 Vierte Runde: Tiefengrenze gezogen.** Auch die vierte Runde endete 0/4 (25 Befunde: 0
blocker, 11 major, 14 minor). Die Befunde waren überwiegend Rückstände der Runde-3-Korrekturen:
Abschnitt 1.3 führte drei Anforderungen weiterhin als fehlend, die Abschnitt 1.2 im selben
Durchgang in den Umfang genommen hatte. Zwei Festlegungen sind neu:

- **Eine prüfbare Tiefengrenze in Abschnitt 0.3.** Der Plan *ordnet zu* — Anforderung,
  Arbeitspaket, Prüffall, Messgröße, Reihenfolge, Vorbedingung. Das Pflichtenheft *legt fest* —
  Datenstrukturen, Verfahren, Zahlenwerte, Merkmale von Prüfbeständen, Abfrageformen. Befunde
  jenseits der Grenze werden als **Pflichtenheft-Vormerkung** in Kapitel 12 des Plans geführt,
  statt im Plan aufgelöst zu werden. Ohne diesen Maßstab zog jede Prüfrunde weitere Festlegungen
  in den Plan, bis er das Pflichtenheft geworden wäre, das er nicht sein will. Entscheidung des
  Nutzers vom 24.08.2026. Fünf Vormerkungen sind bereits eingetragen (PV-01 bis PV-05).
- **SM-SET-002 nachgezogen** (Umfang jetzt 135). Die Persistenz der Modus-Wahl war unter
  SM-SET-003 geführt, deren Wortlaut aber „Panelbreiten und Fensterzustand" nennt; die Übernahme
  der Systemeinstellung mit manueller Übersteuerung liegt in SM-SET-002. Alle neun Nachzüge sind
  vom Nutzer am 24.08.2026 bestätigt.

### 8.10 Befunde der Runde 4

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | Kapitel 6.4 | major | Merkmale des Prüfbestands nicht festgelegt — vier A-Nachweise und PF-IMP-05.1 hängen daran | SM-SRC-007, SM-LIB-009 | als **PV-01** übernommen |
| N-2 | Kapitel 6.4 | major | Messaufbau zu SM-SRC-007 bildet nur eine Abfrageform ab | SM-SRC-007 | als **PV-02** übernommen |
| T-1 | AP-11 | major | D-03, D-04, D-06, D-11 und AK-07 in AP-11 erstnachgewiesen, setzen aber AP-12 voraus | SM-NFR-007 | behoben (Abschnitt 4.2) |
| T-2 | `CLAUDE.md` | major | Die Prüfbereichs-Ergänzung erzeugte an vier weiteren Stellen Widersprüche | `CLAUDE.md` Abs. 3, 4, 8, 9 | behoben |
| T-3 | Analyse Abschnitt 2 | major | Analyse behauptete im Endzustand, keine bestehende Datei zu ändern | `CLAUDE.md` Abs. 10 | behoben |
| C-1 | Abschnitt 1.3 | major | Drei Kennungen zugleich im Umfang und als Reibungsstelle geführt | SM-SET-003, SM-MET-010, SM-FMT-010 | behoben |
| C-2 | AP-05, AP-12 | major | SM-SEC-004 konnte in AP-05 nur leer bestehen | SM-SEC-004 | behoben (`PF-SEC-04.1/.2`) |
| C-3 | Abschnitt 4.2 | major | SM-SEC-008 nur `ui/detail` zugeordnet; vier Anzeigestellen ungebunden | SM-SEC-008 | behoben (querschnittlich) |
| C-4 | AP-11, AP-12 | major | Fokusfang ohne Träger — AP-11 konnte den Nachweis nicht führen | SM-NFR-008 | behoben (`PF-NFR-08.1/.2`) |
| C-5 | AP-16, AP-21 | major | Sandbox-Hälfte von AK-04 und AK-09 nicht getrennt | SM-SEC-014 | behoben |
| C-6 | Abschnitt 2.2 | major | Gatterregel weichte die Frist des Lastenhefts für OP-07, OP-09 und OP-15 auf | `CLAUDE.md` Abs. 3 | behoben (Abschnitt 2.1) |

**8.11 Fünfte Runde: zwei weitere Nachzüge, sieben Vormerkungen.** Die fünfte Runde ergab 28
Befunde (0 blocker, 9 major, 19 minor). Erstmals fielen zwei Befunde ausdrücklich **jenseits** der
in 8.9 gezogenen Tiefengrenze und wurden als Vormerkung übernommen statt im Plan aufgelöst — die
Grenze wirkt also. Inhaltlich:

- **SM-KIA-001 und SM-SET-004 nachgezogen** (Umfang jetzt 137). SM-KIA-007, SM-KIA-008 und
  SM-DES-009 (alle Muss) setzen voraus, dass Vorschläge überhaupt entstehen; SM-KIA-002 deckt nur
  die lokale Verarbeitung. Bei SM-SET-004 galt: Erreichbarkeit ohne Kürzel erfüllt den Buchstaben
  von SM-NFR-008, nicht seinen Zweck.
- **Drei weitere Zirkelschlüsse derselben Klasse:** SM-SRC-005 stand zugleich unter „Zugeordnet"
  und „Mitwirkung"; SM-SEC-005 sollte in AP-04 nachgewiesen werden, wo der Suchpfad noch nicht
  existiert; SM-DES-005/006 verlangten die vollständige Dreiteilung, bevor der Detailbereich
  gebaut ist. Alle drei über geteilte Prüffälle aufgelöst.
- **SM-SEC-008 fehlte im Druckpaket.** Dateiname und Metadaten erscheinen in Druckdialog, Vorschau
  und Seitenkopf; ein auszeichnungsfähiges Textelement dort manipulierte ausgerechnet den
  maßhaltigen Ausdruck.

### 8.12 Befunde der Runde 5

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | AP-22/AP-21 | major | SM-SRC-005 zugleich zugeordnet und als Mitwirkung geführt | SM-SRC-005 | behoben (PF-SRC-05.1/.2) |
| N-2 | AP-02/AP-07 | major | Kosten des Ausschnittabrufs an keine Messgröße gebunden | SM-SRC-007, SM-PRV-007 | behoben; Form als **PV-07** |
| Te-1 | AP-03 | major | SM-SEC-008 nicht an das Druckpaket gebunden | SM-SEC-008, SM-SEC-007 | behoben |
| Te-2 | AP-14 | major | SM-SEC-005 im Suchpfad ohne Prüffall | SM-SEC-005 | behoben (PF-SEC-05.2) |
| Te-3 | Implementierungsplan | major | Sandbox-Hälfte von AK-04 und AK-09 nicht getrennt | SM-SEC-014 | behoben |
| T-1 | Abschnitt 1.2 | major | OP-14 entschied sich in der eigenen Begründung | OP-14 | behoben |
| C-1 | AP-12 | major | Abbruch langer Vorgänge zugesagt, ohne Kennung | SM-IMP-002, OP-15 | behoben (OP-15 erweitert) |
| C-2 | Abschnitt 1.2 | major | Nachweis der Mindestgrößen einem Paket ohne Bedienelemente zugeordnet | OP-16 | behoben |
| C-3 | Kapitel 10.1 | major | KI-Kennzeichnungskette hing an weiter Auslegung | SM-KIA-007, SM-KIA-008 | behoben (SM-KIA-001 nachgezogen) |

Die minor-Befunde beider Runden sind abgearbeitet; sie betrafen Querverweise, Zählwerte,
Prüfbedingungen und sprachliche Einzelheiten.

**8.13 Sechste Runde: Selbstprüfer eingeführt.** Die sechste Runde ergab 16 Befunde (0 blocker, 8
major, 8 minor). Fast alle waren Rückstände der Runde-5-Korrekturen — dieselbe Klasse wie zuvor:
eine Kennung stand zugleich im Umfang und in der Reibungsstellenliste, eine zugleich unter
„Zugeordnet" und unter „Mitwirkung", ein Prüffall fehlte in der Nachweisliste seines Pakets.

Gegen diese Klasse läuft seit dieser Runde ein **maschineller Selbstprüfer vor jedem Gate-Lauf**.
Er prüft sieben Bedingungen: keine Kennung zugleich im Umfang und zurückgestellt; Kapitel 4 und
Matrix deckungsgleich je Arbeitspaket; keine Kennung zugleich zugeordnet und Mitwirkung desselben
Pakets; alle Zählwerte gegen die Matrix; jeder in Kapitel 4 genannte Prüffall in Abschnitt 6.1
oder der Matrix; keine Zeile über 100 Zeichen; jeder OP-Verweis im Lastenheft definiert. **Der
Prüfer liegt außerhalb des Repositorys und ist ausdrücklich kein Bestandteil des Änderungssatzes**
— er ist ein einmaliges, nicht versioniertes Hilfsmittel. Damit die sieben Bedingungen nicht mit
ihm verschwinden, stehen sie oben ausgeschrieben; wer sie dauerhaft erzwingen will, nimmt sie nach
`scripts/` auf und ergänzt einen Selbsttestfall (dann Klasse G).

Inhaltlich neu: **SM-SEC-007 ist an den Dokumentpfad gebunden** (PF-SEC-07.2) — ein Schnittmuster
mit eingebetteter Ressourcenreferenz darf beim Anzeigen keine ausgehende Verbindung auslösen,
sonst bräche die Anzeige eines fremden Dokuments die Offline-Zusage AK-08. **AP-07 läuft jetzt
durch `kern/security`** — Anlegen, Umbenennen, Verschieben und Löschen von Ordnern sind
Schreibvorgänge. Und die Nachweise, deren Gegenstand an OP-05 hängt (SM-KIA-005, entfernter Zweig
von SM-SEC-007), werden bis zur Antwort als **Feststellung der Abwesenheit** geführt statt als
Prüfung einer Zusage — dieselbe Führung wie bei PF-SEC-13.

### 8.14 Befunde der Runde 6

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| T-1 | Abschnitt 1.3 / Kapitel 10.1 | major | Tastaturkürzel zugleich im Umfang und zurückgestellt | SM-SET-004 | behoben |
| T-2 | Kapitel 10.1 | major | Begründungszellen nannten Leistungen nachgezogener Kennungen | SM-FMT-010, SM-MET-010, SM-KIA-001 | behoben |
| T-3 | AP-12 | major | Abbruch langer Vorgänge zugesagt, ohne Kennung und Prüffall | SM-IMP-002, OP-15 | behoben (`PF-OP15-09`) |
| Te-1 | AP-07 | major | Ordner-Schreibpfade nicht an `kern/security` gebunden | SM-SEC-001, SM-SEC-002 | behoben |
| Te-2 | AP-14 | major | SM-SEC-007 nicht an den Dokumentpfad gebunden | SM-SEC-007, AK-08 | behoben (`PF-SEC-07.2`) |
| Te-3 | AP-18 | major | Vier Nachweise ohne Gegenstand, solange kein entfernter Pfad entsteht | SM-KIA-005, OP-05 | behoben (als Feststellung geführt) |
| Te-4 | AP-17, AP-18 | major | SM-SEC-008 fehlte in beiden Mitwirkungslisten | SM-SEC-008 | behoben |
| C-1 | Abschnitt 4.2 | major | SM-NFR-009 und SM-NFR-013 ohne Regressionslauf | SM-NFR-009, SM-NFR-013 | behoben |

**8.15 Siebte Runde.** 16 Befunde (0 blocker, 7 major, 9 minor). Zwei weitere Kennungslücken sind
als **OP-19** (Fortbestehen der Modus-Übersteuerung über den Neustart) und **OP-20**
(Sammelaktionen der Hinweisbox für maschinell erzeugte Werte) ins Lastenheft gegangen; vier
Prüffälle sind geteilt (PF-SET-01, PF-IMP-05, PF-SEC-07, PF-DES-05/06), weil ihr zweiter Teil erst
mit der Oberfläche prüfbar wird. Der Berechtigungssatz des Flatpak-Manifests ist als **PV-08** ins
Vormerkungskapitel gewandert — er ist ein Konfigurationswert und gehört nach der Tiefengrenze
nicht in den Plan.

### 8.16 Befunde der Runde 7

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| T-1 | AP-11, Abschnitt 1.2 | major | Persistenz der Modus-Wahl auf eine Kennung gestützt, die sie nicht deckt | SM-SET-002, SM-SET-003 | behoben (**OP-19**) |
| T-2 | AP-18 | major | Sammelaktionen der Hinweisbox ohne Kennung und ohne Prüffall | SM-KIA-007, DES-STM-001 Abs. 9 | behoben (**OP-20**) |
| N-1 | Kapitel 6.4 | major | Kosten des Ausschnittabrufs an keine Messgröße gebunden | SM-SRC-007, SM-PRV-007 | behoben; Form als **PV-07** |
| Te-1 | AP-15 | major | SM-SEC-008 nicht an das Druckpaket gebunden | SM-SEC-008, SM-SEC-007 | behoben |
| Te-2 | AP-10, AP-04 | major | SM-SEC-005 im Suchpfad ohne Prüffall | SM-SEC-005 | behoben (`PF-SEC-05.2`) |
| C-1 | AP-11, AP-12 | major | PF-SET-01 ungeteilt — Umschalter erst mit der Oberfläche prüfbar | SM-SET-001 | behoben (`PF-SET-01.1/.2`) |
| C-2 | AP-08, AP-12 | major | Entscheidungsvorlage bei Duplikaten ohne Dialoggerüst nachgewiesen | SM-IMP-005 | behoben (`PF-IMP-05.1/.2`) |

**8.17 Achte Runde.** 16 Befunde (0 blocker). Fast alle waren Rückstände des Nachtragens von OP-19
und OP-20 in Runde 7: Das Entscheidungsgatter in Kapitel 2 kannte beide nicht, die Zählwörter im
Plan und in `CLAUDE.md` standen auf dem alten Stand, und die Befundtabellen dieses Abschnitts
waren unvollständig und teils mit Befunden früherer Runden gefüllt. Der Selbstprüfer ist deshalb
um drei Bedingungen erweitert worden: jeder offene Punkt des Lastenhefts muss im
Entscheidungsgatter stehen, die Zahlwörter für die Zahl der offenen Punkte müssen stimmen, und
jede beschriebene Runde braucht ihre Befundtabelle.

### 8.18 Befunde der Runde 8

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| T-1 | Kapitel 2 | major | Entscheidungsgatter kannte OP-19 und OP-20 nicht | OP-19, OP-20 | behoben |
| T-2 | AP-18 | major | Sammelaktionen der Hinweisbox ohne Prüffall im Plan | OP-20 | behoben (`PF-OP20-01`) |
| T-3 | Abschnitt 2.4, Kapitel 0.1 | major | Der Plan zählte fünf offene Punkte, wo sieben entstanden sind | OP-14 bis OP-20 | behoben |
| T-4 | `CLAUDE.md` Abschnitt 8 | major | OP-Register unvollständig fortgeführt | `CLAUDE.md` Abs. 4 | behoben |
| T-5 | Analyse 8.6 ff. | major | Befundübernahme unvollständig und in sich widersprüchlich | `CLAUDE.md` Abs. 15 | behoben |
| C-1 | Analyse 8.10 | major | Runde-4-Tabelle enthielt Befunde der Runden 2 und 3 | `CLAUDE.md` Abs. 15 | behoben (neu geschrieben) |
| C-2 | AP-16 | major | Export ohne Fortschritt und ohne Abbruch, anders als Import und Stapel | SM-NFR-006, OP-15 | behoben (`PF-OP15-10`) |
| C-3 | AP-11 | major | Layoutmaße aus DES-STM-001 Abschnitt 5 und 6.3 nicht als Bezeichner geführt | SM-DES-003 | behoben |

**8.19 Neunte Runde.** 24 Befunde (0 blocker, 7 major, 17 minor).

### 8.19a Befunde der Runde 9

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| T-1 | AP-13 | major | Farbliste im Detailbereich nicht als virtualisiert geführt | SM-PRV-007 | behoben (`PF-PRV-07.1/.2`) |
| T-2 | AP-11, AP-12, AP-18 | major | Sperrende offene Punkte fehlten in den Vorbedingungen der Pakete | OP-16, OP-19, OP-20 | behoben |
| Te-1 | Kapitel 3, Abschnitt 4.2 | major | SM-SEC-008 nicht an die Navigationsspalte gebunden | SM-SEC-008 | behoben |
| Te-2 | AP-18, Abschnitt 6.3 | major | PF-SEC-06, PF-KIA-10 und AK-09 ohne Gegenstand, solange OP-05 offen ist | SM-KIA-010, SM-SEC-006 | behoben (als Feststellung geführt) |
| C-1 | AP-11, AP-12 | major | Zwei offene Punkte sperrten Pakete, standen aber nicht in deren Vorbedingungen | OP-19 | behoben |
| C-2 | Analyse 8.6 ff. | major | Ort-Spalte in drei Runden falsch | `CLAUDE.md` Abs. 15 | behoben |
| N-1 | Kapitel 3 | major | Keine Regel gegen N+1-Abfragen an der Fassade | SM-SRC-007 | behoben (Schnittregel 4) |

**8.20 Zehnte Runde.** Zwei Befunde der Runde 9 gingen auf Schäden zurück, die mein automatisches
Umbrechen langer Zeilen verursacht hatte: eine verdoppelte Halbzeile und ein vierfaches Sternchen,
beide mitten in einer Zusage. Der Selbstprüfer prüft seither je Absatz auf ungerade Zahl von
Auszeichnungszeichen, vierfache Sternchen und wiederholte Wortfolgen — er hat beide Schäden vor
dem Lauf gefunden, nachdem die Prüfung von Zeilen- auf Absatzebene umgestellt war.

### 8.21 Befunde der Runde 10

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| Te-1 | AP-04 | major | Wiederherstellungspfad nicht als Fremddatenpfad geführt | SM-DAT-001, SM-NFR-005 | behoben (`PF-NFR-05.5`) |
| T-1 | AP-11 | major | Persistenz der Modus-Wahl als Zusage formuliert, obwohl OP-19 offen ist | SM-SET-002 | behoben |
| T-2 | AP-12 | major | Betriebsmodus als erreichbar zugesagt, obwohl SM-SET-008 zurückgestellt ist | SM-SET-008 | behoben |
| T-3 | Analyse 8.13 ff. | major | Rundenabschnitt und Befundtabelle für Runde 9 fehlten | `CLAUDE.md` Abs. 15 | behoben |
| C-1 | AP-12 | major | Einstellungsfläche mit falscher Begründung und ohne Prüffall | SM-SET-008 | behoben |
| C-2 | AP-04, AP-12 | major | SM-DAT-001 ohne Bedienweg — Sichern und Wiederherstellen blieben kernseitig | SM-DAT-001 | behoben (`PF-DAT-01.1/.2`) |
| C-3 | Abschnitt 6.1 | major | `PF-OP16-03` entschied OP-16 zugunsten des kleineren Maßes | OP-16 | behoben |
| N-1 | Kapitel 3 | major | Gesamtzahl je Ausschnitt statt je Suchlauf ermittelt | SM-SRC-007 | behoben (Schnittregel 3) |

**8.22 Elfte Runde.** Die Prüfbedingungen des Selbstprüfers liegen seit dieser Runde als
`scripts/check-plan.sh` **im Repository**, mit eigenem Selbsttest (`--selftest`) und eingehängt in
die Stufe 0c über `scripts/check-docs.sh`. Zwei Reviewer hatten unabhängig bemängelt, dass zehn
Prüfbedingungen ohne Träger im Baum nicht nachvollziehbar und nicht erzwingbar sind — zu Recht:
Ein Hilfsmittel im Arbeitsverzeichnis verschwindet mit der Sitzung, eine Prüfung im Gate nicht.

### 8.23 Befunde der Runde 11

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | `scripts/review-gate.sh` | major | Gate-Signatur band die neue Prüflogik nicht — ein alter Cache hätte sie überspringen können | `CLAUDE.md` Abs. 13 | behoben |
| N-2 | Kapitel 3 | major | Schnittregel 3 band die Gesamtzahl an den Ausschnitt statt an den Suchlauf | SM-SRC-007 | behoben |
| T-1 | Analyse Abschnitt 2 | major | Endzustand nannte die neuen Skripte nicht | `CLAUDE.md` Abs. 10 | behoben |
| T-2 | Analyse 8.20 ff. | major | Runde 11 ohne Befundtabelle, Zählwerte unvollständig | `CLAUDE.md` Abs. 15 | behoben |
| T-3 | `scripts/check-plan.sh` | major | Prüfung 10 erkannte fehlende Befundtabellen nicht | `CLAUDE.md` Abs. 13 | behoben (Nummern statt Zahlwörter, Mengenvergleich) |
| T-4 | `scripts/check-plan.sh` | major | `block()` lieferte bei fehlendem Anker still eine leere Menge — Prüfung 1 wurde zum No-Op | fail-closed | behoben |
| T-5 | `scripts/check-docs.sh` | major | Fehlender Plan wurde als PASS gewertet statt als ENTFÄLLT | `CLAUDE.md` Abs. 13 | behoben (Exit 3) |
| Te-1 | `scripts/check-plan.sh` | major | Neues Gate-Skript lag außerhalb der Gate-Signatur | `CLAUDE.md` Abs. 13 | behoben |
| Te-2 | Analyse 8.21 | major | Sicherheitsbefund als behoben geführt, `PF-NFR-05.5` existierte nicht | SM-NFR-005, SM-DAT-001 | behoben — Prüffall angelegt |
| C-1 | AP-12 | major | Bedienweg zu den Einstellungen ohne Prüffall | SM-SET-001 | behoben |

**8.24 Zwölfte Runde.** Der Prüfer selbst war in Runde 11 der Hauptgegenstand — zu Recht: Er
hatte zwei fail-open-Pfade (`block()` ohne Anker, fehlender Plan als PASS) und eine Bedingung,
die ihren eigenen Gegenstand nicht erkannte. Alle drei sind behoben, und der Selbsttest läuft in
Stufe 0b mit. **Die ursprüngliche Fassung dieses Absatzes behauptete, er decke seither jede der
zehn Bedingungen mit einem eigenen Negativfall ab. Das traf nicht zu** und ist in Runde 13 als
T-5 gemeldet worden: Für Bedingung 10 fehlte ein Fall, Bedingung 7 war nur in einer Richtung
abgedeckt. Der tatsächliche Deckungsgrad steht seit Runde 13 in `scripts/check-plan.test.sh`
selbst; die fehlenden Fälle sind dort ergänzt. Zwei Befunde der Runde 10 waren zudem nur in
der Analyse als behoben geführt, im Plan
aber nie angekommen — ein abgebrochener Bearbeitungslauf hatte sie verworfen. Beide sind
nachgetragen.

### 8.25 Befunde der Runde 12

Quelle: `Reviews/20260824-203343_main.md`, 0/4, nicht committet. **24 Befunde (0 blocker,
6 major, 18 minor).** Ort, Kennung, Schweregrad und Status — nicht der beanstandete Wert selbst
(`CLAUDE.md` Abschnitt 15).

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | `scripts/review-gate.sh` | major | Die Gate-Signatur fasste den neu eingehängten Planprüfer nicht — ein alter Cache hätte die neue Prüflogik überspringen können | `CLAUDE.md` Abs. 13 | behoben |
| T-1 | Analyse Abschnitt 2 | major | Die als Endzustand deklarierte Komponententabelle nannte die neuen und geänderten Skripte nicht | `CLAUDE.md` Abs. 10 | behoben |
| T-2 | Analyse 8.20 und 8.22 | major | Beschriebene Runden ohne Befundtabelle und ohne Zählwerte — die Regel wurde im selben Änderungssatz eingeführt und verletzt | `CLAUDE.md` Abs. 15 | behoben |
| Te-1 | `scripts/check-plan.sh` | major | Das neue Gate-Skript lag außerhalb der Gate-Signatur | `CLAUDE.md` Abs. 13 | behoben |
| Te-2 | Analyse 8.21 | major | Ein Sicherheitsbefund war als behoben geführt, der benannte Prüffall existierte nicht | SM-NFR-005 | behoben — Prüffall angelegt; die Matrixzeile blieb offen und wurde in Runde 13 erneut gemeldet |
| C-1 | Plan, AP-11 | major | Der Erstnachweis der Tastaturbedienung lag in einem Paket ohne Bedienelemente und hätte nur leer bestehen können | SM-NFR-008, D-06 | behoben — Erstnachweis geteilt |

Die 18 minor betrafen Zählwerte, Anführungszeichen, die Meldung entfallener Prüfungen und die
Abdeckung des Selbsttests; sie sind abgearbeitet, soweit sie Änderungsbezug hatten.

**8.26 Dreizehnte Runde.** Der Prüfer trug diesmal, was er im Vorlauf nicht sah: Drei Reviewer
meldeten unabhängig dieselbe Matrixlücke, die Bedingung 5 wegen einer zu engen Kennungsregex
nicht erkennen konnte — sie schnitt den Unterfallteil `.N` ab. Zwei weitere Befunde trafen den
Prüfer selbst. Der Zuschnitt der Runde war zudem breiter als der des Plans: Der Änderungssatz
trug seit dieser Runde auch die `README.md` und deren Analyse.

### 8.27 Befunde der Runde 13

Quelle: `Reviews/20260824-205225_main.md`, 0/4, nicht committet. **23 Befunde (0 blocker,
11 major-Meldungen, nach Zusammenführung 9 verschiedene, 12 minor).**

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | Plan, AP-09 und Messaufbau `PF-PRV-02` | major | Die Messung der Zwischenspeicherbelegung trug keine Bestehbedingung und konnte damit nie durchfallen | SM-PRV-002, SM-NFR-001 | behoben — Bestehbedingung aufgenommen, Zahlenwert und Verfahren als **PV-09** vorgemerkt |
| T-1 · Te-1 · C-3 | Plan, Kapitel 7 (SM-NFR-005) | major | Die Matrixzeile führte einen in Abschnitt 6.1 definierten Unterfall nicht — dreifach unabhängig gemeldet, Rückstand aus Runde 12 | SM-NFR-005, URS Abs. 13.3 | behoben |
| T-2 | Plan, Kapitel 7 (SM-PRV-007) | major | Dieselbe Klasse an einer zweiten Kennung | SM-PRV-007 | behoben |
| T-3 | Plan, Abschnitt 4.2 und AP-16 | major | Die rechte Statusleistengruppe hatte keinen Bauort; ein Prüfpunkt hätte dort nur leer bestehen können | SM-EXP-004, D-11 | behoben — Bauort in AP-16, Verweis in 4.2 berichtigt |
| T-4 | Analyse 8.24 | major | Beschriebene Runde ohne Befundtabelle — dieselbe Klasse wie T-2 der Runde 12 | `CLAUDE.md` Abs. 15 | behoben — Abschnitt 8.25 |
| T-5 | `scripts/check-plan.sh`, Bedingung 10 | major | Fail-open: Die Rundennummer kam aus einer beliebigen Erwähnung im Fließtext statt aus der Überschrift, dazu eine zu weit gehende Aussage über die Abdeckung des Selbsttests | `CLAUDE.md` Abs. 13 | behoben — Ordinalwörter, Aussage zurückgenommen |
| Te-2 | `scripts/check-docs.sh`, Prüfung 8 | major | Fehlender Prüfer bei vorhandenem Gegenstand wurde als ENTFÄLLT gemeldet statt als FAIL | S3, SM-NFR-012 | behoben |
| C-1 | Plan, Abschnitt 4.2 gegen AP-12 und AP-14 bis AP-18 | major | Drei querschnittliche Zusagen waren nur in einem Paket verankert; Abweichungen wären erst nach der Paketierung aufgefallen | SM-NFR-007, SM-NFR-009, SM-NFR-013 | behoben — Mitwirkung und Nachweis in sechs Paketen ergänzt |
| C-2 | Plan, AP-11 und Abschnitt 4.2 | major | Der Plan las eine Vorgabe der Design-Beschreibung enger, als sie geschrieben ist: Übergänge statt Übergänge **und** Bewegungen | SM-NFR-013 | behoben |

Von den 12 minor sind die drei an der `README.md` behoben (Aussage zur Werkzeug-Anwendbarkeit,
dritte Fundstelle der Rollendimensionen, abgeschriebene statt verlinkter Werte); zwei weitere
waren durch den Markdown-Änderungssatz bereits erledigt. Der Rest ist Beobachtung ohne
Änderungsbezug.

**Zwei Befunde dieser Runde haben den Prüfer erweitert:** Bedingung 5 löst Bereichs- und
Aufzählungsschreibweisen von Unterfällen jetzt auf, und die neue Bedingung 5b vergleicht die
Unterfallmenge aus Abschnitt 6.1 mit der Matrix. Beide hätten T-1 und T-2 gefunden.

**8.28 Vierzehnte Runde.** Die Runde hat zwei Dinge gezeigt. Erstens: Wer Prüfbedingungen
ergänzt, muss Regelwerk, Selbsttestzusage und Analysetabelle im selben Zug nachziehen — vier
der acht major-Befunde waren Rückstände genau dieser Art aus der Behebung der Runde 13.
Zweitens: Der Versuch, den Markdown-Änderungssatz getrennt zu führen, hat drei Reviewer
unabhängig denselben Befund melden lassen — die `README.md` sagte einen Bedienschritt zu, den
der Baum ohne den zweiten Satz nicht hergab. Beide Sätze sind daraufhin zusammengeführt worden.

### 8.29 Befunde der Runde 14

Quelle: `Reviews/20260824-212927_main.md`, 0/4, nicht committet. **20 Befunde (0 blocker,
8 major, 11 minor, 1 Beobachtung).**

| Nr | Ort | Schwere | Befund | Kennung | Status |
|---|---|---|---|---|---|
| N-1 | Plan, Messaufbau `PF-LIB-09`/`PF-NFR-01` | major | Der Leistungsnachweis für 100.000 Einträge trug keine Bestehbedingung — dieselbe Klasse wie N-1 der Vorrunde an anderer Zeile | SM-NFR-001, SM-LIB-009 | behoben — binäre Bedingung, ohne Zahlenwert und damit unabhängig von OP-08 |
| N-2 · T-5 · C-1 | `README.md` gegen `scripts/check-docs.sh` | major | Die README erklärte einen Bedienschritt für verbindlich, den der Änderungssatz nicht hergab, und begründete ihn mit einer Gate-Wirkung, die das Skript nicht hatte — dreifach unabhängig gemeldet | S3, `CLAUDE.md` Abs. 1 und 13 | behoben — die Änderungssätze sind zusammengeführt, damit trifft die Zusage zu |
| T-1 | `Design/StitchManager_Design_Beschreibung.md`, Abschnitt 13 | major | Fünf neue offene Punkte betreffen Verhalten, das dieses Dokument verbindlich beschreibt, standen dort aber nicht — wer Abschnitt 7 oder 10 umsetzt, sah den fehlenden Abnahmebezug nicht | OP-15 bis OP-17, OP-19, OP-20 | behoben — DES 1.2 auf **1.3**, Historieneintrag, Kopfzeilen in URS, TEC, Plan und README im selben Commit nachgezogen |
| T-2 · C-4 | `CLAUDE.md`, Abschnitt 11 | major | Das Regelwerk beschrieb den Planprüfer mit einer Anzahl von Bedingungen, die er seit der Vorrunde nicht mehr hat | `CLAUDE.md` Abs. 11 und 13 | behoben — Aufzählung vollständig, Anzahl bewusst entfernt |
| T-3 · C-3 | `scripts/check-plan.test.sh`, Kopf | major | Die Zusage „je Prüfbedingung ein Negativfall" deckte die neue Bedingung 11 nicht — derselbe Rückfall, den Runde 13 als T-5 gemeldet hatte | `CLAUDE.md` Abs. 13 | behoben — vier neue Fälle, zusätzlich meldet eine fehlende Zeile jetzt einen Befund statt still durchzugehen |
| T-4 | `Analysis/20260824_01_readme.md`, Abschnitt 2 | major | Dieselbe Datei stand in der Endzustandstabelle als geändert und drei Zeilen tiefer als ungeändert | `CLAUDE.md` Abs. 10 | behoben |
| Te-1 | Plan, AP-04 und Abschnitt 6.1 | major | Der Wiederherstellungspfad ist der dritte Fremddatenparser, hatte aber nur einen statischen Prüfbestand und kein dauerhaftes Fuzzing-Ziel | SM-SEC-011, SM-FMT-012 | behoben — Unterfall `PF-SEC-11.3`, Auslöser des Fuzzing-Gates um AP-04 ergänzt |
| C-2 | `scripts/check-plan.sh`, Kopf | major | `--selftest` war an zwei Stellen zugesagt, aber nicht gebaut; der Aufruf lief als gewöhnliche Prüfung durch | SM-NFR-012 | behoben — der Schalter existiert |
| C-5 | Plan, AP-12 gegen Kapitel 7 | major | Eine Zusage des Pakets hatte keinen Prüffall, der auf sie fällt | SM-LIB-001, SM-SET-002 | behoben — je zwei Unterfälle angelegt und in AP-07, AP-11 und AP-12 verankert |
| N-x | Plan, Messaufbau `PF-PRV-02` | major | SM-PRV-002 fordert Dauerhaftigkeit, nicht Beschränktheit des Zwischenspeichers — ohne Schranke keine Bestehbedingung | SM-PRV-002, SM-NFR-001 | behoben: als **OP-21** ins Lastenheft, Zahlenwert als **PV-09** |

Von den elf minor sind die behoben, die Änderungsbezug hatten; die übrigen sind Beobachtung
nach S4. Die einzelne Beobachtung betrifft SM-NFR-004 ohne Abnahmebezug in Version 1.0.

## 9. Abschluss

*Wird nach bestandenem Gate ergänzt (CLAUDE.md Abschnitt 10, Phase 4).*
