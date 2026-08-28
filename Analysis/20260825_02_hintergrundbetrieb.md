# Analyse — Hintergrundbetrieb für Import, Indizierung und Vorschau (SM-NFR-002)

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260825-02 |
| Datum | 2026-08-25 |
| Auslöser | Auftrag des Nutzers: „mach weiter mit SM-NFR-002" |
| Änderungsklasse | **C** — Quellcode |
| Mitgeltend | URS-STM-001 v1.3 · DES-STM-001 v1.3 · TEC-STM-001 v2.2 · IMP-STM-001 |
| Vorgänger | [`20260825_01_anwendungsbau.md`](20260825_01_anwendungsbau.md), Abschnitt 9.4 |

## 1. Problembeschreibung

SM-NFR-002 verlangt: „Import, Indizierung und **Vorschauerzeugung** laufen im
Hintergrund und blockieren die Bedienung nicht." Der bisherige Stand verletzt das
an **zwei** Stellen, nicht an einer:

1. **Einlesen des Bestands.** `bestand_einlesen` läuft vollständig im
   Qt-Faden. Bei 100.000 Dateien steht das Fenster für die Dauer des Laufs;
   kein Bildaufbau, keine Eingabe, kein Abbruch.
2. **Vorschauerzeugung.** `vorschau_pfad` wird aus der Kachelbindung heraus
   aufgerufen und liest die Datei, wertet die Stichdaten aus, zeichnet und
   schreibt ein PNG — **im Zeichenpfad**. Das ist synchrone Ein-/Ausgabe auf
   dem heißesten Pfad der Anwendung.

Zusätzlich unerfüllt: **SM-IMP-002** verlangt für den Import ausdrücklich eine
**Fortschrittsanzeige**; **AK-01** verlangt, dass die Bedienung während des
Imports von 100.000 Dateien möglich bleibt.

## 2. Betroffene Komponenten

| Komponente | Art der Berührung |
|---|---|
| `crates/kern-services` | **neu gefüllt** — `Kernbetrieb`: Arbeitsfaden, der die Fassade besitzt |
| `crates/ui/src/bruecke.rs` | umgebaut — die Brücke hält die Fassade nicht mehr selbst |
| `crates/ui/qml/Haupt.qml` | Fortschrittsanzeige, Abbruchmöglichkeit, Vorschau über eine Modellrolle |
| `crates/kern-fassade` | unverändert |

## 3. Betroffene Anforderungen

**Erfüllt werden:** SM-NFR-002 (M), SM-IMP-002 (M), SM-BAT-005 (Fortschritt und
Abbruch, hier für den Importlauf), Vorlauf zu AK-01.

**Berührt, unverändert gültig:** SM-PRV-002 (Zwischenspeicher), SM-NFR-005
(defekte Dateien führen nicht zum Absturz), SM-SEC-004 (die Oberfläche erreicht
den Kern weiterhin nur über die Fassade — der Arbeitsfaden liegt **hinter** ihr,
nicht daneben), SM-NFR-006 (Fehlertexte für Endnutzer).

## 4. Berührte offene Punkte

| Punkt | Einordnung | Begründung |
|---|---|---|
| OP-18 | **neutral** | Betrifft Schwellen für Eingabelatenz und Entprellintervall. Diese Änderung nimmt keine Zahl vorweg; sie beseitigt die Blockade, statt sie zu bemessen |
| OP-08 | grundlagenschaffend | Der Messaufbau entsteht; ohne Referenzgerät bleibt der Wert Regressionsschwelle |
| OP-15 | **neutral** | Der ladende Zustand ist in DES Abschnitt 10 bereits ausformuliert; ob er eine **eigene Kennung** braucht, entscheidet diese Änderung nicht |

## 5. Root Cause

Die Fassade hält eine `rusqlite`-Verbindung. Diese ist `Send`, aber nicht
`Sync` — sie lässt sich **verschieben**, aber nicht teilen. Der erste Stand hat
sie deshalb im QObject belassen, und damit lief jeder Kernzugriff zwangsläufig
im Qt-Faden. Das ist die eigentliche Ursache; sie ist nicht durch Nachbessern
einzelner Aufrufe zu beheben, sondern nur dadurch, dass die Fassade den Faden
wechselt.

## 6. Vorgeschlagener Ansatz

**Ein Arbeitsfaden besitzt die Fassade.** Die Oberfläche schickt Aufträge und
empfängt Antworten; sie hält selbst keine Verbindung mehr.

```text
Qt-Faden                              Arbeitsfaden
  Musterliste (QObject)                 Kernbetrieb
   · zeilen: Vec<Kachel>                 · fassade: Fassade  ← besitzt die Verbindung
   · vorschauen: Uid → Pfad              · Warteschlange
   · beauftragen(…)      ──────────────►   Befehle  (Wurzel, Einlesen, Ausschnitt)
   · antwort_verarbeiten ◄──────────────   Vorschauen (eigene Schlange)
        über CxxQtThread::queue
```

**Zwei Schlangen, nicht eine.** Vorschauaufträge entstehen beim Bildlauf
massenhaft; lägen sie in derselben Schlange wie die Befehle, verzögerte ein
Schwall von Vorschauen den nächsten Ausschnittabruf. Der Arbeitsfaden nimmt
deshalb **immer zuerst einen Befehl** und erst danach **eine** Vorschau.

**Die Vorschauschlange ist begrenzt und wird von hinten abgearbeitet.** Wer
schnell blättert, erzeugt Aufträge für Kacheln, die längst wieder aus dem Bild
sind. Zuletzt angefordert heißt zuletzt sichtbar — deshalb LIFO. Läuft die
Schlange über, fällt der **älteste** Auftrag heraus; er wird neu gestellt,
sobald Qt die Zeile wieder erfragt.

**Die Vorschau wird zur Modellrolle.** `data()` liefert den zwischengespeicherten
Pfad oder eine leere Zeichenkette und stellt den Auftrag genau einmal. Kein
Aufruf im Zeichenpfad wartet auf Ein-/Ausgabe. Ist ein Bild fertig, meldet die
Brücke `dataChanged` für **diese eine Zeile**.

**Abbruch und Fortschritt.** Ein `AtomicBool` bricht den Importlauf zwischen
zwei Dateien ab (SM-BAT-005). Der Fortschritt wird gebündelt gemeldet, nicht je
Datei — sonst überflutet die Meldung die Ereignisschlange, die sie gerade frei
halten soll.

## 7. Prüfplan

`kern-services` ist **ohne Qt prüfbar**, weil der Rückkanal ein Verschluss ist
und keine Qt-Bindung:

- Ein langer Importlauf ist abbrechbar; der Abbruch wirkt in beschränkter Zeit.
- Befehle werden **vor** wartenden Vorschauen bedient (Vorrangregel).
- Die Vorschauschlange läuft nicht über die Grenze hinaus.
- Vorschauen werden von hinten abgearbeitet (zuletzt angefordert zuerst).
- Ein Fehler in einer Datei hält den Lauf nicht an (SM-NFR-005).
- Der Fortschritt läuft monoton und endet bei der Gesamtzahl.

## 8. Abgrenzung

Nicht Gegenstand: SM-IMP-003 (inkrementeller Import), SM-IMP-004
(Ordnerüberwachung), SM-IMP-005 (Duplikaterkennung) und SM-IMP-006
(Importvorschau). Sie sind eigene Arbeitspakete und bleiben offen.

---

## 9. Abschluss (Phase 4)

### 9.1 Umsetzung

`kern-services::Kernbetrieb` besitzt die Fassade und arbeitet zwei Schlangen ab —
Befehle mit Vorrang, Vorschauen von hinten und begrenzt auf 256 Aufträge. Die
Brücke hält **keine** Fassade mehr; sie stellt Aufträge und nimmt Antworten über
`CxxQtThread::queue` im Qt-Faden entgegen. Die Vorschau ist eine Modellrolle
geworden: `data()` liefert den zwischengespeicherten Pfad oder eine leere
Zeichenkette und wartet nie auf Ein-/Ausgabe.

Neu in der Oberfläche: Fortschrittsbalken mit Zahlenangabe (SM-IMP-002; die Zahl
ist das zweite Merkmal neben dem Balken, SM-NFR-009) und eine
Abbruchmöglichkeit (SM-BAT-005).

### 9.2 Nachweis — gemessen an der laufenden Anwendung

`scripts/check-hintergrund.sh` fährt die Anwendung mit einem erzeugten
Prüfbestand und misst die Takte eines Zeitgebers **im Qt-Faden** gegen die
Wanduhrzeit. Bleibt der Faden stehen, fallen Takte aus.

| Lauf | Bestand | Wanduhr | Takte erwartet | Takte gezählt | Ergebnis |
|---|---|---|---|---|---|
| Hintergrundbetrieb | 25.000 Dateien | 12,0 s | 120 | **120** | PASS |
| Hintergrundbetrieb | 8.000 Dateien | 12,0 s | 120 | **120** | PASS |
| **Gegenprobe:** Qt-Faden 4 s blockiert | 8.000 Dateien | 16,0 s | 160 | **120** | **FAIL** |

Der Einlesevorgang über 25.000 Dateien dauerte rund fünf Sekunden. In dieser
Zeit fiel **kein einziger** Takt aus; die Bedienung blieb durchgehend
ansprechbar (AK-01). Nebenher entstanden Vorschauen im Hintergrund.

**Ein Fehler am Messaufbau selbst wurde dabei gefunden und behoben.** Die erste
Fassung zählte nur Takte, ohne sie auf die Wanduhr zu beziehen. Das ist
wertlos: Ein blockierter Faden verzögert **auch** den Zeitgeber, der den Lauf
beendet — die Gegenprobe meldete deshalb zunächst vollzählige 120 von 120 Takten,
obwohl vier Sekunden lang nichts lief. Erst der Bezug auf `Date.now()` macht die
Prüfung durchfallbar. Ohne die Gegenprobe wäre der Mangel nicht aufgefallen.

### 9.3 Prüffälle

Zehn neue Fälle in `kern-services`, ohne Qt lauffähig, weil der Rückkanal ein
Verschluss ist:

- Der Auftrag kehrt in unter 50 ms zurück, während 400 Dateien eingelesen werden.
- Ein Lauf über 1.500 Dateien ist abbrechbar — gemessen greift der Abbruch nach
  der **ersten** Datei.
- Befehle werden vor 200 wartenden Vorschauen bedient.
- Die Vorschauschlange bleibt bei dreifacher Überfüllung innerhalb ihrer Grenze.
- Fünf defekte Dateien halten einen Lauf über zehn gültige nicht an (SM-NFR-005).
- Der Fortschritt läuft monoton und endet bei der Gesamtzahl.

Gesamtstand: **126 bestandene Prüffälle**, 0 fehlgeschlagen. `cargo fmt` sauber,
`cargo clippy --all-targets` ohne Meldung, Projektregeln PASS.

### 9.4 Was offen bleibt

- **SM-IMP-003** (inkrementeller Import), **SM-IMP-004** (Ordnerüberwachung),
  **SM-IMP-005** (Duplikaterkennung) und **SM-IMP-006** (Importvorschau) sind
  nicht Gegenstand dieser Änderung.
- Der **Vorschau-Zwischenspeicher wird noch nicht verworfen**, wenn sich die
  Quelldatei ändert (SM-PRV-003). Der Ablageort ist zudem das Temporärverzeichnis
  und damit nicht dauerhaft, wie SM-PRV-002 es verlangt.
- Die Zeilenmenge wird nach einem Einlesevorgang vollständig neu geladen. Das ist
  richtig, aber grob; ein gezieltes Nachführen gehört zu SM-IMP-003.
- Der Selbsttesthaken liegt in der Oberfläche und ist über `SM_SELBSTTEST`
  abgeschaltet. Sobald ein Prüfrahmen für QML besteht, gehört er dorthin.
