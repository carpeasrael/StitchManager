# StitchManager — Implementierungsplan

**Kennung:** IMP-STM-001
**Version:** 1.1
**Datum:** 29.08.2026
**Führendes Dokument:** URS-STM-001 (Lastenheft) v1.4
**Mitgeltend:** DES-STM-001 (Design-Beschreibung) v1.4 · TEC-STM-001 (Tech-Stack) v2.3
**Status:** Entwurf — zur Prüfung und Freigabe
**Nachweis der Herleitung:** `Analysis/20260823_03_implementierungsplan.md`

---

## 0. Zweck, Geltung und Verhältnis zu den drei Dokumenten

Dieses Dokument ordnet die Anforderungen des Lastenhefts in eine Umsetzungsreihenfolge und weist
je Anforderung aus, in welchem Arbeitspaket sie entsteht und an welchem Prüffall sie nachgewiesen
wird. Es beantwortet die Frage **wann und woran**, nachdem URS-STM-001 das **was**, DES-STM-001
das **wie es aussieht** und TEC-STM-001 das **womit** beantwortet hat.

### 0.1 Rangfolge

1. **URS-STM-001** — was das System leisten muss 2. **DES-STM-001** — wie es aussieht und sich
verhält 3. **TEC-STM-001** — womit es gebaut wird 4. `Design/stitchmanager-mockup.html` — visuelle
Referenz 5. **Dieses Dokument** — in welcher Reihenfolge und mit welchem Nachweis

Der Plan steht **hinter** allen Quellen, auch hinter dem Mockup: Er ist keine Anforderungsquelle.
Dieselbe Rangfolge steht in `CLAUDE.md` Abschnitt 3.

**Dieses Dokument begründet keine Anforderung.** Ergibt sich beim Planen eine, wird sie im
Lastenheft aufgenommen und hier nur referenziert. Ein Arbeitspaket, das eine Leistung enthielte,
für die keine Kennung existiert, ist ein Planungsfehler und kein Zusatzumfang. Acht bei der Prüfung
dieses Plans aufgeworfene Lücken gingen deshalb zunächst als **OP-14** bis **OP-21** ins
Lastenheft. URS-STM-001 v1.4 hat OP-15 bis OP-17 entschieden und mit SM-NFR-015, SM-NFR-016
und SM-LIB-011 geschlossen; OP-19 ist in SM-SET-002 präzisiert.

**Der Plan schränkt auch keine Anforderung ein.** Wo der Wortlaut einer Anforderung weiter reicht,
als ein Arbeitspaket abdeckt, wird die Weite benannt und die Lücke als offener Punkt geführt —
nicht der Wortlaut enger gelesen.

**Dieses Dokument führt kein eigenes Register offener Punkte.** Alle offenen Punkte stehen in
Kapitel 14 des Lastenhefts; Kapitel 2 ordnet sie den Arbeitspaketen zu, mehr nicht.

### 0.2 Eintragung in das führende Dokument

IMP-STM-001 ist in Abschnitt 1.5 des Lastenhefts als mitgeltende Unterlage geführt, ausdrücklich
als **nachgeordnet** und mit dem Zusatz, dass es keine Anforderung begründet oder beschränkt.

Ursprünglich war die Nichtaufnahme damit begründet, dass sie eine Versionserhöhung des Lastenhefts
samt Kopfzeilennachzug in DES-STM-001 und TEC-STM-001 auslösen würde. Diese Kosten sind mit
URS-STM-001 v1.4 ohnehin angefallen (OP-14 bis OP-20); die Begründung trug damit nicht mehr und
wurde fallen gelassen.

### 0.3 Was dieses Dokument nicht ist

Es ist **kein Pflichtenheft.** URS-STM-001 Abschnitt 7 und 13.3 verweisen an mehreren Stellen auf
ein Pflichtenheft; dieses entsteht später und tritt zwischen Lastenheft und Plan.

**Die Grenze zwischen beiden ist prüfbar zu ziehen, sonst wandert sie.** Ohne festen Maßstab zieht
jede Prüfrunde weitere Festlegungen in den Plan, bis er das Pflichtenheft ist, das er nach eigener
Aussage nicht sein will. Der Maßstab lautet:

| Der Plan **ordnet zu** | Das Pflichtenheft **legt fest** |
|---|---|
| welche Anforderung in welchem Arbeitspaket entsteht | wie sie umgesetzt wird — Datenstrukturen, Algorithmen, Schnittstellensignaturen |
| an welchem Prüffall sie nachgewiesen wird | wie der Prüffall aufgebaut ist — Testdaten im Einzelnen, Abfrageformen, Randfälle |
| **welche Größe** gemessen wird und **wogegen** | **welcher Zahlenwert** die Schwelle ist, sofern das Lastenheft ihn nicht nennt |
| in welcher Reihenfolge und unter welcher Vorbedingung | mit welchen Merkmalen ein Prüfbestand erzeugt wird — Verteilungen, Anteile, Vokabular |

**Entscheidungsregel für Befunde:** Verlangt ein Befund eine Zuordnung, eine Reihenfolge, eine
Vorbedingung oder die *Benennung* einer Messgröße, gehört er in den Plan und wird dort behoben.
Verlangt er einen Zahlenwert, ein Datenformat, ein Verfahren oder die Merkmale eines Prüfbestands,
ist er eine **Pflichtenheft-Vormerkung** (Kapitel 12) — er wird dort namentlich festgehalten,
nicht im Plan aufgelöst. Ein Befund, der beides verlangt, wird geteilt.

Diese Grenze ist keine Abwehr: Eine Vormerkung ist eine übernommene Verpflichtung mit Adressat,
kein abgelehnter Befund.

Es ist **kein Nachweis.** Die Ergebnisspalte der Rückverfolgbarkeitsmatrix in Kapitel 7 ist
durchgehend offen. Ein Plan, der Ergebnisse einträgt, bevor gemessen wurde, entwertet die Matrix,
die er anlegt.

Es ist **keine Aufwandsschätzung.** TEC-STM-001 Abschnitt 6 bindet eine belastbare Schätzung an
die Entscheidung zwischen den beiden Anbindungswegen; diese Entscheidung steht aus.

---

## 1. Umfang der Version 1.0

### 1.1 Verplanter Umfang

| Priorität | Im Lastenheft | In Version 1.0 verplant | Zurückgestellt |
|---|---|---|---|
| **M — Muss** | 129 | **129** | 0 |
| **S — Soll** | 79 | **11** (nachgezogen, Abschnitt 1.2) | 68 |
| K — Kann | 17 | 0 | 17 |
| **Summe** | 225 | **140** | 85 |

Der Regelumfang ist **Muss**. URS-STM-001 Abschnitt 1.4 verlangt für die Streichung von
Soll-Anforderungen eine Begründung und deren Dokumentation; sie steht in Kapitel 10.

**Zurückgestellt heißt nicht entfallen.** Die Kennungen bleiben gültig, werden nicht neu vergeben
und sind nicht als gestrichen zu vermerken. Sie sind Gegenstand einer späteren Version dieses
Plans, nicht einer Änderung des Lastenhefts.

### 1.2 Elf nachgezogene Soll-Anforderungen

Elf Soll-Anforderungen sind in den Umfang genommen, weil ohne sie eine **Muss**-Zusage ins Leere
läuft, ein verbindlicher Gestaltungsgrundsatz außer Kraft steht oder der Plan sonst eine
Anforderung enger auslegen müsste, als sie geschrieben ist. Die Aufnahme ist eine
Umfangsentscheidung, keine Neubewertung der Priorität — im Lastenheft bleiben sie Soll.

| Kennung | Warum nachgezogen |
|---|---|
| **SM-SRC-009** | Ohne die Chip-Darstellung mit Entfernen-Kreuz gibt es in Version 1.0 **keinen Bedienweg, einen gesetzten Filter wieder aufzuheben**. SM-SRC-003 und SM-SRC-010 (beide Muss) sagen Filterung zu; eine Filterung ohne Rückweg ist keine erfüllte Zusage |
| **SM-BAT-005** | SM-BAT-007 (Muss) sichert zu, dass ein **abgebrochener** Stapelvorgang keinen halb geänderten Zustand hinterlässt. Ohne Fortschritt und Abbruchmöglichkeit gibt es kein Bedienelement, das den Abbruch auslöst; der Prüffall wäre nur über einen Prozessabbruch erreichbar und deckte die Anforderung nicht |
| **SM-NFR-009** | DES-STM-001 Abschnitt 2 legt verbindlich fest: „Farbe trägt nie allein." Ohne SM-NFR-009 dürften Auswahl-, Fehler- und Erfolgszustände allein farblich unterschieden werden. Das zweite Zustandsmerkmal entsteht einmalig in der Gestaltungsgrundlage und ist dort billiger als jede spätere Nachrüstung |
| **SM-NFR-013** | DES-STM-001 Abschnitt 8 sagt zur reduzierten Bewegung „**alle** Übergänge … Keine Ausnahme". AP-11 baut Themenwechsel und Zustandsübergänge; ohne die Abfrage der Systemeinstellung hätten bewegungsempfindliche Nutzer keinen Ausweg. Macht zugleich Prüfpunkt D-07 wieder abnehmbar |
| **SM-FMT-010**, **SM-MET-010** | SM-DES-008 (Muss) verlangt den Detailabschnitt „**Farben**", den DES-STM-001 Abschnitt 6.4 als Liste mit Farbfeld, Garnname, Garnnummer und Stichanteil festlegt. Ohne diese beiden Kennungen hätte der Abschnitt dauerhaft nur Farbanzahl und Farbwechsel — eine **Muss**-Anforderung bliebe auf Dauer teilweise unerfüllt. Für Soll-Streichungen sieht URS-STM-001 Abschnitt 1.4 diesen Weg vor, für Muss-Anforderungen nicht |
| **SM-SET-002**, **SM-SET-003** | SM-SET-001 fordert nur, dass beide Modi **angeboten** werden — die Übernahme der Systemeinstellung samt neustartbeständiger manueller Übersteuerung liegt nach der Entscheidung zu OP-19 in SM-SET-002, Panelbreiten und Fensterzustand in SM-SET-003. Beide Leistungen ohne ihre Kennung zu führen hieße, SM-SET-001 weiter auszulegen, als sie geschrieben ist (Abschnitt 0.1) |
| **SM-KIA-001** | SM-KIA-007 (Muss, Feldübernahme), SM-KIA-008 (Muss, Kennzeichnung) und SM-DES-009 (Muss, Unterscheidbarkeit) setzen voraus, dass **Vorschläge** entstehen. SM-KIA-002 fordert nur die lokale Verarbeitung, nicht die Erzeugung von Metadatenvorschlägen aus dem Vorschaubild — das ist SM-KIA-001. Ohne diese Kennung hinge die gesamte Kennzeichnungs- und Übernahmekette an einer Auslegung von SM-KIA-002, die dem Plan nicht zusteht (Abschnitt 0.1) |
| **SM-SET-004** | SM-NFR-008 (Muss) sichert die vollständige Tastaturbedienung zu. Für Nutzer, die ausschließlich mit der Tastatur arbeiten, ist ohne Kürzel jede häufige Aktion nur über Fokuswanderung erreichbar — Erreichbarkeit ohne Kürzel erfüllt den Buchstaben, nicht den Zweck. Die Zurückstellung wäre zudem schwächer begründet als die der bereits nachgezogenen Kennungen |
| **SM-SEC-010** | SM-NFR-006 (Muss) schreibt technische Angaben ausschließlich ins Protokoll — also die vollständigen Bibliothekspfade. Die Zurückstellung wäre nur haltbar, wenn „exportierbar" eng als „durch eine Ausleitfunktion der Anwendung" gelesen würde; eine Protokolldatei im Anwendungsverzeichnis ist aber faktisch exportierbar, sobald ein Nutzer sie einer Supportanfrage anhängt. Diese Auslegung steht dem Plan nicht zu (Abschnitt 0.1). Die Maskierung im Protokollschreiber ist eine einmalige Funktion |

### 1.3 Was dennoch nicht entsteht — Reibungsstellen zum Zielzustand

DES-STM-001 beschreibt den Zielzustand der Oberfläche. An den folgenden Stellen hängt
beschriebenes Verhalten an einer zurückgestellten Soll-Anforderung. Die Liste ist nach der
Stufe-1-Prüfung erweitert worden und erhebt keinen Anspruch auf Vollständigkeit; sie wird mit
jeder Umfangsänderung fortgeschrieben.

| Vorgabe in DES-STM-001 | Anforderung | Folge für den Nutzer |
|---|---|---|
| Abschnitt 6.4, Zoom-Schalter der Detailvorschau | SM-PRV-004 | Die Vorschau im Detailbereich ist nicht vergrößerbar |
| Abschnitt 6.3, Umschalter Kachel- und Listenansicht | SM-PRV-006, zusätzlich OP-10 (DES-STM-001 Abschnitt 13) | Nur Kachelansicht; der Umschalter entfällt in der Werkzeugleiste |
| Abschnitt 6.4, Rahmenprüfung gegen das Stickfeld | SM-MAC-002 | Die Hinweisbox entsteht nicht; kein Hinweis, ob ein Muster in den Rahmen passt |
| Abschnitt 6.2, Navigationsgruppen | SM-LIB-005 bis SM-LIB-007, SM-PRJ-001 | Die Gruppen „Intelligente Ordner“ (SM-LIB-006) und „Arbeit“ (SM-PRJ-001) entstehen nicht; Sammlungen und Favoriten (SM-LIB-005, SM-LIB-007) erscheinen dort ohnehin nicht als eigene Gruppe, sondern als Einträge der Gruppe „Bibliothek“ |
| Abschnitt 10, Zustand „Datei nicht auffindbar“ mit Schaltfläche „Neu verknüpfen“ | SM-EXP-010 | Die Schaltfläche entfällt; die Abhilfe steht nur als Text (AP-12, Wortlaut nach PV-06) |
| Abschnitt 6.4, Abschnittsfolge des Detailbereichs endet mit „Projekte" | SM-PRJ-001 | Der letzte Abschnitt der fest geordneten Folge entfällt |
| Abschnitt 10, Leerzustand, Trefferlosigkeit und Abbruch langer Vorgänge | SM-NFR-015 | AP-12 bis AP-18 bauen und prüfen die Zustände; die früheren vorläufigen `PF-OP15-*` sind PF-NFR-15-Unterfälle |
| Abschnitt 7, Zustandstabelle und Mindestgrößen von Bedienelementen | SM-NFR-016 | AP-12 weist Zustände und Trefferflächen nach; Trenner besitzen eine 32 px breite unsichtbare Trefferzone um die schmale sichtbare Linie |
| Abschnitt 6.2, Übersichtskarte der Navigationsspalte | SM-LIB-011 | AP-12 setzt Gesamtbestand und Formatverteilung um; OP-12 betrifft nur zusätzliche Kennzahlen |

### 1.4 Vollständigkeit der Zuordnung

Jede der 140 verplanten Anforderungen ist **genau einem** Arbeitspaket zugeordnet. Mehrfache
Zuordnung ist unzulässig: Sie verteilt die Verantwortung und macht den Abschluss eines Pakets
unentscheidbar.

Wo eine Anforderung mehrere Pakete berührt, trägt sie das Paket, **in dem ihr Nachweis tatsächlich
fällt**; die übrigen nennen sie als **Mitwirkung**, ohne sie zu übernehmen. Diese Regel ist der
Grund, warum Oberflächenkennungen wie die Löschbestätigung (SM-DAT-003) oder die
Fortschrittsanzeige des Imports (SM-IMP-002) im Oberflächenpaket stehen und nicht in dem
Kernpaket, das ihre Fachlogik liefert.

---

## 2. Entscheidungsgatter — was vor dem Bauen zu klären ist

Dieses Kapitel entscheidet **keinen** offenen Punkt. Es weist je Punkt aus, welche Arbeitspakete
auf eine Antwort warten und wie die Antwort herbeigeführt wird. Die Einordnung folgt der
Wirkungstabelle aus `CLAUDE.md` Abschnitt 8.

### 2.1 Punkte, die vor dem Beginn zu beantworten sind

**Am 29.08.2026 beantwortet:** OP-13 und die Wegentscheidung bestätigen die Wiederverwendung des
Rust-Kerns mit Weg A/cxx-qt. OP-07 gibt die Festbreitenschrift frei. OP-09 hält die Farbwerte bis
zum Markenabgleich vorläufig. OP-15 bis OP-17 erzeugen SM-NFR-015, SM-NFR-016 und SM-LIB-011;
OP-19 präzisiert SM-SET-002. Diese Punkte sperren AP-11 bis AP-18 nicht mehr.

| Nr | Frage in Kurzform | Wartende Arbeitspakete | Klärungsweg |
|---|---|---|---|
| **OP-13** | Wiederverwendung des vorhandenen Rust-Kerns oder Neuentwicklung? | **beantwortet; keine wartenden Pakete** | Weg A: Rust-Kern wiederverwenden, Anbindung über cxx-qt |
| **OP-07** | Festbreitenschrift für Zahlen und Maße freigegeben? | **beantwortet; keine wartenden Pakete** | freigegeben |
| **OP-09** | Rekonstruierte Farbwerte gegen den Markenstandard abgeglichen? | **beantwortet; keine wartenden Pakete** | noch nicht abgeglichen; Werte bleiben bis zur visuellen Markenabnahme vorläufig |
| **OP-15** | Eigene Anforderung für Zustände und Abbruch? | **beantwortet; keine wartenden Pakete** | SM-NFR-015 aufgenommen |
| **OP-16** | Eigene Anforderung für Zustände und Mindestgrößen? | **beantwortet; keine wartenden Pakete** | SM-NFR-016 aufgenommen; Trenner mit 32-px-Trefferzone |
| **OP-17** | Eigene Anforderung für die Übersichtskarte? | **beantwortet; keine wartenden Pakete** | SM-LIB-011 aufgenommen |
| **OP-19** | Übersteht die manuelle Darstellungswahl den Neustart? | **beantwortet; keine wartenden Pakete** | ja; SM-SET-002 präzisiert |
| **OP-01** | Einzelplatz oder Synchronisation zwischen Geräten? | AP-04, AP-05 | Entscheidung des Auftraggebers. Bei Bejahung entstehen neue Anforderungen im Lastenheft; dieser Plan verplant ausschließlich die Einzelplatzfunktionen |
| **OP-20** | Eigene Anforderung für die Sammelaktionen der Hinweisbox? | AP-18 | Entscheidung des Auftraggebers |

OP-01 und OP-20 bleiben hier, weil ihre Fristen vor dem jeweiligen Umsetzungsbeginn liegen.
AP-04/AP-05 arbeiten bis OP-01 unter der ausdrücklichen Einzelplatzannahme; AP-18 beginnt nicht,
bevor OP-20 entschieden ist.

### 2.2 Punkte mit Termin, aber ohne sofortige Sperre

| Nr | Frage in Kurzform | Vorbedingung für | Wirkung, wenn offen |
|---|---|---|---|
| **OP-08** | Welches Gerät gilt als mittlere Ausstattung? | **AP-07, AP-10, AP-12** (deren Erstnachweise) und AP-22 | **Die Frist des Lastenhefts ist mit diesem Dokument erreicht** — OP-08 ist „vor der Prüfplanung" zu klären, und Kapitel 6 *ist* die Prüfplanung. Kapitel 6 gilt bis zur Beantwortung als **vorläufig**. Die Erstnachweise in AP-07, AP-10 und AP-12 werden ohne Referenzgerät als **Regressionsschwelle** geführt, nicht als Abnahme; die Abnahme fällt in AP-22. Ohne diese Unterscheidung protokollierte M2 einen Wert als *bestanden*, für den URS-STM-001 Abschnitt 13.2 Gerät, Datenbestand und Bedingungen verlangt |
| **OP-03** | Zielformate des Schreibpfads? | AP-16 | SM-EXP-001 (Muss) verlangt ein wählbares Zielformat, nicht welches. Die Formatliste ist Eingang, nicht Inhalt des Pakets |
| **OP-05** | Entfernte KI beibehalten, entfernen oder hinter einen Bau-Schalter? | AP-18 | Verplant ist allein die lokale Verarbeitung (SM-KIA-002). Die Schnittstelle bleibt so geschnitten, dass beide Antworten möglich bleiben |
| **OP-21** | Deckt SM-PRV-002 auch Obergrenze und Verdrängung des Vorschau-Zwischenspeichers? | AP-09 | Entscheidung des Auftraggebers; bis dahin trägt PF-PRV-02 die Bestehbedingung ohne Zahlenwert (PV-09) |
| **OP-14** | Erfassen SM-FMT-012 und SM-SEC-011 die Anzeigekomponente für Fremddokumente? | AP-14 | AP-14 härtet und fuzzt die Anzeige unabhängig von der Antwort. Die Antwort entscheidet nur, ob der Nachweis unmittelbar auf SM-SEC-011 zahlt oder auf eine neue Kennung |
| **OP-18** | Welche Schwellenwerte gelten für Eingabelatenz und Entprellintervall? | AP-12, AP-22 | SM-NFR-002 und SM-SRC-008 tragen die Prüfmethode A, aber keine Zahl. Eine Messung ohne Schwelle ist weder bestehbar noch durchfallbar; die Messgrößen sind in Abschnitt 6.4 benannt, die Schwellen fehlen |
| **OP-04** | Umgang mit der Paketsignatur? | AP-21 | SM-PLT-002 und SM-SEC-012 stehen selbst unter dem Vorbehalt „sofern die Signaturinfrastruktur bereitsteht"; der Plan gibt ihn unverändert weiter |
| **OP-06** | Produktname und Anwendungskennung? | AP-21 | Eine spätere Änderung der Anwendungskennung ist aufwendig; vor dem ersten Auslieferungsbau zu klären |
| **OP-11** | Eigenes Anwendungssymbol im Dunkelmodus? | AP-21 | Betrifft die Ikonografie, kein Bauteil |

### 2.3 Punkte ohne Bezug zu Version 1.0

**OP-02** (gewerblicher Bereich) und **OP-12** (Kennzahlen der Übersichtskarte) betreffen
ausschließlich Umfang, der in Version 1.0 nicht verplant ist. **OP-10** (Listenansicht) ebenso,
mit der in Abschnitt 1.3 benannten Folge.

### 2.4 Die Gatterregel

**Kein Arbeitspaket beginnt, dessen Vorbedingung ein ungeklärter Punkt aus Abschnitt 2.1 ist.**
Punkte aus Abschnitt 2.2 sperren das Paket nicht, sondern seinen Abschluss: Das Paket darf gebaut
werden, gilt aber erst als fertig, wenn die Antwort vorliegt und eingearbeitet ist. Diese
Unterscheidung ist die Umsetzung von `CLAUDE.md` Abschnitt 8 — sie verhindert gleichermaßen, dass
ein offener Punkt stillschweigend im Code entschieden wird, und dass einundzwanzig offene
Punkte jede
Arbeit anhalten.

---

## 3. Zielarchitektur und Modulschnitt

Aufbauend auf TEC-STM-001 Abschnitt 3, dort um `kern/fassade` als Träger von SM-SEC-004, die
Oberflächenmodule und die Querschnitte `bau` und `pruef` erweitert. Die Modulnamen sind **logisch,
nicht als Pfade zu lesen**: Sie gelten unter Weg A wie unter Weg B und nehmen die Entscheidung aus
Abschnitt 2.1 nicht vorweg.

```text
┌──────────────────────────────────────────────────────────────┐
│  Oberflächenschicht                                          │
│  gestaltung · fenster · navigation · auswahl · detail ·      │
│  dialoge · druck                                             │
└───────────────────────────┬──────────────────────────────────┘
                            │  ausschließlich über die Fassade
┌───────────────────────────┴──────────────────────────────────┐
│  Kernschicht                                                 │
│  fassade · parsers · writers · render · db · services ·      │
│  security                                                    │
└──────────────────────────────────────────────────────────────┘
   quer:  bau (Paketierung, Prüfkette)   pruef (Prüffälle, Messungen)
```

| Modul | Verantwortung | Wesentliche Kennungen |
|---|---|---|
| `kern/fassade` | einzige Zugriffsschicht der Oberfläche auf den Kern, **ausschnittweise Lieferung von Treffermengen**, Herkunftsführung der Werte | SM-SEC-004, SM-DTA-001 |
| `kern/security` | Pfadprüfung, Kanonisierung, Eingrenzung auf das jeweils gültige Wurzelverzeichnis, Bereinigung von Fremddatennamen | SM-SEC-001 bis 003, SM-NFR-010 |
| `kern/db` | SQLite mit WAL, additive Migrationen, Volltextindex, parametrisierte Abfragen | SM-DAT-006 bis 008, SM-SEC-005, SM-SRC-001 bis 010 |
| `kern/parsers` | Formate lesen, Kennwerte, Härtung gegen manipulierte Dateien | SM-FMT-001 bis 013, SM-SEC-011 |
| `kern/writers` | Konvertierung in ein wählbares Zielformat | SM-EXP-001 |
| `kern/render` | Vorschau aus Stichdaten, Zwischenspeicher und dessen Verwerfung | SM-PRV-001 bis 003 |
| `kern/services` | Import, Datenträgererkennung, Sicherung, Stapelvorgänge, lokale Analyse, Schlüsselablage, Protokollschreiber | SM-IMP-*, SM-EXP-004 bis 007, SM-BAT-*, SM-KIA-*, SM-DAT-001, SM-SEC-010, SM-MIG-005 |
| `ui/gestaltung` | **eine** Datei als einzige Quelle der Farb-, Schrift- und Abstandsbezeichner; Themen samt Persistenz, Fokusring, Tastaturkürzel, Bewegung, zweites Zustandsmerkmal | SM-DES-001 bis 004, SM-SET-001 bis 004, SM-NFR-007 bis 009, SM-NFR-013 · Mitwirkung SM-DES-009 |
| `ui/fenster` | dreispaltiges Hauptfenster, Trenner, Werkzeug- und Statusleiste | SM-DES-005, SM-DES-006, SM-IMP-002, SM-BAT-005 |
| `ui/navigation` | linke Spalte: Übersichtskarte, Ordnerbaum, Gruppen | SM-LIB-002 · Mitwirkung SM-SEC-008 (Ordnernamen sind Fremdtext) |
| `ui/auswahl` | mittlere Spalte: Kopfzeile mit Filterchips, Trefferzahl und Sortiersteuerung, virtualisiertes Kachelraster, Zustände nach DES-STM-001 Abschnitt 10, Nur-Text-Darstellung von Fremdtext | SM-DES-007, SM-PRV-007, SM-PRV-009, SM-SRC-008, SM-SRC-009, SM-NFR-003, SM-SEC-008 |
| `ui/detail` | rechte Spalte: Detailbereich, Metadatenpflege, Farb- und Garnliste | SM-DES-008, SM-MET-*, SM-DOC-004 · Mitwirkung SM-SEC-008, SM-DES-009 |
| `ui/dialoge` | Dialoggerüst mit Fokusfang und Fokusrückgabe; Bestätigungen, Vorschauen, Konfliktdialoge | SM-DAT-003, SM-BAT-004, SM-EXP-006, SM-KIA-004, SM-KIA-005 · Mitwirkung SM-MET-009, SM-IMP-005 |
| `ui/druck` | Druckvorschau, maßhaltige Ausgabe ohne Themenfarben | SM-PRN-001 bis 015 |
| `bau` | Prüfkette, Lizenzprüfung, Paketierung je Plattform | SM-OSS-*, SM-PLT-*, SM-NFR-012 |
| `pruef` | Prüffälle, Fuzzing-Ziele, erzeugter Prüfbestand, Messaufbauten | Nachweise zu allen Kennungen |

**Fünf Schnittregeln, die nicht verhandelbar sind:**

1. **Die Oberfläche greift nie unmittelbar auf `kern/db` zu.** Jeder Zugriff läuft über
   `kern/fassade` (SM-SEC-004). Diese Regel ist mit dem ersten Oberflächenmodul durchzusetzen;
   nachträglich ist sie praktisch nicht mehr einziehbar.
2. **Kein Farb-, Schrift- oder Abstandswert steht außerhalb von `ui/gestaltung`** (SM-DES-003,
   Prüfpunkt D-05). Die automatisierte Prüfung dazu entsteht in AP-02, also **bevor** das erste
   Oberflächenmodul geschrieben wird.
3. **Die Fassade liefert Treffermengen nie vollständig.** Ausschnitt plus Gesamtzahl; ein
   vollständiger Ergebnistransfer ist unzulässig. **Die Gesamtzahl wird einmal je Suchlauf
   ermittelt und mit dem ersten Ausschnitt geliefert; Folgeausschnitte tragen sie nicht erneut** —
   sonst läuft je Bildlaufabschnitt eine Zählabfrage über die volle Treffermenge. Ohne diese Regel
   endet die Virtualisierung an der Schnittstelle: Ein Suchergebnis über 100.000 Einträge würde je
   Suchlauf vollständig übertragen und materialisiert, SM-SRC-007 fiele an der Schnittstelle statt
   im Index, und SM-NFR-003 wäre faktisch durch den Speicherbedarf des Anzeigemodells begrenzt.
   **Die Form des Ausschnitts — Cursor oder Versatz — legt das Pflichtenheft fest (PV-07);** der
   Plan bindet nur ihre Kosten an eine Messgröße, weil versatzbasiertes Blättern in SQLite mit dem
   Versatz wächst und am Ende einer Treffermenge über 100.000 Einträge teurer wird als am Anfang.
4. **Ein Fassadenaufruf beantwortet eine Ansicht mit einer von der Zeilenzahl unabhängigen Zahl
   von Abfragen.** Garnfarben, angehängte Dateien und Schlagworte werden mit dem Ausschnitt
   geliefert, nicht je Zeile nachgefragt — sonst entsteht genau die N+1-Abfrage, die SM-SRC-007
   bei 100.000 Einträgen reißt. Gemessen in PF-PRV-07.1 als Zahl der Fassadenaufrufe je
   Bildlaufabschnitt.
5. **Jeder Schreibvorgang läuft durch `kern/security`** — gegen die Bibliothekswurzel, das
   Exportziel oder das Sicherungsziel, je nachdem, welches gilt. Ein Modul, das selbst einen Pfad
   zusammensetzt, ist ein Befund (SM-SEC-001, SM-SEC-002).

---

## 4. Arbeitspakete

Der Schnitt ist **fachlich, nicht technologisch**. Jedes Paket ist unter beiden Antworten auf
OP-13 und unter beiden Anbindungswegen dasselbe Paket; die Antwort ändert den Aufwand innerhalb
eines Pakets, nie seinen Gegenstand.

### 4.1 Wann ein Paket abgeschlossen ist

Ein Paket ist abgeschlossen, wenn jede ihm **zugeordnete** Anforderung ihren **Erstnachweis**
bestanden hat — nicht, wenn der Code geschrieben ist. Kennungen, die ein Paket nur als
**Mitwirkung** nennt, binden seinen Abschluss nicht.

**Erstnachweis und Wiederholung sind zu unterscheiden.** Ohne diese Trennung entstünde ein
Zirkelschluss: Die messenden Nachweise lägen in AP-22, AP-22 setzte AP-21 voraus, AP-21 alle
vorstehenden Pakete — und kein Paket mit einer Anforderung der Prüfmethode A könnte je schließen.

| Begriff | Wo | Wirkung |
|---|---|---|
| **Erstnachweis** | im Fachpaket selbst, gegen den erzeugten Prüfbestand aus AP-02 | schließt das Paket und den Meilenstein |
| **Wiederholung** | in AP-22, gegen den Auslieferungsstand und das Referenzgerät nach OP-08 | Veröffentlichungsnachweis, je Veröffentlichung erneut |
| **Regressionslauf** | in jedem folgenden Oberflächenpaket | für die querschnittlichen Zusagen, siehe Abschnitt 4.2 |

**Geteilte Erstnachweise binden nur ihren eigenen Teil.** Fällt ein Unterfall in einem anderen
Paket als die Anforderung selbst — PF-NFR-02.2 in AP-12 statt AP-08, PF-NFR-08.2 in AP-12 statt
AP-11, PF-NFR-05.4 in AP-14 und PF-NFR-05.5 in AP-04 statt AP-08, PF-SEC-04.2 in AP-12 statt
AP-05, PF-SEC-11.2 in AP-14 statt AP-06, PF-SEC-07.2 in AP-14 statt AP-18, PF-SET-01.2,
PF-IMP-05.2 und PF-DAT-01.2 in AP-12 statt AP-11, AP-08 bzw. AP-04, PF-SRC-05.2 in AP-12 statt
AP-10, PF-SEC-05.2 in AP-10 statt AP-04, PF-DES-05.2 und PF-DES-06.2 in AP-13 statt AP-12,
PF-PRV-07.2 in AP-13 statt AP-12 —, bindet er den Abschluss des zuordnenden Pakets **nicht**. Ohne
diese Regel schlösse AP-08 erst nach AP-12 und M2 erst nach M3.

### 4.2 Querschnittliche Zusagen

Die folgenden Zusagen gelten für **jede** Oberfläche, auch für die, die erst nach ihrem
zuordnenden Paket entsteht:

| Zusage | Kennung | Erstnachweis | Regressionslauf in |
|---|---|---|---|
| Tastaturbedienung, Fokus | SM-NFR-008 | AP-11 (Erreichbarkeit), AP-12 (Fokusfang am Dialog) | jedem Oberflächenpaket |
| Deutschsprachige Oberfläche | SM-SET-006 | **AP-12** (erste Oberflächentexte) | AP-13 bis AP-18; Abschlussnachweis AP-20 |
| Verständliche Fehlertexte | SM-NFR-006 | **AP-12** (erster Fehlertext) | AP-13 bis AP-18; Abschlussnachweis AP-20 |
| **Fremdtext nur als Nur-Text** | SM-SEC-008 | **AP-12** (Kachelname, Filter-Chips, Ordnerbaum) | AP-13, AP-14, **AP-15**, AP-16, AP-17, AP-18 |
| **Leerer, ladender und fehlerhafter Zustand je Bildschirm; konsistenter Abbruch** | SM-NFR-015, PF-NFR-15.1 bis .10 | AP-12 | AP-13 bis AP-18 |
| **Schichttrennung: keine Oberfläche greift unmittelbar auf `kern/db` zu** | SM-SEC-004 | AP-05 (Inspektion der Fassade, PF-SEC-04.1) | AP-12 (PF-SEC-04.2) und jedes folgende Oberflächenpaket |
| **Tragfähigkeit bei 100.000 Einträgen** | SM-LIB-009, SM-NFR-001 | AP-07 bzw. AP-10 (kernseitig, gegen den Prüfbestand) | AP-12 und jedes folgende Oberflächenpaket — erst dort zeigt sich, ob der Bestand auch **bedient** tragfähig bleibt |
| **Zweites Zustandsmerkmal neben der Farbe** | SM-NFR-009 | AP-11 (Bezeichner und Muster) | AP-12 bis AP-18, je neuem Zustand |
| **Reduzierte Bewegung wirkt** | SM-NFR-013 | AP-11 (Abfrage der Systemeinstellung) | AP-12 bis AP-18, je neuem Übergang **oder neuer Bewegung, einschließlich Ladeanzeigen** |
| **Kontrast über alle Textpaare** | SM-NFR-007 | AP-11 (rechnerisch gegen die Variablendatei) | AP-12 bis AP-18, je neuem Textpaar |
| **Themenwechsel, Fokusring, Nahtelemente** | Prüfpunkte D-03, D-04, D-06, D-11; AK-07 | AP-12 (erstes Paket mit Komponenten) | jedem Oberflächenpaket |
| **Zustandsbelegung und Mindestgrößen der Bedienelemente** | SM-NFR-016, PF-NFR-16.1 bis .3 | AP-12 (erstes Paket mit Bedienelementen; AP-11 liefert Bezeichner und Werte) | jedem Oberflächenpaket |

**Zu SM-SEC-004:** In AP-05 existiert noch kein Oberflächenmodul; die Regel „die Oberfläche greift
nie unmittelbar auf `kern/db` zu" kann dort nur *leer* bestehen. Verletzbar wird sie erst ab
AP-12. Der Erstnachweis in AP-05 prüft deshalb die Fassade als Bauteil, der Nachweis in AP-12 die
Einhaltung — beides gestützt auf die **automatisierte Schichtprüfung aus AP-02**, nicht auf
Sichtprüfung. Ohne diese Trennung schlösse AP-05 mit einem leer bestandenen Prüffall, und der
erste unmittelbare Datenbankzugriff aus `ui/navigation` fiele erst in AP-22 auf.

**Zu SM-SEC-008:** Der Anforderungstext nennt „Dateinamen, Metadaten, **maschinell erzeugte
Antworten**". Fremdtext erscheint damit nicht nur im Detailbereich, sondern auch auf der Kachel,
in den Filter-Chips, in der Stapelvorschau und bei den Analysewerten. Eine Zuordnung allein zu
AP-13 ließe genau die Stellen ungebunden, die der Anforderungstext ausdrücklich nennt.

**Zum Zustandstrio:** `CLAUDE.md` Abschnitt 14 hält fest: „ein Bildschirm ohne diese drei Zustände
ist unfertig". Das gilt für den Detailbereich, die Dokumentanzeige, die Druckvorschau, den
Exportdialog, die Stapelvorschau und die Analyse ebenso wie für die Musterauswahl.

**Nachgezogene Prüffallkennungen.** Die Entscheidungen zu OP-15 bis OP-17 und OP-19 haben die
vorläufigen Kennungen abgelöst: `PF-OP15-*` wird zu `PF-NFR-15.*`, `PF-OP16-*` zu
`PF-NFR-16.*`, `PF-OP17-01` zu `PF-LIB-11` und `PF-OP19-01` zu `PF-SET-02.3`. Nur
`PF-OP20-01` bleibt vorläufig, bis OP-20 entschieden ist. Die alten Kennungen werden nicht für
andere Gegenstände wiederverwendet.

**Zu den Prüfpunkten D-03, D-04, D-06 und D-11:** Sie prüfen den Themenwechsel „auf allen
Bildschirmen und Dialogen", den Fokusring an „jedem bedienbaren Element" und die Nahtelemente „an
allen vorgesehenen Stellen". AP-11 liefert die Bezeichner und die Rechnung, aber die Nahtlinien,
die Kreuzstichmarke am Navigationseintrag und der Auswahlring der Kachel entstehen erst in
AP-12, der Datenträgerstatus der rechten Statusleistengruppe erst in AP-16, Dialoge in AP-12,
AP-15, AP-17 und AP-18. Ein Erstnachweis in AP-11
könnte deshalb nur leer bestehen — und ein Bedienelement ohne Fokusring oder ohne Kreuzstichmarke
fiele erst in AP-22 auf, also nach der Paketierung.

**Warum das auch für Tastatur, Sprache und Fehlertexte gilt:** Druckdialog, Exportkonflikt,
Stapelvorschau und die Analyse-Bestätigungen kommen nach AP-11 und AP-20 — genau dort bleiben
Tastaturnutzer typischerweise in einem Dialog hängen, und dort entstehen englische
Restzeichenketten.

Regel: Erstnachweis im zuordnenden Paket, **Regressionslauf in jedem folgenden Oberflächenpaket**,
Abschlussnachweis in AP-22. Die Meilensteinbedingung von M3 gilt deshalb für den zu diesem
Zeitpunkt gebauten Umfang, nicht für die spätere Oberfläche.

### AP-00 · Entscheidungsgatter und Prototypvergleich

- **Ziel:** Die Entscheidung zwischen Weg A und Weg B belegen statt annehmen, und die Antworten
  auf die Punkte aus Abschnitt 2.1 herbeiführen.
- **Zugeordnet:** keine Anforderung — das Paket schafft Entscheidungsgrundlagen.
- **Vorbedingungen:** keine. Dies ist das einzige Paket ohne Vorbedingung.
- **Ergebnis:** Zwei lauffähige Prototypen mit je einer virtualisierten Liste über 100.000
  Einträge und einem Testdruck mit Kalibrierquadrat, gemessen am körperlichen Ausdruck auf A4 und
  US Letter. Messprotokoll mit Gerät, Datenbestand und Bedingungen. Entscheidungsvorlage.
- **Nachweis:** Vorlauf zu AK-01 und AK-06; Vorabnachweis der Toleranz aus SM-PRN-006.
- **Risiko:** Verfehlt **kein** Weg die Toleranz, ist die Technologiewahl offen und TEC-STM-001
  Abschnitt 2 neu zu bewerten. Das ist der teuerste denkbare Ausgang — und genau deshalb steht das
  Paket vorn.

### AP-01 · Lizenz- und Herkunftsgrundlage

- **Ziel:** Die Vorgabe ausschließlich quelloffener Komponenten vom Satz im Dokument zur Bedingung
  machen, die der Bau erzwingt.
- **Zugeordnet:** SM-OSS-001, SM-OSS-002, SM-OSS-003, SM-OSS-004, SM-OSS-005, SM-OSS-006,
  SM-OSS-007, SM-OSS-009, SM-OSS-011, SM-OSS-013
- **Vorbedingungen:** keine. Läuft nebenläufig zu AP-00.
- **Ergebnis:** Positivliste erlaubter Lizenzen, die den Bau abbricht, sobald eine Abhängigkeit
  außerhalb der Liste liegt; je Abhängigkeit nachgewiesene Verträglichkeit mit der Projektlizenz;
  eindeutige Ausweisung der Projektlizenz in Lizenzdatei, Projektdatei und Paketmetadaten;
  dynamische Bindung der Bibliotheken unter schwachem Copyleft; Herkunftsnachweis für vorgebaute
  Fremdpakete; mitgelieferte Schriften unter freier Schriftlizenz; Abschaltbarkeit jeder Anbindung
  an proprietäre Dienste. **Laufende Beobachtung der Sicherheitsmeldungen** zu den mitgelieferten
  Fremdkomponenten, insbesondere der Anzeigekomponente für Fremddokumente. Sie ist an
  **SM-OSS-011** gebunden: Wer Quelle und Bauweg eines vorgebauten Pakets nachweisen muss, muss
  auch wissen, ob dieser Stand zurückgezogen wurde — die Lizenzprüfung allein deckt Schwachstellen
  nicht ab. Nachweis PF-OSS-11.
- **Nicht in diesem Umfang:** Die maschinenlesbare Stückliste (SM-OSS-008) und die Lizenzanzeige
  im Programm (SM-OSS-010) sind Soll und zurückgestellt. **Folge:** AK-11 ist nur teilweise
  abnehmbar — der Bauabbruch nach SM-OSS-009 ist nachweisbar, die Stücklistenhälfte nicht.
- **Nachweis:** PF-OSS-01 bis PF-OSS-13 (Lücken für zurückgestellte Kennungen); AK-11 teilweise.
- **Risiko:** Ein dauerhaft rotes Pflicht-Gate wird umgangen statt befolgt. Bekannte, nicht sofort
  behebbare Funde gehören namentlich, begründet und **befristet** in eine Baseline; blockiert wird
  alles Neue.

### AP-02 · Projektgerüst, Prüfkette und Prüfbestand

- **Ziel:** Die automatisierte Prüfung **und die Messgrundlage** stehen, bevor der erste Fachcode
  entsteht.
- **Zugeordnet:** SM-NFR-012
- **Vorbedingungen:** AP-00 (Wegentscheidung bestimmt die Oberflächenprüfwerkzeuge), AP-01; für
  Stufe (b) des Prüfbestands zusätzlich AP-04.
- **Ergebnis:** Projektgerüst; Format-, Lint-, Test- und Lizenzprüfung in Stufe 0c des
  Commit-Freigabe-Gates; Projektregelprüfungen zu SM-DES-003 (Prüfpunkt D-05), SM-PLT-007 und
  SM-SEC-005 — letztere ausdrücklich **auch über den Suchpfad** in `kern/db`; dazu die
  **Schichtprüfung zu SM-SEC-004** (kein Oberflächenmodul verweist unmittelbar auf `kern/db`) und
  die **Maskierungsprüfung zu SM-SEC-010** (kein unmaskierter Pfad im Protokollschreiber);
  Bauläufe für alle drei Plattformen. Zusätzlich der **Prüfbestand in zwei Stufen**: **(a)** ein
  erzeugter **Dateibestand** von 100.000 Stickdateien in `pruef`, reproduzierbar, ohne
  Datenbankschema — entsteht in M0; **(b)** ein **Einspielwerkzeug**, das daraus einen
  Datenbestand gegen das Schema aus AP-04 erzeugt, ohne den Importweg aus AP-08 zu benötigen —
  entsteht mit AP-04 in M1. Stufe (a) trägt die Messungen des Importwegs, Stufe (b) die
  Erstnachweise von AP-07, AP-10 und AP-12. **Die Vorbedingung AP-04 gilt nur für Stufe (b)** —
  andernfalls entstünde ein Ring AP-02 ↔ AP-04, und M0 wäre nicht schließbar.
- **Anmerkung zur Herkunft:** Die drei Projektregelprüfungen stammen aus `CLAUDE.md` Abschnitt 11,
  nicht aus dem Anforderungsumfang. SM-PLT-007 ist Soll; seine **Prüfung** entsteht hier mit,
  seine **Abnahme** ist in Version 1.0 nicht geschuldet. Eine mitgebaute Prüfung macht eine
  zurückgestellte Anforderung nicht zur verplanten.
- **Nachweis:** PF-NFR-12.
- **Risiko:** Der Prüfbestand ist die Voraussetzung, unter der AP-07, AP-10 und AP-12 ihre
  A-Anforderungen überhaupt messen können. Entstünde er erst mit dem Import (AP-08), wären die
  Erstnachweise dieser drei Pakete gegenstandslos — es gäbe nichts, woran gemessen würde. Deshalb
  die Zweiteilung: Der Dateibestand braucht kein Schema, das Einspielwerkzeug keinen Importweg.

### AP-03 · Pfadsicherheit

- **Ziel:** Jeder Pfad, der aus einer Datei oder von der Bedienung kommt, ist geprüft, bevor er
  ein Dateisystem berührt — **gleich, welches Wurzelverzeichnis gerade gilt**.
- **Zugeordnet:** SM-SEC-001, SM-SEC-002, SM-SEC-003, SM-NFR-010
- **Vorbedingungen:** AP-02.
- **Ergebnis:** Kanonisierung **beider** Seiten und Eingrenzung auf das jeweils gültige
  Wurzelverzeichnis — Bibliothek, **Exportziel und Sicherungsziel gleichermaßen**; Behandlung von
  Symlinks und Schreibungsfaltung; **Ablehnung von Systemwurzeln beim Setzen jedes
  Wurzelverzeichnisses** — Bibliothek, Exportziel und Sicherungsziel gleichermaßen, denn alle drei
  sind nach demselben Absatz gleichrangig; korrekte Behandlung von Unicode- und langen Pfaden auf
  allen drei Plattformen. **Bereinigung aller aus Fremddaten gebildeten Dateinamen:** Pfadtrenner,
  `..`, absoluter Pfad, Nullzeichen, reservierte Gerätenamen, Längengrenze.
- **Nachweis:** PF-SEC-01 bis PF-SEC-03, PF-NFR-10.1 und PF-NFR-10.2. Geprüft wird durch
  Inspektion **und** Angriffsversuch (URS-STM-001 Abschnitt 13.2), ausdrücklich auch gegen Export-
  und Umbenennungspfade.
- **Risiko:** Ohne die Erweiterung auf Schreibziele außerhalb der Bibliothek bleibt ein konkreter
  Angriffsweg offen: manipulierte Datei importieren, deren Name als Metadatum übernehmen, Stapel
  danach umbenennen oder exportieren — und der Schreibvorgang verlässt das gewählte Ziel. Derselbe
  Weg gilt für ein Sicherungsarchiv beim Wiederherstellen.

### AP-04 · Datenhaltung

- **Ziel:** Eine sicherbare, migrierbare, absturzfeste Ablage an genau einem Ort.
- **Zugeordnet:** SM-DAT-001, SM-DAT-006, SM-DAT-007, SM-DAT-008, SM-DTA-002, SM-DTA-003,
  SM-DTA-004, SM-SEC-005, SM-LIB-010
- **Mitwirkung:** **SM-SEC-001, SM-SEC-002** (Sicherungs- und Wiederherstellungsziele laufen
  durch `kern/security` — Schnittregel 5), SM-DAT-003 (Bestätigung vor dem Löschen; Nachweis in
  AP-12)
- **Vorbedingungen:** AP-02, **AP-03**; OP-01 beantwortet.
- **Ergebnis:** Datenbank im WAL-Modus; additive, versionierte Migrationen, bestehende Schritte
  unveränderlich; Sicherung vor jeder Schemaanpassung; Wiederanlauf ohne Datenverlust nach
  unerwartetem Programmende; Abmessungen in Millimetern mit einer Nachkommastelle;
  zeitzonenunabhängige Speicherung mit Anzeige in Ortszeit; dauerhafte interne Kennung je Eintrag,
  die Umbenennung und Verschiebung übersteht; ausschließlich parametrisierte Abfragen. **Das
  Sicherungsarchiv ist ein Fremddatenpfad wie der Import:** Ein aus fremder Quelle bezogenes
  Archiv wird beim Wiederherstellen geprüft, nicht vertraut: Jeder Eintrag schreibt ausschließlich
  innerhalb des Zielverzeichnisses — kein Verzeichniswechsel, kein absoluter Pfad, kein Symlink —,
  und jeder aus dem Archivkopf gelesene Längen- oder Anzahlwert wird geprüft, bevor er eine
  Allokation oder eine Schleifengrenze steuert. Ein fehlerhaftes oder manipuliertes Archiv führt
  zu Fehlerstatus statt Absturz und hinterlässt keinen halb wiederhergestellten Stand. Unterfall
  **PF-NFR-05.5**. Weil der Archivpfad damit der **dritte** Fremddatenparser des Programms ist,
  entsteht hier auch ein **dauerhaftes Fuzzing-Ziel** (`PF-SEC-11.3`) — ein statischer
  Prüfbestand deckt nur die Fälle ab, an die jemand gedacht hat.
- **Nachweis:** PF-DAT-01.1 (Kerndienst; der Bedienweg PF-DAT-01.2 fällt in AP-12), PF-DAT-06 bis
  PF-DAT-08, PF-DTA-02 bis PF-DTA-04, PF-SEC-05.1, PF-NFR-05.5, **PF-SEC-11.3**, PF-LIB-10.
- **Risiko:** SM-DAT-007 verbietet die nachträgliche Änderung bestehender Migrationsschritte. Ein
  Fehler im ersten Schema wird dauerhaft mitgeführt; das erste Schema verdient entsprechende
  Sorgfalt.

### AP-05 · Kernfassade

- **Ziel:** Eine eng geschnittene Schnittstelle, über die die Oberfläche den Kern erreicht — und
  die einzige.
- **Zugeordnet:** SM-SEC-004, SM-DTA-001
- **Vorbedingungen:** AP-04.
- **Ergebnis:** Fassade mit Validierung an der Schnittstelle; jedes maschinell erzeugte Metadatum
  führt seine Herkunft mit, sodass die Kennzeichnung in der Oberfläche nicht auf einer Vermutung
  beruht. **Treffermengen werden ausschnittweise geliefert** — Versatz und Anzahl oder Cursor,
  dazu die Gesamtzahl; ein vollständiger Ergebnistransfer ist unzulässig (Schnittregel 3 in
  Kapitel 3).
- **Nachweis:** PF-SEC-04.1 (Inspektion der Fassade; PF-SEC-04.2 fällt in AP-12 und bindet den
  Abschluss dieses Pakets nicht), PF-DTA-01. Die Wirksamkeit der ausschnittweisen Lieferung wird
  in PF-PRV-07.1 mitgemessen: übertragene Datensätze je Suchlauf, unabhängig von der Bestandsgröße.
- **Risiko:** Die Regel „keine Oberfläche greift unmittelbar auf die Datenhaltung zu" ist
  nachträglich nicht durchsetzbar. Sie ist ab dem ersten Oberflächenmodul zu prüfen.

### AP-06 · Formatparser, lesend und gehärtet

- **Ziel:** Die vier Pflichtformate lesen, Kennwerte gewinnen, gegen manipulierte Dateien
  bestehen.
- **Zugeordnet:** SM-FMT-001, SM-FMT-002, SM-FMT-003, SM-FMT-004, SM-FMT-008, SM-FMT-010,
  SM-FMT-012, SM-FMT-013, SM-SEC-011
- **Vorbedingungen:** AP-02, AP-03; OP-13 beantwortet.
- **Ergebnis:** Parser für PES, DST, JEF und VP3; Stichanzahl, Abmessungen, Farbanzahl und
  Farbwechsel je Datei; Zuordnung der herstellerspezifischen Garnfarbpaletten, mindestens Brother
  und Janome (SM-FMT-010, nach Abschnitt 1.2 nachgezogen); Formatbestimmung am Dateiinhalt, nicht
  an der Endung — auch für angehängte Dokumente und Bilder, deren Typ AP-14 klassifiziert; je
  Parser ein dauerhaftes Fuzzing-Ziel; kein Absturz, keine unbegrenzte Speicherbelegung, keine
  Endlosschleife bei fehlerhaften Dateien.
- **Nachweis:** PF-FMT-01 bis PF-FMT-13 (Lücken für zurückgestellte Kennungen), PF-SEC-11.1.
- **Risiko:** RB-06 — die Formate sind herstellereigen und nicht offen dokumentiert.
  Vollständigkeit je Formatvariante ist nicht zugesagt und darf nicht zugesagt werden.

### AP-07 · Bibliothek und Ordnerverwaltung

- **Ziel:** Bestehende Ordnerstrukturen verwalten, ohne Originaldateien anzutasten.
- **Zugeordnet:** SM-LIB-001, SM-LIB-003, SM-LIB-004, SM-LIB-009
- **Mitwirkung:** **SM-SEC-001, SM-SEC-002** (Anlegen, Umbenennen, Verschieben und Löschen von
  Ordnern sind Schreibvorgänge und laufen durch `kern/security` — Schnittregel 5), SM-LIB-002
  (Abbildung der Hierarchie in der Navigation; Nachweis in AP-12), SM-DAT-003 (auch das Löschen
  eines **Ordners** ist bestätigungspflichtig — Unterfall PF-DAT-03.3)
- **Vorbedingungen:** AP-03, AP-04, AP-05, **AP-02 (Prüfbestand)**.
- **Ergebnis:** Bibliothek aus einem oder mehreren Wurzelverzeichnissen; Anlegen, Umbenennen,
  Verschieben und Löschen von Ordnern; nachgewiesene Tragfähigkeit bei 100.000 Einträgen, gemessen
  gegen den Prüfbestand aus AP-02.
- **Nachweis:** PF-LIB-01.1, PF-LIB-03, PF-LIB-04, PF-LIB-09; Beitrag zu AK-01.
- **Risiko:** RB-04 gilt unbedingt — der Import verändert und verschiebt keine Quelldatei.

### AP-08 · Import

- **Ziel:** Massenimport im Hintergrund, ohne die Bedienung anzuhalten und ohne bei einer defekten
  Datei abzubrechen.
- **Zugeordnet:** SM-IMP-001, SM-IMP-003, SM-IMP-005, SM-IMP-009, SM-NFR-002, SM-NFR-005
- **Mitwirkung:** SM-IMP-002 (Fortschrittsanzeige in der Statusleiste; Nachweis in AP-12),
  SM-SEC-001, SM-SEC-002 (jede eingelesene Datei ist Fremddaten)
- **Vorbedingungen:** AP-03, AP-06, AP-07.
- **Ergebnis:** Rekursiver Import; inkrementelle Läufe über neue, geänderte und entfernte Dateien;
  Duplikaterkennung über einen Hash des Dateiinhalts mit Vorlage zur Entscheidung — **der Hash
  wird nur für Dateien gebildet, die die Änderungserkennung als neu oder geändert ausweist; der
  Größenvergleich wirkt als Vorfilter innerhalb dieser Menge**. Ein unveränderter Bestandseintrag
  wird nie erneut gehasht — andernfalls läse ein inkrementeller Lauf über 100.000 Einträge
  entgegen SM-IMP-003 den gesamten Dateiinhalt der Bibliothek; defekte Dateien mit Fehlerstatus
  statt Abbruch; kein Absturz bei fehlenden Berechtigungen oder entferntem Datenträger. Die
  Änderungserkennung aus SM-IMP-003 ist zugleich der **Auslöser der Vorschau-Verwerfung** in
  AP-09.
- **Nachweis:** PF-IMP-01, PF-IMP-03, PF-IMP-05.1 (die Entscheidungsvorlage PF-IMP-05.2 fällt in
  AP-12), PF-IMP-09, PF-NFR-05.1 bis PF-NFR-05.3; **PF-NFR-02.1** (Nichtblockierung der
  Kernaufrufe unter Importlast — ohne Oberfläche messbar). **PF-NFR-02.2** (Eingabelatenz der
  Oberfläche unter Importlast) und der Beitrag zu **AK-01** fallen erst in AP-12, weil beide eine
  Oberfläche voraussetzen.
- **Risiko:** Der Import ist der Fremddatenpfad dieses Programms. Jede eingelesene Datei ist
  unvertrauenswürdig, auch wenn sie aus dem Dateisystem des Nutzers stammt.

### AP-09 · Vorschauerzeugung

- **Ziel:** Aus Stichdaten eine farbige Vorschau erzeugen, vorhalten und rechtzeitig verwerfen.
- **Zugeordnet:** SM-PRV-001, SM-PRV-002, SM-PRV-003
- **Mitwirkung:** SM-IMP-003 (liefert die Änderungserkennung, die die Verwerfung auslöst)
- **Vorbedingungen:** AP-06, **AP-08**.
- **Ergebnis:** Vorschauerzeugung aus den Stichdaten; dauerhafter Zwischenspeicher; Verwerfung,
  sobald sich die Quelldatei ändert. **Die Stichvorschau wird bei geänderter Anzeigegröße neu
  gezeichnet, nicht hochskaliert** (DES-STM-001 Abschnitt 11, Zeile „Hohe Auflösung“); der
  Zwischenspeicher hält sie je **diskret gestufter** Auflösung — nicht je beliebiger Pixelgröße.
  Das Neuzeichnen läuft asynchron und erst nach Ende einer Größenänderung; bis dahin bedient der
  Zeichenpfad die vorhandene Stufe. **Zwei Auslöser, damit die Zusage ohne Ordnerüberwachung
  trägt:** die inkrementelle Änderungserkennung des Importlaufs **und** eine Prüfung beim Lesen
  aus dem Zwischenspeicher **ausschließlich gegen Größe und Änderungszeit** der Quelldatei. Der
  Inhaltshash aus SM-IMP-005 bleibt dem Importlauf vorbehalten: Im Lesepfad müsste er je
  angezeigter Kachel die vollständige Quelldatei lesen und bräche damit die Zusage aus AP-12, dass
  im Zeichenpfad keine Datei-E/A stattfindet. Ohne den zweiten Auslöser zeigte eine außerhalb der
  Anwendung geänderte Datei bis zum nächsten Importlauf eine falsche Vorschau — bei 100.000
  Einträgen ist ein Importlauf kein beiläufiger Vorgang.
- **Nachweis:** PF-PRV-01 bis PF-PRV-03; AK-03. PF-PRV-03 benennt den Auslöser ausdrücklich.
- **Risiko:** Ein Zwischenspeicher, der nicht verworfen wird, zeigt dauerhaft Falsches. Die
  Verwerfung ist der schwierigere Teil, nicht die Erzeugung. Die Ordnerüberwachung SM-IMP-004 ist
  zurückgestellt; deshalb der zweite Auslöser beim Lesen. Die Prüfung gegen Größe und
  Änderungszeit läuft **außerhalb des Zeichenpfads** — siehe AP-12.

### AP-10 · Suche und Index

- **Ziel:** Volltextsuche und Filter über die gesamte Bibliothek, unter einer Sekunde bei warmem
  Index.
- **Zugeordnet:** SM-SRC-001, SM-SRC-002, SM-SRC-003, SM-SRC-005, SM-SRC-007, SM-SRC-010,
  SM-NFR-001
- **Mitwirkung:** SM-SEC-005 (**PF-SEC-05.2** — die Suchanfrage trägt den höchsten
  Fremddatenanteil), SM-SRC-008 und SM-NFR-003 (Ereignis- und Zeichenpfad; Nachweis in AP-12)
- **Vorbedingungen:** AP-04, AP-07, **AP-02 (Prüfbestand)**.
- **Ergebnis:** Volltextindex über Name, Thema, Beschreibung, Schlagworte und weitere Textfelder;
  ordnerübergreifende Suche; Filter nach Schlagwort, Format, Größenbereich, Stichanzahlbereich,
  Farbanzahl und Importquelle, kombinierbar mit der Volltextsuche; Sortierung. **Suchbegriffe
  werden als Ausdruck der Volltextsuche gequotet, nie verkettet** — sonst erzeugt eine Eingabe mit
  Sonderzeichen der Abfragesprache einen Fehler oder eine ungewollt teure Abfrage über den
  Gesamtbestand. Der Nachweis dafür ist **PF-SEC-05.2** und fällt hier, nicht in AP-04: Ein
  fehlendes Quoting ist statisch nicht als Zeichenkettenverkettung sichtbar, weil auch ein
  parametrisiert übergebener Suchausdruck als Syntax der Abfragesprache gelesen wird. Die
  Projektregelprüfung aus AP-02 fängt das nicht ab.
- **Nachweis:** PF-SRC-01 bis PF-SRC-10 (Lücken für zurückgestellte Kennungen; PF-SRC-05.1 — die
  Sortiersteuerung PF-SRC-05.2 fällt in AP-12), PF-SEC-05.2 aus der Mitwirkung, PF-NFR-01; AK-02.
- **Risiko:** Eine Abfrage je Ansicht, nicht eine je Zeile. Filterung in der Anwendung statt im
  Index bricht SM-SRC-007 bei 100.000 Einträgen.

### AP-11 · Gestaltungsgrundlage

- **Ziel:** Eine Variablenquelle, zwei Themen, mitgelieferte Schriften, sichtbarer Fokus — und die
  beiden Zusagen, die Farbe und Bewegung betreffen.
- **Zugeordnet:** SM-DES-001, SM-DES-002, SM-DES-003, SM-DES-004, SM-SET-001, SM-SET-002,
  SM-SET-003, SM-SET-004, SM-NFR-007, SM-NFR-008, SM-NFR-009, SM-NFR-013
- **Mitwirkung:** SM-DES-009 (dieses Paket stellt die Marken und Flächen der Kennzeichnung bereit;
  nachgewiesen wird sie dort, wo sie erscheint — AP-12, AP-13 und AP-18)
- **Vorbedingungen:** AP-00; die früheren Gatter OP-07, OP-09, OP-15, OP-16 und OP-19 sind in
  URS-STM-001 v1.4 entschieden. Die Farbwerte bleiben nach OP-09 bis zum Markenabgleich
  vorläufig; entwickelt wird gegen Bezeichner.
- **Ergebnis:** Genau eine Datei mit allen Farb-, Schrift- und Abstandsbezeichnern; warme Palette
  mit Terracotta als Leitfarbe; Dunkelmodus in Espressotönen ohne neutrales Grau; Übernahme der
  Systemeinstellung mit manueller Übersteuerung (SM-SET-002) und Speicherung der Panelbreiten über
  Sitzungen hinweg (SM-SET-003) — beide nach Abschnitt 1.2 nachgezogen. Die manuelle
  Übersteuerung übersteht den Neustart (SM-SET-002, PF-SET-02.3); eingebettete, zur
  Laufzeit registrierte Schriften ohne Nachladen; **alle Symbole als Vektor** (DES-STM-001
  Abschnitt 11, Zeile „Hohe Auflösung“); Themenwechsel zur Laufzeit; durchgehend sichtbarer
  Fokusring,
  nie unterdrückt; Raster-, Radien- und Rahmenbezeichner nach DES-STM-001 Abschnitt 5, Dauer- und
  Verlaufsbezeichner nach Abschnitt 8 sowie die Layoutmaße aus Abschnitt 5 und 6.3 — Höhen von
  Werkzeug- und Statusleiste, Spaltenbreiten und
  ihre Wachstumsfaktoren, Spaltenbreite und Abstände des Kachelrasters, **Ziehfläche der
  Trenner** samt 32-px-Trefferzone; **die Werte der Zustandsbelegung je Komponente** nach
  DES-STM-001 Abschnitt 7 in **beiden** Modi und **die Mindestgrößen als Bezeichner**
  (SM-NFR-016). Dass beides an jedem gebauten Bedienelement auch **anliegt**, weist AP-12 nach
  (`PF-NFR-16.1`, `PF-NFR-16.2`); in AP-11
  existiert noch kein Bedienelement, der Nachweis könnte dort nur leer bestehen. Vollständige
  Tastaturerreichbarkeit jedes Bedienelements und **Tastaturkürzel für die häufigsten Aktionen**
  (SM-SET-004, nach Abschnitt 1.2 nachgezogen); **jeder Zustand trägt neben der Farbe ein zweites
  Merkmal** — Text, Symbol oder Form; **bei aktiver Systemeinstellung für reduzierte Bewegung
  entfallen alle Übergänge und Bewegungen ersatzlos** — einschließlich der Dauerbewegung des
  Ladezustands (DES-STM-001 Abschnitt 10, „ruhiger Puls“); der Plan liest die Vorgabe nicht
  enger, als sie geschrieben ist; Marken und Flächen für die doppelte Kennzeichnung
  maschinell erzeugter Werte — über Farbe **und** Text.
- **Querschnittlich:** SM-NFR-008 nach Abschnitt 4.2. **Der Erstnachweis ist geteilt:**
  **PF-NFR-08.1** (Fokusring- und Kürzelbezeichner liegen in `ui/gestaltung`, und die Regel
  „der Fokusring wird nie unterdrückt" ist dort verankert) fällt hier; **PF-NFR-08.3**
  (Erreichbarkeit und sichtbarer Fokusring an **jedem gebauten** Bedienelement) fällt in AP-12
  und läuft ab AP-13 als Regression mit; PF-NFR-08.2 (Fokusfang und Fokusrückgabe am
  Dialoggerüst) fällt in AP-12, weil das Dialoggerüst dort entsteht. **Warum die Teilung von
  .1 nötig ist:** In AP-11 existiert noch kein Bedienelement — derselbe Grund, aus dem die
  Zeilen darüber D-06 und `PF-NFR-16.1/.2` nach AP-12 ziehen. Ein dort geführter Nachweis der
  Erreichbarkeit bestünde zwangsläufig leer, und ein Bedienelement ohne Fokusring fiele erst
  im Regressionslauf auf, im ungünstigsten Fall erst in AP-22 nach der Paketierung. Getroffen
  wären genau die Nutzer, die ausschließlich mit der Tastatur arbeiten.
- **Nachweis:** PF-DES-01 bis PF-DES-04; PF-SET-01.1 (Themenwerte für beide Modi vorhanden),
  PF-SET-02.1 (Systemübernahme mit manueller Übersteuerung), PF-SET-03 (Panelbreiten überleben den
  Neustart); PF-NFR-07 (rechnerisch gegen die Variablendatei), PF-NFR-08.1, PF-NFR-09,
  PF-NFR-13, PF-SET-04.1 (Kürzel registriert, konfliktfrei und im Menü angezeigt);
  Prüfpunkt D-07. **Nicht hier:** PF-SET-01.2,
  PF-SET-02.3 und PF-IMP-05.2 fallen in AP-12 und binden diesen Abschluss nicht. **Nicht hier:**
  AK-07 sowie die Prüfpunkte D-03, D-06 und D-11 setzen Komponenten voraus, die erst in AP-12
  entstehen — Erstnachweis dort, Regressionslauf in jedem folgenden Oberflächenpaket (Abschnitt
  4.2).
- **Risiko:** Kontrastwerte werden **gerechnet**, nicht nach Augenmaß beurteilt. Die Rechnung
  gehört als Gate in die Prüfkette; ohne sie ist SM-NFR-007 nicht abnehmbar.

### AP-12 · Hauptfenster, Navigation, Musterauswahl und Dialoggerüst

- **Ziel:** Die dreispaltige Aufteilung mit ihren drei Spalten, den Zuständen aus DES-STM-001
  Abschnitt 10 und einem Dialoggerüst, das den Fokus hält.
- **Zugeordnet:** SM-DES-005, SM-DES-006, SM-DES-007, SM-PRV-007, SM-PRV-009, SM-LIB-002,
  SM-LIB-011, SM-IMP-002, SM-DAT-003, SM-SRC-008, SM-SRC-009, SM-NFR-003, SM-NFR-015,
  SM-NFR-016, SM-SEC-008, SM-SET-006, SM-NFR-006
- **Mitwirkung:** SM-NFR-008 (**Erstnachweis PF-NFR-08.2** — Fokusfang und Fokusrückgabe am
  Dialoggerüst), SM-NFR-002 (**PF-NFR-02.2** — Eingabelatenz unter Importlast; Beitrag zu AK-01),
  SM-SEC-004 (**PF-SEC-04.2** — hier wird die Schichttrennung erstmals verletzbar), SM-SRC-005
  (**PF-SRC-05.2** — die Sortiersteuerung steht in der Kopfzeile der Musterauswahl), SM-DES-009
  (die Marke maschinell erzeugter Werte sitzt auf der Kachel), SM-SET-001 (**PF-SET-01.2** — der
  Umschalter wirkt auf allen Bildschirmen), SM-LIB-001 (**PF-LIB-01.2** — die
  Bibliothekswurzel braucht einen Bedienweg, sonst bleibt sie kernseitig), SM-SET-002
  (**PF-SET-02.2/.3** — die manuelle Übersteuerung wird erreicht und bleibt nach Neustart),
  SM-IMP-005 (**PF-IMP-05.2** — die Entscheidungsvorlage
  bei Duplikaten steht auf dem Dialoggerüst), SM-DAT-001 (**PF-DAT-01.2** — Sichern und
  Wiederherstellen brauchen einen Bedienweg, sonst bleiben sie kernseitig),
  SM-NFR-007, SM-NFR-009, SM-NFR-013 (Regressionslauf nach Abschnitt 4.2)
- **Vorbedingungen:** AP-05, AP-09, AP-10, AP-11, **AP-02 (Prüfbestand Stufe b)**. Die früheren
  Oberflächengatter sind in URS-STM-001 v1.4 entschieden. **OP-18** bleibt für die Abnahme von
  PF-SRC-08 und PF-NFR-02.2 offen; beide werden bis dahin als Regressionsschwelle geführt.
- **Ergebnis:** Dreispaltiges Hauptfenster mit verschiebbaren Trennern; keine gestapelte
  Ersatzdarstellung, unterhalb der Mindestbreite waagerechtes Scrollen; **Navigationsspalte** mit
  Übersichtskarte und Ordnerbaum; virtualisiertes Kachelraster, das nur sichtbare Einträge
  zeichnet; feststehende Kachelhöhe vor dem Laden der Vorschau; auf der Kachel ausschließlich
  Bild, Format- und KI-Marke — DES-STM-001 Abschnitt 6.3 nennt sie „KI-Marke", SM-DES-007
  „Herkunftsmarke" —, Name und Größe; **Kopfzeile mit aktiven Filtern als
  Chips samt Entfernen-Kreuz
  und Chip „Zurücksetzen", rechts Trefferzahl und Sortiersteuerung in Festbreite**; Bestand,
  Treffer und Auswahl in der Statusleiste; entprellte Sucheingabe ohne Blockade der Bedienung;
  keine harte Obergrenze der darstellbaren Einträge; **Fortschritt langer Vorgänge in der
  Statusleiste mit Abbruch, nicht als modaler Dialog** (SM-NFR-015, PF-NFR-15.9);
  **Bedienweg zu den Einstellungen** aus der Werkzeugleiste — ohne ihn sind Darstellungsart und
  Bibliothekswurzel nicht erreichbar (SM-SET-001, SM-SET-002, SM-LIB-001; nachgewiesen über
  PF-SET-01.2, **PF-SET-02.2/.3** und **PF-LIB-01.2**); der Betriebsmodus bleibt mit SM-SET-008
  zurückgestellt;
  **Dialoggerüst mit Fokusfang und Fokusrückgabe**, darauf die Bestätigung vor dem Löschen — für
  Eintrag, Datei und Ordner. **Im Zeichenpfad findet keine Datei- oder Datenbank-E/A statt**:
  Vorschauen werden asynchron nachgeladen und bis dahin durch die Platzhalterfläche vertreten;
  auch die Gültigkeitsprüfung des Zwischenspeichers aus AP-09 läuft außerhalb des Zeichenpfads.
  Andernfalls kostete ein Zugriff je sichtbarer Kachel und Bild messbar Bildzeit, und die
  feststehende Kachelhöhe wäre der einzige Schutz vor Rucklern.
- **Zustände nach DES-STM-001 Abschnitt 10:** Bibliothek leer mit Einstieg „Ordner importieren";
  Filter ohne Treffer mit „Filter zurücksetzen"; Vorschau lädt ohne Layoutsprung; Datei nicht
  lesbar mit Marke „Fehler" und Klartextgrund; **Datei nicht auffindbar** mit einem Handlungsweg
  im Text statt einer Schaltfläche — die Schaltfläche „Neu verknüpfen" hängt an SM-EXP-010 und ist
  zurückgestellt, ein Fehlertext ohne Abhilfe erfüllt SM-NFR-006 aber nicht. Den **Wortlaut** legt
  das Pflichtenheft fest (PV-06); der Plan fordert nur, dass er eine Abhilfe nennt. Leerzustand
  und Trefferlosigkeit tragen SM-NFR-015 und werden in PF-NFR-15.1 geprüft.
  **Die Zustände gelten für beide gebauten Spalten:** In der Navigationsspalte betrifft das die
  noch nicht gesetzte Bibliothekswurzel (leer), den Aufbau von Ordnerbaum und Übersichtskarte
  (ladend) und ein nicht lesbares Wurzelverzeichnis (fehlerhaft).
- **Nachweis:** PF-DES-05.1, PF-DES-06.1, PF-DES-07, PF-PRV-07.1, PF-PRV-09, PF-LIB-02,
  PF-IMP-02, PF-DAT-03.1 bis PF-DAT-03.3 (Eintrag, Datei, Ordner), PF-SRC-08, PF-SRC-09,
  PF-NFR-03, PF-SEC-08, PF-SET-06,
  PF-NFR-06, PF-NFR-15.1, PF-NFR-15.8, PF-NFR-15.9, **PF-NFR-16.1 bis PF-NFR-16.3,
  PF-LIB-11 und PF-SET-02.3**;
  zusätzlich PF-NFR-08.3 (Erreichbarkeit und sichtbarer Fokusring an jedem gebauten
  Bedienelement), PF-SET-04.2 (das Kürzel löst die zugeordnete Aktion aus, nicht nur ihre
  Registrierung), PF-NFR-08.2, PF-NFR-02.2, PF-SEC-04.2, PF-LIB-01.2 und PF-SET-02.2/.3 aus der
  Mitwirkung sowie
  PF-NFR-07, PF-NFR-09 und PF-NFR-13 als Regressionslauf nach Abschnitt 4.2; AK-01, AK-07, AK-12;
  Prüfpunkte D-01, D-02, D-03, D-04 (Anwendung auf die gebauten Textpaare; die Rechnung fällt in
  AP-11), D-06, D-10, D-11.
- **Risiko:** Jede Schleife über den Gesamtbestand im Zeichenpfad bricht SM-PRV-007. Die
  Virtualisierung ist Bauart, nicht Optimierung.

### AP-13 · Detailbereich und Metadatenpflege

- **Ziel:** Alle Angaben eines Eintrags im rechten Bereich, bearbeitbar, ohne Verlust
  ungespeicherter Änderungen.
- **Zugeordnet:** SM-DES-008, SM-MET-001, SM-MET-002, SM-MET-005, SM-MET-007, SM-MET-009,
  SM-MET-010
- **Mitwirkung:** SM-NFR-007, SM-NFR-008, SM-SET-006, SM-NFR-006, SM-SEC-008, SM-NFR-009,
  SM-NFR-013,
  SM-DES-009 (Regressionslauf; die Kennzeichnung erscheint auch am Schlagwort), **SM-DES-005 und
  SM-DES-006** (PF-DES-05.2, PF-DES-06.2 — die Dreiteilung ist erst mit befülltem Detailbereich
  vollständig nachweisbar), **SM-PRV-007** (PF-PRV-07.2 — auch die Farbliste ist virtualisiert,
  DES-STM-001 Abschnitt 11), SM-FMT-008 und SM-FMT-010 (liefern den Inhalt des Abschnitts
  „Farben")
- **Vorbedingungen:** AP-12; SM-NFR-015 ist durch die Entscheidung zu OP-15 verbindlich.
- **Ergebnis:** Detailbereich gegliedert in Angaben, Größe, Farben und Optionen; Name, Thema,
  Beschreibung und Notizen bearbeitbar; freie Schlagworte, mehrere je Eintrag; Verschlagwortung
  über Mehrfachauswahl in einem Vorgang; die Schnittmusterfelder Designer, Kleidungstyp,
  Größenbereich, Schwierigkeitsgrad, Sprache und Bezugsquelle; Nachfrage beim Verlassen mit
  ungespeicherten Änderungen; Fremdtext ausschließlich als Nur-Text.
- **Abschnittsfolge des Detailbereichs** nach DES-STM-001 Abschnitt 6.4: Vorschau · KI-Hinweis ·
  Angaben · Größe und technische Werte · Farben · Optionen · Aktionen · Schlagworte · Projekte. In
  Version 1.0 entstehen alle bis auf **Projekte** (SM-PRJ-001, zurückgestellt — Abschnitt 1.3);
  der Zoom der Vorschau entfällt (SM-PRV-004), und die **Aktionen** verweisen auf AP-15 und AP-16,
  die sie liefern.
- **Zum Abschnitt „Farben":** Er entsteht vollständig nach DES-STM-001 Abschnitt 6.4 — Farbfeld
  mit diagonaler Stichschraffur, Garnname, Garnnummer in Festbreite, Stichanteil rechtsbündig.
  Möglich wird das durch SM-MET-010 und SM-FMT-010, die nach Abschnitt 1.2 nachgezogen sind; ohne
  sie bliebe SM-DES-008 als **Muss**-Anforderung dauerhaft teilweise unerfüllt.
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): kein Eintrag ausgewählt · Metadaten
  werden geladen · Feld nicht lesbar. Nachweis PF-NFR-15.2.
- **Nachweis:** PF-DES-08, PF-MET-01, PF-MET-02, PF-MET-05, PF-MET-07, PF-MET-09, PF-MET-10,
  PF-NFR-15.2, PF-PRV-07.2 (virtualisierte Farbliste); PF-DES-05.2 und PF-DES-06.2 aus der
  Mitwirkung (vollständige Dreiteilung mit
  befülltem Detailbereich).
  **Regressionslauf nach Abschnitt 4.2:** PF-NFR-07, PF-NFR-09, PF-NFR-13 (Kontrast, zweites
  Zustandsmerkmal, reduzierte Bewegung je neuem Element), PF-NFR-08.3 und PF-NFR-08.2
  (Erreichbarkeit und
  Fokusring), PF-NFR-16.1 und PF-NFR-16.2 (Mindestgrößen und Zustandsbelegung), PF-SEC-08 (Fremdtext
  als Nur-Text), PF-SET-06 und PF-NFR-06 (deutschsprachig, verständliche Fehlertexte); Prüfpunkte
  D-03, D-04, D-06, D-11.
- **Risiko:** Ein auszeichnungsfähiges Textelement an einer Stelle, an der Dateinamen oder
  Metadaten stehen, ist ein Sicherheitsbefund — auch ohne Webansicht.

### AP-14 · Schnittmuster und Anleitungen

- **Ziel:** Mehrere typisierte Dateien je Eintrag, im Programm anzeigbar — und gegen manipulierte
  Dokumente abgesichert.
- **Zugeordnet:** SM-DOC-001, SM-DOC-002, SM-DOC-003, SM-DOC-004
- **Mitwirkung:** **SM-SEC-007** (PF-SEC-07.2 — keine ausgehende Verbindung aus der
  Dokumentanzeige), SM-NFR-007, SM-NFR-009, SM-NFR-013 (Regressionslauf nach Abschnitt 4.2),
  SM-NFR-008, SM-SET-006, SM-NFR-006 (Regressionslauf — die Meldung „Dokument
  nicht lesbar" bei einem defekten oder manipulierten Schnittmuster ist der heikelste Fehlertext
  des Programms), SM-SEC-001, SM-SEC-008, **SM-FMT-012 und SM-SEC-011** (deren Wortlaut „alle
  Formatparser" lautet — ob er die Anzeigekomponente erfasst, ist offen und als OP-14 geführt),
  **SM-NFR-005** (eine defekte Datei führt nicht zum Absturz — Unterfall PF-NFR-05.4)
- **Vorbedingungen:** AP-12, AP-13; OP-14 für den Abnahmebezug der Härtung.
- **Ergebnis:** PDF-Schnittmuster als eigenständiger Eintragstyp; mehrere Dateien je Eintrag
  (Schnittmuster, Nähanleitung, Titelbild, Maßtabelle, Stoffbedarf), jede typisiert klassifiziert;
  Anzeige der Nähanleitung innerhalb der Anwendung ohne externes Programm. **Fehlerhafte oder
  manipulierte Dokumente führen zu Fehlerstatus statt Absturz**; Prüfbestand gezielt manipulierter
  PDF-Dateien in `pruef` **und ein dauerhaftes Fuzzing-Ziel für den Dokumentpfad** (Prüffall
  PF-SEC-11.2) — ein Prüfbestand deckt die bekannten Fälle einmalig ab, ein Fuzzing-Ziel die
  unbekannten dauerhaft; die Anzeigekomponente stammt ausschließlich aus der gepflegten
  Laufzeitumgebung. **Die Anzeigekomponente stellt keine ausgehende Verbindung her** — ein
  Dokument mit eingebetteter Ressourcenreferenz darf sie nicht auslösen (SM-SEC-007, Unterfall
  PF-SEC-07.2); andernfalls bräche die Anzeige eines fremden Schnittmusters die Offline-Zusage
  AK-08.
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): kein Dokument angehängt · Dokument
  wird aufgebaut, ohne Layoutsprung · Dokument nicht lesbar. Nachweis PF-NFR-15.3.
- **Nachweis:** PF-DOC-01 bis PF-DOC-04; PF-NFR-07, PF-NFR-09 und PF-NFR-13 als
  Regressionslauf nach Abschnitt 4.2; AK-05. Dazu **PF-NFR-08.3** und **PF-NFR-08.2**
  (Erreichbarkeit und Fokusring der neu
  entstandenen Elemente sowie Fokusfang und Fokusrückgabe der neu entstandenen Dialoge),
  **PF-NFR-16.1** und **PF-NFR-16.2** (Mindestgrößen und
  Zustandsbelegung), **PF-SEC-08** (Fremdtext als Nur-Text), **PF-SET-06** und **PF-NFR-06**
  (deutschsprachig, verständliche Fehlertexte) sowie die Prüfpunkte D-03, D-04, D-06 und D-11 — je
  Regressionslauf nach Abschnitt 4.2.
- **Risiko:** **Die Anzeige von Fremddokumenten ist der zweite Fremddatenparser dieses
  Programms.** Ein Schnittmuster aus dem Netz ist in dieser Domäne der Regelfall; trifft ein
  manipuliertes Dokument einen Speicherfehler der in einer speicherunsicheren Sprache
  geschriebenen Fremdkomponente, steht der Prozess offen, der Zugriff auf die gesamte Bibliothek
  und auf den Schlüsselspeicher hat. Ob SM-FMT-012 und SM-SEC-011 mit „alle Formatparser" diese
  Komponente erfassen, ist **offen und als OP-14 geführt** — der Plan legt den Wortlaut nicht aus
  (Abschnitt 0.1). Bis zur Antwort trägt der Absturzschutz über SM-NFR-005 einen Abnahmebezug;
  Härtung und Fuzzing warten nicht auf die Antwort.

### AP-15 · Druck

- **Ziel:** Die härteste Anforderung des Vorhabens: 100 mm im Quelldokument messen am Ausdruck 100
  mm mit der Toleranz aus SM-PRN-006.
- **Zugeordnet:** SM-PRN-001, SM-PRN-002, SM-PRN-003, SM-PRN-004, SM-PRN-005, SM-PRN-006,
  SM-PRN-007, SM-PRN-008, SM-PRN-009, SM-PRN-015
- **Mitwirkung:** SM-NFR-007, SM-NFR-009, SM-NFR-013 (Regressionslauf nach Abschnitt 4.2),
  SM-NFR-008, SM-SET-006, SM-NFR-006 (Regressionslauf im Druckdialog),
  **SM-SEC-008** — Dateiname und Metadaten erscheinen in Druckdialog, Vorschau und Seitenkopf; ein
  auszeichnungsfähiges Textelement dort manipulierte ausgerechnet den maßhaltigen Ausdruck und
  könnte über eine eingebettete Ressourcenreferenz eine ausgehende Verbindung auslösen
  (SM-SEC-007, AK-08); **SM-NFR-005** (PF-NFR-05.4 — ein manipuliertes Dokument darf auch im
  Druckpfad nicht zum Absturz führen)
- **Vorbedingungen:** AP-00 (Prototypnachweis), AP-14.
- **Ergebnis:** Druck aus der Anwendung ohne externes Programm; Druckvorschau, die dem Ausdruck
  entspricht; Wahl von Papierformat, Ausrichtung, Seitenbereich und Drucker; A4 und US Letter;
  maßhaltige Ausgabe; unbeabsichtigte Skalierung standardmäßig unterbunden; Warnung, sobald eine
  Einstellung den Maßstab verändern kann; keine Themenfarben im Ausdruck.
- **Nicht in diesem Umfang:** Die Kachelung großformatiger Schnittmuster (SM-PRN-010) ist Soll und
  zurückgestellt. **Folge:** AK-06 prüft ausdrücklich den gekachelten A4-Druck und ist damit nur
  teilweise abnehmbar.
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): kein Drucker verfügbar ·
  Druckvorschau wird aufgebaut · Vorschau nicht erzeugbar. Nachweis PF-NFR-15.4.
- **Nachweis:** PF-PRN-01 bis PF-PRN-15 (Lücken für zurückgestellte Kennungen); AK-06 teilweise;
  Prüfpunkt D-12. **Der Erstnachweis der Maßhaltigkeit fällt in diesem Paket**, gemessen am
  körperlichen Ausdruck; AP-22 wiederholt ihn je Veröffentlichung auf allen drei Plattformen.
  Dazu PF-NFR-07, PF-NFR-09 und PF-NFR-13 als Regressionslauf nach Abschnitt 4.2. Dazu
  **PF-NFR-08.3** und **PF-NFR-08.2** (Erreichbarkeit und Fokusring der neu
  entstandenen Elemente sowie Fokusfang und Fokusrückgabe der neu entstandenen Dialoge),
  **PF-NFR-16.1** und
  **PF-NFR-16.2** (Mindestgrößen und Zustandsbelegung), **PF-SEC-08** (Fremdtext als Nur-Text),
  **PF-SET-06** und **PF-NFR-06** (deutschsprachig, verständliche Fehlertexte) sowie die
  Prüfpunkte D-03, D-04, D-06 und D-11 — je Regressionslauf nach Abschnitt 4.2.
- **Risiko:** Das Ergebnis ist je Plattform und je Druckertreiber zu belegen. Ein Gate, das einen
  Drucker und ein Lineal braucht, ist kein Hook.

### AP-16 · Konvertierung, Export und Wechseldatenträger

- **Ziel:** Dateien in ein wählbares Zielformat bringen und sicher auf einen Datenträger
  schreiben.
- **Zugeordnet:** SM-EXP-001, SM-EXP-003, SM-EXP-004, SM-EXP-005, SM-EXP-006, SM-EXP-007
- **Mitwirkung:** **SM-SEC-001, SM-SEC-002** (das Exportziel ist ein Wurzelverzeichnis wie die
  Bibliothek), **SM-NFR-005** (PF-NFR-05.3 — entfernter Datenträger, volllaufendes Ziel),
  SM-NFR-007, SM-NFR-009, SM-NFR-013 (Regressionslauf nach Abschnitt 4.2),
  SM-NFR-008, SM-SET-006, SM-NFR-006, SM-SEC-008 (Regressionslauf im Exportdialog)
- **Vorbedingungen:** AP-06, AP-13, **AP-03**; OP-03 für den Abschluss.
- **Ergebnis:** Konvertierung in ein wählbares Zielformat; Ausweis der für die gewählte Datei
  verfügbaren Zielformate; Erkennung angeschlossener Wechseldatenträger als Exportziel; Prüfung
  von freiem Speicherplatz und Schreibrechten vor dem Kopieren; Überschreiben, Umbenennen oder
  Abbrechen bei Namenskonflikt; Export einzelner Dateien und ganzer Auswahlmengen. **Jeder
  Zielpfad wird gegen das Exportziel kanonisiert und eingegrenzt; aus Fremddaten gebildete
  Dateinamen sind bereinigt.** **Fortschritt des Kopiervorgangs in der Statusleiste mit Abbruch**,
  wie bei Import und Stapelvorgang (DES-STM-001 Abschnitt 10) — auch der Abbruch eines Exports
  trägt SM-NFR-015 und wird in `PF-NFR-15.10` geprüft. Ein während
  des Kopierens entferntes oder volllaufendes Ziel führt zu einer verständlichen Meldung, nicht
  zum Absturz. **Rechte Statusleistengruppe nach DES-STM-001 Abschnitt 6.5:** Datenträgerstatus
  mit Kreuzstichmarke in `--kn-ok`, sobald ein Datenträger verbunden ist, und Bibliotheksstatus.
  Ohne diesen Bauort hätte die Gruppe keinen — AP-12 baut die Statusleiste nur mit Bestand,
  Treffer und Auswahl, und Prüfpunkt D-11 könnte diesen Teil dort nur leer bestehen.
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): kein Datenträger angeschlossen ·
  Datenträger wird gelesen · Datenträger nicht beschreibbar. Nachweis PF-NFR-15.5.
- **Nachweis:** PF-EXP-01, PF-EXP-03 bis PF-EXP-05, PF-EXP-06.1 bis PF-EXP-06.3, PF-EXP-07;
  PF-NFR-15.5, PF-NFR-15.10; PF-NFR-07, PF-NFR-09 und PF-NFR-13 als Regressionslauf nach
  Abschnitt 4.2; Dazu **PF-NFR-08.3** und **PF-NFR-08.2** (Erreichbarkeit und Fokusring der neu
  entstandenen Elemente sowie Fokusfang und Fokusrückgabe der neu entstandenen Dialoge),
  **PF-NFR-16.1** und **PF-NFR-16.2** (Mindestgrößen und Zustandsbelegung), **PF-SEC-08**
  (Fremdtext als Nur-Text), **PF-SET-06** und **PF-NFR-06** (deutschsprachig, verständliche
  Fehlertexte) sowie die Prüfpunkte D-03, D-04, D-06 und D-11 — je Regressionslauf nach Abschnitt
  4.2. **Prüfpunkt D-11 als Regressionsnachweis** für die rechte
  Statusleistengruppe; **PF-NFR-05.3** (Datenträger während des Kopierens entfernt) aus der
  Mitwirkung — ohne diese Zeile hätte die Zusage im Ergebnis keinen Prüffall; **AK-04
  zweigeteilt:** Die Nicht-Sandbox-Hälfte auf allen drei Plattformen ist hier Erstnachweis; die
  Sandbox-Hälfte unter Linux setzt das Flatpak-Paket voraus und fällt in AP-21 — vorher existiert
  keine Sandbox, in der sie bestehen könnte.
- **Risiko:** OP-03 bestimmt den Umfang des Schreibpfads. SM-EXP-001 verlangt ein wählbares
  Zielformat, nicht welches — der Plan setzt die Formatliste nicht fest.

### AP-17 · Stapelverarbeitung

- **Ziel:** Massenänderungen mit Vorschau, sichtbarem Fortschritt, Abbruchmöglichkeit und ohne
  halb geänderten Zustand.
- **Zugeordnet:** SM-BAT-001, SM-BAT-002, SM-BAT-004, SM-BAT-005, SM-BAT-007
- **Mitwirkung:** **SM-SEC-001** (Zielnamen entstehen aus Metadaten, also aus Fremddaten),
  **SM-SEC-008** (Regressionslauf — die Zielnamen der Stapelvorschau sind aus Metadaten gebildet,
  also Fremdtext), SM-NFR-007, SM-NFR-009, SM-NFR-013 (Regressionslauf nach Abschnitt 4.2),
  SM-NFR-008, SM-SET-006, SM-NFR-006 (Regressionslauf in der Stapelvorschau)
- **Vorbedingungen:** AP-13, **AP-03**.
- **Ergebnis:** Mehrfachauswahl in der Musterauswahl; stapelweises Umbenennen nach
  konfigurierbaren Namensmustern mit mindestens den Variablen Name, Thema, Format und laufender
  Nummer; Vorschau der Änderungen vor der Ausführung; **Fortschritt in der Statusleiste mit
  Abbruchmöglichkeit**; kein halb geänderter Zustand nach Abbruch, bereits ausgeführte
  Einzelvorgänge im Protokoll ausgewiesen. **Aus Metadaten gebildete Zielnamen werden bereinigt,
  bevor sie ein Dateisystem berühren.**
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): keine Auswahl getroffen · Vorschau
  wird berechnet · Vorschau nicht erzeugbar. Nachweis PF-NFR-15.6.
- **Nachweis:** PF-BAT-01, PF-BAT-02, PF-BAT-04, PF-BAT-05, PF-BAT-07; PF-NFR-07, PF-NFR-09
  und PF-NFR-13 als Regressionslauf nach Abschnitt 4.2. Dazu **PF-NFR-08.1** (Erreichbarkeit und
  Fokusring der neu entstandenen Elemente), **PF-NFR-16.1** und **PF-NFR-16.2** (Mindestgrößen und
  Zustandsbelegung), **PF-SEC-08** (Fremdtext als Nur-Text), **PF-SET-06** und **PF-NFR-06**
  (deutschsprachig, verständliche Fehlertexte) sowie die Prüfpunkte D-03, D-04, D-06 und D-11 — je
  Regressionslauf nach Abschnitt 4.2.
- **Risiko:** SM-BAT-007 ist eine Zusicherung über Fehlerfälle. Ohne SM-BAT-005 wäre der Abbruch
  nur über einen Prozessabbruch erreichbar gewesen — genau deshalb ist SM-BAT-005 nachgezogen
  (Abschnitt 1.2). Geprüft wird der Abbruchpfad, nicht nur der Gutfall.

### AP-18 · Lokale Analyse und Schlüsselablage

- **Ziel:** Metadatenvorschläge aus lokaler Verarbeitung, standardmäßig aus, jederzeit erkennbar
  und einzeln übernehmbar.
- **Zugeordnet:** SM-KIA-001, SM-KIA-002, SM-KIA-004, SM-KIA-005, SM-KIA-007, SM-KIA-008,
  SM-KIA-010, SM-KIA-011, SM-SEC-006, SM-SEC-007, SM-DES-009
- **Mitwirkung:** **SM-SEC-008** (Regressionslauf — maschinell erzeugte Antworten sind Fremdtext
  und stehen im Anforderungstext namentlich), SM-NFR-007, SM-NFR-009, SM-NFR-013
  (Regressionslauf nach Abschnitt 4.2),
  SM-NFR-008, SM-SET-006, SM-NFR-006 (Regressionslauf
  in den Bestätigungsdialogen); AP-11 liefert die Marken der Kennzeichnung
- **Vorbedingungen:** AP-13; **OP-20 vor Beginn** (Abschnitt 2.1); OP-05 für den
  Abschluss.
- **Ergebnis:** Lokale Verarbeitung, die aus dem Vorschaubild Metadatenvorschläge erzeugt — Name,
  Thema, Beschreibung, Schlagworte, Farben (SM-KIA-001, nach Abschnitt 1.2 nachgezogen); Funktion
  standardmäßig deaktiviert und ausdrücklich zu aktivieren; unmissverständlicher Hinweis mit
  Bestätigung, bevor Bilddaten erstmals ein entferntes Ziel erreichen; jedes Ergebnisfeld einzeln
  übernehmbar oder verwerfbar; **Hinweisbox über den Angaben, solange unbestätigte Vorschläge
  vorliegen** (DES-STM-001 Abschnitt 9). Die Box selbst trägt über SM-DES-009 und SM-KIA-008; ihre
  **Sammelaktionen** — alle Vorschläge auf einmal übernehmen oder verwerfen — tragen keine
  Kennung, denn SM-KIA-007 fordert ausdrücklich die Übernahme jedes **einzelnen** Felds. Die Lücke
  ist als **OP-20** geführt, vorläufiger Prüffall `PF-OP20-01`; maschinell erzeugte Werte bis zur
  Bestätigung gekennzeichnet; Zugangsschlüssel ausschließlich im Schlüsselspeicher des
  Betriebssystems; ausgehende Verbindungen begrenzt und vollständig abschaltbar; verständliche
  Meldung bei Ausfall, alle übrigen Funktionen bleiben nutzbar.
- **Zustände nach DES-STM-001 Abschnitt 10** (SM-NFR-015): Analyse nicht aktiviert · Analyse
  läuft · Analyse ohne Ergebnis oder fehlgeschlagen. Nachweis PF-NFR-15.7.
- **Nachweis:** PF-KIA-01, PF-KIA-02, PF-KIA-04, PF-KIA-05, PF-KIA-07, PF-KIA-08, PF-KIA-10,
  PF-KIA-11, PF-SEC-06, PF-SEC-07.1, PF-DES-09, PF-NFR-15.7, PF-OP20-01;
  PF-NFR-07, PF-NFR-09 und PF-NFR-13 als Regressionslauf nach Abschnitt 4.2; AK-08 teilweise,
  AK-09; Dazu **PF-NFR-08.3** und **PF-NFR-08.2** (Erreichbarkeit und Fokusring der neu
  entstandenen Elemente sowie Fokusfang und Fokusrückgabe der neu entstandenen Dialoge),
  **PF-NFR-16.1** und **PF-NFR-16.2** (Mindestgrößen und Zustandsbelegung), **PF-SEC-08** (Fremdtext
  als Nur-Text), **PF-SET-06** und **PF-NFR-06** (deutschsprachig, verständliche Fehlertexte)
  sowie die Prüfpunkte D-03, D-04, D-06 und D-11 — je Regressionslauf nach Abschnitt 4.2.
  Prüfpunkt D-08.
- **Nachweise, deren Gegenstand an OP-05 hängt:** SM-KIA-005 (Hinweis vor dem Verlassen des
  Geräts) und der entfernte Zweig von SM-SEC-007 haben nur dann einen prüfbaren Gegenstand, wenn
  ein entfernter Pfad entsteht. **Bis zur Beantwortung von OP-05 werden sie als Feststellung
  geführt:** In Version 1.0 existiert kein entfernter Pfad; PF-KIA-05 weist die Abwesenheit nach,
  PF-SEC-07.1 die Abschaltbarkeit der lokalen Verarbeitung. **Dasselbe gilt für PF-SEC-06 und
  PF-KIA-10:** Solange kein entfernter Dienst entsteht, entsteht auch kein Zugangsschlüssel; beide
  weisen bis dahin nach, dass die Anwendung keinen Schlüssel anlegt und keinen in Datenbank oder
  Klartext ablegt. AK-09 ist damit in Version 1.0 nur zur Hälfte erbringbar und in Abschnitt 6.3
  entsprechend geführt. Entsteht der Pfad später, werden beide zu Prüfungen der Zusage. Das ist
  dieselbe Führung wie bei PF-SEC-13.
- **Risiko:** SM-KIA-005 gilt auch dann, wenn die entfernte Anbindung nicht gebaut wird — der
  Hinweis ist an den Pfad gebunden, nicht an den Zeitpunkt. SM-KIA-001 (Vorschlagserzeugung) ist
  nach Abschnitt 1.2 nachgezogen — ohne sie hinge die Kennzeichnungs- und Übernahmekette an einer
  weiten Auslegung von SM-KIA-002.

### AP-19 · Unversehrtheit der Quelldaten

- **Ziel:** Nachweisen, dass kein Weg, auf dem Fremdbestände in die Bibliothek gelangen, die
  Quelldaten verändert.
- **Zugeordnet:** SM-MIG-005
- **Mitwirkung:** SM-SEC-001, SM-SEC-002
- **Vorbedingungen:** AP-08, AP-03.
- **Wogegen der Nachweis geführt wird — und warum das benannt sein muss:** SM-MIG-001 bis
  SM-MIG-003 legen fest, **woraus** übernommen wird (Fremdsoftware, Sidecar-Dateien, CSV und
  JSON); alle drei sind zurückgestellt. Damit gäbe es entweder eine Bedienfunktion ohne Kennung —
  nach Abschnitt 0.1 ein Planungsfehler — oder einen Prüffall ohne Gegenstand. Der Plan wählt
  keines von beidem: **In Version 1.0 ist der Import der einzige Weg, auf dem Fremdbestände in die
  Bibliothek gelangen.** PF-MIG-05 wird deshalb am Importweg aus AP-08 geführt und weist RB-04
  nach: Die Quelldateien bleiben unverändert und werden nicht verschoben. Eine Übernahmefunktion
  im Sinne des Kapitels 6.18 entsteht nicht.
- **Nachweis:** PF-MIG-05, geführt am Importweg. **AK-10 ist gegenstandslos** — es prüft die
  Übernahme von Schlagworten und Metadaten aus einem Fremdbestand, den es in Version 1.0 nicht
  gibt (Abschnitt 6.3).
- **Risiko:** Ohne diese Festlegung wäre PF-MIG-05 ein nicht durchführbarer Prüffall — und ein
  nicht durchführbarer Prüffall ist kein bestandener.

### AP-20 · Protokoll, Sprache und Fehlertexte — Sammelnachweis

- **Ziel:** Das Protokoll maskiert Pfade; die deutschsprachige Bedienung und die Fehlertexte
  werden über den in M3 gebauten Umfang **zusammengeführt und abschließend nachgewiesen**. Die
  Erstnachweise zu SM-SET-006 und SM-NFR-006 fallen in AP-12, wo die ersten Oberflächentexte und
  der erste Fehlertext entstehen — ein Sammelnachweis vor dem Erstnachweis wäre keiner.
- **Zugeordnet:** SM-SEC-010
- **Vorbedingungen:** AP-13.
- **Ergebnis:** Vollständig deutschsprachige Oberfläche in der Terminologie der Begriffstabelle
  des Lastenhefts; Fehlertexte für Endnutzer formuliert, technische Angaben ausschließlich im
  Protokoll — **mit maskierten Pfaden und ohne unmaskierte personenbeziehbare Angaben**.
  Ursprünglich war SM-SEC-010 zurückgestellt mit dem Argument, das Protokoll sei „nicht
  exportierbar". Diese enge Lesart steht dem Plan nicht zu (Abschnitt 0.1): Eine Protokolldatei im
  Anwendungsverzeichnis ist faktisch exportierbar, sobald ein Nutzer sie einer Supportanfrage
  anhängt. Die Maskierung ist eine einmalige Funktion im Protokollschreiber und damit billiger als
  die Auslegungsfrage.
- **Querschnittlich:** SM-SET-006 und SM-NFR-006 nach Abschnitt 4.2. Der **Erstnachweis fällt in
  AP-12** — dort entstehen die ersten Oberflächentexte und der erste Fehlertext, ein
  Regressionslauf davor wäre kein Nachweis. Dieses Paket führt den **Sammel- und
  Abschlussnachweis** über den in M3 gebauten Umfang; AP-14 bis AP-18 laufen als Regression.
- **Nachweis:** PF-SEC-10 (Maskierung, zusätzlich als automatisierte Prüfung in AP-02);
  Sammelnachweis über PF-SET-06 und PF-NFR-06, deren Erstnachweise in AP-12 fallen. PF-NFR-06
  erfasst ausdrücklich auch den Zustand „Datei nicht auffindbar" aus AP-12: eine Meldung, die nur
  den Fehler nennt und nicht die Abhilfe, erfüllt SM-NFR-006 nicht.
- **Risiko:** Fehlertexte entstehen verteilt über alle Pakete. Dieses Paket sammelt und prüft sie;
  es erzeugt sie nicht allein.

### AP-21 · Auslieferung

- **Ziel:** Drei Plattformen aus einem Bau, ohne Administratorrechte, ohne Telemetrie.
- **Zugeordnet:** SM-PLT-001, SM-PLT-002, SM-PLT-003, SM-PLT-004, SM-PLT-005, SM-SEC-009,
  SM-SEC-012, SM-SEC-013, SM-SEC-014, SM-NFR-011, SM-NFR-014
- **Vorbedingungen:** die **Ergebnisse** der Pakete AP-01 bis AP-20 — nicht deren
  Abnahmeprotokolle (Abschnitt 4.1); OP-04, OP-06 und OP-11 für den Abschluss.
- **Ergebnis:** macOS als Universal-Paket; Windows-Installationspaket; Linux als Flatpak auf der
  KDE-Laufzeitumgebung; Linux-Paket ohne Netzwerkzugriff baubar; Signaturprüfung als Bedingung
  jedes Aktualisierungswegs; vollständige Nutzbarkeit ohne Internetverbindung; keine Übertragung
  von Telemetrie. **Zur Signaturprüfung (SM-SEC-013):** Version 1.0 enthält keinen
  Aktualisierungsmechanismus; PF-SEC-13 ist deshalb die dokumentierte **Feststellung** seiner
  Abwesenheit, nicht die Prüfung einer Signaturkette. Entsteht später ein solcher Weg, wird
  PF-SEC-13 zur Prüfung der Signaturkette. **Berechtigungen so eng wie möglich:** Schreibzugriff
  auf Wechseldatenträger und Zugriff auf den Geheimnisdienst wie gefordert, Dateizugriff aber nur
  auf Bibliothekswurzel und Exportziel statt pauschal auf das Benutzerverzeichnis.
- **Nachweis:** PF-PLT-01 bis PF-PLT-05, PF-SEC-09, PF-SEC-12 bis PF-SEC-14, PF-NFR-11, PF-NFR-14;
  **die Sandbox-Hälften von AK-04 und AK-09** (Wechseldatenträger schreibend, Geheimnisdienst
  erreichbar) — beide setzen das gebaute Flatpak-Paket voraus; AK-08 teilweise (Abschnitt 6.3).
- **Risiko:** SM-SEC-009 hat zwei Hälften — Abhängigkeiten und Berechtigungen. Die
  Abhängigkeitshälfte prüft AP-01 laufend; die Zuordnung liegt hier, weil die Berechtigungen erst
  mit der Paketierung entstehen. **Restrisiko:** SM-SEC-015 (Datei-Portale) ist zurückgestellt.
  Ein statisches Flatpak-Manifest kann ein erst zur Laufzeit gewähltes Wurzelverzeichnis nicht
  ausdrücken; der Dateizugriff wird deshalb enger gesetzt als auf das gesamte Benutzerverzeichnis,
  was die Fläche verkleinert, aber keine erzwungene Schranke ist. **Den konkreten
  Berechtigungssatz legt das Pflichtenheft fest (PV-08)** — ein Wert im Plan wäre eine Festlegung
  jenseits der Tiefengrenze aus Abschnitt 0.3. Mit Datum der erneuten Bewertung in Kapitel 9
  geführt.

### AP-22 · Abnahmenachweise

- **Ziel:** Die messenden Nachweise gegen den Auslieferungsstand wiederholen.
- **Zugeordnet:** keine eigene Anforderung — das Paket **wiederholt** die Erstnachweise der
  übrigen Pakete gegen den Auslieferungsstand und das Referenzgerät.
- **Vorbedingungen:** AP-21; OP-08 beantwortet.
- **Ergebnis:** Messprotokolle zu AK-01 (Import von 100.000 Dateien), AK-02 (Suchzeit bei warmem
  Index), AK-06 (Maßhaltigkeit am ungekachelten Ausdruck, A4 und US Letter, drei Plattformen),
  AK-07 (Kontrast über alle Bildschirme); Vorführungen zu AK-03, AK-04, AK-05, AK-08, AK-09,
  AK-12; Abschlussnachweis **aller** querschnittlichen Zusagen nach Abschnitt 4.2; Prüfpunkte
  D-01 bis D-08 und D-10 bis D-12. **Nicht erbracht** werden die Soll-Hälften von AK-10 und AK-11
  sowie D-09 und D-13 — Abschnitt 6.3.
- **Nachweis:** die Protokolle selbst, mit Messgerät, Datenbestand und Messbedingungen
  (URS-STM-001 Abschnitt 13.2).
- **Risiko:** Diese Nachweise sind **je Veröffentlichung** zu erbringen, nicht einmalig. Sie
  ersetzen keinen Erstnachweis und gehören nicht in das Commit-Gate.

---

## 5. Meilensteine

Ein Meilenstein ist erreicht, wenn **alle** seine Arbeitspakete abgeschlossen sind — also jede
zugeordnete Anforderung ihren Erstnachweis nach Abschnitt 4.1 bestanden hat. Ein Meilenstein wird
nicht teilweise erreicht.

| M | Bezeichnung | Arbeitspakete | Bedingung, die ihn schließt |
|---|---|---|---|
| **M0** | Entscheidung und Grundlage | AP-00, AP-01, AP-02 (Stufe a) | Wegentscheidung ist gemessen belegt; die Lizenzvorgabe wird vom Bau erzwungen statt behauptet; Prüfkette und **Dateibestand** von 100.000 Stickdateien stehen, bevor der erste Fachcode entsteht. **Stufe (b)** des Prüfbestands — das Einspielwerkzeug gegen das Schema — setzt AP-04 voraus und gehört zu M1; M0 fordert sie nicht |
| **M1** | Kern und Datenhaltung | AP-03, AP-04, AP-05, AP-06, AP-02 (Stufe b) | Die Pfadprüfung besteht den Angriffsversuch — gegen Bibliothek, Exportziel und Sicherungsziel; Migration und Wiederanlauf nach unerwartetem Programmende sind geprüft; die vier Pflichtparser lesen und werden dauerhaft gefuzzt; die Fassade ist die einzige Zugriffsschicht; **das Einspielwerkzeug erzeugt aus dem Dateibestand einen Datenbestand gegen das Schema** — ohne ihn hat kein Erstnachweis in M2 einen Gegenstand |
| **M2** | Bestand, Import, Vorschau, Suche | AP-07, AP-08, AP-09, AP-10 | Der Prüfbestand ist importierbar und durchsuchbar; die Suche liefert bei warmem Index unter einer Sekunde — Erstnachweis gegen den Bestand aus AP-02, **als Regressionsschwelle, solange OP-08 offen ist**; Vorschauen entstehen und werden bei erkannter Quelländerung verworfen. **Nicht Teil dieser Bedingung:** die Eingabelatenz der Oberfläche unter Importlast (PF-NFR-02.2) und AK-01 — beide setzen eine Oberfläche voraus und fallen in M3 |
| **M3** | Oberfläche | AP-11, AP-12, AP-13, AP-20 | **OP-07, OP-09, OP-15 bis OP-17 und OP-19 sind beantwortet** — das Lastenheft setzt sie vor den Umsetzungsbeginn der Oberfläche. Die drei Bereiche stehen bei 860 px, 1280 px und 2560 px nebeneinander; Kontraste sind gerechnet und erfüllt; kein Zustand hängt allein an der Farbe; bei reduzierter Bewegung findet keine Bewegung statt; die Auswahl ist virtualisiert und springt nicht; ein gesetzter Filter lässt sich wieder aufheben; jedes Bedienelement ist treffbar. **Die querschnittlichen Zusagen sind für den zu diesem Zeitpunkt gebauten Umfang nachgewiesen** — Druck, Export, Stapel und Analyse folgen und laufen als Regression erneut (Abschnitt 4.2) |
| **M4** | Dokumente und Druck | AP-14, AP-15 | Schnittmuster und Nähanleitung liegen in einem Eintrag und werden beide angezeigt; ein manipuliertes Dokument führt zu Fehlerstatus statt Absturz; ein Prüfmaß von 100 mm misst am körperlichen Ausdruck 100 mm innerhalb der Toleranz aus SM-PRN-006, auf A4 und US Letter |
| **M5** | Ausgabe und Übernahme | AP-16, AP-17, AP-19 | Der Export auf einen Wechseldatenträger erkennt Namenskonflikte und bietet alle drei Auswege; ein Stapelvorgang zeigt Fortschritt, lässt sich abbrechen und hinterlässt keinen halb geänderten Zustand; eine Übernahme verändert die Quelldaten nicht |
| **M6** | Lokale Analyse | AP-18 | **OP-20 ist beantwortet.** Die Funktion ist standardmäßig aus; maschinell erzeugte Werte sind ohne Farbwahrnehmung erkennbar; ein Zugangsschlüssel wird nach Neustart wiedergefunden und ist in der Datenhaltung nicht auffindbar |
| **M7** | Auslieferung und Abnahme | AP-21, AP-22 | Alle drei Plattformen gehen aus einem Bau hervor; die Messprotokolle nach Kapitel 6 liegen vor; die querschnittlichen Kennungen sind über den vollständigen Umfang nachgewiesen |

**Zur Reihenfolge.** M0 vor allem anderen ist die einzige zwingende Setzung von außen (TEC-STM-001
Abschnitt 8, Schritt 3). M1 vor M2 folgt aus der Sicherheitslage: Die Pfadprüfung muss vor dem
ersten Import stehen, nicht danach. M2 vor M3 folgt daraus, dass die Oberfläche gegen eine bereits
tragfähige Fassade gebaut werden soll. M4 und M5 sind untereinander tauschbar; M6 ist als einziger
Block ohne Nachfolger verschiebbar.

---

## 6. Prüf- und Nachweisplan

> **Vorläufigkeit.** Dieses Kapitel ist die Prüfplanung im Sinne von OP-08, dessen Frist damit
> erreicht ist. Es gilt bis zur Festlegung des Referenzgeräts als **vorläufig** und wird
> danach bestätigt. Betroffen sind ausschließlich die Messaufbauten in Abschnitt 6.4.

### 6.1 Prüffallschema

`PF-<Bereich>-<nn>` — `<Bereich>` und `<nn>` übernehmen Kürzel und Nummer der Anforderung.
SM-SEC-012 wird an PF-SEC-12 nachgewiesen, SM-LIB-009 an PF-LIB-09.

Die Ableitung aus der Anforderungsnummer ist Absicht: Eine fortlaufende Zählung über die
*verplanten* Anforderungen würde zurückgestellte überspringen und beim Nachrücken in einer
späteren Version eine Verschiebung erzeugen — gegen die eigene Regel, dass Prüffallkennungen nicht
wiederverwendet werden. **Lücken in der Nummernfolge sind gewollt** und weisen auf eine
zurückgestellte Anforderung hin.

**Eine Anforderung darf mehrere Prüffälle haben.** URS-STM-001 Abschnitt 13.2 verlangt
*mindestens* einen. Anforderungen mit mehreren Fehlerzweigen — oder mit Nachweisen, die in
verschiedenen Paketen fallen — erhalten Unterfälle `PF-<Bereich>-<nn>.<n>`, damit gerade die
Fehler- und Abbruchpfade nicht ungeprüft bleiben und kein Paket auf einen Nachweis wartet, den es
nicht führen kann:

| Anforderung | Unterfälle | Zweige |
|---|---|---|
| SM-NFR-005 | PF-NFR-05.1 bis .5 | defekte Datei · fehlende Berechtigung · entfernter Datenträger, auch während eines laufenden Exports · manipuliertes Fremddokument (AP-14) · manipuliertes Sicherungsarchiv (AP-04) |
| SM-NFR-010 | PF-NFR-10.1, .2 | Unicode-Pfade · lange Pfade, je auf drei Plattformen |
| SM-EXP-006 | PF-EXP-06.1 bis .3 | Überschreiben · Umbenennen · Abbrechen |
| SM-DAT-003 | PF-DAT-03.1 bis .3 | Eintrag · Datei · Ordner — jeder Löschweg ist datenverlustrelevant |
| SM-NFR-008 | PF-NFR-08.1, .2, .3 | Bezeichner und Regel „nie unterdrückt" in `ui/gestaltung` (AP-11) · Fokusfang und -rückgabe am Dialog (AP-12) · Erreichbarkeit und sichtbarer Fokusring an jedem gebauten Element (AP-12, Regression ab AP-13) |
| SM-SET-004 | PF-SET-04.1, .2 | Kürzel registriert, konfliktfrei, im Menü angezeigt (AP-11) · Kürzel löst die zugeordnete Aktion aus (AP-12) |
| SM-NFR-002 | PF-NFR-02.1, .2 | Nichtblockierung der Kernaufrufe (AP-08) · Eingabelatenz der Oberfläche (AP-12) |
| SM-SEC-011 | PF-SEC-11.1, .2, .3 | Fuzzing-Ziel je Stickformatparser (AP-06) · Fuzzing-Ziel für den Dokumentpfad (AP-14) · Fuzzing-Ziel für den Wiederherstellungspfad (AP-04) |
| SM-PRV-007 | PF-PRV-07.1, .2 | Kachelraster virtualisiert (AP-12) · Farbliste im Detailbereich virtualisiert (AP-13) |
| SM-LIB-001 | PF-LIB-01.1, .2 | Bibliothekswurzel kernseitig setzen und lesen (AP-07) · Bedienweg dorthin aus der Werkzeugleiste (AP-12) |
| SM-SET-002 | PF-SET-02.1, .2, .3 | Übernahme der Systemeinstellung (AP-11) · manuelle Übersteuerung über den Bedienweg (AP-12) · Fortbestehen über Neustart (AP-12) |
| SM-NFR-015 | PF-NFR-15.1 bis .10 | Zustandstrios der Oberflächen · konsistenter Abbruch von Import und Export |
| SM-NFR-016 | PF-NFR-16.1 bis .3 | Mindestgrößen · Zustandsbelegung · Trefferzone der Trenner |
| SM-SET-001 | PF-SET-01.1, .2 | Themenwerte für beide Modi vorhanden (AP-11) · Umschalter zur Laufzeit auf allen Bildschirmen (AP-12) |
| SM-DAT-001 | PF-DAT-01.1, .2 | Sichern und Wiederherstellen als Kerndienst (AP-04) · Bedienweg dorthin in der Oberfläche (AP-12) |
| SM-IMP-005 | PF-IMP-05.1, .2 | Erkennung über den Inhaltshash (AP-08) · Entscheidungsvorlage auf dem Dialoggerüst (AP-12) |
| SM-SEC-007 | PF-SEC-07.1, .2 | ausgehende Verbindungen begrenzt und abschaltbar (AP-18) · keine Verbindung aus der Dokumentanzeige (AP-14) |
| SM-SEC-004 | PF-SEC-04.1, .2 | Inspektion der Fassade als Bauteil (AP-05) · Einhaltung der Schichttrennung (AP-12) |
| SM-SRC-005 | PF-SRC-05.1, .2 | Sortierung im Index (AP-10) · Sortiersteuerung in der Kopfzeile (AP-12) |
| SM-SEC-005 | PF-SEC-05.1, .2 | parametrisierte Abfragen der Datenhaltung (AP-04) · Quotierung des Volltextausdrucks im Suchpfad (AP-10) |
| SM-DES-005, SM-DES-006 | PF-DES-05.1/.2, PF-DES-06.1/.2 | Rahmen, Trenner und Spaltenbreiten (AP-12) · vollständige Dreiteilung mit befülltem Detailbereich (AP-13) |

**Nach der Entscheidung nachgezogene Unterfälle.** URS-STM-001 v1.4 bindet die früher
vorläufigen Fälle aus OP-15 bis OP-17 und OP-19 an Anforderungen. Nur PF-OP20-01 bleibt bis zur
Entscheidung von OP-20 ohne Anforderungskennung:

| Prüffall | Gegenstand | Paket |
|---|---|---|
| `PF-NFR-15.1` | Leerzustand und Trefferlosigkeit der Musterauswahl | AP-12 |
| `PF-NFR-15.2` | Zustandstrio des Detailbereichs | AP-13 |
| `PF-NFR-15.3` | Zustandstrio der Dokumentanzeige | AP-14 |
| `PF-NFR-15.4` | Zustandstrio der Druckvorschau | AP-15 |
| `PF-NFR-15.5` | Zustandstrio des Exportdialogs | AP-16 |
| `PF-NFR-15.6` | Zustandstrio der Stapelvorschau | AP-17 |
| `PF-NFR-15.7` | Zustandstrio der Analyse | AP-18 |
| `PF-NFR-15.8` | Zustandstrio der Navigationsspalte: keine Bibliothekswurzel gesetzt · Ordnerbaum und Übersichtskarte im Aufbau · Wurzelverzeichnis nicht lesbar | AP-12 |
| `PF-NFR-15.9` | Abbruch eines laufenden Imports über die Statusleiste; danach ist der Bestand konsistent und der Lauf wiederholbar | AP-12 |
| `PF-NFR-15.10` | Abbruch eines laufenden Exports über die Statusleiste. **Bedingung binär:** Nach dem Abbruch enthält das Ziel keine unvollständige Datei — die angefangene Zieldatei ist entfernt, bereits vollständig kopierte bleiben erhalten, und das Protokoll weist beide Mengen aus | AP-16 |
| `PF-SET-02.3` | Die manuelle Übersteuerung der Darstellungsart übersteht den Neustart | AP-12 |
| `PF-OP20-01` | Sammelaktionen der Hinweisbox: alle Vorschläge übernehmen bzw. verwerfen | AP-18 |
| `PF-LIB-11` | Übersichtskarte zeigt Gesamtbestand als Festbreitenzahl und Formatverteilung als Balken mit Legende (DES-STM-001 Abschnitt 6.2) | AP-12 |
| `PF-NFR-16.1` | Mindestgrößen der Bedienelemente nach DES-STM-001 Abschnitt 7, gegen die Bezeichner der Variablenquelle geprüft | AP-12 |
| `PF-NFR-16.2` | Zustandsbelegung je Komponente nach DES-STM-001 Abschnitt 7, in beiden Modi | AP-12 |
| `PF-NFR-16.3` | Trenner: 1 px sichtbare Linie, mindestens 6 px Ziehfläche und 32 px breite unsichtbare Trefferzone | AP-12 |

Wie bei den Anforderungskennungen gilt: Prüffallkennungen werden **nicht wiederverwendet**.
Entfällt ein Prüffall, bleibt die Kennung mit Streichgrund stehen.

### 6.2 Methodenverteilung

Die Prüfmethode je Anforderung ist im Lastenheft festgelegt und wird hier nicht neu bestimmt. Über
die 140 verplanten Anforderungen:

| Methode | Anzahl | Wer erbringt sie |
|---|---|---|
| **T** — Test | 63 | automatisiert, Teil der Prüfkette, läuft je Commit |
| **I** — Inspektion | 37 | teils automatisierbar (Projektregeln, Literalprüfung, Abfrageform), teils Sichtprüfung im Review |
| **D** — Demonstration | 31 | Vorführung am laufenden System, je Meilenstein |
| **A** — Analyse | 9 | Messung mit protokolliertem Gerät, Datenbestand und Bedingungen |

**Die Reviewer messen nicht.** Kontrastwerte, Laufzeiten und Druckmaße werden von
deterministischen Gates oder von Hand ermittelt und im Änderungssatz belegt. Ein Reviewer verlangt
den Nachweis, wo einer fehlt; er behauptet keinen Messwert.

### 6.3 Anwendbarkeit der Abnahmelisten in Version 1.0

| Kriterium | Bezug | Stand | Begründung |
|---|---|---|---|
| AK-01 bis AK-05 | Muss | anwendbar | alle geprüften Anforderungen sind verplant |
| **AK-06** | SM-PRN-006 (M), SM-PRN-010 (S) | **teilweise** | Maßhaltigkeit am ungekachelten Ausdruck nachweisbar; der gekachelte Fall setzt SM-PRN-010 voraus |
| AK-07 | Muss | anwendbar | — |
| **AK-09** | SM-KIA-010 (M), SM-SEC-006 (M), SM-KIA-003 (K) | **teilweise** | Das Kriterium prüft, dass ein Zugangsschlüssel nach Neustart wiedergefunden und in der Datenhaltung nicht auffindbar ist. Solange OP-05 offen ist und kein entfernter Dienst entsteht, entsteht kein Schlüssel; nachweisbar ist bis dahin die zweite Hälfte — in der Datenhaltung liegt keiner |
| **AK-08** | SM-NFR-011 (M), SM-KIA-011 (M), SM-KIA-003 (K) | **teilweise** | Die Offline-Nutzbarkeit ist vollständig nachweisbar. Der zweite Halbsatz — die entfernte KI meldet einen verständlichen Fehler statt zu blockieren — wird am **lokalen** Dienst nachgewiesen; ob ein entfernter Zweig überhaupt entsteht, hängt an OP-05 |
| **AK-10** | SM-MIG-001 bis 004 (S), SM-MIG-005 (M) | **gegenstandslos** | Das Kriterium prüft die Übernahme von Schlagworten und Metadaten aus einem Fremdbestand. Da SM-MIG-001 bis SM-MIG-003 zurückgestellt sind, entsteht in Version 1.0 kein Übernahmepfad; SM-MIG-005 wird am Importweg nachgewiesen (AP-19) |
| **AK-11** | SM-OSS-008 (S), SM-OSS-009 (M) | **teilweise** | Bauabbruch bei Fremdpaket nachweisbar; die Stückliste nicht |
| AK-12 | SM-DES-006 (M) | anwendbar | — |
| D-01 bis D-06, D-08 | Muss | anwendbar | — |
| **D-07** | SM-NFR-013 | **anwendbar** | SM-NFR-013 ist nach Abschnitt 1.2 nachgezogen |
| **D-09** | SM-MAC-002 (S) | **nicht anwendbar** | die Rahmenprüfung ist zurückgestellt |
| D-10 bis D-12 | Muss | anwendbar | — |
| **D-13** | SM-SET-008 (S) | **gegenstandslos** | die geprüfte Abwesenheit besteht trivial, weil weder Betriebsmodus noch gewerblicher Bereich entstehen |

**Führungsregel:** Ein teilweise abnehmbares Kriterium wird als *teilweise* protokolliert, nie als
bestanden. Ein nicht anwendbares wird als *nicht anwendbar* geführt — der Satz „ein nicht
durchgeführter Test ist kein bestandener Test" gilt hier unverändert.

### 6.4 Messaufbauten

Ein Prüffall der Methode A ohne benannte Messgröße ist kein Nachweis. Deshalb trägt jede der neun
A-Anforderungen hier einen Aufbau.

| Nachweis | Messgröße und Aufbau | Vorbedingung |
|---|---|---|
| PF-LIB-09, PF-NFR-01 (SM-LIB-009, SM-NFR-001) | Bestand von 100.000 Einträgen aus AP-02; Stabilität über einen vollständigen Durchlauf, Speicherbedarf im Verlauf. **Bestehbedingung:** kein Abbruch, kein Funktionsverlust und keine unbeantwortete Eingabe über den gesamten Durchlauf; der Speicherbedarf erreicht nach dem Einschwingen ein Plateau und wächst nicht weiter mit der Zahl der durchlaufenen Einträge. Beides ist ohne Zahlenwert entscheidbar und hängt deshalb **nicht** an OP-08; das Referenzgerät bestimmt allein die Laufzeitschwellen | **OP-08** |
| PF-SRC-07 (SM-SRC-007) | Antwortzeit einer Schlagwortsuche mit Größenbereich **gegen den Prüfbestand aus AP-02 (100.000 Einträge)**, gemessen in **zwei Indexzuständen** — frisch eingespielt und nach mehreren inkrementellen Läufen; „warm" heißt: die erste Abfrage ist Aufwärmlauf und wird verworfen; protokollierte Wiederholungen, Median und Streuung | AP-02, **OP-08** |
| PF-NFR-02.1 (SM-NFR-002) | Antwortzeit der Kernaufrufe unter laufender Importlast — **ohne Oberfläche messbar**, deshalb schon in AP-08. **Bestehbedingung binär, ohne Schwellenwert:** kein Kernaufruf wird durch den laufenden Import blockiert, jeder kehrt zurück, während der Import läuft. Der Zahlenwert der Latenz hängt an OP-18 und wird nur protokolliert | AP-02 |
| PF-NFR-02.2 (SM-NFR-002) | Eingabelatenz der Oberfläche während Import, Indizierung und Vorschauerzeugung | AP-12, **OP-08**, **OP-18** |
| PF-PRV-02 (SM-PRV-002) | **Belegung des Zwischenspeichers je Eintrag und in Summe über den Prüfbestand** sowie **Zahl vorgehaltener Auflösungsstufen je Eintrag** — ein Zwischenspeicher ohne Obergrenze wächst mit Bestand und Fenstergrößen. **Bestehbedingung:** Die Zahl der je Eintrag vorgehaltenen Stufen ist beschränkt und die Schranke wird über den gesamten Prüfbestand eingehalten; die Summenbelegung überschreitet die in **PV-09** festgelegte Obergrenze nicht, und das Verdrängungsverfahren greift nachweislich, bevor sie erreicht wird | AP-02, AP-09 |
| PF-PRV-07.1, .2 (SM-PRV-007) | **Anzahl gezeichneter Elemente je Bild, Speicherbedarf je sichtbarem Ausschnitt, übertragene Datensätze je Suchlauf** beim Durchlauf über den Gesamtbestand — der Nachweis der Virtualisierung ist die Unabhängigkeit aller drei Größen von der Bestandsgröße. Zusätzlich **Dateisystem- und Datenbankzugriffe im Zeichenpfad je Bild (Sollwert 0)** und **höchstens eine Dateisystemabfrage je neu sichtbarem Eintrag, nie je Bild**, **Dateisystemabfragen je Bildlaufabschnitt** aus der Gültigkeitsprüfung des Vorschau-Zwischenspeichers, **Bildzeit beim Bildlauf über den Gesamtbestand ohne Hintergrundlast** und **Anzahl der Fassadenaufrufe je Bildlaufabschnitt** und **Antwortzeit eines Ausschnittabrufs am Anfang und am Ende der Treffermenge** — bleibt die zweite deutlich über der ersten, ist der Ausschnitt versatzbasiert und wächst mit der Bestandsgröße | AP-02, AP-05, AP-12 **Je Größe entscheidbar ohne Referenzgerät:** Dateisystem- und Datenbankzugriffe im Zeichenpfad je Bild = 0; Dateisystemabfragen je Bildlaufabschnitt höchstens eine je neu sichtbarem Eintrag; gezeichnete Elemente, Speicherbedarf je Ausschnitt, übertragene Datensätze und Fassadenaufrufe bleiben bei wachsender Bestandsgröße konstant. Nur die Bildzeit trägt keine Schwelle und wird protokolliert (OP-18) |
| PF-SRC-08 (SM-SRC-008) | **Entprellintervall in Millisekunden, Eingabelatenz und Reaktionsfähigkeit der Oberfläche während der laufenden Abfrage** | AP-02, AP-12, **OP-08**, **OP-18** |
| PF-PRN-06 (SM-PRN-006) | Kalibrierquadrat von 100 mm Kantenlänge, Ausdruck auf A4 und US Letter, Messung am **Papier** mit protokolliertem Messmittel, je Plattform und Druckertreiber | AP-00 liefert den Vorabnachweis |
| PF-NFR-07 (SM-NFR-007) | Rechnerische Kontrastprüfung über alle Textpaare in beiden Modi, als Gate in der Prüfkette | AP-11 |
| PF-OSS-04 (SM-OSS-004) | Verträglichkeitsnachweis je Abhängigkeit gegen die Projektlizenz, aus der Positivliste erzeugt | AP-01 |
| PF-SEC-01 bis PF-SEC-03 | Inspektion **und** Angriffsversuch mit Verzeichniswechsel, Symlink, abweichender Groß-/Kleinschreibung — je gegen Bibliothek, Exportziel und Sicherungsziel, zusätzlich mit aus Fremddaten gebildeten Dateinamen. PF-SEC-03 prüft die Ablehnung von Systemwurzeln bei **allen drei** Zielen | AP-03 |
| PF-FMT-12, PF-SEC-11.1 | Je Stickformatparser ein dauerhaftes Fuzzing-Ziel; Bestand gezielt manipulierter Dateien als Prüfmaterial | AP-06 |
| PF-SEC-11.2, PF-NFR-05.4 | Dauerhaftes Fuzzing-Ziel für den Dokumentpfad; Bestand gezielt manipulierter PDF-Dokumente; erwartet wird Fehlerstatus, nicht Absturz | AP-14 |
| PF-NFR-05.5 | Bestand gezielt manipulierter Sicherungsarchive: Verzeichniswechsel, absoluter Pfad, Symlink, überhöhte Längen- und Anzahlwerte im Archivkopf; erwartet wird Fehlerstatus ohne halb wiederhergestellten Stand | AP-04 |
| PF-DTA-03 (SM-DTA-003) | Lauf unter **mindestens zwei Zeitzonen** und über eine Sommerzeitumstellung hinweg, Anzeige in Ortszeit gegengeprüft — ein Test in der Zeitzone der Entwicklungsmaschine weist Zeitzonenunabhängigkeit nicht nach | AP-04 |

**Zwei Messgrößen ohne Schwelle.** PF-NFR-02.2 und PF-SRC-08 haben eine benannte Messgröße, aber
keinen Grenzwert — weder im Lastenheft noch hier. Nach dem eigenen Maßstab dieses Kapitels ist
eine Messung ohne Schwelle weder bestehbar noch durchfallbar; „bestanden" wäre eine Einschätzung.
Der Plan darf die Schwelle nicht selbst setzen (Abschnitt 0.1), deshalb ist die Lücke als
**OP-18** im Lastenheft geführt. Bis zur Antwort werden beide Größen gemessen und protokolliert,
aber nicht als bestanden gewertet.

**Begleitmessung ohne Abnahmezusage:** Die Zeit bis zur Bedienbereitschaft (SM-NFR-004,
zurückgestellt) wird **im Aufbau zu PF-NFR-02.2 in AP-12** mitprotokolliert, nicht bei PF-LIB-09:
„bedienbereit" setzt eine Oberfläche voraus, die es in AP-07 noch nicht gibt. Der Bestand steht
dort ohnehin, die Messung kostet nichts, und ein früher Wert ist wertvoller als gar keiner. Eine
Zusage entsteht daraus nicht.

**Prüfmaterial ist Fremddaten.** Stickdateien und PDF-Schnittmuster als Prüfbestand sind binär und
werden vom Gate namentlich als inhaltlich ungeprüft ausgewiesen. Sie gehören mit Herkunftsangabe
dokumentiert, nicht kommentarlos hinzugefügt.

**Diese Nachweise gehören nicht in das Commit-Gate.** Ein Gate, das einen Drucker, ein Lineal und
ein Referenzgerät braucht, ist kein Hook. Der Erstnachweis fällt im Fachpaket, die Wiederholung je
Veröffentlichung in AP-22 (Abschnitt 4.1).

---

## 7. Rückverfolgbarkeitsmatrix

Erfüllt URS-STM-001 Abschnitt 13.3. Eine Zeile je verplanter Anforderung, jede genau einem
Arbeitspaket zugeordnet; Mitwirkungen nach Abschnitt 1.4 erscheinen hier nicht. Wo eine
Anforderung mehrere Prüffälle trägt (Abschnitt 6.1), nennt die Spalte alle.

**Nicht in dieser Matrix:** allein der vorläufige Prüffall `PF-OP20-01`, weil OP-20 noch keine
Anforderungskennung besitzt. Die früheren OP-15-, OP-16-, OP-17- und OP-19-Fälle sind in
URS-STM-001 v1.4 Anforderungen zugeordnet und erscheinen deshalb regulär in der Matrix.

**Abweichung vom Schema des Lastenhefts, bewusst und nachzutragen:** Abschnitt 13.3 sieht die
Spalte *Spezifikation* (Abschnitt im Pflichtenheft) vor. Das Pflichtenheft steht aus (Abschnitt
0.3); die Spalte bleibt deshalb weg und wird nachgetragen, sobald es vorliegt. An ihre Stelle
treten **Prio** und **Prüf** — beide aus dem Lastenheft übernommen, nicht neu bestimmt. `M` ist
Muss, `S¹` eine nach Abschnitt 1.2 nachgezogene Soll-Anforderung.

**Zur Modulspalte:** Sie nennt die Module, in denen die Anforderung **erfüllt und nachgewiesen**
wird — die Endpunkte, nicht den vollständigen Aufrufweg. Dass jeder Weg von einem Oberflächenmodul
zu `kern/db` über `kern/fassade` führt, folgt aus Schnittregel 1 und wird nicht in jeder Zeile
wiederholt.

**Die Ergebnisspalte bleibt bis zum tatsächlichen Prüflauf auf `offen`** und wird erst mit einem
ausführbaren, bestandenen Nachweis fortgeschrieben (`CLAUDE.md` Abschnitt 10, Phase 4). Der
Eintrag nennt den ausgeführten Regellauf; ausgesetzte Fälle werden nicht als bestanden gewertet.

| Anforderung | Prio | Prüf | AP | Modul | Prüffall | Ergebnis |
|---|---|---|---|---|---|---|
| SM-LIB-001 | M | T | AP-07 | `kern/services · ui/fenster` | PF-LIB-01.1, .2 | offen |
| SM-LIB-002 | M | D | AP-12 | `ui/navigation` | PF-LIB-02 | offen |
| SM-LIB-003 | M | T | AP-07 | `kern/services` | PF-LIB-03 | bestanden — `cargo test -p kern-services pf_mig_05_import_veraendert_keine_quelldatei` (29.08.2026: 1 bestanden) |
| SM-LIB-004 | M | T | AP-07 | `kern/services` | PF-LIB-04 | offen |
| SM-LIB-009 | M | A | AP-07 | `kern/services` | PF-LIB-09 | offen |
| SM-LIB-010 | M | T | AP-04 | `kern/db` | PF-LIB-10 | bestanden — `cargo test -p kern-db` (29.08.2026: 25 bestanden; Kennungsfälle enthalten) |
| SM-LIB-011 | M | D | AP-12 | `ui/navigation` | PF-LIB-11 | offen |
| SM-IMP-001 | M | T | AP-08 | `kern/services` | PF-IMP-01 | bestanden — `cargo test -p kern-services pf_mig_05_import_veraendert_keine_quelldatei` (29.08.2026: 1 bestanden) |
| SM-IMP-002 | M | D | AP-12 | `ui/fenster · kern/services` | PF-IMP-02 | offen |
| SM-IMP-003 | M | T | AP-08 | `kern/services` | PF-IMP-03 | bestanden — `cargo test -p kern-fassade -p kern-services` (29.08.2026: 29 + 18 bestanden; Änderungsfälle enthalten) |
| SM-IMP-005 | M | T | AP-08 | `kern/services · ui/dialoge` | PF-IMP-05.1, .2 | offen |
| SM-IMP-009 | M | T | AP-08 | `kern/services` | PF-IMP-09 | bestanden — `cargo test -p kern-services defekte_dateien_halten_den_lauf_nicht_an` (29.08.2026: 1 bestanden) |
| SM-FMT-001 | M | T | AP-06 | `kern/parsers` | PF-FMT-01 | offen |
| SM-FMT-002 | M | T | AP-06 | `kern/parsers` | PF-FMT-02 | offen |
| SM-FMT-003 | M | T | AP-06 | `kern/parsers` | PF-FMT-03 | offen |
| SM-FMT-004 | M | T | AP-06 | `kern/parsers` | PF-FMT-04 | offen |
| SM-FMT-008 | M | T | AP-06 | `kern/parsers` | PF-FMT-08 | offen |
| SM-FMT-010 | S¹ | T | AP-06 | `kern/parsers` | PF-FMT-10 | offen |
| SM-FMT-012 | M | T | AP-06 | `kern/parsers` | PF-FMT-12 | offen |
| SM-FMT-013 | M | T | AP-06 | `kern/parsers` | PF-FMT-13 | offen |
| SM-DOC-001 | M | T | AP-14 | `kern/db` | PF-DOC-01 | offen |
| SM-DOC-002 | M | T | AP-14 | `kern/db` | PF-DOC-02 | offen |
| SM-DOC-003 | M | T | AP-14 | `kern/db` | PF-DOC-03 | offen |
| SM-DOC-004 | M | D | AP-14 | `ui/detail` | PF-DOC-04 | offen |
| SM-MET-001 | M | T | AP-13 | `ui/detail` | PF-MET-01 | offen |
| SM-MET-002 | M | T | AP-13 | `ui/detail` | PF-MET-02 | offen |
| SM-MET-005 | M | T | AP-13 | `ui/detail` | PF-MET-05 | offen |
| SM-MET-007 | M | T | AP-13 | `ui/detail` | PF-MET-07 | offen |
| SM-MET-009 | M | D | AP-13 | `ui/detail · ui/dialoge` | PF-MET-09 | offen |
| SM-MET-010 | S¹ | T | AP-13 | `ui/detail` | PF-MET-10 | offen |
| SM-PRV-001 | M | D | AP-09 | `kern/render` | PF-PRV-01 | offen |
| SM-PRV-002 | M | T | AP-09 | `kern/render` | PF-PRV-02 | offen |
| SM-PRV-003 | M | T | AP-09 | `kern/render` | PF-PRV-03 | bestanden — `cargo test -p kern-render -p kern-services` (29.08.2026: 21 + 18 bestanden; Verwerfungsfälle enthalten) |
| SM-PRV-007 | M | A | AP-12 | `ui/auswahl · ui/detail` | PF-PRV-07.1, .2 | offen |
| SM-PRV-009 | M | D | AP-12 | `ui/auswahl` | PF-PRV-09 | offen |
| SM-SRC-001 | M | T | AP-10 | `kern/db` | PF-SRC-01 | offen |
| SM-SRC-002 | M | T | AP-10 | `kern/db` | PF-SRC-02 | offen |
| SM-SRC-003 | M | T | AP-10 | `kern/db` | PF-SRC-03 | offen |
| SM-SRC-005 | M | T | AP-10 | `kern/db · ui/auswahl` | PF-SRC-05.1, .2 | offen |
| SM-SRC-007 | M | A | AP-10 | `kern/db` | PF-SRC-07 | offen |
| SM-SRC-008 | M | A | AP-12 | `ui/auswahl` | PF-SRC-08 | offen |
| SM-SRC-009 | S¹ | D | AP-12 | `ui/auswahl` | PF-SRC-09 | offen |
| SM-SRC-010 | M | T | AP-10 | `kern/db` | PF-SRC-10 | offen |
| SM-BAT-001 | M | D | AP-17 | `ui/auswahl` | PF-BAT-01 | offen |
| SM-BAT-002 | M | T | AP-17 | `kern/services` | PF-BAT-02 | offen |
| SM-BAT-004 | M | D | AP-17 | `ui/dialoge` | PF-BAT-04 | offen |
| SM-BAT-005 | S¹ | D | AP-17 | `ui/fenster · kern/services` | PF-BAT-05 | offen |
| SM-BAT-007 | M | T | AP-17 | `kern/services` | PF-BAT-07 | offen |
| SM-EXP-001 | M | T | AP-16 | `kern/writers` | PF-EXP-01 | offen |
| SM-EXP-003 | M | D | AP-16 | `ui/detail` | PF-EXP-03 | offen |
| SM-EXP-004 | M | D | AP-16 | `kern/services · ui/fenster` | PF-EXP-04 | offen |
| SM-EXP-005 | M | T | AP-16 | `kern/services` | PF-EXP-05 | offen |
| SM-EXP-006 | M | T | AP-16 | `ui/dialoge · kern/services` | PF-EXP-06.1 bis .3 | offen |
| SM-EXP-007 | M | T | AP-16 | `kern/services` | PF-EXP-07 | offen |
| SM-PRN-001 | M | D | AP-15 | `ui/druck` | PF-PRN-01 | offen |
| SM-PRN-002 | M | D | AP-15 | `ui/druck` | PF-PRN-02 | offen |
| SM-PRN-003 | M | D | AP-15 | `ui/druck` | PF-PRN-03 | offen |
| SM-PRN-004 | M | T | AP-15 | `ui/druck` | PF-PRN-04 | offen |
| SM-PRN-005 | M | D | AP-15 | `ui/druck` | PF-PRN-05 | offen |
| SM-PRN-006 | M | A | AP-15 | `ui/druck` | PF-PRN-06 | offen |
| SM-PRN-007 | M | T | AP-15 | `ui/druck` | PF-PRN-07 | offen |
| SM-PRN-008 | M | D | AP-15 | `ui/druck` | PF-PRN-08 | offen |
| SM-PRN-009 | M | T | AP-15 | `ui/druck` | PF-PRN-09 | offen |
| SM-PRN-015 | M | I | AP-15 | `ui/druck` | PF-PRN-15 | offen |
| SM-KIA-001 | S¹ | D | AP-18 | `kern/services` | PF-KIA-01 | offen |
| SM-KIA-002 | M | T | AP-18 | `kern/services` | PF-KIA-02 | offen |
| SM-KIA-004 | M | I | AP-18 | `ui/dialoge` | PF-KIA-04 | offen |
| SM-KIA-005 | M | D | AP-18 | `ui/dialoge` | PF-KIA-05 | offen |
| SM-KIA-007 | M | D | AP-18 | `ui/detail` | PF-KIA-07 | offen |
| SM-KIA-008 | M | I | AP-18 | `ui/detail` | PF-KIA-08 | offen |
| SM-KIA-010 | M | I | AP-18 | `kern/services` | PF-KIA-10 | offen |
| SM-KIA-011 | M | T | AP-18 | `ui/detail` | PF-KIA-11 | offen |
| SM-DAT-001 | M | T | AP-04 | `kern/services · ui/dialoge` | PF-DAT-01.1, .2 | offen |
| SM-DAT-003 | M | D | AP-12 | `ui/dialoge` | PF-DAT-03.1 bis .3 | offen |
| SM-DAT-006 | M | T | AP-04 | `kern/db` | PF-DAT-06 | offen |
| SM-DAT-007 | M | I | AP-04 | `kern/db` | PF-DAT-07 | bestanden — `cargo test -p kern-db` (29.08.2026: 25 bestanden; Migrationsprüfsummen und Wiederholung enthalten) |
| SM-DAT-008 | M | T | AP-04 | `kern/db` | PF-DAT-08 | offen |
| SM-SET-001 | M | D | AP-11 | `ui/gestaltung · ui/auswahl` | PF-SET-01.1, .2 | offen |
| SM-SET-002 | S¹ | T | AP-11 | `ui/gestaltung · ui/fenster` | PF-SET-02.1, .2, .3 | offen |
| SM-SET-003 | S¹ | T | AP-11 | `ui/gestaltung` | PF-SET-03 | offen |
| SM-SET-004 | S¹ | D | AP-11 | `ui/gestaltung` | PF-SET-04.1, .2 | offen |
| SM-SET-006 | M | I | AP-12 | `ui (durchgehend)` | PF-SET-06 | offen |
| SM-MIG-005 | M | T | AP-19 | `kern/services` | PF-MIG-05 | bestanden — `cargo test -p kern-services pf_mig_05_import_veraendert_keine_quelldatei` (29.08.2026: 1 bestanden) |
| SM-DTA-001 | M | I | AP-05 | `kern/fassade` | PF-DTA-01 | offen |
| SM-DTA-002 | M | I | AP-04 | `kern/db` | PF-DTA-02 | offen |
| SM-DTA-003 | M | T | AP-04 | `kern/db` | PF-DTA-03 | offen |
| SM-DTA-004 | M | I | AP-04 | `kern/db` | PF-DTA-04 | offen |
| SM-NFR-001 | M | A | AP-10 | `kern/db` | PF-NFR-01 | offen |
| SM-NFR-002 | M | A | AP-08 | `kern/services · ui/auswahl` | PF-NFR-02.1, .2 | offen |
| SM-NFR-003 | M | T | AP-12 | `ui/auswahl · kern/db` | PF-NFR-03 | offen |
| SM-NFR-005 | M | T | AP-08 | `kern/services · kern/security · kern/db` | PF-NFR-05.1 bis .5 | offen |
| SM-NFR-006 | M | I | AP-12 | `ui (durchgehend)` | PF-NFR-06 | offen |
| SM-NFR-007 | M | A | AP-11 | `ui/gestaltung` | PF-NFR-07 | offen |
| SM-NFR-008 | M | D | AP-11 | `ui (durchgehend) · ui/dialoge` | PF-NFR-08.1, .2, .3 | offen |
| SM-NFR-009 | S¹ | I | AP-11 | `ui/gestaltung` | PF-NFR-09 | offen |
| SM-NFR-010 | M | T | AP-03 | `kern/security` | PF-NFR-10.1, .2 | offen |
| SM-NFR-011 | M | T | AP-21 | `kern/services` | PF-NFR-11 | offen |
| SM-NFR-012 | M | I | AP-02 | `bau · pruef` | PF-NFR-12 | offen |
| SM-NFR-013 | S¹ | D | AP-11 | `ui/gestaltung` | PF-NFR-13 | offen |
| SM-NFR-014 | M | I | AP-21 | `bau` | PF-NFR-14 | offen |
| SM-NFR-015 | M | D | AP-12 | `ui (durchgehend) · kern/services` | PF-NFR-15.1, .2, .3, .4, .5, .6, .7, .8, .9, .10 | offen |
| SM-NFR-016 | M | D | AP-12 | `ui (durchgehend)` | PF-NFR-16.1 bis .3 | offen |
| SM-SEC-001 | M | T | AP-03 | `kern/security` | PF-SEC-01 | offen |
| SM-SEC-002 | M | T | AP-03 | `kern/security` | PF-SEC-02 | offen |
| SM-SEC-003 | M | T | AP-03 | `kern/security` | PF-SEC-03 | offen |
| SM-SEC-004 | M | I | AP-05 | `kern/fassade` | PF-SEC-04.1, .2 | offen |
| SM-SEC-005 | M | I | AP-04 | `kern/db` | PF-SEC-05.1, .2 | offen |
| SM-SEC-006 | M | I | AP-18 | `kern/services` | PF-SEC-06 | offen |
| SM-SEC-007 | M | I | AP-18 | `kern/services · ui/detail` | PF-SEC-07.1, .2 | offen |
| SM-SEC-008 | M | I | AP-12 | `ui/navigation · ui/auswahl · ui/detail · ui/dialoge · ui/druck` | PF-SEC-08 | offen |
| SM-SEC-009 | M | I | AP-21 | `bau` | PF-SEC-09 | offen |
| SM-SEC-010 | S¹ | I | AP-20 | `kern/services` | PF-SEC-10 | offen |
| SM-SEC-011 | M | I | AP-06 | `pruef · kern/services` | PF-SEC-11.1, .2, .3 | offen |
| SM-SEC-012 | M | I | AP-21 | `bau` | PF-SEC-12 | offen |
| SM-SEC-013 | M | I | AP-21 | `bau` | PF-SEC-13 | offen |
| SM-SEC-014 | M | T | AP-21 | `bau` | PF-SEC-14 | offen |
| SM-OSS-001 | M | I | AP-01 | `bau` | PF-OSS-01 | offen |
| SM-OSS-002 | M | I | AP-01 | `bau` | PF-OSS-02 | offen |
| SM-OSS-003 | M | I | AP-01 | `bau` | PF-OSS-03 | offen |
| SM-OSS-004 | M | A | AP-01 | `bau` | PF-OSS-04 | offen |
| SM-OSS-005 | M | I | AP-01 | `bau` | PF-OSS-05 | offen |
| SM-OSS-006 | M | I | AP-01 | `bau` | PF-OSS-06 | offen |
| SM-OSS-007 | M | I | AP-01 | `bau · ui/gestaltung` | PF-OSS-07 | offen |
| SM-OSS-009 | M | T | AP-01 | `bau` | PF-OSS-09 | offen |
| SM-OSS-011 | M | I | AP-01 | `bau` | PF-OSS-11 | offen |
| SM-OSS-013 | M | T | AP-01 | `bau` | PF-OSS-13 | offen |
| SM-PLT-001 | M | T | AP-21 | `bau` | PF-PLT-01 | offen |
| SM-PLT-002 | M | I | AP-21 | `bau` | PF-PLT-02 | offen |
| SM-PLT-003 | M | T | AP-21 | `bau` | PF-PLT-03 | offen |
| SM-PLT-004 | M | T | AP-21 | `bau` | PF-PLT-04 | offen |
| SM-PLT-005 | M | T | AP-21 | `bau` | PF-PLT-05 | offen |
| SM-DES-001 | M | I | AP-11 | `ui/gestaltung` | PF-DES-01 | offen |
| SM-DES-002 | M | I | AP-11 | `ui/gestaltung` | PF-DES-02 | offen |
| SM-DES-003 | M | I | AP-11 | `ui/gestaltung` | PF-DES-03 | offen |
| SM-DES-004 | M | I | AP-11 | `ui/gestaltung` | PF-DES-04 | offen |
| SM-DES-005 | M | D | AP-12 | `ui/fenster` | PF-DES-05.1, .2 | offen |
| SM-DES-006 | M | D | AP-12 | `ui/fenster` | PF-DES-06.1, .2 | offen |
| SM-DES-007 | M | I | AP-12 | `ui/auswahl` | PF-DES-07 | offen |
| SM-DES-008 | M | D | AP-13 | `ui/detail` | PF-DES-08 | offen |
| SM-DES-009 | M | I | AP-18 | `ui/detail · ui/auswahl` | PF-DES-09 | offen |

**Abgleich.** 140 Zeilen: alle 129 Muss-Anforderungen des Lastenhefts und die elf nachgezogenen
Soll-Anforderungen. Keine Kennung doppelt, keine Muss-Anforderung ohne Arbeitspaket, keine
Prüffallkennung doppelt. Die Zuordnungsspalte ist aus den Zuordnungslisten in Kapitel 4 erzeugt,
nicht daneben gepflegt — beide können nicht auseinanderlaufen.

### 7.1 Ist-Abgleich vom 29.08.2026

Alle 140 Zeilen wurden gegen den Quellstand und den Regellauf
`cargo test -p kern-db -p kern-fassade -p kern-render -p kern-services --all-targets`
abgeglichen. Ergebnis: 93 bestanden, 0 fehlgeschlagen, 2 als Messfälle ausgesetzt. Die beiden
ausgesetzten 100.000-Einträge-Fälle bleiben Regressionsmessungen und begründen wegen OP-08
keinen Statuswechsel. Acht Anforderungen besitzen jetzt einen vollständigen, ausgeführten
Erstnachweis; 132 bleiben `offen`. Komponenten- und Teiltests werden dabei nicht als Abnahme
einer umfassenderen Anforderung gezählt.

| Arbeitspaket | Zeilen | bestanden | Grund für verbleibende offene Zeilen |
|---|---:|---:|---|
| AP-01 | 10 | 0 | Lizenz-, Stücklisten- und Auslieferungsnachweise fehlen |
| AP-02 | 1 | 0 | plattformübergreifende Prüfkette und formaler 100.000-Dateien-Prüfbestand fehlen |
| AP-03 | 4 | 0 | Pfadprüfungen decken Bibliothek, aber noch nicht Export- und Sicherungsziel auf allen Plattformen |
| AP-04 | 9 | 2 | PF-LIB-10 und PF-DAT-07 bestehen; Sicherung, Wiederanlauf, Zeitzonen- und vollständige Inspektionsnachweise fehlen |
| AP-05 | 2 | 0 | Komponenten- und Schichtinspektion ist noch nicht als PF-SEC-04.1/.2 und PF-DTA-01 abgeschlossen |
| AP-06 | 9 | 0 | Parser bestehen komponentenseitig; Prüfdateibestand, Inhaltserkennung und ausgeführte Fuzzing-Nachweise fehlen |
| AP-07 | 4 | 1 | PF-LIB-03 besteht; mehrere Wurzeln, Ordneroperationen und 100.000-Einträge-Nachweis fehlen |
| AP-08 | 6 | 3 | PF-IMP-01, PF-IMP-03 und PF-IMP-09 bestehen; Duplikatentscheidung sowie vollständige Hintergrund- und Fehlerzweige fehlen |
| AP-09 | 3 | 1 | PF-PRV-03 besteht; visuelle Vorschauabnahme und durch OP-21/PV-09 offene Speichergrenze fehlen |
| AP-10 | 7 | 0 | Filter- und Sortierumfang ist unvollständig; die beiden ausgesetzt laufenden Messfälle bleiben Regressionswerte |
| AP-11 | 12 | 0 | Gestaltungsnachweise fehlen; Farbwerte bleiben bis zum Markenabgleich vorläufig |
| AP-12 | 17 | 0 | Oberfläche ist nur teilweise umgesetzt; Demonstrationen und Messungen fehlen |
| AP-13 | 7 | 0 | Detailanzeige und Metadatenbearbeitung fehlen |
| AP-14 | 4 | 0 | Dokumentpfad fehlt |
| AP-15 | 10 | 0 | Druckpfad und körperliche Messung fehlen |
| AP-16 | 6 | 0 | Export- und Datenträgerpfad fehlen |
| AP-17 | 5 | 0 | Stapelverarbeitung fehlt |
| AP-18 | 11 | 0 | Analyse-, Zustimmungs- und Schlüsselspeicherpfad fehlen |
| AP-19 | 1 | 1 | PF-MIG-05 besteht; keine Restzeile |
| AP-20 | 1 | 0 | vollständiger Protokollmaskierungsnachweis fehlt |
| AP-21 | 11 | 0 | Paketierung und Plattformnachweise fehlen |

**Folgeaufgaben aus dem Abgleich:** PF-SEC-04.1/.2 und PF-DTA-01 als ausdrückliche
Architekturinspektion automatisieren; die fehlenden Filter- und Sortierzweige vor PF-SRC-01 bis
PF-SRC-10 ergänzen; die Parserfälle erst nach Bereitstellung des Prüfdateibestands und einem
ausgeführten Fuzzing-Lauf fortschreiben; PF-PRV-02 bis zur Entscheidung OP-21/PV-09 offen
halten. Diese Lücken werden nicht durch die grüne Paketsumme verdeckt.

---

## 8. Mitwachsende Prüfkette

`CLAUDE.md` Abschnitt 11 teilt die Prüfkette in „heute anwendbar" und „mit dem ersten Quellcode
verbindlich", legt aber nicht fest, **wann** der Wechsel eintritt. Dieses Kapitel bindet ihn an
Arbeitspakete.

Maßgeblich ist die Anwendbarkeitsregel S3: Ein Gate, dessen Gegenstand im Baum nicht existiert,
steht als **ENTFÄLLT** im Protokoll. Ein Gate, dessen Gegenstand existiert, das aber nicht laufen
kann, ist **FAIL**. Der Übergang ist maschinell bestimmt, nicht eingeschätzt.

| Gate | Heute | Wird scharf mit | Auslösendes Merkmal im Baum |
|---|---|---|---|
| Rust-Format, Lint, Test | ENTFÄLLT | **AP-02** | erste `Cargo.toml` |
| Lizenz- und Abhängigkeitsprüfung | ENTFÄLLT | **AP-01**, spätestens AP-02 | erste `Cargo.toml` |
| Fuzzing der Formatparser | ENTFÄLLT | **AP-06**, für den Dokumentpfad **AP-14**, für den Wiederherstellungspfad **AP-04** | erstes Fuzzing-Ziel |
| Oberflächenprüfwerkzeuge | ENTFÄLLT | **AP-11** | erste Oberflächenquelldatei; welche Werkzeuge, entscheidet AP-00 |
| Literalprüfung nach Prüfpunkt D-05 | läuft über die Dokumente; das Mockup ist **dateiweit ausgenommen**, weil Inhalts- und Themenfarben dort noch nicht getrennt sind (Restrisiko in `Analysis/20260823_01_gate-befunde-rueckstand.md`) | **AP-02** | erweitert sich auf den Quellbaum |
| Kontrastrechnung | — | **AP-11** | erste Fassung der Variablendatei |
| Schichtprüfung (SM-SEC-004) und Maskierungsprüfung (SM-SEC-010) | — | **AP-02** | erstes Oberflächenmodul bzw. erster Protokollschreiber |
| Messungen der Prüfmethode A | — | **AP-02** liefert den Bestand, die Fachpakete die Erstnachweise | erzeugter Prüfbestand in `pruef` |
| Dokumentprüfungen, Secret-Scan, Gate-Selbsttest | laufen | unverändert | — |

**Die Reihenfolge ist kein Zufall.** AP-02 steht vor jedem Fachcode, weil die Prüfung sonst
nachträglich über bereits geschriebenen Code gezogen wird und dann eine Nachbesserungswelle
auslöst statt Fehler zu verhindern. Umgekehrt gilt: Steht die Kette zum Zeitpunkt der ersten
`Cargo.toml` nicht, blockiert jeder folgende Commit fail-closed — das ist die vorgesehene Wirkung
und kein Störungsfall.

---

## 9. Risiken

| Risiko | Wirkung | Gegenmaßnahme | Paket |
|---|---|---|---|
| Kein Anbindungsweg hält die Drucktoleranz | Die Technologieentscheidung aus TEC-STM-001 Abschnitt 2 ist neu zu treffen; SM-PRN-006 ist die härteste Anforderung des Vorhabens | Prototypvergleich **vor** allem Bauen, gemessen am körperlichen Ausdruck | AP-00 |
| Formatvarianten sind nicht offen dokumentiert (RB-06) | Einzelne Dateien einer Variante bleiben unlesbar; Vollständigkeit ist nicht zusagbar | Fehlerstatus statt Abbruch; dauerhaftes Fuzzing; keine Vollständigkeitszusage im Abnahmetext | AP-06, AP-08 |
| **Die Anzeigekomponente für Fremddokumente ist ein zweiter Parser ohne eigene Kennung** | Ein manipuliertes Schnittmuster trifft einen Speicherfehler der Fremdkomponente; der Prozess hat Zugriff auf Bibliothek und Schlüsselspeicher | Härtung nach dem Muster von SM-FMT-012 unabhängig von der Kennungsfrage; Prüfbestand manipulierter Dokumente; Beobachtung der Sicherheitsmeldungen in AP-01; Kennungslücke als **OP-14** geführt | AP-14, AP-01 |
| **Zurückgestellte Datei-Portale unter Linux (SM-SEC-015)** | Ein statisches Flatpak-Manifest kann ein erst zur Laufzeit gewähltes Wurzelverzeichnis nicht ausdrücken. Die Eingrenzung bleibt eine Konfigurationszusage, keine erzwungene Schranke; in Verbindung mit dem vorstehenden Risiko ist genau die Eindämmung geschwächt, für die SM-SEC-014 die Sandbox vorschreibt | **Befristet akzeptiertes Restrisiko.** Der Dateizugriff wird enger gesetzt als auf das gesamte Benutzerverzeichnis; den konkreten Berechtigungssatz legt das Pflichtenheft fest (**PV-08**). **Erneute datierte Bewertung bis 24.02.2027** — ein Ablauf ohne neue Bewertung hebt die Annahme auf, ein stilles Hochsetzen des Datums ist unzulässig | AP-21 |
| **Veraltete Vorschau bis zum nächsten Importlauf** | Ohne Ordnerüberwachung (SM-IMP-004, zurückgestellt) erkennt die Anwendung eine außerhalb geänderte Datei erst beim Lesen aus dem Zwischenspeicher oder beim nächsten Importlauf | **Befristet akzeptiertes Restrisiko.** Die Prüfung gegen Größe und Änderungszeit beim Lesen begrenzt das Fenster; der Ausweg — Neuaufbau der Vorschau von Hand oder beim nächsten vollständigen Importlauf — ist in PF-PRV-03 benannt. **Erneute datierte Bewertung bis 24.02.2027** | AP-09 |
| **Keine Stückliste, gegen die Schwachstellenmeldungen abgeglichen werden** | AP-01 nimmt sich die laufende Beobachtung der Sicherheitsmeldungen vor, hat dafür aber weder Kennung noch Prüffall noch Datenbestand: SM-OSS-008 ist zurückgestellt. Die Beobachtung kann damit nie als versäumt festgestellt werden — und sie betrifft die einzige Komponente, für die der Plan selbst ein Übernahmerisiko benennt | **Befristet akzeptiertes Restrisiko** mit **erneuter datierter Bewertung bis 24.02.2027**. Zu prüfen ist dann, ob SM-OSS-008 nachgezogen wird — die Stückliste fällt bei der ohnehin verplanten Lizenzprüfung als Nebenprodukt an | AP-01 |
| Die Fassadenregel wird unterlaufen | SM-SEC-004 ist nachträglich praktisch nicht mehr durchsetzbar | Regel ab dem ersten Oberflächenmodul automatisiert prüfen, nicht im Review besprechen | AP-05, AP-12 |
| Ein Fehler im ersten Datenbankschema | SM-DAT-007 verbietet die nachträgliche Änderung bestehender Migrationsschritte; der Fehler wird dauerhaft mitgeführt | Erhöhte Sorgfalt und Prüftiefe beim ersten Schema; Sicherung vor jeder Anpassung | AP-04 |
| Flatpak-Berechtigungen fehlen | USB-Export und Schlüsselablage sind unter Linux funktionslos; AK-04 und AK-09 fallen in der Sandbox durch | Berechtigungen als Prüffall führen, nicht als Konfigurationsdetail | AP-21 |
| Signaturinfrastruktur bleibt ungeklärt (OP-04) | macOS verlangt manuelle Freigabe, Windows warnt; SM-PLT-002 und SM-SEC-012 stehen unter Vorbehalt | Entscheidung vor dem ersten Auslieferungsbau erzwingen; der Vorbehalt wird unverändert weitergegeben, nicht ausgelegt | AP-21 |
| Referenzgerät bleibt ungeklärt (OP-08) | Leistungswerte sind messbar, aber nicht abnehmbar; Kapitel 6 bleibt vorläufig | Erstnachweise gegen den erzeugten Prüfbestand führen und nach Festlegung bestätigen | AP-22 |
| Ein dauerhaft rotes Pflicht-Gate | Ein Gate, das immer blockiert, wird umgangen statt befolgt | Bekannte Funde namentlich, begründet und **befristet** in eine Baseline; blockiert wird alles Neue | AP-01 |
| Die Umfangsentscheidung wird als vollständige Abnahme gelesen | Drei Abnahmekriterien, eine Muss-Anforderung und ein Prüfpunkt sind nur teilweise oder nicht anwendbar | Abschnitte 1.3 und 6.3; teilweise abnehmbar wird nie als bestanden protokolliert | — |

---

## 10. Zurückgestellter Umfang

URS-STM-001 Abschnitt 1.4 verlangt für die Streichung von Soll-Anforderungen eine Begründung und
deren Dokumentation. Dieses Kapitel erbringt sie. Die elf nach Abschnitt 1.2 nachgezogenen
Kennungen sind hier nicht mehr aufgeführt.

### 10.1 Zurückgestellte Soll-Anforderungen (68)

| Bereich | Zurückgestellt | Begründung |
|---|---|---|
| Bibliothek | SM-LIB-005, SM-LIB-006, SM-LIB-007 | Sammlungen, intelligente Ordner und Favoriten sind zusätzliche Ordnungsebenen über der Ordnerabbildung; SM-LIB-001 bis SM-LIB-004 tragen den Bestand allein. Folge: die zugehörigen Navigationsgruppen aus DES-STM-001 Abschnitt 6.2 entstehen nicht |
| Import | SM-IMP-004, SM-IMP-006, SM-IMP-007, SM-IMP-008, SM-IMP-010 | Ordnerüberwachung, Ergebnisvorschau, Regelwerk, Ziehen und Ablegen sowie Laufstatistik setzen den lauffähigen Import voraus, den SM-IMP-001 bis 003 liefern. Folge: Der Auslöser der Vorschau-Verwerfung ist der Importlauf, nicht das Dateisystemereignis (AP-09) |
| Formate | SM-FMT-005, SM-FMT-006, SM-FMT-009, SM-FMT-011 | EXP und XXX sowie Rahmen- und eingebettete Vorschaudaten erweitern die Abdeckung über die vier Pflichtformate hinaus, die den Kern der Bibliothek tragen |
| Dokumente | SM-DOC-005, SM-DOC-006, SM-DOC-007, SM-DOC-008, SM-DOC-009 | Seitennavigation, Notizen, Lesezeichen, Dokumenteigenschaften und Bildanhänge bauen auf der Anzeige nach SM-DOC-004 auf; ohne sie bleibt die Anleitung lesbar, nur nicht kommentierbar |
| Metadaten | SM-MET-003, SM-MET-004, SM-MET-006, SM-MET-008 | Schlagwortpflege, Eingabevorschläge, eigene Felder und Bearbeitungsstatus erweitern die Pflichtfelder |
| Vorschau | SM-PRV-004, SM-PRV-006 | Zoom im Detailbereich und Listenansicht sind alternative Darstellungen derselben Daten; SM-PRV-006 steht zusätzlich unter OP-10 |
| Suche | SM-SRC-004, SM-SRC-006 | zusätzliche Filter- und Sortierfelder über die Pflichtfelder hinaus; die Kombinierbarkeit selbst ist mit SM-SRC-010 zugesagt und verplant |
| Stapel | SM-BAT-003, SM-BAT-006 | Einsortierung nach Metadaten und das Protokoll fehlgeschlagener Einzelvorgänge erweitern den geprüften Kern aus SM-BAT-002, SM-BAT-005 und SM-BAT-007 |
| Export | SM-EXP-002, SM-EXP-008, SM-EXP-009, SM-EXP-010 | konkrete Zielformatliste (OP-03), Metadatenexport, Exportpaket und Neuverknüpfung. Folge: der Zustand „Datei nicht auffindbar" hat keinen Ausweg in der Oberfläche (Abschnitt 1.3) |
| Druck | SM-PRN-010, SM-PRN-011, SM-PRN-012, SM-PRN-014 | Kachelung, Copyshop-Ausgabe, Hervorhebung des Kalibrierquadrats und PDF-Bericht setzen den maßhaltigen Einzelseitendruck voraus, der die eigentliche technische Hürde ist. Folge für AK-06 in Abschnitt 6.3 |
| Analyse | SM-KIA-006, SM-KIA-009 | Anzeige des erzeugten Auftrags vor dem Senden und Stapelanalyse mit Fortschritt; die Vorschlagserzeugung selbst (SM-KIA-001) und alle **Schutz**anforderungen sind im Umfang |
| Projekte | SM-PRJ-001, SM-PRJ-002, SM-PRJ-003, SM-PRJ-004 | Kapitel 6.12 des Lastenhefts trägt durchgehend Soll und Kann und ist gegenüber der Bibliotheksverwaltung ein eigenständiger Funktionsblock |
| Maschinen | SM-MAC-001, SM-MAC-002, SM-MAC-003, SM-MAC-004 | Maschinenprofile und Rahmenprüfung setzen die Formatkennwerte aus SM-FMT-008 voraus, die verplant sind; sie sind additiv und ohne Rückwirkung auf den Kern. Folge für Prüfpunkt D-09 in Abschnitt 6.3 |
| Lizenzen des Bestands | SM-LIC-001, SM-LIC-002, SM-LIC-004 | Kapitel 6.15 trägt durchgehend Soll und Kann; es setzt auf den Metadaten auf, ohne sie zu verändern |
| Datensicherung | SM-DAT-002, SM-DAT-004, SM-DAT-005 | Versionsstände, Papierkorb und Änderungsprotokoll sind Bequemlichkeiten über der Sicherung; Sicherung, Wiederanlauf und Migration selbst sind Muss und verplant |
| Einstellungen | SM-SET-007, SM-SET-008 | Die englische Oberfläche ist eine zweite Sprachfassung über der verbindlichen deutschen (SM-SET-006, Muss); der Betriebsmodus trennt Umfang ab, der in Version 1.0 ohnehin nicht entsteht (Kapitel 6.13). Folge: Prüfpunkt D-13 ist gegenstandslos |
| Bestandsübernahme | SM-MIG-001, SM-MIG-002, SM-MIG-003, SM-MIG-004 | Fremdsoftware, Sidecar-Dateien, CSV und JSON sowie Übernahmebericht; der Schutz der Quelldaten (SM-MIG-005) gilt unabhängig davon und ist verplant. Folge für AK-10 in Abschnitt 6.3 |
| Nicht-funktional | SM-NFR-004 | Soll, außerhalb des auf Muss und begründete Nachzüge begrenzten Umfangs; anders als bei den nachgezogenen Kennungen läuft ohne sie keine Muss-Zusage leer. Die Messung wird nach Abschnitt 6.4 als Begleitmessung ohne Abnahmezusage protokolliert, damit ein früher Wert vorliegt |
| Sicherheit | SM-SEC-015 | Datei-Portale unter Linux. Als befristet akzeptiertes Restrisiko mit Datum der erneuten Bewertung in Kapitel 9 geführt. **SM-SEC-010 ist nicht mehr zurückgestellt**, sondern nach Abschnitt 1.2 nachgezogen |
| Lizenzvorgabe | SM-OSS-008, SM-OSS-010, SM-OSS-012, SM-OSS-014 | Stückliste, Lizenzanzeige im Programm, Modelllizenz und Herkunft fremder Datenbestände sind Nachweis- und Anzeigepflichten über die erzwungene Lizenzprüfung hinaus, die als Muss verplant ist. Folge für AK-11 in Abschnitt 6.3. **SM-OSS-008 zusätzlich als Restrisiko in Kapitel 9:** Ohne Stückliste fehlt der Datenbestand, gegen den Schwachstellenmeldungen abgeglichen würden |
| Plattform | SM-PLT-006, SM-PLT-007, SM-PLT-008, SM-PLT-009, SM-PLT-010 | Desktop-Integration, Versionsgleichheit, ein CI-Lauf für alle Plattformen, Reproduzierbarkeit und der Verzicht auf Administratorrechte betreffen die Güte der Auslieferung, nicht ihre Lauffähigkeit. Die **Prüfung** zu SM-PLT-007 entsteht in AP-02 mit; abgenommen wird die Anforderung dadurch nicht |

### 10.2 Kann-Anforderungen (17)

Ausbaustufen ohne Zusage (URS-STM-001 Abschnitt 1.4). Zurückgestellt ohne weitere Begründung:
SM-LIB-008, SM-FMT-007, SM-PRV-005, SM-PRV-008, SM-EXP-011, SM-PRN-013, SM-KIA-003, SM-PRJ-005,
SM-LIC-003, SM-SET-005 sowie das vollständige Kapitel 6.13 des Lastenhefts (SM-MFG-001 bis
SM-MFG-007).

**Zu Kapitel 6.13:** Der gewerbliche Bereich ist über den Betriebsmodus abtrennbar und durchgehend
mit Kann bewertet. Seine Zurückstellung ist keine Vorwegnahme von **OP-02** — sie folgt allein aus
der Umfangsentscheidung dieser Version.

---

## 11. Offene Punkte

Dieses Dokument führt **kein eigenes Register**. Alle offenen Punkte stehen in Kapitel 14 des
Lastenhefts URS-STM-001. Ihre Zuordnung zu den Arbeitspaketen und die Gatterregel stehen in
Kapitel 2.

**OP-14** bis **OP-21** — acht Punkte — sind bei der Prüfung dieses Plans entstanden und dort
aufgenommen worden, nicht hier. Vier davon sind in URS-STM-001 v1.4 entschieden und in
Anforderungen überführt oder präzisiert; vier bleiben offen:

| Gruppe | Punkte |
|---|---|
| Verhalten, das DES-STM-001 verbindlich beschreibt, ohne Kennung im Lastenheft | OP-20 offen; OP-15 bis OP-17 entschieden |
| Reichweite eines vorhandenen Anforderungstextes | OP-14 (Geltung von SM-FMT-012 und SM-SEC-011 für die Anzeige von Fremddokumenten), OP-21 (deckt SM-PRV-002 auch eine Schranke des Zwischenspeichers?) |
| Messanforderung ohne Schwellenwert | OP-18 |
| Zusage, die zwischen zwei bestehende Kennungen fiel | OP-19 entschieden und in SM-SET-002 präzisiert |

Ergibt sich bei der Umsetzung ein weiterer offener Punkt, wird er ebenso **dort** aufgenommen und
hier nur referenziert. Ein zweites Register wäre der sichere Weg zu zwei Ständen derselben Frage.

---

## 12. Pflichtenheft-Vormerkungen

Festlegungen, die nach der Grenze in Abschnitt 0.3 in das Pflichtenheft gehören. Sie sind aus
Prüfbefunden entstanden, in denen die Sache zu Recht als Lücke erkannt, aber auf der falschen
Ebene verortet wurde. Jede Vormerkung nennt, **was** festzulegen ist und **woran** sie hängt.

| Nr | Festzulegen | Hängt an | Herkunft |
|---|---|---|---|
| **PV-01** | Merkmale des erzeugten Prüfbestands aus AP-02: Formatverteilung, Datei- und Stichzahlverteilung, Füllung und Wortvielfalt der Volltextfelder, Anteil Duplikate und defekter Dateien | PF-LIB-09, PF-NFR-01, PF-SRC-07, PF-NFR-02.1 — vier der neun A-Nachweise — sowie PF-IMP-05.1 (Prüfmethode T) | Leistungsprüfung Runde 4 |
| **PV-02** | Zu messende Abfrageformen für SM-SRC-007: neben Schlagwort mit Größenbereich mindestens der teure Fall — häufiger Begriff mit großer Treffermenge, mehrere kombinierte Bereichsfilter, Sortierung | PF-SRC-07 | Leistungsprüfung Runde 4 |
| **PV-03** | Schwellenwerte für Eingabelatenz und Entprellintervall, sobald **OP-18** beantwortet ist | PF-NFR-02.2, PF-SRC-08 | Leistungsprüfung Runde 3 |
| **PV-04** | Signatur und Verhalten der Fassadenschnittstelle im Einzelnen: Form der ausschnittweisen Lieferung, Fehlerbilder, Validierungsregeln je Aufruf | SM-SEC-004, Schnittregel 3 | Leistungs- und Sicherheitsprüfung Runde 4 |
| **PV-05** | Aufbau des Prüfbestands manipulierter Stick- und Dokumentdateien: Herkunft, Mutationsverfahren, Abdeckungsziel des Fuzzings | PF-FMT-12, PF-SEC-11.1, PF-SEC-11.2 | Sicherheitsprüfung Runde 2 |
| **PV-06** | Wortlaut des Ersatztexts für den Zustand „Datei nicht auffindbar", solange SM-EXP-010 zurückgestellt ist | PF-NFR-06, `PF-NFR-15.1` | Design-Prüfung Runde 5 |
| **PV-07** | Form des Ausschnittabrufs an der Fassade: Cursor oder Versatz, samt Begründung der Wahl | Schnittregel 3, PF-PRV-07.1, PF-SRC-07 | Leistungsprüfung Runde 5 |
| **PV-08** | Berechtigungsumfang des Flatpak-Manifests, solange SM-SEC-015 zurückgestellt ist | SM-SEC-009, SM-SEC-014, PF-PLT-04 | Design-Prüfung Runde 7 |
| **PV-09** | Obergrenze der Zwischenspeicherbelegung und Verdrängungsverfahren — Zahlenwert und Verfahren; der Plan legt allein die Bestehbedingung fest (Abschnitt 0.3) | SM-PRV-002, SM-NFR-001, PF-PRV-02 | Leistungsprüfung Runde 13 |

**Diese Liste ist kein Ersatz für einen Nachweis.** Solange eine Vormerkung offen ist, trägt der
zugehörige Prüffall seine Messgröße, aber keine Abnahmebedingung — er wird als Regressionsschwelle
geführt, nicht als bestanden (Abschnitt 6.4).

---

## 13. Änderungshistorie

| Version | Datum | Änderung |
|---|---|---|
| 1.1 | 29.08.2026 | URS-STM-001 v1.4 und DES-STM-001 v1.4 nachgezogen. OP-13 und Weg A bestätigt; OP-07, OP-09, OP-15 bis OP-17 und OP-19 entschieden. Neu verplant: SM-LIB-011, SM-NFR-015 und SM-NFR-016 in AP-12; SM-SET-002 um PF-SET-02.3 ergänzt. Umfang auf 140 Anforderungen (129 Muss und elf nachgezogene Soll) fortgeschrieben; vorläufige OP-Prüffälle in reguläre PF-LIB-, PF-NFR- und PF-SET-Kennungen überführt. Rückverfolgbarkeitsmatrix und Paketabgleich aktualisiert. Nachweis: `Analysis/20260829_01_sprintplanung.md`. |
| 1.0 | 24.08.2026 | Erstfassung. Abgeleitet aus URS-STM-001 v1.3, DES-STM-001 v1.3 und TEC-STM-001 v2.2. Verplant 137 Anforderungen — alle 126 Muss und elf nachgezogene Soll (SM-KIA-001, SM-SRC-009, SM-BAT-005, SM-NFR-009, SM-NFR-013, SM-SEC-010, SM-FMT-010, SM-MET-010, SM-SET-002, SM-SET-003, SM-SET-004) — in 23 Arbeitspaketen und acht Meilensteinen; Rückverfolgbarkeitsmatrix nach URS-STM-001 Abschnitt 13.3; Entscheidungsgatter für die offenen Punkte, ohne einen davon zu entscheiden. Die übrigen 68 Soll- und 17 Kann-Anforderungen sind begründet zurückgestellt. Herleitung: `Analysis/20260823_03_implementierungsplan.md`. |
