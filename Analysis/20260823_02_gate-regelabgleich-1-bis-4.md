# Vier Regelabweichungen zwischen CLAUDE.md und dem Gate — Behebung

**Datum:** 23.08.2026
**Auslöser:** Regelabgleich vom 23.08.2026 — `CLAUDE.md` wurde vollständig gegen
`scripts/review-gate.sh`, `scripts/check-docs.sh`, `scripts/install-hooks.sh`,
`scripts/review-gate.test.sh` und `.claude/agents/*.md` geprüft. Der Abgleich ergab dreizehn
Abweichungen; diese Analyse behandelt die vier gravierendsten.
**Nicht Gegenstand:** die neun übrigen Abweichungen (siehe Abschnitt 8) und der bereits
geführte Rückstand in `Analysis/20260823_01_gate-befunde-rueckstand.md`.
**Änderungsklasse:** G (Begründung in Abschnitt 7).

---

## 1. Problembeschreibung

Vier Regeln aus `CLAUDE.md` sind in den Skripten nicht, unvollständig oder gegenteilig
umgesetzt. Alle vier betreffen den Kern des Commit-Freigabe-Prozesses: welcher Änderungssatz
geprüft wird, wie das Votum entsteht, wie es an den Commit gebunden wird und wie viel Prüfung
eine Änderung überhaupt zieht.

| Nr | Regelstelle | Soll | Ist | Wirkung heute |
|---|---|---|---|---|
| 1 | `CLAUDE.md:475-479` | `@{upstream}..HEAD`, hilfsweise `merge-base(origin/main, HEAD)`; leerer Bereich bei ungepushten Commits blockiert | `review-gate.sh:120-125` rechnet `merge-base(main, HEAD)`; leere Dateiliste beendet das Gate grün (`:687-690`) | Auf `main` prüft das push-Tier **nichts** |
| 2 | `CLAUDE.md:545-553` | Zufallstoken je Lauf, damit begrenzter Datenblock, eingerückte Diffzeilen, Votum `VERDICT: <token> APPROVE` | `build_prompt` (`:424-448`) und `parse_verdict` (`:450-460`) kennen kein Token; alle vier Rollentexte und die Selbsttestfälle A1–A7 schreiben das tokenlose Format fest | Die als primär bezeichnete strukturelle Schutzschicht fehlt |
| 3 | `CLAUDE.md:502-508` | Bindung über Commit-Trailer `Gate-Report: <pfad> <sha256>`, **nicht** durch nachträgliches Ändern des Protokolls | `install-hooks.sh:70-74` und `review-gate.sh:739-749` tragen den Commit nachträglich im Protokoll nach; einen Trailer erzeugt nichts | Die verbotene Variante ist die einzige gebaute; `CLAUDE.md:818` ist nicht erfüllbar |
| 4 | `CLAUDE.md:237-250`, `:808` | Änderungsklasse maschinell bestimmt, im Protokollkopf vermerkt; Klasse T zieht 0, 0b, 0c | Keine Einstufung im Skript, kein Feld im Protokollkopf (`:595-606`) | Jede redaktionelle Korrektur zahlt eine volle Reviewer-Runde |

### 1.1 Abweichung 1 — das push-Tier prüft auf `main` nichts

Nachgerechnet am Bestand: `git merge-base main HEAD` ergibt `2b60182`, also `HEAD` selbst. Der
Bereich `HEAD..HEAD` ist leer, die Dateiliste bleibt leer, und `main()` beendet den Lauf mit
„Keine prüfpflichtige Änderung" grün — ohne Secret-Scan über den Push-Inhalt, ohne Stufe 1,
ohne Stufe 2. `CLAUDE.md:479` benennt genau diese Formel als die frühere, die „dort
systematisch leer lief"; der Ersatz wurde beschrieben, aber nie gebaut.
`install-hooks.sh:65` schreibt die alte Formel zusätzlich im Hook-Kommentar fest.

Der grüne Ausstieg bei leerer Dateiliste ist im commit-Tier richtig (nichts gestaget) und im
push-Tier falsch (Commits vorhanden, Bereich falsch gerechnet). Er unterscheidet die Lagen
nicht.

### 1.2 Abweichung 2 — der Nonce existiert nirgends

`CLAUDE.md:545-553` beschreibt vier zusammenhängende Maßnahmen: Token je Lauf, damit
begrenzter Datenblock, zeilenweise Einrückung des Diffs, Votum nur mit gültigem Token. Keine
davon ist umgesetzt. Der Diff steht uneingerückt im Prompt, die Blockgrenzen sind feste
Zeichenketten, und `parse_verdict` nimmt jede letzte Zeile an, die nach dem Entfernen von
Leerzeichen und Auszeichnungen `VERDICT:APPROVE` ergibt.

Damit trägt die Abwehr allein der Mustervorfilter — den `CLAUDE.md:553` ausdrücklich als
*zweite* Schicht führt und den der eigene Rückstand bereits als umgehbar ausweist (N2-m6). Die
Rollentexte (`curie.md:81`, `newton.md:82`, `tesla.md:77`, `turing.md:83`) und die
Selbsttestfälle A1–A7 zementieren das tokenlose Format an zwei weiteren Stellen.

### 1.3 Abweichung 3 — die Nachweisbindung tut das Verbotene

`CLAUDE.md:502-508` begründet ausführlich, warum ein Nachtrag im committeten Protokoll eine
Schleife ohne Ende erzeugt, und legt den Commit-Trailer als Weg fest. Gebaut ist der Nachtrag:
`write_report` schreibt die Platzhalterzeile „(wird vom post-commit-Hook nachgetragen)", der
`post-commit`-Hook ersetzt sie. Der Trailer entsteht nirgends, weshalb die Checklistenzeile
`CLAUDE.md:818` heute nur von Hand erfüllbar ist. Der `post-commit`-Hook selbst kommt in
`CLAUDE.md` nicht vor; genannt sind dort nur `pre-commit` und `pre-push` (`:393`).

Solange das Protokoll ungetrackt ist, ändert der Nachtrag nichts Committetes — die Regel greift
erst beim ersten committeten grünen Protokoll. Der Mechanismus ist trotzdem der falsche, und er
ist bereits installiert.

### 1.4 Abweichung 4 — Änderungsklassen sind unimplementiert

`CLAUDE.md:237-250` nennt die Einstufung „maschinell prüfbar", `:808` verlangt sie im
Protokollkopf. Weder bestimmt das Skript eine Klasse noch führt der Kopf ein solches Feld. Die
in `CLAUDE.md:244` beschriebene Klasse T — Gates 0, 0b, 0c, keine Analysepflicht, kein
Reviewer — existiert deshalb nur auf dem Papier. Eine Tippfehlerkorrektur kostet heute vier
Reviewer-Läufe; das ist der Anreiz, aus dem heraus Ausstiege benutzt statt Regeln befolgt
werden.

---

## 2. Betroffene Komponenten

| Datei | Was sich ändert |
|---|---|
| `scripts/review-gate.sh` | Bereichsberechnung, Nonce-Erzeugung, Prompt-Aufbau, Votum-Auswertung, Votum-Signatur, Protokollkopf, Einstufung, Entfall von `__record-commit` |
| `scripts/install-hooks.sh` | `post-commit` entfällt aus der Installation (bleibt in `--status` und `--uninstall`), `prepare-commit-msg` kommt hinzu, Hook-Kommentar zum Diffbereich |
| `scripts/review-gate.test.sh` | Umstellung der Fälle A1–A7, neue Fälle je Änderung (Abschnitt 6) |
| `.claude/agents/newton.md`, `turing.md`, `tesla.md`, `curie.md` | Votumsabsatz auf das Token-Format |
| `CLAUDE.md` | Abschnitt 2 und 13: der dritte Hook wird benannt |

Nicht betroffen: die drei Fachdokumente, das Mockup, `scripts/check-docs.sh`.

---

## 3. Betroffene Anforderungen und Prüfpunkte

Der Gegenstand ist Prüfwerkzeug, nicht Erzeugnis. Es entsteht **keine** neue Anforderung, und
nichts davon gehört ins Lastenheft.

| Kennung | Bezug |
|---|---|
| SM-NFR-012 | Verlangt automatisierte Prüfung. Abweichung 1 hebt sie im push-Tier faktisch auf, Abweichung 4 verteuert sie bis zur Umgehung. |
| SM-PLT-001 | macOS bleibt Zielplattform: die Umsetzung bleibt bash-3.2-tauglich (keine assoziativen Arrays, keine `${var^^}`-Formen), Rückfallschutz H5. |
| SM-SEC-006, SM-KIA-010 | Der Secret-Scan bleibt unverändert und läuft in **jeder** Klasse, auch in T (`CLAUDE.md:250`). |

Regeln aus `CLAUDE.md` ohne Anforderungskennung — S3 (Anwendbarkeit), die Nachweisbindung und
die Klassentabelle — sind nach `CLAUDE.md:670` zulässige Befundgrundlage; sie brauchen keine
`SM-`-Kennung.

Prüfpunkte der Abnahme (`AK-…`, `D-…`) sind nicht berührt.

---

## 4. Berührte offene Punkte

**Keiner der dreizehn offenen Punkte wird berührt.** Die vier Änderungen betreffen den
Prüfweg, nicht das Erzeugnis: Sie sind unter jeder Antwort auf OP-01 bis OP-13 unverändert
gültig — auch unter der Antwort, dass der vorhandene Rust-Kern nicht wiederverwendet wird
(OP-13), und unabhängig vom Ausgang des Prototypvergleichs zwischen Weg A und Weg B.

Die serverseitige Durchsetzung (Branch-Schutz auf `main`, CI-Lauf des push-Tiers) ist in
`CLAUDE.md:402` ausdrücklich als offen vermerkt, steht aber **nicht** im Register des
Lastenhefts. Abweichung 1 verbessert die client-seitige Lage und nimmt zu dieser Frage nichts
vorweg.

---

## 5. Root Cause

Die Skripte wurden gebaut, bevor die betroffenen Regeln in `CLAUDE.md` standen; die Regeln
wurden anschließend präzisiert — teils aus Befunden der drei Prüfrunden vom 23.08.2026 —,
ohne die Skripte nachzuziehen. Sichtbar wird das an Abweichung 1: `CLAUDE.md:479` beschreibt
die alte Formel bereits in der Vergangenheitsform.

Zwei Verstärker haben verhindert, dass der Rückstand auffiel:

1. **Der Selbsttest prüft das Gate gegen sich selbst, nicht gegen `CLAUDE.md`.** Die Fälle
   A1–A7 belegen, dass das Votum so gelesen wird, wie das Skript es schreibt. Ob das dem
   Regelwerk entspricht, prüft kein Fall. Derselbe blinde Fleck erklärt, warum 65 grüne
   Prüffälle neben vier offenen Regelabweichungen stehen können.
2. **Das push-Tier wurde nie ausgeführt.** Ein Lauf, der immer sofort grün endet, erzeugt kein
   Protokoll und keinen Widerspruch.

---

## 6. Vorgeschlagener Ansatz

Vier Schritte, **vier getrennte Commits**, jeder für sich durch das Gate. Ein Zug über alle
vier ergäbe erneut einen Änderungssatz, der mehr Befunde erzeugt, als eine Runde auflösen kann
(Erfahrung aus Analyse 01, Abschnitt 5). Reihenfolge nach Risiko, aufsteigend — nicht nach
Cache-Nutzen: jede der vier Änderungen fasst `review-gate.sh` an und verändert damit die
Gate-Signatur, weshalb ohnehin kein Ergebnis wiederverwendet wird.

### Schritt 1 — Diffbereich des push-Tiers

1. Neue Funktion `resolve_push_base()` mit dieser Rangfolge: `@{upstream}` (über
   `git rev-parse --abbrev-ref --symbolic-full-name @{upstream}`) · sonst
   `merge-base(origin/$GATE_MAIN_BRANCH, HEAD)`, falls die entfernte Referenz existiert ·
   sonst `merge-base($GATE_MAIN_BRANCH, HEAD)` · sonst der Wurzel-Commit.
2. `collect_scope` benutzt diese Basis; `SCOPE_DESC` nennt den tatsächlich gewählten Weg
   namentlich, damit im Protokoll steht, wogegen geprüft wurde.
3. Der Frühausstieg bei leerer Dateiliste wird tier-abhängig:
   - commit-Tier, leerer Index → grün wie bisher.
   - push-Tier, `git rev-list --count <basis>..HEAD` = 0 → nichts zu pushen, grün, benannt.
   - push-Tier, Zähler > 0 **und** leere Dateiliste → **FAIL** mit dem Hinweis, dass die
     Bereichsberechnung fehlerhaft ist (`CLAUDE.md:476`).
4. `install-hooks.sh:65` im Hook-Kommentar richtigstellen.

**Rückfallschutz** (neue Selbsttestfälle): Push-Tier mit Upstream prüft den ungepushten Commit
und startet die Reviewer · ohne Upstream greift der `origin/`-Weg · leerer Bereich bei
vorhandenen ungepushten Commits blockiert · „nichts zu pushen" endet grün ohne Reviewer.

### Schritt 2 — Änderungsklassen

1. `classify_change()` setzt `CHANGE_CLASS` und `CHANGE_CLASS_REASON` aus Dateiliste und Diff:
   - **G**, sobald ein Pfad auf `scripts/`, `.claude/`, `CLAUDE.md`, `LICENSE`, eine
     Wurzelkonfiguration oder einen unbekannten Pfad passt.
   - **C**, sobald ein Pfad auf `src/`, `*.rs`, `*.qml`, `*.py`, `Cargo.toml`, `Cargo.lock`
     oder `pyproject.toml` passt.
   - **T** nur, wenn **alle** Bedingungen aus `CLAUDE.md:244` zutreffen: keine `SM-`, `AK-`,
     `OP-` oder `D-`-Kennung in einer hinzugefügten **oder** entfernten Zeile, kein Pfad aus
     der G-Liste, höchstens 20 geänderte Zeilen, ausschließlich Dokumentpfade.
   - sonst **D**. Bei Uneindeutigkeit die höhere Klasse.
2. Protokollkopf erhält `| Änderungsklasse | <T/D/C/G> — <Begründung> |`.
3. Klasse T überspringt Stufe 1; das Protokoll führt sie als `ENTFÄLLT — Klasse T`, nie als
   PASS (S3).
4. **Absicherung gegen das bekannte Loch:** Ein Klasse-T-Protokoll ist ein grünes Protokoll
   ohne Reviewer. Es wird deshalb **nicht** in `emitted-reports` eingetragen und bleibt damit
   prüfpflichtig, wenn es committet wird. Ohne diese Klausel entstünde genau der Selbstentzug
   aus dem Prüfumfang, den N2-B1 beschreibt.

**Rückfallschutz:** je ein Fall für T, D, C und G · eine hinzugefügte Kennung hebt T auf D ·
21 geänderte Zeilen heben T auf D · eine Zeile in `scripts/` ergibt G · Klasse T startet keinen
Reviewer · das Klasse-T-Protokoll steht nicht in `emitted-reports`.

### Schritt 3 — Nachweisbindung über den Trailer

1. `__record-commit` entfällt; `install-hooks.sh` installiert `post-commit` nicht mehr, führt
   ihn aber weiter in `--status` und `--uninstall`, damit bestehende Installationen den Hook
   loswerden.
2. Neuer Hook `prepare-commit-msg` — er läuft nach `pre-commit` und darf die Nachricht noch
   ändern. Er hängt `Gate-Report: <pfad> <sha256>` an, wenn der laufende Gate-Lauf eine frische
   Merkdatei hinterlassen hat und der Trailer fehlt.
3. `write_report` schreibt Pfad und SHA-256 nach `$CACHE_DIR/last-report`, statt die
   Platzhalterzeile zu erzeugen; die Zeile „Commit" entfällt aus dem Kopf.
4. `sha256_of()` benutzt `sha256sum`, hilfsweise `shasum -a 256`; fehlt beides, ist das
   **FAIL** mit Installationshinweis — der Gegenstand existiert, das Werkzeug fehlt (S3).
5. `CLAUDE.md` Abschnitt 2 und 13 benennen den dritten Hook.

**Bewusste Folge, die im Protokoll steht:** Das Protokoll bleibt zum Zeitpunkt des Commits
ungetrackt und wird mit dem nächsten Änderungssatz committet, wo es über die Herkunftsprüfung
vom Reviewer-Diff ausgenommen ist. Der Trailer bindet Pfad und Inhalt; das Protokoll selbst
wird nie mehr angefasst. Nicht gelöst und außerhalb dieses Schrittes: `git commit --amend` und
Rebase führen kein Gate erneut aus.

**Rückfallschutz:** Ein grüner Lauf im Wegwerf-Repository mit installierten Hooks erzeugt einen
Commit, dessen Trailer Pfad und Hash des Protokolls trägt · ein blockierter Lauf erzeugt keinen
Trailer · ein zweiter Commit ohne neuen Gate-Lauf erbt den Trailer nicht.

### Schritt 4 — Nonce

1. `nonce_new()` erzeugt je Lauf 16 Hexstellen aus `/dev/urandom` (`od`-basiert, damit
   bash-3.2- und macOS-tauglich).
2. `build_prompt` setzt den Diff in einen mit dem Token begrenzten Block und rückt **jede**
   Diffzeile um zwei Leerzeichen ein. Der Prompt weist die Einrückung ausdrücklich als Zutat
   des Gates aus, damit Reviewer sie nicht als Änderung lesen.
3. Der Prompt verlangt als letzte Zeile wörtlich `VERDICT: <token> APPROVE` bzw.
   `VERDICT: <token> CHANGES_REQUESTED`.
4. `parse_verdict` erhält das Token als zweites Argument und akzeptiert nur die exakte Form.
   Jede andere letzte Zeile — auch eine mit falschem Token — ergibt wie bisher **ABGEBROCHEN**
   und zählt fail-closed.
5. **Wechselwirkung, die mitzulösen ist:** Die Votum-Signatur wird heute über den fertigen
   Prompt gebildet (`review-gate.sh:465`). Mit einem Token je Lauf wäre sie bei jedem Lauf
   verschieden und die Votum-Wiederverwendung (`CLAUDE.md:495`) stillschweigend tot. Die
   Signatur wird deshalb über Gate-Signatur, Rolle, Diff-Inhalt und `REVIEW_GATE_MODEL`
   gebildet — ohne das Token. Dass `REVIEW_GATE_MODEL` dabei aufgenommen wird, behebt
   nebenbei Abweichung 7 des Abgleichs; die Zeile wird ohnehin neu geschrieben, und sie ohne
   das Modell neu zu schreiben hieße, einen bekannten Mangel wissentlich zu erhalten.
6. Die vier Rollentexte stellen ihren Votumsabsatz auf das Token-Format um.

**Rückfallschutz:** A1–A7 auf das Token umgestellt · Votum ohne Token gilt als ABGEBROCHEN ·
Votum mit fremdem Token gilt als ABGEBROCHEN · ein Diff, der eine vollständige Votumszeile mit
fremdem Token enthält, ändert das Ergebnis nicht · der Prompt enthält keine Diffzeile am
Zeilenanfang · ein zweiter Lauf mit gleichem Diff verwendet das APPROVE weiter wieder ·
ein geändertes `REVIEW_GATE_MODEL` verwirft es.

### Was bewusst nicht mitgeändert wird

Der Mustervorfilter bleibt, wie er ist — auch die von `CLAUDE.md:553` verlangte Abstufung
zwischen Quellcode- und Dokumentpfaden (Abweichung 11). Schritt 4 schafft die Voraussetzung
dafür, entscheidet sie aber nicht; sie gehört in den zweiten Änderungssatz, weil sie den
Vorfilter in Dokumentpfaden **abschwächt** und deshalb erst tragfähig ist, wenn das Token
nachweislich läuft.

---

## 7. Prüfung des Änderungssatzes und Änderungsklasse

**Änderungsklasse G** für alle vier Commits: Sie ändern `scripts/`, `.claude/` und `CLAUDE.md`.
Damit ziehen sie alle zutreffenden Gates, und Stufe 0b ist zwingend. Die Rust-Gates stehen als
ENTFÄLLT im Protokoll, solange kein `Cargo.toml` existiert (S3).

Diese Analyse allein ist Klasse T: Sie vergibt keine Kennung und ändert keine Zeile in
`scripts/`, `.claude/` oder `CLAUDE.md` (`CLAUDE.md:280`). Da Schritt 2 noch nicht umgesetzt
ist, fährt ihr Commit trotzdem die volle Runde — ein Beleg für den Nutzen von Schritt 2, kein
Grund, ihn vorzuziehen.

**Kein Notfall-Ausstieg vorgesehen.** Das Gate startet und läuft; Störungsfall 1 (`S2`,
`CLAUDE.md:449`) ist damit **nicht** einschlägig. Jeder der vier Commits geht regulär durch
Stufe 0 bis 1. Einschlägig wird der Störungsfall erst, wenn eine dieser Änderungen das Gate so
beschädigt, dass es nicht mehr startet — dann gilt `Gate-Override: gate-reparatur <text>` mit
den dort genannten Auflagen: nur Gate-Dateien im Commit, Selbsttest danach grün, Abdeckung
durch den nächsten regulären Lauf.

**Erwarteter Aufwand:** vier Runden zu je vier Reviewern. Der erste push-Tier-Lauf nach
Schritt 1 ist der erste echte Stufe-2-Lauf dieses Repositorys und prüft alles, was noch nicht
auf `origin/main` steht.

---

## 8. Nicht Gegenstand dieser Analyse

Die neun übrigen Abweichungen des Abgleichs bleiben offen und gehören in einen zweiten
Änderungssatz mit eigener Analyse: ENTFÄLLT statt FAIL bei fehlendem Markdown-Linter sowie
fehlende QML-, Python- und Fuzzing-Anwendbarkeit (S3) · fehlende `.markdownlint-cli2.jsonc` ·
Protokollkopf ohne Modell und Reviewer-Besetzung · fehlender Stückelungslauf · fehlende
Sicherheitsprüfung der Stufe 2 · Abstufung des Injection-Vorfilters nach Pfad · Prüfbereiche
in `check-docs.sh` (`Analysis/` zu viel, `scripts/` zu wenig) · fehlender Gleichschritt-Check
zwischen `CLAUDE.md` und `.claude/agents/` · Sperre, Dateinamensbereinigung,
Stellschrauben-Liste.

Abweichung 7 (Modell im Cache-Schlüssel) wird in Schritt 4 miterledigt; die Begründung steht
dort.

---

## 9. Abschluss

*Nach der Umsetzung auszufüllen: was tatsächlich geändert wurde, welche Rückfallschutzfälle
hinzugekommen sind, welche Befunde die Prüfrunden ergeben haben und was davon als begründetes
Restrisiko geführt wird.*
