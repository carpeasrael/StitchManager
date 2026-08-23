# Befunde der ersten Gate-Läufe — Behebung und begründete Restrisiken

**Datum:** 23.08.2026
**Quelle:** Gate-Protokolle `Reviews/20260823-214726_main.md` (Runde 1, 0/4),
`Reviews/20260823-220439_main.md` (Runde 2, 0/4) und `Reviews/20260823-222046_main.md`
(Runde 3, 0/4) und `Reviews/20260823-225149_main.md` (Runde 4, 0/4), commit-Tier. Alle vier
Protokolle sind rot und bleiben deshalb ungetrackt (CLAUDE.md: rote Protokolle werden nicht
committet); ihre Befunde sind hier vollständig festgehalten.
**Entscheidung:** Auftraggeber, 23.08.2026 — Blocker beheben, die verbleibenden Befunde als
ausdrücklich begründete Restrisiken führen und den Erstimport damit freigeben.

---

## 1. Ausgangslage

Der erste Commit umfasst den gesamten Bestand in einem Änderungssatz: vier Dokumente, das
Commit-Freigabe-Gate, dessen Selbsttest und die vier Reviewer-Rollen. Vier vollständige
Prüfrunden ergaben je 0/4 — bei Befundzahlen von 34, 21, 21 und 26; ab Runde 3 mit
ausdrücklichem Vermerk der Reviewer, welche Punkte sie **nicht erneut** erheben.

Das ist kein Zufall, sondern eine Eigenschaft der Lage. Das Gate ist für **inkrementelle**
Änderungen gebaut; hier prüft es den Änderungssatz, der es selbst einführt, und tut das über
330 KB Diff hinweg. Ein Prüfumfang dieser Größe erzeugt bei gründlichen Reviewern verlässlich
Befunde, und ein Teil davon betrifft Festlegungen, die noch offene Punkte des Lastenhefts
berühren und deshalb gar nicht im Code entschieden werden dürfen.

**Betroffene Komponenten:** `scripts/review-gate.sh`, `scripts/check-docs.sh`,
`Design/StitchManager_Design_Beschreibung.md`, `Design/stitchmanager-mockup.html`,
`Requirements/StitchManager_Lastenheft.md`.

---

## 2. Behoben

| Nr | Befund | Kennung | Behebung | Rückfallschutz |
|---|---|---|---|---|
| N1-1 | Ein Commit, der ausschließlich löscht, lief ungeprüft durch: `--diff-filter=ACMR` ließ Löschungen aus der Dateiliste, der Leer-Abbruch beendete das Gate grün. | — | Dateiliste auf `ACMRD`; der Secret-Pfadscan bleibt auf `ACMR`, weil eine entfernte Datei kein Geheimnis einbringt. | H1, H1b |
| N1-2 | Der Cache der Stufe 0c war auf **Dateinamen** geschlüsselt. Eine zweite Änderung an denselben Dateien lieferte ein PASS aus dem Cache, ohne geprüft zu werden. | — | Signatur über Name **und** Inhalt jeder betroffenen Datei. | H2, H3 |
| C1-B1 | Weiße Schrift auf `--kn-brand` erreicht 3,41:1 (hell) und 2,57:1 (dunkel); Abschnitt 3.3 Regel 3 erlaubte sie ausdrücklich. | SM-NFR-007 | Regel 3 umgeschrieben, neuer Bezeichner `--kn-on-brand` (4,88:1 / 6,48:1) als einzige Textfarbe auf Terracotta. DES v1.2, Verweise in URS v1.2 und TEC v2.2 nachgezogen. | — |
| N2-B1 | Null Reviewer ergaben 0/0 und damit ein **grünes** Protokoll, das sich über die Herkunftsliste künftig selbst dem Prüfumfang entzogen hätte. | — | Weniger als zwei Stimmen blockieren. Neuer `GATE_DRY_RUN=1` fährt die Stufen 0 bis 0c und schreibt bewusst **kein** Protokoll. | H4, H4b, H4c, H7, H7b, H7c |
| N2-M6 | Die Skripte nutzten `declare -A` und setzten damit bash ≥ 4 voraus. macOS liefert bash 3.2 aus — auf einer Zielplattform wäre das Gate beim Laden abgebrochen. | SM-PLT-001 | Zustand liegt in Dateien statt in assoziativen Arrays. | H5 |
| N2-m3 | Eine blockierende Stufe fehlte in der Protokolltabelle, weil diese aus einer festen Namensliste gespeist wurde. | — | Die Tabelle folgt der Ausführungsreihenfolge. | H6 |
| C2-M10 | Das Gate löschte ungetrackte Protokolle unter `Reviews/` ohne Meldung. | — | Jede Entfernung wird auf der Konsole gemeldet und im Protokoll vermerkt. | — |
| C2-M9 | Stufe 0c meldete PASS, obwohl eine Teilprüfung mangels Werkzeug gar nicht lief. | SM-NFR-012 | `check-docs.sh` zählt entfallene Prüfungen und benennt sie in der Ergebniszeile. | — |
| N2-7 | Der Prüfumfang hing über `-- .` am Arbeitsverzeichnis statt am Wurzelverzeichnis. | — | `:(top)`. | — |

| T3-1 | **Blocker.** Der Secret-Pfadscan war durch git-Quotierung umgehbar: `git diff --name-only` escapt Nicht-ASCII-Pfade (`"a/\303\234b.md"`), wodurch `(^|/)\.secure/` ins Leere griff. Dieselbe Ursache machte die Größengrenze für Binärdateien und die Cache-Signatur unwirksam und ließ `check-docs.sh` Dateien mit Umlaut lautlos überspringen. | SM-SEC-006, SM-KIA-010 | Alle Pfadlisten laufen über `core.quotePath=false` und `-z`. | H9, H9b, H10 |
| T3-2 | Ein gitleaks-Treffer wurde durch den Rückfallaufruf überschrieben und damit verworfen. | — | Erfolg, Treffer und Werkzeugfehler sind getrennt; der Rückfall greift nur bei unbekanntem Unterbefehl, jeder andere Fehler blockiert. | — |
| T3-3 | Die Größe von Binärdateien kam aus dem Arbeitsbaum; eine gestagete, im Arbeitsbaum fehlende Datei lieferte 0. | — | Größe aus dem geprüften Objekt (`git cat-file -s`). | — |
| T3-5 | Der Regex-Rückfall kannte keine präfixbasierten Zugangsschlüssel. | — | Ergänzt um `sk-ant-`, `sk_live_`/`sk_test_`, `glpat-`, `npm_`, `hf_`, `dop_v1_`. | — |
| T3-D05 | Das D-05-Gate nahm zwei **ganze Dateien** aus, statt zweier Blöcke. | D-05, SM-DES-003 | Die Design-Beschreibung wird blockgenau geprüft: Farbwerte nur in Abschnitt 3. Das Mockup bleibt vorerst ausgenommen — siehe Abschnitt 4. | — |
| T3-Zahl | `CLAUDE.md` nannte 49 Prüffälle, der Selbsttest hatte 65. | — | Zahl und Laufzeit richtiggestellt. | — |
| T3-SEC010 | `CLAUDE.md` führte SM-SEC-010 als verbindlich, im Lastenheft ist es eine **Soll**-Anforderung. | SM-SEC-010 | Als Arbeitsregel gekennzeichnet, nicht als Muss. | — |

Nebenbefund aus der eigenen Arbeit: Der Prompt ging ursprünglich als **ein**
Kommandozeilenargument an die Reviewer. Linux begrenzt ein einzelnes Argument auf 128 KB —
jeder Diff darüber wäre stillschweigend an allen vier gescheitert. Jetzt über stdin,
Rückfallschutz F9/F9b.

---

## 3. Akzeptierte Restrisiken — Gate und Skripte

**Gemeinsame Begründung.** Das Gate bewacht derzeit ein Repository **ohne Quellcode**. Die
verbliebenen Schwächen mindern die Gründlichkeit der Prüfung, sie gefährden kein
ausgeliefertes Erzeugnis; keine davon lässt einen unbemerkten Commit entstehen, weil alle
Stufen fail-closed bleiben. **Frist: vor dem ersten Commit mit Quellcode** — ab dann prüft
das Gate Erzeugnisse statt Dokumente, und die Genauigkeit des Prüfumfangs wird tragend.

| Nr | Befund | Schwere | Was schlimmstenfalls passiert | Frist |
|---|---|---|---|---|
| N2-M2 | Stufe 0c prüft den **Arbeitsbaum**, freigegeben wird der **Index**. | major | Eine ungestagete Änderung wird mitgeprüft oder eine gestagete nicht — das Ergebnis gilt einem anderen Stand als dem committeten. | vor dem ersten Quellcode-Commit |
| N2-M1 | Die Votum-Wiederverwendung bindet den Arbeitsbaum nicht ein. | major | Ein APPROVE wird wiederverwendet, obwohl sich Dateien geändert haben, die nicht im Diff stehen. | vor dem ersten Quellcode-Commit |
| N2-M3 | Das commit-Tier fährt die volle Testsuite und weist sie als „änderungsbezogen" aus. | major | Falsche Erwartung an die Laufzeit; keine Prüflücke. | vor dem ersten Quellcode-Commit |
| N2-M5 | Das D-05-Gate prüft nur Hex-Literale, nicht Abstände und Schriftgrößen. | major | SM-DES-003 wird teilweise ungeprüft zugesichert. Solange kein Komponentencode existiert, geht nichts durch. | mit dem ersten Oberflächencode |
| T-1 | Die Ausnahme für `Reviews/*.md` bindet den Dateiinhalt nicht, nur die Herkunft des Pfades. | major | Eine registrierte Protokolldatei könnte nachträglich beliebigen Inhalt tragen und bliebe ungeprüft. Voraussetzung ist Schreibzugriff auf das Repository. | vor dem ersten Quellcode-Commit |
| T-2 | Secret-Scan und Injection-Vorfilter sehen nur hinzugefügte Zeilen; übertragen wird der ganze Diff. | major | Ein Geheimnis in einer **entfernten** Zeile geht an die Reviewer, ohne den Scan auszulösen. Es stand dann allerdings bereits zuvor im Repository. | vor dem ersten Quellcode-Commit |
| N2-m6 | Der Injection-Vorfilter ist durch Zeichenkettentrennung umgehbar — belegt am eigenen Repository, wo genau das getan wird, um die Testfixtures zu schreiben. | minor | Der Vorfilter ist eine Hürde, keine Garantie. Die eigentliche Absicherung ist die Kennzeichnung des Diffs als Datenmaterial und das Lesen des Votums nur vom Antwortende. | mit der nächsten Überarbeitung des Vorfilters |
| T-3 | `gitleaks` läuft nur im commit-Tier. | minor | Im push-Tier greift allein der Regex-Rückfall. | vor dem ersten Quellcode-Commit |
| T-5 | Das Protokoll übernimmt Reviewer-Antworten ungeprüft, samt möglicher absoluter Pfade. | minor | SM-SEC-010 gilt für exportierbare Protokolle; Gate-Protokolle sind lokal. | vor der ersten Veröffentlichung |
| T-6 | Einfachanführung um `$ROOT` in `bash -c`. | minor | Bricht bei einem Apostroph im Pfad. | vor dem ersten Quellcode-Commit |
| N2-m1 | `xargs` ohne `-r`, zusätzlich ein Volldurchlauf je toter Kennung. | minor | Laufzeit, kein Ergebnisfehler. Bei 224 Kennungen und vier Dokumenten nicht messbar. | vor dem ersten Quellcode-Commit |
| N2-m2 | `sort` und `comm` mit unterschiedlicher Sortierordnung. | minor | Anforderungskennungen sind durchgehend ASCII; ein Fehlvergleich ist erst möglich, wenn nicht-ASCII-Kennungen eingeführt werden. Das sieht das Kennungsschema nicht vor. | vor dem ersten Quellcode-Commit |
| N2-m4 | Prozessaufwand je Datei in den heißen Schleifen. | minor | Laufzeit bei sehr großen Änderungssätzen; heute unter einer Sekunde. | vor dem ersten Quellcode-Commit |
| C2-m8 | Zwei Läufe in derselben Sekunde überschreiben das Protokoll des ersten. | minor | Die Repository-Sperre lässt nur ein Gate gleichzeitig zu; der Fall setzt zwei Läufe in derselben Sekunde nach Freigabe der Sperre voraus. | vor dem ersten Quellcode-Commit |
| T3-Mockup | Das D-05-Gate nimmt das Mockup weiterhin dateiweit aus. | major | Ein Themenfarbliteral im Mockup bliebe unbemerkt. Das Mockup enthält neben Themenfarben auch **Inhaltsfarben** — Garnfelder und Motiv-Grafiken sind Daten, keine Gestaltung. Eine blockgenaue Prüfung müsste beides trennen, und das setzt die Entscheidung über den Status des Mockups voraus. Bis dahin meldet `check-docs.sh` die Prüfung ausdrücklich als **entfallen**, nie als bestanden. | mit der Entscheidung über das Mockup |
| C2-m9 | `check-docs.sh` hat keinen eigenen Prüffall. | minor | Ein Fehler im Prüfer bliebe unentdeckt. Er wird bei jedem Lauf ausgeübt, und seine Befunde sind während dieser drei Runden mehrfach eingetreten — er ist also nicht unbelegt. | vor dem ersten Quellcode-Commit |
| T3-N1 | Die Cache-Signatur der Stufe 0c bildet den **Änderungssatz** ab, `check-docs.sh` prüft aber den **gesamten** Arbeitsbaum. | major | Eine Änderung an einer Datei außerhalb des Änderungssatzes kann ein zwischengespeichertes PASS überleben. Sie käme jedoch spätestens mit ihrem eigenen Commit erneut zur Prüfung. | vor dem ersten Quellcode-Commit |
| C2-m7 | Die Rollentexte engen das Votum stärker ein als CLAUDE.md. | minor | Reine Minor-Befunde führen nach den Rollentexten nie zu CHANGES_REQUESTED; CLAUDE.md formuliert das offener. | mit der nächsten Überarbeitung der Rollen |

---

## 4. Akzeptierte Restrisiken — Dokumente und Mockup

**Gemeinsame Begründung.** Diese Befunde betreffen Festlegungen, die an **offenen Punkten**
des Lastenhefts hängen: OP-07 (Festbreitenschrift), OP-09 (Abgleich der Farbwerte gegen den
Markenstandard), OP-10 (Listenansicht), OP-12 (Übersichtskarte). CLAUDE.md verbietet
ausdrücklich, einen offenen Punkt stillschweigend zu entscheiden. Die Behebung nimmt
Entscheidungen vorweg, die dem Auftraggeber zustehen. **Frist: vor Umsetzungsbeginn der
Oberfläche** — dieselbe Schranke, an der die offenen Punkte selbst stehen.

**Wo diese Begründung nicht trägt, steht sie beim Eintrag.** Nicht jeder Befund hängt an einem
offenen Punkt: T1-M3, T1-M5, T1-M6/C-M1, C-M2 bis C-M6, C-m2 und C-m5 sind schlichte
Widersprüche oder Lücken **innerhalb** der Design-Beschreibung. Für sie gilt die zweite,
schwächere, aber tragfähige Begründung: Es existiert keine Oberfläche und kein
Komponentencode. Kein Nutzer kann von ihnen erreicht werden, und ihre Behebung ändert
ausschließlich Text, der ohnehin vor Umsetzungsbeginn gegen OP-09 abgeglichen wird. Sie
zweimal anzufassen — jetzt und nach dem Abgleich — kostet zweimal Prüfung und erzeugt zwei
Versionsstände desselben Abschnitts.

Hinzu kommt: Kein Befund dieses Abschnitts betrifft ausgelieferte Software. Es gibt keine.

| Nr | Befund | Kennung | Schwere | Frist |
|---|---|---|---|---|
| T1-M1 | Für **Abstände** existiert kein einziger Bezeichner; die Variablenquelle deckt nur Farben ab, SM-DES-003 verlangt Farb-, Schrift- und Abstandswerte. | SM-DES-003 | major | vor Umsetzungsbeginn der Oberfläche |
| T1-M3 | `--kn-ink-3` trägt zwei Bedienelemente, obwohl es für lesbaren Text gesperrt ist. | SM-NFR-007 | major | vor Umsetzungsbeginn der Oberfläche |
| T1-M5 | Waagerechte Bereichstrenner sind durchgezogene Striche statt Nähte. | SM-DES-001 | major | vor Umsetzungsbeginn der Oberfläche |
| T1-M6 / C-M1 | Statusleiste 26 px und Mindesttrefferfläche 32 × 32 px schließen einander aus; der Abbruch langer Vorgänge ist damit nicht bedienbar. | SM-NFR-008 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M2 | Die drei Zustände (leer, ladend, fehlerhaft) sind nur für die mittlere Spalte ausformuliert. | SM-NFR-006 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M3 | Kein Dialog-Bauteil, obwohl sechs Muss-Anforderungen Dialoge verlangen. | SM-NFR-008 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M4 | Bei reduzierter Bewegung ist „Vorschau lädt" von „leer" nicht unterscheidbar. | SM-NFR-013 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M5 | Prüfpunkt D-06 verengt die Fokus-Vorgabe: die verbotene Variante besteht die Abnahme. | SM-NFR-008 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M6 | Der Fokusring erreicht auf Werkzeugleiste, Navigation und Statusleiste nur 2,93:1. | SM-NFR-007 | major | vor Umsetzungsbeginn der Oberfläche |
| C-M7 | Mockup: weiße Schrift auf Terracotta, dazu die Behauptung, alle Textfarben hielten 4,5:1. | SM-NFR-007 | major | siehe Entscheidung unten |
| C-M8 | Mockup: kein Bedienelement der Hauptbereiche ist tastaturerreichbar. | SM-NFR-008 | major | siehe Entscheidung unten |
| T1-m1 | Drei Bezeichner im Mockup, die Abschnitt 3 nicht kennt. | SM-DES-003 | minor | siehe Entscheidung unten |
| T1-m2 | Direkt gesetzter Stil, wo eine Komponente definiert ist. | SM-DES-003 | minor | siehe Entscheidung unten |
| T1-m3 | Zustandstabelle der Hauptschaltfläche ist nicht umsetzbar benannt („8 % abgedunkelt"). | — | minor | vor Umsetzungsbeginn der Oberfläche |
| C-m1 | Entfernen-Kreuz der Filter-Chips in der gesperrten Stufe `--kn-ink-3`. | SM-NFR-007 | minor | vor Umsetzungsbeginn der Oberfläche |
| C-m2 | Chip hat weder Zeigerkontakt- noch Fokus- noch Aktivzustand. | — | minor | vor Umsetzungsbeginn der Oberfläche |
| C-m4 | Vier Muss-Anforderungen der Bedienbarkeit haben keinen Prüfpunkt. | — | minor | vor der Prüfplanung |
| C-m5 | Oberflächentexte weichen von der Begriffstabelle ab. | SM-SET-006 | minor | vor Umsetzungsbeginn der Oberfläche |
| C-m10 | Reihenfolge in Änderungshistorie und Abnahmeliste. | — | minor | mit der nächsten Fortschreibung der Dokumente |
| T3-T1 | Abschnitt 6 setzt Maße, die das in Abschnitt 5 festgelegte 4-px-Raster verletzen (13 px, 14 px, 158 px) — ohne die dort geforderte Begründung. | SM-DES-003 | major | vor Umsetzungsbeginn der Oberfläche |
| T3-C2 | SM-NFR-007 fordert WCAG AA nur für **Text**. Für Nicht-Text-Kontrast — Fokusring, Elementgrenzen, Zustandsmarken — fehlt eine Anforderung, weshalb der Fokusring mit 2,93:1 formal nichts verletzt. | SM-NFR-007 (Lücke) | major | vor der Prüfplanung; die Aufnahme einer Anforderung entscheidet der Auftraggeber |
| T3-C6 | Abschnitt 6.3 schreibt für Format- und KI-Marke 9 px vor, die kleinste Stufe in Abschnitt 4 ist `t-xs` = 11 px. | SM-DES-003 | minor | vor Umsetzungsbeginn der Oberfläche |
| T3-T5 | Mockup: `border-radius: 2px` außerhalb der Radienskala. | DES Abs. 5 | minor | mit der Entscheidung über das Mockup |
| N-m5 | „Warmer Index" ist als Messbedingung nicht definiert. | SM-SRC-007 | minor | vor der Prüfplanung (gehört zu OP-08) |
| N-m7 / C-m3 | Prüfpunkt D-10 misst die Layoutstabilität bei 10.000 statt 100.000 Kacheln. | SM-LIB-009 | minor | vor der Prüfplanung |

**Zu entscheiden — das Mockup.** Vier Befunde (C-M7, C-M8, T1-m1, T1-m2) richten sich gegen
`stitchmanager-mockup.html`. Es rangiert als visuelle Referenz nach der Design-Beschreibung
und ist ausdrücklich **keine** Umsetzungsvorlage. Zu klären ist deshalb zuerst, ob es
nachgezogen oder als Momentaufnahme eingefroren wird. Solange das offen ist, wäre jede
Änderung daran Arbeit auf Verdacht. Diese Frage gehört als offener Punkt ins Lastenheft,
sobald sie ansteht.

---

## 5. Vorgeschlagener Ansatz für die Abarbeitung

1. **Vor dem ersten Quellcode:** die sieben Gate-Befunde aus Abschnitt 3 mit Frist
   „vor dem ersten Quellcode-Commit" in einem Zug. Sie betreffen alle dieselbe Frage —
   was der Prüfumfang tatsächlich abdeckt — und lassen sich gemeinsam prüfen.
2. **Vor Umsetzungsbeginn der Oberfläche:** zuerst OP-07, OP-09, OP-10 und OP-12 klären,
   dann die Dokumentbefunde aus Abschnitt 4. Der Maßwiderspruch (T1-M6 / C-M1) und die
   fehlenden Abstandsbezeichner (T1-M1) ändern beide Abschnitt 5 der Design-Beschreibung und
   gehören zusammen.
3. **Das Mockup** erst nach der Entscheidung über seinen Status anfassen.
4. **Vor der Prüfplanung:** die Prüfpunktlücken (C-m4, N-m5, N-m7).

Jeder Schritt ist eine eigene Analyse nach Phase 1 und läuft erneut durch das Gate. Die
Erfahrung dieser Runden spricht dafür, sie klein zu halten: 330 KB in einem Zug
erzeugen mehr Befunde, als eine Runde auflösen kann.

---

## 6. Runde 4, Notfall-Ausstieg und Nachreview-Auflage

**Protokoll:** `Reviews/20260823-225149_main.md`, commit-Tier, Tree
`6a7d1db39f87e774971e86f55f8e9dc8a48e8b2e`, Diff 330.584 Byte. Stufen 0, 0b (65/65 Prüffälle)
und 0c grün, Rust-Gates ENTFÄLLT. **Stufe 1: 0/4** — alle vier CHANGES_REQUESTED, kein
Abbruch. Das Protokoll ist rot, bleibt ungetrackt und wird nach `REPORT_RETENTION_DAYS` (14)
entfernt; die Befunde stehen deshalb unten.

**Entscheidung:** Auftraggeber, 23.08.2026 — Erstimport mit dem ersten Notfall-Ausstieg
(`REVIEW_GATE_DISABLE=1`) committen und pushen, Trailer `Gate-Override:` gesetzt.

**Begründung.** Vier Runden über denselben Änderungssatz ergaben 34, 21, 21 und 26 Befunde
ohne Konvergenz. Die Ursache steht in Abschnitt 1: Der Änderungssatz führt das Gate ein, das
ihn prüft, und tut das über 330 KB. Ein Teil der Befunde richtet sich gegen `CLAUDE.md`
selbst — nach S1 ein Befund gegen die Datei, kein Mangel am Änderungssatz. Kein Produkt-
Quellcode ist betroffen; es gibt keinen.

**Auflage.** `CLAUDE.md` bindet den ersten Ausstieg an ein Nachreview vor dem Merge. Der
Erstimport hat keinen Merge-Zielpunkt — er geht direkt auf `main`. Die Auflage gilt deshalb in
dieser Form: **Das Nachreview findet vor dem ersten Commit mit Quellcode statt** und umfasst
alle unten mit „offen" geführten Befunde. Bis dahin ist jeder weitere Commit regulär durch das
Gate zu führen; der Ausstieg gilt einmalig für diesen Änderungssatz.

### 6.1 Befunde der Runde 4

Der Fahrplan für die vier gravierendsten steht in
`Analysis/20260823_02_gate-regelabgleich-1-bis-4.md`; die dort mitbehandelten sind hier so
vermerkt.

| Nr | Befund | Schwere | Status |
|---|---|---|---|
| N4-1 | Das push-Tier rechnet `merge-base(main, HEAD)`; auf `main` ist der Bereich leer und das Gate endet grün, ohne zu prüfen. | blocker | offen — Analyse 02, Schritt 1 |
| N4-2 | Die Cache-Signatur der Stufe 0c bildet den Änderungssatz ab, `check-docs.sh` prüft den ganzen Arbeitsbaum. | major | offen — bereits als T3-N1 geführt |
| N4-3 | Stufe 0c prüft den Arbeitsbaum, freigegeben wird der Index. | major | offen — bereits als N2-M2 geführt |
| N4-4 | Die Votum-Wiederverwendung bindet weder Arbeitsbaum noch `REVIEW_GATE_MODEL` ein. | major | offen — Arbeitsbaum als N2-M1 geführt, Modell in Analyse 02, Schritt 4 |
| N4-5 | Protokollkopf ohne Modell, Besetzung und Änderungsklasse; Nachweisbindung als verbotener Nachtrag. | major | offen — Analyse 02, Schritte 2 und 3 |
| N4-6 | Die als primär bezeichnete Injection-Schutzschicht (Nonce) fehlt vollständig. | major | offen — Analyse 02, Schritt 4 |
| N4-7 bis N4-12 | Prozessaufwand in `run_gate`, `check-docs.sh` und `scan_secrets_paths`; totes Array in `collect_scope`; D-10 misst bei 10.000 statt 100.000; Laufzeitzusagen ohne hinterlegte Messung. | minor | offen — Laufzeit, keine Prüflücke |
| T4-1 | DES-STM-001 Abschnitt 6.4 gegen Abschnitt 7 (Widerspruch in der Zustandsfestlegung). | major | offen — vor Umsetzungsbeginn der Oberfläche |
| T4-2 bis T4-4 | Nonce, Nachweisbindung und Diffbereich — `CLAUDE.md` gegen Gate und Rollentexte. | major | offen — Analyse 02, Schritte 1, 3 und 4 |
| T4-5 | `.markdownlint-cli2.jsonc` fehlt, obwohl `CLAUDE.md` Abschnitt 11 sie als vorhanden führt. | major | offen — zweiter Änderungssatz |
| T4-6 | Prüfbereiche in `check-docs.sh` weichen von Abschnitt 11 ab: `Analysis/` zu viel, `scripts/` zu wenig. | major | offen — zweiter Änderungssatz |
| T4-7 | `write_report` gegen die Kopfvorgaben aus Abschnitt 13. | major | offen — Analyse 02, Schritt 2 |
| T4-8 bis T4-12 | Raster- und Zustandsangaben in DES; unvollständige Stellschraubenliste; drei weitere beschriebene, nicht gebaute Mechanismen; Redaktionsfehler in dieser Analyse. | minor | Redaktionsfehler in diesem Commit behoben, Rest offen |
| S4-1 | Secret-Scan und Injection-Vorfilter prüfen registrierte Protokolldateien nicht. | major | offen — verwandt mit T-1, vor dem ersten Quellcode-Commit |
| S4-2 | Fehlende Nonce, Votumsauswertung zu locker. | major | offen — Analyse 02, Schritt 4 |
| S4-3 bis S4-5 | Protokolle übernehmen Reviewer-Antworten unmaskiert; Dateinamensbereinigung schwächer als festgelegt; der Rauchtest prüft Erreichbarkeit, nicht die Werkzeugsperre. | minor | offen |
| C4-M1 | Statusleiste 26 px gegen Mindesttrefferfläche 32 × 32 px — der Abbruch langer Vorgänge ist nicht bedienbar. | major | offen — bereits als T1-M6/C-M1 geführt |
| C4-M2 | Die drei Zustände sind nur für die mittlere Spalte ausformuliert. | major | offen — bereits als C-M2 geführt |
| C4-M3 | Prüfpunkt D-06 lässt die verbotene Fokusvariante bestehen. | major | offen — bereits als C-M5 geführt |
| C4-M4 | Das Mockup behauptet Barrierefreiheit, die es nachgerechnet verfehlt (3,40:1 und 2,57:1). | major | offen — hängt an der Entscheidung über den Status des Mockups |
| C4-M5 | Angenommene major-Befunde stehen nur in `Analysis/` — laut Abschnitt 3 ein Nachweis, keine Quelle. Die Fristen sind damit unbewacht. | major | offen — siehe 6.2 |
| C4-m1 bis C4-m14 | Gesperrte Textstufe, Trefferflächen und Tastaturferne im Mockup; Begriffstabelle; Schriftgröße unter der Skala; Fokusring 2,89:1; Nachweislücken; D-10; Rollentexte enger als `CLAUDE.md`; D-05-Gate ohne Kurzschreibweisen; Reihenfolgen; Anglizismen in `CLAUDE.md`; `check-docs.sh` ohne eigenen Prüffall. | minor | offen |

### 6.2 Curies Befund C4-M5 — anzunehmen oder zu widerlegen

Der Befund trifft einen wunden Punkt dieser Analyse: Sie führt Befunde mit der Frist „vor
Umsetzungsbeginn der Oberfläche", steht aber in `Analysis/` und ist damit nach `CLAUDE.md`
Abschnitt 3 ein Nachweis ohne Bindungswirkung. Wer die Design-Beschreibung aufschlägt, findet
an den betroffenen Stellen keinen Hinweis.

Das ist nicht durch diesen Commit zu heilen, weil die Aufnahme weiterer offener Punkte ins
Register des Lastenhefts eine inhaltliche Dokumentänderung wäre — mit Versionserhöhung, Historieneintrag und
eigener Analyse nach Phase 1. Der Punkt wird deshalb hier als **erste Aufgabe des
Nachreviews** vermerkt: entweder die drei textnahen Widersprüche (C4-M1 bis C4-M3) direkt
beheben oder sie als offene Punkte ins Register aufnehmen und in DES-STM-001 darauf verweisen.
