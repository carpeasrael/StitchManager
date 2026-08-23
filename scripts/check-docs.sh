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
MOCKUP="Design/stitchmanager-mockup.html"

# Dateien, in denen Farbliterale zulässig sind (die eine Variablenquelle je Medium).
COLOR_ALLOW="$DES $MOCKUP"

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
# core.quotePath=false: sonst liefert git Nicht-ASCII-Pfade escapt ("a/\303\234b.md")
# und jede Datei mit Umlaut fiele lautlos aus der Prüfung.
tracked_and_new() { git -c core.quotePath=false ls-files --cached --others --exclude-standard "$@" 2>/dev/null | sort -u; }
docs()  { tracked_and_new '*.md' | grep -vE '^Reviews/'; }
sources() { tracked_and_new '*.html' '*.qml' '*.rs' '*.css' | grep -vE '^Reviews/'; }

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
  esac
}
ver_bad=0
for f in "$URS" "$DES" "$TEC"; do
  [ -f "$f" ] || continue
  while IFS= read -r ref; do
    [ -n "$ref" ] || continue
    kenn="$(printf '%s' "$ref" | grep -oE '(URS|DES|TEC)-STM-001')"
    want="$(printf '%s' "$ref" | grep -oE 'v[0-9]+\.[0-9]+$' | tr -d 'v')"
    is="$(doc_version "$kenn")"
    [ -n "$is" ] || continue
    if [ "$want" != "$is" ]; then
      finding "$f verweist im Kopf auf $kenn v$want — tatsächlich ist $kenn v$is"
      ver_bad=1
    fi
  done < <(sed -n '1,60p' "$f" | grep -oE '(URS|DES|TEC)-STM-001[^|·]{0,40}v[0-9]+\.[0-9]+')
done
[ "$ver_bad" = 0 ] && pass "Versionsverweise in den Dokumentköpfen stimmen"

# ── 6 · Farbliterale nur in der Variablenquelle (Vorwegnahme D-05) ──────────
# Blockgenau, nicht dateiweit: In der Design-Beschreibung sind Farbwerte allein in
# Abschnitt 3 zulässig — auch die Änderungshistorie nennt Bezeichner, nicht Werte.
HEX='#[0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F][0-9a-fA-F]'
color_bad=0
if [ -f "$DES" ]; then
  n="$(awk -v hex="$HEX" '/^## / { ins = ($0 ~ /^## 3\./) } ins { next } $0 ~ hex { c++ } END { print c+0 }' "$DES")"
  if [ "$n" -gt 0 ]; then
    finding "$DES enthält $n Farbliteral(e) außerhalb von Abschnitt 3 (SM-DES-003, D-05)"
    color_bad=1
  fi
fi
while IFS= read -r f; do
  [ -n "$f" ] || continue
  case " $COLOR_ALLOW " in *" $f "*) continue ;; esac
  n="$(grep -cE "$HEX" "$f" 2>/dev/null || true)"
  if [ "${n:-0}" -gt 0 ]; then
    finding "$f enthält $n Farbliteral(e) — Farben stehen ausschließlich in $DES Abschnitt 3 (SM-DES-003, D-05)"
    color_bad=1
  fi
done < <( { docs; sources; } | sort -u)
[ "$color_bad" = 0 ] && pass "Keine Farbliterale außerhalb der Variablenquelle"
# Das Mockup bleibt vorerst dateiweit ausgenommen: es trägt neben Themenfarben auch
# Inhaltsfarben (Garnfelder, Motiv-Grafiken), die Daten sind und keine Gestaltung.
# Die Trennung setzt die Entscheidung über den Status des Mockups voraus — als
# Restrisiko geführt in Analysis/20260823_01_gate-befunde-rueckstand.md.
skip "Farbliterale im Mockup (Inhalts- und Themenfarben noch nicht getrennt)"

# ── 7 · Markdown-Stil, sofern ein Linter vorhanden ist ──────────────────────
if command -v markdownlint-cli2 >/dev/null 2>&1; then
  if markdownlint-cli2 "**/*.md" >/dev/null 2>&1; then pass "markdownlint-cli2"
  else finding "markdownlint-cli2 meldet Verstöße (Details: markdownlint-cli2 \"**/*.md\")"; fi
elif command -v markdownlint >/dev/null 2>&1; then
  if markdownlint . >/dev/null 2>&1; then pass "markdownlint"
  else finding "markdownlint meldet Verstöße"; fi
else
  skip "Markdown-Stil (markdownlint-cli2 nicht installiert)"
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
