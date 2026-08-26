# StitchManager — Tech-Stack

**Kennung:** TEC-STM-001
**Version:** 2.2
**Datum:** 23.08.2026
**Führendes Dokument:** URS-STM-001 (Lastenheft) v1.3
**Mitgeltend:** DES-STM-001 (Design-Beschreibung) v1.3 · ANA-STM-001 (Konsolidierungsanalyse) v1.1
**Status:** Empfehlung zur Entscheidung

> **Änderung gegenüber Version 1.0.** Die erste Fassung empfahl, StitchManager-3 unverändert
> auf Tauri v2 fortzuführen. Diese Empfehlung stand unter der Annahme, dass die
> Anzeigekomponente des Betriebssystems eine hinnehmbare Abhängigkeit ist. Mit der
> Festlegung **ausschließlich quelloffener Komponenten** (SM-OSS-001 bis SM-OSS-003) trifft
> diese Annahme nicht mehr zu. Abschnitt 2 begründet die Neubewertung.

---

## 0. Verhältnis zum Lastenheft

Das Lastenheft URS-STM-001 beschreibt StitchManager als **neu zu entwickelnde Anwendung** und
nimmt bewusst keinen Bezug auf Vorgängerstände. Dieses Dokument entscheidet, **womit** diese
Anforderungen erfüllt werden — und kommt zu dem Ergebnis, einen vorhandenen Rust-Kern als
Baustein wiederzuverwenden, statt ihn nachzubauen.

**Das ist kein Widerspruch, sondern die vorgesehene Arbeitsteilung:**

| Dokument | Fragestellung | Bezug auf Vorgängerstände |
|---|---|---|
| URS-STM-001 | Was muss das System leisten? | keiner — bewusst herkunftsfrei formuliert |
| TEC-STM-001 (dieses) | Womit wird es gebaut? | ja — Wiederverwendung ist eine Technologieentscheidung |
| ANA-STM-001 | Woher stammen die Anforderungen? | vollständig — reine Herleitung |

Die Anforderungen gelten unverändert für den wiederverwendeten Kern. Er genießt **keinen
Bestandsschutz**: Jede Anforderung des Lastenhefts ist am Ergebnis nachzuweisen, gleich ob der
zugehörige Code neu geschrieben oder übernommen wurde. Übernommener Code, der eine
Muss-Anforderung nicht erfüllt, wird angepasst wie neuer auch.

> **Zu entscheiden:** Falls „neue Applikation" im Sinne des Auftraggebers eine Neuentwicklung
> **ohne** Wiederverwendung meint, ist dieses Dokument hinfällig und neu zu erstellen. Der
> Unterschied beträgt rund 31.500 Zeilen geprüften Code. Geführt als **OP-13** im Lastenheft.

---

## 1. Was von Version 1.0 bestehen bleibt

Die Vorgabe verändert die Oberflächenschicht — nicht den Rest. Unverändert gilt:

- **StitchManager-3 bleibt die Codebasis.** StitchMan (Python/PySide6) und StichMan2
  (Rust/iced) werden eingestellt.
- **Der Rust-Kern bleibt vollständig erhalten.** Parser, Datenbankschicht, Vorschauerzeugung,
  Dateiüberwachung, Datenträgererkennung, Berichte, Pfadprüfung — rund 31.500 Zeilen mit 197
  Tests und Fuzzing-Targets. Das ist der wertvollste Teil des Bestands, und er ist von der
  Vorgabe nicht berührt.
- **SQLite mit FTS5 bleibt.** Gemeinfrei, kein zweiter Suchindex.
- **Die Formatarbeit bleibt.** Parser aus R3, ergänzt um EXP und XXX aus R2.

Was ersetzt werden muss, ist die Oberflächenschicht: 78 TypeScript-Module, davon rund 35
Komponenten. Das ist der Preis der Vorgabe, und er ist erheblich — Abschnitt 6 beziffert ihn.

---

## 2. Neubewertung der Oberflächentechnologie

### 2.1 Warum Tauri ausfällt

Tauri nutzt auf jeder Plattform die Anzeigekomponente des Betriebssystems:

| Plattform | Komponente | Quelloffen |
|---|---|---|
| Linux | WebKitGTK | ja (LGPL / BSD) |
| macOS | WKWebView | nein — WebKit ist offen, das ausgelieferte Systemframework nicht |
| Windows | WebView2 | **nein** — proprietäre Microsoft-Laufzeit, eigene Weiterverteilungsbedingungen |

WebView2 ist dabei der harte Fall: Es ist keine Systembibliothek im engeren Sinn, sondern
eine getrennt verteilte Laufzeit, deren Installation die Anwendung voraussetzt und deren
Nachinstallation der Installer typischerweise anstößt. Das kollidiert unmittelbar mit
SM-OSS-002 und SM-OSS-003.

**Eine Teillösung gibt es nicht.** Tauri unter Linux und macOS beizubehalten und nur unter
Windows etwas anderes einzusetzen, hieße zwei Oberflächen zu bauen und dauerhaft
gleichzuhalten. Der Aufwand wäre höher als eine einheitliche Umstellung, und jede
Funktionsergänzung fiele doppelt an. Die Vorgabe wirkt daher auf alle drei Plattformen.

### 2.2 Die Kandidaten

| Kandidat | Lizenz | PDF-Anzeige | Maßstabsgetreuer Druck | Bewertung |
|---|---|---|---|---|
| **Qt 6** | LGPL-3 | Qt PDF (auf pdfium, BSD) | QPrinter mit physikalischen Einheiten | **Empfehlung.** Einziger Kandidat, der beide harten Anforderungen mitbringt statt sie zu verlangen. |
| GTK 4 | LGPL | Poppler (GPL-2/3, verträglich) | GtkPrintOperation | Tragfähig. Unter Linux erstklassig, unter Windows und macOS bei Erscheinungsbild und Paketierung deutlich schwächer. |
| Electron / Chromium | MIT / BSD | pdf.js | **löst das Druckproblem nicht** | Quelloffen nur mit codecfreiem Bau. Behält den TypeScript-Bestand, erbt aber genau die Druckschwäche, die schon in Version 1.0 der wunde Punkt war. Dazu über 100 MB je Paket und eine eigene Chromium-Sicherheitspflege. |
| Avalonia (.NET) | MIT | Fremdbibliothek nötig | schwach | Vollständig quelloffen, aber ohne Bezug zum Rust-Kern und mit schwacher Druckunterstützung. |
| Compose Multiplatform | Apache-2.0 | PDFBox (Apache-2.0) | Java Print Service | Vollständig quelloffen und beim Druck brauchbar. Bindet aber eine dritte Sprache ein und trennt die Oberfläche vom Rust-Kern. |
| Slint | GPL-3 (u. a.) | — | — | Für ein GPL-3-Projekt lizenzrechtlich nutzbar, aber ohne PDF- und Druckunterlage. |
| iced / egui | MIT / Apache | — | — | Reines Rust, aber PDF-Anzeige und Druckaufbereitung müssten vollständig selbst entstehen. Das ist der Stack von R2, und er ist an diesem Umfang bereits einmal gescheitert. |

### 2.3 Die Empfehlung — und warum die Vorgabe hier etwas zurückgibt

> **Qt 6 unter LGPL-3 als Oberfläche, Rust-Kern unverändert, Anbindung über cxx-qt.**

Der bemerkenswerte Punkt: Die teure Vorgabe löst nebenbei das teuerste offene Problem.

In Version 1.0 war der maßstabsgetreue Druck (SM-PRN-006, SM-PRN-007) die größte
technische Unsicherheit — gerade **weil** eine Webansicht den Druckmaßstab nicht
zuverlässig kontrolliert. Ich hatte deshalb einen Umweg über selbst erzeugte PDF-Dateien
vorgeschlagen. Qt braucht diesen Umweg nicht: Seitenformat, Ränder und Auflösung werden in
physikalischen Einheiten gesetzt, und das Druckergebnis ist prüfbar statt zu hoffen, dass
die Webansicht nicht doch skaliert. Dieselbe Grundlage rendert PDF-Seiten für die Vorschau.

Damit trägt die Umstellung nicht nur Kosten, sondern beseitigt das Risiko, das das gesamte
Vorhaben am stärksten belastet hat. Das rechtfertigt sie nicht allein — aber es verändert
die Rechnung spürbar.

**Anbindung: zwei Wege.**

| Weg | Aufbau | Bewertung |
|---|---|---|
| **A — cxx-qt** (empfohlen) | Rust bindet direkt an Qt, Oberfläche in QML | Der Rust-Kern bleibt Hauptprogramm, keine dritte Sprache. cxx-qt steht unter MIT/Apache. Risiko: die Bindung ist jünger als Qt selbst, und QML ist für dichte Tabellen weniger erprobt als Qt Widgets. |
| **B — Python-Schale über PySide6** | Rust-Kern als Erweiterungsmodul über PyO3, Oberfläche in Qt Widgets | Reifere Bausteine, Qt Widgets sind für dichte Datenoberflächen die bessere Wahl, PySide6 steht unter LGPL. Preis: Python in der Auslieferung, größere Pakete, langsamerer Start. Und: Das ist strukturell der Stack von R1 — mit dem Unterschied, dass die Fachlogik diesmal in Rust bleibt statt im Oberflächencode. |

**Vor der Festlegung:** Beide Wege an einem Prototyp gegeneinander prüfen — eine
virtualisierte Liste mit 100.000 Einträgen (SM-LIB-009, SM-PRV-007, AK-01) und ein
Testdruck mit einem Kalibrierquadrat von 100 mm Kantenlänge, nachgemessen am körperlichen
Ausdruck auf A4 und US Letter. Zulässige Abweichung nach **SM-PRN-006: ± 0,5 mm**; damit ist
der Prototyp zugleich der Vorabnachweis für **AK-06**. Das sind die zwei Punkte, an denen
sich die Wege real unterscheiden; alles andere lässt sich aus der Dokumentation ableiten.

### 2.4 Was die LGPL verlangt

Qt unter LGPL-3 ist zulässig und mit der GPL-3-Projektlizenz verträglich, verlangt aber
Disziplin bei der Auslieferung (SM-OSS-006):

- **Dynamisch binden.** Qt-Bibliotheken werden als eigene Dateien mitgeliefert, nicht ins
  Programm einkompiliert. Statisches Binden wäre nur unter zusätzlichen Auflagen zulässig.
- **Austausch ermöglichen.** Der Empfänger muss eine eigene Qt-Fassung einsetzen können.
  Bei dynamischer Bindung ist das erfüllt, sofern keine Prüfung den Austausch unterbindet.
- **Lizenztext und Quellenhinweis mitliefern**, im Programm erreichbar (SM-OSS-010).
- **Keine Nutzungsbeschränkung** über die Qt-Lizenz hinaus.

Praktisch heißt das: Standardauslieferung ohne statisches Binden — der Weg, den die
Qt-Werkzeuge ohnehin gehen.

---

## 3. Zielarchitektur

```text
┌──────────────────────────────────────────────────────────────┐
│  Oberfläche  —  Qt 6 (LGPL-3)                                │
│  ┌────────────┬──────────────────┬────────────────────────┐  │
│  │ Navigation │ Ergebnisliste    │ Detail / Metadaten     │  │
│  │            │ (virtualisiert)  │                        │  │
│  └────────────┴──────────────────┴────────────────────────┘  │
│  Qt PDF      → Anzeige von Schnittmustern und Anleitungen    │
│  QPrinter    → maßstabsgetreuer Druck, Kachelung             │
│  Kreuznaht-Gestaltung als Qt-Stilvorlage, hell / dunkel      │
└───────────────────────────┬──────────────────────────────────┘
                            │  cxx-qt  (MIT / Apache-2.0)
┌───────────────────────────┴──────────────────────────────────┐
│  Kern  —  Rust  (unverändert aus StitchManager-3)            │
│                                                              │
│  parsers/    PES · DST · JEF · VP3 · EXP* · XXX*             │
│  writers/    PES · DST  (Ziel: + JEF · EXP · VP3)            │
│  render/     Stichgrafik, Vorschau-Zwischenspeicher          │
│  db/         SQLite (WAL), additive Migrationen, FTS5        │
│  services/   Dateiüberwachung, Datenträgererkennung,         │
│              KI-Anbindung, Berichte, Sicherung               │
│  security/   Pfadprüfung, Eingrenzung, Bereinigung           │
└──────────────────────────────────────────────────────────────┘
   * = aus StichMan2 zu übernehmen
```

---

## 4. Komponentenwahl

| Schicht | Wahl | Lizenz |
|---|---|---|
| Oberfläche | Qt 6 | LGPL-3 |
| Anbindung | cxx-qt | MIT / Apache-2.0 |
| Kernsprache | Rust, Edition 2021 → **2024 anheben** | Apache-2.0 / MIT |
| Datenhaltung | SQLite über `rusqlite`, WAL-Modus | gemeinfrei / MIT |
| Volltextsuche | SQLite FTS5 | gemeinfrei |
| Stickformate | eigene Rust-Parser | Projektlizenz |
| PDF anzeigen | Qt PDF | LGPL-3, auf pdfium (BSD-3) |
| PDF erzeugen | `printpdf`, `lopdf` | MIT |
| Druck | QPrinter / QPageLayout | LGPL-3 |
| Grafik | `image`, `tiny-skia` | MIT / Apache-2.0, BSD-3 |
| Schlüsselablage | `keyring` | MIT / Apache-2.0 |
| Schriften | Josefin Sans, Lato, IBM Plex Mono | SIL Open Font License |
| Werkzeuge | `cargo test`, `cargo-fuzz`, `cargo-deny`, CodeQL | offen |

**Entfällt gegenüber Version 1.0:** Tauri und seine Erweiterungen, Vite, TypeScript, Biome,
Vitest, dompurify, marked, pdfjs-dist. Alle waren quelloffen — sie entfallen nicht aus
Lizenzgründen, sondern weil die Web-Oberfläche entfällt.

**Was dadurch nebenbei wegfällt:** die Klasse der Einschleusungsangriffe über HTML. Die
Bereinigung von Fremdtext vor der Anzeige, die in R3 an mehreren Stellen nötig war, entfällt
mit dem HTML-Rendering. Qt-Textelemente können ebenfalls Auszeichnungen darstellen — dort
ist Nur-Text zu erzwingen, wo Fremddaten angezeigt werden. Die Anforderung SM-SEC-008 bleibt
also bestehen, ihre Umsetzung wird nur einfacher.

---

## 5. Auslieferung je Plattform

| | macOS | Windows | Linux |
|---|---|---|---|
| Format | `.dmg`, Universal-Binary | MSI oder NSIS | Flatpak |
| Laufzeitumgebung | — | — | **org.kde.Platform** (statt org.gnome.Platform) |
| Qt-Auslieferung | `macdeployqt` | `windeployqt` | aus der Laufzeitumgebung |
| Signatur | Developer ID + Notarisierung | Authenticode | Flathub-Signatur |

**Änderung gegenüber Version 1.0:** Die Flatpak-Laufzeitumgebung wechselt von GNOME auf KDE,
weil dort Qt 6 bereits enthalten ist. Das verkleinert das Paket und entlastet die
Sicherheitspflege — Qt-Aktualisierungen kommen dann über die Laufzeitumgebung.

**Unverändert gültig bleiben die vier Flatpak-Befunde aus Version 1.0:**

| Punkt | Ist | Soll |
|---|---|---|
| Wechseldatenträger | nur lesend | Schreibzugriff — sonst ist der USB-Export unter Linux funktionslos |
| Schlüsselspeicher | keine Berechtigung | Zugriff auf den Geheimnisdienst — sonst schlägt die Schlüsselablage fehl |
| Benutzerverzeichnis | vollständiger Zugriff | Datei-Portale |
| Laufzeitumgebung | veraltet | aktuelle KDE-Generation |

### Plattformübergreifend

Aus dem Lastenheft ergeben sich drei Vorgaben, die für alle drei Plattformen gleichermaßen gelten:

| Anforderung | Umsetzung |
|---|---|
| SM-PLT-007 — gleiche Versionsangabe in allen Projekt- und Paketdateien | Automatisierte Prüfung in der CI; eine Notiz in der Bauanleitung genügt nicht |
| SM-PLT-009 — reproduzierbare Bauläufe | Abhängigkeitssperrdateien in der Versionsverwaltung, Bauumgebung festgeschrieben |
| SM-PLT-010 — keine Administratorrechte für Installation und Betrieb | Betrifft vor allem den Windows-Installer; Installation im Benutzerkontext vorsehen |

Ergänzend: Die Paketkonfiguration sollte nur die drei geforderten Ausgabeformate erzeugen.
Alle verfügbaren Formate zu bauen verlängert Bauzeit und Prüfaufwand ohne Nutzen.

### Zur Signatur (offener Punkt OP-04; Anforderungen SM-PLT-002, SM-SEC-012)

Die Open-Source-Vorgabe ändert hieran nichts: Für Apple Developer ID und Authenticode gibt
es keine quelloffene Entsprechung. Es sind kostenpflichtige Dienste, keine Komponenten.
Zwei Wege stehen offen — Signatur als Infrastruktur akzeptieren, oder unsigniert ausliefern
und die Warnungen in Kauf nehmen. Unter macOS bedeutet unsigniert, dass Nutzer die
Anwendung beim ersten Start manuell freigeben müssen; unter Windows erscheint eine
Warnung des Reputationsdienstes. Für Linux stellt sich die Frage nicht.

---

## 6. Was die Vorgabe kostet

Ich will das nicht beschönigen — es ist die teuerste Einzelentscheidung dieses Vorhabens.

| Betroffen | Umfang |
|---|---|
| Rust-Kern | **unverändert** — rund 31.500 Zeilen, 197 Tests, Fuzzing bleiben bestehen |
| Datenbank und Migrationen | **unverändert** — 47 Tabellen, Schema v27 |
| Oberfläche | **Neubau** — rund 35 Komponenten, dreispaltiges Hauptfenster, Dialoge |
| Gestaltung | Kreuznaht-Vorlage als Qt-Stil statt als CSS; das Mockup bleibt als Vorlage gültig |
| Paketierung | Flatpak-Wechsel auf KDE, Qt-Auslieferungswerkzeuge je Plattform |
| Drucken | **Aufwand sinkt** — die Unsicherheit aus Version 1.0 entfällt |
| Frontend-Prüfwerkzeuge | entfallen; Qt-Tests treten an ihre Stelle |

Grob eingeordnet: Etwa zwei Drittel des Bestands bleiben, ein Drittel wird neu gebaut. Der
Aufwand liegt in der Größenordnung mehrerer Monate konzentrierter Arbeit — eine belastbare
Schätzung setzt aber die Entscheidung zwischen Weg A und Weg B voraus.

**Ein Hinweis, den ich schuldig bin:** Wenn die Vorgabe aus einer Lizenz- oder
Beschaffungsrichtlinie stammt, ist sie zu befolgen und die Rechnung geht auf, weil der
Druckvorteil einen Teil zurückgibt. Wenn sie dagegen aus Prüfbarkeit oder Vertrauen in die
Lieferkette motiviert ist, wäre auch ein engerer Weg denkbar: Alle **mitgelieferten**
Komponenten quelloffen halten, die Anzeigekomponente des Betriebssystems aber als Teil der
Plattform behandeln. Das ist die Auslegung, unter der die meisten quelloffenen Anwendungen
auf proprietären Betriebssystemen laufen, und sie hätte den Bestand vollständig erhalten.
Die Entscheidung liegt bei dir — ich wollte nur, dass die Alternative benannt ist und nicht
stillschweigend verworfen wird.

---

## 7. Lizenzprüfung — Ergebnis und Absicherung

### 7.1 Geprüfter Bestand (Stand 23.08.2026)

| Bereich | Ergebnis |
|---|---|
| 31 direkte Rust-Abhängigkeiten | ausnahmslos MIT, Apache-2.0, BSD-3-Clause, CC0 oder Unlicense |
| 200 npm-Pakete (entfallen mit der Umstellung) | MIT 120, Apache-2.0 25, Dual 25, MPL-2.0 13, BSD 7, ISC 5, weitere 5 — kein Paket ohne Lizenzangabe |
| Schriften | SIL Open Font License |
| SQLite, Flatpak-Laufzeitumgebung | gemeinfrei bzw. quelloffen |
| Verträglichkeit mit GPL-3.0 | für alle gefundenen Lizenzen gegeben; Apache-2.0 ist mit GPL-3 verträglich, MPL-2.0 über die eigene Verträglichkeitsklausel |

### 7.2 Offene Lizenzpunkte

> Die Kennungen B-09 bis B-13 stammen aus **ANA-STM-001** (Konsolidierungsanalyse). Es sind
> Befunde am Altbestand, keine Anforderungen. Soweit sie fortwirken, sind sie im Lastenheft
> als Anforderung geführt — die Lizenzeindeutigkeit etwa als SM-OSS-005.

| Nr | Punkt |
|---|---|
| B-09 | **StichMan2 ist widersprüchlich lizenziert** — Projektdatei MIT, Lizenzdatei GPL-3.0. Vor Übernahme der EXP-/XXX-Parser aufzulösen. |
| B-10 | **StitchMan hat keine Lizenzdatei** — vor Übernahme des Sidecar-Formats nachzutragen. |
| B-11 | GPL-3.0 und Mac App Store sind unvereinbar. Direktvertrieb nicht betroffen. |
| B-12 | MPL-2.0 bei dreizehn npm-Paketen — entfällt mit der Umstellung. |
| B-13 | Signaturzertifikate bleiben proprietäre Dienste, siehe OP-04. |
| — | **GPL-3.0 und LGPL-3.0 im selben Werk** sind ausdrücklich verträglich; die LGPL-Auflagen aus Abschnitt 2.4 sind einzuhalten. |
| — | **Lokale KI-Modelle:** Ollama steht unter MIT, die Modelle nicht zwangsläufig. Als Vorgabewert ein Modell mit tatsächlich freier Gewichtslizenz setzen (SM-OSS-012). |

### 7.3 Absicherung in der CI

Einmalige Prüfungen veralten. Empfohlen wird:

1. **`cargo-deny`** mit Positivliste erlaubter Lizenzen. Der Bau bricht ab, sobald eine neue
   Abhängigkeit etwas Unpassendes einschleppt (SM-OSS-009).
2. **Stückliste im CycloneDX-Format** je Veröffentlichung, als Artefakt abgelegt
   (SM-OSS-008). Das erfüllt zugleich die Nachweispflichten der GPL.
3. **Lizenzanzeige im Programm** aus derselben Stückliste erzeugen, statt sie zu pflegen
   (SM-OSS-010).
4. **Herkunftsprüfung für vorgebaute Binärpakete** — Qt aus der Laufzeitumgebung bzw. aus
   offiziellen Quellen, keine unbelegten Drittpakete (SM-OSS-011).

Aufwand: etwa ein halber Tag. Danach ist die Vorgabe nicht mehr eine Aussage im Dokument,
sondern eine Bedingung, die der Bau erzwingt.

---

## 8. Vorgehen

| Schritt | Inhalt | Ergebnis |
|---|---|---|
| **1** | Lizenzwiderspruch in StichMan2 auflösen (B-09), Lizenz in StitchMan nachtragen (B-10) | Saubere Ausgangslage für die Übernahme |
| **2** | `cargo-deny`, Stückliste und Lizenzanzeige einrichten | Vorgabe wird erzwungen statt behauptet |
| **3** | Prototyp: Weg A gegen Weg B, geprüft an 100.000 Einträgen (AK-01) und einem Testdruck mit ± 0,5 mm Toleranz (AK-06) | Belegte Entscheidung statt Annahme |
| **4** | Rust-Kern von der Tauri-Befehlsschicht entkoppeln, saubere Schnittstelle ziehen | Kern wird oberflächenunabhängig |
| **5** | Flatpak auf KDE-Laufzeitumgebung umstellen, vier Befunde beheben | Linux-Auslieferung funktionsfähig |
| **6** | Hauptfenster in Qt neu bauen, Kreuznaht-Gestaltung einziehen | Oberfläche auf dem Stand von R3 |
| **7** | EXP- und XXX-Parser sowie Konverter aus R2 übernehmen, Fuzzing ergänzen | Formatabdeckung wie R2 |
| **8** | PDF-, Anleitungs- und Druckfunktion umsetzen | Schnittmusterverwaltung nutzbar |
| **9** | Import-Regelwerk und Duplikaterkennung aus R2 übernehmen | Massenimport auf R2-Niveau |
| **10** | Signaturketten und Migrationspfad aus R1 | Auslieferbar, Altbestände übernehmbar |

Schritt 3 steht bewusst vor allem Bauen: Die Wahl zwischen den beiden Anbindungswegen ist
die einzige verbliebene Entscheidung, deren Korrektur später teuer wäre.

---

## 9. Was bewusst nicht empfohlen wird

| Nicht empfohlen | Grund |
|---|---|
| Zwei Oberflächen (Tauri unter macOS/Linux, anderes unter Windows) | Doppelter Bau- und Pflegeaufwand, jede Ergänzung fällt zweimal an |
| Electron als quelloffener Ersatz für Tauri | Behält den TypeScript-Bestand, erbt aber genau die Druckschwäche, die es zu beseitigen gilt — und bringt über 100 MB je Paket sowie eine eigene Chromium-Sicherheitspflege mit |
| Reines Rust-Oberflächen-Toolkit | PDF-Anzeige und Druckaufbereitung müssten vollständig selbst entstehen. Das ist der Stack von R2, an diesem Umfang bereits einmal gescheitert |
| Statisches Binden von Qt | Unter LGPL nur mit zusätzlichen Auflagen zulässig; der dynamische Weg ist der Standardweg der Qt-Werkzeuge |
| Tantivy zusätzlich zu FTS5 | Zweiter Index bedeutet zweiten Konsistenzpfad |
| Server- oder Cloudkomponente | Solange OP-01 offen ist, wäre das eine Wette |
| Mobile Zielplattformen | Keine Anforderung vorhanden; ungenutzter Pflegeaufwand |
| Eigener Aktualisierungsdienst | Ohne Signaturprüfung würde er alle übrigen Schutzmaßnahmen unterlaufen |

---

## 10. Änderungshistorie

| Version | Datum | Änderung |
|---|---|---|
| 1.0 | 23.08.2026 | Erstfassung — Fortführung auf Tauri v2 |
| 2.0 | 23.08.2026 | Neubewertung unter der Vorgabe ausschließlich quelloffener Komponenten: Oberfläche auf Qt 6 (LGPL-3), Rust-Kern unverändert; Lizenzkapitel und Befunde B-09 bis B-13 ergänzt |
| 2.2 | 23.08.2026 | Verweise auf URS-STM-001 v1.2 und DES-STM-001 v1.2 nachgezogen. Anlass ist eine Farbkorrektur in DES-STM-001; an der Technologieentscheidung ändert sich nichts. |
| 2.1 | 23.08.2026 | Abstimmung mit URS-STM-001 v1.1 und DES-STM-001 v1.1. Neues Kapitel 0 klärt das Verhältnis von Neuentwicklung und Wiederverwendung. Verweis SM-SEC-009 auf SM-SEC-008 richtiggestellt, Signaturfrage auf OP-04 umgestellt, Befunde als Fremddokument gekennzeichnet, Prototypprüfung an die Toleranz aus SM-PRN-006 gebunden, Abschnitt zu SM-PLT-007/009/010 ergänzt. Nachweis: ABG-STM-001. |
