#!/usr/bin/env bash
# Deterministische Dokumentprüfungen — Stufe 0c des Commit-Freigabe-Gates.
# Prüft die Regeln aus CLAUDE.md, Abschnitt „Regeln für die Arbeit an den Dokumenten"
# und die Vorwegnahme von Prüfpunkt D-05.
# Exit 0 = alles grün. Jeder Befund blockiert.

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2

URS="Requirements/StitchManager_Lastenheft.md"
DES="Design/StitchManager_Design_Beschreibung.md"
TEC="TechStack/StitchManager_TechStack.md"
IMP="Implementation/StitchManager_Implementierungsplan.md"
MOCKUP="Design/stitchmanager-mockup.html"
# Eine Frage, eine Antwort: `PLAN` war ein zweiter Name für denselben Pfad.
PLAN="$IMP"

# Dateien, in denen Farbliterale zulässig sind (die eine Variablenquelle je Medium).
COLOR_ALLOW="$DES $MOCKUP"

TAB="$(printf '\t')"
# Oberflächenpfad aus `.projektregeln.conf` — dieselbe Quelle, die
# check-projektregeln.sh liest. Fehlt sie, bleibt es baumweit bei der
# Farbklasse: weniger prüfen ist hier richtig, denn die übrigen Klassen
# hätten ohne Oberflächenbezug keinen Gegenstand.
OBERFLAECHE="$(sed -n 's/^[[:space:]]*oberflaeche[[:space:]]*=[[:space:]]*//p' .projektregeln.conf 2>/dev/null | sed 's/[[:space:]]*$//' | head -1)"
TMPFEHLER="$(mktemp)"
trap 'rm -f "$TMPFEHLER"' EXIT
FINDINGS=0
SKIPPED=0
SKIPNAMES=""
finding() { printf '  ✗ %s\n' "$*"; FINDINGS=$((FINDINGS + 1)); }
pass()    { printf '  ✓ %s\n' "$*"; }
# Eine entfallene Prüfung ist keine bestandene. Sie wird gezählt und benannt,
# damit ein PASS der Stufe 0c nicht mehr aussagt, als tatsächlich geprüft wurde.
skip()    { printf '  – %s\n' "$*"; SKIPPED=$((SKIPPED + 1)); SKIPNAMES="$SKIPNAMES${SKIPNAMES:+, }$1"; }

# Arbeitsbaum, nicht nur der Index — und ohne die maschinell erzeugten Gate-Protokolle:
# sie betten den Rundendiff wörtlich ein und trügen dessen Farbwerte und Kennungen hierher.
# Die Frage „welche Dateien gehören zum Baum?" wird **einmal** beantwortet, in
# scripts/lib/dateien.sh — samt der Vorkehrung gegen escapte Nicht-ASCII-Pfade.
tracked_and_new() { kn_dateien "$@"; }
# Die Regeln zu SM-DES-003/D-05 stehen **einmal** im Baum. Fehlt die Datei,
# ist das FAIL, nicht ENTFÄLLT: Ihr Gegenstand — Markdown und Quelltext —
# liegt immer vor (S3). Sie muss **vor** den Dateilisten eingebunden sein: Der
# Prüfbereich leitet sich aus ihrer Erweiterungsliste ab.
if [ -r "$ROOT/scripts/lib/gestaltung.sh" ]; then
  # shellcheck source=lib/gestaltung.sh
  . "$ROOT/scripts/lib/gestaltung.sh"
else
  echo "  ✗ scripts/lib/gestaltung.sh fehlt — die Designregeln sind nicht prüfbar (SM-DES-003)" >&2
  exit 1
fi
# Dieselbe Erwägung für die Dateiliste: eine Antwort im Baum, nicht drei.
if [ -r "$ROOT/scripts/lib/dateien.sh" ]; then
  # shellcheck source=lib/dateien.sh
  . "$ROOT/scripts/lib/dateien.sh"
else
  echo "  ✗ scripts/lib/dateien.sh fehlt — der Prüfbereich ist nicht bildbar (S3)" >&2
  exit 1
fi

# **Einmal je Lauf — und zwar auf Skriptebene.**
#
# Die Zwischenspeicherung stand zuvor *in* den Funktionen. Die wurden aber
# ausnahmslos in Pipelines und Kommandosubstitutionen aufgerufen, also in
# Subshells: Die Zuweisung ging jedes Mal verloren, und es liefen weiterhin
# sieben vollständige `git ls-files` je Lauf plus zwei je toter
# Anforderungsreferenz — genau das N+1-Muster, das der Kommentar als behoben
# auswies.
#
# Die Listen entstehen deshalb hier, vor der ersten Nutzung. Die Funktionen
# geben sie nur noch aus.
# **Der S3-Vertrag der Bibliothek wird ausgewertet, nicht verworfen.**
# `kn_dateien` meldet Rückgabewert 1, wenn kein Git-Arbeitsbaum vorliegt (etwa
# in einem entpackten Archiv). Ein `|| true` machte daraus eine leere Liste —
# und die Prüfung meldete grün über eine Menge, die sie nie gesehen hat.
if ! kn_arbeitsbaum; then
  echo "  ✗ Kein Git-Arbeitsbaum — der Prüfbereich ist nicht bildbar." >&2
  echo "    Ein nicht durchgeführter Test ist kein bestandener Test (S3)." >&2
  echo "    Behebung: aus einem Klon heraus aufrufen, nicht aus einem entpackten Archiv." >&2
  exit 1
fi

# **Der Rückgabewert wird vor dem Filtern ausgewertet.** In einer Pipeline mit
# `|| true` ginge er verloren, und ein git-Fehler — beschädigter Index,
# unlesbare `.gitignore` — ergäbe eine leere Liste bei grünem Ergebnis: eine
# Zusage über eine nie gesehene Menge (S3).
_ROH_DOCS="$(tracked_and_new '*.md')" || {
  echo "  ✗ Die Dateiliste ist nicht bildbar (git-Fehler)." >&2
  echo "    Ein nicht durchgeführter Test ist kein bestandener Test (S3)." >&2
  exit 1
}
_DOCS_CACHE="$(printf '%s\n' "$_ROH_DOCS" | grep -vE '^Reviews/' || true)"

# Der Prüfbereich der Quelltexte kommt aus `KN_PRUEF_EXT` (die
# gestaltungstragenden Oberflächenarten beider Wege plus Rust) und wird um die
# Arten ergänzt, die keine Gestaltung tragen, aber Prüfbereich sind. Die Liste
# steht damit **einmal** im Baum.
_SRC_MUSTER=()
for _e in $(printf '%s' "$KN_PRUEF_EXT" | tr '|' ' ') sh html; do
  _SRC_MUSTER+=("*.$_e")
done
_ROH_SRC="$(tracked_and_new "${_SRC_MUSTER[@]}")" || {
  echo "  ✗ Die Dateiliste der Quelltexte ist nicht bildbar (git-Fehler)." >&2
  echo "    Ein nicht durchgeführter Test ist kein bestandener Test (S3)." >&2
  exit 1
}
_SOURCES_CACHE="$(printf '%s\n' "$_ROH_SRC" | grep -vE '^Reviews/' || true)"
_ALLE_CACHE="$(printf '%s\n%s\n' "$_DOCS_CACHE" "$_SOURCES_CACHE" | grep . | sort -u)"

docs()         { printf '%s\n' "$_DOCS_CACHE"; }
sources()      { printf '%s\n' "$_SOURCES_CACHE"; }
alle_dateien() { printf '%s\n' "$_ALLE_CACHE"; }

echo "Dokumentprüfungen"

# ── 1 · Anforderungskennungen: jede Referenz ist definiert ───────────────────
if [ -f "$URS" ]; then
  defined="$(grep -oE '^\| ~?~?SM-[A-Z]{3}-[0-9]{3}~?~? \|' "$URS" | grep -oE 'SM-[A-Z]{3}-[0-9]{3}' | sort -u)"
  referenced="$( { docs; sources; } | xargs grep -hoE 'SM-[A-Z]{3}-[0-9]{3}' 2>/dev/null | sort -u)"
  dangling="$(comm -13 <(printf '%s\n' "$defined") <(printf '%s\n' "$referenced"))"
  if [ -n "$dangling" ]; then
    while IFS= read -r id; do
      [ -n "$id" ] || continue
      finding "tote Anforderungsreferenz: $id ist nirgends im Lastenheft definiert ($( { docs; sources; } | xargs grep -lE "$id" 2>/dev/null | tr '\n' ' '))"
    done <<< "$dangling"
  else
    pass "Anforderungsreferenzen: $(printf '%s\n' "$referenced" | grep -c .) Kennungen, alle definiert"
  fi

  # ── 2 · Keine Kennung doppelt vergeben ────────────────────────────────────
  dupes="$(grep -oE '^\| ~?~?SM-[A-Z]{3}-[0-9]{3}~?~? \|' "$URS" | grep -oE 'SM-[A-Z]{3}-[0-9]{3}' | sort | uniq -d)"
  if [ -n "$dupes" ]; then
    while IFS= read -r id; do
      [ -n "$id" ] && finding "Kennung doppelt vergeben: $id — Kennungen werden nie wiederverwendet"
    done <<< "$dupes"
  else
    pass "Keine doppelt vergebene Kennung"
  fi

  # ── 3 · Gestrichene Kennungen tragen einen Grund ──────────────────────────
  bad_struck=0
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    printf '%s' "$line" | grep -qi 'gestrichen\|entfällt\|verschoben' || {
      finding "gestrichene Kennung ohne Grund: $(printf '%s' "$line" | grep -oE 'SM-[A-Z]{3}-[0-9]{3}')"
      bad_struck=1
    }
  done < <(grep -E '^\| ~~SM-[A-Z]{3}-[0-9]{3}~~ \|' "$URS")
  [ "$bad_struck" = 0 ] && pass "Gestrichene Kennungen tragen ihren Grund"

  # ── 4 · Offene Punkte: genau ein Register ─────────────────────────────────
  op_defined="$(grep -oE '^\| \*?\*?OP-[0-9]{2}\*?\*? \|' "$URS" | grep -oE 'OP-[0-9]{2}' | sort -u)"
  op_referenced="$(docs | xargs grep -hoE 'OP-[0-9]{2}' 2>/dev/null | sort -u)"
  op_dangling="$(comm -13 <(printf '%s\n' "$op_defined") <(printf '%s\n' "$op_referenced"))"
  if [ -n "$op_dangling" ]; then
    while IFS= read -r op; do
      [ -n "$op" ] && finding "offener Punkt $op wird referenziert, ist aber nicht im Lastenheft geführt — das Register liegt allein dort"
    done <<< "$op_dangling"
  else
    pass "Offene Punkte: $(printf '%s\n' "$op_defined" | grep -c .) geführt, keine Fremdnummer"
  fi
else
  skip "Kennungsprüfungen (Lastenheft nicht gefunden)"
fi

# ── 5 · Versionsabgleich in den Dokumentköpfen ──────────────────────────────
doc_version() {
  case "$1" in
    URS-STM-001) [ -f "$URS" ] && sed -n '1,20p' "$URS" | grep -oE '^\| \*\*Version\*\* \| [0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+' ;;
    DES-STM-001) [ -f "$DES" ] && sed -n '1,20p' "$DES" | grep -oE '^\*\*Version:\*\* [0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+' ;;
    TEC-STM-001) [ -f "$TEC" ] && sed -n '1,20p' "$TEC" | grep -oE '^\*\*Version:\*\* [0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+' ;;
    IMP-STM-001) [ -f "$IMP" ] && sed -n '1,20p' "$IMP" | grep -oE '^\*\*Version:\*\* [0-9]+\.[0-9]+' | grep -oE '[0-9]+\.[0-9]+' ;;
  esac
}
# **README.md steht hier bewusst nicht.** Sie stand es einmal, mit der
# Begründung, sie nenne die Dokumentversionen in einer Tabelle. Beides traf
# nicht zu: Die Datei enthält keine einzige Dokumentkennung, und das Muster
# unten schließt mit `[^|·]` gerade den senkrechten Strich aus — die Tabelle
# wäre also selbst dann nicht erfasst worden. Die Bedingung täuschte Deckung
# vor, die es nicht gab (fail-open).
#
# Was die README tatsächlich deckt, prüft `check-plan.sh` (Bedingung 11:
# Zählwerte der README gegen Lastenheft und Plan). Der Versionsabgleich für
# die README bleibt damit **offen** — geführt als T-8.
ver_bad=0
for f in "$URS" "$DES" "$TEC" "$IMP"; do
  [ -f "$f" ] || continue
  bereich="1,60p"
  while IFS= read -r ref; do
    [ -n "$ref" ] || continue
    kenn="$(printf '%s' "$ref" | grep -oE '(URS|DES|TEC|IMP)-STM-001')"
    want="$(printf '%s' "$ref" | grep -oE 'v[0-9]+\.[0-9]+$' | tr -d 'v')"
    is="$(doc_version "$kenn")"
    [ -n "$is" ] || continue
    if [ "$want" != "$is" ]; then
      finding "$f verweist im Kopf auf $kenn v$want — tatsächlich ist $kenn v$is"
      ver_bad=1
    fi
  done < <(sed -n "$bereich" "$f" | grep -oE '(URS|DES|TEC|IMP)-STM-001[^|·]{0,40}v[0-9]+\.[0-9]+')
done
[ "$ver_bad" = 0 ] && pass "Versionsverweise in den Dokumentköpfen stimmen"

# ── 6 · Gestaltungsliterale nur in der Gestaltungsquelle (D-05) ────────────
# Blockgenau, nicht dateiweit: In der Design-Beschreibung sind Farbwerte allein in
# Abschnitt 3 zulässig — auch die Änderungshistorie nennt Bezeichner, nicht Werte.
#
# Muster, Erweiterungsliste, Deklarations- und Ausnahmeform stehen in
# `scripts/lib/gestaltung.sh`; dieses Skript und `check-projektregeln.sh`
# lesen dieselbe Quelle. Zuvor lagen sie doppelt im Baum und waren bereits
# auseinandergelaufen.
# Einzigkeit der Gestaltungsquelle — baumweit, nicht nur innerhalb von
# `oberflaeche`: Eine zweite deklarierende Datei außerhalb dieses Pfades bekäme
# sonst einen dateiweiten Freibrief, den kein Gate bemerkt.
deklarierende=""
while IFS= read -r f; do
  [ -n "$f" ] || continue
  kn_ist_gestaltungsquelle "$f" && deklarierende="$deklarierende$f
"
done < <(alle_dateien)
deklarierende="$(printf '%s' "$deklarierende" | grep . || true)"
anzahl_dekl="$(printf '%s' "$deklarierende" | grep -c . || true)"
# **Genau eine** — dieselbe Bedingung wie in check-projektregeln.sh. Zuvor
# meldete dieses Skript nur „mehr als eine"; für eine Anforderung, die genau
# eine verlangt, galten damit zwei Maßstäbe.
if [ "${anzahl_dekl:-0}" -gt 1 ]; then
  finding "$anzahl_dekl Dateien erklären sich zur Gestaltungsquelle; SM-DES-003 verlangt genau eine: $(printf '%s' "$deklarierende" | tr '\n' ' ')"
fi
color_bad=0
if [ -f "$DES" ]; then
  # **Ohne Intervallausdruck.** `{3}` ist eine POSIX-Erweiterung, die nicht
  # jede awk-Fassung kennt; wo sie fehlt, trifft das Muster nichts und die
  # blockgenaue Farbprüfung meldet still 0 Treffer — fail-open.
  hex_awk='#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]'
  n="$(awk -v hex="$hex_awk" '/^## / { ins = ($0 ~ /^## 3\./) } ins { next } $0 ~ hex { c++ } END { print c+0 }' "$DES")"
  if [ "$n" -gt 0 ]; then
    finding "$DES enthält $n Farbliteral(e) außerhalb von Abschnitt 3 (SM-DES-003, D-05)"
    color_bad=1
  fi
fi

# **Zwei Gruppen, zwei Prozesse — nicht einer je Datei.**
#
# Der Umfang folgt der Anforderung: D-05 und DES-STM-001 Abschnitt 3/5 gelten
# dem **Komponentencode**. Die Farbklasse steht baumweit; Maß-, Schrift- und
# Farbwortklassen greifen nur unterhalb des in `.projektregeln.conf` benannten
# Oberflächenpfads. Ohne diese Grenze beanstandete die Prüfung auch
# `let width: u32 = 1024;` im Kern — eine Regel ohne begehbaren Weg (S1).
#
# Zuvor lief je Datei ein eigener `perl`-Start, für Markdown sogar der
# vollständige Mehrklassendurchlauf, dessen Ergebnis anschließend verworfen
# wurde. Jetzt: ein Aufruf je Gruppe, und die Einschränkung auf die Farbklasse
# geschieht **im** Durchlauf.
_ui_dateien=()
_rest_dateien=()
while IFS= read -r f; do
  [ -n "$f" ] || continue
  case " $COLOR_ALLOW " in *" $f "*) continue ;; esac
  kn_ist_gestaltungsquelle "$f" && continue
  if [ -n "$OBERFLAECHE" ]; then
    case "$f" in
      "$OBERFLAECHE"/*) _ui_dateien+=("$f"); continue ;;
    esac
  fi
  _rest_dateien+=("$f")
done < <(alle_dateien)

# **Keine Pipe.** `finding` erhöht `FINDINGS`; in einer Pipe liefe die
# Schleife in einer Subshell, der Zähler ginge verloren und das Skript meldete
# „grün", nachdem es zehn Befunde gedruckt hat. Genau dieselbe Subshell-Falle,
# die die Zwischenspeicherung der Dateilisten wirkungslos gemacht hat.

if [ "${#_ui_dateien[@]}" -gt 0 ]; then
  if ! aus="$(kn_literale "${_ui_dateien[@]}")"; then
    finding "Literalprüfung der Oberfläche nicht durchführbar (perl-Rückgabewert) — nicht geprüft heißt nicht bestanden (S3, SM-DES-003)"
    color_bad=1
  elif [ -n "$aus" ]; then
    while IFS="$TAB" read -r klasse datei nr _rest; do
      [ -n "$klasse" ] || continue
      finding "$datei:$nr — $(kn_klassentext "$klasse") außerhalb der Gestaltungsquelle. Wert als Bezeichner in die Gestaltungsquelle aufnehmen oder in derselben Zeile 'D-05-Ausnahme: <Grund>' setzen (SM-DES-003, D-05)"
      color_bad=1
    done <<< "$aus"
  fi
fi
if [ "${#_rest_dateien[@]}" -gt 0 ]; then
  if ! aus="$(KN_NUR_KLASSE=farbe kn_literale "${_rest_dateien[@]}")"; then
    finding "Farbliteralprüfung nicht durchführbar (perl-Rückgabewert) — nicht geprüft heißt nicht bestanden (S3, SM-DES-003)"
    color_bad=1
  elif [ -n "$aus" ]; then
    while IFS="$TAB" read -r klasse datei nr _rest; do
      [ -n "$klasse" ] || continue
      finding "$datei:$nr — $(kn_klassentext "$klasse") außerhalb der Gestaltungsquelle. Wert als Bezeichner in die Gestaltungsquelle aufnehmen oder in derselben Zeile 'D-05-Ausnahme: <Grund>' setzen (SM-DES-003, D-05)"
      color_bad=1
    done <<< "$aus"
  fi
fi
[ "$color_bad" = 0 ] && pass "Keine Gestaltungsliterale außerhalb der Gestaltungsquelle"
# Das Mockup bleibt vorerst dateiweit ausgenommen: es trägt neben Themenfarben auch
# Inhaltsfarben (Garnfelder, Motiv-Grafiken), die Daten sind und keine Gestaltung.
# Die Trennung setzt die Entscheidung über den Status des Mockups voraus — als
# Restrisiko geführt in Analysis/20260823_01_gate-befunde-rueckstand.md.
skip "Farbliterale im Mockup (Inhalts- und Themenfarben noch nicht getrennt)"

# ── 7 · Markdown-Stil ───────────────────────────────────────────────────────
# Gegenstand sind die Markdown-Dateien im Baum; die gibt es immer. Deshalb kennt
# diese Prüfung kein ENTFÄLLT (CLAUDE.md, "Anwendbarkeit", S3): Fehlt das Werkzeug,
# ist das FAIL mit Installationshinweis, nicht skip — ein nicht durchgeführter Test
# ist kein bestandener Test. Die Fassung ist in package.json gebunden; der Vorrang
# der lokalen Installation verhindert, dass eine abweichende globale Fassung
# stillschweigend andere Regeln anlegt (MD060-Vorfall, siehe
# Analysis/20260824_02_markdown-stilpruefung.md).
MDL=""
# Ein **versioniertes** `node_modules/` ist in diesem Repository nie legitim:
# `.gitignore` schließt es aus, `git add -f` umgeht das aber. Ein fremder Zweig
# könnte darüber eine ausführbare Datei mitbringen, die der `pre-commit`-Hook
# anschließend von selbst startet — Fremdcode mit den Rechten der Nutzerin.
# Deshalb wird der lokale Pfad nur verwendet, wenn er nicht unter Versionierung
# steht (SM-OSS-011, sachnah zu SM-SEC-011).
# **Der ganze Baum, nicht ein Pfad.** Geprüft wurde zuvor allein die
# Startdatei `node_modules/.bin/markdownlint-cli2`. Das ausgeführte Programm
# liegt aber nicht dort — die Startdatei ist ein Verweis auf
# `node_modules/markdownlint-cli2/…`. Ein zugelieferter Zweig, der **diesen**
# Zielbestand versioniert mitbringt, überschreibt beim Auschecken die ignorierte
# Datei, und der nächste Hook startet den ausgetauschten Programmtext mit den
# Rechten der Nutzerin. `.npmrc` (`ignore-scripts=true`) greift dort nicht:
# Es findet gar kein `npm`-Lauf statt.
if [ -n "$(git ls-files node_modules 2>/dev/null | head -1)" ]; then
  finding "node_modules/ steht unter Versionierung — Fremdcode im Baum, den der pre-commit-Hook starten würde. Beheben mit: git rm -r --cached node_modules (SM-OSS-011)"
  # **Kein zweiter Befund.** Zuvor folgte unmittelbar „markdownlint-cli2 fehlt
  # — 'npm ci' im Wurzelverzeichnis". Das war unwahr (das Werkzeug kann
  # vorhanden sein) und führte nicht aus der Sperre: `npm ci` legt
  # `node_modules/` neu an, nimmt es aber nicht aus dem Index. Der Nutzer lief
  # im Kreis (S1).
  MDL_GESPERRT=1
elif [ -x node_modules/.bin/markdownlint-cli2 ]; then
  MDL="node_modules/.bin/markdownlint-cli2"
elif command -v markdownlint-cli2 >/dev/null 2>&1; then
  MDL="markdownlint-cli2"
fi
if [ -n "$MDL" ]; then
  # **Der Prüfbereich steht in `.markdownlint-cli2.jsonc` (`globs`), nicht hier.**
  # Er lag zuvor in vier Fassungen im Baum — Konfigurationsdatei, `package.json`,
  # dieses Skript und CLAUDE.md. Vier Antworten auf dieselbe Frage laufen
  # auseinander (CLAUDE.md Abschnitt 11).
  if mdout="$("$MDL" 2>&1)"; then
    pass "Markdown-Stil ($MDL)"
  else
    # markdownlint-cli2 schreibt die Befunde nach stderr im Format datei:zeile:spalte …
    # Ein Zähler auf " error " trifft nie, weil die Ausgabe kein umschlossenes Wort führt.
    mdn="$(printf '%s\n' "$mdout" | grep -cE -- '^[^[:space:]]+:[0-9]+')"
    if [ "${mdn:-0}" -gt 0 ]; then
      finding "Markdown-Stil: $mdn Verstoß/Verstöße (Details: npm run lint:md)"
    else
      # **Null Verstöße bei rotem Werkzeug ist keine Meldung, sondern ein
      # Rätsel.** Passt die Ausgabe nicht auf das erwartete Format —
      # Konfigurationsfehler, andere Fassung —, sagt „0 Verstöße" weder was
      # geschah noch was zu tun ist (sinngemäß SM-NFR-006).
      finding "Markdown-Stil: Werkzeug meldete einen Fehler, dessen Ausgabe nicht auswertbar ist. Letzte Zeilen: $(printf '%s\n' "$mdout" | tail -3 | tr '\n' ' ' | cut -c1-200)"
    fi
  fi
elif [ "${MDL_GESPERRT:-0}" = 1 ]; then
  : # Der Befund steht bereits oben; eine zweite, unwahre Meldung verwirrt nur.
else
  finding "Markdown-Stil nicht prüfbar: markdownlint-cli2 fehlt — 'npm ci' im Wurzelverzeichnis (CLAUDE.md Abschnitt 11)"
fi

# ── 8 · Konsistenz des Implementierungsplans ────────────────────────────────
if [ -f scripts/check-plan.sh ]; then
  out="$(bash scripts/check-plan.sh 2>&1)"; rc_plan=$?
  case "$rc_plan" in
    0) printf '%s\n' "$out" ;;
    3) skip "Planprüfungen (kein Implementierungsplan im Baum)" ;;
    *) printf '%s\n' "$out"; FINDINGS=$((FINDINGS + 1)) ;;
  esac
elif [ -f "$PLAN" ]; then
  # S3: Der Gegenstand liegt im Baum, nur der Prüfer fehlt — das ist FAIL, nicht
  # ENTFÄLLT. Andernfalls schaltete ein versehentlich entfernter oder umbenannter
  # Prüfer sämtliche Planbedingungen still ab, ohne dass eine Stufe rot würde.
  finding "Planprüfungen nicht durchführbar: $PLAN liegt im Baum, scripts/check-plan.sh fehlt (S3)"
else
  skip "Planprüfungen (weder Plan noch Prüfskript im Baum)"
fi

# ── 9 · Lizenzen der Abhängigkeitskette (SM-OSS-009, SM-OSS-011) ────────────
# Positivliste, fail-closed: Eine Lizenz außerhalb der Liste blockiert, statt
# lautlos durchzugehen. Erweitert wird die Liste, indem die Lizenz geprüft und
# der Eintrag begründet wird — nie, indem die Prüfung gelockert wird
# (CLAUDE.md Abschnitt 11). Der Rust-Zweig bekommt sie mit `cargo deny` (AP-01);
# hier geht es um die npm-Kette, für die es sonst keine erzwingende Prüfung gäbe.
if [ ! -f package-lock.json ]; then
  # **Der Gegenstand ist die Kette, nicht die Sperrdatei.** Liegt
  # `package.json` im Baum, existiert eine npm-Abhängigkeitskette — dann ist
  # eine fehlende Sperrdatei FAIL, nicht ENTFÄLLT (S3). Sonst löst das
  # folgende `npm install` ungebunden auf und SM-PLT-009 (reproduzierbare
  # Bauläufe) fällt gleich mit.
  if [ -f package.json ]; then
    finding "package.json liegt im Baum, package-lock.json fehlt — die Abhängigkeitskette ist nicht prüfbar. Sperrdatei mit 'npm install' erzeugen und mitversionieren (S3, SM-OSS-009, SM-PLT-009)"
  else
    skip "Lizenzprüfung der Abhängigkeitskette (keine npm-Kette im Baum)"
  fi
elif ! command -v python3 >/dev/null 2>&1; then
  finding "Lizenzprüfung nicht durchführbar: python3 fehlt (S3, SM-OSS-009)"
elif [ ! -f .lizenzen.conf ]; then
  finding "Lizenzprüfung nicht durchführbar: .lizenzen.conf fehlt (S3, SM-OSS-009)"
else
  # **Der Rückgabewert entscheidet, nicht die Ausgabemenge.** Zuvor wurde er
  # verworfen: Brach das Prüfprogramm ab, war `stdout` leer, der `else`-Zweig
  # griff und meldete „N Pakete geprüft" — für eine Prüfung, die nie lief.
  lizenz_rc=0
  lizenzbefund="$(python3 "$ROOT/scripts/lib/lizenzen.py" 2>"$TMPFEHLER")" || lizenz_rc=$?
  if [ "$lizenz_rc" -ne 0 ]; then
    finding "Lizenz- und Herkunftsprüfung abgebrochen (Rückgabewert $lizenz_rc): $(tr '\n' ' ' < "$TMPFEHLER" | cut -c1-200) — nicht durchführbar heißt nicht bestanden (S3, SM-OSS-009)"
    lizenzbefund=""
    lizenz_abbruch=1
  fi
  if [ -n "$lizenzbefund" ]; then
    # Weder `case` noch Kommandosubstitution in der `while`-Zeile: Der Parser
    # bricht an der schließenden Klammer eines `case`-Musters ab, wenn beides
    # zusammentrifft — derselbe Fehler ist in diesem Repository schon zweimal
    # aufgetreten (CLAUDE.md, Abschnitt 11).
    while IFS="$TAB" read -r art text; do
      # Eine Zeile ohne Präfix ist keine wohlgeformte Meldung. Sie zu
      # verwerfen hieße, einen unverstandenen Zustand als „nichts gefunden"
      # zu lesen.
      if [ -z "$text" ]; then
        finding "Lizenzprüfung lieferte eine unverständliche Meldung: $art (SM-OSS-009)"
      elif [ "$art" = LIZENZ ]; then
        finding "Abhängigkeit außerhalb der Lizenz-Positivliste — $text. Erweitert wird die Liste durch einen begründeten Eintrag in .lizenzen.conf, nie durch Lockern der Prüfung (SM-OSS-009)"
      elif [ "$art" = HERKUNFT ]; then
        finding "Abhängigkeit mit ungeklärter Herkunft — $text. Lockfile mit 'npm install --package-lock-only' gegen registry.npmjs.org neu erzeugen; eine begründete Ausnahme wird als 'herkunft <paket>  # <Grund>' in .lizenzen.conf eingetragen (SM-OSS-011)"
      else
        finding "Lizenzprüfung: $art: $text (SM-OSS-009)"
      fi
    done <<< "$lizenzbefund"
  elif [ "${lizenz_abbruch:-0}" = 0 ]; then
    anzahl="$(python3 -c "import json;print(len([k for k in json.load(open('package-lock.json')).get('packages',{}) if k]))" 2>/dev/null || echo '?')"
    # **Null geprüfte Pakete ist kein Bestehen.** Eine Zahl, die aus einem
    # stillschweigend abgefangenen Fehler stammt, darf keine PASS-Zeile tragen
    # (S3).
    if [ "$anzahl" = "?" ] || [ "${anzahl:-0}" -eq 0 ] 2>/dev/null; then
      finding "Lizenz- und Herkunftsprüfung zählte $anzahl Pakete — ohne geprüften Eintrag ist das kein Bestehen (S3, SM-OSS-009)"
    else
      pass "Lizenzen und Herkunft der Abhängigkeitskette: $anzahl Pakete geprüft"
    fi
  fi
fi

echo
if [ "$FINDINGS" = 0 ]; then
  if [ "$SKIPPED" -gt 0 ]; then
    echo "Dokumentprüfungen: grün — $SKIPPED Prüfung(en) entfallen: $SKIPNAMES"
  else
    echo "Dokumentprüfungen: grün"
  fi
  exit 0
fi
echo "Dokumentprüfungen: $FINDINGS Befund(e)"
exit 1
