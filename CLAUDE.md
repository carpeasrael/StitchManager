# CLAUDE.md

Diese Datei leitet Claude Code (claude.ai/code) bei der Arbeit in diesem Repository an.

## 0. Wie diese Datei zu lesen ist (VERBINDLICH)

Diese Datei ist Prozessvorgabe **und** Prüfmaßstab: Turing prüft gegen ihre Konventionen. Damit
sie nicht zur Quelle unlösbarer Blockaden wird, gelten vier übergeordnete Sätze. Sie stehen
über jeder Einzelregel dieser Datei.

**S1 — Jede Regel hat einen begehbaren Weg.** Eine Regel, die einen zulässigen Vorgang
verhindert, ohne einen dokumentierten Weg zu benennen, ist ein Fehler *dieser Datei*, kein
Grund zum Abbruch. Wer auf eine solche Regel stößt: den Vorgang anhalten, den Widerspruch als
offenen Punkt (Kapitel 14 des Lastenhefts) oder als Befund gegen diese Datei aufnehmen,
Rückfrage an den Nutzer stellen. Nicht raten, nicht stillschweigend umgehen.

**S2 — Nichts blockiert seine eigene Reparatur.** Ein defektes Gate, ein defekter Selbsttest,
eine widersprüchliche Regel dieser Datei und eine gestörte Reviewer-Schnittstelle dürfen die
Änderung, die genau diesen Defekt behebt, nicht verhindern. Der dafür vorgesehene Weg steht in
„Störungsfälle und Notfall-Ausstiege". Er ist protokollpflichtig, aber er existiert.

**S3 — Nicht anwendbar ist nicht dasselbe wie nicht bestanden.** Ein Gate, dessen
Gegenstand im Baum nicht existiert (kein `Cargo.toml` → keine Rust-Gates), steht als
**ENTFÄLLT** im Protokoll. Ein Gate, dessen Gegenstand existiert, das aber nicht laufen kann
(Werkzeug fehlt, Skript bricht ab), ist **FAIL**. Entscheidend ist ein maschinell prüfbares
Vorhandensein, nie eine Einschätzung. Die Zuordnung steht in „Anwendbarkeit".

**S4 — Befunde beziehen sich auf den Änderungssatz.** Reviewer und Gates bewerten den Diff,
nicht den Gesamtzustand des Repositorys. Altlasten, die die Änderung weder verursacht noch
verschlimmert, sind **Beobachtung ohne Blockerwirkung**; sie werden notiert und als offener
Punkt oder Ticket aufgenommen. Einzige Ausnahme: ein akut ausnutzbarer Sicherheitsmangel oder
ein Geheimnis im Baum — beides blockiert unabhängig vom Änderungsbezug.

**Vorrang bei Widersprüchen:** Lastenheft vor dieser Datei in Sachfragen · innerhalb dieser
Datei die speziellere Regel vor der allgemeinen · S1 bis S4 vor allem anderen. Ein erkannter
Widerspruch wird gemeldet, nicht ausgelegt.

## 1. Was dieses Repository ist

**Reines Spezifikations-Repository — es existiert noch kein Quellcode.** Der Bestand besteht
aus drei abgestimmten Dokumenten plus einem HTML-Mockup, dazu die Gate-Skripte unter
`scripts/`. Es gibt kein Build-System, keine Testsuite, keine Paketdateien; entsprechend auch
keine Build-, Lint- oder Testbefehle für Quellcode. Das Mockup ist eine einzelne, in sich
geschlossene HTML-Datei ohne Abhängigkeiten und wird direkt im Browser geöffnet
(`xdg-open Design/stitchmanager-mockup.html`).

Die Abschnitte „Arbeitsablauf" und „Commit-Freigabe-Prozess" gehen auf die `CLAUDE.md` eines
fremden Vorhabens zurück (ValidationApp V2, NestJS/React/Prisma, GxP-reguliert). Übernommen
wurde das Prozesskonzept, nicht der Stack und nicht die regulatorische Rahmung; die
Vorlagedatei ist bewusst nicht Teil dieses Repositorys und war nie Anforderungsquelle.

**Dokumentsprache ist Deutsch, und sie ist verbindlich** (SM-SET-006, RB-05). Neue Abschnitte,
Anforderungen und Änderungshistorien werden auf Deutsch geschrieben, im Stil der bestehenden
Dokumente (Sie-freie Sachsprache, Tabellenform, keine Anglizismen wo ein deutsches Wort
existiert — „Vorschau", „Wechseldatenträger", „Schlagwort").

Nicht von dieser Regel erfasst und ausdrücklich zulässig: Bezeichner im Quellcode,
Werkzeugausgaben, Commit-Trailer (`Gate-Override:`, `Gate-Report:`), Dateinamen und die
festgelegten Votumsworte `APPROVE` / `CHANGES_REQUESTED`.

## 2. Befehle

Quellcode existiert noch nicht; das Commit-Freigabe-Gate ist gebaut und läuft:

```bash
bash scripts/install-hooks.sh            # Hooks installieren — einmalig je Klon
bash scripts/install-hooks.sh --status   # Zustand anzeigen
bash scripts/install-hooks.sh --uninstall

bash scripts/review-gate.sh              # Gate von Hand, commit-Tier
REVIEW_GATE_TIER=push bash scripts/review-gate.sh
bash scripts/review-gate.test.sh         # Selbsttest, Stufe 0b (~10 s)
bash scripts/check-docs.sh               # Dokumentprüfungen, Stufe 0c (~0,1 s)
```

Stellschrauben (Vorgabewerte in Klammern): `AGENT_TIMEOUT` (900) · `DIFF_CAP_BYTES` (400000) ·
`BINARY_MAX_BYTES` (2000000) · `GATE_REVIEWERS` (newton turing tesla curie) ·
`REVIEW_GATE_CACHE_TTL_DAYS` (1) · `REPORT_RETENTION_DAYS` (14) · `GATE_LOCK_WAIT` (300) ·
`REVIEW_GATE_NO_CACHE` · `REVIEW_GATE_SMOKE` · `REVIEW_GATE_MODEL`.

`GATE_DRY_RUN=1 bash scripts/review-gate.sh` fährt die Stufen 0 bis 0c und hält vor Stufe 1.
Es entsteht **kein** Protokoll: Ein grünes Protokoll ohne Reviewer wäre eine Freigabe ohne
Prüfung und entzöge sich über die Herkunftsprüfung künftig selbst dem Prüfumfang. Aus
demselben Grund blockiert eine Reviewer-Liste mit weniger als zwei Namen.

**Regelbesetzung sind vier Reviewer.** Wo diese Datei „4/4" schreibt, ist die vollständige
Zustimmung **aller in `GATE_REVIEWERS` konfigurierten** Reviewer gemeint (mindestens zwei,
Regelfall vier). Eine verkleinerte Besetzung steht namentlich im Protokollkopf und ist für
Veröffentlichungen unzulässig.

Eine vollständige Stufe-1-Runde kostet im Normalfall wenige Minuten Wanduhrzeit — die
Reviewer laufen nebenläufig, nicht nacheinander. `AGENT_TIMEOUT` ist die Obergrenze je
Reviewer, nicht die Erwartung.

## 3. Dokumentenhierarchie

Bei Widersprüchen gilt strikt diese Rangfolge:

1. `Requirements/StitchManager_Lastenheft.md` — **URS-STM-001**, führend.
   Was das System leisten muss.
2. `Design/StitchManager_Design_Beschreibung.md` — **DES-STM-001**.
   Wie es aussieht und sich verhält; konkretisiert Kapitel 12 des Lastenhefts.
3. `TechStack/StitchManager_TechStack.md` — **TEC-STM-001**. Womit es gebaut wird.
4. `Design/stitchmanager-mockup.html` — visuelle Referenz, **keine Umsetzungsvorlage**.

Im Lastenheft referenziert, aber (noch) nicht im Repository: ANA-STM-001
(Konsolidierungsanalyse, reine Herleitung — **keine Anforderungsquelle**) und ABG-STM-001
(Abstimmungsprotokoll).

`Analysis/` und `Reviews/` sind **Nachweise**, keine Quellen: Sie dürfen keine Anforderung
begründen und sind für die Rangfolge ohne Bedeutung.

## 4. Regeln für die Arbeit an den Dokumenten

Diese Regeln stehen in den Dokumenten selbst und werden beim Bearbeiten leicht verletzt:

- **Anforderungskennungen (`SM-<Bereich>-<Nr>`) werden nie wiederverwendet.** Entfällt eine
  Anforderung, bleibt sie mit durchgestrichener Kennung (`~~SM-MFG-008~~`) und Streichgrund
  im Dokument stehen — Verweise aus anderen Dokumenten dürfen nicht ins Leere laufen. Siehe
  SM-MFG-008/009 als Muster.
- **Nur das Lastenheft darf Anforderungen begründen.** Ergibt sich beim Gestalten oder bei
  der Technologiewahl eine neue Anforderung, wird sie im Lastenheft aufgenommen und in
  DES/TEC nur referenziert.
- **Es gibt genau ein Register offener Punkte:** Kapitel 14 des Lastenhefts (OP-01 … OP-13).
  DES-STM-001 und TEC-STM-001 führen bewusst keine eigene Nummerierung, sondern verweisen
  dorthin. Kein neues Register anlegen.
- **Befunde B-01 … B-13** stammen aus ANA-STM-001 und sind Befunde am Altbestand, **keine
  Anforderungen**. Nicht als solche behandeln.
- Jede **inhaltliche** Änderung erhält einen Eintrag in der Änderungshistorie des betroffenen
  Dokuments und eine erhöhte Versionsnummer im Kopf.
- Jede Anforderung trägt **Priorität** (`M` Muss · `S` Soll · `K` Kann) und **Prüfmethode**
  (`T` Test · `D` Demonstration · `I` Inspektion · `A` Analyse). Beide Spalten sind Pflicht.

### Versionsnachzug ohne Endlosschleife

Die Kopfzeile „Mitgeltende Unterlagen" nennt die Versionen der beiden anderen Dokumente. Ohne
klare Regel entsteht daraus eine Schleife: A wird geändert → B und C ziehen die Kopfzeile nach
→ das gilt als Änderung an B und C → deren Version steigt → A muss nachziehen → und so fort.

Deshalb gilt:

- Der Nachzug einer „Mitgeltend"-Kopfzeile ist **Metadatenpflege, keine inhaltliche
  Änderung**: keine Versionserhöhung, kein Historieneintrag im nachziehenden Dokument.
- Der Nachzug geschieht **im selben Commit** wie die auslösende inhaltliche Änderung. Die
  Prüfung „Versionsabgleich" bewertet den Endzustand des Commits, nicht Zwischenstände.
- Genau **eine** Ausbreitungsrunde. Ist der Endzustand widerspruchsfrei, ist die Sache erledigt.
- Ändert sich in einem Dokument sowohl Inhalt als auch nur die Kopfzeile eines anderen, zählt
  je Dokument, was tatsächlich zutrifft.

## 5. Produkt in einem Absatz

StitchManager ist eine reine Einzelplatz-Desktop-Anwendung (macOS, Windows, Linux) zur
Verwaltung von Stickdateien (PES, DST, JEF, VP3, EXP, XXX) und PDF-Schnittmustern: Bibliothek,
Import, Metadaten, Vorschau aus Stichdaten, Volltextsuche, Formatkonvertierung, Export auf
Wechseldatenträger und **maßhaltiger Druck**. Sie arbeitet offline, verändert Originaldateien
nie (RB-04) und kennt zwei Betriebsmodi: **Standard** und **Gewerblich** (Fertigung,
Beschaffung, Kalkulation — im Standardmodus vollständig ausgeblendet, nicht deaktiviert).
Ausdrücklich nicht enthalten: Digitizing, Maschinensteuerung, Server/Cloud, Mobilplattformen,
Nutzerverwaltung.

## 6. Technische Zielarchitektur (TEC-STM-001 v2.2)

```
Qt 6 (LGPL-3)  ── Oberfläche, Qt PDF (Anzeige), QPrinter (maßhaltiger Druck)
      │  cxx-qt (Weg A, empfohlen)  oder  PySide6 + PyO3 (Weg B)
Rust-Kern      ── parsers/ writers/ render/ db/ services/ security/
      └── SQLite (rusqlite, WAL) + FTS5
```

Entscheidende Randbedingungen, die fast jede technische Frage vorentscheiden:

- **RB-01 / SM-OSS-001 bis 003: ausschließlich quelloffene Komponenten.** Daraus folgt der
  Ausschluss von Tauri (WebView2 unter Windows ist proprietär) und damit der Neubau der
  gesamten Oberflächenschicht. Electron ist ebenfalls verworfen — es löst das Druckproblem
  nicht. Kein zweiter Suchindex neben FTS5, kein eigener Aktualisierungsdienst, keine
  Server- oder Cloudkomponente.
- **Projektlizenz ist GPL-3.0** (`LICENSE`). Qt wird unter LGPL-3 **dynamisch** gebunden,
  austauschbar, mit mitgeliefertem Lizenztext (SM-OSS-006). Statisches Binden ist unzulässig.
- **Maßhaltiger Druck ist die härteste Anforderung:** 100 mm im Quelldokument müssen am
  körperlichen Ausdruck 100 mm ± 0,5 mm messen (SM-PRN-006, AK-06), auf A4 und US Letter,
  auf allen drei Plattformen. Geprüft wird am Papier, nicht an der Bildschirmvorschau.
- **Leistung:** 100.000 Einträge, Suche unter einer Sekunde bei warmem Index, virtualisierte
  Listen (SM-LIB-009, SM-SRC-007, SM-PRV-007).
- **Auslieferung:** `.dmg` (Universal) · MSI/NSIS · Flatpak auf **org.kde.Platform** (nicht
  GNOME — Qt 6 ist dort enthalten). Flatpak braucht Schreibzugriff auf Wechseldatenträger
  und Zugriff auf den Geheimnisdienst, sonst sind USB-Export und Schlüsselablage funktionslos.

## 7. Gestaltung — was nicht wegoptimiert werden darf

Die Design-Beschreibung ist ungewöhnlich präzise; die folgenden Punkte sind explizit als
nicht verhandelbar formuliert:

- **Kreuznaht-Design:** warme Palette, Terracotta als Leitfarbe, Dunkelmodus in **Espresso**,
  nie neutrales Grau. Trennlinien sind gestrichelte Nähte, die Auswahlmarke ist ein Kreuzstich.
- **Alle Farb-, Schrift- und Abstandswerte liegen als `--kn-*`-Bezeichner in genau einer
  Datei. Kein Literalwert im Komponentencode** (SM-DES-003, Prüfpunkt D-05). Die Werte in
  DES-STM-001 Abschnitt 3 sind aus dem Markenstandard *rekonstruiert* und stehen unter OP-09;
  **die Bezeichner ändern sich dabei nie** — gegen Namen entwickeln, nicht gegen Werte.
- `--kn-brand` erreicht als Textfarbe nur 3,17:1 → für Text ausschließlich `--kn-brand-ink`.
  **`--kn-ink-3` ist für lesbaren Text gesperrt** (nur Deaktiviert/Platzhalter).
- **Dreispaltiges Hauptfenster, senkrecht in jeder Fensterbreite.** Keine gestapelte
  Ersatzdarstellung; unter 860 px wird waagerecht gescrollt (SM-DES-005/006, AK-12).
- **Auf der Kachel stehen nur Bild, Format-/KI-Marke, Name und Größe** (SM-DES-007). Jede
  weitere Angabe gehört in den Detailbereich — auch wenn sie „praktisch wäre".
- **Maschinell erzeugte Werte sind doppelt gekennzeichnet**, über Farbe *und* Text, und
  bleiben es bis zur Bestätigung (SM-KIA-008, SM-DES-009).
- Kachelhöhe steht **vor** dem Laden der Vorschau fest — kein Layoutsprung (SM-PRV-009).
- Der Ausdruck übernimmt **keine** Themenfarben (SM-PRN-015).

## 8. Offene Punkte, die vor dem Bauen zu klären sind

- **OP-13** — Ist Wiederverwendung des vorhandenen Rust-Kerns (~31.500 Zeilen, 197 Tests)
  zulässig, oder ist eine Neuentwicklung ohne Wiederverwendung gemeint? Kippt TEC-STM-001.
- **OP-01** — Bleibt es bei der Einzelplatzanwendung? Entscheidet Datenhaltung und Aufbau.
- **Weg A (cxx-qt) gegen Weg B (PySide6 + PyO3)** ist noch nicht entschieden. Vorgesehen ist
  ein Prototypvergleich an zwei Punkten: virtualisierte Liste mit 100.000 Einträgen und ein
  Testdruck mit Kalibrierquadrat (± 0,5 mm). Dieser Schritt steht bewusst **vor** allem Bauen.
- **OP-02** (gewerblicher Bereich im ersten Stand?), **OP-03** (Zielformate des Schreibpfads),
  **OP-04** (Paketsignatur), **OP-05** (entfernte KI-Anbindung), **OP-06** (Produktname),
  **OP-07** (Freigabe der Festbreitenschrift), **OP-08** (Referenzgerät für Messungen),
  **OP-09** bis **OP-12** (Gestaltungsdetails).

### Was ein offener Punkt blockiert — und was nicht

„Kein offener Punkt wird stillschweigend im Code entschieden" darf nicht bedeuten, dass bis
zur Klärung aller dreizehn Punkte gar nichts geschehen kann. Maßgeblich ist die **Wirkung**
der Änderung, nicht die bloße Berührung:

| Fall | Wirkung |
|---|---|
| Die Änderung **entscheidet** den offenen Punkt oder setzt eine Antwort voraus | **Blocker.** Vorher klären, Rückfrage an den Nutzer. |
| Die Änderung ist **unter beiden Antworten gültig** (Prototyp, Messaufbau, Herleitung, Dokumentation der Alternativen) | Zulässig. Der berührte OP wird in der Analyse genannt: „berührt OP-xx, Entscheidung nicht vorweggenommen". |
| Die Änderung **schafft die Entscheidungsgrundlage** (Vergleichsmessung, Machbarkeitsnachweis) | Ausdrücklich erwünscht. |

Ein Reviewer, der einen berührten OP sieht, prüft diese drei Fälle, bevor er blockiert.

## 9. Änderungsklassen

Der Prüfaufwand richtet sich nach der Klasse. Die Einstufung ist **maschinell prüfbar** und
steht im Protokollkopf; im Zweifel gilt die höhere Klasse.

| Klasse | Definition (alle Bedingungen) | Analyse | Gates |
|---|---|---|---|
| **T** — redaktionell | Nur Rechtschreibung, Zeichensetzung, Umbruch, Formatierung, Linkziel. **Keine** `SM-…`/`AK-…`/`OP-…`-Kennung hinzugefügt, entfernt oder umgehängt; keine Zeile in `scripts/`, `.claude/`, `CLAUDE.md`, `LICENSE`; höchstens 20 geänderte Zeilen. | entfällt | 0, 0b, 0c |
| **D** — Dokument, inhaltlich | Änderung an einem der drei Fachdokumente oder am Mockup | Pflicht | 0, 0b, 0c, 1 |
| **C** — Quellcode | Jede Änderung unter dem künftigen Quellbaum | Pflicht | 0, 0b, 0c, 1 (+2 im push-Tier) |
| **G** — Werkzeug und Regeln | `scripts/`, `.claude/`, `CLAUDE.md`, Wurzelkonfiguration, unbekannte Pfade | Pflicht | alle zutreffenden, zusätzlich 0b zwingend |

Klasse T ist kein Ermessensspielraum, sondern eine Prüfbedingung: Trifft eine Bedingung nicht
zu, ist die Änderung mindestens Klasse D. Klasse T entbindet **nie** vom Secret-Scan.

## 10. Arbeitsablauf: Umsetzung und Definition of Done (VERBINDLICH)

Jede Änderung der Klassen D, C und G durchläuft vier Phasen. Keine Phase wird übersprungen.
Das gilt für Änderungen an den Spezifikationsdokumenten besonders, weil ein Dokument die
Grundlage aller späteren Nachweise ist.

### Phase 1 — Analyse (vor der Umsetzung)

Vor jeder Änderung eine Analyse erstellen, die das Problem untersucht und beantwortet:

- **Problembeschreibung** — was ist falsch bzw. was wird gefordert?
- **Betroffene Komponenten** — Dateien, Module, Dokumentabschnitte.
- **Betroffene Anforderungen** — welche `SM-…`-Kennungen berührt die Änderung, welche
  Abnahmekriterien (`AK-…`, `D-…`) hängen daran? Fehlt eine Kennung, ist zuerst zu klären,
  ob die Anforderung im Lastenheft fehlt — dann gehört sie **dorthin**, nicht in den Code.
- **Berührte offene Punkte** — mit Einordnung nach der Tabelle in Abschnitt 8.
- **Root Cause bzw. Begründung** — warum tritt das Problem auf, warum wird das Feature gebraucht.
- **Vorgeschlagener Ansatz** — schrittweiser Plan.
- **Änderungsklasse** — T/D/C/G mit einem Satz Begründung.

Ablage: `Analysis/<yyyymmdd>_<nn>_<kurzname>.md`; `<nn>` ist die laufende Nummer des Tages,
`<kurzname>` klein geschrieben, ohne Leerzeichen. Existiert ein Ticket, gehört die Analyse
zusätzlich an dessen Quelle.

**Nicht mit der Umsetzung beginnen, bevor die Analyse geschrieben und vom Nutzer ausdrücklich
freigegeben ist. Stillschweigen ist keine Freigabe.**

Die Analyse selbst löst **keine eigene Analysepflicht** aus und wird zusammen mit der
Änderung committet. Ein Analysedokument allein — ohne die zugehörige Änderung — ist
Klasse T, sofern es keine Kennungen vergibt.

### Phase 2 — Umsetzung

Die Aufgabe zu 100 % lösen, entlang des in Phase 1 freigegebenen Ansatzes. Teilumsetzungen
nur mit dokumentierter Begründung. Ändert sich der Ansatz unterwegs, wird die Analyse
nachgeführt — nicht der Ansatz stillschweigend ersetzt.

### Phase 3 — Review und Gates

Vollständig geregelt im Abschnitt „Commit-Freigabe-Prozess". Freigabe nur bei vollständiger
Zustimmung aller konfigurierten Reviewer **und** allen zutreffenden Gates grün.

**Empfehlung (Präflug):** Vor der ersten teuren Vollrunde einen einzelnen Vorreview gegen den
Arbeitsdiff laufen lassen (`/code-review` oder ein zusätzlicher, **nicht** zum Konsens
zählender Sparringspartner). Runden mit 0/4 bis 2/4 sind der Regelfall; ein billiger Vorlauf
spart mehrere vollständige Runden.

### Phase 4 — Abschluss

- Zusammenfassung der Lösung an der Quelle dokumentieren (Ticket bzw. Abschlussabschnitt im
  zugehörigen `Analysis/`-Dokument).
- Bei inhaltlichen Änderungen an einem Spezifikationsdokument: **Änderungshistorie ergänzen,
  Version im Kopf erhöhen, Kopfzeilen der beiden anderen Dokumente im selben Commit
  nachziehen** (Abschnitt 4, „Versionsnachzug ohne Endlosschleife").
- Rückverfolgbarkeit fortschreiben (Lastenheft Abschnitt 13.3): Anforderung → Umsetzung →
  Prüffall → Ergebnis. Eine Anforderung ohne zugeordneten Prüffall gilt als nicht abgenommen.
- Ticket schließen, falls eines die Quelle war.

## 11. Lint- und Prüfkette

Die Kette ist zweigeteilt: was **heute** läuft (das Repository enthält nur Dokumente) und was
**mit dem ersten Quellcode** dazukommt. Beides gehört in dieselbe CI-Stufe; SM-NFR-012 verlangt
automatisierte Prüfung, nicht Sichtprüfung.

**Prüfbereiche.** Stil- und Konsistenzprüfungen laufen über `Requirements/`, `Design/`,
`TechStack/`, `CLAUDE.md` und `scripts/`. `Reviews/` und `Analysis/` sind Nachweise: Dort
laufen nur Secret-Scan und Namensschema-Prüfung. Andernfalls würde ein Protokoll, das einen
Befund korrekt zitiert (etwa ein Farbliteral oder eine gestrichene Kennung), genau dadurch
das nächste Gate rot färben.

**Konfiguration liegt im Repository.** `.markdownlint-cli2.jsonc` legt fest: Zeilenlänge 100
für Fließtext, **ausgenommen Tabellen und Codeblöcke** (`MD013: tables:false, code_blocks:false`).
Ohne diese Ausnahme verletzt diese Datei ihre eigene Regel.

### Heute anwendbar — Dokumente und Mockup

| Prüfung | Werkzeug / Befehl | Prüft |
|---|---|---|
| Markdown-Stil | `markdownlint-cli2` mit der Konfiguration im Wurzelverzeichnis | Überschriftenfolge, Tabellensyntax, Zeilenlänge |
| Tote Anforderungsverweise | `SM-[A-Z]{3}-[0-9]{3}` über die Prüfbereiche gegen die Kennungen des Lastenhefts | Jede referenzierte Kennung existiert; gestrichene sind als `~~…~~` markiert und dürfen referenziert werden |
| Kennungs-Wiederverwendung | Doppelte **Definitionen** (erste Tabellenspalte) in URS-STM-001 | Kennungen werden nie neu vergeben. Mehrfache *Erwähnung* ist erlaubt und kein Befund |
| Versionsabgleich | Kopfzeilen „Mitgeltend" gegen die tatsächlichen Dokumentversionen im Endzustand des Commits | Die drei Dokumente verweisen auf den Stand, gegen den sie abgestimmt wurden |
| Zweitregister | `OP-[0-9]` in `Design/` und `TechStack/` | DES/TEC verweisen nur, führen keine eigene Nummerierung |
| Farbliterale | `#[0-9a-fA-F]{6}` außerhalb des `:root`-Blocks des Mockups und außerhalb Abschnitt 3 von DES-STM-001 | Vorwegnahme von D-05 |

### Mit dem ersten Quellcode verbindlich

| Schicht | Befehl | Regel |
|---|---|---|
| Rust-Format | `cargo fmt --all -- --check` | Kein Formatstreit im Review; CI bricht bei Abweichung ab |
| Rust-Lint | `cargo clippy --all-targets --all-features -- -D warnings` | Warnungen sind Fehler |
| Lizenzen | `cargo deny check licenses bans sources` | Positivliste; ein Fremdpaket lässt den Bau scheitern (SM-OSS-009, AK-11) |
| Tests | `cargo test --all` · Einzeltest: `cargo test <name> -- --exact --nocapture` | |
| Fuzzing | `cargo fuzz run <ziel>` | Je Formatparser ein Ziel, dauerhaft (SM-SEC-011, SM-FMT-012) |
| QML (Weg A) | `qmllint`, `qmlformat -n` | Nur falls cxx-qt gewählt wird |
| Python (Weg B) | `ruff check` · `ruff format --check` · `mypy` | Nur falls PySide6 gewählt wird |
| Projektregeln | eigenes Skript in der CI | D-05, SM-PLT-007 (Versionsgleichheit), SM-SEC-005 (keine Zeichenkettenverkettung in Abfragen) |

Diese Zeile ergänzt Abschnitt 12: Sobald ein Rust-Projekt existiert, gehören zusätzlich
CycloneDX-Stückliste je Veröffentlichung und CodeQL in die Kette (TEC-STM-001 Abschnitt 4
und 7.3).

**Regeln für den Umgang mit dem Linter:**

- Eine Unterdrückung (`#[allow(...)]`, `// noqa`, `qmllint-disable`) braucht in derselben
  Zeile eine Begründung mit Anforderungskennung. Unbegründete Unterdrückungen sind ein
  Review-Befund.
- Die Positivliste von `cargo-deny` wird erweitert, indem die Lizenz geprüft und der Eintrag
  begründet wird — nie, indem die Prüfung gelockert wird.
- Formatierung wird nicht im Review diskutiert. Was `cargo fmt` erzeugt, ist richtig.

## 12. Sensible Daten (VERBINDLICH)

Im Repository liegen keine Geheimnisse — weder in Code noch in Dokumenten, Analysen oder
Review-Protokollen. Das betrifft Zugangsdaten, API-Schlüssel, SSH-Zugänge, Zertifikate und
private Schlüssel sowie produktive Konfigurationswerte.

- Ablageort für Geheimnisse ist `/.secure/` im Wurzelverzeichnis, geführt in `.gitignore`.
  **Solange dieses Verzeichnis nicht existiert, gehören Geheimnisse gar nicht ins Repository.**
- Zur Laufzeit gilt SM-SEC-006 und SM-KIA-010: Zugangsschlüssel liegen ausschließlich im
  Schlüsselspeicher des Betriebssystems — nie in der Datenbank, nie in einer Klartextdatei,
  nie im Protokoll. Nachzuweisen über AK-09.
- Werte werden aus der Umgebung gelesen, nie hartkodiert.
- Protokolle enthalten keine unmaskierten vollständigen Pfade und keine personenbezogenen
  Daten, sofern sie exportierbar sind (SM-SEC-010).
- Inhalte aus `/.secure/` werden nicht in Antworten, Commits oder Protokolle übernommen.
- Der Secret-Scan in Stufe 0 des Gates **ergänzt** diese Sorgfalt, er ersetzt sie nicht.

**Beispielwerte in Dokumentation** verwenden ausschließlich offensichtliche Platzhalter
(`sk-BEISPIEL-NICHT-ECHT`, `AKIAEXAMPLE…`) oder die vom Scanner mitgelieferten Testmuster.
Ein echter, widerrufener Schlüssel ist kein zulässiges Beispiel. Bewusste Ausnahmen werden in
derselben Zeile markiert und im Commit begründet; die Markierung erlaubt **eine** Zeile, nie
eine Datei oder ein Verzeichnis.

## 13. Commit-Freigabe-Prozess (VERBINDLICH)

Jede Änderung durchläuft dieses Gate vollständig, **bevor** ein Commit freigegeben wird. Ein
Commit gilt erst als freigegeben, wenn **alle konfigurierten Reviewer** einig sind **und alle**
zutreffenden Gates grün sind.

**Umsetzung.** `scripts/review-gate.sh` führt den Ablauf aus, `scripts/install-hooks.sh`
verdrahtet ihn als `pre-commit` und `pre-push`, `scripts/review-gate.test.sh` ist der
Selbsttest der Stufe 0b und `scripts/check-docs.sh` sind die deterministischen
Dokumentprüfungen der Stufe 0c. Die Protokolle landen unter `Reviews/`.

**Geltungsbereich — ehrlich, nicht suggestiv.** Ein Git-Hook ist client-seitig, wird je Klon
von Hand installiert und hat dokumentierte Ausstiege. Die Protokolle unter `Reviews/` sind
damit ein **Sorgfaltsnachweis**, kein **Freigabenachweis**: Sie belegen, dass ein Gate
gelaufen ist und was es gesehen hat — nicht, dass kein Commit ohne Gate entstehen konnte.
„VERBINDLICH" ist eine Prozessvorgabe an die Beteiligten, keine technische Unmöglichkeit.
Die serverseitige Durchsetzung (Branch-Schutz auf `main`, CI-Lauf des push-Tiers) ist offen.

**Dieses Projekt ist nicht reguliert.** Die Nachweispflicht kommt nicht aus einer
Aufsichtsvorgabe, sondern aus dem Lastenheft selbst: SM-NFR-012 verlangt automatisierte
Prüfung, Abschnitt 13.2 verlangt Messung statt Einschätzung, Abschnitt 13.3 verlangt
Rückverfolgbarkeit. Die Review-Protokolle sind kein regulatorischer Datensatz und unterliegen
keiner Aufbewahrungspflicht über diesen Zweck hinaus.

### Anwendbarkeit — ENTFÄLLT, FAIL, PASS

Jedes Gate beantwortet zuerst eine maschinell prüfbare Frage nach seinem Gegenstand:

| Gate | Gegenstand vorhanden, wenn … | Fehlt der Gegenstand | Gegenstand da, Werkzeug fehlt |
|---|---|---|---|
| Rust-Format/Lint/Test/Deny/Fuzz | `Cargo.toml` im Baum | ENTFÄLLT | **FAIL** mit Installationshinweis |
| QML | `*.qml` im Baum | ENTFÄLLT | **FAIL** |
| Python | `pyproject.toml` oder `*.py` im Baum | ENTFÄLLT | **FAIL** |
| Markdown-Stil, Dokumentprüfungen | immer | — | **FAIL** |
| Secret-Scan | immer | — | Regex-Rückfall, nie ENTFÄLLT |

**ENTFÄLLT steht namentlich im Protokoll, nie als PASS.** Damit gilt der Satz „ein nicht
durchgeführter Test ist kein bestandener Test" unverändert — er trifft nur nicht mehr Gates,
deren Gegenstand es noch gar nicht gibt. Ohne diese Unterscheidung wäre heute jede Änderung an
`scripts/` blockiert, weil sie „alle Gates zieht" und die Rust-Gates mangels Projekt nicht
laufen können.

**Änderungsbezug.** Eine reine Dokumentänderung zahlt keine Code-Gates und umgekehrt. Alles
Uneindeutige — Wurzelkonfigurationen, `scripts/`, unbekannte Pfade — zieht **alle zutreffenden**
Gates (Klasse G).

### Störungsfälle und Notfall-Ausstiege — abschließende Liste

Andere Wege gibt es nicht. Jeder wird laut protokolliert und ist **nie** für Veröffentlichungen
zulässig:

| Ausstieg | Wirkung | Auflage |
|---|---|---|
| `REVIEW_GATE_DISABLE=1 git commit …` | Gate komplett aus | Commit-Trailer `Gate-Override: <Grund/Ticket>`; Nachreview vor dem Merge, als Ticket angelegt |
| `git push --no-verify` | push-Tier (Stufe 2) entfällt | Commit-Trailer `Gate-Override: …`; Stufe 2 vor dem Merge nachholen |
| `DIFF_CAP_ALLOW_TRUNCATE=1` | Kürzt einen überlangen Reviewer-Diff, statt zu blockieren | Kürzung steht sichtbar im Protokoll und in dessen Zusammenfassung; **letztes Mittel nach dem Stückelungsweg** |

Kein Ausstieg erlaubt es, **einzelne Suiten** eines laufenden Gates zu überspringen. Ein
Schalter, der Tests nicht ausführt, ist kein Ausstieg: Er blockiert fail-closed.

**Benannte Störungsfälle.** Diese Lagen sind kein Regelbruch, sondern vorgesehene Anwendung des
ersten Ausstiegs — sie werden genau so protokolliert und ziehen eine Nachprüfpflicht nach sich:

1. **Reparatur des Gates selbst (S2).** Ist `review-gate.sh` oder der Selbsttest so defekt, dass
   das Gate nicht startet, ist der Commit, der genau diesen Defekt behebt, mit
   `Gate-Override: gate-reparatur <kurzbeschreibung>` zulässig. Auflagen: Der Commit ändert
   ausschließlich Gate-Dateien, der Selbsttest ist danach grün, und der nächste reguläre Lauf
   deckt den Reparatur-Commit mit ab (`merge-base(main)..HEAD` erfasst ihn).
2. **Widerspruch in dieser Datei (S1).** Blockiert eine Regel dieser Datei einen zulässigen
   Vorgang ohne Ausweg, wird angehalten und der Nutzer gefragt. Kein Selbst-Override.
3. **Reviewer-Schnittstelle gestört oder Kontingent erschöpft.** Kein technischer Mangel am
   Code — aber auch kein Votum. Regel: einmal automatisch wiederholen, danach eine
   Wartepause, danach ist der Vorgang zu verschieben. Ist das nicht möglich, greift der erste
   Ausstieg mit `Gate-Override: reviewer-nicht-erreichbar` und **verpflichtendem** Nachreview
   vor dem Merge. Ein Ausfall der Werkzeugkette darf nicht dazu führen, dass tagelang gar
   nichts committet werden kann — er darf aber auch nicht unbemerkt bleiben.

### Reihenfolge und Tiers

Billig vor teuer, fail-fast:

```
0  Secret-Scan + Vorfilter   →  0b  Gate-Selbsttest  →  0c  deterministische Schnell-Gates
                            →  1   Vier-Augen-Konsens  →  2  schwere Gates
```

- **commit-Tier** (`pre-commit`): Stufen 0, 0b, 0c und 1 über den Index.
- **push-Tier** (`pre-push`): zusätzlich Stufe 2.

**Diffbereich im push-Tier.** Vorrangig `@{upstream}..HEAD`; existiert kein Upstream,
`merge-base(origin/main, HEAD)..HEAD`. Ergibt der berechnete Bereich einen **leeren Diff,
obwohl Commits zu pushen sind**, ist das ein Fehler der Bereichsberechnung und blockiert —
ein leerer Diff ist kein bestandener Review. Auf `main` selbst greift damit der Upstream-Weg
statt der früheren Merge-Base-Formel, die dort systematisch leer lief.

**Häufig pushen, nicht sammeln.** Der Reviewer-Diff des push-Tiers wächst mit jedem
ungepushten Commit. Ein Branch mit dreißig ungepushten Commits kommt bauartbedingt nicht durch
Stufe 1.

### Querschnittliche Prinzipien

Jeweils fail-closed und nur in Richtung PASS/APPROVE wirkend, nie umgekehrt:

- **Parallelität.** Die Reviewer laufen nebenläufig; das Protokoll wird in fester Reihenfolge
  geschrieben und bleibt dadurch deterministisch.
- **Gate-Cache.** Ein PASS eines deterministischen Gates darf über eine Inhaltssignatur
  wiederverwendet werden; ein FAIL nie. Der Cache-Schlüssel enthält die Signatur des
  Gate-Skripts, den Rollentext, den Reviewer-Prompt **und `REVIEW_GATE_MODEL`** — wer Fokus,
  Prompt, Modell oder Gate-Logik ändert, bekommt kein altes Ergebnis zurück.
- **Votum-Wiederverwendung.** Ein **APPROVE** darf bei byte-identischem Reviewer-Diff,
  unverändertem Arbeitsbaum und unveränderter Gate-Signatur wiederverwendet werden, mit
  Herkunftsvermerk. **CHANGES_REQUESTED wird nie wiederverwendet.**
- **Nebenläufigkeit.** Je Repository läuft höchstens ein Gate gleichzeitig. Ein zweiter Lauf
  **derselben** Sitzung ist ein Verdrahtungsfehler und bricht sofort mit klarer Meldung ab,
  statt `GATE_LOCK_WAIT` abzuwarten — ein Gate, das auf sich selbst wartet, ist ein Deadlock,
  kein Wartefall.
- **Nachweisbindung ohne Nachtrag.** Der Protokollkopf trägt den geprüften Tree, die
  Gate-Version, das Modell und die Reviewer-Besetzung. Die Bindung an den Commit entsteht
  über den Commit-Trailer `Gate-Report: <pfad> <sha256>` — **nicht** durch nachträgliches
  Ändern des bereits committeten Protokolls. Grund: Ein Nachtrag wäre eine Änderung an einem
  committeten Protokoll, damit prüfpflichtig, damit ein neuer Gate-Lauf, damit ein neues
  Protokoll — eine Schleife ohne Ende.

**Prüfumfang.** Aus dem Reviewer-Diff ausgenommen sind nur neu angelegte Gate-Protokolle unter
`Reviews/`, die dem Namensschema entsprechen **und** nachweislich aus einem Lauf dieses Gates
stammen — geprüft wird die Herkunft, nicht der Name. Änderungen und Löschungen bereits
committeter Protokolle sind immer prüfpflichtig. **`Cargo.lock` wird nie ausgenommen:** Es ist
hier kein beiläufiges Ableitungsprodukt, sondern Gegenstand von SM-PLT-009 (reproduzierbare
Bauläufe) und SM-OSS-009 (Lizenzprüfung) — eine neue Abhängigkeit ist eine prüfpflichtige
Entscheidung.

**Protokolle betten keinen Rohdiff ein.** Sie enthalten Kennwerte (Dateien, Zeilen, SHA-256
des Reviewer-Diffs) und höchstens kurze Belegausschnitte je Befund. Ein eingebetteter
Volldiff würde bei jeder Runde den nächsten Diff vergrößern und die Kappungsgrenze
absehbar aus sich selbst heraus reißen.

**Protokolldateiname.** `Reviews/<yyyymmdd>-<hhmmss>_<branch>.md`, wobei `<branch>` bereinigt
wird: alles außer `[A-Za-z0-9._-]` wird zu `-`, Länge auf 60 Zeichen gekürzt. Ohne diese
Regel scheitert jeder Branch mit `/` im Namen an der Dateierzeugung.

### Stufe 0 — Secret-Scan und Vorfilter (deterministisch, vorgeschaltet)

1. **Secret-Scan.** Die gestageten Änderungen werden lokal und fail-closed gescannt. Sensible
   Pfade (`/.secure/`, `.env`, `*.pem`, `*.key`, `id_rsa`) werden immer geprüft. Für den
   Inhaltsscan wird `gitleaks` bevorzugt, sonst greift ein Regex-Rückfall. Jeder Treffer
   blockiert sofort; Werte werden **nie** ins Protokoll geschrieben, nur `datei:zeile` und
   Mustername. Bewusste Ausnahmen werden in der Zeile markiert (Abschnitt 12).
2. **Diff-Kappung.** Überschreitet der Reviewer-Diff `DIFF_CAP_BYTES`, startet **kein**
   Reviewer über den Gesamtdiff: Ein gekürzter Diff trägt keine Konsensaussage. Vorgehen in
   dieser Reihenfolge:
   1. Änderungssatz aufteilen (Regelweg).
   2. Ist er nicht teilbar, weil eine **einzelne** Datei die Grenze überschreitet
      (Erstimport, große Spezifikation): **Stückelungslauf.** Die Datei wird in geordnete
      Abschnitte mit Überlappung zerlegt, jeder Abschnitt vollständig geprüft; das Votum je
      Reviewer ist die Konjunktion über alle Abschnitte, ein einziges CHANGES_REQUESTED
      blockiert. Der Protokollkopf nennt Abschnittszahl und -grenzen.
   3. Erst danach `DIFF_CAP_ALLOW_TRUNCATE=1` als Notfall-Ausstieg.
3. **Prompt-Injection: Neutralisierung vor Filterung.** Der Diff geht als Text an die
   Reviewer. Primäre Maßnahme ist **strukturell**, nicht mustergestützt:
   - Der Gate erzeugt je Lauf ein Zufallstoken (Nonce). Der Diff steht im Prompt in einem
     mit diesem Token begrenzten Datenblock, jede Diffzeile zusätzlich zeilenweise
     eingerückt. Eine Diffzeile kann damit im Prompt nicht am Zeilenanfang stehen.
   - Der Prompt weist den Block ausdrücklich als **Datenmaterial** aus, nie als Anweisung.
   - Das Votum wird nur aus der **letzten** Antwortzeile gelesen und muss exakt
     `VERDICT: <nonce> APPROVE` oder `VERDICT: <nonce> CHANGES_REQUESTED` lauten. Da das
     Token erst nach dem Einlesen des Diffs entsteht, kann kein Diffinhalt ein gültiges
     Votum vorwegnehmen. Eine Votumszeile ohne gültiges Token gilt als **ABGEBROCHEN**.
   - Der Mustervorfilter bleibt als zweite Schicht: In Quellcodepfaden blockieren
     hinzugefügte Zeilen, die eine Rolleninstruktion imitieren. In Dokumentations- und
     Nachweispfaden **warnt** er nur und vermerkt den Fund im Protokoll — sonst könnte diese
     Datei, die das Votumsformat beschreibt, nicht committet werden.
4. **Binärdateien.** Sie liefern im Diff nur „Binary files differ" und stehen deshalb
   namentlich im Abschnitt „Binärdateien (inhaltlich ungeprüft)". Das ist in diesem Projekt
   kein Randfall: Stickdateien und PDF-Schnittmuster als Prüfmaterial sind binär. Oberhalb
   einer Größengrenze und außerhalb einer Positivliste blockieren sie. Prüfmaterial für Parser
   gehört mit Herkunftsangabe dokumentiert, nicht kommentarlos hinzugefügt.

### Stufe 0b — Gate-Selbsttest (vor den Reviewern)

`scripts/review-gate.test.sh` läuft **vor** Stufe 1 (rund 10 Sekunden). Grund: Ist die
Reviewer-Verdrahtung defekt, scheitern alle Reviewer fail-closed, das Gate bricht bei 0/n ab —
und ausgerechnet die Diagnose liefe nie, während mehrere Zeitbudgets verbrennen. Ein roter
Selbsttest blockiert sofort; die Ausnahme ist der Störungsfall 1 (Reparatur des Gates).

Ergänzend gilt die bekannte Grenze: Der Selbsttest arbeitet mit nachgebildeten CLIs und sieht
deshalb nicht, dass die echte Reviewer-Schnittstelle ein Flag nicht mehr akzeptiert. Dafür
läuft unmittelbar vor Stufe 1 ein kurzer Rauchtest gegen die echte Schnittstelle
(`REVIEW_GATE_SMOKE=0` schaltet ihn ab).

### Stufe 0c — Deterministische Schnell-Gates (vor den Reviewern)

Inhalt ist die **Lint- und Prüfkette** aus Abschnitt 11 — heute die Dokumentprüfungen, mit dem
ersten Quellcode zusätzlich Format, Lint, Tests, Lizenzprüfung und die Projektregeln (D-05,
SM-PLT-007, SM-SEC-005). Anwendbarkeit nach der Tabelle in „Anwendbarkeit".

Sie laufen **vor** Stufe 1 und blockieren fail-fast: Ein nicht übersetzbarer oder nicht
prüfbarer Stand verbrennt keine Reviewer-Ressourcen. Alle sind cachebar und laufen über den
Änderungsbezug nur bei betroffener Änderung, sonst „ENTFÄLLT".

**Zur Lizenz- und Abhängigkeitsprüfung:** Sie gehört deterministisch ins Gate, nicht zu einem
Reviewer. „Abhängigkeitsrisiken" ist Teslas Dimension, aber ein Sprachmodell kennt keine
Sicherheitsmeldungen nach seinem Trainingsstand. Umgekehrt gilt: Ein dauerhaft rotes
Pflicht-Gate wird umgangen, nicht befolgt. Bekannte, nicht sofort behebbare Funde stehen
deshalb **namentlich, begründet und befristet** in einer Baseline-Datei; blockiert wird alles
**Neue**. Ein Eintrag ohne Begründung blockiert. Ein abgelaufener Eintrag blockiert ebenfalls —
verlängert wird er nur durch eine **erneute, datierte Bewertung** im selben Eintrag, nie durch
stilles Hochsetzen des Datums. Damit bleibt der Ablauf ein Anlass zur Prüfung und wird nicht
zur unlösbaren Sperre im ungünstigsten Moment.

### Stufe 1 — Vier-Augen-Konsens

Jede Änderung wird von den konfigurierten Agenten geprüft, die ohne Kenntnis der jeweils
anderen Bewertungen arbeiten. Rollen, Dimensionen und Prüfaufträge stehen im Abschnitt
„Review-Agenten" und in `.claude/agents/`.

- Alle laufen **parallel** und als Claude-Code-Subagenten. Es gibt in diesem Projekt
  **keine zweite Engine und keinen Engine-Rückfall.**
- Jeder Agent liefert ein ausdrückliches Votum: **APPROVE** oder **CHANGES_REQUESTED** mit
  Begründung, als letzte Zeile im Nonce-Format aus Stufe 0.
- **Freigabe nur bei vollständiger Zustimmung.** Ein einziges CHANGES_REQUESTED blockiert.
- **Abbruch ist kein Votum.** Zeitüberschreitung, technischer Fehler oder ein nicht lesbares
  Votum gelten als **ABGEBROCHEN** und zählen fail-closed wie CHANGES_REQUESTED — sichtbar
  getrennt ausgewiesen, weil „Mangel gefunden" einen Fix am Code verlangt und „nicht
  geantwortet" einen an der Verdrahtung. Ein technischer Abbruch wird je Reviewer genau
  **einmal** automatisch wiederholt; danach greift Störungsfall 3.
- Bei CHANGES_REQUESTED: Befunde dokumentieren, beheben, dann **erneut von allen** prüfen
  lassen — nicht nur von den zuvor roten.
- Liefert ein Reviewer CHANGES_REQUESTED ohne einen einzigen blocker- oder major-Befund, gibt
  es genau **eine** Klärungsrückfrage; bleibt er dabei, **gilt sein Votum**. Keine stille
  Umdeutung in APPROVE.
- Dubletten werden zusammengeführt. Widersprüche zwischen Agenten werden **aufgelöst**, nicht
  überstimmt: Ein APPROVE des einen impliziert kein APPROVE des anderen.
- Reviewer haben ausschließlich **Lesewerkzeuge** (Read, Grep, Glob); Bash, Schreibwerkzeuge
  und MCP sind gesperrt. Sie schreiben ihre Befunde nicht selbst weg — das Gate erzeugt das
  Protokoll aus ihren Antworten.
- **Daraus folgt: Reviewer messen nicht, sie prüfen Nachweise.** Kontrastwerte, Laufzeiten und
  Druckmaße werden von deterministischen Gates oder von Hand ermittelt und im Änderungssatz
  belegt. Ein Reviewer verlangt den Nachweis, wo einer fehlt — er behauptet keinen Messwert
  und blockiert nicht mangels eigener Messmöglichkeit.
- Der Prompt geht über **stdin**, nie als Kommandozeilenargument: Linux begrenzt ein einzelnes
  Argument auf 128 KB, die Diff-Grenze liegt darüber. Wer das zurückdreht, lässt jeden Diff
  über 128 KB stillschweigend an allen Reviewern scheitern (Selbsttestfall F9).
- Ist ein Reviewer wiederholt zu langsam, wird das Zeitbudget erhöht — nicht das Votum
  interpretiert.

Die Dimensionen sind an zwei Stellen hinterlegt: in diesem Dokument und in den Dateien unter
`.claude/agents/`. **Änderungen immer im Gleichschritt.** Ein Gate-Check vergleicht die
Rollenüberschriften und Prüfschwerpunkte beider Seiten und meldet Abweichungen als Befund
gegen die Änderung, die sie erzeugt hat — nicht gegen die nächste beliebige Änderung.

### Stufe 2 — Schwere Gates (nur im push-Tier)

Erst nach vollständigem Konsens. Sobald Quellcode existiert:

1. **Volle Testsuite** statt der änderungsbezogenen Auswahl des commit-Tiers.
2. **Fuzzing-Kurzlauf** über alle Formatparser-Ziele (SM-SEC-011, SM-FMT-012).
3. **Lizenz- und Abhängigkeitsprüfung** als Veröffentlichungsnachweis (SM-OSS-009, AK-11).
4. **Sicherheitsprüfung** gegen den Diff — mindestens Pfadprüfung und Eingrenzung
   (SM-SEC-001 bis 003), parametrisierte Abfragen (SM-SEC-005), Nur-Text-Darstellung von
   Fremdtext (SM-SEC-008), Schlüsselablage (SM-KIA-010), Parser gegen manipulierte Dateien
   (SM-FMT-012).

Solange kein `Cargo.toml` existiert, stehen 1 bis 3 als ENTFÄLLT im Protokoll.

**Was hier bewusst nicht läuft:** Die messenden Abnahmenachweise — Import von 100.000 Dateien
(AK-01), maßhaltiger Ausdruck mit ± 0,5 mm am körperlichen Papier (AK-06), Kontrastrechnung
über alle Bildschirme (AK-07) — sind je Veröffentlichung zu erbringen, nicht je Push. Ein Gate,
das einen Drucker und ein Lineal braucht, ist kein Hook. Ihr Ergebnis gehört in die
Rückverfolgbarkeitsmatrix.

## 14. Review-Agenten

Die vier Rollen liegen als Subagenten unter `.claude/agents/` — `newton.md`, `turing.md`,
`tesla.md`, `curie.md` — und werden über den Agent-Aufruf mit ihrem Namen gestartet. Sie sind
**Stufe 1 des Commit-Freigabe-Prozesses**; die dort festgelegten Regeln (Parallelität,
vollständiger Konsens, Votum als letzte Zeile, Abbruch ist kein Votum, Änderungsbezug nach S4)
gelten für sie unverändert. Dieser Abschnitt ist die Kurzfassung; die Dateien enthalten die
vollständigen Prüfaufträge.

Jede Rolle hat einen **primären** Blickwinkel, der Vorrang hat, und einen **sekundären**, der
bei Gelegenheit mitgeprüft wird. Die Sekundärfoki bleiben, weil vier Rollen sonst blinde
Flecken hätten — die GxP-Begründung der Vorlage entfällt hier ersatzlos, ebenso deren
Aufteilung auf zwei Engines.

Befunde werden mit der betroffenen Anforderungskennung belegt; eine Beobachtung ohne Bezug auf
Lastenheft, Design-Beschreibung oder eine Regel dieser Datei ist eine Meinung, kein Befund.
**Eine Ausnahme, damit diese Regel keinen echten Mangel verschluckt:** Findet ein Reviewer
einen Mangel, für den keine Kennung existiert, meldet er ihn als **Befund ohne Bezug** mit
Vorschlag für eine neue Anforderung im Lastenheft. Blockerwirkung hat er nur, wenn er
sicherheits- oder datenverlustrelevant ist; sonst ist er eine Beobachtung samt OP-Vorschlag.

> **Herkunft der Rollentexte:** Sie stammen aus der genannten Vorlage (ValidationApp V2,
> React/Mantine). Die stackspezifischen Begriffe sind hier abgebildet: Mantine-Theme-Token →
> `--kn-*`-Variablen aus DES-STM-001 Abschnitt 3 · Inline-Styles → Literalwerte im
> Komponentencode (SM-DES-003) · DTO-Validierung → Validierung an der Kernschnittstelle und in
> den Formatparsern · `api/v1`-Prefix → die eng geschnittene Schnittstelle zwischen Oberfläche
> und Rust-Kern (SM-SEC-004) · Workflow-Konstanten → Statuswerte aus SM-MET-008 und die
> Projektzustände.

| Agent | Primär | Sekundär |
|---|---|---|
| **Newton** | Performance | Korrektheit und Logik |
| **Turing** | Design-Konsistenz | Architektur, Wartbarkeit, Konventionen dieser Datei |
| **Tesla** | Sicherheit (OWASP) | — |
| **Curie** | Bedienbarkeit und Barrierefreiheit | Testabdeckung und Regressionen |

### Newton — Performance

**Primär:** algorithmische Komplexität, N+1-Abfragen, unnötige Allokationen, synchrone E/A auf
heißen Pfaden, Zeichenkosten in der Oberfläche, große Datentransfers.

**Sekundär:** Fehler und Logikfehler, Wettlaufsituationen, Fehlerbehandlung, Datenkonsistenz,
Statuslogik der Abläufe.

Prüfschwerpunkte in diesem Projekt: SM-LIB-009 und SM-NFR-001 (100.000 Einträge),
SM-SRC-007 (Suchergebnis unter einer Sekunde bei warmem Index), SM-PRV-007 (virtualisierte
Listen — jede Schleife über den Gesamtbestand im Zeichenpfad ist ein Befund), SM-NFR-002
(Import, Indizierung und Vorschauerzeugung blockieren die Bedienung nicht), SM-NFR-004
(fünf Sekunden bis bedienbereit), SM-PRV-002/003 (Vorschau-Zwischenspeicher und seine
Verwerfung). Bei der Datenbank: eine Abfrage je Ansicht, nicht eine je Zeile; FTS5 statt
Filterung in der Anwendung. Leistungsanforderungen werden **gemessen** — Newton prüft, ob eine
Messung vorliegt und ob sie den Anspruch trägt, und schätzt keine Zahlen.

### Turing — Design-Konsistenz

**Primär:** Wiederverwendung bestehender Komponenten statt Einzelstücken, konsistente Nutzung
der Theme-Variablen über Hell- und Dunkelmodus, keine Literalwerte am Designsystem vorbei,
visuelle Parität mit Schwesterkomponenten.

**Sekundär:** Architektur und Wartbarkeit sowie die Konventionen dieser Datei — Modulstruktur,
Zugriffsschranken, Validierung an der Schnittstelle, Schnitt zwischen Oberfläche und Kern,
Statuskonstanten.

Prüfschwerpunkte: SM-DES-003 und D-05 (kein Farb-, Schrift- oder Abstandsliteral außerhalb der
Variablendatei — der häufigste Befund), Abschnitt 7 von DES-STM-001 (Zustandstabelle: ein neues
Bedienelement übernimmt die dort festgelegten Zustände, es erfindet keine eigenen),
SM-DES-007 (auf der Kachel steht nichts über Bild, Marke, Name und Größe hinaus), Abschnitt 5
(Grundraster 4 px, festgelegte Radien), SM-SEC-004 (die Oberfläche greift nie direkt auf die
Datenhaltung zu), SM-DAT-007 (Migrationen additiv, bestehende Schritte unverändert).
Ein neues Einzelstück, für das eine Schwesterkomponente existiert, ist ein Befund — auch wenn
es für sich genommen gut aussieht.

Turing prüft auch die Konventionen dieser Datei. Findet er dabei einen **Widerspruch in dieser
Datei selbst**, ist das ein Befund gegen die Datei (S1), kein Blocker gegen die vorliegende
Änderung — es sei denn, die Änderung erzeugt den Widerspruch.

### Tesla — Sicherheit (OWASP)

Umgehung von Authentifizierung, Autorisierung und Rechteprüfung; unsicherer Objektzugriff
(IDOR); Einschleusung (SQL, XSS); Cross-Site Request Forgery; Abfluss von Geheimnissen;
unsichere Datei-Uploads; Risiken aus Abhängigkeiten.

Prüfschwerpunkte in diesem Projekt — die Anwendung ist eine Einzelplatzanwendung ohne
Nutzerverwaltung, die klassischen Web-Angriffsflächen verschieben sich damit auf Dateien und
Pfade: SM-SEC-001 bis 003 (Pfadprüfung, Kanonisierung beider Seiten, Symlinks,
Groß-/Kleinschreibungsfaltung, Ablehnung von Systemwurzeln), SM-SEC-005 (parametrisierte
Abfragen, keine Zeichenkettenverkettung), SM-SEC-008 (Fremdtext — Dateinamen, Metadaten,
maschinell erzeugte Antworten — ausschließlich als Nur-Text; ein auszeichnungsfähiges
Textelement an einer solchen Stelle ist ein Befund), SM-KIA-010 und SM-SEC-006
(Zugangsschlüssel nur im Schlüsselspeicher des Betriebssystems, nie in Datenbank oder
Klartext), SM-FMT-012 (Parser gegen manipulierte Dateien: kein Absturz, keine unbegrenzte
Speicherbelegung, keine Endlosschleife), SM-SEC-007 und SM-NFR-014 (ausgehende Verbindungen
begrenzt und abschaltbar, keine Telemetrie), SM-SEC-010 (Protokolle ohne unmaskierte Pfade),
SM-SEC-013 (kein automatisches Update ohne Signaturprüfung), SM-OSS-009/011 (Lizenz- und
Herkunftsrisiken in Abhängigkeiten).

Der Import ist der Upload-Pfad dieses Programms: jede eingelesene Datei ist Fremddaten, auch
wenn sie aus dem Dateisystem des Nutzers stammt.

Tesla ist die einzige Rolle, deren Befunde **unabhängig vom Änderungsbezug** blockieren dürfen
(S4, zweite Hälfte) — beschränkt auf akut ausnutzbare Mängel und Geheimnisse im Baum. Alles
Übrige folgt auch bei ihm dem Änderungsbezug.

### Curie — Bedienbarkeit

**Primär:** Nutzerfluss, Auffindbarkeit von Aktionen, Tastatur- und Fokusbehandlung, Qualität
der Fehlermeldungen, Barrierefreiheit (WCAG, Kontrast, zugängliche Benennung), Korrektheit der
deutschen Oberflächentexte, leere sowie ladende und fehlerhafte Zustände.

**Sekundär:** Testabdeckung und Regressionen — fehlende oder unzureichende Tests, ungeprüfte
Pfade, brüchige Annahmen.

Prüfschwerpunkte: SM-NFR-007 (4,5:1 für Fließtext, 3:1 für große Schrift, in **beiden** Modi —
gerechnet, nicht nach Augenmaß; die Rechnung liefert das Kontrast-Gate, Curie prüft ihr
Vorliegen und ihre Vollständigkeit), SM-NFR-008 (vollständige Tastaturbedienung; jeder Dialog
hält den Fokus und gibt ihn beim Schließen zurück), Abschnitt 7 von DES-STM-001 (der Fokusring
wird nie unterdrückt, auch nicht bei Zeigerbedienung; Bedienelemente mindestens 26 × 26 px
sichtbar und 32 × 32 px Trefferfläche), SM-NFR-009 und SM-KIA-008 (kein Zustand allein über
Farbe; maschinell erzeugte Werte auch ohne Farbwahrnehmung erkennbar), SM-NFR-013 (bei
reduzierter Bewegung entfallen **alle** Übergänge, keine Ausnahme), Abschnitt 10 von
DES-STM-001 (jeder leere, ladende und fehlerhafte Zustand ist ausformuliert — ein Bildschirm
ohne diese drei Zustände ist unfertig), SM-NFR-006 (Fehlertexte für Endnutzer verständlich,
technische Angaben ins Protokoll), SM-SET-006 (vollständig deutschsprachig, Terminologie der
Begriffstabelle in Abschnitt 1.6 des Lastenhefts), SM-MET-009 (ungespeicherte Änderungen
werden erkannt und nachgefragt).

Zur Testabdeckung: jede Muss-Anforderung braucht mindestens einen zugeordneten Nachweis
(Abschnitt 13.2 des Lastenhefts). Eine Änderung an einem Formatparser ohne neuen Prüffall und
ohne Fuzzing-Fall ist ein Befund. Fehlende Nachweise für Anforderungen, die die Änderung nicht
berührt, sind Beobachtung nach S4, kein Blocker.

## 15. Observations und Fix-Vorschläge

Beobachtungen aus allen Stufen werden dokumentiert. Je Befund:

- **Beobachtung** — was, wo (`datei:zeile`), durch welchen Agenten oder welches Gate gefunden.
- **Kennung** — die betroffene Anforderung (`SM-…`, `AK-…`, `D-…`) oder die Regel aus dieser
  Datei. Fehlt beides: „Befund ohne Bezug" mit Vorschlag (Abschnitt 14).
- **Änderungsbezug** — verursacht/verschlimmert durch diese Änderung, oder Altlast (S4).
- **Schweregrad** — blocker / major / minor / beobachtung.
- **Fix-Vorschlag** — konkret, idealerweise mit Diff-Skizze. Nicht nur das Problem benennen.
- **Status** — offen / behoben / akzeptiertes Risiko mit Begründung.

Ablage: `Reviews/<yyyymmdd>-<hhmmss>_<branch>.md` (Namensbereinigung siehe oben). Der Commit
erfolgt erst, wenn alle blocker- und major-Befunde **mit Änderungsbezug** behoben sind;
Altlasten werden als offener Punkt oder Ticket überführt, akzeptierte Restrisiken ausdrücklich
begründet. Rote Protokolle werden nicht committet.

## 16. Checklisten

### commit-Tier (pre-commit)

- [ ] Änderungsklasse bestimmt (T/D/C/G) und im Protokollkopf vermerkt
- [ ] Ab Klasse D: Analyse (Phase 1) geschrieben und vom Nutzer ausdrücklich freigegeben
- [ ] Berührte offene Punkte eingeordnet (entscheidend / neutral / grundlagenschaffend)
- [ ] Präflug-Vorreview gegen den Arbeitsdiff gelaufen (Empfehlung)
- [ ] Stufe 0 grün: Secret-Scan, Diff-Kappung, Injection-Neutralisierung, Binärdateien benannt
- [ ] Stufe 0b grün: Gate-Selbsttest
- [ ] Stufe 0c grün oder begründet ENTFÄLLT: Lint- und Prüfkette, soweit anwendbar
- [ ] Vollständiger Konsens aller konfigurierten Reviewer, parallel und unabhängig
- [ ] Observations dokumentiert, alle blocker und major mit Änderungsbezug behoben
- [ ] Bei inhaltlicher Dokumentänderung: Historie, Version und Kopfzeilen im selben Commit
- [ ] Commit-Trailer `Gate-Report: <pfad> <sha256>` gesetzt

### push-Tier (pre-push)

- [ ] Diffbereich korrekt bestimmt und nicht leer
- [ ] Volle Testsuite grün oder ENTFÄLLT (kein `Cargo.toml`)
- [ ] Fuzzing-Kurzlauf über alle Parser-Ziele grün oder ENTFÄLLT
- [ ] Lizenz- und Abhängigkeitsprüfung grün oder ENTFÄLLT
- [ ] Sicherheitsprüfung gegen den Diff grün
- [ ] Rückverfolgbarkeit fortgeschrieben: Anforderung → Umsetzung → Prüffall → Ergebnis
- [ ] Offene `Gate-Override:`-Trailer im Bereich? Dann Nachreview vor dem Merge angelegt

## Anhang A — Prüfmuster für neue Regeln in dieser Datei

Jede Regel, die dieser Datei hinzugefügt wird, beantwortet vier Fragen. Fehlt eine Antwort,
gehört die Regel noch nicht hierher:

1. **Wer prüft sie** — deterministisches Gate, Reviewer oder Mensch?
2. **Woran** — an einer maschinell prüfbaren Bedingung oder an einer Einschätzung?
3. **Was passiert, wenn die Prüfung nicht laufen kann** — ENTFÄLLT oder FAIL (S3)?
4. **Welcher begehbare Weg bleibt, wenn sie blockiert** (S1) — und blockiert sie ihre eigene
   Korrektur (S2)?
