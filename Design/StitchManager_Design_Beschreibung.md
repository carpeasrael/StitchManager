# StitchManager — Design-Beschreibung für die Entwicklung

**Kennung:** DES-STM-001
**Version:** 1.4
**Datum:** 29.08.2026
**Führendes Dokument:** URS-STM-001 (Lastenheft) v1.4
— dieses Dokument konkretisiert dessen Kapitel 12
**Mitgeltend:** TEC-STM-001 (Tech-Stack) v2.3 · stitchmanager-mockup.html
**Umsetzung:** Qt 6 (LGPL-3), siehe TEC-STM-001 Abschnitt 2

---

## 1. Zweck und Verbindlichkeit

Dieses Dokument beschreibt das visuelle und interaktive Verhalten der Oberfläche so genau,
dass es ohne Rückfragen umgesetzt werden kann. Es ist die **verbindliche Quelle** für Farben,
Schrift, Abstände, Zustände und Komponentenverhalten.

**Rangfolge bei Widersprüchen:**

1. **URS-STM-001 (Lastenheft)** — was das System leisten muss
2. Dieses Dokument — wie es aussieht und sich verhält
3. Das Mockup `stitchmanager-mockup.html` (visuelle Referenz, nicht Umsetzungsvorlage)
4. Der Kreuznaht-Markenstandard

Dieses Dokument darf keine Anforderung begründen, die im Lastenheft fehlt. Ergibt sich beim
Gestalten eine neue Anforderung, wird sie **dort** aufgenommen und hier nur referenziert.

**Wichtiger Vorbehalt:** Die Farbwerte in Abschnitt 3 sind aus dem Kreuznaht-Markenstandard
rekonstruiert, weil `theme-standard.json` bei der Erstellung nicht vorlag. Der Auftraggeber hat
am 29.08.2026 bestätigt, dass sie noch nicht markenseitig abgeglichen sind (URS-STM-001,
entschiedener OP-09). Sie bleiben bis dahin vorläufig und dürfen keine visuelle Markenabnahme
begründen. **Die Variablennamen bleiben unverändert** — entwickelt wird gegen Namen, nie gegen
wiederholte Farbwerte.

---

## 2. Gestaltungsgrundsätze

**Warm, nicht neutral.** Kein Grauton der Oberfläche ist farblos. Flächen tragen Creme- und
Sandtöne, im Dunkelmodus Espressotöne. Ein neutrales Grau ist immer ein Fehler, kein Sparansatz.

**Die Naht ist das Signaturelement.** Trennlinien zwischen Bereichen sind gestrichelte Nähte,
keine durchgezogenen Striche. Die Auswahl wird mit einem Kreuzstich markiert, nicht mit einem
Häkchen oder Farbbalken. Das ist kein Zierrat — es ist das einzige Element, das die Oberfläche
mit dem Gegenstand verbindet, und es wird nicht wegoptimiert.

**Ruhe in der Mitte.** Die Musterauswahl zeigt Bild, Name und Größe. Sonst nichts. Jede weitere
Angabe gehört in den Detailbereich. Diese Regel ist der Kern der Aufteilung und wird auch dann
nicht aufgeweicht, wenn eine Angabe „ja auch praktisch wäre".

**Zahlen in Festbreite.** Maße, Stichzahlen, Garnnummern und Kennungen stehen in der
Festbreitenschrift. Sie sollen untereinander bündig stehen und beim Blättern nicht springen.

**Farbe trägt nie allein.** Jeder Zustand hat neben der Farbe ein zweites Merkmal — Text,
Symbol oder Form.

---

## 3. Farbvariablen

Sämtliche Farben werden ausschließlich über diese Bezeichner verwendet. **Kein Literalwert im
Komponentencode.**

### 3.1 Hellmodus

| Bezeichner | Wert | Verwendung |
|---|---|---|
| `--kn-bg` | `#faf6f0` | Fensterhintergrund, mittlere Spalte |
| `--kn-surface` | `#ffffff` | Kacheln, Detailbereich, Eingabefelder |
| `--kn-surface-2` | `#f4ede2` | Navigationsspalte, Werkzeugleiste, Statusleiste |
| `--kn-surface-3` | `#ebe1d3` | aktiver Navigationseintrag, gedrückte Schaltfläche |
| `--kn-border` | `#e0d4c2` | Standardrahmen |
| `--kn-border-strong` | `#c9b9a2` | Rahmen bei Zeigerkontakt, Fensterrahmen |
| `--kn-ink` | `#2e2a27` | Fließtext, Überschriften |
| `--kn-ink-2` | `#6e6259` | Sekundärtext, Abschnittslabels, Zähler, Feldbeschriftungen |
| `--kn-ink-3` | `#8d8177` | **nur deaktivierte Zustände und Platzhalter — nie lesbarer Text** |
| `--kn-brand` | `#e85d5d` | Terracotta als Fläche: Hauptschaltfläche, Auswahlmarke, Schalter |
| `--kn-brand-ink` | `#c2452f` | Terracotta als Textfarbe |
| `--kn-brand-soft` | `#fbe7e2` | Auswahlring der Kachel |
| `--kn-amber` | `#d98c1f` | Bernstein als Fläche, zweiter Akzent |
| `--kn-amber-soft` | `#fbeed6` | Hintergrund der Warnhinweisbox |
| `--kn-ok` | `#3b754a` | Erfolgstext, USB verbunden |
| `--kn-ok-soft` | `#e3f0e5` | Hintergrund der Erfolgshinweisbox |
| `--kn-warn` | `#955e0e` | Warntext |
| `--kn-ki` | `#6b5b8f` | Kennzeichnung maschinell erzeugter Inhalte |
| `--kn-ki-soft` | `#eee9f5` | Hintergrund KI-Hinweis |
| `--kn-on-brand` | `#2b1a15` | **einzige** Textfarbe auf `--kn-brand`-Flächen, in beiden Modi |
| `--kn-seam` | `#cbbaa4` | Farbe der Nahtlinien |

### 3.2 Dunkelmodus — Espresso, nicht Blauschwarz

| Bezeichner | Wert | Anmerkung |
|---|---|---|
| `--kn-bg` | `#221b17` | Espresso |
| `--kn-surface` | `#2b231e` | |
| `--kn-surface-2` | `#251e19` | Navigation liegt hier **dunkler** als die Fläche, im Hellmodus heller — der Kontrastsprung dreht sich um |
| `--kn-surface-3` | `#3a2f27` | |
| `--kn-border` | `#443830` | |
| `--kn-border-strong` | `#5c4c40` | |
| `--kn-ink` | `#f3e9dd` | warmes Weiß, kein reines `#ffffff` |
| `--kn-ink-2` | `#b3a294` | |
| `--kn-ink-3` | `#91816f` | nur deaktiviert |
| `--kn-brand` | `#f0836e` | aufgehellt — `#e85d5d` erreicht auf Espresso zu wenig Kontrast |
| `--kn-brand-ink` | `#f0836e` | im Dunkelmodus identisch mit der Flächenfarbe |
| `--kn-brand-soft` | `#402620` | |
| `--kn-amber` | `#e5b463` | |
| `--kn-amber-soft` | `#3b2d18` | |
| `--kn-ok` | `#7fb98c` | |
| `--kn-ok-soft` | `#23331f` | |
| `--kn-warn` | `#e0ac5a` | |
| `--kn-ki` | `#b3a3d6` | |
| `--kn-ki-soft` | `#2f2839` | |
| `--kn-on-brand` | `#2b1a15` | unverändert — trägt auf beiden Terracotta-Tönen |
| `--kn-seam` | `#5c4c40` | |

### 3.3 Nachgerechnete Kontraste

Die folgenden Werte sind gemessen, nicht geschätzt. Alle Textpaare erreichen mindestens 4,5:1.

| Paar | Hell | Dunkel |
|---|---|---|
| Fließtext auf Fensterhintergrund | 13,21:1 | 14,15:1 |
| Fließtext auf Kachel | 14,22:1 | 12,86:1 |
| Sekundärtext auf Fensterhintergrund | 5,49:1 | 6,88:1 |
| Label in der Navigation | 5,08:1 | 6,66:1 |
| Markentext auf Fensterhintergrund | 4,66:1 | 6,61:1 |
| Erfolgstext in Hinweisbox | 4,66:1 | 5,89:1 |
| Warntext in Hinweisbox | 4,71:1 | 6,49:1 |
| KI-Text in Hinweisbox | 5,01:1 | 6,15:1 |
| Text auf Terracotta-Fläche (`--kn-on-brand`) | 4,88:1 | 6,48:1 |

**Drei Regeln, die daraus folgen und einzuhalten sind:**

1. **`--kn-brand` erreicht als Textfarbe im Hellmodus nur 3,17:1.** Für Text ist ausschließlich
   `--kn-brand-ink` zu verwenden. `--kn-brand` ist Flächen, Rahmen und Symbolen vorbehalten.
2. **`--kn-ink-3` ist für lesbaren Text gesperrt.** Es gibt im Hellmodus keine dritte Textstufe,
   die AA hält, ohne von `--kn-ink-2` ununterscheidbar zu werden. Abschnittslabels erhalten
   ihre Nachordnung deshalb über Schriftgröße, Versalien und Sperrung — nicht über Farbe.
3. **Weiße Schrift auf `--kn-brand` ist unzulässig.** Sie erreicht im Hellmodus 3,41:1 und im
   Dunkelmodus 2,57:1 und verfehlt SM-NFR-007 in beiden Fällen — die Hauptschaltfläche wäre
   damit nicht abnahmefähig. Text und Symbole auf Terracotta stehen ausschließlich in
   `--kn-on-brand`; dieser Wert trägt auf beiden Terracotta-Tönen (4,88:1 hell, 6,48:1 dunkel)
   und ist deshalb in beiden Modi derselbe.

---

## 4. Typografie

| Rolle | Schrift | Verwendung |
|---|---|---|
| Auszeichnung | **Josefin Sans**, 600 | Marke, Detailüberschrift, Abschnittslabels |
| Fließtext | **Lato**, 400 / 600 / 700 | alle Beschriftungen, Fließtext, Schaltflächen |
| Festbreite | **IBM Plex Mono**, 400 / 500 | Maße, Stichzahlen, Garnnummern, Kennungen, Zähler |

**Größenstufen** (`t-xs` bis `t-xl`):

| Stufe | Größe | Verwendung |
|---|---|---|
| `t-xs` | 11 px | Abschnittslabels, Zähler, Marken, Hinweistext |
| `t-sm` | 12 px | Schaltflächen, Feldbeschriftungen, Statusleiste |
| `t-md` | 13 px | Fließtext, Eingabefelder, Kachelname |
| `t-lg` | 15 px | Markenschriftzug |
| `t-xl` | 19 px | Detailüberschrift |

**Zeilenhöhe** 1,45 für Fließtext, 1,25 für den Kachelnamen.
**Abschnittslabels** sind versal gesetzt mit 0,13 em Sperrung.

**Zwei Vorbehalte:**

- **Lizenz.** Josefin Sans, Lato und IBM Plex Mono stehen unter der SIL Open Font License und
  erfüllen SM-OSS-007. Sie werden mitgeliefert; kein Nachladen von einem externen Dienst
  (SM-DES-004).
- **IBM Plex Mono ist eine freigegebene Ergänzung.** Der Kreuznaht-Markenstandard sieht keine
  Festbreitenschrift vor; der Auftraggeber hat ihre Verwendung am 29.08.2026 bestätigt
  (URS-STM-001, entschiedener OP-07).

---

## 5. Maße, Abstände, Formen

**Grundraster 4 px.** Alle Abstände sind Vielfache davon; Ausnahmen sind zu begründen.

| Größe | Wert | Verwendung |
|---|---|---|
| Radius klein | 4 px | Eingabefelder, Marken, Farbfelder |
| Radius mittel | 7 px | Kacheln, Hinweisboxen, Statistikfelder |
| Radius groß | 11 px | Fensterrahmen |
| Radius Pille | voll | Schaltflächen, Chips, Schlagworte, Schalter |
| Rahmenstärke | 1 px | durchgehend; keine 2-px-Rahmen außer beim Fokusring |

| Bereich | Maß |
|---|---|
| Werkzeugleiste | 46 px hoch |
| Statusleiste | 26 px hoch |
| Navigationsspalte | 190–226 px, Vorgabe 226 px |
| Musterauswahl | ab 300 px, wächst mit Faktor 1,25 |
| Detailbereich | ab 290 px, wächst mit Faktor 0,95 |
| Trenner | 1 px sichtbar, **Ziehfläche mindestens 6 px**, unsichtbare Trefferzone 32 px breit |
| Fenster Mindestbreite | 860 px |

---

## 6. Layout

### 6.1 Dreiteilung — senkrecht, unbedingt

```text
┌────────────────────────────────────────────────────────────┐
│ Werkzeugleiste                                       46 px │
├──────────┬─────────────────────────┬───────────────────────┤
│ Übersicht│  Musterauswahl          │  Details              │
│    und   │  Bild · Name · Größe    │  Farben · Größe ·     │
│ Navigation│                        │  Optionen             │
│  226 px  │  ≥ 300 px               │  ≥ 290 px             │
├──────────┴─────────────────────────┴───────────────────────┤
│ Statusleiste                                         26 px │
└────────────────────────────────────────────────────────────┘
```

**Die drei Bereiche stehen in jeder Fensterbreite nebeneinander (SM-DES-005, SM-DES-006).**
Es gibt keine gestapelte Ersatzdarstellung. Unterschreitet das Fenster 860 px, wird waagerecht gescrollt.

Die Trenner sind Ziehgriffe. Eingestellte Breiten überleben den Programmneustart (SM-SET-003).

### 6.2 Linke Spalte — Übersicht und Navigation

Von oben nach unten: **Übersichtskarte** (Gesamtbestand als große Festbreitenzahl,
Formatverteilung als Balken mit Legende; SM-LIB-011), dann durch Nähte getrennt die Gruppen
**Bibliothek**, **Ordner**, **Intelligente Ordner**, **Arbeit**.

**Betriebsmodus (SM-SET-008):** Im Standardmodus enthält die Gruppe „Arbeit" ausschließlich
**Projekte**. Die Einträge **Fertigung** und **Beschaffung** sind vollständig ausgeblendet —
nicht ausgegraut, nicht leer, nicht vorhanden. Enthält die Gruppe dadurch nur einen Eintrag,
entfällt die Gruppenüberschrift und der Eintrag rückt in die Gruppe „Bibliothek".

Jeder Eintrag: Kreuzstichmarke (nur beim aktiven Eintrag sichtbar), Beschriftung, Zähler
rechtsbündig in Festbreite. Aktiver Eintrag: Fläche `--kn-surface-3`, Text `--kn-ink`, fett.
Unterordner um 20 px eingerückt.

### 6.3 Mittlere Spalte — Musterauswahl

**Kopfzeile:** aktive Filter als Chips mit Entfernen-Kreuz, Chip „Zurücksetzen" gestrichelt
umrandet in `--kn-brand-ink`, rechts Trefferzahl und Sortierung in Festbreite.

**Kachelraster:** Spaltenbreite mindestens 158 px, füllt automatisch auf, Abstand 13 px,
Innenabstand 14 px. Bei engem Fenster sinkt die Mindestbreite auf 136 px.

**Aufbau einer Kachel:**

| Element | Vorgabe |
|---|---|
| Vorschaufläche | Seitenverhältnis 1:1, Hintergrund `--kn-bg`, unten 1 px Rahmen |
| Formatmarke | unten links auf der Vorschau, Festbreite 9 px, versal, gesperrt |
| KI-Marke | unten rechts, nur wenn Metadaten maschinell erzeugt wurden |
| Name | 13 px, fett, **höchstens zwei Zeilen**, danach Auslassungszeichen; feste Mindesthöhe 2,5 em, damit die Kacheln bündig stehen |
| Größe | Festbreite 11 px, `--kn-ink-2`, Format `B × H mm` |

**Mehr steht nicht auf der Kachel (SM-DES-007).** Keine Stichzahl, kein Datum, keine Schlagworte.

**Zustände:** Zeigerkontakt → Rahmen `--kn-border-strong`, 1 px angehoben.
Ausgewählt → Rahmen `--kn-brand`, 2-px-Ring in `--kn-brand-soft`, Kreuzstichmarke oben rechts.

### 6.4 Rechte Spalte — Details

Reihenfolge fest: **Vorschau** (mit Zoom-Schaltern oben rechts) → **KI-Hinweis**, sofern
Vorschläge vorliegen → **Angaben** → **Größe und technische Werte** → **Farben** →
**Optionen** → **Aktionen** → **Schlagworte** → **Projekte**.

**Technische Werte** als zweispaltiges Raster aus Statistikfeldern: Beschriftung 11 px in
`--kn-ink-2`, Wert 13 px in Festbreite. Breite und Höhe stehen **getrennt**, jeweils mit
einer Nachkommastelle und Einheit.

**Farben** als Liste, eine Zeile je Garn: Farbfeld 20 px mit diagonaler Stichschraffur,
Garnname, Garnnummer in Festbreite, Stichanteil rechtsbündig in Festbreite. Die Schraffur
ist verbindlich — ein glattes Farbfeld liest sich als Farbwähler, nicht als Garn.

**Optionen** als Zeilenliste, Beschriftung links auf 40 % Breite, Bedienelement rechtsbündig,
1-px-Trenner zwischen den Zeilen, keiner nach der letzten. Aufzuklappende Auswahl als
abgesetzte Fläche mit Pfeil, Ja/Nein als Schalter 34 × 19 px.

**Rahmenprüfung (SM-MAC-002):** Unter den Optionen erscheint eine Hinweisbox, die die
Musterabmessung gegen das eingestellte Stickfeld des aktiven Maschinenprofils rechnet.
Passt es, grün mit ausgewiesener Reserve. Passt es nicht,
bernsteinfarben mit Angabe der Überschreitung und einem Vorschlag zum größeren Rahmen.
Diese Box wird immer angezeigt, nie nur im Fehlerfall — ihre Abwesenheit wäre sonst
mehrdeutig.

### 6.5 Statusleiste

Links Bestand, Treffer und Auswahl in Festbreite. Rechts Datenträgerstatus (mit Kreuzstich,
in `--kn-ok`, sobald verbunden) und Bibliotheksstatus. Bei laufenden Vorgängen ersetzt ein
Fortschrittsbalken mit Abbruchmöglichkeit die rechte Gruppe.

---

## 7. Komponenten und Zustände

| Komponente | Ruhe | Zeigerkontakt | Aktiv/Ausgewählt | Deaktiviert |
|---|---|---|---|---|
| Schaltfläche Standard | Fläche `--kn-surface`, Rahmen `--kn-border` | Rahmen `--kn-border-strong` | Fläche `--kn-surface-3` | Text `--kn-ink-3`, Rahmen bleibt |
| Schaltfläche Haupt | Fläche `--kn-brand`, Text `--kn-on-brand` | 8 % abgedunkelt | 14 % abgedunkelt | 40 % Deckkraft |
| Schaltfläche KI | Fläche `--kn-ki-soft`, Rahmen und Text `--kn-ki` | Rahmen kräftiger | — | wie Standard |
| Navigationseintrag | Text `--kn-ink-2` | Fläche 55 % `--kn-surface-3` | Fläche `--kn-surface-3`, fett, Kreuzstich sichtbar | — |
| Kachel | Rahmen `--kn-border` | Rahmen `--kn-border-strong`, −1 px Versatz | Rahmen `--kn-brand` + Ring | — |
| Eingabefeld | Fläche `--kn-bg`, Rahmen `--kn-border` | — | Rahmen `--kn-brand` | Fläche `--kn-surface-3` |
| Eingabefeld mit KI-Wert | Fläche `--kn-ki-soft`, Rahmen `--kn-ki` | — | wie oben | — |
| Schalter | Fläche `--kn-surface-3` | — | Fläche `--kn-brand`, Knopf rechts | 40 % Deckkraft |
| Chip / Schlagwort | Fläche `--kn-surface-3` | — | — | — |

**Fokus:** 2 px `--kn-brand`, 2 px Abstand, immer außen liegend. Der Fokusring wird **nie**
unterdrückt, auch nicht bei Zeigerbedienung. Er ist an jedem bedienbaren Element sichtbar.

**Mindestgröße für Bedienelemente (SM-NFR-016):** 26 × 26 px sichtbar, Trefferfläche
mindestens 32 × 32 px. Das gilt auch für Trenner: Die sichtbare Linie und die 6-px-Ziehfläche
bleiben schlank, liegen aber mittig in einer 32 px breiten unsichtbaren Trefferzone.

---

## 8. Bewegung

| Vorgang | Dauer | Verlauf |
|---|---|---|
| Farbwechsel bei Zeigerkontakt | 150 ms | ease |
| Themenwechsel hell/dunkel | 250 ms | ease |
| Schalter | 180 ms | ease |
| Kachelversatz | 150 ms | ease |

Ist die Systemeinstellung für reduzierte Bewegung aktiv, entfallen **alle** Übergänge und
Bewegungen ersatzlos (SM-NFR-013). Keine Ausnahme für „dezente" Übergänge.

---

## 9. Kennzeichnung maschinell erzeugter Inhalte

Maschinell erzeugte Werte sind **doppelt** gekennzeichnet — über Farbe und über Text
(SM-DES-009, SM-KIA-008):

- **Feld:** Fläche `--kn-ki-soft`, Rahmen `--kn-ki`, Beschriftung ergänzt um „· KI-Vorschlag"
- **Kachel:** Marke „KI" unten rechts auf der Vorschau
- **Schlagwort:** gestrichelter Rahmen in `--kn-ki`, Beschriftung endet auf „· KI"
- **Hinweisbox** über den Angaben, solange unbestätigte Vorschläge vorliegen, mit
  „Alle übernehmen" und „Alle verwerfen"

Die Kennzeichnung verschwindet erst, wenn der Wert bestätigt wurde. Ein maschinell erzeugter
Wert, den niemand angesehen hat, darf nie wie ein gepflegter aussehen.

---

## 10. Leere, ladende und fehlerhafte Zustände

Alle folgenden Zustände und der konsistente Abbruch langer Vorgänge tragen SM-NFR-015.

| Fall | Darstellung |
|---|---|
| Bibliothek leer | Kreuzstich groß in `--kn-border-strong`, Text „Noch keine Muster", Hauptschaltfläche „Ordner importieren" |
| Filter ohne Treffer | Text „Kein Muster passt zu diesen Filtern", darunter Schaltfläche „Filter zurücksetzen" |
| Vorschau lädt | Platzhalterfläche in `--kn-bg`, ruhiger Puls; **kein Springen des Layouts**, die Kachelhöhe steht vorher fest (SM-PRV-009) |
| Datei nicht lesbar | Kachel mit bernsteinfarbenem Rahmen, Marke „Fehler" statt Format; Detailbereich nennt den Grund im Klartext |
| Datei nicht auffindbar | wie oben, zusätzlich Schaltfläche „Neu verknüpfen" (SM-EXP-010) |
| Langer Vorgang | Fortschritt in der Statusleiste mit Abbruch, nicht als modaler Dialog |

Fehlertexte sind für Endnutzer formuliert (SM-NFR-006). Technische Angaben gehören ins
Protokoll, nicht in die Oberfläche.

---

## 11. Umsetzung in Qt

| Punkt | Vorgabe |
|---|---|
| Variablenablage | **Eine** Datei als einzige Quelle der Farb-, Schrift- und Abstandswerte. Kein Literalwert in Komponenten (SM-DES-003). |
| Themenwechsel | Zur Laufzeit ohne Neustart. Systemeinstellung als Vorgabewert; die manuelle Übersteuerung bleibt über Neustarts erhalten (SM-SET-002). |
| Schriften | Als Ressource eingebettet, zur Laufzeit registriert. Keine Abhängigkeit von systemseitig installierten Schriften. |
| Hohe Auflösung | Alle Symbole als Vektor. Die Stichvorschau wird bei Bedarf neu gezeichnet, nicht hochskaliert. |
| Listen | Musterauswahl und Farbliste virtualisiert (SM-PRV-007). Die Kachelhöhe steht vor dem Laden der Vorschau fest. |
| Fremdtext | Dateinamen, Metadaten und maschinell erzeugte Texte werden als **Nur-Text** dargestellt. Auszeichnungsfähige Textelemente sind an diesen Stellen auf Nur-Text zu stellen (SM-SEC-008). |
| Druckfarben | Die Vorgaben dieses Dokuments gelten für den Bildschirm. Der Druck erfolgt maßhaltig und ohne Themenfarben — hell auf weiß, unabhängig vom eingestellten Modus (SM-PRN-015). |
| Tastatur | Jedes Element erreichbar, jeder Dialog hält den Fokus und gibt ihn beim Schließen zurück (SM-NFR-008). |

---

## 12. Abnahmeliste

| Nr | Prüfung |
|---|---|
| D-01 | Die drei Bereiche stehen bei 860 px, 1280 px und 2560 px Fensterbreite nebeneinander. Keine gestapelte Darstellung. |
| D-02 | Auf der Kachel stehen ausschließlich Bild, Format-/KI-Marke, Name und Größe. |
| D-03 | Der Themenwechsel wirkt auf alle Bildschirme und Dialoge; kein Element bleibt unlesbar (AK-07). |
| D-04 | Alle Textfarben erreichen gemessen mindestens 4,5:1; kein Text in `--kn-ink-3`. |
| D-05 | Kein Literalfarbwert außerhalb der Variablendatei — automatisiert prüfbar. |
| D-06 | Jedes bedienbare Element zeigt bei Tastaturnavigation den Fokusring. |
| D-07 | Bei aktiver Einstellung für reduzierte Bewegung findet keine Bewegung statt. |
| D-08 | Maschinell erzeugte Werte sind ohne Farbwahrnehmung als solche erkennbar. |
| D-09 | Die Rahmenprüfung erscheint in beiden Fällen — passend und nicht passend (SM-MAC-002). |
| D-13 | Im Standardmodus sind Fertigung und Beschaffung in der Navigation nicht vorhanden (SM-SET-008). |
| D-10 | Beim Blättern durch 10 000 Kacheln springt kein Layout und keine Kachelhöhe. |
| D-11 | Nähte und Kreuzstichmarken sind an allen vorgesehenen Stellen vorhanden. |
| D-12 | Der Ausdruck trägt keine Themenfarben und ist maßhaltig. |

---

## 13. Offene Punkte

Dieses Dokument führt **kein eigenes Register**. Alle offenen Punkte stehen in Kapitel 14 des
Lastenhefts URS-STM-001. Gestaltungsrelevant sind dort:

| Nr | Frage | Wirkung auf dieses Dokument |
|---|---|---|
| **OP-10** | Bleibt die Listenansicht neben der Kachelansicht bestehen? | Entfällt sie, wird der Umschalter aus der Werkzeugleiste gestrichen. |
| **OP-11** | Erhält der Dunkelmodus ein eigenes Anwendungssymbol? | Betrifft die Ikonografie, nicht das Fenster. |
| **OP-12** | Zeigt die Übersichtskarte weitere Kennzahlen? | Betrifft Abschnitt 6.2. Weitere Kennzahlen kosten Höhe, die dem Ordnerbaum fehlt. |
| **OP-20** | Erhalten die Sammelaktionen der Hinweisbox eine eigene Anforderung? | Betrifft Abschnitt 9. „Alle übernehmen“ und „Alle verwerfen“ sind hier verbindlich beschrieben, ohne dass eine Kennung sie trägt. |

Das frühere eigene Register dieses Dokuments (Kennungen D1 bis D5) ist aufgelöst; seine
Einträge entsprechen der Reihe nach OP-07, OP-09, OP-10, OP-11 und OP-12. Die am 29.08.2026
entschiedenen Punkte OP-07, OP-09, OP-15 bis OP-17 und OP-19 stehen weiterhin nachvollziehbar
in URS-STM-001 Abschnitt 14.2. Ihre Folgen sind hier umgesetzt: Festbreitenschrift freigegeben,
Farbwerte vorläufig, Abnahmebezug über SM-NFR-015, SM-NFR-016 und SM-LIB-011 sowie
neustartbeständige Darstellungswahl in SM-SET-002. Offen bleiben in diesem Dokument nur die
oben aufgeführten Punkte.

---

## 14. Änderungshistorie

| Version | Datum | Änderung |
|---|---|---|
| 1.4 | 29.08.2026 | Entscheidungen aus URS-STM-001 v1.4 nachgezogen: IBM Plex Mono freigegeben; Farbwerte bis zum Markenabgleich als vorläufig gekennzeichnet; Übersichtskarte an SM-LIB-011, Zustände und Abbruch an SM-NFR-015 sowie Komponentenzustände und Trefferflächen an SM-NFR-016 gebunden. Trenner besitzen bei 1 px sichtbarer Linie und mindestens 6 px Ziehfläche eine 32 px breite unsichtbare Trefferzone. Die manuelle Darstellungswahl übersteht Neustarts nach SM-SET-002. Mitgeltende Verweise auf URS-STM-001 v1.4 und TEC-STM-001 v2.3 nachgezogen. |
| 1.3 | 24.08.2026 | Abschnitt 13 um **OP-15, OP-16, OP-17, OP-19 und OP-20** ergänzt — Verweise, kein eigenes Register. Alle fünf betreffen Verhalten, das dieses Dokument verbindlich beschreibt, ohne dass URS-STM-001 eine Kennung dafür führt; sie tragen dieselbe Frist wie OP-07, OP-09 und OP-12. Ohne den Eintrag sähe an der maßgeblichen Stelle niemand, dass die Vorgaben derzeit ohne Abnahmebezug sind. Aufgeworfen bei der Design-Prüfung des Implementierungsplans (Runde 14). Nachweis: `Analysis/20260823_03_implementierungsplan.md`. |
| 1.2 | 23.08.2026 | Farbkorrektur nach einem Befund der Stufe-1-Prüfung (siehe `Analysis/20260823_01_gate-befunde-rueckstand.md`, Befund Curie B-1): Regel 3 in Abschnitt 3.3 erlaubte weiße Schrift auf `--kn-brand`. Nachgerechnet erreicht sie 3,41:1 (hell) und 2,57:1 (dunkel) und verfehlt SM-NFR-007. Neuer Bezeichner `--kn-on-brand` (4,88:1 hell, 6,48:1 dunkel) als einzige Textfarbe auf Terracotta; er ersetzt zugleich das Literal in der Komponententabelle in Abschnitt 7 (SM-DES-003). Verweise auf URS v1.2 und TEC v2.2 nachgezogen. Das Mockup zeigt an dieser Stelle weiterhin den alten Stand — es ist visuelle Referenz, nicht Umsetzungsvorlage, und rangiert nach diesem Dokument. |
| 1.1 | 23.08.2026 | Abstimmung mit URS-STM-001 v1.1 und TEC-STM-001 v2.1. Verweise SM-DES-008, SM-DES-007, SM-EXP-009 und SM-SEC-009 richtiggestellt (zeigten auf bestehende, aber inhaltlich andere Anforderungen). Eigenes Register (D1 bis D5) aufgelöst. Betriebsmodus in Abschnitt 6.2 ergänzt, Rahmenprüfung an SM-MAC-002 gebunden, Prüfpunkt D-13 ergänzt, Dokumentenhierarchie festgelegt. Nachweis: ABG-STM-001. |
| 1.0 | 23.08.2026 | Erstfassung. Enthält gegenüber dem Mockup korrigierte Werte für `--kn-ok`, `--kn-warn` (Hellmodus) sowie die Sperre von `--kn-ink-3` für Text — fünf Token-Paare erfüllten WCAG AA nicht und wurden nachgerechnet ersetzt. |
