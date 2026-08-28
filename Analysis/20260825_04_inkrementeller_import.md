# Analyse — Inkrementeller Import (SM-IMP-003)

| Feld | Wert |
|---|---|
| Kennung | ANA-STM-20260825-04 |
| Datum | 2026-08-25 |
| Auslöser | Auftrag des Nutzers: „mach weiter mit SM-IMP-003" |
| Änderungsklasse | **C** — Quellcode |
| Arbeitspaket | AP-08 |
| Vorgänger | [`20260825_03_vorschau_zwischenspeicher.md`](20260825_03_vorschau_zwischenspeicher.md) |

## 1. Problembeschreibung

SM-IMP-003 (M): „Das System muss inkrementell importieren und **nur neue,
geänderte oder entfernte** Dateien verarbeiten."

Der bisherige Lauf verarbeitet **jede** gefundene Datei: Er liest sie
vollständig, wertet die Stichdaten aus und legt einen **neuen** Eintrag an. Drei
Folgen:

1. **Ein zweiter Lauf verdoppelt den Bestand.** Die Kennung entsteht aus Inhalt
   und Dateiname, der Pfad trägt einen Eindeutigkeitsindex — ein zweiter Lauf
   über denselben Ordner scheitert je Datei am Index und weist sie als
   „abgewiesen" aus. Der Bestand bleibt zwar sauber, aber der Lauf meldet
   sämtliche Dateien als Fehler.
2. **Unveränderte Dateien werden vollständig gelesen.** Bei 100.000 Einträgen
   liest ein Lauf die gesamte Bibliothek — genau das, was SM-IMP-003 ausschließt.
3. **Entfernte Dateien bleiben unbemerkt.** Ihr Eintrag verweist dauerhaft auf
   einen Ort, an dem nichts mehr liegt.

## 2. Betroffene Komponenten

| Komponente | Art der Berührung |
|---|---|
| `crates/kern-db` | **Schemaschritt 4** (additiv): `vermisst_seit`; Bestandsübersicht, Fortschreiben, Vermisstmarke |
| `crates/kern-fassade` | Änderungsermittlung; `erneuern`; Inhaltshash für neue und geänderte Dateien |
| `crates/kern-services` | Der Lauf arbeitet den Änderungssatz ab statt der Fundmenge; verwirft Vorschauen geänderter Dateien |
| `crates/ui/src/bruecke.rs` | Die Abschlussmeldung nennt die vier Mengen |

## 3. Betroffene Anforderungen

**Erfüllt wird:** SM-IMP-003 (M). Nachweis PF-IMP-03.

**Berührt:** SM-IMP-001 (rekursiver Import, unverändert), SM-IMP-005 (der
**Inhaltshash** entsteht hier — die Entscheidungsvorlage bleibt offen),
SM-PRV-003 (die Änderungserkennung ist nach AP-08 zugleich der Auslöser der
Vorschau-Verwerfung), SM-DAT-007 (der Schemaschritt ist additiv), SM-NFR-002
(der Lauf bleibt im Hintergrund), SM-NFR-005 (defekte Dateien halten nicht an).

## 4. Berührte offene Punkte

| Punkt | Einordnung | Begründung |
|---|---|---|
| OP-21 | neutral | Der Zwischenspeicher bekommt weiterhin keine Obergrenze |
| OP-08 | grundlagenschaffend | Der Lauf wird messbar; keine Zahl zugesagt |

## 5. Die Entwurfsfrage: was heißt „entfernte Dateien verarbeiten"?

SM-IMP-003 verlangt, entfernte Dateien zu **verarbeiten**, sagt aber nicht, wie.
Zwei Anforderungen sprechen dagegen, den Eintrag einfach zu löschen:

- **SM-DAT-003 (M):** „Vor dem Löschen von Einträgen oder Dateien muss das
  System bestätigen lassen." Ein Importlauf, der stillschweigend Einträge
  löscht, verletzt das.
- **SM-DAT-004 (S):** gelöschte Einträge gehören in einen Papierkorb.

Deshalb wird der Eintrag **als vermisst gekennzeichnet**, nicht gelöscht: Die
gepflegten Metadaten — Schlagworte, Notizen, Status — bleiben erhalten, und
taucht die Datei wieder auf, wird die Marke aufgehoben. Das verarbeitet die
Entfernung, ohne SM-DAT-003 oder SM-DAT-004 vorwegzunehmen. Der Löschweg selbst
ist ein eigenes Paket.

## 6. Vorgeschlagener Ansatz

**Erkennung an Größe und Änderungszeit** — dieselben zwei Größen wie beim
Vorschau-Zwischenspeicher, und aus demselben Grund: Ein Inhaltsvergleich läse
die ganze Bibliothek.

```text
Fundmenge (Dateisystem)  ×  Bestand (Datenhaltung), verglichen über den Pfad

  nur im Dateisystem              → neu          → lesen, aufnehmen, hashen
  in beiden, Größe/Zeit gleich    → unverändert  → nichts tun (der Regelfall)
  in beiden, Größe/Zeit verschieden → geändert   → lesen, fortschreiben, hashen,
                                                   Vorschau verwerfen
  nur im Bestand                  → vermisst     → kennzeichnen, nicht löschen
```

**Der Inhaltshash entsteht nur für neue und geänderte Dateien** (AP-08
wörtlich). Er kostet dabei **keine zusätzliche Ein-/Ausgabe**: Die Datei liegt
für den Parser ohnehin im Speicher. Ein unveränderter Eintrag wird nie erneut
gehasht — sonst läse ein inkrementeller Lauf über 100.000 Einträge entgegen
SM-IMP-003 den gesamten Dateiinhalt.

**Der Fortschritt zählt die Arbeit, nicht die Fundmenge.** Meldet ein Lauf über
100.000 unveränderte Dateien „0 von 0", ist das die richtige Aussage.

## 7. Prüfplan

- Ein zweiter Lauf über denselben Bestand nimmt **nichts** auf und weist
  **nichts** ab; alles gilt als unverändert.
- Eine neue Datei wird erkannt, ohne die übrigen erneut zu lesen.
- Eine geänderte Datei wird fortgeschrieben — **die Kennung bleibt**, damit
  Schlagworte und Notizen erhalten bleiben (SM-LIB-010).
- Eine entfernte Datei wird gekennzeichnet, **nicht gelöscht**; ihre Metadaten
  überstehen den Lauf.
- Taucht sie wieder auf, wird die Marke aufgehoben.
- Der Inhaltshash steht für neue und geänderte Einträge, und ein unveränderter
  Lauf liest **keine** Datei (nachgewiesen über die Zahl der Lesezugriffe).
- Der Zwischenspeicher einer geänderten Datei wird verworfen.

## 8. Abgrenzung

Nicht Gegenstand: die **Entscheidungsvorlage** bei Duplikaten (SM-IMP-005 —
nach AP-08 fällt sie in AP-12), die Ordnerüberwachung (SM-IMP-004,
zurückgestellt), Regeln zur Metadatenableitung (SM-IMP-007), Ziehen und Ablegen
(SM-IMP-008) sowie der Löschweg samt Papierkorb (SM-DAT-003, SM-DAT-004).

---

## 9. Abschluss (Phase 4)

### 9.1 Umsetzung

Der Lauf ermittelt zuerst den **Änderungssatz**, ohne eine Datei zu lesen, und
arbeitet dann nur diesen ab. Verglichen wird über den Pfad, entschieden über
Größe und Änderungszeit.

Schemaschritt 4 (`vermisste_eintraege`) ist additiv; die Prüfsummen der Schritte
1 bis 3 sind unverändert — der Prüffall zu SM-DAT-007 belegt das.

Der Fortschritt zählt jetzt die **Arbeit**, nicht die Fundmenge: Ein Lauf über
4.000 unveränderte Dateien meldet „0 von 0".

### 9.2 Drei Mängel, die dabei gefunden wurden

**1 — Die Anwendung hielt ihren Bestand im Arbeitsspeicher.** `Fassade::im_speicher()`
war noch der Stand aus dem ersten Gerüst. Damit war der Bestand nach jedem
Programmende verloren — SM-DAT-006 („ohne Datenverlust wieder anlaufen") war
verfehlt, und der inkrementelle Import wäre wirkungslos geblieben: Jeder Start
hätte die gesamte Bibliothek erneut eingelesen. **Die Prüffälle konnten das
nicht zeigen**, weil sie zwei Läufe im selben Prozess fahren; erst der Lauf
gegen die gebaute Anwendung hat es aufgedeckt. Die Datenhaltung liegt jetzt
dauerhaft neben dem Vorschau-Zwischenspeicher.

**2 — Beide Seiten des Vergleichs lagen in unterschiedlicher Form vor.** Der
Bestand hält den **aufgelösten** Pfad, weil `aufnehmen` ihn durch `kern-security`
schickt; ein Fundpfad aus dem Dateisystem ist es nicht (unter macOS
`/var/…` gegen `/private/var/…`). Ohne Auflösung **beider** Seiten galt jede
Datei als neu — der inkrementelle Import hätte nie gegriffen. Vier Prüffälle
haben es aufgedeckt.

**3 — Die Ablagekonvention lag doppelt.** Der Ort des Anwendungsverzeichnisses
war in `kern-render` implementiert; die Datenhaltung hätte ihn ein zweites Mal
gebraucht. Er liegt jetzt einmal in `kern-typen`, das alle kennen und das selbst
von nichts abhängt.

### 9.3 Nachweis

**Acht Prüffälle** in `kern-fassade` zur Erkennung, **vier** in `kern-services`
zum Lauf. Zwei tragen besonders:

- **„Ein unveränderter Lauf liest keine Datei"** — Beweisführung ohne Zählwerk:
  Nach dem ersten Lauf wird der **Inhalt** jeder Datei durch Unsinn ersetzt,
  Größe und Änderungszeit aber wiederhergestellt. Läse der zweite Lauf sie,
  scheiterte der Parser an jeder einzelnen. Er meldet null nicht lesbare Dateien.
- **„Geänderte Datei behält Kennung und Schlagworte"** — die Kennung übersteht
  die Fortschreibung (SM-LIB-010), und die gepflegten Schlagworte stehen danach
  noch. Ein Löschen-und-Neuanlegen verlöre genau das.

**An der laufenden Anwendung** (4.000 Dateien in sechzehn Unterordnern, zwei
getrennte Programmläufe):

| | Lauf 1 | Lauf 2 |
|---|---|---|
| Fortschritt | 4.000 / 4.000 | **0 / 0** |
| Meldung | „4000 neu aufgenommen" | „**Bestand ist aktuell — 4000 unverändert**" |
| Bestand nach dem Lauf | 4.000 | 4.000 |
| Takte (Wanduhr) | 120 / 120 | 120 / 120 |

Der zweite Lauf ist ein **eigener Programmstart**. Dass er 4.000 Einträge
vorfindet, belegt zugleich die dauerhafte Datenhaltung.

Gesamtstand: **154 bestandene Prüffälle**, 0 fehlgeschlagen. `cargo fmt` sauber,
`cargo clippy --all-targets` ohne Meldung, Projektregeln PASS.

### 9.4 Was offen bleibt

- **SM-IMP-005** ist nur zur Hälfte erfüllt: Der **Inhaltshash** entsteht jetzt
  für neue und geänderte Dateien und steht indiziert in der Datenhaltung. Die
  **Entscheidungsvorlage** bei Duplikaten fehlt — sie fällt nach AP-08 ohnehin
  in AP-12.
- **Vermisste Einträge sind in der Oberfläche nicht kenntlich.** Sie werden
  gekennzeichnet und gezählt, aber die Kachel zeigt es nicht. Das gehört zu
  AP-12 und berührt OP-15/OP-16.
- **Eine Änderung, die Größe *und* Änderungszeit unberührt lässt**, bleibt
  unbemerkt — dieselbe bewusste Grenze wie beim Vorschau-Zwischenspeicher.
- **Umbenennen wird als Entfernen plus Neuaufnahme gesehen**, weil der Vergleich
  über den Pfad läuft. Die gepflegten Metadaten hängen dann am vermissten
  Eintrag. Der Inhaltshash läge als Erkennungsmerkmal bereit; das zu nutzen ist
  ein eigener Vorgang (verwandt mit SM-IMP-005).
- **SM-IMP-004** (Ordnerüberwachung) bleibt zurückgestellt.
