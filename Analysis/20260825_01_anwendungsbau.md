# Analyse — Beginn des Anwendungsbaus (Weg A, Kernwiederverwendung)

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260825-01 |
| Datum | 2026-08-25 |
| Auslöser | Auftrag des Nutzers: „erstelle die app entsprechend den Anforderungen unter ./Design, ./Requirements und ./TechStack" |
| Änderungsklasse | **C** — Quellcode. Es entsteht der erste Quellbaum des Vorhabens. |
| Mitgeltend | URS-STM-001 v1.3 · DES-STM-001 v1.3 · TEC-STM-001 v2.2 · IMP-STM-001 |

## 1. Problembeschreibung

Das Repository ist bis heute ein reines Spezifikations-Repository. Gefordert ist die Umsetzung
der spezifizierten Anwendung. Der Implementierungsplan IMP-STM-001 sperrt in Abschnitt 2.4 jedes
Arbeitspaket, dessen Vorbedingung ein ungeklärter Punkt aus Abschnitt 2.1 ist. Vor dem Bau war
daher das Entscheidungsgatter zu räumen, nicht zu umgehen.

## 2. Herbeigeführte Entscheidungen

Die folgenden Punkte hat der Auftraggeber am 2026-08-25 ausdrücklich entschieden. Sie sind damit
nicht mehr „im Code entschieden", sondern beantwortet — die Voraussetzung nach `CLAUDE.md`
Abschnitt 8 und IMP-STM-001 Abschnitt 2.4 ist erfüllt.

| Punkt | Frage | Entscheidung |
|---|---|---|
| **Weg A / Weg B** | cxx-qt gegen PySide6 mit PyO3 | **Weg A — cxx-qt.** Deckt sich mit der Empfehlung in TEC-STM-001 Abschnitt 2. |
| **OP-13** | Wiederverwendung des vorhandenen Rust-Kerns oder Neuentwicklung | **Wiederverwendung.** Quelle ist der Kern aus `StitchManager-3` (31.529 Zeilen Rust, GPL-3.0). |
| Rückstand | Umgang mit dem bei 0/4 blockierten Dokument-Änderungssatz | Quellcode zuerst; die offenen Befunde werden mit dem Quellcode in einem gemeinsamen Gate-Lauf abgearbeitet. |

**Nicht entschieden und weiterhin gesperrt:** OP-01, OP-07, OP-09, OP-15 bis OP-17, OP-19, OP-20.
Sie sperren nach Abschnitt 2.1 die Oberflächenpakete AP-11, AP-12 und AP-18. Der vorliegende
Änderungssatz nimmt keinen davon vorweg (siehe Abschnitt 5).

## 3. Betroffene Komponenten

| Komponente | Art der Berührung |
|---|---|
| `Cargo.toml` (Wurzel), `crates/` | **neu** — Werkstattverbund nach dem Modulschnitt aus IMP-STM-001 Abschnitt 3 |
| `crates/kern-security` | **neu** — Pfadprüfung, Kanonisierung, Eingrenzung |
| `crates/kern-parsers` | **übernommen** aus `StitchManager-3/src-tauri/src/parsers/` — PES, DST, JEF, VP3 **und der Schreibpfad** |
| `crates/kern-db` | **übernommen und umgebaut** — SQLite/WAL, additive Migrationen, FTS5 |
| `crates/kern-render` | **übernommen** — Vorschauerzeugung aus Stichdaten |
| `crates/kern-services` | **übernommen** — Import, Datenträger, Sicherung, Stapel, Analyse |
| `crates/kern-fassade` | **neu** — einzige Zugriffsschicht der Oberfläche (SM-SEC-004) |
| `crates/ui` samt QML | **neu** — Oberflächenschicht unter Qt 6 |
| `.gitignore` | Ergänzung um `target/` |

## 4. Betroffene Anforderungen

Der Änderungssatz legt das Gerüst und die Kernschicht an. Unmittelbar berührt sind die
Arbeitspakete **AP-01** (Lizenz- und Herkunftsgrundlage), **AP-02** (Projektgerüst und
Prüfkette), **AP-03** (Pfadsicherheit), **AP-04** (Datenhaltung), **AP-05** (Kernfassade) und
**AP-06** (Formatparser) und damit die Kennungen SM-OSS-001 bis 013, SM-NFR-012, SM-SEC-001 bis
005, SM-DAT-006 bis 008, SM-FMT-001 bis 013 und SM-SRC-001 bis 010.

Die fünf Schnittregeln aus IMP-STM-001 Abschnitt 3 sind bauartlich umzusetzen, insbesondere
Regel 1 (die Oberfläche greift nie unmittelbar auf `kern/db` zu) und Regel 2 (kein Farb-,
Schrift- oder Abstandswert außerhalb von `ui/gestaltung`). Beide sind nachträglich praktisch
nicht mehr einziehbar und entstehen deshalb mit dem ersten Modul.

## 5. Berührte offene Punkte

Einordnung nach der Wirkungstabelle in `CLAUDE.md` Abschnitt 8:

| Punkt | Einordnung | Begründung |
|---|---|---|
| Weg A / Weg B, OP-13 | **beantwortet** | Entscheidung des Auftraggebers, Abschnitt 2 |
| OP-08 | grundlagenschaffend | Der Prüfbestand entsteht; Messungen laufen bis zur Antwort als Regressionsschwelle, nicht als Abnahme |
| OP-03, OP-05, OP-21 | neutral | Die Schnittstellen bleiben so geschnitten, dass beide Antworten möglich bleiben |
| OP-07, OP-09, OP-15 bis OP-17, OP-19, OP-20 | **weiterhin sperrend** | Betreffen AP-11, AP-12, AP-18. Diese Pakete werden in diesem Satz **nicht** begonnen |

## 6. Begründung der Wiederverwendung

Der Kern aus `StitchManager-3` steht unter **GPL-3.0** und ist damit mit der Projektlizenz
identisch; die Übernahme wirft keine Lizenzfrage auf (SM-OSS-004, SM-OSS-005). Die Prüfung der
Kopplung ergab: `parsers/` (PES, DST, JEF, VP3, Schreibpfad, rund 3.500 Zeilen) enthält **keinen**
Verweis auf die abgelöste Rahmenbibliothek und ist unverändert übernehmbar. Kopplung besteht
allein in `db/models.rs`, `services/file_watcher.rs` und `services/usb_monitor.rs`; sie wird beim
Umzug entfernt. Die Oberflächenschicht (`commands/`, `lib.rs`, `main.rs`) entfällt vollständig —
sie trägt genau die Rahmenbibliothek, die RB-01 ausschließt.

## 6a. Aufteilung des Änderungssatzes

Der vollständige Quellbaum überschreitet die Kappungsgrenze der Stufe 0.2
(`DIFF_CAP_BYTES`, 400.000 Byte). `CLAUDE.md` Abschnitt 13 nennt als Regelweg
das **Aufteilen**, nicht das Kürzen. Der Baum kommt daher in drei Commits, die
je für sich prüfbar sind:

| Commit | Inhalt | Umfang |
|---|---|---|
| **Q1** | Werkstattverbund, **vollständige Kernschicht** (einschließlich `kern-parsers`), Lizenzprüfung, Zuordnungsdatei, diese Analyse | ~330 KB |
| **Q2** | Dienste und Oberfläche samt zugehöriger Analysen | ~156 KB |

**Jeder Commit steht für sich.** Der Verbund führt nur Mitglieder, deren
Verzeichnis auch vorliegt: Ein Member ohne Verzeichnis macht **jeden**
cargo-Aufruf unmöglich — `metadata`, `fmt`, `clippy`, `test` und `deny`
gleichermaßen —, und der committete Stand wäre nicht baubar. `kern-fassade`
hängt an `kern-parsers`; beide gehören deshalb in denselben Commit.

**Was Q1 bewusst nicht enthält:** die Erweiterungen am Freigabe-Gate
(Projektregelprüfung, Lizenz-Positivliste, ENTFÄLLT-Auswertung) und die
Nachführung von `CLAUDE.md` und `README.md` auf den neuen Baumzustand. Beides
sind Änderungen der Klasse G am Regelwerk und an den Prüfmitteln; sie gehören in
einen eigenen Vorgang mit eigener Analyse. Sie zusammen mit dem Quellbaum zu
führen hat sich in fünf Stufe-1-Runden als Fehler erwiesen: Jede neue
Prüfbedingung zieht Analyse, Selbsttest, Regelwerkseintrag und Befehlsliste nach
sich und vergrößert den Änderungssatz, statt ihn prüfbar zu halten.

## 7. Vorgeschlagener Ansatz

1. **Werkstattverbund anlegen** — ein Cargo-Workspace mit je einer Kiste je Modul des
   Modulschnitts. Der Verbund erzwingt Schnittregel 1 über die Abhängigkeitsrichtung: `ui`
   kennt `kern-fassade`, aber nicht `kern-db`.
2. **AP-01 — Lizenzgrundlage:** `deny.toml` mit Positivliste, die den Bau abbricht.
3. **AP-03 — `kern-security`** zuerst, weil Schnittregel 5 jeden Schreibvorgang durch dieses
   Modul führt.
4. **AP-04 — `kern-db`** mit WAL, additiven Migrationen und FTS5.
5. **AP-06 — `kern-parsers`** übernehmen, Härtung gegen manipulierte Dateien belegen.
6. **AP-05 — `kern-fassade`** mit ausschnittweiser Lieferung nach Schnittregel 3 und 4.
7. **`ui/gestaltung`** als einzige Quelle der `--kn-*`-Bezeichner, danach das dreispaltige
   Hauptfenster.

## 7a. Warum es keine Kiste `kern-writers` gibt

Der Modulschnitt in IMP-STM-001 Kapitel 3 führt `kern/writers` als eigenes
Modul. Umgesetzt ist es als Modul **innerhalb** von `kern-parsers`
(`crates/kern-parsers/src/writers.rs`), nicht als eigene Kiste. Das ist zulässig
und beabsichtigt: Kapitel 3 hält ausdrücklich fest, die Modulnamen seien
„**logisch, nicht als Pfade zu lesen**".

Der sachliche Grund: Schreib- und Lesepfad teilen sich den Stichabschnittstyp
und die Formatkenntnis. Eine eigene Kiste müsste beides von `kern-parsers`
beziehen und brächte eine Abhängigkeitsrichtung ohne Gegenwert. Eine **leere**
Kiste mitzuliefern, nur damit der Verbund die Modulliste nachbildet, wäre totes
Gewicht — und ein Prüfgegenstand, der nichts prüft.

## 7b. Benannte Lücken dieses Änderungssatzes

**SM-FMT-013 — Formaterkennung aus dem Inhalt — ist unerfüllt.** Das Format
wird ausschließlich aus der Dateiendung bestimmt (`Format::aus_endung`), und
diese Entscheidung legt fest, welcher Parser die Fremdbytes zu sehen bekommt.
Heute fängt die Magic-Prüfung jedes einzelnen Parsers eine falsch benannte
Datei als Lesefehler ab — die Absicherung hängt damit an der Sorgfalt der
Parser, nicht an der Zuordnung. Für die im Lastenheft geführten Formate ohne
verlässliche Signatur (EXP, XXX) fällt diese Rückversicherung weg. Zugleich
wird eine inhaltlich korrekte, nur falsch benannte Datei abgewiesen statt
erkannt.

Die Erkennung gehört nach `kern-parsers` und kommt daher mit **Q2**
(`format_aus_inhalt`, Endung nur noch als Rückfall, Abweichung protokolliert
ohne vollständigen Pfad nach SM-SEC-010, dazu ein Prüffall „PES-Inhalt unter
der Endung `.dst`" und ein Fuzzing-Fall nach SM-SEC-011). Bis dahin ist
SM-FMT-013 **offen und hier benannt**, nicht stillschweigend übergangen.

**SM-NFR-004** (fünf Sekunden bis bedienbereit bei 100.000 Einträgen) ist
ebenfalls ohne Nachweis; der Messaufbau dafür fehlt.

## 8. Abgrenzung

Nicht Gegenstand dieses Satzes: der gewerbliche Bereich (OP-02), die Listenansicht (OP-10) und
alles, was IMP-STM-001 Kapitel 10 zurückstellt.

---

## 9. Abschluss (Phase 4) — Stand Q1

> **Dieser Abschnitt beschreibt ausschließlich den Umfang von Q1** (Abschnitt 6a):
> Werkstattverbund, Kernschicht und Lizenzprüfung. Dienste und Oberfläche
> (`kern-services`, `ui`) kommen mit Q2; ihre Nachweise stehen dort und **nicht**
> hier. Eine frühere Fassung dieses Abschnitts nannte Zahlen und Belege aus dem
> Gesamtbaum — das beschrieb einen Zustand, den der Commit nicht liefert.

### 9.1 Was entstanden ist

| Kiste | Inhalt | Herkunft | Prüffälle |
|---|---|---|---|
| `kern-typen` | Fehlertyp mit Nutzersätzen, Format, Garnfarbe, Kennwerte, Herkunft | neu | 7 |
| `kern-security` | Wurzelprüfung, Eingrenzung, **Schreibziel**, Bereinigung von Fremdnamen | neu | 20 |
| `kern-parsers` | PES, DST, JEF, VP3 lesend; DST/PES schreibend; vier Fuzzing-Ziele | übernommen (OP-13) | 56, davon 12 bestandsabhängig ausgesetzt |
| `kern-render` | Vorschau aus Stichdaten, dauerhafter Zwischenspeicher | neu | 20 |
| `kern-db` | SQLite/WAL, sechs Schemaschritte, FTS5, Ausschnittabruf | neu | 24 + 2 Messfälle |
| `kern-fassade` | einzige Zugriffsschicht, gepufferter Vorschauweg | neu | 29 |

`cargo fmt` sauber, `cargo clippy --all-targets` ohne Meldung,
`cargo deny check licenses bans sources` grün über rund 90 Kisten.

### 9.2 Gemessen, nicht geschätzt

Messfall `crates/kern-db/tests/messung.rs`, 100.000 Einträge, Apple Silicon,
Freigabebau. **Keine Abnahme** — ohne Referenzgerät (OP-08) ist der Wert eine
Regressionsschwelle (IMP-STM-001 Abschnitt 2.2).

| Messgröße | Wert |
|---|---|
| Schlechtester Einzelwert über alle Fälle | **308 ms** |
| Volltext über den Gesamtbestand (100.000 Treffer), Versatz 99.940 | 245–308 ms |
| Volltext mit Teilmenge (890–25.000 Treffer), letzte gefüllte Seite | 4–71 ms |
| Ohne Volltext, tiefster Versatz, je Sortierschlüssel | 2,8–3,8 ms |
| Datenbankabfragen je Ausschnittabruf | konstant 2 bei 10 bis 500 Zeilen |

Drei Befunde entstanden aus dieser Messung. Ein Volltext mit Filtern lag bei
698 ms, weil SQLite den Namensindex als Zugriffsweg wählte und den Volltextindex
je Zeile prüfte — die Treffermenge wird jetzt zuerst festgehalten (32 ms). Die
Sortierklauseln fanden keinen passenden Index, weil sie absteigend nach dem Wert
und aufsteigend nach der Kennung sortieren; mit richtungsgleichen Indizes fiel
`Importdatum` von 175 ms auf 1,8 ms. Und der Messaufbau selbst maß am teuersten
Fall vorbei: Er prüfte Volltext nur bei Versatz 0 und 5.000, den tiefsten Versatz
nur **ohne** Volltext, und kein Wort des Prüfbestands traf mehr als ein Fünfzehntel
der Einträge.

**Der teuerste Fall, jetzt gemessen.** Jeder Name des Prüfbestands enthält nun das
Wort „Muster"; der tiefste Versatz wird je Fall aus dessen Trefferzahl bestimmt
statt fest auf 5.000 gesetzt. Damit liegt der ungünstigste Fall offen: Ein Wort,
das den Gesamtbestand trifft, zwingt den Tafelausdruck, je Seite 100.000 Zeilen
festzuhalten und zu sortieren — 308 ms bei Versatz 99.940. **SM-SRC-007 hält mit
Faktor drei Abstand.** Der Wert ist der Preis dafür, dass die Treffermenge zuerst
festgehalten wird; ohne das kostete der schmale Volltextfall 698 ms. Der Tausch
ist bewusst: Die schmalen Fälle sind der Alltag, der bestandsweite Begriff ist
der Rand, und beide bleiben unter der Sekunde.

Für SM-PRV-007 (Flüssigkeit beim Blättern) ist der Wert **kein Nachweis, aber
auch kein Widerspruch**: 308 ms je Seite wären beim Ziehen des Rollbalkens
spürbar. Die Oberfläche fordert Ausschnitte jedoch nebenläufig an (SM-NFR-002,
`kern-services`), die Kachelhöhe steht vor dem Laden fest (SM-PRV-009), und die
Messung des Blätterns gehört an die Oberfläche, nicht an `kern-db`. Sie steht
als Nachweis von Q2 aus.

### 9.3 Was maschinell geprüft ist

- **SM-DAT-007** — `bestehende_schritte_sind_unveraendert` prüft die Prüfsumme
  jedes ausgelieferten Schemaschritts. Die Schritte 1 bis 5 sind seit ihrer
  Aufnahme unverändert; Schritt 6 kam additiv hinzu.
- **Schnittregel 5** — `kern_security::Schreibziel` lässt sich außerhalb von
  `kern-security` nicht herstellen. Wer schreiben will, braucht es; der
  Übersetzer erzwingt die Regel, statt sie einer Zusage zu überlassen.
- **SM-SEC-005** — jeder Fremdwert steht als Platzhalter; die wenigen
  zusammengesetzten Abfragen tragen eine Begründung in derselben Zeile.

### 9.4 Was offen bleibt

- **SM-FMT-013** (Formaterkennung aus dem Inhalt) ist unerfüllt — siehe
  Abschnitt 7b. Die Erkennung gehört nach `kern-parsers` und ist dort
  vorzusehen; heute entscheidet die Dateiendung.
- **SM-NFR-004** (fünf Sekunden bis bedienbereit bei 100.000 Einträgen) hat
  keinen Messaufbau.
- **AK-02** ist mit der jetzigen Schnittstelle nicht messbar.
- **SM-IMP-005** — der Inhaltshash entsteht, die Entscheidungsvorlage bei
  Duplikaten fehlt (nach AP-08 fällt sie ohnehin in AP-12).
- Die **Fuzzing-Ziele** liegen im Baum, ein dauerhafter Lauf ist noch nicht
  eingerichtet (SM-SEC-011 verlangt ihn dauerhaft).
