# StitchManager

![StitchManager — Stickdateien. Schnittmuster. Geordnet.](Design/stitchmanager-logo.svg)

Eine freie Desktop-Anwendung zum Verwalten, Finden, Anzeigen, Konvertieren und
Drucken von Stickdateien und Schnittmustern.

[Installation](#installation) ·
[Version 1.0](#funktionsumfang-von-version-10) ·
[Mockup](Design/stitchmanager-mockup.html) ·
[Lastenheft](Requirements/StitchManager_Lastenheft.md)

---

## Was macht StitchManager?

StitchManager bringt verstreute Stickdateien und PDF-Schnittmuster in eine
übersichtliche, lokale Bibliothek. Die Anwendung liest vorhandene
Ordnerstrukturen ein, erzeugt Vorschaubilder direkt aus den Stichdaten und macht
Muster über Suche, Filter und technische Werte schnell auffindbar. Metadaten und
Schlagworte lassen sich ergänzen, Dateien in passende Maschinenformate
konvertieren und sicher auf Wechseldatenträger übertragen.

Schnittmuster und Nähanleitungen können ohne externes Anzeigeprogramm geöffnet
und maßhaltig gedruckt werden. Dabei gilt: Originaldateien werden beim Import
weder verschoben noch verändert. Bibliothek und Metadaten bleiben lokal, die
Kernfunktionen arbeiten vollständig offline und StitchManager übermittelt keine
Telemetrie.

| Eigenschaft | Ausrichtung |
|---|---|
| Zielplattformen | macOS, Windows und Linux |
| Stickformate zum Lesen | PES, DST, JEF und VP3 |
| Dokumente | PDF-Schnittmuster und Nähanleitungen |
| Datenhaltung | lokal in SQLite, ohne Server oder Cloud-Zwang |
| Oberfläche | Qt 6 im warmen Kreuznaht-Design, Hell- und Dunkelmodus |
| Lizenz | GPL-3.0-only |

## Funktionsumfang von Version 1.0

> [!IMPORTANT]
> Version 1.0 beschreibt den verbindlich geplanten Auslieferungsumfang. Der
> aktuelle Quellbaum trägt noch die Entwicklungsfassung **0.1.0**; fertige
> Installationspakete stehen noch nicht bereit.

| Planungsstand | Umfang |
|---|---|
| Anforderungen im Lastenheft | 225 |
| Für Version 1.0 verplant | 140 |
| Arbeitspakete | 23 (AP-00 bis AP-22) |
| Offene Punkte | 21 |

### Aktueller Entwicklungsstand

Stand 29.08.2026 ist der Konsolidierungssprint abgeschlossen. Der vollständige
Workspace-Regellauf besteht mit 185 Prüffällen; 14 Fälle benötigen den externen
Prüfbestand oder werden nur als gezielte Messfälle gestartet. In der
Rückverfolgbarkeitsmatrix sind 8 von 140 verplanten Anforderungen durch einen
vollständigen ausgeführten Nachweis belegt, 132 bleiben offen. Diese Matrixzahl
ist ein Nachweisstand und kein prozentualer Umsetzungsgrad.

Für AP-13 ist der kernseitige Detail-Lesepfad vorhanden: Datenbank, Fassade und
Hintergrunddienst liefern Metadaten, technische Werte, Fehlerzustand,
Schlagworte und Garnfarben mit unterscheidbaren Lade- und Fehlerantworten. Die
sichtbare QML-Detailansicht und die Metadatenbearbeitung sind noch nicht
umgesetzt. Der vollständige Stand und die Folgeplanung stehen in der
[Sprintplanung vom 29.08.2026](Analysis/20260829_01_sprintplanung.md#75-sp-05--kernseitiger-vertikalschnitt-für-ap-13).

### Bibliothek und Import

- Eine oder mehrere lokale Bibliothekswurzeln einbinden und ihre
  Ordnerhierarchie abbilden
- Ordner anlegen, umbenennen, verschieben und löschen, ohne Originaldateien
  beim Import zu verändern
- Bis zu 100.000 Einträge verwalten
- Verzeichnisse rekursiv und inkrementell im Hintergrund einlesen
- Neue, geänderte, entfernte, doppelte oder beschädigte Dateien erkennen
- Laufende Vorgänge mit Fortschritt anzeigen und kontrolliert abbrechen

### Formate, Vorschau und Metadaten

- PES, DST, JEF und VP3 anhand ihres Inhalts sicher erkennen und lesen
- Stichanzahl, Abmessungen, Farbanzahl und Farbwechsel auslesen
- Brother- und Janome-Garnfarben einschließlich Garnname, Nummer und
  Stichanteil zuordnen
- Farbige Vorschauen aus Stichdaten erzeugen, dauerhaft zwischenspeichern und
  bei Dateiänderungen erneuern
- Namen, Thema, Beschreibung, Notizen und freie Schlagworte pflegen
- PDF-Schnittmuster, Nähanleitungen, Titelbilder, Maßtabellen und Stoffbedarf
  typisiert einem Eintrag zuordnen

### Suchen, Filtern und Stapelverarbeitung

- Bibliotheksweite Volltextsuche über Namen, Themen, Beschreibungen und
  Schlagworte
- Filter nach Schlagwort, Format, Größe, Stichanzahl, Farbanzahl und Quelle
  kombinieren und einzeln wieder entfernen
- Nach Name, Datum, Größe, Stichanzahl oder Relevanz sortieren
- Mehrere Einträge auswählen, nach Namensmustern umbenennen und Änderungen vor
  der Ausführung prüfen
- Abgebrochene Stapelvorgänge in einem nachvollziehbaren Zustand hinterlassen

### Konvertieren, Exportieren und Drucken

- Verfügbare Zielformate je Datei anzeigen und Stickdateien konvertieren
- Einzelne Dateien oder eine Mehrfachauswahl auf Wechseldatenträger exportieren
- Vor dem Kopieren Schreibrechte und freien Speicher prüfen
- Namenskonflikte durch Überschreiben, Umbenennen oder Abbrechen lösen
- Schnittmuster direkt in StitchManager anzeigen und drucken
- Drucker, Papierformat, Ausrichtung und Seitenbereich wählen
- A4 und US Letter ohne unbeabsichtigte Skalierung maßhaltig drucken
  (Prüfmaß 100 mm ± 0,5 mm)

### Lokale Analyse, Bedienung und Sicherheit

- Auf Wunsch lokal Metadatenvorschläge aus einem Vorschaubild erzeugen
- Jedes vorgeschlagene Feld einzeln übernehmen oder verwerfen und maschinelle
  Werte bis zur Bestätigung eindeutig kennzeichnen
- Deutsche, vollständig per Tastatur bedienbare Oberfläche
- Hell- und Dunkelmodus mit Übernahme der Systemeinstellung
- Fensterzustand und Spaltenbreiten über Sitzungen hinweg erhalten
- Verständliche Fehlermeldungen, abgesicherte Dateipfade und eine sicherbare
  lokale Datenhaltung

Version 1.0 enthält bewusst kein Digitizing, keine direkte Maschinensteuerung,
keine Cloud-Synchronisation, keine Nutzerverwaltung und keine mobilen Apps.
Projekte, intelligente Ordner, Sammlungen, Favoriten sowie die gewerblichen
Bereiche Fertigung, Beschaffung, Qualitätsprüfung, Zeiterfassung und Kalkulation
sind für spätere Versionen vorgesehen.

## Design

StitchManager folgt dem Kreuznaht-Design aus [`Design/`](Design/): warme Creme-
und Sandtöne im Hellmodus, Espresso statt neutralem Grau im Dunkelmodus und
Terracotta als Leitfarbe. Nähte gliedern die Oberfläche; der Kreuzstich dient als
wiederkehrende Auswahl- und Markenform. Das Hauptfenster bleibt dreispaltig:
Navigation links, bildgeführte Musterauswahl in der Mitte und Details rechts.

Das [interaktive HTML-Mockup](Design/stitchmanager-mockup.html) lässt sich ohne
Installation im Browser öffnen. Die verbindlichen Farben, Abstände und Zustände
stehen in der
[Design-Beschreibung](Design/StitchManager_Design_Beschreibung.md).

## Installation

### Geplante Pakete für Version 1.0

Sobald Version 1.0 veröffentlicht ist, werden die Pakete unter
[GitHub Releases](https://github.com/carpeasrael/StitchManager/releases)
bereitgestellt.

| Plattform | Paket | Installation |
|---|---|---|
| macOS | signiertes und notarisiertes Universal-`.dmg` für Apple Silicon und Intel | DMG öffnen und StitchManager in „Programme“ ziehen |
| Windows | MSI- oder NSIS-Installationspaket | Installer öffnen und im Benutzerkontext ausführen; Administratorrechte sind nicht vorgesehen |
| Linux | Flatpak auf `org.kde.Platform` | Paket über die Softwareverwaltung oder mit Flatpak im Benutzerkontext installieren |

Derzeit existieren diese Auslieferungspakete noch nicht. Die Anwendung kann aus
dem Quellcode gebaut und gestartet werden.

### Voraussetzungen für den Bau aus dem Quellcode

- Git
- Rust **1.82 oder neuer** mit Cargo
- Qt **6** mit Qt Quick und Qt Quick Controls 2
- ein C++-Compiler für die jeweilige Plattform
- `qmake` aus derselben Qt-Installation, die für den Bau verwendet wird

Nach dem Klonen wird die Anwendung auf jeder Plattform gleich gebaut:

```bash
git clone https://github.com/carpeasrael/StitchManager.git
cd StitchManager
cargo build --release --locked
```

Das Programm liegt danach unter `target/release/stitchmanager` beziehungsweise
unter Windows unter `target\release\stitchmanager.exe`.

### macOS

1. Die Xcode-Kommandozeilenwerkzeuge installieren:

   ```bash
   xcode-select --install
   ```

2. [Rust über `rustup`](https://www.rust-lang.org/tools/install) installieren.
3. Qt 6 installieren, zum Beispiel mit Homebrew:

   ```bash
   brew install qt
   ```

4. Falls `qmake` nicht automatisch gefunden wird, den Pfad für den Bau setzen:

   ```bash
   export QMAKE="$(brew --prefix qt)/bin/qmake"
   cargo build --release --locked
   ./target/release/stitchmanager
   ```

### Windows

1. Die **Visual Studio 2022 Build Tools** mit der Arbeitslast
   „Desktopentwicklung mit C++“ installieren.
2. [Rust über `rustup-init.exe`](https://www.rust-lang.org/tools/install) mit
   dem vorgegebenen MSVC-Werkzeugsatz installieren.
3. Über den
   [Qt Online Installer](https://doc.qt.io/qt-6/get-and-install-qt.html) eine
   freie Qt-6-Desktopinstallation für **MSVC 2022 64-bit** mit Qt Quick und Qt
   Quick Controls 2 einrichten.
4. In der „x64 Native Tools Command Prompt for VS 2022“ bauen. Den Beispielpfad
   dabei an die installierte Qt-Fassung anpassen:

   ```powershell
   $env:QMAKE = "C:\Qt\6.x.x\msvc2022_64\bin\qmake.exe"
   cargo build --release --locked
   .\target\release\stitchmanager.exe
   ```

### Linux

Unter Debian und Ubuntu lassen sich die benötigten Baupakete aus den
Paketquellen installieren:

```bash
sudo apt update
sudo apt install build-essential qt6-base-dev qt6-declarative-dev
```

Danach [Rust über `rustup`](https://www.rust-lang.org/tools/install) installieren
und bauen. Da Debian und Ubuntu das Qt-6-Werkzeug als `qmake6` bereitstellen,
wird es ausdrücklich angegeben:

```bash
export QMAKE="$(command -v qmake6)"
cargo build --release --locked
./target/release/stitchmanager
```

Bei anderen Distributionen werden die entsprechenden Entwicklungspakete für
Qt Base, Qt Declarative/Quick, Qt Quick Controls 2, OpenGL und einen
C++-Compiler benötigt. Alternativ kann die freie Qt-Fassung über den
[Qt Online Installer](https://doc.qt.io/qt-6/get-and-install-qt.html)
installiert werden.

## Erste Schritte

1. Über **Bibliothek wählen…** das oberste Verzeichnis des Musterbestands
   auswählen.
2. **Bestand einlesen** starten. Der Import läuft im Hintergrund und verändert
   die Quelldateien nicht.
3. Über das Suchfeld, die Formatchips und die Sortierung den Bestand eingrenzen.
4. Die Kacheln zeigen Vorschau, Format, Name und Abmessungen. Die sichtbare
   Detailansicht mit Stichzahl, Garnfarben und bearbeitbaren Metadaten folgt im
   nächsten AP-13-Umsetzungsschnitt.

Die Anwendung speichert Datenbank und Vorschauzwischenspeicher im
plattformspezifischen Anwendungsverzeichnis. Stickdateien bleiben an ihrem
ursprünglichen Speicherort.

## Technik und Entwicklung

StitchManager besteht aus einem Rust-Kern und einer Qt-6-Oberfläche in QML. Die
Oberfläche sendet Aufträge an den Hintergrunddienst; dieser greift ausschließlich
über die schmale Fassade auf Parser, Datenhaltung und Vorschauerzeugung zu.
Import, Suche, Detailabruf und Vorschauerzeugung laufen außerhalb des
Oberflächenfadens; große Bibliotheken werden ausschnittsweise und virtualisiert
dargestellt.

```text
Qt 6 / QML
    │
kern-services           Hintergrundvorgänge und Rückkanal
    │
kern-fassade            einzige Zugriffsschicht auf den Kern
    ├── kern-parsers    PES, DST, JEF, VP3
    ├── kern-render     Vorschauerzeugung und Zwischenspeicher
    ├── kern-security   Pfadprüfung
    └── kern-db         SQLite und Migrationen
```

Tests und Projektprüfungen:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets

npm ci
bash scripts/check-qml.sh
bash scripts/check-docs.sh
bash scripts/check-plan.sh
bash scripts/check-projektregeln.sh
```

Wer am Repository mitarbeitet, installiert zusätzlich einmalig die lokalen
Prüfhaken:

```bash
bash scripts/install-hooks.sh
```

## Dokumentation

- [Lastenheft](Requirements/StitchManager_Lastenheft.md) — was StitchManager
  leisten muss
- [Design-Beschreibung](Design/StitchManager_Design_Beschreibung.md) — wie die
  Oberfläche aussieht und sich verhält
- [Technikentscheidung](TechStack/StitchManager_TechStack.md) — womit die
  Anwendung gebaut und ausgeliefert wird
- [Implementierungsplan](Implementation/StitchManager_Implementierungsplan.md) —
  Umfang und Nachweise von Version 1.0

## Lizenz

StitchManager ist freie Software unter der
[GNU General Public License v3.0](LICENSE). Qt 6 wird unter LGPL-3 dynamisch
gebunden; sämtliche mitgelieferten Komponenten müssen quelloffen und mit der
Projektlizenz vereinbar sein.
