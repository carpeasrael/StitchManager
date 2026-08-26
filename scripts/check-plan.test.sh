#!/usr/bin/env bash
# Selbsttest von scripts/check-plan.sh — je Prüfbedingung mindestens ein Negativfall,
# einschließlich der Bedingungen 5b (Unterfallmenge) und 11 (Zählwerte der README).
# Wer eine Bedingung ergänzt, ergänzt hier den Fall: Eine Pauschalzusage ohne Deckung
# ist selbst ein Befund (Runde 13, T-5; Runde 14, T-M3 und C-M3).
# Ein Prüfer, dessen Verdrahtung nie geprüft wird, degradiert unbemerkt zu einem No-Op.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
PRUEFER="$ROOT/scripts/check-plan.sh"
TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT
OK=0; FEHL=0

# Ein Plan, der alle Bedingungen erfüllt. Jeder Fall verfälscht genau eine Stelle.
basis() {
cat <<'EOF'
**Kennung:** IMP-STM-001
**Version:** 1.0
**Datum:** 24.08.2026
**Führendes Dokument:** URS-STM-001 v1.3
**Mitgeltend:** DES-STM-001 v1.3

---

## 0. Zweck

Text.

## 2. Entscheidungsgatter

| **OP-01** | Frage? | AP-01 | Weg |
| **OP-02** | Frage? | AP-01 | Weg |

Bis zur Klärung dürfen nicht zwei offene Punkte jede Arbeit anhalten.

## 3. Zielarchitektur

Text.

### 6.1 Prüffallschema

| SM-LIB-001 | PF-LIB-01 | Zweig |

### 6.2 Methodenverteilung

Über die 2 verplanten Anforderungen:

### AP-01 · Beispiel

- **Zugeordnet:** SM-LIB-001, SM-LIB-002
- **Nachweis:** PF-LIB-01, PF-LIB-02

## 7. Rückverfolgbarkeitsmatrix

Jede der 2 verplanten Anforderungen ist genau einem Arbeitspaket zugeordnet.

| SM-LIB-001 | M | T | AP-01 | `kern` | PF-LIB-01 | offen |
| SM-LIB-002 | M | T | AP-01 | `kern` | PF-LIB-02 | offen |

| **Summe** | 2 | **2** | 0 |

## 8. Mitwachsende Prüfkette

Text.

### 10.1 Zurückgestellte Soll-Anforderungen

| Bereich | SM-LIB-003 | Begründung |

### 10.2 Kann-Anforderungen

Text.
EOF
}
urs() {
cat <<'EOF'
| SM-LIB-001 | Anforderung | M | T |
| SM-LIB-002 | Anforderung | M | T |
| **OP-01** | Frage | Auswirkung | Frist |
| **OP-02** | Frage | Auswirkung | Frist |
EOF
}

fall() { # name, sed-Ausdruck auf den Plan, erwartet: "befund" oder "grün"
  local name="$1" aend="$2" erwartet="$3"
  mkdir -p "$TMP/Implementation" "$TMP/Requirements" "$TMP/Analysis"
  basis > "$TMP/Implementation/p.md"
  urs > "$TMP/Requirements/u.md"
  : > "$TMP/Analysis/x_implementierungsplan.md"
  [ -n "$aend" ] && perl -0pi -e "$aend" "$TMP/Implementation/p.md"
  local rc=0
  ( cd "$TMP" && CHECK_PLAN_FILE="Implementation/p.md" CHECK_PLAN_URS="Requirements/u.md" \
      CHECK_PLAN_ANA="Analysis/*_implementierungsplan.md" bash "$PRUEFER" ) >/dev/null 2>&1 || rc=$?
  if { [ "$erwartet" = befund ] && [ "$rc" = 1 ]; } || { [ "$erwartet" = grün ] && [ "$rc" = 0 ]; }; then
    printf '  ✓ %s\n' "$name"; OK=$((OK+1))
  else
    printf '  ✗ %s (exit %s, erwartet %s)\n' "$name" "$rc" "$erwartet"; FEHL=$((FEHL+1))
  fi
}

echo "Selbsttest der Planprüfungen"
fall "B0  unverfälschter Plan ist grün"                ''                                                          grün
fall "B1  Kennung zugleich im Umfang und zurückgestellt" 's/SM-LIB-003/SM-LIB-002/'                                 befund
fall "B2  Arbeitspaket und Matrix weichen ab"          's/- \*\*Zugeordnet:\*\* SM-LIB-001, SM-LIB-002/- **Zugeordnet:** SM-LIB-001/' befund
fall "B3  zugleich zugeordnet und Mitwirkung"          's/- \*\*Nachweis:\*\*/- **Mitwirkung:** SM-LIB-001\n- **Nachweis:**/'         befund
fall "B4  Zählwert weicht ab"                          's/Jede der 2 verplanten/Jede der 5 verplanten/'             befund
fall "B5  Prüffall ohne Eintrag im Schema"             's/PF-LIB-01, PF-LIB-02/PF-LIB-01, PF-LIB-09/'               befund
fall "B6  Zeile über 100 Zeichen"                      's/^Text\.$/Text. AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA/m' befund
fall "B7  OP-Verweis ohne Definition"                  's/\*\*OP-01\*\* \| Frage\?/**OP-09** | Frage?/'             befund
fall "B8  falsches Zahlwort"                           's/nicht zwei offene Punkte/nicht sieben offene Punkte/'     befund
# B4b und B8b prüfen dieselben Bedingungen über einen **Zeilenumbruch** hinweg.
# Ohne sie deckten B4 und B8 den Ausfall nicht auf: Beide Attrappen setzen die
# Wortfolge einzeilig, der Plan bricht sie bei 100 Zeichen um — die Bedingungen
# fanden ihren Gegenstand deshalb nie und meldeten grün.
fall "B4b Zählwert weicht ab, über einen Zeilenumbruch"  's/Jede der 2 verplanten/Jede der 5\nverplanten/'          befund
fall "B8b falsches Zahlwort, über einen Zeilenumbruch"   's/nicht zwei offene Punkte/nicht sieben offene\nPunkte/'  befund
fall "B8c Nicht-Zahlwort löst keinen Befund aus"          's/nicht zwei offene Punkte/nicht diese offene Punkte/'     grün

fall "B9  unbalancierte Auszeichnung"                  's/^## 0\. Zweck$/## 0. Zweck\n\nEin **kaputter Absatz./m'    befund
fall "B10 Ankerüberschrift fehlt"                      's/### 10\.1 Zurückgestellte Soll-Anforderungen/### 10.1 Umbenannt/' befund
fall "B20 Kopffeld nicht am Zeilenanfang"                's/\*\*Version:\*\* 1\.0/x **Version:** 1.0/'      befund
fall "B15 stehen gebliebener Artikel vor der Auszeichnung" 's/^Text\.$/Der **Das Ergebnis liegt vor.**/m' befund
fall "B13 OP geführt, aber nicht im Entscheidungsgatter" 's/\| \*\*OP-02\*\* \| Frage\? \| AP-01 \| Weg \|\n//'        befund
fall "B14 Matrix führt eine Kennung ohne ihren Unterfall" 's/\| SM-LIB-001 \| PF-LIB-01 \| Zweig \|/| SM-LIB-001 | PF-LIB-01.1, .2 | Zweig |/; s/PF-LIB-01, PF-LIB-02/PF-LIB-01.1, PF-LIB-02/; s/\| PF-LIB-01 \| offen \|/| PF-LIB-01.1 | offen |/' befund

# Bedingung 10 arbeitet an der Analyse, nicht am Plan — eigener Aufbau.
# Der zweite Fall ist die Regression zu T-5 aus Runde 13: Vor der Behebung ordnete
# die Bedingung eine Überschrift "Zwölfte Runde" wegen eines Rückverweises der
# Runde 11 zu, fand deren Tabelle und meldete grün.
fall_ana() { # name, Inhalt der Analyse, erwartet
  local name="$1" inhalt="$2" erwartet="$3"
  mkdir -p "$TMP/Implementation" "$TMP/Requirements" "$TMP/Analysis"
  basis > "$TMP/Implementation/p.md"
  urs > "$TMP/Requirements/u.md"
  printf '%s\n' "$inhalt" > "$TMP/Analysis/x_implementierungsplan.md"
  local rc=0
  ( cd "$TMP" && CHECK_PLAN_FILE="Implementation/p.md" CHECK_PLAN_URS="Requirements/u.md" \
      CHECK_PLAN_ANA="Analysis/*_implementierungsplan.md" bash "$PRUEFER" ) >/dev/null 2>&1 || rc=$?
  if { [ "$erwartet" = befund ] && [ "$rc" = 1 ]; } || { [ "$erwartet" = grün ] && [ "$rc" = 0 ]; }; then
    printf '  ✓ %s\n' "$name"; OK=$((OK+1))
  else
    printf '  ✗ %s (exit %s, erwartet %s)\n' "$name" "$rc" "$erwartet"; FEHL=$((FEHL+1))
  fi
}
# Bedingung 11 liest die README, nicht den Plan — eigener Aufbau mit Attrappe.
fall_readme() { # name, Inhalt der README (leer = keine README), erwartet
  local name="$1" inhalt="$2" erwartet="$3"
  mkdir -p "$TMP/Implementation" "$TMP/Requirements" "$TMP/Analysis"
  basis > "$TMP/Implementation/p.md"
  urs > "$TMP/Requirements/u.md"
  : > "$TMP/Analysis/x_implementierungsplan.md"
  rm -f "$TMP/R.md"
  [ -n "$inhalt" ] && printf '%s\n' "$inhalt" > "$TMP/R.md"
  local rc=0
  ( cd "$TMP" && CHECK_PLAN_FILE="Implementation/p.md" CHECK_PLAN_URS="Requirements/u.md" \
      CHECK_PLAN_ANA="Analysis/*_implementierungsplan.md" CHECK_PLAN_README="R.md" \
      bash "$PRUEFER" ) >/dev/null 2>&1 || rc=$?
  if { [ "$erwartet" = befund ] && [ "$rc" = 1 ]; } || { [ "$erwartet" = grün ] && [ "$rc" = 0 ]; }; then
    printf '  ✓ %s\n' "$name"; OK=$((OK+1))
  else
    printf '  ✗ %s (exit %s, erwartet %s)\n' "$name" "$rc" "$erwartet"; FEHL=$((FEHL+1))
  fi
}
# Die Attrappe nennt die vier Etiketten mit den Werten des Attrappenplans:
# 2 Anforderungen, 2 verplant, 1 Arbeitspaket, 2 offene Punkte.
README_OK='| Anforderungen im Lastenheft | 2 |
| Für Version 1.0 verplant | 2 Anforderungen |
| Arbeitspakete | 1 |
| Offene Punkte | 2 |'
fall_readme "B16 README ohne Zählwerte ist grün"        ''                                          grün
fall_readme "B17 stimmige README-Zählwerte sind grün"   "$README_OK"                                grün
fall_readme "B18 verfälschter README-Zählwert"          "${README_OK/| Arbeitspakete | 1 |/| Arbeitspakete | 99 |}" befund
fall_readme "B19 gelöschte README-Zeile bleibt nicht unbemerkt" "${README_OK/| Offene Punkte | 2 |/}" befund

fall_ana "B12 beschriebene Runde ohne Befundtabelle" \
  '**8.4 Zweite Runde.** Text.' befund
fall_ana "B12b Ordinalwort schlägt den Rückverweis nicht" \
  '**8.24 Zwölfte Runde.** Aufgeworfen in Runde 11.

### 8.23 Befunde der Runde 11

| Nr | Ort |
|---|---|
| T-1 | irgendwo |' befund
fall_ana "B12c vollständig belegte Runde ist grün" \
  '**8.4 Zweite Runde.** Text.

### 8.5 Befunde der Runde 2

| Nr | Ort |
|---|---|
| T-1 | irgendwo |' grün

# ENTFÄLLT: kein Plan im Baum → Exit 3, nie 0
mkdir -p "$TMP/leer/Requirements"; urs > "$TMP/leer/Requirements/u.md"
rc=0
( cd "$TMP/leer" && CHECK_PLAN_FILE="Implementation/fehlt.md" CHECK_PLAN_URS="Requirements/u.md" \
    bash "$PRUEFER" ) >/dev/null 2>&1 || rc=$?
if [ "$rc" = 3 ]; then printf '  ✓ B11 kein Plan im Baum meldet ENTFÄLLT (Exit 3)\n'; OK=$((OK+1))
else printf '  ✗ B11 kein Plan im Baum: exit %s, erwartet 3\n' "$rc"; FEHL=$((FEHL+1)); fi

echo
if [ "$FEHL" = 0 ]; then echo "Planprüfungen-Selbsttest: $OK bestanden, 0 fehlgeschlagen"; exit 0; fi
echo "Planprüfungen-Selbsttest: $OK bestanden, $FEHL fehlgeschlagen"; exit 1
