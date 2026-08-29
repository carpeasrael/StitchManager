# StitchManager — Lastenheft / User Requirements Specification

| | |
|---|---|
| **Dokument** | Lastenheft StitchManager |
| **Kennung** | URS-STM-001 |
| **Version** | 1.4 |
| **Datum** | 29.08.2026 |
| **Status** | Entwurf — zur Prüfung und Freigabe |
| **Ersetzt** | — |

---

## 1. Dokumentenlenkung

### 1.1 Zweck

Dieses Dokument beschreibt die Anforderungen an **StitchManager**, eine neu zu entwickelnde
Desktop-Anwendung zur Verwaltung von Stickdateien und Schnittmustern. Es beschreibt, **was**
das System leisten muss, nicht **wie** es umgesetzt wird.

Es dient als Grundlage für die technische Spezifikation, die Aufwandsschätzung, die
Verifikation und die Abnahme.

### 1.2 Geltungsbereich

Das Dokument erfasst den vollständigen Funktionsumfang der Anwendung für alle drei
Zielplattformen. Es gilt für die Erstentwicklung und ist bei jeder Anforderungsänderung
fortzuschreiben.

### 1.3 Zielgruppe

Auftraggeber, Entwicklung, Test, Abnahme.

### 1.4 Verbindlichkeit

Anforderungen mit der Priorität **Muss** sind Bestandteil des Leistungsumfangs. Anforderungen
mit **Soll** werden umgesetzt, sofern der Aufwand vertretbar ist; ihre Streichung ist zu
begründen und zu dokumentieren. **Kann**-Anforderungen sind Ausbaustufen ohne Zusage.

### 1.5 Mitgeltende Unterlagen

| Kennung | Dokument | Inhalt |
|---|---|---|
| DES-STM-001 v1.4 | StitchManager_Design_Beschreibung.md | Verbindliche Gestaltungsvorgaben, Farbvariablen, Komponentenverhalten. Konkretisiert Kapitel 12 dieses Dokuments. |
| TEC-STM-001 v2.3 | StitchManager_TechStack.md | Technologieentscheidung und Auslieferungswege. Löst die Anforderungen dieses Dokuments technisch auf. |
| ANA-STM-001 v1.1 | StitchManager_Requirements_konsolidiert.md | Herleitung aus drei Vorgängerständen. **Nur Herleitung, keine Anforderungsquelle.** Die dortigen Befunde B-01 bis B-13 sind keine Anforderungen dieses Dokuments. |
| ABG-STM-001 | StitchManager_Abstimmungsprotokoll.md | Nachweis der Abstimmung der drei Dokumente |
| — | stitchmanager-mockup.html | Visuelle Referenz des Hauptfensters |
| IMP-STM-001 v1.1 | StitchManager_Implementierungsplan.md | Zuordnung der Anforderungen zu Arbeitspaketen und Prüffällen. **Nachgeordnet — begründet und beschränkt keine Anforderung.** |

**Dokumentenhierarchie:** Dieses Lastenheft ist führend. Bei Widersprüchen gilt es vor
DES-STM-001 und TEC-STM-001. Keines der beiden nachgeordneten Dokumente darf eine Anforderung
begründen, die hier fehlt.

### 1.6 Begriffe

| Begriff | Bedeutung |
|---|---|
| **Stickdatei** | Maschinenlesbare Datei mit Stichdaten für eine Stickmaschine (PES, DST, JEF, VP3, EXP, XXX) |
| **Schnittmuster** | Dokument, meist PDF, aus dem Stoffteile zugeschnitten werden |
| **Muster** | Oberbegriff für beide, ein Eintrag der Bibliothek |
| **Bibliothek** | Gesamtbestand der erfassten Muster einschließlich Metadaten |
| **Eintrag** | Ein Muster mit allen zugehörigen Dateien und Metadaten |
| **Stickfeld / Rahmen** | Maximal bestickbarer Bereich einer Maschine |
| **Sidecar** | Metadatendatei neben der eigentlichen Datei |
| **Intelligenter Ordner** | Gespeicherte Filterabfrage, die wie ein Ordner erscheint |
| **Kachelung** | Aufteilung eines großformatigen Schnittmusters auf mehrere Druckseiten |
| **Maßhaltig** | Der Ausdruck entspricht exakt der Sollgröße, ohne Skalierung |

---

## 2. Produktüberblick

### 2.1 Produktvision

StitchManager verwaltet den Musterbestand einer Näherei — vom einzelnen Hobbybestand bis zum
gewerblichen Betrieb. Die Anwendung findet Muster wieder, zeigt sie an, bereitet sie für die
Maschine auf und druckt Schnittmuster maßhaltig aus. Sie arbeitet vollständig lokal und ohne
Internetverbindung.

### 2.2 Systemkontext

```text
        ┌──────────────────────┐
        │  Nutzer              │
        └──────────┬───────────┘
                   │
        ┌──────────┴───────────┐        ┌───────────────────────┐
        │                      │◄──────►│ Dateisystem           │
        │    StitchManager     │        │ (Musterbestand)       │
        │                      │        └───────────────────────┘
        │  Bibliothek          │        ┌───────────────────────┐
        │  Vorschau            │◄──────►│ Wechseldatenträger    │
        │  Konvertierung       │        │ (Maschinentransfer)   │
        │  Druck               │        └───────────────────────┘
        │  Projekte            │        ┌───────────────────────┐
        │  Fertigung           │◄──────►│ Drucker               │
        │                      │        └───────────────────────┘
        └──────────┬───────────┘        ┌───────────────────────┐
                   └───────────────────►│ KI-Dienst (optional,  │
                       abschaltbar      │ lokal oder entfernt)  │
                                        └───────────────────────┘
```

### 2.3 Nutzerrollen

| Rolle | Beschreibung | Schwerpunkt |
|---|---|---|
| **Hobbynutzer** | einige hundert bis tausend Muster | Suchen, Vorschau, Drucken, Übertragen |
| **Power-User** | zehntausende Muster | Massenimport, Regelwerke, Stapelverarbeitung, Verschlagwortung |
| **Gewerblicher Nutzer** | fertigt und verkauft | zusätzlich Projekte, Stücklisten, Beschaffung, Zeiterfassung, Kalkulation |

Ein Rechte- und Rollenkonzept innerhalb der Anwendung ist nicht gefordert. Die Anwendung
läuft im Benutzerkontext des Betriebssystems.

### 2.4 Betriebsmodi

Die Anwendung unterscheidet zwei Modi, die der Nutzer in den Einstellungen wählt:

| Modus | Umfang |
|---|---|
| **Standard** | Bibliothek, Vorschau, Suche, Konvertierung, Druck, Projekte |
| **Gewerblich** | zusätzlich Fertigung, Beschaffung, Qualitätsprüfung, Zeiterfassung, Kalkulation |

Im Standardmodus sind die gewerblichen Bereiche vollständig ausgeblendet — nicht nur
deaktiviert.

---

## 3. Abgrenzung

### 3.1 Im Leistungsumfang

Verwaltung, Anzeige, Suche, Konvertierung, Übertragung und Druck von Stickdateien und
Schnittmustern; Projekt- und Fertigungsverwaltung; Betrieb auf macOS, Windows und Linux.

### 3.2 Ausdrücklich nicht im Leistungsumfang

| Nicht enthalten | Anmerkung |
|---|---|
| Erstellen oder Bearbeiten von Stickmustern (Digitizing) | Die Anwendung verwaltet, sie entwirft nicht |
| Direkte Maschinensteuerung über Netzwerk oder serielle Schnittstelle | Übertragung erfolgt über Wechseldatenträger |
| Serverbetrieb, gemeinsame Datenbank, Cloud-Synchronisation | siehe OP-01 |
| Webshop-, Kassen- oder Buchhaltungsfunktionen | Kalkulation ja, Buchhaltung nein |
| Mobile Plattformen | keine Anforderung |
| Nutzerverwaltung, Mandantenfähigkeit | Einzelplatzanwendung |

---

## 4. Annahmen und Randbedingungen

| Nr | Randbedingung |
|---|---|
| RB-01 | **Ausschließlich quelloffene Komponenten.** Bindende Vorgabe des Auftraggebers, siehe Kapitel 10. |
| RB-02 | **Einzelplatzanwendung.** Ein Nutzer, ein Gerät, eine lokale Datenhaltung. |
| RB-03 | **Offline-fähig.** Bis auf die optionale entfernte KI-Analyse ist keine Funktion von einer Internetverbindung abhängig. |
| RB-04 | **Originaldateien bleiben unverändert.** Die Anwendung verwaltet den Bestand, ohne die Quelldateien beim Import zu verändern oder zu verschieben. |
| RB-05 | **Deutsch ist die führende Oberflächensprache.** |
| RB-06 | Die Stickdateiformate sind herstellereigen und nicht offen dokumentiert; die Verarbeitung beruht auf rückgewonnenen Formatbeschreibungen. Vollständigkeit je Formatvariante ist nicht garantiert. |

---

## 5. Aufbau der Anforderungen

Jede Anforderung trägt eine eindeutige, dauerhafte Kennung nach dem Muster
`SM-<Bereich>-<Nr>`. **Kennungen werden nicht wiederverwendet.** Entfällt eine Anforderung,
bleibt die Kennung als gestrichen vermerkt.

Eine gestrichene Anforderung bleibt mit durchgestrichener Kennung und dem Grund im Dokument
stehen. Verweise aus anderen Dokumenten laufen dadurch nicht ins Leere.

**Priorität:** `M` Muss · `S` Soll · `K` Kann

**Prüfmethode:** `T` Test (ausführbar, wiederholbar) · `D` Demonstration (Vorführung am
laufenden System) · `I` Inspektion (Sichtprüfung von Ergebnis, Code oder Konfiguration) ·
`A` Analyse (Messung, Berechnung, Auswertung)

---

## 6. Funktionale Anforderungen

### 6.1 Bibliothek und Ordnerverwaltung (LIB)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-LIB-001 | Das System muss eine lokale Bibliothek aus einem oder mehreren Wurzelverzeichnissen verwalten. | M | T |
| SM-LIB-002 | Das System muss die Ordnerhierarchie des Dateisystems in der Navigation abbilden. | M | D |
| SM-LIB-003 | Das System muss bestehende Ordnerstrukturen integrieren, ohne die Originaldateien zu verändern. | M | T |
| SM-LIB-004 | Das System muss Ordner anlegen, umbenennen, verschieben und löschen können. | M | T |
| SM-LIB-005 | Das System muss Sammlungen unterstützen, die unabhängig von der Ordnerstruktur bestehen. | S | T |
| SM-LIB-006 | Das System muss intelligente Ordner als gespeicherte Filterabfragen unterstützen. | S | T |
| SM-LIB-007 | Das System muss Einträge als Favoriten kennzeichnen können. | S | T |
| SM-LIB-008 | Das System muss leere Ordner erkennen und deren Bereinigung anbieten. | K | D |
| SM-LIB-009 | Das System muss mindestens 100.000 Einträge ohne Funktionsverlust verwalten. | M | A |
| SM-LIB-010 | Das System muss jedem Eintrag eine dauerhafte, eindeutige interne Kennung zuweisen, die bei Umbenennung oder Verschiebung erhalten bleibt. | M | T |
| SM-LIB-011 | Die Navigationsspalte muss eine Übersichtskarte mit dem Gesamtbestand als Zahl und der Formatverteilung als Balken mit Legende anzeigen. | M | D |

### 6.2 Import und Erfassung (IMP)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-IMP-001 | Das System muss Dateien aus einem lokalen Verzeichnis rekursiv importieren. | M | T |
| SM-IMP-002 | Der Import muss als Hintergrundvorgang mit Fortschrittsanzeige ablaufen und die Bedienung nicht blockieren. | M | D |
| SM-IMP-003 | Das System muss inkrementell importieren und nur neue, geänderte oder entfernte Dateien verarbeiten. | M | T |
| SM-IMP-004 | Das System muss überwachte Ordner auf Änderungen überwachen und neue Dateien automatisch erfassen. | S | T |
| SM-IMP-005 | Das System muss Duplikate über einen Hash des Dateiinhalts erkennen und dem Nutzer zur Entscheidung vorlegen. | M | T |
| SM-IMP-006 | Das System muss vor der Übernahme eine Vorschau des Importergebnisses anzeigen. | S | D |
| SM-IMP-007 | Das System muss konfigurierbare Regeln unterstützen, die Metadaten aus Pfad- und Ordnernamen ableiten (Ordnername als Schlagwort, Pfadsegment als Kategorie, regulärer Ausdruck). | S | T |
| SM-IMP-008 | Das System muss den Import per Ziehen und Ablegen unterstützen. | S | D |
| SM-IMP-009 | Das System muss defekte oder nicht lesbare Dateien mit Fehlerstatus erfassen, ohne den Importlauf abzubrechen. | M | T |
| SM-IMP-010 | Das System muss nach jedem Importlauf eine Statistik ausgeben: verarbeitet, übersprungen, fehlerhaft, Laufzeit. | S | D |

### 6.3 Stickdateiformate (FMT)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-FMT-001 | Das System muss PES lesen (Brother, Babylock). | M | T |
| SM-FMT-002 | Das System muss DST lesen (Tajima). | M | T |
| SM-FMT-003 | Das System muss JEF lesen (Janome). | M | T |
| SM-FMT-004 | Das System muss VP3 lesen (Pfaff, Husqvarna). | M | T |
| SM-FMT-005 | Das System muss EXP lesen (Melco). | S | T |
| SM-FMT-006 | Das System muss XXX lesen (Singer). | S | T |
| SM-FMT-007 | Das System soll HUS und VIP lesen. | K | T |
| SM-FMT-008 | Das System muss je Datei Stichanzahl, Abmessungen, Farbanzahl und Farbwechsel auslesen. | M | T |
| SM-FMT-009 | Das System muss Rahmeninformationen auslesen, sofern das Format sie enthält. | S | T |
| SM-FMT-010 | Das System muss herstellerspezifische Garnfarbpaletten zuordnen, mindestens Brother und Janome. | S | T |
| SM-FMT-011 | Das System muss eingebettete Vorschaubilder extrahieren, sofern vorhanden. | S | T |
| SM-FMT-012 | Alle Formatparser müssen gegen fehlerhafte und gezielt manipulierte Dateien abgesichert sein: kein Absturz, keine unbegrenzte Speicherbelegung, keine Endlosschleife. | M | T |
| SM-FMT-013 | Das System muss das Format anhand des Dateiinhalts bestimmen, nicht allein anhand der Dateiendung. | M | T |

### 6.4 Schnittmuster und Dokumente (DOC)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-DOC-001 | Das System muss PDF-Schnittmuster als eigenständigen Eintragstyp verwalten. | M | T |
| SM-DOC-002 | Das System muss mehrere Dateien je Eintrag verwalten: Schnittmuster, Nähanleitung, Titelbild, Maßtabelle, Stoffbedarf. | M | T |
| SM-DOC-003 | Jede angehängte Datei muss typisiert klassifiziert werden. | M | T |
| SM-DOC-004 | Das System muss Nähanleitungen innerhalb der Anwendung anzeigen; ein externes Programm darf nicht erforderlich sein. | M | D |
| SM-DOC-005 | Das System muss seitenweise Navigation in Anleitungen unterstützen. | S | D |
| SM-DOC-006 | Das System muss Notizen an einzelne Seiten oder Abschnitte einer Anleitung binden können. | S | T |
| SM-DOC-007 | Das System muss Lesezeichen auf Anleitungsseiten unterstützen. | S | T |
| SM-DOC-008 | Das System muss Seitenanzahl, Papierformat und Dokumenteigenschaften anzeigen. | S | D |
| SM-DOC-009 | Das System muss gängige Bildformate als Anhang unterstützen. | S | T |

### 6.5 Metadaten und Verschlagwortung (MET)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-MET-001 | Das System muss je Eintrag Name, Thema, Beschreibung und Notizen bearbeitbar machen. | M | T |
| SM-MET-002 | Das System muss freie Schlagworte zulassen, mehrere je Eintrag. | M | T |
| SM-MET-003 | Das System muss Schlagworte umbenennen, zusammenführen und löschen können; die Änderung wirkt auf alle betroffenen Einträge. | S | T |
| SM-MET-004 | Das System muss bei der Eingabe von Schlagworten Vorschläge aus dem Bestand anbieten. | S | D |
| SM-MET-005 | Das System muss die Verschlagwortung über eine Mehrfachauswahl in einem Vorgang erlauben. | M | T |
| SM-MET-006 | Das System muss benutzerdefinierte Metadatenfelder unterstützen (Text, Zahl, Datum). | S | T |
| SM-MET-007 | Das System muss für Schnittmuster die Felder Designer, Kleidungstyp, Größenbereich, Schwierigkeitsgrad, Sprache und Bezugsquelle führen. | M | T |
| SM-MET-008 | Das System muss je Eintrag einen Bearbeitungsstatus führen: nicht begonnen, geplant, in Arbeit, fertig, archiviert. | S | T |
| SM-MET-009 | Das System muss ungespeicherte Änderungen erkennen und beim Verlassen des Eintrags nachfragen. | M | D |
| SM-MET-010 | Das System muss die Garnfarben je Stickdatei speichern und anzeigen, mit Garnname, Nummer und Stichanteil. | S | T |

### 6.6 Vorschau und Darstellung (PRV)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-PRV-001 | Das System muss aus den Stichdaten eine farbige Vorschau erzeugen. | M | D |
| SM-PRV-002 | Vorschaubilder müssen dauerhaft zwischengespeichert werden. | M | T |
| SM-PRV-003 | Der Zwischenspeicher muss verworfen werden, sobald sich die Quelldatei ändert. | M | T |
| SM-PRV-004 | Die Vorschau muss Zoom und Verschieben unterstützen. | S | D |
| SM-PRV-005 | Das System soll das Ein- und Ausblenden einzelner Farbebenen erlauben. | K | D |
| SM-PRV-006 | Das System muss zwischen Kachel- und Listenansicht umschalten. | S | D |
| SM-PRV-007 | Listen und Kachelraster müssen virtualisiert sein; nur sichtbare Einträge werden gezeichnet. | M | A |
| SM-PRV-008 | Das System soll eine Schnellvorschau ohne Öffnen eines Dialogs bieten. | K | D |
| SM-PRV-009 | Die Höhe einer Kachel muss feststehen, bevor die Vorschau geladen ist; das Layout darf beim Nachladen nicht springen. | M | D |

### 6.7 Suche, Filter und Sortierung (SRC)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-SRC-001 | Das System muss eine Volltextsuche über Name, Thema, Beschreibung, Schlagworte und weitere Textfelder bieten. | M | T |
| SM-SRC-002 | Die Suche muss ordnerübergreifend über die gesamte Bibliothek laufen. | M | T |
| SM-SRC-003 | Das System muss nach Schlagwort, Format, Größenbereich, Stichanzahlbereich, Farbanzahl und Importquelle filtern. | M | T |
| SM-SRC-004 | Das System muss nach Schwierigkeitsgrad, Designer, Sprache, Status und Kleidungstyp filtern. | S | T |
| SM-SRC-005 | Das System muss nach Name, Import- und Dateidatum, Größe, Stichanzahl und Relevanz sortieren. | M | T |
| SM-SRC-006 | Das System muss zusätzlich nach Designer, Kategorie und letzter Änderung sortieren. | S | T |
| SM-SRC-007 | Die Suche muss bei 100.000 Einträgen und warmem Index innerhalb einer Sekunde ein Ergebnis liefern. | M | A |
| SM-SRC-008 | Die Sucheingabe muss entprellt sein und darf die Bedienung nicht blockieren. | M | A |
| SM-SRC-009 | Aktive Filter müssen sichtbar dargestellt und einzeln entfernbar sein. | S | D |
| SM-SRC-010 | Das System muss mehrere Filter gleichzeitig anwenden und mit der Volltextsuche kombinieren. | M | T |

### 6.8 Stapelverarbeitung (BAT)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-BAT-001 | Das System muss Mehrfachauswahl in der Musterauswahl unterstützen. | M | D |
| SM-BAT-002 | Das System muss Dateien anhand konfigurierbarer Namensmuster stapelweise umbenennen; Variablen mindestens Name, Thema, Format, laufende Nummer. | M | T |
| SM-BAT-003 | Das System muss anhand von Metadaten eine Zielverzeichnisstruktur erzeugen und Dateien dorthin einsortieren. | S | T |
| SM-BAT-004 | Vor der Ausführung eines Stapelvorgangs muss eine Vorschau der Änderungen erscheinen. | M | D |
| SM-BAT-005 | Stapelvorgänge müssen einen Fortschritt anzeigen und abbrechbar sein. | S | D |
| SM-BAT-006 | Stapelvorgänge müssen ein Protokoll erzeugen, aus dem fehlgeschlagene Einzelvorgänge mit Grund hervorgehen. | S | T |
| SM-BAT-007 | Ein abgebrochener Stapelvorgang darf keinen halb geänderten Zustand hinterlassen; bereits ausgeführte Einzelvorgänge müssen im Protokoll ausgewiesen sein. | M | T |

### 6.9 Konvertierung, Export und Datenträger (EXP)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-EXP-001 | Das System muss Stickdateien in ein wählbares Zielformat konvertieren. | M | T |
| SM-EXP-002 | Der Schreibpfad muss mindestens PES und DST umfassen; JEF, EXP und VP3 sollen ergänzt werden. | S | T |
| SM-EXP-003 | Das System muss vor der Konvertierung ausweisen, welche Zielformate für die gewählte Datei verfügbar sind. | M | D |
| SM-EXP-004 | Das System muss angeschlossene Wechseldatenträger erkennen und als Exportziel anbieten. | M | D |
| SM-EXP-005 | Vor dem Kopieren müssen freier Speicherplatz und Schreibrechte geprüft werden. | M | T |
| SM-EXP-006 | Bei Namenskonflikten muss das System Überschreiben, Umbenennen oder Abbrechen anbieten. | M | T |
| SM-EXP-007 | Das System muss einzelne Dateien und Auswahlmengen exportieren. | M | T |
| SM-EXP-008 | Das System muss Metadaten strukturiert exportieren, mindestens als JSON und CSV. | S | T |
| SM-EXP-009 | Das System muss ein Exportpaket aus Metadaten und referenzierten Dateien erzeugen, das auf einem anderen Gerät wieder eingelesen werden kann. | S | T |
| SM-EXP-010 | Das System muss fehlende Dateien nach einem Pfadwechsel neu verknüpfen können. | S | T |
| SM-EXP-011 | Das System muss den Speicherort einer Datei im Dateimanager des Betriebssystems anzeigen können. | K | D |

### 6.10 Drucken (PRN)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-PRN-001 | Das System muss Schnittmuster direkt aus der Anwendung drucken. | M | D |
| SM-PRN-002 | Für den Druck darf kein externes Programm erforderlich sein. | M | D |
| SM-PRN-003 | Das System muss eine Druckvorschau anzeigen, die dem Ausdruck entspricht. | M | D |
| SM-PRN-004 | Das System muss den Druck des gesamten Dokuments oder ausgewählter Seiten erlauben. | M | T |
| SM-PRN-005 | Papierformat, Ausrichtung, Seitenbereich und Drucker müssen wählbar sein. | M | D |
| SM-PRN-006 | Das System muss maßhaltig drucken. Ein Prüfmaß von 100 mm im Quelldokument muss im Ausdruck 100 mm ± 0,5 mm messen. | M | A |
| SM-PRN-007 | Jede unbeabsichtigte Skalierung muss standardmäßig unterbunden sein. | M | T |
| SM-PRN-008 | Das System muss warnen, wenn gewählte Einstellungen den Maßstab verändern können. | M | D |
| SM-PRN-009 | Das System muss A4 und US Letter unterstützen. | M | T |
| SM-PRN-010 | Das System muss großformatige Schnittmuster gekachelt über mehrere Seiten drucken, mit Klebemarken, Überlappung und Seitenkennzeichnung. | S | T |
| SM-PRN-011 | Das System soll Großformate für den Copyshop ausgeben, sofern die Quelldatei sie enthält. | S | T |
| SM-PRN-012 | Das System soll ein im Quelldokument enthaltenes Kalibrierquadrat in der Vorschau hervorheben. | S | D |
| SM-PRN-013 | Das System soll den Druck einzelner Ebenen oder Größen erlauben, sofern das Dokument Ebenen enthält. | K | T |
| SM-PRN-014 | Das System muss je Stickdatei einen PDF-Bericht mit Vorschau, Kennwerten und Farbliste erzeugen. | S | T |
| SM-PRN-015 | Der Ausdruck darf keine Themenfarben der Oberfläche übernehmen; er erfolgt unabhängig vom eingestellten Hell- oder Dunkelmodus. | M | I |

### 6.11 KI-gestützte Analyse (KIA)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-KIA-001 | Das System muss aus dem Vorschaubild Metadatenvorschläge erzeugen: Name, Thema, Beschreibung, Schlagworte, Farben. | S | D |
| SM-KIA-002 | Das System muss eine lokale Verarbeitung unterstützen. | M | T |
| SM-KIA-003 | Das System kann eine Verarbeitung über einen entfernten Dienst unterstützen. | K | T |
| SM-KIA-004 | Die KI-Funktion muss standardmäßig deaktiviert sein und ausdrücklich aktiviert werden. | M | I |
| SM-KIA-005 | Vor der ersten Nutzung eines entfernten Dienstes muss das System unmissverständlich darauf hinweisen, dass Bilddaten das Gerät verlassen, und eine Bestätigung einholen. | M | D |
| SM-KIA-006 | Das System muss den erzeugten Auftrag vor dem Senden anzeigen und bearbeitbar machen. | S | D |
| SM-KIA-007 | Der Nutzer muss jedes Ergebnisfeld einzeln übernehmen oder verwerfen können. | M | D |
| SM-KIA-008 | Maschinell erzeugte Werte müssen bis zur Bestätigung als solche gekennzeichnet bleiben, erkennbar auch ohne Farbwahrnehmung. | M | I |
| SM-KIA-009 | Das System muss eine Stapelanalyse mit Fortschrittsanzeige unterstützen. | S | D |
| SM-KIA-010 | Zugangsschlüssel müssen im Schlüsselspeicher des Betriebssystems abgelegt werden, nie in der Datenbank oder in Klartextdateien. | M | I |
| SM-KIA-011 | Fällt der KI-Dienst aus, muss das System eine verständliche Meldung ausgeben; alle übrigen Funktionen bleiben nutzbar. | M | T |

### 6.12 Projekte (PRJ)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-PRJ-001 | Das System muss Projekte anlegen und mit einem oder mehreren Einträgen verknüpfen. | S | T |
| SM-PRJ-002 | Ein Projekt muss projektspezifische Angaben führen: gewählte Größe, Stoff, geplante Änderungen, Zuschnittvariante, Notizen. | S | T |
| SM-PRJ-003 | Ein Projekt muss dupliziert werden können, ohne die zugrunde liegenden Dateien zu duplizieren. | S | T |
| SM-PRJ-004 | Projektnotizen müssen getrennt von den Metadaten der Vorlage gespeichert werden. | S | T |
| SM-PRJ-005 | Das System muss Projektkosten aus verbrauchten Materialien und erfassten Zeiten ermitteln. | K | T |

### 6.13 Fertigung und Beschaffung (MFG) — nur im gewerblichen Modus

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-MFG-001 | Das System muss Produkte und Produktvarianten verwalten. | K | T |
| SM-MFG-002 | Das System muss Stücklisten je Produkt führen. | K | T |
| SM-MFG-003 | Das System muss Arbeitsschritte und deren Definitionen verwalten. | K | T |
| SM-MFG-004 | Das System muss Materialbestände und Bestandsbewegungen führen. | K | T |
| SM-MFG-005 | Das System muss Lieferanten, Bestellungen, Bestellpositionen und Wareneingänge verwalten. | K | T |
| SM-MFG-006 | Das System muss Qualitätsprüfungen und Fehlerbilder erfassen. | K | T |
| SM-MFG-007 | Das System muss Zeiten je Arbeitsschritt erfassen und mit Kostensätzen bewerten. | K | T |
| ~~SM-MFG-008~~ | *gestrichen — verschoben nach SM-MAC-001, weil die Anforderung in beiden Betriebsmodi gilt* | — | — |
| ~~SM-MFG-009~~ | *gestrichen — verschoben nach SM-MAC-002, weil die Anforderung in beiden Betriebsmodi gilt* | — | — |

### 6.14 Maschinen und Stickfeld (MAC) — in beiden Betriebsmodi

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-MAC-001 | Das System muss Maschinenprofile mit Bezeichnung, Stickfeldbreite, Stickfeldhöhe und unterstützten Formaten führen. | S | T |
| SM-MAC-002 | Das System muss zu jedem angezeigten Stickmuster ausweisen, ob es in das eingestellte Stickfeld passt — mit Angabe der Reserve bei passenden und der Überschreitung bei nicht passenden Mustern. Die Angabe erscheint in beiden Fällen, nicht nur im Fehlerfall. | S | D |
| SM-MAC-003 | Das System muss das aktive Maschinenprofil ohne Verlassen des Detailbereichs umschaltbar machen. | S | D |
| SM-MAC-004 | Unterstützt das aktive Maschinenprofil das Format eines Musters nicht, muss das System darauf hinweisen und die verfügbaren Zielformate der Konvertierung anbieten. | S | D |

### 6.15 Lizenzverwaltung des Musterbestands (LIC)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-LIC-001 | Das System muss je Eintrag Lizenzangaben und Bezugsquelle erfassen. | S | T |
| SM-LIC-002 | Das System muss Lizenzdokumente als Anhang speichern und mit Einträgen verknüpfen. | S | T |
| SM-LIC-003 | Das System muss Lizenzen mit Projekten verknüpfen, um die kommerzielle Nutzbarkeit nachzuweisen. | K | T |
| SM-LIC-004 | Das System muss Einträge ohne hinterlegte Lizenzangabe auffindbar machen. | S | T |

### 6.16 Datensicherung, Versionierung und Nachvollziehbarkeit (DAT)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-DAT-001 | Das System muss Bibliothek und Metadaten sichern und wiederherstellen können. | M | T |
| SM-DAT-002 | Das System muss bei Bearbeitungen Versionsstände anlegen. | S | T |
| SM-DAT-003 | Vor dem Löschen von Einträgen oder Dateien muss das System bestätigen lassen. | M | D |
| SM-DAT-004 | Das System soll gelöschte Einträge in einem Papierkorb vorhalten, aus dem sie wiederherstellbar sind. | S | T |
| SM-DAT-005 | Das System muss Änderungen an Einträgen in einem Änderungsprotokoll festhalten. | S | T |
| SM-DAT-006 | Die Datenhaltung muss nach einem unerwarteten Programmende ohne Datenverlust wieder anlaufen. | M | T |
| SM-DAT-007 | Schemaänderungen müssen additiv und versioniert erfolgen; bestehende Migrationsschritte dürfen nicht nachträglich verändert werden. | M | I |
| SM-DAT-008 | Das System muss eine ältere Datenhaltung beim Start erkennen und vor der Anpassung eine Sicherung anlegen. | M | T |

### 6.17 Einstellungen, Darstellung und Sprache (SET)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-SET-001 | Das System muss einen Hell- und einen Dunkelmodus bieten. | M | D |
| SM-SET-002 | Das System muss die Systemeinstellung übernehmen können, mit manueller Übersteuerung; die manuelle Wahl muss einen Neustart überstehen. | S | T |
| SM-SET-003 | Panelbreiten und Fensterzustand müssen über Sitzungen hinweg erhalten bleiben. | S | T |
| SM-SET-004 | Das System muss Tastaturkürzel für die häufigsten Aktionen bieten. | S | D |
| SM-SET-005 | Tastaturkürzel sollen konfigurierbar sein. | K | T |
| SM-SET-006 | Das System muss vollständig auf Deutsch bedienbar sein. | M | I |
| SM-SET-007 | Das System soll auf Englisch umschaltbar sein. | S | D |
| SM-SET-008 | Das System muss den Betriebsmodus (Standard / Gewerblich) umschaltbar machen. | S | D |

### 6.18 Bestandsübernahme (MIG)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-MIG-001 | Das System muss Bestände aus verbreiteter Fremdsoftware zur Musterverwaltung übernehmen können, mindestens „2stitch Organizer". | S | T |
| SM-MIG-002 | Das System muss Metadaten aus Sidecar-Dateien neben den Stickdateien einlesen. | S | T |
| SM-MIG-003 | Das System muss Metadaten aus CSV und JSON einlesen und den Feldern zuordnen lassen. | S | T |
| SM-MIG-004 | Jede Übernahme muss einen Bericht erzeugen: übernommen, übersprungen, fehlerhaft. | S | T |
| SM-MIG-005 | Eine Übernahme darf die Quelldaten nicht verändern. | M | T |

---

## 7. Datenobjekte

Anforderungen an das fachliche Datenmodell. Die technische Ausgestaltung erfolgt im Pflichtenheft.

| Objekt | Muss enthalten | Beziehungen |
|---|---|---|
| **Eintrag** | interne Kennung, Name, Typ (Stickdatei/Schnittmuster), Thema, Beschreibung, Notizen, Status, Zeitstempel | 1:n Datei, n:m Schlagwort, n:m Sammlung, n:m Projekt |
| **Datei** | Pfad, Klassifizierung, Inhalts-Hash, Größe, Änderungszeit, Fehlerstatus | n:1 Eintrag |
| **Stickdaten** | Stichanzahl, Breite, Höhe, Farbwechsel, Format und Formatvariante, Rahmenangabe | 1:1 Datei |
| **Garnfarbe** | Reihenfolge, Farbwert, Hersteller, Garnnummer, Garnname, Stichanteil | n:1 Stickdaten |
| **Schlagwort** | Bezeichnung, Herkunft (manuell / maschinell) | n:m Eintrag |
| **Sammlung** | Bezeichnung, Beschreibung | n:m Eintrag |
| **Intelligenter Ordner** | Bezeichnung, gespeicherte Filterbedingung | — |
| **Projekt** | Bezeichnung, Status, gewählte Größe, Stoff, Notizen, Termine | n:m Eintrag |
| **Lizenz** | Art, Bezugsquelle, kommerzielle Nutzung erlaubt (ja/nein), Belegdokument | n:1 Eintrag |
| **Maschinenprofil** | Bezeichnung, Stickfeldbreite, Stickfeldhöhe, unterstützte Formate, aktiv (ja/nein) | — |
| **Änderungseintrag** | Zeitpunkt, betroffenes Objekt, Feld, Wert vorher, Wert nachher | n:1 Eintrag |

**Übergreifende Anforderungen:**

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-DTA-001 | Jedes maschinell erzeugte Metadatum muss seine Herkunft mitführen. | M | I |
| SM-DTA-002 | Abmessungen müssen in Millimetern mit einer Nachkommastelle geführt werden. | M | I |
| SM-DTA-003 | Zeitstempel müssen zeitzonenunabhängig gespeichert und in Ortszeit angezeigt werden. | M | T |
| SM-DTA-004 | Die gesamte Datenhaltung muss in einem einzigen, sicherbaren Ablageort liegen. | M | I |

---

## 8. Nicht-funktionale Anforderungen (NFR)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-NFR-001 | Das System muss 100.000 Einträge verwalten, ohne instabil zu werden. | M | A |
| SM-NFR-002 | Import, Indizierung und Vorschauerzeugung laufen im Hintergrund und blockieren die Bedienung nicht. | M | A |
| SM-NFR-003 | Es darf keine harte Obergrenze der darstellbaren Einträge geben. | M | T |
| SM-NFR-004 | Das System muss innerhalb von **fünf Sekunden** bedienbereit sein, gemessen auf einem Gerät mittlerer Ausstattung bei 100.000 Einträgen. | S | A |
| SM-NFR-005 | Defekte Dateien, fehlende Berechtigungen oder entfernte Datenträger dürfen nicht zum Absturz führen. | M | T |
| SM-NFR-006 | Fehler müssen für Endnutzer verständlich formuliert und zusätzlich technisch protokolliert werden. | M | I |
| SM-NFR-007 | Alle Text-Hintergrund-Kombinationen müssen WCAG AA erfüllen: 4,5:1 für Fließtext, 3:1 für große Schrift — in beiden Modi. | M | A |
| SM-NFR-008 | Das System muss vollständig per Tastatur bedienbar sein; jeder Dialog hält den Fokus und gibt ihn beim Schließen zurück. | M | D |
| SM-NFR-009 | Zustände dürfen nicht allein über Farbe kommuniziert werden. | S | I |
| SM-NFR-010 | Unicode-Pfade und lange Pfade müssen auf allen Zielplattformen korrekt behandelt werden. | M | T |
| SM-NFR-011 | Das System muss ohne Internetverbindung uneingeschränkt nutzbar sein, ausgenommen die entfernte KI-Analyse. | M | T |
| SM-NFR-012 | Der Code muss automatisiert geprüft werden: Modultests, Formatparser-Fuzzing, statische Analyse und Lizenzprüfung in der CI. | M | I |
| SM-NFR-013 | Die Systemeinstellung für reduzierte Bewegung muss respektiert werden; dann findet keine Bewegung statt. | S | D |
| SM-NFR-014 | Das System darf keine Telemetrie und keine Nutzungsdaten übertragen. | M | I |
| SM-NFR-015 | Jeder Bildschirm muss einen verständlichen Leer-, Lade- und Fehlerzustand zeigen; lange Vorgänge müssen abbrechbar sein und danach einen konsistenten, wiederholbaren Stand hinterlassen. | M | D |
| SM-NFR-016 | Jedes Bedienelement muss die Zustände Ruhe, Zeigerkontakt, Aktiv und Deaktiviert besitzen, mindestens 26 × 26 px sichtbar und über mindestens 32 × 32 px treffbar sein. Trenner dürfen 1 px sichtbar sein, besitzen mindestens 6 px Ziehfläche und eine 32 px breite unsichtbare Trefferzone. | M | D |

---

## 9. Sicherheitsanforderungen (SEC)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-SEC-001 | Alle vom Nutzer oder aus Dateien stammenden Pfade müssen gegen Verzeichniswechsel geprüft werden. | M | T |
| SM-SEC-002 | Dateizugriffe müssen nach Kanonisierung beider Seiten auf das Bibliothekswurzelverzeichnis eingegrenzt werden; Symlinks und Groß-/Kleinschreibungsfaltung sind zu berücksichtigen. | M | T |
| SM-SEC-003 | Das Bibliothekswurzelverzeichnis muss beim Setzen validiert werden; Systemwurzeln sind abzulehnen. | M | T |
| SM-SEC-004 | Die Oberfläche darf keinen direkten Zugriff auf die Datenhaltung haben; jeder Zugriff läuft über eine eng geschnittene Funktion. | M | I |
| SM-SEC-005 | Datenbankzugriffe müssen parametrisiert erfolgen; keine Zeichenkettenverkettung von Abfragen. | M | I |
| SM-SEC-006 | Zugangsschlüssel müssen im Schlüsselspeicher des Betriebssystems liegen. | M | I |
| SM-SEC-007 | Ausgehende Verbindungen müssen auf die tatsächlich benötigten Ziele begrenzt und vollständig abschaltbar sein. | M | I |
| SM-SEC-008 | Fremdtext — Dateinamen, Metadaten, maschinell erzeugte Antworten — muss als Nur-Text dargestellt werden; auszeichnungsfähige Anzeige ist an diesen Stellen unzulässig. | M | I |
| SM-SEC-009 | Die Berechtigungen der Anwendung müssen minimal gehalten werden; nicht genutzte Abhängigkeiten und Berechtigungen sind zu entfernen. | M | I |
| SM-SEC-010 | Protokolldateien dürfen keine vollständigen Pfade oder personenbezogenen Daten in unmaskierter Form enthalten, sofern sie exportierbar sind. | S | I |
| SM-SEC-011 | Alle Formatparser müssen dauerhaft gefuzzt werden; ein neues Format erfordert ein neues Fuzzing-Ziel. | M | I |
| SM-SEC-012 | Veröffentlichte Pakete müssen signiert sein, sofern die Signaturinfrastruktur bereitsteht — siehe OP-04. | M | I |
| SM-SEC-013 | Ein Aktualisierungsmechanismus muss signierte Pakete prüfen. Ohne Signaturprüfung ist kein automatisches Update zulässig. | M | I |
| SM-SEC-014 | Unter Linux muss die Anwendung in der Sandbox betrieben werden können und dabei Schreibzugriff auf Wechseldatenträger sowie Zugriff auf den Schlüsselspeicherdienst besitzen. | M | T |
| SM-SEC-015 | Der Zugriff auf das Dateisystem soll unter Linux über Portale erfolgen statt über pauschalen Zugriff auf das Benutzerverzeichnis. | S | I |

---

## 10. Lizenz- und Herkunftsanforderungen (OSS)

> **Bindende Vorgabe des Auftraggebers:** Das Produkt verwendet ausschließlich quelloffene
> Komponenten. Die Vorgabe erfasst alle Komponenten, die die Anwendung mitliefert, gegen die
> sie bindet oder deren Installation sie voraussetzt. Sie erfasst nicht das Betriebssystem,
> auf dem die Anwendung läuft.

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-OSS-001 | Alle mitgelieferten und eingebundenen Komponenten müssen unter einer von der Open Source Initiative anerkannten Lizenz stehen. | M | I |
| SM-OSS-002 | Das System darf keine proprietäre Laufzeitumgebung voraussetzen, deren Installation es anstößt oder mitliefert. | M | I |
| SM-OSS-003 | Die Anzeigekomponente der Oberfläche muss quelloffen sein; proprietäre System-Webansichten sind ausgeschlossen. | M | I |
| SM-OSS-004 | Alle Lizenzen müssen mit der Projektlizenz verträglich sein; die Verträglichkeit ist je Abhängigkeit nachzuweisen. | M | A |
| SM-OSS-005 | Die Projektlizenz muss eindeutig und widerspruchsfrei ausgewiesen sein — in Lizenzdatei, Projektdatei und Paketmetadaten übereinstimmend. | M | I |
| SM-OSS-006 | Bei Bibliotheken unter schwachem Copyleft muss die Bindung dynamisch erfolgen und der Austausch durch den Empfänger möglich bleiben. | M | I |
| SM-OSS-007 | Schriften müssen unter einer freien Schriftlizenz stehen und mitgeliefert werden. | M | I |
| SM-OSS-008 | Jede Veröffentlichung muss eine maschinenlesbare Stückliste aller Abhängigkeiten samt Lizenzen enthalten. | S | I |
| SM-OSS-009 | Die Lizenzprüfung muss automatisiert in der CI erfolgen; der Bau bricht ab, wenn eine Abhängigkeit außerhalb der Positivliste liegt. | M | T |
| SM-OSS-010 | Das System muss die Lizenztexte aller eingebundenen Komponenten im Programm anzeigen können. | S | D |
| SM-OSS-011 | Vorgebaute Binärpakete Dritter dürfen nur eingebunden werden, wenn Quelle und Bauweg nachvollziehbar sind; andernfalls ist selbst zu bauen. | M | I |
| SM-OSS-012 | Ein mitgeliefertes oder empfohlenes KI-Modell muss seine Lizenz ausweisen; Modelle mit eingeschränkten Nutzungslizenzen dürfen nicht als Vorgabewert gesetzt sein. | S | I |
| SM-OSS-013 | Die Anbindung an proprietäre Dienste muss abschaltbar sein und darf nicht Voraussetzung einer Kernfunktion sein. | M | T |
| SM-OSS-014 | Datenbestände Dritter, etwa Garnfarbkarten, müssen ihre Herkunft ausweisen und als Kompatibilitätsangabe erkennbar sein, nicht als Herstellerbindung. | S | I |

---

## 11. Plattform und Auslieferung (PLT)

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-PLT-001 | Das System muss auf macOS als native Anwendung laufen, als Universal-Paket für beide Prozessorarchitekturen. | M | T |
| SM-PLT-002 | Das System muss auf macOS signiert und notarisiert ausgeliefert werden, sofern die Signaturinfrastruktur bereitsteht (OP-04). | M | I |
| SM-PLT-003 | Das System muss auf Windows als native Anwendung laufen und als Installationspaket ausgeliefert werden. | M | T |
| SM-PLT-004 | Das System muss auf Linux als Flatpak ausgeliefert werden. | M | T |
| SM-PLT-005 | Das Linux-Paket muss ohne Netzwerkzugriff baubar sein. | M | T |
| SM-PLT-006 | Das System muss sich in die Desktop-Umgebung integrieren: Anwendungseintrag, Symbole, Paketmetadaten. | S | I |
| SM-PLT-007 | Alle Versionsangaben in Projekt- und Paketdateien müssen identisch sein; die Übereinstimmung wird automatisiert geprüft. | S | T |
| SM-PLT-008 | Die Auslieferung aller drei Plattformen muss aus einem CI-Lauf hervorgehen. | S | T |
| SM-PLT-009 | Die Bauläufe müssen reproduzierbar sein; Abhängigkeitssperrdateien liegen in der Versionsverwaltung. | S | I |
| SM-PLT-010 | Das System darf keine Administratorrechte für Installation oder Betrieb voraussetzen. | S | T |

---

## 12. Gestaltung (DES)

Die vollständigen Vorgaben stehen in **DES-STM-001**. Hier stehen nur die Anforderungen, gegen
die abgenommen wird.

| ID | Anforderung | Prio | Prüf |
|---|---|---|---|
| SM-DES-001 | Die Oberfläche muss dem Kreuznaht-Design folgen: warme Grundpalette, Terracotta als Leitfarbe. | M | I |
| SM-DES-002 | Der Dunkelmodus verwendet Espressotöne, keine neutralen Graustufen. | M | I |
| SM-DES-003 | Alle Farb-, Schrift- und Abstandswerte müssen zentral als benannte Variablen definiert sein; kein Literalwert im Komponentencode. | M | I |
| SM-DES-004 | Schriften werden mitgeliefert und lokal eingebunden; kein Nachladen von externen Diensten zur Laufzeit. | M | I |
| SM-DES-005 | Das Hauptfenster ist dreispaltig: links Übersicht und Navigation, in der Mitte die Musterauswahl, rechts die Details. Die Trenner sind verschiebbar. | M | D |
| SM-DES-006 | Die Dreiteilung bleibt in jeder Fensterbreite senkrecht; eine gestapelte Ersatzdarstellung gibt es nicht. | M | D |
| SM-DES-007 | Die Musterauswahl zeigt je Eintrag ausschließlich Bild, Format- und Herkunftsmarke, Name und Größe. | M | I |
| SM-DES-008 | Alle Details eines Eintrags müssen im rechten Bereich erreichbar sein, gegliedert in Angaben, Größe, Farben und Optionen. | M | D |
| SM-DES-009 | Maschinell erzeugte Inhalte müssen visuell und textlich von manuell gepflegten unterscheidbar sein. | M | I |

---

## 13. Verifikation und Abnahme

### 13.1 Abnahmekriterien

| ID | Kriterium | Bezug | Methode |
|---|---|---|---|
| AK-01 | Der Import von 100.000 Dateien erzeugt eine durchsuchbare Bibliothek; die Bedienung bleibt währenddessen möglich. | SM-LIB-009, SM-NFR-002 | A |
| AK-02 | Eine Suche nach Schlagwort kombiniert mit einem Größenbereich liefert bei warmem Index in unter einer Sekunde ein Ergebnis. | SM-SRC-007 | A |
| AK-03 | Für jedes unterstützte Stickformat wird eine korrekte farbige Vorschau angezeigt. | SM-PRV-001 | D |
| AK-04 | Ein Export auf einen Wechseldatenträger erkennt einen Namenskonflikt und bietet Überschreiben, Umbenennen und Abbrechen — auf allen drei Plattformen, unter Linux auch in der Sandbox. | SM-EXP-006, SM-SEC-014 | T |
| AK-05 | Ein Schnittmuster und seine Nähanleitung liegen in einem Eintrag und werden beide in der Anwendung angezeigt. | SM-DOC-002, SM-DOC-004 | D |
| AK-06 | Ein gekachelter A4-Druck ergibt bei Standardeinstellungen ein maßhaltiges Ergebnis; ein Prüfmaß von 100 mm misst am Ausdruck 100 mm ± 0,5 mm. Nachzuweisen auf A4 und US Letter, auf allen drei Plattformen. | SM-PRN-006, SM-PRN-010 | A |
| AK-07 | Der Wechsel zwischen Hell- und Dunkelmodus wirkt auf alle Bildschirme und Dialoge; kein Element bleibt unlesbar. | SM-SET-001, SM-NFR-007 | D |
| AK-08 | Das System ist ohne Internetverbindung vollständig bedienbar; die entfernte KI meldet einen verständlichen Fehler statt zu blockieren. | SM-NFR-011, SM-KIA-011 | T |
| AK-09 | Ein Zugangsschlüssel wird nach Neustart wiedergefunden und ist in der Datenhaltung nicht auffindbar — auf allen drei Plattformen, unter Linux auch in der Sandbox. | SM-KIA-010, SM-SEC-014 | T |
| AK-10 | Eine Bestandsübernahme übernimmt Schlagworte und Metadaten und verändert die Quelldateien nicht. | SM-MIG-004, SM-MIG-005 | T |
| AK-11 | Die Stückliste einer Veröffentlichung weist ausschließlich anerkannte Open-Source-Lizenzen aus; ein eingeschleustes Fremdpaket lässt den Bau scheitern. | SM-OSS-008, SM-OSS-009 | T |
| AK-12 | Die drei Bereiche stehen bei 860 px, 1280 px und 2560 px Fensterbreite nebeneinander. | SM-DES-006 | D |

### 13.2 Grundsätze der Verifikation

- Jede Muss-Anforderung braucht mindestens einen zugeordneten Nachweis.
- Leistungsanforderungen werden **gemessen**, nicht eingeschätzt. Messgerät, Datenbestand und
  Messbedingungen sind zu protokollieren.
- Die Maßhaltigkeit des Drucks wird am **körperlichen Ausdruck** geprüft, nicht an der
  Bildschirmvorschau.
- Barrierefreiheitsanforderungen werden rechnerisch geprüft, nicht nach Augenmaß.
- Sicherheitsanforderungen werden durch Inspektion **und** durch einen Angriffsversuch geprüft,
  wo dies möglich ist.

### 13.3 Rückverfolgbarkeit

Für jede Anforderung ist über den gesamten Lebenszyklus nachzuweisen, wo sie umgesetzt und
wo sie geprüft wurde:

| Anforderung | Spezifikation | Umsetzung | Prüffall | Ergebnis |
|---|---|---|---|---|
| SM-xxx-nnn | Abschnitt im Pflichtenheft | Modul / Komponente | Prüffallkennung | bestanden / offen |

Die Matrix wird gepflegt, sobald das Pflichtenheft vorliegt. Eine Anforderung ohne
zugeordneten Prüffall gilt als nicht abgenommen.

---

## 14. Offene Punkte

> **Dieses Register ist die einzige Liste offener Punkte des Vorhabens.** DES-STM-001 und
> TEC-STM-001 führen keine eigene Nummerierung, sondern verweisen hierher.

### 14.1 Ungeklärte Punkte

| Nr | Frage | Auswirkung | Zu klären bis |
|---|---|---|---|
| **OP-01** | Bleibt es bei einer Einzelplatzanwendung, oder ist Synchronisation zwischen Geräten bzw. Mehrbenutzerbetrieb vorgesehen? | Grundlegend: entscheidet über Datenhaltung, Konfliktbehandlung, Rechtekonzept und einen erheblichen Teil des technischen Aufbaus. Alle Anforderungen dieses Dokuments sind als Einzelplatzfunktionen formuliert. | vor Beginn der Spezifikation |
| **OP-02** | Wird der gewerbliche Bereich (Kapitel 6.13) im ersten Stand umgesetzt oder zurückgestellt? | Er ist durchgehend mit **Kann** bewertet und über den Betriebsmodus abtrennbar. Eine Zurückstellung verkleinert den ersten Stand deutlich. | vor der Aufwandsschätzung |
| **OP-03** | Welche Zielformate muss die Konvertierung schreiben können? | Bestimmt den Aufwand für SM-EXP-002 erheblich. | vor der Spezifikation |
| **OP-04** | Wie wird mit der Paketsignatur umgegangen? Signaturzertifikate sind kostenpflichtige proprietäre Dienste ohne quelloffene Entsprechung. | Signatur als Infrastruktur akzeptieren — dann gelten SM-PLT-002 und SM-SEC-012 unverändert. Oder unsigniert ausliefern — dann warnt Windows beim Start, und macOS verweigert ihn ohne manuellen Eingriff. | vor dem ersten Auslieferungsbau |
| **OP-05** | Wird die Anbindung an einen entfernten KI-Dienst beibehalten, entfernt oder hinter einen Bau-Schalter gelegt? | SM-OSS-013 verlangt nur Abschaltbarkeit. Vollständiges Entfernen vereinfacht die Prüfung, streicht aber SM-KIA-003. | vor der Spezifikation |
| **OP-06** | Behält das Produkt den Namen „StitchManager", oder wird es unter der Marke Kreuznaht geführt? | Betrifft Anwendungskennung, Symbole und Paketnamen. Eine spätere Änderung der Anwendungskennung ist aufwendig. | vor dem ersten Auslieferungsbau |
| **OP-08** | Welches Gerät gilt als „mittlere Ausstattung" für die Messungen in SM-NFR-004 und SM-SRC-007? | Ohne festgelegte Referenz sind die Leistungsanforderungen nicht prüfbar. | vor der Prüfplanung |
| **OP-10** | Bleibt die Listenansicht als Alternative zur Kachelansicht bestehen (SM-PRV-006)? | Entfällt sie, wird der Umschalter gestrichen und SM-PRV-006 entfällt. | vor Umsetzungsbeginn der Oberfläche |
| **OP-11** | Erhält der Dunkelmodus ein eigenes Anwendungssymbol? | Terracotta auf Espresso trägt; ein farbiges Symbol möglicherweise nicht. | vor dem ersten Auslieferungsbau |
| **OP-12** | Soll die Übersichtskarte weitere Kennzahlen zeigen, etwa Lizenzstatus oder ungepflegte Muster? | Weitere Kennzahlen kosten Höhe, die dem Ordnerbaum fehlt. | vor Umsetzungsbeginn der Oberfläche |
| **OP-14** | Erfassen SM-FMT-012 und SM-SEC-011 mit der Formulierung „Alle Formatparser“ auch die Anzeigekomponente für Fremddokumente — PDF-Schnittmuster und Nähanleitungen —, oder ist dafür eine eigene Anforderung nötig? | SM-DOC-004 verlangt die Anzeige innerhalb der Anwendung; die dafür nötige Komponente liest Fremddaten wie ein Stickformatparser. Ob der Wortlaut „Alle Formatparser“ in SM-FMT-012 und SM-SEC-011 sie einschließt, ist dem Text nicht eindeutig zu entnehmen. Von der Antwort hängt ab, ob Absturzfreiheit, begrenzte Speicherbelegung und Verhalten bei manipulierten Dokumenten für diese Komponente einen Abnahmebezug haben. Aufgeworfen bei der Sicherheitsprüfung des Implementierungsplans. | vor der Spezifikation |
| **OP-18** | Welche Schwellenwerte gelten für Eingabelatenz (SM-NFR-002) und Entprellintervall (SM-SRC-008)? | Beide Anforderungen tragen die Prüfmethode **A** — Messung —, nennen aber keine Zahl. Eine Messung ohne Schwelle ist weder bestehbar noch durchfallbar; „bestanden“ wäre eine Einschätzung, die Abschnitt 13.2 ausschließt. Die einzige numerische Zeitzusage des Dokuments (SM-NFR-004, fünf Sekunden) betrifft die Startzeit. Aufgeworfen bei der Leistungsprüfung des Implementierungsplans. | vor der Prüfplanung |
| **OP-20** | Erhalten die Sammelaktionen der Hinweisbox für maschinell erzeugte Werte — „Alle übernehmen“ und „Alle verwerfen“ — eine eigene Anforderung? | DES-STM-001 Abschnitt 9 beschreibt beide verbindlich. SM-KIA-007 fordert ausdrücklich die Übernahme **jedes einzelnen** Ergebnisfelds, SM-KIA-008 und SM-DES-009 nur die Kennzeichnung. Ohne Kennung entsteht eine Bedienfunktion, der sich kein Prüffall zuordnen lässt (Abschnitt 13.2). Aufgeworfen bei der Prüfung des Implementierungsplans. | vor Umsetzungsbeginn der Oberfläche |
| **OP-21** | Deckt SM-PRV-002 („Vorschaubilder müssen dauerhaft zwischengespeichert werden“) auch eine **Obergrenze und ein Verdrängungsverfahren** des Zwischenspeichers, oder ist beides eine eigene Anforderung? | Ein Zwischenspeicher ohne Schranke wächst mit Bestandsgröße und Zahl der Fenstergrößen; bei 100.000 Einträgen und mehreren Auflösungsstufen je Eintrag berührt das SM-NFR-001. Der Wortlaut fordert Dauerhaftigkeit, nicht Beschränktheit — ohne Antwort ist die Bestehbedingung des zugehörigen Prüffalls nicht abnehmbar. Aufgeworfen bei der Leistungsprüfung des Implementierungsplans. | vor der Spezifikation |

### 14.2 Entschiedene Punkte

| Nr | Entscheidung vom 29.08.2026 | Folge |
|---|---|---|
| **OP-07** | Die Festbreitenschrift für Zahlen und Maße ist freigegeben. | DES-STM-001 verwendet IBM Plex Mono wie beschrieben. |
| **OP-09** | Die rekonstruierten Farbwerte sind noch nicht gegen den Markenstandard bestätigt. | Entwickelt wird ausschließlich gegen die stabilen Variablennamen. Die Werte bleiben vorläufig; eine visuelle Markenabnahme ist erst nach dem Abgleich zulässig. |
| **OP-13** | Weg A ist bestätigt: Der vorhandene Rust-Kern wird wiederverwendet und über cxx-qt an Qt 6 angebunden. | TEC-STM-001 ist als Technologieentscheidung bestätigt; der übernommene Code besitzt keinen Bestandsschutz gegen Anforderungen. |
| **OP-15** | Leer-, Lade-, Trefferlos-, Fehler- und Abbruchzustände erhalten einen eigenen Abnahmebezug. | Neu: SM-NFR-015; die bisherigen vorläufigen Prüffälle werden PF-NFR-15-Unterfälle. |
| **OP-16** | Zustandsbelegung und Mindesttrefferflächen erhalten einen eigenen Abnahmebezug. Trenner bleiben 1 px sichtbar und mindestens 6 px breit ziehbar, besitzen aber eine 32 px breite unsichtbare Trefferzone. | Neu: SM-NFR-016; der scheinbare Maßwiderspruch ist aufgelöst. |
| **OP-17** | Die Übersichtskarte erhält eine eigene Anforderung. | Neu: SM-LIB-011; OP-12 entscheidet weiterhin nur über zusätzliche Kennzahlen. |
| **OP-19** | Die manuelle Wahl der Darstellungsart muss einen Neustart überstehen. | SM-SET-002 ist präzisiert; kein neuer, danebenliegender Zustandsbegriff wird eingeführt. |

---

## 15. Freigabe

| Rolle | Name | Datum | Unterschrift |
|---|---|---|---|
| Erstellt | | | |
| Fachlich geprüft | | | |
| Technisch geprüft | | | |
| Freigegeben | | | |

Mit der Freigabe gilt der Anforderungsumfang als vereinbart. Änderungen nach der Freigabe
erfolgen über eine neue Version dieses Dokuments mit Eintrag in der Änderungshistorie.

---

## 16. Änderungshistorie

| Version | Datum | Autor | Änderung |
|---|---|---|---|
| 1.4 | 29.08.2026 | Auftraggeber | OP-07, OP-09, OP-13, OP-15, OP-16, OP-17 und OP-19 entschieden. Festbreitenschrift und Weg A mit Wiederverwendung des Rust-Kerns/cxx-qt bestätigt; Farbwerte bleiben bis zum Markenabgleich vorläufig. Neu aufgenommen: SM-LIB-011 (Übersichtskarte), SM-NFR-015 (Leer-, Lade-, Fehler- und Abbruchzustände) und SM-NFR-016 (Komponentenzustände und Trefferflächen). SM-SET-002 um die Neustartbeständigkeit der manuellen Darstellungswahl präzisiert. DES-STM-001 v1.4 und TEC-STM-001 v2.3 nachgezogen. Nachweis: `Analysis/20260829_01_sprintplanung.md`. |
| 1.3 | 24.08.2026 | | Acht neue offene Punkte: **OP-14** (Geltung von SM-FMT-012 und SM-SEC-011 für die Anzeigekomponente von Fremddokumenten), **OP-15** (leere, ladende und fehlerhafte Zustände), **OP-16** (Zustandstabelle und Mindestgrößen von Bedienelementen), **OP-17** (Übersichtskarte der Navigationsspalte) **OP-18** (fehlende Schwellenwerte für zwei Messanforderungen), **OP-19** (Fortbestehen der Modus-Wahl über den Neustart) **OP-20** (Sammelaktionen der Hinweisbox für maschinell erzeugte Werte) und **OP-21** (Obergrenze und Verdrängung des Vorschau-Zwischenspeichers). Alle acht gehen auf Befunde der Stufe-1-Prüfung des Implementierungsplans zurück und benennen Stellen, an denen dieses Dokument eine Lücke lässt: fünf, an denen DES-STM-001 Verhalten verbindlich beschreibt, ohne dass hier eine Kennung dafür steht (OP-14 bis OP-17, OP-20); eine, an der zwei Messanforderungen ohne Schwellenwert bleiben (OP-18); und eine, an der eine Zusage zwischen zwei bestehende Kennungen fällt (OP-19). Sichtbar wurden sie erst bei der Verplanung. An den bestehenden Anforderungen ändert sich nichts. Ergänzt: IMP-STM-001 in Abschnitt 1.5. Nachweis: `Analysis/20260823_03_implementierungsplan.md`. |
| 1.0 | 23.08.2026 | | Erstfassung. Abgeleitet aus ANA-STM-001 v1.1, umgestellt auf ein eigenständiges Lastenheft ohne Bezug auf Vorgängerstände. Ergänzt: Prüfmethode je Anforderung, Datenobjekte, Betriebsmodi, Verifikationskonzept, Rückverfolgbarkeit, Freigabe. |
| 1.2 | 23.08.2026 | | Verweise auf DES-STM-001 v1.2 und TEC-STM-001 v2.2 nachgezogen. Anlass ist eine Farbkorrektur in DES-STM-001 (weiße Schrift auf Terracotta verfehlte SM-NFR-007); an den Anforderungen dieses Dokuments ändert sich nichts. Nachweis: `Analysis/20260823_01_gate-befunde-rueckstand.md`. |
| 1.1 | 23.08.2026 | | Abstimmung mit DES-STM-001 und TEC-STM-001. Neues Kapitel 6.14 (SM-MAC-001 bis 004); SM-MFG-008/009 dorthin verschoben und gestrichen, weil sie in beiden Betriebsmodi gelten. Offene Punkte OP-09 bis OP-12 aus der Design-Beschreibung übernommen; das Register ist jetzt die einzige Liste des Vorhabens. Dokumentenhierarchie und Regel für gestrichene Kennungen ergänzt. Nachweis: ABG-STM-001. |
