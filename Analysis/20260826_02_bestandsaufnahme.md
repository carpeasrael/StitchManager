# Analyse — Bestandsaufnahme und Weg zu den offenen Anforderungen

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260826-02 |
| Datum | 2026-08-26 |
| Auslöser | Auftrag des Auftraggebers: „prüfe den aktuellen Stand und implementiere die noch offenen Requirements" |
| Änderungsklasse | **G** — Werkzeug und Regeln. Dieser Satz (1a) ändert die **Wurzelkonfiguration**: `package.json`, `package-lock.json`, `.gitignore`. Das Analysedokument für sich allein wäre T; maßgeblich ist der Änderungssatz, nicht das Dokument (Abschnitt 9 von `CLAUDE.md`, „im Zweifel gilt die höhere Klasse"). Die folgenden Sätze tragen ihre eigene Klasse, siehe Abschnitt 8 |
| Vorgänger | `Analysis/20260826_01_projekt-readme.md` — als Textnennung, nicht als Verweis: Das Dokument kommt erst mit Satz 2 in den Baum (Abschnitt 10), ein Link zeigte bis dahin ins Leere |

## 1. Problembeschreibung

Der Auftrag lautet, die noch offenen Anforderungen umzusetzen. Vor der Umsetzung ist zu
bestimmen, welche der 137 für Version 1.0 verplanten Anforderungen (IMP-STM-001 Abschnitt 1.1)
bereits erfüllt sind und welche nicht. Die Bestandsaufnahme fördert dabei zwei Befunde zutage,
die dem Auftrag in seiner umfassenden Form entgegenstehen und deshalb vor jeder Zeile Code zu
klären sind.

**Befund 1 — der halbe Quellbaum ist nicht übersetzbar und wird von keiner Prüfung erfasst.**
`Cargo.toml` führt sechs der acht vorhandenen Kisten als Verbundmitglieder. `kern-services` und
`ui` liegen im Baum, sind aber **kein** Mitglied. Folge: `cargo check`, `cargo clippy`,
`cargo fmt`, `cargo test --all` und `cargo deny` sehen sie nicht. Beide sind gegen die
Kernschnittstelle veraltet und übersetzen heute nicht mehr.

**Befund 2 — das führende Register offener Punkte ist unverändert leer beantwortet.** Alle 21
Punkte aus Kapitel 14 des Lastenhefts stehen offen. Sieben davon tragen die Frist „vor
Umsetzungsbeginn der Oberfläche"; nach der Gatterregel in IMP-STM-001 Abschnitt 2.4 sperren sie
den **Beginn** von AP-11 bis AP-18. AP-11 und AP-12 sind gleichwohl gebaut. Weitere
Oberflächenpakete zu bauen, vertiefte diesen Widerspruch, statt ihn aufzulösen.

## 2. Betroffene Komponenten

| Komponente | Zustand | Beleg |
|---|---|---|
| `Cargo.toml` | führt sechs von acht Kisten | `members`-Liste, Kommentar „kommen mit ihrem eigenen Änderungssatz" |
| `crates/kern-services` | Bibliothek übersetzt, **Prüffälle nicht** | 5 Übersetzungsfehler in `src/tests.rs`: `kern_security` nicht als Entwicklungsabhängigkeit geführt, `write_dst` erwartet `&Schreibziel` |
| `crates/ui` | übersetzt **nicht** | 4 Übersetzungsfehler in `src/bruecke.rs`: `Kachel` führt `stichzahl`, `farbzahl`, `favorit` und `name_herkunft` nicht mehr |
| `scripts/check-hintergrund.sh` | wirkungslos | baut mit `cargo build --release -p ui`; die Kiste ist kein Verbundmitglied |
| `Requirements/…Lastenheft.md` Kapitel 14 | 21 Punkte offen | Registertabelle, Spalte „Zu klären bis" |

## 3. Stand je Arbeitspaket

Maßstab ist die Rückverfolgbarkeitsmatrix (IMP-STM-001 Kapitel 7), 137 Zeilen. Ein Paket gilt
hier als **gebaut**, wenn Code und Prüffälle vorliegen — nicht als abgeschlossen im Sinne von
Abschnitt 4.1, denn der Erstnachweis ist bei keinem Paket in der Matrix eingetragen.

| AP | Gegenstand | Anf. | Stand |
|---|---|---|---|
| AP-01 | Lizenz- und Herkunftsgrundlage | 10 | gebaut — `deny.toml`, `.lizenzen.conf`, `LICENSE` |
| AP-02 | Projektgerüst und Prüfkette | 1 | gebaut — `scripts/`, drei Selbsttests |
| AP-03 | Pfadsicherheit | 4 | gebaut — `kern-security`, 18 Prüffälle |
| AP-04 | Datenhaltung | 9 | gebaut — `kern-db`, fünf Schemaschritte, 25 Prüffälle |
| AP-05 | Kernfassade | 2 | gebaut — `kern-fassade`, 29 Prüffälle |
| AP-06 | Formatparser | 9 | gebaut — PES, DST, JEF, VP3, vier Fuzzing-Ziele, 56 Prüffälle |
| AP-07 | Bibliothek und Ordnerverwaltung | 4 | gebaut in `kern-services` — **Prüffälle laufen nicht** |
| AP-08 | Import | 6 | gebaut in `kern-services` — **Prüffälle laufen nicht** |
| AP-09 | Vorschauerzeugung | 3 | gebaut — `kern-render`, 21 Prüffälle |
| AP-10 | Suche und Index | 7 | gebaut — FTS5 in `kern-db` |
| AP-11 | Gestaltungsgrundlage | 12 | gebaut in `crates/ui/qml/Gestaltung.qml` — **übersetzt nicht** |
| AP-12 | Hauptfenster und Musterauswahl | 14 | überwiegend gebaut — **übersetzt nicht**, Detailbereich ist ein Rumpf, **SM-DES-007 unvollständig** (siehe unten) |
| **AP-13** | **Detailbereich und Metadatenpflege** | **7** | **offen** — der Detailbereich zeigt eine Überschrift und einen Leerzustand |
| **AP-14** | **Schnittmuster und Anleitungen** | **4** | **offen** — kein PDF-Weg im Baum |
| **AP-15** | **Druck** | **10** | **offen** — kein Druckweg im Baum |
| **AP-16** | **Konvertierung, Export, Wechseldatenträger** | **6** | **offen** — Schreibpfade für DST und PES liegen vor, Export und Dialog nicht |
| **AP-17** | **Stapelverarbeitung** | **5** | **offen** |
| **AP-18** | **Lokale Analyse und Schlüsselablage** | **11** | **offen** |
| **AP-19** | **Unversehrtheit der Quelldaten** | **1** | **offen** — SM-MIG-005 |
| **AP-20** | **Protokoll, Sprache, Fehlertexte** | **1** | **offen** — SM-SEC-010 |
| **AP-21** | **Auslieferung** | **11** | **offen** |
| AP-22 | Abnahmenachweise | 0 | **offen** — das Paket trägt keine eigene Anforderung; es wiederholt die messenden Nachweise am Auslieferungsstand (AK-01, AK-06, AK-07). Es steht hier der Vollständigkeit halber: Abschnitt 5 nennt es als von OP-08 und OP-18 betroffen |

Damit stehen **56 der 137 verplanten Anforderungen** ohne jede Umsetzung im Baum. Weitere **36**
sind gebaut, aber nicht nachweisbar, und zwar aus zwei verschiedenen Gründen: **26** (AP-11,
AP-12) hängen an einer Oberfläche, die nicht übersetzt; **10** (AP-07, AP-08) übersetzen zwar,
aber ihre Prüffälle laufen nicht, weil `kern-services` kein Verbundmitglied ist.

## 4. Betroffene Anforderungen

Der Reparaturschritt aus Abschnitt 7 berührt keine Fachanforderung neu, sondern stellt die
Nachweisbarkeit vorhandener wieder her:

- **SM-NFR-012** (automatisierte Prüfung) — ein Verbund, der zwei Kisten nicht führt, prüft sie
  nicht. Die Zusage gilt heute nur für sechs Achtel des Baums.
- **SM-SEC-004** (die Oberfläche kennt die Datenhaltung nicht) — die Schnittregel wird laut
  `Cargo.toml` „beim Bau erzwungen". Eine Kiste, die nicht gebaut wird, erzwingt nichts.
- **SM-LIB-001, SM-LIB-003, SM-LIB-004, SM-LIB-009** (AP-07) und **SM-IMP-001, SM-IMP-003,
  SM-IMP-005, SM-IMP-009, SM-NFR-002, SM-NFR-005** (AP-08) — ihre Prüffälle liegen in
  `crates/kern-services/src/tests.rs` und laufen nicht.
- **SM-DES-001 bis SM-DES-007, SM-NFR-003, SM-NFR-006 bis SM-NFR-009, SM-NFR-013, SM-PRV-007,
  SM-PRV-009, SM-SEC-008, SM-SET-001 bis SM-SET-004, SM-SET-006, SM-SRC-008, SM-SRC-009** sowie
  **SM-LIB-002, SM-IMP-002 und SM-DAT-003** (AP-11, AP-12; zusammen die 26 aus Abschnitt 3) —
  sie hängen an einer Oberfläche, die nicht startet.

## 5. Berührte offene Punkte

Einordnung nach der Wirkungstabelle in `CLAUDE.md` Abschnitt 8.

| Punkt | Betrifft | Einordnung |
|---|---|---|
| OP-07, OP-09, OP-15, OP-16, OP-17, OP-19 | AP-11 bis AP-18 | **Blocker für den Beginn** neuer Oberflächenpakete (IMP-STM-001 Abschnitt 2.4). Der Reparaturschritt entscheidet keinen davon: Er stellt her, was unter jeder Antwort gleich gebaut worden wäre |
| OP-20 | AP-18 | **Blocker** für den Beginn von AP-18 |
| OP-13 und die Wegwahl A/B | AP-03 bis AP-06, AP-09, AP-11 bis AP-15 | Der Baum ist Weg A gefolgt; `Analysis/20260825_01_anwendungsbau.md` weist die Entscheidung des Auftraggebers nach. Kapitel 14 ist **nicht fortgeschrieben** — nach `CLAUDE.md` Abschnitt 3 ist `Analysis/` Nachweis, keine Quelle, der Punkt gilt damit als offen |
| OP-03 | AP-16 | Kein Blocker: SM-EXP-001 fordert ein **wählbares** Zielformat, nicht welches |
| OP-08, OP-18 | AP-07, AP-10, AP-12, AP-22 | Kein Blocker für den Bau, aber für den Abschluss. Messwerte gelten bis zur Antwort als Regressionsschwelle |
| OP-14 | AP-14 | Kein Blocker: AP-14 härtet die Anzeige unabhängig von der Antwort |
| OP-04, OP-06, OP-11 | AP-21 | **Blocker für den Auslieferungsbau** |
| OP-21 | AP-09 | Kein Blocker; betrifft die Bestehbedingung von PF-PRV-02 |
| OP-01, OP-02, OP-05, OP-10, OP-12 | — | ohne Wirkung auf den vorgeschlagenen Schritt |

## 6. Root Cause

**Zu Befund 1.** Der Verbund wurde bewusst schmal begonnen: Ein `members`-Eintrag ohne
Verzeichnis macht jeden `cargo`-Aufruf unmöglich, deshalb sollte jede Kiste mit ihrem eigenen
Änderungssatz hinzukommen. Der Kommentar in `Cargo.toml` hält das fest. `kern-services` und `ui`
sind seither entstanden, der Nachzug der `members`-Liste unterblieb. Weil das Gate sie damit nie
sah, konnten spätere Änderungen an der Fassade sie brechen, ohne dass eine Prüfung anschlug —
namentlich die begründete Verschlankung von `Kachel` nach SM-DES-007, die vier Felder entfernte,
die `bruecke.rs` weiter liest. Das ist kein Fehler jener Änderung, sondern die Folge einer
Prüflücke: Ein Verbund, der eine Kiste nicht führt, kann ihren Bruch nicht melden.

**Zu Befund 2.** Die Gatterregel bindet den Beginn von AP-11 bis AP-18 an sieben Antworten, die
bis heute nicht vorliegen. AP-11 und AP-12 sind trotzdem entstanden. Ob das eine bewusste
Entscheidung des Auftraggebers war, ist im Baum nicht belegt — die vorhandenen Analysedokumente
sind Nachweise und dürfen die Frage nach `CLAUDE.md` Abschnitt 3 nicht entscheiden. Nach S1 wird
der Widerspruch deshalb gemeldet und nicht ausgelegt.

## 7. Vorgeschlagener Ansatz

Der Auftrag „alle offenen Anforderungen umsetzen" umfasst 56 Anforderungen in neun
Arbeitspaketen. Er ist **kein** Änderungssatz: Die Kappungsgrenze für den Prüfdiff liegt bei
400.000 Byte, und jedes Paket zieht nach `CLAUDE.md` Abschnitt 10 eine eigene Analyse, eine
eigene Umsetzung und eine eigene Vier-Augen-Runde. Der Vorschlag zerlegt ihn deshalb in eine
Reparatur und eine geordnete Folge von Fachpaketen.

### Stufe A — Verbund vervollständigen (sofort, ohne offenen Punkt)

Dieser Schritt ist unter **jeder** Antwort auf jeden offenen Punkt gültig: Er baut nichts Neues,
sondern stellt die Übersetzbarkeit und die Prüfbarkeit dessen her, was bereits im Baum liegt.

1. `Cargo.toml`: `crates/kern-services` und `crates/ui` in `members` aufnehmen. **Abweichend
   umgesetzt** (Abschnitt 9): Statt der namentlichen Aufnahme steht dort `members =
   ["crates/*"]`. Die Aufzählung hat zwei Fehlerrichtungen, und beide waren eingetreten; das
   Muster kennt keine davon.
2. `crates/kern-services/Cargo.toml`: `kern-security` als Entwicklungsabhängigkeit führen; die
   vier Aufrufe in `src/tests.rs` auf `Schreibziel` umstellen.
3. `crates/ui/src/bruecke.rs`: die vier Rollen `stichzahl`, `farbzahl`, `favorit` und
   `name_herkunft` entfernen — **aus zwei verschiedenen Gründen**, die nicht zu vermengen sind.
   `stichzahl`, `farbzahl` und `favorit` entfallen, weil SM-DES-007 den Kachelaufbau
   abschließend aufzählt und sie nicht dazugehören; sie sind Filter- und Sortiergrößen. Die
   **Herkunftsmarke ist Bestandteil dieser Aufzählung** („ausschließlich Bild, Format- und
   Herkunftsmarke, Name und Größe") und wird nur **zurückgestellt**: Bis AP-18 gibt es keinen
   maschinell erzeugten Wert, der zu kennzeichnen wäre, und eine Marke, die fest auf
   `Herkunft::Datei` steht, kennzeichnet nichts (SM-KIA-008, SM-DES-009). Folge für die
   Rückverfolgbarkeit siehe unten.
4. `qmllint` in `scripts/check-docs.sh` bzw. die Stufe 0c aufnehmen — die Anwendbarkeitstabelle
   in `CLAUDE.md` Abschnitt 13 führt das QML-Gate bereits, ein `*.qml` liegt im Baum, das Gate
   steht damit auf **FAIL**, solange es nicht läuft (S3). **Abweichend umgesetzt**
   (Abschnitt 9): als eigenständiges `scripts/check-qml.sh` samt Selbsttest, nicht innerhalb der
   Dokumentprüfungen — sein Gegenstand ist `*.qml`, nicht der Dokumentbestand, und die
   Anwendbarkeit muss er selbst beantworten können.
5. Nachweis: `cargo test --all` führt die Prüffälle von `kern-services` mit; `cargo clippy`
   erfasst `ui`; `scripts/check-hintergrund.sh` baut wieder.

**Änderungsklasse G.** Stufe A ändert `scripts/` und `CLAUDE.md`; Abschnitt 9 von `CLAUDE.md`
ordnet diese Pfade der Klasse G zu, nicht C. Praktisch bedeutet das keine Mehrarbeit — Stufe 0b
ist bei G ohnehin zwingend —, aber die Einstufung im Protokollkopf muss zum Diff passen.
Erwarteter Umfang: weit unter der Kappungsgrenze.

### Stufe B — Fachpakete, je ein eigener Änderungssatz

Reihenfolge nach Abhängigkeit und nach dem Anteil, den der vorhandene Kern schon trägt:

| Reihenfolge | Paket | Anf. | Vorbedingung |
|---|---|---|---|
| B1 | AP-19 · Unversehrtheit der Quelldaten (SM-MIG-005) | 1 | keine — reiner Kernnachweis |
| B2 | AP-20 · Protokoll, Sprache, Fehlertexte (SM-SEC-010) | 1 | keine — reiner Kernnachweis |
| B3 | AP-16 · Konvertierung und Export | 6 | OP-03 offen, aber nicht sperrend |
| B4 | AP-13 · Detailbereich und Metadatenpflege | 7 | **OP-15, OP-16 zu beantworten** |
| B5 | AP-17 · Stapelverarbeitung | 5 | **OP-15 zu beantworten** |
| B6 | AP-14 · Schnittmuster und Anleitungen | 4 | **OP-15 zu beantworten**; OP-14 offen, nicht sperrend |
| B7 | AP-15 · Druck | 10 | **OP-15 zu beantworten**; AK-06 verlangt eine Messung am körperlichen Ausdruck |
| B8 | AP-18 · Lokale Analyse und Schlüsselablage | 11 | **OP-15, OP-20 zu beantworten**; OP-05 offen, nicht sperrend |
| B9 | AP-21 · Auslieferung | 11 | **OP-04, OP-06, OP-11 zu beantworten** |

**Zwei Grenzen sind zu benennen, nicht zu übergehen.** AK-06 (100 mm ± 0,5 mm am Papier, A4 und
US Letter, drei Plattformen) und AK-01 (100.000 Einträge auf dem Referenzgerät nach OP-08) sind
Messungen an körperlichen Geräten. Sie lassen sich hier vorbereiten — Kalibrierquadrat,
Messaufbau, Protokollvorlage —, aber nicht durchführen. Ein Paket, das sie als bestanden
protokollierte, verstieße gegen Abschnitt 13.2 des Lastenhefts.

### Was vor Stufe B zu entscheiden ist

Die Fortschreibung von Kapitel 14 ist eine inhaltliche Änderung am führenden Dokument und
gehört nach `CLAUDE.md` Abschnitt 4 in einen eigenen Vorgang mit Version, Historieneintrag und
Kopfzeilennachzug in DES-STM-001 und TEC-STM-001. Sie ist Klasse D und geht B4 voraus.

## 8. Änderungsklasse

**Maßgeblich ist der Änderungssatz, nicht das Dokument.** Für sich genommen wäre diese Analyse
Klasse T: Sie vergibt keine Kennung und begründet keine Anforderung. Sie wird aber nach
Abschnitt 10 von `CLAUDE.md` (Phase 4) **zusammen mit der Änderung** committet, und der
Änderungssatz von Stufe A berührt `scripts/`, `CLAUDE.md` und die Wurzelkonfiguration. Nach
Abschnitt 9 ist das **Klasse G — Werkzeug und Regeln**; Klasse T ist dort bei einer Zeile in
`scripts/` oder `CLAUDE.md` ausdrücklich ausgeschlossen, und im Zweifel gilt die höhere Klasse.

| Änderungssatz | Klasse | Begründung |
|---|---|---|
| Stufe A, Satz 1a (dieser) | **G** | Wurzelkonfiguration: `package.json`, `package-lock.json`, `.gitignore` |
| Stufe A, Satz 1b | **G** | `scripts/`, `CLAUDE.md` |
| B1 bis B9 | **C** | Änderungen unter `crates/` |
| Fortschreibung von Kapitel 14 | **D** | inhaltliche Änderung am Lastenheft |

## 9. Abschluss — Stufe A

Freigegeben durch den Auftraggeber am 26.08.2026; umgesetzt im selben Vorgang.

### Was geändert wurde

**Die Spalte „Satz" nennt, mit welchem Änderungssatz die Zeile in den Baum kommt** (Aufteilung
in Abschnitt 10). Ohne sie läse sich die Tabelle, als sei alles davon bereits committet.

| Datei | Satz | Änderung |
|---|---|---|
| `package.json` | 1a | `engines: node >= 22` — die gebundene Linter-Fassung verlangt es; `npm ci` warnt nur, erzwingt es nicht. Der Prüfbereich steht nicht mehr im Skriptaufruf: Kommandozeilen-Globs überschreiben die `globs` der Konfigurationsdatei, beide Wege prüften sonst verschiedene Mengen |
| `package-lock.json` | 1a | Mit `npm install --package-lock-only` aus **dieser** `package.json` erzeugt; der Wurzeleintrag führte zuvor weder `version` noch `engines` (SM-PLT-009) |
| `.gitignore` | 1a | Fuzzing-Pfade ohne Kistennamen (`**/fuzz/target/`, `-corpus/`, `-coverage/`); `artifacts/` bleibt sichtbar samt Regel, wohin der Fund danach wandert. Saatkorpus unter `crates/<kiste>/fuzz-saat/<ziel>/`, im Lauf **hinter** dem erzeugten Korpus genannt |
| `Cargo.toml` | 3 | `members = ["crates/*"]` statt der gepflegten Aufzählung. Die Liste hatte zwei Fehlerrichtungen, und beide sind eingetreten: ein Eintrag ohne Verzeichnis bricht jeden cargo-Aufruf, eine Kiste ohne Eintrag wird von keinem Gate erfasst. Das Muster kennt beide nicht — es führt, was da ist, und alles, was da ist. Damit ist Befund 1 nicht nur behoben, sondern in seiner Ursache abgestellt |
| `crates/kern-services/Cargo.toml` | 4 | `kern-security` als Entwicklungsabhängigkeit — die Prüffälle legen ihren Bestand über ein `Schreibziel` an |
| `crates/kern-services/src/tests.rs` | 4 | Vier ausgeschriebene Doppelungen des vorhandenen Helfers `schreibe_muster` auf diesen zurückgeführt; ein Prüffall auf das Verhalten nach SM-IMP-009 nachgezogen (siehe unten) |
| `crates/ui/src/bruecke.rs` | 5 | Die vier Rollen `stichzahl`, `farbzahl`, `favorit` und `name_herkunft` entfernt; die damit aufruferlose Hilfe `tausender` ebenfalls, mit Vermerk auf AP-13 |
| `crates/ui/qml/*.qml` | 5 | `pragma ComponentBehavior: Bound`; alle Zugriffe qualifiziert; `gestaltung` in vier Bauteilen von `property var` auf `required property Gestaltung` gehoben — damit prüft `qmllint` die Bezeichner der Gestaltungsquelle mit |
| `scripts/check-qml.sh` (neu) | 1b | Das QML-Gate: `qmllint` und `qmlformat` (ohne `-n`), PASS / FAIL / ENTFÄLLT nach S3, fail-closed bei Werkzeugfehler |
| `scripts/lib/pruefumgebung.sh` (neu) | 1b | Die Zeitgrenze der Selbsttests und des Messwerkzeugs — eine Antwort statt dreier |
| `scripts/lib/dateien.sh` (neu) | 1b | Die Dateiliste des Arbeitsbaums — eine Antwort für alle drei Prüfskripte, statt dreier auseinanderlaufender. Ihr Rückgabewert 1 („kein Git-Arbeitsbaum") wird von jedem Aufrufer ausgewertet (S3) |
| `scripts/check-qml.test.sh` (neu) | 1b | Sein Selbsttest, 26 Fälle |
| `CLAUDE.md` | 1b | Abschnitt 11 fortgeschrieben (Zeile „QML (Weg A)" samt Begründung zu `-n`), Abschnitt 2 (Befehlsliste) und Abschnitt 13 (Umsetzung, Stellung nach `cargo clippy`) |
| `scripts/review-gate.sh` | 1b | Gate in Stufe 0c **nach** `cargo clippy`, Selbsttest in Stufe 0b; beide Skripte in der Gate-Signatur |

### Nachweis

| Prüfung | Ergebnis |
|---|---|
| `cargo test --all` | 173 bestanden, 0 fehlgeschlagen, 14 ausgesetzt (zuvor 156 — die 17 Prüffälle aus `kern-services` liefen nie) |
| `cargo clippy --all-targets --all-features -- -D warnings` | ohne Befund, jetzt einschließlich `kern-services` und `ui` |
| `cargo fmt --all -- --check` | ohne Befund |
| `bash scripts/check-qml.sh` | 8 Dateien, `qmllint` und `qmlformat` ohne Befund |
| `bash scripts/check-qml.test.sh` | 26 bestanden, 0 fehlgeschlagen (ohne Qt: 5 geprüft, 21 entfallen) |
| `bash scripts/review-gate.test.sh` | 155 bestanden, 0 fehlgeschlagen |
| `bash scripts/check-projektregeln.test.sh` | 61 bestanden, 0 fehlgeschlagen |
| `npm install --package-lock-only`, zweimal hintereinander | zweiter Lauf ohne Unterschied — die Sperrdatei ist stabil und aus **dieser** `package.json` erzeugt (SM-PLT-009). Die Peer-Markierung an `markdownlint-cli2` bleibt dabei erhalten; sie wird also nicht bei jedem `npm install` neu geschrieben |
| `GATE_DRY_RUN=1 bash scripts/review-gate.sh` | grün bis einschließlich Stufe 0c |
| `SM_PRUEFBESTAND_ANZAHL=200 bash scripts/check-hintergrund.sh` | **Kurzprüfung bestanden** — 120 von 120 Takten während des Laufs, zwei Läufe, zweiter inkrementell. Keine Abnahme: Die Bezugsmenge ist 100.000 (SM-LIB-009, SM-NFR-001, AK-01) |

Damit läuft die Anwendung wieder — der Messlauf baut sie, liest 200 Dateien ein, erzeugt
Vorschauen und hält den Oberflächenfaden frei.

### Ein Prüffall trug eine überholte Zusage

`defekte_dateien_halten_den_lauf_nicht_an` erwartete `neu == 10` und `abgewiesen == 5`. Das ist
das Verhalten **vor** SM-IMP-009. Seither wird eine nicht lesbare Datei **erfasst** statt
übersprungen: `Fassade::einlesen` gibt einen Fehlereintrag zurück, der Lauf zählt ihn als
aufgenommen. Der Prüffall stützt sich jetzt auf den Fehlergrund an der Kachel und weist damit
beide Zusagen nach — SM-NFR-005 (der Lauf läuft zu Ende) und SM-IMP-009 (er erfasst die Datei).

**Beobachtung ohne Blockerwirkung (S4):** `Importbefund.abgewiesen` zählt seit SM-IMP-009 keine
unlesbare Datei mehr. Die Abschlussmeldung des Imports nennt sie damit nicht. Ob die Nutzerin
nach einem Lauf über 100.000 Dateien erfährt, dass 300 davon beschädigt sind, ist eine echte
Endnutzerwirkung — und sie gehört deshalb **nicht nur hierher**: `Analysis/` ist nach Abschnitt 3
Nachweis, keine Quelle und kein Register. Ohne Eintrag in Kapitel 14 fiele die Frage bei AP-12
und AP-13 nicht wieder auf den Tisch.

**Zwei Vormerkungen für die Fortschreibung von Kapitel 14** (Klasse D, Abschnitt 8 dieses
Dokuments). Beide entstehen dort, nicht in diesem Satz: Ein neuer offener Punkt ist eine
inhaltliche Änderung am führenden Dokument (Abschnitt 4), und die betroffenen Kisten liegen
nicht in diesem Änderungssatz.

1. „Welche Bilanz nennt die Abschlussmeldung des Imports nach SM-IMP-009 — erfasste,
   abgewiesene und beschädigte Dateien getrennt?"
2. **SM-DES-007 ist unvollständig, und das ist bereits mit einer Kennung belegt.** Die
   Anforderung nennt die Herkunftsmarke ausdrücklich als Bestandteil des Kachelaufbaus; sie ist
   bis AP-18 zurückgestellt, weil es keinen zu kennzeichnenden Wert gibt. **PF-DES-07 ist damit
   heute nicht bestehbar.** Das gehört anders als die Importbilanz **zusätzlich in die
   Rückverfolgbarkeitsmatrix** zu AP-12/PF-DES-07 (Lastenheft Abschnitt 13.3): Ohne diesen
   Eintrag gälte AP-12 als gebaut, während AP-18 die Anforderung nicht führt — eine
   Muss-Anforderung fiele zwischen zwei Paketen durch.

### Aufgelöster Widerspruch gegen `CLAUDE.md` (S1) — `qmlformat -n`

Abschnitt 11 nennt als QML-Gate `qmllint` **und** `qmlformat -n`. Der Schalter `-n`
(*normalize*) sortiert die Eigenschaften eines Objekts alphabetisch. Auf
`crates/ui/qml/Gestaltung.qml` angewandt zerreißt er die Gliederung nach den Abschnitten von
DES-STM-001 — Farben nach 3.1/3.2, Grundraster nach 5 — und löst die Begründungskommentare von
den Werten, zu denen sie gehören. Die Gestaltungsquelle verlöre damit genau die Nachvollziehbarkeit
gegen das Gestaltungsdokument, um derentwillen SM-DES-003 sie zu **einer** Datei macht.

Ohne `-n` ändert `qmlformat` nur Leerraum und Strichpunkte und ließe die Gliederung unberührt;
der Unterschied beträgt 351 Zeilen über acht Dateien statt 1.058.

Nach S1 wurde der Widerspruch **gemeldet, nicht ausgelegt**. Der Auftraggeber hat am
26.08.2026 zugunsten der Gliederung entschieden: **`qmlformat` ohne `-n`**, und Abschnitt 11
von `CLAUDE.md` ist entsprechend fortgeschrieben — Zeile „QML (Weg A)", ein Absatz zur
Begründung, dazu die Befehlsliste in Abschnitt 2 und die Umsetzungsbeschreibung in
Abschnitt 13.

Umgesetzt: `qmlformat` ist in `scripts/check-qml.sh` verdrahtet, die acht QML-Dateien sind
formatiert (351 geänderte Zeilen; die Gliederung nach den DES-Abschnitten und die Bindung der
Kommentare an ihre Werte bleiben erhalten). Der Selbsttest hält **beide** Hälften fest:
Eine abweichende Einrückung blockiert, und die Reihenfolge der Eigenschaften ist kein Befund —
liefe die Prüfung mit `-n`, schlüge der zweite Fall fehl.

### Was Stufe A **nicht** getan hat

- **Kapitel 14 des Lastenhefts ist unverändert.** Alle 21 offenen Punkte stehen weiter offen;
  die Gatterregel sperrt den Beginn von AP-13 bis AP-18 unverändert (Abschnitt 5).
- **Kein Erstnachweis ist in die Rückverfolgbarkeitsmatrix eingetragen** — siehe unten.

### Vier-Augen-Konsens — acht Runden, alle Befunde behoben

Die Protokolle sind rot und deshalb nicht committet. Übernahme nach Abschnitt 15 in
Kurzform: Ort, Kennung, Schweregrad und Status je blocker- und major-Befund, minor je Runde
gezählt und zusammengefasst — ohne den beanstandeten Wert, ohne Agentennamen bei blocker und
major (die Zuordnung gäbe dort das Votum preis) und ohne Votumsbilanz.

| Runde | Ort | Kennung | Schwere | Befund in einem Satz | Status |
|---|---|---|---|---|---|
| 1 | `check-qml.test.sh`, Werkzeugabwesenheit | S1, S3 | **blocker** | Der Fall stellte die fehlende Qt-Werkzeugkette durch Verengen des Suchpfads nach und zählte den nicht darstellbaren Fall als Fehlschlag — auf jeder Ablage, die `qmllint` ins Grundsystem legt, wäre Stufe 0b dauerhaft rot | behoben — die Abwesenheit wird jetzt hergestellt statt vorausgesetzt |
| 1 | `review-gate.sh`, Stufe 0c | Abschnitt 13 | major | Das QML-Gate hing an `Cargo.toml` statt an `*.qml` | behoben |
| 1 | `review-gate.test.sh`, `make_repo` | Lastenheft 13.2 | major | Die neue Verdrahtung war vollständig ungeprüft | behoben — die Verdrahtung ist jetzt im Gate-Selbsttest belegt |
| 1 | `check-qml.sh`, Formatteil | S1, S3 | major | Der Zweig „qmlformat fehlt" hatte weder Prüffall noch Vorbedingung | behoben — beide Werkzeuge werden vorab geprüft, mit Negativfall |
| 1 | `check-qml.sh`, Modulsuche | SM-NFR-006 | major | Fehlendes Modul ergab Befunde ohne Hinweis auf die Ursache | behoben |
| 1 | `CLAUDE.md` Abschnitt 2 | Lastenheft 13.2 | major | Kennwerte durch denselben Änderungssatz überholt | behoben (neu gemessen) |
| 2 | `check-qml.sh`, qmllint-Aufruf | Abschnitt 11 | major | Zwei Maßstäbe an zwei gleichartige Ausnahmen: für die eine der enge Musterweg, für die andere eine baumweit abgeschaltete Prüfart | behoben — Musterfilter aus abgeleiteten Typnamen statt abgeschalteter Prüfart |
| 2 | `check-qml.test.sh`, Vorspann | S1, S3 | major | Der Selbsttest **behauptete** seine Anwendbarkeit, statt sie zu prüfen | behoben |
| 2 | dieses Dokument, Kopf | Abschnitt 9 | major | Änderungsklasse T statt G | behoben |
| 3 | `check-qml.test.sh`, Vorspann | S1 | major | Nur die halbe Sackgasse war beseitigt: Mit `*.qml` im Baum sperrte der Selbsttest auf jedem Gerät ohne Qt jeden Commit — Stufe 0b ist nicht änderungsbezogen | behoben — Anwendbarkeit je Fallgruppe statt global |
| 3 | `check-qml.test.sh` | Abschnitt 11 | major | Vierte, wortkopierte Fassung des Dateilisten-Idioms neben der Bibliothek, die es abschaffen sollte | behoben — die Wortkopie ist ersatzlos entfallen |
| 4 | `check-qml.sh`, Modulsuche | SM-NFR-012 | major | Ein gebauter Baum trägt das Modul mehrfach; alle gingen als Importpfad an qmllint, und der Bau-Hash entschied, welche Typbeschreibung gilt — ein älterer Stand ließe eine entfernte Rolle als vorhanden erscheinen | behoben — je Modulname gilt das jüngste Verzeichnis |
| 4 | `review-gate.test.sh` | Lastenheft 13.2 | major | Der **rote** Zweig des Gates war verhaltensmäßig ungeprüft | behoben — das Verhalten des roten Zweigs ist jetzt belegt |
| 4 | `lib/dateien.sh` | Abschnitt 11, 15 | major | Die Funktion, mit der Runde 3 einen Befund als behoben auswies, hatte keinen Aufrufer | behoben — Funktion und Begründung gestrichen |
| 5 | `CLAUDE.md` Abschnitt 2 | S1 | major | Zwei Zählwerte für denselben Stand | behoben |
| 5 | `check-qml.sh`, Kopfkommentar | Abschnitt 11 | major | Der Kommentar beschrieb das Gegenteil des Codes und begründete es mit einem entfernten Schalter | behoben |
| 5 | `check-qml.test.sh`, Gruppe E | Lastenheft 13.2 | major | Die Modulauswahl aus Runde 4 hatte keinen Prüffall | behoben — die Auswahl ist jetzt belegt, samt Gegenprobe für verschiedene Module |
| 5 | `CLAUDE.md` Abschnitt 15 | S1 | major | Die Regel aus Runde 4 erreichte ihr Ziel nur halb: „major (X)" ist dieselbe Angabe wie „X hat blockiert" | behoben |
| 6 | `CLAUDE.md` Abschnitt 2 | S1 | major | Der Erklärabsatz zu den Laufzeiten war beim Nachmessen unbemerkt invertiert worden | behoben — der Absatz nennt jetzt den Mechanismus, keine Richtung |
| 6 | `check-qml.test.sh`, Schattenpfad | Abschnitt 11 | major | Zwei Kommentarblöcke, der obere überholt | in Runde 6 nur scheinbar behoben, siehe Runde 7 |
| 7 | `check-qml.test.sh`, Schattenpfad | Abschnitt 15 | major | Der in Runde 6 als „ersatzlos gestrichen" gemeldete Block lag unverändert im Baum — der neue war daneben eingefügt worden | behoben |
| 7 | `check-qml.sh`, Filterlauf | Abschnitt 13 | major | Der Filter war der einzige Schritt ohne fail-closed-Absicherung | behoben |
| 7 | `check-docs.sh`, `check-projektregeln.sh` | S3 | major | Der Vertrag „jeder git-Fehler ist FAIL" wurde an vier von fünf Aufrufstellen verworfen | behoben — eigener Rückgabewert für „Liste nicht bildbar", mit Negativfällen |
| 7 | `check-qml.sh`, Kommentare | Abschnitt 11 | major | Zwei auseinandergelaufene Blöcke über derselben Ausnahme | behoben |
| 7 | `lib/pruefumgebung.sh` | Abschnitt 11 | major | `check-hintergrund.sh` trug die Zeitgrenze weiterhin wortgleich | behoben |
| 8 | `check-qml.test.sh`, Anwendbarkeit eines Einzelfalls | S1, S3 | major | An `qmllint` gebunden, obwohl der Prüfling beide Werkzeuge vorab prüft — auf einem Gerät ohne `qmlformat` dauerhaft rot | behoben |
| 8 | `check-qml.sh`, Modulauswahl | Abschnitt 11 | major | Doppelte Initialisierung, der obere Kommentarblock beschrieb einen verworfenen Entwurf | behoben |
| 8 | dieses Dokument | Abschnitt 15 | major | Die Fallnamen standen weiterhin in drei Dateien, obwohl der Abschluss von Runde 7 das Gegenteil festhielt | behoben — in Skript und Regeldatei stehen jetzt sachliche Verweise |

**Minor-Befunde je Runde:** 13 · 11 · 9 · 16 · 14 · 16 · 12 · 9 — alle übernommen; Dubletten
über die vier Rollen hinweg sind dabei zusammengeführt (Abschnitt 13, Stufe 1). Sie betrafen
durchweg dieselben vier Klassen: Doppelantworten im Baum (daraus entstanden
`scripts/lib/dateien.sh` und `scripts/lib/pruefumgebung.sh`), Meldungen ohne begehbaren Weg,
Prozesskosten in Pfaden, die je Commit laufen, und Zusagen ohne Prüffall.

**Drei Muster haben sich über die Runden wiederholt** und sind der eigentliche Ertrag dieser
Kette: Ein Kommentar, der nach einer Korrektur stehen bleibt, beschreibt bald das Gegenteil des
Codes und führt beim nächsten Umbau zurück in den behobenen Fehler. Eine Zusage ohne Prüffall
degradiert unbemerkt zum No-Op. Und eine Anwendbarkeitsfrage, die **behauptet** statt geprüft
wird, sperrt Stufe 0b auf fremden Geräten dauerhaft — Stufe 0b ist nicht änderungsbezogen, ein
Fehler dort trifft jeden Commit.

**Ein minor bleibt bewusst offen und ist vorgemerkt:** Dass die Abschlussmeldung des Imports
beschädigte Dateien seit SM-IMP-009 nicht mehr nennt, gehört als offener Punkt in Kapitel 14 des
Lastenhefts — `Analysis/` ist Nachweis, kein Register. Der Eintrag entsteht im dafür ohnehin
vorgesehenen Klasse-D-Vorgang; die betroffene Kiste liegt nicht in diesem Änderungssatz (S4).

Umfang nach Runde 8: Gate-Selbsttest 155 Fälle, QML-Selbsttest 26 (ohne Qt 5 geprüft,
21 entfallen), Projektregeln 61, Planprüfungen 26 — alle grün.

## 10. Aufnahme des Quellbaums in die Versionsgeschichte

Der gesamte Quellbaum war bis zum 26.08.2026 unverfolgt. Ein Commit über den Gesamtstand ergibt
einen Reviewer-Diff von rund 1,43 MB gegen eine Kappungsgrenze von 400.000 Byte. Der Regelweg
ist die **Aufteilung des Änderungssatzes** (`CLAUDE.md` Abschnitt 13, Stufe 0.2, Ziffer 1); der
Auftraggeber hat sie am 26.08.2026 gewählt, mit vollständigem Gate einschließlich Stufe 1 je
Satz.

**Eine Einschränkung des Zwischenstands, ausdrücklich benannt:** Nach Satz 1a liegt die
npm-Abhängigkeitskette im Baum, die sie prüfenden Gates aber noch nicht — `scripts/` kommt mit
1b. Die *schützenden* Vorkehrungen sind gleichwohl vorhanden, weil sie bereits verfolgt sind:
`.npmrc` mit `ignore-scripts=true` (SM-OSS-011), `.lizenzen.conf` als Positivliste und die
`integrity`-Bindung jedes Eintrags. Es fehlt im Zwischenstand die *Prüfung*, nicht die
*Maßnahme*; mit 1b ist auch sie da.

**Satz 1 ist nach acht Runden noch einmal geteilt worden.** Die Befunde wurden von Runde zu
Runde kleiner, aber ein wesentlicher Teil entstand daran, dass eine Korrektur Kommentare und
Nachweistexte hinterließ, die den Code nicht mehr trafen — bei 222 KB Änderungssatz wächst diese
Fläche schneller, als sie sich prüfen lässt. Die Werkzeugbindung steht sachlich für sich und
geht deshalb voran. Eine weitere Trennung von 1b gibt es nicht: Das QML-Gate, die beiden
gemeinsamen Bibliotheken und die Regeln in `CLAUDE.md` bedingen einander — die Bibliotheken sind
aus den Befunden am Gate entstanden, und Code ohne die zugehörige Regel wäre sofort eine
Divergenz.

**`Reviews/` bleibt außen vor.** Alle 15 vorhandenen Protokolle tragen das Ergebnis
**BLOCKIERT**; Abschnitt 15 hält fest: „Rote Protokolle werden nicht committet." Ihre Befunde
sind nach derselben Regel in die zugehörigen `Analysis/`-Dokumente übernommen. Damit fallen
584 KB weg, und die Aufteilung geht in fünf Sätzen auf. Die Herkunftsausnahme des Gates greift
für sie ohnehin nicht: Die Liste `emitted-reports` im Gate-Zwischenspeicher ist leer, die
Herkunft also nicht belegt — geprüft wird die Herkunft, nicht der Name.

**Die Wurzelkonfiguration liegt bereits im Baum.** `.npmrc` (mit `ignore-scripts=true`,
SM-OSS-011), `.lizenzen.conf`, `.markdownlint-cli2.jsonc` und `.projektregeln.conf` sind
verfolgt — belegt über `git ls-files`. Die npm-Abhängigkeitskette kommt damit in Satz 1 **nicht**
ohne ihre Schutzvorkehrung in den Baum; der Reihenfolgefehler, den ein unverfolgtes `.npmrc` an
dieser Stelle bedeutet hätte, tritt nicht ein. Unverfolgt ist von den Wurzeldateien allein
`.messwerkzeug.conf`, und die gehört mit dem Messwerkzeug in Satz 5.

| Satz | Inhalt | Diff |
|---|---|---|
| 1a | Werkzeugbindung und dieses Analysedokument — `package.json`, `package-lock.json`, `.gitignore`. Zwei Festlegungen, die für sich stehen: node-Mindestfassung und Fuzzing-Pfade ohne Kistennamen. Der Nachweis kommt hier mit, weil Phase 4 ihn an seine Änderung bindet und er den ganzen Vorgang trägt | ~83 KB |
| 1b | Gate, Bibliotheken und Regeln — **das gesamte** `scripts/` samt `scripts/lib/` und `CLAUDE.md`. `scripts/check-hintergrund.sh` gehört dazu, weil es dieselbe Umgebungsbibliothek einbindet wie die Selbsttests; seine Zuordnungsdatei `.messwerkzeug.conf` kommt mit der Oberfläche in Satz 5, und bis dahin meldet es ENTFÄLLT | ~120 KB |
| 2 | Planung und Nachweise — `Implementation/`, die übrigen `Analysis/`-Dokumente, `Design/stitchmanager-logo.svg` | ~260 KB |
| 3 | Verbund und unterer Kern — `Cargo.toml`, `Cargo.lock`, `deny.toml`, `kern-typen`, `kern-security`, `kern-parsers`, `kern-render` | ~246 KB |
| 4 | Datenhaltung, Fassade, Kernbetrieb — `kern-db`, `kern-fassade`, `kern-services` | ~168 KB |
| 5 | Oberfläche und Messwerkzeugzuordnung — `crates/ui`, `.messwerkzeug.conf` | ~82 KB |

**Jeder Zwischenstand ist übersetzbar und grün.** Das ist keine Behauptung: Satz 3 mit
102 Prüffällen und Satz 4 mit 173 wurden vor der Aufteilung in einem getrennten Baum gebaut
und gefahren. Möglich wird das durch `members = ["crates/*"]`: Eine Aufzählung
hätte in Satz 3 auf Verzeichnisse gezeigt, die es dort noch nicht gibt, und jeden cargo-Aufruf
unmöglich gemacht.

**Kein Erstnachweis ist in die Rückverfolgbarkeitsmatrix eingetragen.** Die Läufe oben sind
Regressionsschwellen; ohne das Referenzgerät nach OP-08 tragen sie keine Abnahme.
