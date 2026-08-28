# Analyse — Dauerhafter Vorschau-Zwischenspeicher und seine Verwerfung

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260825-03 |
| Datum | 2026-08-25 |
| Auslöser | Auftrag des Nutzers: „mach weiter mit SM-PRV-002 und SM-PRV-003" |
| Änderungsklasse | **C** — Quellcode |
| Arbeitspaket | AP-09 |
| Vorgänger | [`20260825_02_hintergrundbetrieb.md`](20260825_02_hintergrundbetrieb.md), Abschnitt 9.4 |

## 1. Problembeschreibung

| Kennung | Text | Stand |
|---|---|---|
| SM-PRV-002 (M) | Vorschaubilder müssen **dauerhaft** zwischengespeichert werden | verfehlt |
| SM-PRV-003 (M) | Der Zwischenspeicher muss verworfen werden, **sobald sich die Quelldatei ändert** | fehlt vollständig |

Der bisherige Stand legt Vorschauen unter `std::env::temp_dir()` ab und benennt
sie allein nach der Kennung. Daraus folgen drei Mängel:

1. **Nicht dauerhaft.** Das Temporärverzeichnis wird vom Betriebssystem
   geleert. Nach einem Neustart entstehen alle Vorschauen neu.
2. **Keine Verwerfung.** Ändert sich die Quelldatei, bleibt das alte Bild
   liegen und wird auf ewig ausgeliefert. Der Plan nennt das in AP-09
   ausdrücklich als das größere Risiko: „Die Verwerfung ist der schwierigere
   Teil, nicht die Erzeugung."
3. **Nur eine Auflösung.** AP-09 verlangt, dass der Zwischenspeicher je
   **diskret gestufter** Auflösung hält — nicht je beliebiger Pixelgröße.

## 2. Zusätzlich gefundener Mangel

Der Vorschauweg erhält bisher nur den **Dateinamen**, nicht den Pfad. Die
Fassade löst ihn gegen die Bibliothekswurzel auf. Das trägt ausschließlich, wenn
alle Dateien unmittelbar in der Wurzel liegen — bei jeder Unterordnerstruktur
(SM-LIB-002, SM-LIB-003) findet der Weg die Datei nicht und die Kachel bleibt
dauerhaft leer. Der bisherige Messbestand war flach und hat den Mangel verdeckt.

## 3. Betroffene Komponenten

| Komponente | Art der Berührung |
|---|---|
| `crates/kern-render` | **neu** — `Zwischenspeicher`; laut Modulschnitt trägt `kern/render` „Vorschau aus Stichdaten, **Zwischenspeicher und dessen Verwerfung**" |
| `crates/kern-db` | `Trefferzeile` trägt den Pfad |
| `crates/kern-fassade` | `Kachel` trägt den Pfad; `vorschau` nimmt ihn entgegen |
| `crates/kern-services` | nutzt den Zwischenspeicher statt eines eigenen Ablageorts |
| `crates/ui/src/bruecke.rs` | `Vorschauauftrag` trägt den Pfad |

## 4. Betroffene Anforderungen

**Erfüllt werden:** SM-PRV-002 (M), SM-PRV-003 (M). Nachweise PF-PRV-02 und
PF-PRV-03; Mitwirkung an AK-03.

**Berührt:** SM-LIB-002/003 (Unterordner — Mangel aus Abschnitt 2), SM-SEC-001
bis 003 (der Pfad läuft weiterhin durch `kern-security`), SM-NFR-002 (die
Gültigkeitsprüfung bleibt außerhalb des Zeichenpfads).

## 5. Berührte offene Punkte

| Punkt | Einordnung | Begründung |
|---|---|---|
| **OP-21** | **neutral — nicht vorwegnehmen** | Fragt, ob SM-PRV-002 auch **Obergrenze und Verdrängung** des Zwischenspeichers deckt. Diese Änderung führt deshalb **weder eine Obergrenze noch eine Verdrängung** ein. Verworfen wird ausschließlich, was nachweislich überholt ist — das folgt aus SM-PRV-003 und nicht aus einer Größenannahme |
| **OP-06** | **neutral** | Der Ablageort trägt den Produktnamen. Bis zur Klärung gilt `StitchManager` als Arbeitsname; die Stelle ist im Code benannt |
| OP-08 | grundlagenschaffend | Keine Zahlenwerte zugesagt |

## 6. Vorgeschlagener Ansatz

**Der Schlüssel trägt die Prüfung.** Statt Bild und Gültigkeitsangaben getrennt
zu halten, steht beides im Dateinamen:

```text
<ablage>/vorschau/<kennung[0..2]>/<kennung>_<groesse>_<aenderungszeit>_<stufe>.png
```

Damit ist die Gültigkeitsprüfung **ein** `stat` der Quelldatei und **ein**
`exists` — kein Nebenakten-Lesen, kein Inhaltshash. Ändert sich die Quelldatei,
ändert sich der Name, der alte Stand wird nicht mehr gefunden und beim nächsten
Ablegen entfernt. Genau die zwei Größen, die AP-09 vorschreibt: **Größe und
Änderungszeit**, nie der Inhaltshash — der müsste im Lesepfad je Kachel die
vollständige Datei lesen und bräche die Zusage aus AP-12.

**Streuung über zwei Zeichen der Kennung.** Ein Verzeichnis mit 100.000
Einträgen ist auf mehreren Dateisystemen selbst ein Leistungsproblem, und das
Aufräumen überholter Stände müsste es vollständig durchlaufen. 256 Fächer machen
aus beidem einen beherrschbaren Vorgang.

**Diskrete Auflösungsstufen** (160, 320, 640, 1280 px Kantenlänge). Eine
beliebige Pixelgröße erzeugte je Fensterbreite einen neuen Stand.

**Dauerhafter Ablageort** nach Plattformbrauch, ohne zusätzliche Fremdkiste:
`~/Library/Application Support` · `%LOCALAPPDATA%` · `$XDG_DATA_HOME`.

**Die Prüfung bleibt außerhalb des Zeichenpfads.** Sie läuft im Arbeitsfaden
aus `kern-services`; `data()` sieht ausschließlich den Speicher im
Arbeitsspeicher (SM-NFR-002).

## 7. Prüfplan

- Ein abgelegter Stand wird wiedergefunden — auch von einem **neuen**
  Zwischenspeicherobjekt am selben Ort (Dauerhaftigkeit, SM-PRV-002).
- Ändert sich die **Größe** der Quelldatei, gilt der Stand als überholt.
- Ändert sich die **Änderungszeit** bei gleicher Größe, ebenso.
- Der überholte Stand wird beim nächsten Ablegen **entfernt**, nicht angehäuft.
- Zwei Auflösungsstufen derselben Datei bestehen nebeneinander.
- Eine gelöschte Quelldatei liefert keinen Treffer.
- Vorschauen für Dateien in **Unterordnern** entstehen (Mangel aus Abschnitt 2).
- Der Zwischenspeicher kennt **keine** Obergrenze (OP-21 nicht vorweggenommen).

## 8. Abgrenzung

Nicht Gegenstand: die inkrementelle Änderungserkennung des Importlaufs als
**zweiter** Auslöser (SM-IMP-003, eigenes Paket) und die Ordnerüberwachung
(SM-IMP-004, zurückgestellt). Diese Änderung liefert den Auslöser **beim Lesen**,
den AP-09 gerade deshalb verlangt, weil die Ordnerüberwachung zurückgestellt ist.

---

## 9. Abschluss (Phase 4)

### 9.1 Umsetzung

`kern-render::Zwischenspeicher` legt Vorschauen am dauerhaften Ort des
Betriebssystems ab. Der Schlüssel trägt die Prüfung:

```text
<ablage>/vorschau/<kennung[0..2]>/<kennung>_<groesse>_<aenderungszeit>_<stufe>.png
```

Die Gültigkeitsprüfung ist damit **ein** `stat` der Quelldatei und **ein**
`exists` — kein Nebenaktenlesen, kein Inhaltshash. Sie läuft im Arbeitsfaden aus
`kern-services`, nie im Zeichenpfad (AP-09, AP-12, SM-NFR-002).

Vier Auflösungsstufen (160/320/640/1280 px). `Stufe::fuer_breite` rundet
**auf**, nie ab: Hochskalieren ist nach DES-STM-001 Abschnitt 11 unzulässig.

### 9.2 Drei Mängel, die dabei gefunden wurden

**1 — Dateien in Unterordnern bekamen nie eine Vorschau.** Der Vorschauweg
erhielt nur den **Dateinamen** und löste ihn gegen die Bibliothekswurzel auf.
Das trägt ausschließlich bei flacher Ablage; bei jeder Unterordnerstruktur
(SM-LIB-002, SM-LIB-003) blieb die Kachel dauerhaft leer. Der bisherige
Messbestand war flach und hat den Mangel verdeckt — der Prüfbestandserzeuger
streut die Dateien jetzt über sechzehn Unterordner.

**2 — Ein Ablegen löschte die anderen Auflösungsstufen.** Die erste Fassung
entfernte beim Ablegen *jeden* anderen Stand derselben Kennung, also auch die
Stufen desselben Quellstands. AP-09 verlangt ausdrücklich, dass der Speicher je
diskret gestufter Auflösung hält. Verworfen wird jetzt, was zu einer **anderen
Fassung der Quelldatei** gehört. Zwei Prüffälle haben das aufgedeckt.

**3 — Ein relativer Pfad wurde gegen das Arbeitsverzeichnis aufgelöst.** Die
Gültigkeitsprüfung las damit die Kennzeichen der falschen Datei — beziehungsweise
gar keiner. Der Pfad läuft jetzt zuerst durch `Fassade::pfad_pruefen` und damit
durch `kern-security` (Schnittregel 5), bevor er das Dateisystem berührt.

### 9.3 Nachweis

**Zwanzig Prüffälle** in `kern-render`, darunter:

- Ein **neu geöffneter** Speicher am selben Ort findet den Stand wieder — das
  ist die eigentliche Zusage aus SM-PRV-002.
- Der Standardort liegt nachweislich **nicht** im Temporärverzeichnis.
- Geänderte **Größe** verwirft den Stand.
- Geänderte **Änderungszeit bei gleicher Größe** verwirft ihn ebenfalls — der
  schwierigere Fall, der ohne die zweite Größe unbemerkt bliebe.
- Überholte Stände werden **entfernt**, nicht angehäuft (nach sechs Änderungen
  bleibt genau einer).
- Zwei Auflösungsstufen bestehen nebeneinander.
- Der Speicher kennt **keine Obergrenze**: 300 gültige Stände bleiben
  vollständig erhalten (OP-21 nicht vorweggenommen).

**Drei Prüffälle** in `kern-services` belegen das Zusammenspiel ohne Zeitmessung:
Nach dem ersten Lauf wird die abgelegte Datei durch eine erkennbare Marke
ersetzt. Kommt sie zurück, stammt die Antwort aus dem Zwischenspeicher; kommt
nach einer Änderung der Quelldatei ein neues PNG, hat die Verwerfung gegriffen.

**An der laufenden Anwendung** (`scripts/check-hintergrund.sh`, 8.000 Dateien in
sechzehn Unterordnern):

| Prüfung | Ergebnis |
|---|---|
| Bedienung während des Einlesens | 120 von 120 Takten, beide Läufe |
| Abgelegte Vorschauen nach Lauf 1 | 24 |
| Marke überlebt Lauf 2 | ja — der Speicher wurde wiederverwendet |

Gesamtstand: **142 bestandene Prüffälle**, 0 fehlgeschlagen. `cargo fmt` sauber,
`cargo clippy --all-targets` ohne Meldung, Projektregeln PASS.

### 9.4 Was offen bleibt

- **Der zweite Auslöser aus AP-09** — die inkrementelle Änderungserkennung des
  Importlaufs — gehört zu SM-IMP-003 und ist nicht Gegenstand dieser Änderung.
  Der Auslöser **beim Lesen**, den AP-09 gerade wegen der zurückgestellten
  Ordnerüberwachung verlangt, ist umgesetzt.
- **Eine Änderung, die Größe *und* Änderungszeit unberührt lässt**, bleibt
  unbemerkt. Das ist die bewusste Grenze aus AP-09: Der Inhaltshash bliebe dem
  Importlauf vorbehalten, weil er im Lesepfad je Kachel die volle Datei läse.
- **OP-21** ist unverändert offen. Obergrenze und Verdrängung sind deshalb
  **nicht** umgesetzt; ein Prüffall hält das fest.
- **OP-06** bestimmt den Verzeichnisnamen. Bis zur Klärung gilt der Arbeitsname
  `StitchManager`; die Stelle ist im Code als solche benannt.
- Verlässt ein Eintrag den Bestand, wird `verwerfen` noch nicht gerufen — es
  gibt bisher keinen Löschweg (SM-DAT-003, eigenes Paket).
