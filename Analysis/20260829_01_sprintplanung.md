# Analyse — aktueller Stand und Sprint zur weiteren Umsetzung

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260829-01 |
| Datum | 29.08.2026 |
| Auslöser | Auftrag: aktuellen Stand analysieren und einen Sprint zur weiteren Umsetzung der Anforderungen erstellen |
| Änderungsklasse | **T** — Analyse und Planung; keine Anforderung und keine Entscheidung wird in diesem Dokument begründet |
| Führende Quelle | URS-STM-001 v1.3 |
| Planungsquelle | IMP-STM-001 v1.0 |
| Vorgänger | `Analysis/20260826_02_bestandsaufnahme.md` |

## 1. Ergebnis in Kürze

Der Quellbaum besitzt einen tragfähigen Rust- und Qt-Unterbau, ist aber noch kein belastbarer
Stand von Version 1.0. Die automatisierten Prüfungen sind am 29.08.2026 grün: 173 Rust-Prüffälle
bestehen, 14 sind ausgesetzt; Formatierung, Clippy, QML-, Projektregel-, Dokument- und
Planprüfung bestehen. Die vorhandene Messung mit 100.000 Datenbankeinträgen bleibt mit einem
schlechtesten Suchwert von rund 390 ms unter der Sekundengrenze.

Diese Zahlen sind kein Fertigstellungsgrad. In der Rückverfolgbarkeitsmatrix stehen weiterhin
alle 137 für Version 1.0 verplanten Anforderungen auf `offen`. Mehrere Arbeitspakete, die die
Bestandsaufnahme vom 26.08.2026 als „gebaut“ bezeichnet, sind nur teilweise umgesetzt. Besonders
sichtbar sind die fehlende Verwaltung mehrerer Bibliothekswurzeln und Ordner, die nur teilweise
vorhandenen Filter, die fehlende Entscheidungsvorlage für Duplikate und der leere Detailbereich.

Vor dem Beginn eines weiteren Oberflächenpakets müssen außerdem die Entscheidungsgatter aus
Kapitel 14 des Lastenhefts geschlossen und im führenden Dokument fortgeschrieben werden. Der
Quellbaum folgt bereits Weg A mit cxx-qt, während OP-13 und die oberflächenbezogenen Punkte im
Lastenheft formal offen sind. Der nächste Sprint kombiniert deshalb Konsolidierung mit einem
kleinen, tatsächlich abschließbaren Anforderungsumfang.

## 2. Geprüfter Ausgangsstand

### 2.1 Repository und Umfang

- Zweig `main` liegt einen Commit vor `origin/main`.
- Der Arbeitsbaum enthält verfolgte Änderungen und einen großen unverfolgten Quell- und
  Dokumentbestand. Der Anwendungsquellbaum ist damit noch nicht vollständig in der
  Versionsgeschichte gesichert.
- Der Cargo-Verbund umfasst acht Kisten über `members = ["crates/*"]`.
- Rust-, QML- und Paketdateien unter `crates/` umfassen zusammen rund 12.500 Zeilen.
- Der vorhandene Aufteilungsplan in `Analysis/20260826_02_bestandsaufnahme.md` sieht fünf
  prüfbare Änderungssätze für die Aufnahme des Quellbaums vor; nur Satz 1a ist bereits
  versioniert.

Die unverfolgten und geänderten Dateien sind bestehender Nutzerstand. Diese Analyse verändert
oder verwirft davon nichts.

### 2.2 Ausgeführte Prüfungen

| Prüfung | Ergebnis am 29.08.2026 | Einordnung |
|---|---|---|
| `cargo test --workspace --all-targets` | 173 bestanden, 14 ausgesetzt | Kernbestand grün; die aus externem Prüfbestand abhängigen Parserfälle und zwei Messfälle laufen nicht im Regelfall |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | bestanden | alle acht Kisten erfasst |
| `cargo fmt --all -- --check` | bestanden | Formatierung sauber |
| `bash scripts/check-qml.sh` | bestanden, acht Dateien | QML ist syntaktisch und formal sauber |
| `bash scripts/check-projektregeln.sh` | bestanden | Gestaltungsliterale, Schichttrennung, Abfragen, Installationsskripte und Versionen ohne Befund |
| `bash scripts/check-docs.sh` | bestanden, eine Prüfung entfallen | 224 Kennungen definiert, 21 offene Punkte, Dokumentverweise konsistent |
| `bash scripts/check-plan.sh` | bestanden | 137 Anforderungen, 23 Arbeitspakete, 21 offene Punkte |
| 100.000er-Messfälle in `kern-db` | bestanden | schlechtester Suchwert rund 390 ms; zwei Abfragen je Ausschnitt unabhängig von dessen Größe |

Die Leistungsmessung ist eine Regressionsschwelle, keine Abnahme. OP-08 benennt weiterhin kein
Referenzgerät; eine bestandene Messung auf dem aktuellen Rechner darf deshalb nicht als
Abnahmenachweis eingetragen werden.

## 3. Tatsächlicher Stand der Arbeitspakete

Die Bewertung unterscheidet bewusst zwischen vorhandenem Code und abgeschlossenem Paket. Ein
Paket ist erst abgeschlossen, wenn Ergebnis, Prüffälle und Nachweis nach IMP-STM-001 Abschnitt
4.1 vorliegen.

| Paket | Bewertung | Beleg und wesentliche Lücke |
|---|---|---|
| AP-00 | **nicht abgeschlossen** | Weg A ist gebaut und in einer Analyse als Nutzerentscheidung belegt; OP-13 und die Wegentscheidung sind im führenden Register nicht fortgeschrieben. Ein vollständiger Prototypvergleich samt körperlichem Testdruck liegt nicht vor |
| AP-01 bis AP-02 | **teilweise tragfähig** | Lizenz-, Herkunfts- und Prüfgates bestehen. Plattformbauläufe, vollständiger externer Prüfbestand und formale Nachweise sind noch nicht abgeschlossen |
| AP-03 | **teilweise umgesetzt** | Bibliothekspfade, Symlinks und Fremddateinamen sind gehärtet. Export- und Sicherungsziele existieren als Anwendungswege noch nicht und können daher nicht nachgewiesen werden |
| AP-04 | **teilweise umgesetzt** | SQLite, WAL, additive Migrationen und FTS5 sind vorhanden. Bedienbare Sicherung und Wiederherstellung sowie deren sichere Archivpfade fehlen |
| AP-05 | **teilweise umgesetzt** | Die Fassade erzwingt die Schichttrennung und liefert Ausschnitte. Schnittstellen für Detail, Dokumente, Export, Druck und weitere Pakete fehlen erwartungsgemäß |
| AP-06 | **teilweise umgesetzt** | PES, DST, JEF und VP3 werden gelesen; vier Fuzzing-Ziele liegen vor. Der Import wählt den Parser zunächst über die Dateiendung; SM-FMT-013 verlangt die Erkennung am Inhalt. Zwölf Parserfälle hängen an einem nicht mitgelieferten Prüfbestand |
| AP-07 | **nicht vollständig gebaut** | Rekursives Einlesen einer einzelnen Wurzel und 100.000er-Datenhaltung bestehen. Mehrere Wurzeln, Ordnernavigation sowie Anlegen, Umbenennen, Verschieben und Löschen von Ordnern fehlen |
| AP-08 | **teilweise umgesetzt** | Hintergrundlauf, Fortschritt, Abbruch, inkrementelle Erkennung und Fehlererfassung bestehen. Die Entscheidungsvorlage für Duplikate aus SM-IMP-005 fehlt |
| AP-09 | **kernseitig weitgehend gebaut** | Vorschauerzeugung, dauerhafter Zwischenspeicher und Verwerfung bestehen. OP-21 verhindert weiterhin eine abnehmbare Aussage zu Obergrenze und Verdrängung |
| AP-10 | **teilweise umgesetzt** | FTS5, Ausschnitte, Sortierung und mehrere Kernfilter bestehen; die 100.000er-Messung ist grün. Größen- und Quellenfilter fehlen, die Oberfläche bietet derzeit nur den Formatfilter |
| AP-11 | **teilweise umgesetzt** | Eine zentrale Gestaltung, Hell-/Dunkelmodus und QML-Bauteile bestehen. Systemübernahme, Sitzungsfortbestand und mehrere Nachweise hängen an offenen Entscheidungen oder fehlen |
| AP-12 | **teilweise umgesetzt** | Hauptfenster, Kachelraster, Suche, Formatfilter, Sortierung, Importfortschritt und Abbruch laufen. Verschiebbare Trenner, Ordnerbaum, Übersichtskarte, vollständige Filterchips, Fehler-/Vermisstzustände, Lösch- und Sicherungsdialoge sowie mehrere Persistenz- und Fokusnachweise fehlen |
| AP-13 bis AP-22 | **offen** | Der Detailbereich ist ein Leerzustand. PDF, Druck, Exportdienst, Stapelverarbeitung, lokale Analyse, Auslieferung und Abnahmenachweise fehlen. Vorarbeiten wie PES-/DST-Schreiber und technische Fehlertexte erfüllen noch kein Paket |

Eine exakte Zahl „erfüllter Anforderungen“ lässt sich aus dem vorhandenen Code nicht seriös
ableiten, solange die zugeordneten Prüffälle nicht einzeln ausgeführt und in der Matrix
eingetragen sind. Der formal belastbare Wert lautet deshalb weiterhin: **0 von 137 als bestanden
protokolliert**, nicht „0 umgesetzt“.

## 4. Wichtigste Abweichungen und Risiken

### 4.1 Die bisherige Paketbewertung ist zu optimistisch

Die Bestandsaufnahme vom 26.08.2026 setzt AP-03 bis AP-12 überwiegend auf „gebaut“. Der heutige
Abgleich mit den Paket-Ergebnissen zeigt, dass diese Einordnung Codevorhandensein mit
Paketabschluss vermischt. Beispiel: Die Tabelle `ordner` existiert, es gibt aber keinen
Ordnerdienst und keinen Ordnerbaum. Ebenso besitzt die Datenbank Felder für Metadaten, während
der Detailbereich weder liest noch schreibt.

### 4.2 Formale Entscheidungsgatter und gebauter Stand widersprechen sich

Alle 21 Punkte in Kapitel 14 des Lastenhefts stehen offen. Vor AP-13 sind mindestens OP-15 und
nach der bestehenden Bestandsanalyse auch OP-16 zu beantworten; AP-11 und AP-12 wurden bereits
trotz offener Gatter begonnen. Weitere Oberflächenentwicklung ohne Fortschreibung des führenden
Dokuments würde diesen Widerspruch vergrößern.

### 4.3 Der Quellbaum ist nicht vollständig versioniert

Ein grüner lokaler Stand ist nicht reproduzierbar, solange wesentliche Dateien unverfolgt
bleiben. Die Aufnahme in prüfbaren Sätzen ist daher keine Aufräumarbeit, sondern Voraussetzung
für jede weitere Anforderungsumsetzung.

### 4.4 Prüfzahl und Produktabdeckung sind nicht dasselbe

Die 173 bestandenen Prüfungen konzentrieren sich auf Parser, Datenhaltung, Pfadsicherheit,
Vorschau und Hintergrundbetrieb. Die größten Produktrisiken — PDF, maßhaltiger Druck,
Datenträgerexport, Metadatenpflege, Paketierung und reale Drei-Plattform-Nachweise — haben noch
keinen ausführbaren End-to-End-Weg.

## 5. Sprint 2026-08-31 — Konsolidierung und erster Detailschnitt

### 5.1 Rahmen

| Merkmal | Festlegung |
|---|---|
| Zeitraum | 31.08.2026 bis 11.09.2026, zehn Arbeitstage |
| Sprintziel | Reproduzierbaren und entscheidungsfähigen Entwicklungsstand herstellen, SM-MIG-005 abschließen und den kernseitigen Vertikalschnitt für den Detailbereich bereitstellen |
| Planlast | 19 relative Aufwandspunkte; vor der Zusage gegen Teamgröße und bisherige Geschwindigkeit zu prüfen |
| Nicht im Sprintziel | vollständiges AP-13, PDF, Druck, Export, Stapelverarbeitung, KI und Paketierung |

### 5.2 Sprint-Backlog

#### SP-01 · Quellstand reproduzierbar versionieren — 5 Punkte

**Nutzen:** Der geprüfte Stand ist nicht mehr nur lokal vorhanden.

**Umfang:** Die noch offenen Sätze 1b bis 5 aus `Analysis/20260826_02_bestandsaufnahme.md` werden
in der dort festgelegten Reihenfolge aufgenommen. Rote Prüfprotokolle bleiben nach der geltenden
Regel außerhalb der Versionsgeschichte. Bestehende Nutzeränderungen werden weder verworfen noch
zusammengezogen.

**Akzeptanzkriterien:**

- alle beabsichtigten Anwendungs-, Analyse- und Werkzeugdateien sind verfolgt;
- jeder Änderungssatz bleibt unter der Prüfkappungsgrenze oder wird nach dokumentierter Regel
  weiter geteilt;
- jeder Satz besteht die für seine Änderungsklasse geltenden Gates;
- `git status` enthält danach nur ausdrücklich benannte, nicht zum Satz gehörende Änderungen;
- kein rotes Review-Protokoll wird als Freigabenachweis versioniert.

#### SP-02 · Entscheidungsgatter der Oberfläche schließen — 3 Punkte

**Nutzen:** AP-13 kann regelkonform begonnen werden.

**Umfang:** Entscheidung und Fortschreibung für OP-13 sowie Weg A, OP-07, OP-09, OP-15, OP-16,
OP-17 und OP-19. Die Werte oder Antworten liefert der Auftraggeber; der Sprint erfindet sie
nicht. Die bereits nachgewiesene Entscheidung zugunsten von Weg A wird in die führende Quelle
überführt, sofern sie bestätigt bleibt.

**Akzeptanzkriterien:**

- jede genannte Frage besitzt eine eindeutige, vom Auftraggeber bestätigte Antwort;
- Kapitel 14 des Lastenhefts ist in einem eigenen Klasse-D-Änderungssatz fortgeschrieben;
- Version, Datum und Änderungshistorie des Lastenhefts sind nachgezogen;
- die Mitgeltend-Kopfzeilen in DES-STM-001 und TEC-STM-001 sind genau einmal nachgezogen;
- Design, Technik und Implementierungsplan enthalten keine abweichende Festlegung.

#### SP-03 · Rückverfolgbarkeit auf den Ist-Stand bringen — 3 Punkte

**Nutzen:** „gebaut“, „geprüft“ und „abgenommen“ sind wieder unterscheidbar.

**Umfang:** Die 137 Matrixzeilen werden gegen vorhandene Prüffälle und ausführbare Nachweise
abgeglichen. Nur tatsächlich ausgeführte, anforderungsdeckende Fälle wechseln von `offen` auf
`bestanden`. Teilnachweise bleiben als solche gekennzeichnet und schließen keine Anforderung.

**Akzeptanzkriterien:**

- jede geänderte Statuszeile nennt einen ausführbaren Prüffall und dessen Ergebnis;
- Messungen ohne festgelegtes Referenzgerät bleiben als Regressionswert gekennzeichnet;
- ausgesetzt laufende Fälle gelten nicht als bestanden;
- die Summenprüfung des Implementierungsplans bleibt grün;
- Abweichungen zwischen Paketbeschreibung und Quellstand sind als Folgeaufgaben erfasst.

#### SP-04 · SM-MIG-005 am Importweg abschließen — 3 Punkte

**Nutzen:** Die unveränderliche Behandlung der Quelldaten wird nicht nur kommentiert, sondern
nachgewiesen.

**Umfang:** PF-MIG-05 wird am Importweg aus AP-08 automatisiert umgesetzt. Er erfasst
Dateiinhalte und Speicherorte vor und nach dem ersten sowie dem inkrementellen Lauf. Ein
abgebrochener Lauf und eine beschädigte, aber erfasste Datei gehören zum Prüfbestand.

**Akzeptanzkriterien:**

- Inhaltshashes aller Quelldateien sind vor und nach jedem Lauf identisch;
- keine Quelldatei wird verschoben, umbenannt, gelöscht oder neu angelegt;
- der Nachweis umfasst Unterordner, einen inkrementellen Zweitlauf, Abbruch und beschädigte Datei;
- ausschließlich Datenbank und Vorschauablage dürfen sich ändern;
- PF-MIG-05 besteht im Regellauf und die Matrixzeile SM-MIG-005 kann belegt nachgezogen werden.

#### SP-05 · Kernseitiger Vertikalschnitt für AP-13 — 5 Punkte

**Nutzen:** Die Auswahl einer Kachel kann danach ohne Schichtbruch und ohne Datei-E/A im
Oberflächenfaden einen vollständigen Detaildatensatz anfordern.

**Umfang:** Ein eng geschnittener Detailtyp und Leseweg durch `kern-db`, `kern-fassade` und
`kern-services`; noch keine bearbeitbare QML-Maske. Der Datensatz enthält mindestens dauerhafte
Kennung, Name, Thema, Beschreibung, Notizen, Format, Abmessungen, Stich- und Farbzahl,
Fehlerzustand, Schlagworte und Garnfarben. Fehlende Felder werden als fehlend übertragen, nicht
erfunden.

**Akzeptanzkriterien:**

- die Oberfläche erhält Detaildaten ausschließlich über `kern-fassade` und `kern-services`;
- Laden, Erfolg, unbekannte Kennung und Datenfehler sind unterscheidbare Antworten;
- kein Datenbank- oder Dateizugriff läuft im Qt-Faden;
- ein Auswahlwechsel kann eine überholte Antwort nicht dem neuen Eintrag zuordnen;
- Kern- und Dienstprüffälle decken vollständigen Datensatz, fehlende Werte und Fehler ab;
- die QML-Anzeige und Bearbeitung bleiben ausdrücklich Folgeumfang und werden nicht als erfülltes
  AP-13 ausgewiesen.

### 5.3 Reihenfolge und Abhängigkeiten

```text
SP-01 Quellstand sichern
  ├── SP-03 Nachweise nachziehen
  └── SP-04 SM-MIG-005 belegen

SP-02 Entscheidungen bestätigen
  └── SP-05 Detail-Leseweg beginnen
```

SP-01 und SP-02 beginnen am ersten Sprinttag. SP-04 kann nach Aufnahme des betroffenen
Kernstands parallel zu SP-03 laufen. SP-05 beginnt erst, wenn SP-02 abgeschlossen ist. Liegen
die Entscheidungen bis Ende des zweiten Arbeitstags nicht vor, wird SP-05 nicht begonnen; das
ist ein Blocker, keine technische Auslegungsfrage.

### 5.4 Definition of Done des Sprints

- Sprintziel und alle zugesagten Akzeptanzkriterien sind erfüllt;
- Formatierung, Clippy, alle nicht ausgesetzten Rust-Prüffälle, QML-, Projektregel-, Dokument-
  und Planprüfung sind grün;
- neue Fachlogik besitzt anforderungsbezogene Prüffälle;
- die Rückverfolgbarkeitsmatrix behauptet keinen nicht ausgeführten Nachweis;
- keine körperliche Druck- oder plattformfremde Messung wird als bestanden protokolliert;
- bekannte Restlücken sind im führenden Register oder als Folgeaufgabe verankert, nicht nur in
  einem Kommentar.

## 6. Folgeplanung nach diesem Sprint

Nach erfolgreichem Sprint ist die sinnvolle Reihenfolge:

1. AP-13 vollständig umsetzen: Detailanzeige, Metadatenbearbeitung, Schlagworte,
   Mehrfachauswahl, ungespeicherte Änderungen und Farbliste einschließlich Stichanteil.
2. AP-20 nach AP-13: Protokollmaskierung sowie Sammelnachweis für Sprache und Fehlertexte.
3. AP-16 nach den dafür nötigen Entscheidungen: Konvertierung, Export und Datenträger.
4. AP-17, danach AP-14 und AP-15; Druck bleibt wegen der körperlichen Messung der kritischste
   Pfad.
5. AP-18 und AP-21 erst nach ihren eigenen Entscheidungsgattern; AP-22 am paketierten
   Auslieferungsstand.

Der nächste Sprint sollte nicht gleichzeitig das vollständige AP-13 und ein weiteres großes
Oberflächenpaket versprechen. AP-13 verändert Schema, Fassade, Hintergrundbetrieb und QML und
trägt sieben unmittelbar zugeordnete Anforderungen sowie zahlreiche Querschnittsnachweise.

## 7. Umsetzungsstand

### 7.1 SP-01 · Quellstand reproduzierbar machen

**Abgeschlossen am 29.08.2026.** Der zuvor unversionierte Bestand wurde in fünf fachlich
getrennten lokalen Änderungssätzen aufgenommen: Werkzeugkette, Planung und Nachweise,
Kernbibliotheken, Daten-/Dienstschicht sowie Desktop-Oberfläche. Die deterministischen Stufen
0a bis 0c waren vor jedem Satz grün. Der unabhängige Reviewer war nach zwei vorgeschriebenen
Rauchtests nicht erreichbar, weil Claude Code nicht angemeldet ist; jeder betroffene Commit
trägt deshalb den vorgeschriebenen Trailer `Gate-Override: reviewer-nicht-erreichbar`. Die
Nachprüfung durch den unabhängigen Reviewer bleibt vor einer Veröffentlichung verpflichtend.

### 7.2 SP-04 · SM-MIG-005

**Technisch abgeschlossen am 29.08.2026.** Der automatisierte Fall
`pf_mig_05_import_veraendert_keine_quelldatei` vergleicht relative Speicherorte und
Inhaltsdigests aller 351 Quelldateien vor und nach einem abgebrochenen Lauf, dem vollständigen
Erstlauf und dem inkrementellen Zweitlauf. Der Bestand enthält verschachtelte Unterordner und
eine beschädigte PES-Datei, die als Fehlereintrag erhalten bleibt. Datenbank und
Vorschauablage liegen außerhalb des Quellbaums. Ausgeführter Nachweis:

```text
cargo test -p kern-services pf_mig_05_import_veraendert_keine_quelldatei
1 bestanden; 0 fehlgeschlagen; 0 ausgesetzt
```

Die Zeile SM-MIG-005 in der führenden Rückverfolgbarkeitsmatrix ist mit genau diesem
Regellauf auf `bestanden` fortgeschrieben.

### 7.3 SP-03 · Rückverfolgbarkeit

**Abgeschlossen am 29.08.2026.** Zunächst wurden alle 137 Matrixzeilen nach Arbeitspaket gegen
den Quellstand und die ausgeführten Kernprüfungen abgeglichen. Nach den in SP-02 bestätigten drei
neuen Muss-Anforderungen umfasst die fortgeschriebene Matrix 140 Zeilen. Acht vollständig
gedeckte Anforderungen stehen auf `bestanden`, 132 bleiben `offen`. Die zwei ausgesetzt laufenden
100.000-Einträge-Messfälle wurden ausdrücklich nicht hochgestuft; ihre vorhandenen Werte sind
ohne das Referenzgerät aus OP-08 nur Regressionswerte. Der Implementierungsplan enthält den
paketweisen Abgleich und die daraus abgeleiteten Folgeaufgaben.

### 7.4 SP-02 · Auftraggeberentscheidungen

**Abgeschlossen am 29.08.2026.** Der Auftraggeber hat alle im Sprint empfohlenen Antworten
bestätigt: Weg A mit Wiederverwendung des Rust-Kerns und cxx-qt, Freigabe der
Festbreitenschrift, vorläufige Farbwerte bis zum Markenabgleich, eigene Anforderungen für
Zustände/Abbruch, Trefferflächen und Übersichtskarte sowie die neustartbeständige manuelle
Darstellungswahl. URS-STM-001 v1.4, DES-STM-001 v1.4, TEC-STM-001 v2.3 und IMP-STM-001 v1.1
sind konsistent fortgeschrieben. Neu entstanden SM-LIB-011, SM-NFR-015 und SM-NFR-016;
SM-SET-002 ist präzisiert. OP-07, OP-09, OP-13, OP-15 bis OP-17 und OP-19 stehen im führenden
Register nachvollziehbar unter den entschiedenen Punkten.
