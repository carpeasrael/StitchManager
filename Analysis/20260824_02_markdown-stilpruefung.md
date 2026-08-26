# Markdown-Stilprüfung verbindlich machen — Analyse vor der Umsetzung

**Datum:** 24.08.2026
**Auslöser:** Auftrag, den in `Analysis/20260824_01_readme.md` Abschnitt 8 festgehaltenen
Beobachtungsbefund als eigenen Vorgang aufzusetzen: `.markdownlint-cli2.jsonc` fehlt, obwohl
`CLAUDE.md` Abschnitt 11 sie als vorhanden führt.
**Vorbefund:** Dieselbe Sache ist bereits zweimal aktenkundig — als **T4-5** in
`Analysis/20260823_01_gate-befunde-rueckstand.md` (Schweregrad major, Status „offen — zweiter
Änderungssatz") und als Beobachtung in `Analysis/20260823_03_implementierungsplan.md`
Abschnitt 6.2.
**Änderungsklasse:** G (Begründung in Abschnitt 7).

---

## 1. Problembeschreibung

Der Mangel hat zwei Hälften, die getrennt zu behandeln sind.

**Erste Hälfte — die Konfigurationsdatei fehlt.** `CLAUDE.md` Abschnitt 11 schreibt:
„Konfiguration liegt im Repository. `.markdownlint-cli2.jsonc` legt fest: Zeilenlänge 100 für
Fließtext, ausgenommen Tabellen und Codeblöcke". Die Datei existiert nicht. Die Aussage ist
damit unwahr, und die Ausnahme für Tabellen — auf die sich sämtliche Dokumente stützen — ist
nirgends hinterlegt.

**Zweite Hälfte — das Fehlen des Werkzeugs wird falsch bewertet.** Die Anwendbarkeitstabelle
in `CLAUDE.md` Abschnitt 13 ordnet der Zeile „Markdown-Stil, Dokumentprüfungen" den Gegenstand
**immer** zu und schreibt für den Fall „Gegenstand da, Werkzeug fehlt" ausdrücklich **FAIL**
vor. `scripts/check-docs.sh` meldet stattdessen `skip`. Damit widerspricht das Skript der
Regel, gegen die es prüft, und ein Gate meldet grün, obwohl eine Pflichtprüfung nicht lief —
genau das, was der Satz „ein nicht durchgeführter Test ist kein bestandener Test" ausschließt.
Nach S3 ist das ein Befund, kein zulässiger ENTFÄLLT-Fall: Markdown-Dateien liegen im Baum.

Betroffen ist SM-NFR-012, der automatisierte Prüfung verlangt, nicht Sichtprüfung.

## 2. Korrektur der eigenen Einschätzung

In der vorangegangenen Antwort war vorgeschlagen, den Punkt als **offenen Punkt in Kapitel 14
des Lastenhefts** aufzunehmen. **Dieser Vorschlag war falsch**, und zwar aus einem im
Repository bereits dokumentierten Grund: `Analysis/20260823_03_implementierungsplan.md`
Abschnitt 6.2 hat die Frage schon entschieden — „Ein offener Punkt des Lastenhefts wäre der
falsche Ort, weil die Sache keine Anforderung berührt, sondern die Werkzeugkette dieses
Repositorys." Kapitel 14 führt offene **Anforderungsfragen**; die Wahl eines Prüfwerkzeugs ist
keine.

Der Weg über einen offenen Punkt wäre zudem teuer und würde zwei Folgeänderungen erzwingen,
die mit der Sache nichts zu tun haben:

| Folge | Grund |
|---|---|
| URS-STM-001 auf **v1.4**, Historieneintrag, Kopfdatum | Kapitel 14 ist Teil des Lastenhefts; jede inhaltliche Änderung erhöht die Version |
| Kopfzeilen in DES-STM-001, TEC-STM-001 und IMP-STM-001 nachziehen | Versionsnachzug im selben Commit |
| Eintrag im Entscheidungsgatter des Implementierungsplans | `scripts/check-plan.sh` Regel 7 erzwingt für **jeden** geführten Punkt einen Eintrag in Kapitel 2 |
| Zahlwort in Kapitel 2.4 des Plans von „zwanzig" auf „einundzwanzig" | `scripts/check-plan.sh` Regel 8 leitet es aus der Zahl der Punkte ab |

Vier Dokumente änderten sich also, weil eine Konfigurationsdatei fehlt. Der richtige Weg ist
der bereits vorgesehene: **ein Änderungssatz der Klasse G**, der den Mangel behebt, und die
Schließung des Befunds T4-5 an seiner Quelle.

## 3. Betroffene Komponenten

| Datei | Art der Änderung |
|---|---|
| `Analysis/20260824_02_markdown-stilpruefung.md` | **neu** — dieses Dokument |
| `.markdownlint-cli2.jsonc` | **neu** — die fehlende Konfiguration |
| `scripts/check-docs.sh` | geändert — Prüfung 7 von `skip` auf `finding`, sobald die Voraussetzungen stehen |
| `CLAUDE.md` Abschnitt 11 | geändert — Werkzeugfassung und Bezugsweg benannt, Regelabschaltungen begründet |
| `Requirements/…`, `Design/…`, `TechStack/…`, `Implementation/…`, `Analysis/20260823_01_…` | geändert — die 19 tatsächlichen Stilbefunde aus Abschnitt 4 |
| `package.json` | **neu**, nur bei Variante A der Werkzeugbindung |

**Nicht geändert:** die drei Fachdokumente inhaltlich. Sämtliche Eingriffe sind Umbrüche und
Auszeichnungen — keine Versionserhöhung, kein Historieneintrag, kein Kopfzeilennachzug.

**Nachtrag vom 26.08.2026 — tatsächlicher Umfang des Werkzeug-Satzes.** Diese Analyse wurde
für die Markdown-Stilprüfung geschrieben; der Änderungssatz, mit dem sie committet wird, ist
breiter. Zusätzlich berührt sind `scripts/check-projektregeln.sh` und deren Selbsttest,
`scripts/check-plan.sh` und deren Selbsttest, die neue gemeinsame Regelbibliothek
`scripts/lib/gestaltung.sh`, die Lizenz- und Herkunftsprüfung `scripts/lib/lizenzen.py` mit
`.lizenzen.conf`, die Zuordnungsdatei `.projektregeln.conf` sowie die Umbauten in
`scripts/review-gate.sh` (Kistenauswahl, Signaturbindung, Stufe 0b). Deren fachliche
Herleitung steht in `Analysis/20260823_01_gate-befunde-rueckstand.md` und
`Analysis/20260823_03_implementierungsplan.md`; hier wird sie nur benannt, damit die
Aufstellung nicht schmaler ist als der Satz.

## 4. Messung statt Einschätzung

Vor jeder Festlegung wurde der Linter probeweise gegen den Bestand gefahren — mit **genau der
Konfiguration, die `CLAUDE.md` beschreibt** (`MD013` auf 100, `tables:false`,
`code_blocks:false`), über `**/*.md` ohne `Reviews/`. Ergebnis:

| Regel | Befunde | Bewertung |
|---|---|---|
| **MD060** — Tabellenspaltenstil | **903** | Die Regel verlangt gepolsterte Trennstriche in einem bestimmten Stil und ist in neueren Fassungen des Werkzeugs hinzugekommen. Der Bestand ist durchgängig anders gesetzt |
| **MD013** — Zeilenlänge | **11** | Echte Befunde, alle behebbar |
| **MD040** — Codeblock ohne Sprachangabe | **8** | Echte Befunde, alle behebbar |
| Summe | **922** | |

**Das ist der eigentliche Grund, warum die Datei nie angelegt wurde.** Wer sie so anlegt, wie
`CLAUDE.md` sie beschreibt, und den Linter installiert, färbt das Gate mit 922 Befunden
dauerhaft rot — und ein dauerhaft rotes Pflicht-Gate wird umgangen, nicht befolgt. Die
Konfiguration braucht deshalb mehr als die zwei dokumentierten Zeilen.

Die 903 MD060-Befunde verteilen sich über **jede** Markdown-Datei des Repositorys, die eine
Tabelle enthält. Sie mechanisch zu beheben hieße, sämtliche Tabellen in allen vier
Fachdokumenten umzuschreiben — ein Diff, der die Kappungsgrenze `DIFF_CAP_BYTES` reißt und
jeden Reviewer über eine Änderung ohne fachlichen Gehalt laufen ließe.

Die 19 echten Befunde im Einzelnen:

| Ort | Regel | Sache |
|---|---|---|
| `Analysis/20260823_01_…` Zeilen 47 bis 53 | MD013 | Eine Befundtabelle ist durch eine Leerzeile getrennt fortgesetzt, ohne die Kopfzeile zu wiederholen. Der Linter sieht deshalb keine Tabelle, sondern sieben überlange Fließtextzeilen. **Markup-Fehler, kein Zeilenlängenproblem** |
| `Analysis/20260823_01_…` Zeile 230 | MD013 | Fließtextzeile, 108 Zeichen |
| `CLAUDE.md` Zeile 414 | MD013 | Fließtextzeile, 131 Zeichen |
| `Design/…` Zeile 6 | MD013 | Kopfzeile „Führendes Dokument", 103 Zeichen |
| `Design/…` Zeile 280 | MD013 | Fließtextzeile, 144 Zeichen |
| `CLAUDE.md` (2), `Design/…` (1), `Implementation/…` (1), `Requirements/…` (1), `TechStack/…` (1) | MD040 | Codeblöcke ohne Sprachangabe — Systemkontext, Architekturskizze, Stufenfolge |

Zwei weitere MD040-Befunde in `README.md` sind bei dieser Messung aufgefallen und **bereits im
README-Änderungssatz behoben** (Sprachangabe `text` an beiden Blöcken), weil sie von jener
Änderung verursacht waren.

**Nachtrag vom 26.08.2026 — am gelieferten Stand gemessen.** Die Zahl „69 bestanden" stammt
aus dem Stand vor dem Werkzeug-Satz. Gemessen auf Apple Silicon, warmer
Dateisystemzwischenspeicher, Freigabebau:

| Werkzeug | Laufzeit | Fälle |
|---|---|---|
| `scripts/check-docs.sh` | 1,2 s | — |
| `scripts/check-projektregeln.sh` | 0,7 s | — |
| `scripts/check-plan.sh` | 0,1 s | — |
| `scripts/review-gate.test.sh` | 40,4 s | 137 |
| `scripts/check-projektregeln.test.sh` | 12,1 s | 57 |
| `scripts/check-plan.test.sh` | 1,2 s | 26 |
| Trockenlauf Stufen 0 bis 0c | 47 s | — |

Die Einzelwerte addieren sich **nicht** zum Trockenlauf: Sie sind je für sich kalt gemessen,
im Trockenlauf laufen sie nacheinander gegen einen warmen Zwischenspeicher. Die Werte sind
**Regressionsschwellen, keine Zusagen** — ohne Referenzgerät (OP-08) gibt es dafür keine
Grundlage.

## 5. Betroffene Anforderungen und offene Punkte

| Kennung | Bezug |
|---|---|
| **SM-NFR-012** | Verlangt automatisierte Prüfung. Eine Prüfung, die mangels Werkzeug stillschweigend entfällt, erfüllt die Zusage nicht |
| **SM-OSS-009** | Bei Variante A der Werkzeugbindung entsteht eine Fremdabhängigkeit mit Lizenz- und Herkunftsfrage. `markdownlint-cli2` steht unter MIT; die Prüfung ist zu belegen, nicht zu behaupten |

**Berührte offene Punkte: keiner.** Die Änderung entscheidet keinen Punkt aus Kapitel 14 und
setzt keine Antwort voraus. Sie legt auch keinen neuen an — siehe Abschnitt 2.

## 6. Vorgeschlagener Ansatz

Die Reihenfolge ist zwingend: Erst muss die Prüfung grün laufen können, dann darf sie
blockieren. Andersherum blockiert das Gate seine eigene Reparatur (S2).

1. **`.markdownlint-cli2.jsonc` anlegen.** Inhalt: `MD013` mit `line_length: 100`,
   `tables: false`, `code_blocks: false` — wie in `CLAUDE.md` beschrieben — **zuzüglich einer
   begründeten Abschaltung von MD060**. Jede Abschaltung trägt in derselben Datei einen
   Kommentar mit Grund; eine unbegründete Unterdrückung ist nach Abschnitt 11 ein
   Review-Befund.
2. **Die 19 echten Befunde beheben**, in der Aufstellung aus Abschnitt 4. Der Tabellenbruch in
   `Analysis/20260823_01_…` wird durch Entfernen der trennenden Leerzeile geschlossen, nicht
   durch Umbrechen der Zeilen.
3. **Werkzeugbindung festlegen** — die einzige echte Wahl, siehe Abschnitt 6.1.
4. **`scripts/check-docs.sh` umstellen:** fehlendes Werkzeug ergibt `finding` statt `skip`,
   mit Installationshinweis in der Meldung. Damit stimmt das Skript wieder mit der
   Anwendbarkeitstabelle überein.
5. **`CLAUDE.md` Abschnitt 11 nachführen:** Fassung und Bezugsweg des Werkzeugs, die
   abgeschalteten Regeln mit Grund.
6. **Selbsttest ergänzen:** ein Fall, der belegt, dass ein fehlendes Werkzeug blockiert statt
   zu entfallen. Ohne ihn fällt die Umstellung beim nächsten Umbau lautlos zurück.
7. `bash scripts/check-docs.sh` und `bash scripts/review-gate.test.sh` grün; danach Stufe 1.

### 6.1 Zu entscheiden — Werkzeugbindung

| Variante | Vorgehen | Preis |
|---|---|---|
| **A — Manifest** | `package.json` mit `markdownlint-cli2` als Entwicklungsabhängigkeit und fester Fassung; Aufruf über `npx` | Node-Abhängigkeitsbaum in einem Repository, das sonst Rust und Qt vorsieht; `node_modules/` ist bereits in `.gitignore` geführt. Fassung ist reproduzierbar gebunden |
| **B — dokumentierte Installation** | Kein Manifest; `CLAUDE.md` nennt Werkzeug, Mindestfassung und Installationsbefehl, das Gate prüft nur die Anwesenheit | Kein Fremdbaum im Repository. Die Fassung ist **nicht** gebunden — eine neuere Fassung kann neue Regeln mitbringen und das Gate über Nacht rot färben, genau wie MD060 es hier vorgeführt hat |

Variante A wird empfohlen: Der eben gemessene Fall ist der Beweis dafür, dass eine
ungebundene Werkzeugfassung dieses Gate unvorhersehbar macht.

### 6.2 Zu entscheiden — Umgang mit MD060

| Variante | Vorgehen | Preis |
|---|---|---|
| **1 — abschalten** | `MD060` in der Konfiguration aus, mit Begründung in derselben Datei | Der Tabellenstil bleibt ungeprüft. Er ist im Bestand ohnehin einheitlich |
| **2 — Bestand umformatieren** | 903 Stellen in allen Markdown-Dateien angleichen | Ein sehr großer Diff ohne fachlichen Gehalt, der die Kappungsgrenze reißt und einen Stückelungslauf erzwingt |

Variante 1 wird empfohlen.

### 6.3 Änderungssatz

Ursprünglich war ein eigener Satz **nach** dem Plan- und dem README-Satz vorgesehen: Er berührt
vier Dateien, die auch dort geändert werden, und eine Verschränkung schien Konflikte ohne Not zu
erzeugen.

> **Verworfen am 24.08.2026, nach Runde 14.** Die Trennung wurde versucht und ist gescheitert.
> Newton, Turing und Curie meldeten unabhängig denselben Befund: Die `README.md` erklärte
> `npm install` zum verbindlichen ersten Schritt und begründete das mit S3, während der Baum
> ohne diesen Satz weder `package.json` noch die Konfiguration trug und `check-docs.sh` die
> Stilprüfung weiterhin als entfallen meldete. Alle drei nannten als ersten Fix dasselbe: die
> Sätze zusammenführen. Der Zwischenstand war nicht bloß grob geschnitten, er war **falsch** —
> die Aussage der README wird erst mit der Werkzeugbindung wahr. Beide Sätze sind deshalb
> zusammengeführt; die Historie ist gröber, dafür ist kein committeter Stand unrichtig.

## 7. Änderungsklasse

**Klasse G.** Die Änderung fasst `scripts/`, `CLAUDE.md` und eine neue Wurzelkonfiguration an —
drei Merkmale, von denen jedes für sich Klasse G auslöst. Die Eingriffe in die Fachdokumente
sind rein redaktionell und ändern die Klasse nicht nach unten; im Zweifel gilt ohnehin die
höhere.

Gates: 0, 0b, 0c und 1. Rust-, QML- und Python-Gates stehen als **ENTFÄLLT** im Protokoll.

## 8. Abschluss

**Umgesetzt am 24.08.2026**, mit den vom Nutzer gewählten Varianten: **A** (Manifest),
**1** (MD060 abschalten) und Umstellung des Gates im selben Satz.

| Schritt | Ergebnis |
|---|---|
| 1 · Konfiguration | `.markdownlint-cli2.jsonc` angelegt; `MD013` wie beschrieben, `MD060` abgeschaltet, jede Abschaltung mit Grund in derselben Datei. `node_modules/` und `.git/` ausgenommen |
| 2 · Befunde | Alle 17 verbliebenen behoben: 8 Codeblöcke mit `text` ausgezeichnet, 5 Fließtextzeilen umbrochen, die zerrissene Befundtabelle in `Analysis/20260823_01_…` durch Entfernen der Leerzeile geschlossen |
| 3 · Werkzeugbindung | `package.json` mit `markdownlint-cli2` in fester Fassung (MIT), `package-lock.json` im Baum, Skript `npm run lint:md` |
| 4 · Gate | `scripts/check-docs.sh` Prüfung 7: fehlendes Werkzeug ergibt **finding** statt `skip`, mit Installationshinweis. Lokale Installation hat Vorrang vor einer globalen |
| 5 · Regelwerk | `CLAUDE.md` Abschnitt 1 (Paketdatei benannt und abgegrenzt), Abschnitt 2 (`npm install`, `npm run lint:md`) und Abschnitt 11 (Fassung, Bezugsweg, MD060-Abschaltung, FAIL statt ENTFÄLLT) nachgeführt |
| 6 · Selbsttest | Neuer Fall **J** mit drei Prüfungen: fehlendes Werkzeug blockiert, nennt den Installationsweg und meldet die Prüfung **nicht** als entfallen |

**Ein Nebenfund aus Schritt 2, der ohne diese Arbeit unentdeckt geblieben wäre:** Nach dem
Schließen der zerrissenen Tabelle meldete der Linter eine Zeile mit sechs statt fünf Zellen.
Ursache war ein unmaskierter senkrechter Strich innerhalb eines Codeabschnitts — in einer
Tabellenzeile trennt er die Zelle, auch zwischen Akzentzeichen. Der Befundtext war dadurch
verstümmelt dargestellt. Maskiert und damit behoben.

Deterministische Nachweise nach der Umsetzung:

| Prüfung | Ergebnis |
|---|---|
| `npm run lint:md` | 0 Verstöße in 15 Dateien — zuvor 922 |
| `bash scripts/check-docs.sh` | grün, **Markdown-Stil läuft jetzt tatsächlich**; nur noch eine Prüfung entfällt (Farbliterale im Mockup) statt zweier |
| `bash scripts/review-gate.test.sh` | 69 bestanden, 0 fehlgeschlagen — zuvor 66 Fälle |

**Zwischenbefund während der Umsetzung:** Die Umstellung auf FAIL ließ zunächst 14 Fälle des
Selbsttests scheitern, weil dessen Prüfrepos das Werkzeug nicht haben und deshalb schon in
Stufe 0c ausfielen. Behoben durch eine nachgebildete Fassung in `make_repo` — dieselbe Technik,
mit der der Selbsttest die Reviewer-Schnittstelle nachbildet. Fall J entfernt sie eigens wieder,
sonst prüfte er sich selbst am Vorbeiweg.

**Offen:** Stufe 1 des Gates über den zusammengeführten Änderungssatz und der Commit.

Mit dieser Umsetzung schließt der Befund **T4-5** aus
`Analysis/20260823_01_gate-befunde-rueckstand.md`; dessen Status ist dort auf „behoben"
gesetzt.

**T4-6** derselben Aufstellung — abweichende Prüfbereiche zwischen Skript und Abschnitt 11 —
war zum Zeitpunkt dieser Analyse offen und ist **inzwischen geschlossen**: Der Werkzeug-Satz
vom 26.08.2026 nimmt `*.sh` und die Oberflächensprachen beider Wege in `sources()` auf. Die
Aufstellung wird hier fortgeschrieben, weil beide Befunde denselben Absatz betreffen:

| Hälfte von T4-6 | Stand |
|---|---|
| „`Analysis/` zu viel" | erledigt — Abschnitt 11 führt `Analysis/` inzwischen ausdrücklich auf, mit Begründung |
| „`scripts/` zu wenig" | **erledigt** — `sources()` in `scripts/check-docs.sh` bildet seinen Bereich seit dem Werkzeug-Satz aus `KN_QUELL_EXT` zuzüglich `rs`, `sh` und `html`; Schaltdateien und die Oberflächensprachen beider Wege laufen damit mit |
| `README.md` | mit dem README-Änderungssatz in die Aufzählung aufgenommen; Skript und Abschnitt 11 stimmen für diesen Pfad überein |

## 9. Übernommene Befunde der roten Gate-Läufe (Werkzeug-Satz, 26.08.2026)

Rote Protokolle werden nicht committet (CLAUDE.md Abschnitt 15). Ihre blocker- und
major-Befunde stehen deshalb hier in Kurzform: **Ort, Kennung, Schweregrad, Status** — nie der
beanstandete Wert selbst, sonst trüge dieses Dokument genau den Inhalt in den Prüfbereich, um
dessentwillen der Befund entstand.

### 9.1 Runden 1 bis 3 — behoben

| Runde | Reviewer | Kurzbezeichnung | Kennung | Schwere | Status |
|---|---|---|---|---|---|
| 2 | Tesla | Kistenname aus Pfad in Befehlszeichenkette | SM-NFR-012 | blocker | behoben, Prüffälle O1–O4 |
| 2 | Turing | Bibliothek eingebunden, Duplikate blieben stehen | SM-DES-003 | major | behoben |
| 2 | Turing | D-05 weiter durchgesetzt als die Anforderung reicht | SM-DES-003 | major | behoben, Umfang in CLAUDE.md |
| 2 | Curie | README-Versionszweig konnte nie zünden | CLAUDE.md Abs. 11 | major | behoben, T-8 wieder offen geführt |
| 2 | Curie | Zeitgrenze im Selbsttest ohne Verfügbarkeitsprüfung | S1/S3 | major | behoben |
| 3 | Curie | D-05 und SM-SEC-005 fielen ohne Interpreter still aus | S3 | blocker | behoben, Abbruch mit Installationshinweis |
| 3 | Tesla | Secret-Scan las den Arbeitsbaum statt des Index | SM-SEC-006 | major | behoben, Prüffälle U1/U1b |
| 3 | Tesla | Zugangsdatei gelistet, vom Muster nicht erfassbar | SM-SEC-006 | major | behoben, Prüffall U2 |
| 3 | Tesla | Ältere Lockfile-Fassung bestand mit null Paketen | SM-OSS-009 | major | behoben, Prüffälle V1/V2 |
| 3 | Newton | Zwischenspeicherung der Dateilisten in Subshells wirkungslos | Abs. 13 | major | behoben |
| 3 | Newton | Ein Prozessstart je Datei | Abs. 13 | major | behoben, Stapelbetrieb |
| 3 | Newton | Laufzeitschwellen für den Stand unbelegt | Abs. 2 | major | behoben, gemessen (Abschnitt 4) |
| 3 | Turing | Prüflücken bei Deklaration, Zuweisung, Methodenform | SM-DES-003 | major | behoben, Prüffälle U1–U5b |
| 3 | Curie | Herkunftsbefund ohne begehbaren Weg | S1 | major | behoben, Ausnahmeform aufgenommen |
| 3 | Curie | Rautefolgen in Markdown als Farbliteral gewertet | SM-DES-003 | major | behoben, Prüffälle X1/X2 |

### 9.2 Runde 4 — offen, Gegenstand des Nachreviews

Zwei Reviewer (Newton, Tesla) sind technisch abgebrochen — Störungsfall 3, kein Mangel am
Code und **kein** Votum. Die beiden anderen meldeten keinen Blocker.

| Reviewer | Ort | Kurzbezeichnung | Kennung | Schwere | Status |
|---|---|---|---|---|---|
| Turing | `scripts/check-docs.sh` | Farbklasse baumweit zugesagt, nur ein Teil umgesetzt | SM-DES-003, D-05 | major | **offen** |
| Turing | `scripts/check-docs.sh` | zweite, abweichende Farbregel für DES-STM-001 | SM-DES-003 | major | **offen** |
| Turing | `scripts/lib/gestaltung.sh` | Zeichen-API der Oberfläche fällt durch D-05 | SM-DES-003 | major | **offen** |
| Turing | `scripts/review-gate.sh` | Fuzzing-Anwendbarkeit am falschen Ort geprüft | SM-SEC-011 | major | **offen** |
| Curie | `scripts/review-gate.test.sh` | Prüffall der Cache-Signatur ohne Deckung | Abs. 13.2 | major | **offen** |

Turing meldete zusätzlich vier, Curie fünf minor-Befunde (Doppelungen in der Regelbibliothek,
dreifacher Leser der Zuordnungsdatei, toter Anteilszweig, uneinheitliche Umlautschreibung,
Form der Prüfattrappen).

**Der erste Befund wiegt am schwersten und ist zuerst zu beheben:** Die in diesem Commit
aufgenommene Zusage in `CLAUDE.md` Abschnitt 11 — die Farbklasse gelte baumweit, einschließlich
benannter Farben und Farbfunktionen — ist breiter als der Code. Außerhalb des Oberflächenpfads
läuft nur die Hex-Prüfung. Regelwerk und Werkzeug widersprechen sich damit im selben
Änderungssatz.

### 9.3 Warum trotzdem committet wurde

Auf Weisung des Auftraggebers vom 26.08.2026, über den in CLAUDE.md Abschnitt 13 vorgesehenen
Ausstieg mit Commit-Trailer `Gate-Override:`. Auflage nach derselben Stelle: **Nachreview vor
dem Merge**, dessen Gegenstand Abschnitt 9.2 ist. Für Veröffentlichungen ist dieser Weg
ausgeschlossen.
