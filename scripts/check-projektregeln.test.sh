#!/usr/bin/env bash
# Selbsttest der Projektregelprüfung.
#
# Je Prüfbedingung mindestens ein **Negativfall**: Ein Prüfer ohne Negativfall
# degradiert unbemerkt zum No-Op, und niemand merkt es, weil er ja grün meldet.
# Zusätzlich je ein Positivfall, damit die Prüfung nicht einfach alles rot färbt.
set -u

WURZEL="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PRUEFER="$WURZEL/scripts/check-projektregeln.sh"

# **`timeout` gehört auf macOS nicht zum Grundsystem.** Ohne diese Prüfung
# liefert *jeder* Fall 127, der Selbsttest ist rot, Stufe 0b blockiert jeden
# Commit — und die Ausgabe zeigt auf den Prüfling statt auf das fehlende
# Werkzeug. Dieselbe Vorkehrung trifft scripts/check-hintergrund.sh (S1/S3).
if command -v timeout >/dev/null 2>&1; then
    ZEITGRENZE=timeout
elif command -v gtimeout >/dev/null 2>&1; then
    ZEITGRENZE=gtimeout
else
    echo "Selbsttest Projektregeln: FAIL — weder 'timeout' noch 'gtimeout' vorhanden."
    echo "  Unter macOS: brew install coreutils (liefert gtimeout)."
    exit 1
fi

# Die Attrappenwerte, die der eigene Prüfer erkennen soll, entstehen aus
# Variablen. Stünden sie literal in dieser Datei, meldete `check-docs.sh` sie
# als Gestaltungsliterale und der Selbsttest blockierte seinen eigenen Commit —
# dieselbe Vorkehrung, die `review-gate.test.sh` für den Injection-Vorfilter
# trifft. Eine Zeilenmarkierung `D-05-Ausnahme:` scheidet für diese Werte aus:
# Sie nähme dem Prüfling genau die Eigenschaft, die der Fall nachweisen soll.
#
# Ausgenommen sind die Fälle, in denen die **Markierung selbst** der
# Prüfgegenstand ist (N1f, N1g): Dort gehört sie in die Attrappe, und der Wert
# steht daneben trotzdem aus Variablen zusammengesetzt.
FAM="font.fam""ily"; RAD="rad""ius"; OPA="opa""city"; DRK="Qt.dar""ker"
# Auch ein Farbwert ist zusammengesetzt — dieselbe Vorkehrung. Das Rautezeichen
# steht getrennt: Jede Teilung, die `#` und drei Hexziffern beisammen lässt,
# ergibt wieder ein gültiges Farbliteral.
RAUTE="#"
HEXWERT="${RAUTE}c8102e"; HEXKURZ="${RAUTE}fff"; HEXLANG="${RAUTE}c8102e80"

# **Aufräumen.** Der Selbsttest läuft in Stufe 0b bei *jedem* Commit und legte
# je Fall ein eigenes `mktemp -d` mit Git-Repo an, ohne es je zu entfernen —
# nach hundert Commits mehrere tausend Verzeichnisse im Temporärbereich. Die
# Schwesterskripte räumen auf; dieses tat es nicht.
TMPWURZEL="$(mktemp -d)"
trap 'rm -rf "$TMPWURZEL"' EXIT

bestanden=0
fehlgeschlagen=0

pruefe() {  # $1 = Name, $2 = erwarteter Rückgabewert, $3 = Verzeichnis, $4… = erwarteter Text
    local name="$1" erwartet="$2" d="$3"; shift 3
    local out rc
    out="$(cd "$d" && "$ZEITGRENZE" 60 bash "$d/scripts/check-projektregeln.sh" 2>&1)"; rc=$?
    local ok=1
    [ "$rc" = "$erwartet" ] || ok=0
    for muster in "$@"; do
        case "$out" in *"$muster"*) ;; *) ok=0 ;; esac
    done
    if [ "$ok" = 1 ]; then
        echo "  ✓ $name"
        bestanden=$((bestanden + 1))
    else
        echo "  ✗ $name (Rückgabewert $rc, erwartet $erwartet)"
        printf '%s\n' "$out" | sed 's/^/      /'
        fehlgeschlagen=$((fehlgeschlagen + 1))
    fi
}

# Legt einen regelkonformen Prüfbaum an; die Fälle verderben ihn dann gezielt.
baue() {
    local d; d="$(mktemp -d "$TMPWURZEL/fall.XXXXXX")"
    mkdir -p "$d/scripts/lib" "$d/crates/ui/qml" "$d/crates/ui/src" "$d/crates/kern-db/src"
    cp "$PRUEFER" "$d/scripts/check-projektregeln.sh"
    # Die gemeinsamen Designregeln gehören mit ins Prüfrepo — der Prüfling
    # bindet sie ein und meldet ohne sie FAIL.
    cp "$WURZEL/scripts/lib/gestaltung.sh" "$d/scripts/lib/gestaltung.sh"
    ( cd "$d" && git init -q . && git config user.email t@t && git config user.name t )

    cat > "$d/.projektregeln.conf" <<'CONF'
oberflaeche = crates/ui
treiber = rusqlite sqlite3 QSqlDatabase
datenhaltung = kern-db
abfragen = crates
CONF
    cat > "$d/Cargo.toml" <<'C'
[workspace]
members = ["crates/ui", "crates/kern-db"]
[workspace.package]
version = "0.1.0"
C
    cat > "$d/crates/ui/qml/Gestaltung.qml" <<'Q'
// GESTALTUNGSQUELLE
import QtQuick
QtObject {
    readonly property color bg: "#faf6f0"  // D-05-Ausnahme: Attrappe eines Prüffalls
    readonly property string schrift: "Lato"
}
Q
    cat > "$d/crates/ui/Cargo.toml" <<'C'
[package]
name = "ui"
[dependencies]
kern-fassade = { path = "../kern-fassade" }
C
    echo 'fn main() { println!("ui"); }' > "$d/crates/ui/src/main.rs"
    cat > "$d/crates/kern-db/src/lib.rs" <<'R'
pub fn suche() -> String {
    let wo = "WHERE id = ?1";
    // Fester Rumpf; der Wert steht als ?1 darin.
    // SM-SEC-005-Ausnahme: er wird gebunden, nicht eingesetzt.
    format!("SELECT a FROM t {wo} LIMIT 10")
}
R
    echo "$d"
}

echo "Selbsttest der Projektregelprüfung"

# ── Positivfall ──────────────────────────────────────────────────────────────
d="$(baue)"
pruefe "P1 regelkonformer Baum besteht" 0 "$d" "PASS"

# ── 1 · D-05 ─────────────────────────────────────────────────────────────────
d="$(baue)"
printf 'Item { property color x: "#ff00ff" }\n' >> "$d/crates/ui/qml/Kachel.qml"  # D-05-Ausnahme: Attrappe eines Negativfalls
pruefe "N1 Farbliteral außerhalb der Gestaltungsquelle blockiert" 1 "$d" \
    "Farbliteral" "FAIL"

d="$(baue)"
printf '// eine Datenfarbe\nItem { property color x: "#ff00ff" } // D-05-Ausnahme: Garnfarbe aus der Datei\n' >> "$d/crates/ui/qml/Kachel.qml"
pruefe "P2 begründete Ausnahme in derselben Zeile besteht" 0 "$d" "PASS"

d="$(baue)"
sed -i.bak 's|// GESTALTUNGSQUELLE|// keine Markierung|' "$d/crates/ui/qml/Gestaltung.qml"
pruefe "N2 fehlende Gestaltungsquelle blockiert" 1 "$d" \
    "keine Datei trägt die Markierung" "FAIL"

d="$(baue)"
printf '// GESTALTUNGSQUELLE\nitem\n' > "$d/crates/ui/qml/Zweite.qml"
pruefe "N3 zwei Gestaltungsquellen blockieren" 1 "$d" \
    "SM-DES-003 verlangt genau eine" "FAIL"

d="$(baue)"
printf 'Text { %s: "Comic Sans" }\n' "$FAM" >> "$d/crates/ui/qml/Kachel.qml"
pruefe "N4 Schriftliteral außerhalb der Gestaltungsquelle blockiert" 1 "$d" \
    "Schriftliteral" "FAIL"

d="$(baue)"
printf 'Item { %s: 7 }\n' "$RAD" >> "$d/crates/ui/qml/Kachel.qml"
pruefe "N4b Abstandsliteral außerhalb der Gestaltungsquelle blockiert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'Item { radius: kn.radiusMittel; spacing: 0 }\n' >> "$d/crates/ui/qml/Kachel.qml"
pruefe "P2b Bezeichner und Null sind zulässig" 0 "$d" "PASS"

# Ein Gestaltungswert *innerhalb* eines Ausdrucks — DES Abschnitt 7 legt die
# Zustandswerte fest, ein selbst gewählter ist ein erfundener Zustand.
d="$(baue)"
printf 'Item { %s: aktiv ? 1.0 : 0.5 }\n' "$OPA" >> "$d/crates/ui/qml/Kachel.qml"
pruefe "N4c Deckkraftwert im Ausdruck blockiert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

# Rechnung bleibt zulässig: `height / 2` ist die Definition von „voll gerundet".
d="$(baue)"
printf 'Item { radius: height / 2; width: kn.s6 * 2 }\n' >> "$d/crates/ui/qml/Kachel.qml"
pruefe "P2c Skalierung eines Ausdrucks ist keine Gestaltung" 0 "$d" "PASS"

# Kurzes und achtstelliges Hex fielen zuvor durch das Muster.
d="$(baue)"
printf 'Item { property color x: "#fff" }\n' >> "$d/crates/ui/qml/Kachel.qml"  # D-05-Ausnahme: Attrappe eines Prüffalls
pruefe "N1c dreistelliges Hex blockiert" 1 "$d" "Farbliteral" "FAIL"

d="$(baue)"
printf 'Item { property color x: "#ff00ff80" }\n' >> "$d/crates/ui/qml/Kachel.qml"  # D-05-Ausnahme: Attrappe eines Prüffalls
pruefe "N1d achtstelliges Hex blockiert" 1 "$d" "Farbliteral" "FAIL"

d="$(baue)"
printf 'Item { color: %s(kn.bg, 1.4) }\n' "$DRK" >> "$d/crates/ui/qml/Kachel.qml"
pruefe "N1e Abdunkelungsfaktor blockiert" 1 "$d" "Farbfaktor" "FAIL"

d="$(baue)"
# Eine Schaltdatei kann die Gestaltungsquelle nicht sein. Ohne diese Schranke
# befreit sich jede Datei, die die Deklaration auch nur erzeugt, selbst.
printf '#!/bin/sh\n# GESTALTUNGSQUELLE\necho "#ff00ff"\n' > "$d/crates/ui/hilf.sh"  # D-05-Ausnahme: Attrappe
chmod +x "$d/crates/ui/hilf.sh"
printf 'Item { property color x: "#00ff00" }\n' >> "$d/crates/ui/qml/Kachel.qml"  # D-05-Ausnahme: Attrappe
pruefe "N1b Schaltdatei ist keine Gestaltungsquelle" 1 "$d" "Farbliteral" "FAIL"

# ── 2 · SM-SEC-004 ───────────────────────────────────────────────────────────
d="$(baue)"
printf 'use kern_db::Datenhaltung;\n' >> "$d/crates/ui/src/main.rs"
pruefe "N5 Verweis der Oberfläche auf die Datenhaltung blockiert" 1 "$d" \
    "verweist unmittelbar auf die Datenhaltung" "FAIL"

d="$(baue)"
printf 'kern-db = { path = "../kern-db" }\n' >> "$d/crates/ui/Cargo.toml"
pruefe "N6 Datenhaltung in der Projektdatei der Oberfläche blockiert" 1 "$d" \
    "greift an der Fassade vorbei" "FAIL"

# ── 3 · SM-SEC-005 ───────────────────────────────────────────────────────────
d="$(baue)"
cat > "$d/crates/kern-db/src/lib.rs" <<'R'
pub fn suche(name: &str) -> String {
    format!("SELECT a FROM t WHERE n = '{name}'")
}
R
pruefe "N7 Abfrage mit benanntem Platzhalter blockiert" 1 "$d" \
    "ohne Begründung" "FAIL"

# Die kanonische Rust-Schreibweise ist das Positionsargument. Sie fiel zuvor
# vollständig durch — die häufigste Einschleusungsform blieb unerkannt.
d="$(baue)"
cat > "$d/crates/kern-db/src/lib.rs" <<'R'
pub fn suche(name: &str) -> String {
    format!("SELECT a FROM t WHERE n = '{}'", name)
}
R
pruefe "N7b Abfrage mit Positionsplatzhalter blockiert" 1 "$d" \
    "ohne Begründung" "FAIL"

d="$(baue)"
cat > "$d/crates/kern-db/src/lib.rs" <<'R'
pub fn suche(name: &str) -> String {
    format!("SELECT a FROM t WHERE n = '{0}'", name)
}
R
pruefe "N7c Abfrage mit nummeriertem Platzhalter blockiert" 1 "$d" \
    "ohne Begründung" "FAIL"

# `write!` baut Zeichenketten genauso zusammen wie `format!`.
d="$(baue)"
cat > "$d/crates/kern-db/src/lib.rs" <<'R'
use std::fmt::Write;
pub fn suche(name: &str) -> String {
    let mut q = String::new();
    write!(q, "SELECT a FROM t WHERE n = '{}'", name).unwrap();
    q
}
R
pruefe "N7d Abfrage über write! blockiert" 1 "$d" "FAIL"

d="$(baue)"
cat > "$d/crates/kern-db/src/lib.rs" <<'R'
pub fn suche(name: &str) -> String {
    let s = "SELECT a FROM t WHERE n = " ;
    let mut q = String::from(s);
    q.push_str(name);
    q
}
R
pruefe "N8 verkettete Abfragezeichenkette blockiert" 1 "$d" "FAIL"

# ── 3b · SM-OSS-011 ─────────────────────────────────────────────────────────
d="$(baue)"
printf '{ "lockfileVersion": 3, "packages": { "": {} } }\n' > "$d/package-lock.json"
pruefe "N11 Abhängigkeitskette ohne .npmrc blockiert" 1 "$d" ".npmrc fehlt" "FAIL"

d="$(baue)"
printf '{ "lockfileVersion": 3, "packages": { "": {} } }\n' > "$d/package-lock.json"
printf 'registry=https://example.invalid\n' > "$d/.npmrc"
pruefe "N12 .npmrc ohne ignore-scripts blockiert" 1 "$d" "ignore-scripts" "FAIL"

d="$(baue)"
printf '{ "lockfileVersion": 3, "packages": { "": {} } }\n' > "$d/package-lock.json"
printf 'ignore-scripts=true\n' > "$d/.npmrc"
pruefe "P7 abgeschaltete Installationsskripte bestehen" 0 "$d" "abgeschaltet"

# ── 4 · SM-PLT-007 ───────────────────────────────────────────────────────────
d="$(baue)"
printf '{ "version": "9.9.9" }\n' > "$d/package.json"
pruefe "N9 abweichende Versionsangaben blockieren" 1 "$d" \
    "weichen voneinander ab" "FAIL"

d="$(baue)"
printf '{ "version": "0.1.0" }\n' > "$d/package.json"
pruefe "P3 gleiche Versionsangaben bestehen" 0 "$d" "stimmen überein"

# ── Anwendbarkeit (S3) ───────────────────────────────────────────────────────
d="$(baue)"
rm "$d/.projektregeln.conf"
pruefe "N10 Quellbaum ohne Zuordnung blockiert (FAIL, nicht ENTFÄLLT)" 1 "$d" \
    "fehlt" "FAIL"

d="$(baue)"
rm "$d/.projektregeln.conf" "$d/Cargo.toml"
pruefe "P4 kein Quellbaum und keine Zuordnung entfällt (Rückgabewert 3)" 3 "$d" \
    "ENTFÄLLT"

d="$(baue)"
rm -rf "$d/crates/ui"
pruefe "P5 fehlende Oberflächenschicht entfällt, blockiert nicht" 0 "$d" \
    "ENTFÄLLT: keine Oberflächenschicht"

# Läuft keine der vier Prüfungen, ist das kein Bestehen (Rückgabewert 3).
d="$(baue)"
rm -rf "$d/crates" "$d/package.json"
printf 'oberflaeche =\ndatenhaltung =\nabfragen =\n' > "$d/.projektregeln.conf"
pruefe "P6 keine Prüfung mit Gegenstand meldet ENTFÄLLT, nicht PASS" 3 "$d" \
    "Prüfungen hatte einen Gegenstand"

# ── Wegneutralität: der Prüfer trägt unter Weg A wie unter Weg B ───────────
#
# Der Skriptkopf begründet ausführlich, dass die Prüfung technologiefrei bleiben
# muss (OP-13, Wegwahl offen). Belegt war das bisher nur für `.qml` und `.rs` —
# für `.py` und `.qss` gab es keinen einzigen Fall, die Zusage war also breiter
# als ihre Deckung.
d="$(baue)"
rm "$d/crates/ui/qml/Gestaltung.qml"
{ printf '# GESTALTUNGSQUELLE\n'; printf 'BRAND = "%s"\n' "$HEXWERT"; } > "$d/crates/ui/gestaltung.py"
git -C "$d" add -A >/dev/null 2>&1
pruefe "Q1 Weg B: eine .py-Datei kann Gestaltungsquelle sein" 0 "$d" \
    "Gestaltungsquelle: crates/ui/gestaltung.py"

d="$(baue)"
printf '.kachel { color: %s; }\n' "$HEXWERT" > "$d/crates/ui/stil.qss"
git -C "$d" add -A >/dev/null 2>&1
pruefe "Q2 Weg B: Farbliteral in einer .qss blockiert" 1 "$d" "Farbliteral" "FAIL"

# ── Mehrzeilige Zuweisung ──────────────────────────────────────────────────
#
# QML bricht Zuweisungen regelmäßig um. Eine zeilenweise Bedingung geht an
# ihrem Gegenstand vorbei, sobald er über eine Zeilengrenze läuft — und meldet
# grün, ohne je etwas gesehen zu haben.
d="$(baue)"
{ printf 'Item {\n'; printf '    %s: aktiv\n' "$OPA"; printf '        ? 0.5\n'
  printf '        : 1.0\n'; printf '}\n'; } > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "N4d Deckkraftwert über einen Zeilenumbruch blockiert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

# ── Skalierende Zahl ist Rechnung, kein Gestaltungsliteral ─────────────────
#
# Zuvor galt `kn.s6 * 2` als Rechnung und `2 * kn.s6` als Literal — die
# Prüfung sah nur den Operator **vor** der Zahl. Eine willkürliche
# Unterscheidung, die zu Ausnahmemarkierungen ohne Sachgrund verleitet.
d="$(baue)"
{ printf 'Item {\n'; printf '    %s: 2 * kn.s6\n' "$RAD"
  printf '    height: parent.height / 2\n'; printf '}\n'; } > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "N4e skalierende Zahl gilt beidseitig als Rechnung" 0 "$d" \
    "keine Literale außerhalb der Gestaltungsquelle"

# ── Die Ausnahmemarkierung verlangt einen Grund ────────────────────────────
d="$(baue)"
printf 'Item { color: "%s" }  // D-05-Ausnahme:\n' "$HEXWERT" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "N1f nackte Ausnahmemarkierung befreit nicht" 1 "$d" "Farbliteral" "FAIL"

d="$(baue)"
printf 'Item { color: "%s" }  // D-05-Ausnahme: Garnfarbe des Pruefbestands.\n' \
    "$HEXWERT" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "N1g begründete Ausnahme wird anerkannt" 0 "$d" \
    "keine Literale außerhalb der Gestaltungsquelle"

# ── SM-SEC-005: die Markierung gilt einer Zeile, nicht einem Block ─────────
#
# Eine einmal gegebene Begründung überdauerte sonst den Austausch der Abfrage
# darunter: Aus einem festen Rumpf wurde ein eingesetzter Fremdwert, und das
# Gate meldete weiter grün.
AUS="SM-SEC-005-Ausnahme"
d="$(baue)"
{ printf 'fn f() {\n'
  printf '    // %s: gilt der Abfrage weiter unten.\n' "$AUS"
  printf '    let x = 1;\n'
  printf '    let y = 2;\n'
  printf '    let s = format!("SELECT * FROM t WHERE a = {}", fremd);\n'
  printf '}\n'; } > "$d/crates/kern-db/src/abfragen.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "R1 Begründung außerhalb der Nachbarzeile trägt nicht" 1 "$d" \
    "ohne Begründung" "FAIL"

d="$(baue)"
{ printf 'fn f() {\n'
  printf '    // %s: fester Rumpf, Werte werden gebunden.\n' "$AUS"
  printf '    let s = format!("SELECT * FROM t WHERE a = {}", nr);\n'
  printf '}\n'; } > "$d/crates/kern-db/src/abfragen.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "R2 Begründung in der Nachbarzeile trägt" 0 "$d" \
    "keine Abfrage setzt einen Wert ohne Begründung ein"

d="$(baue)"
{ printf 'fn f() {\n'
  printf '    // %s:\n' "$AUS"
  printf '    let s = format!("SELECT * FROM t WHERE a = {}", nr);\n'
  printf '}\n'; } > "$d/crates/kern-db/src/abfragen.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "R3 nackte Markierung ohne Grund trägt nicht" 1 "$d" "ohne Begründung" "FAIL"

# ── SM-SEC-005: der Abfragetext aus einer Variablen ────────────────────────
#
# Liegt der Rumpf als Konstante in einer anderen Datei und wird hier nur noch
# zusammengesetzt übergeben, greift weder das `format!`-Raster noch das
# Verkettungsraster — genau der Weg, auf dem ein Fremdwert in den Abfragetext
# gerät.
d="$(baue)"
{ printf 'fn f() {\n'
  printf '    let q = BASIS.to_owned() + fremdwert;\n'
  printf '    conn.prepare(q)?;\n'
  printf '}\n'; } > "$d/crates/kern-db/src/suche.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "R4 Abfragetext aus einer Variablen ist begründungspflichtig" 1 "$d" \
    "aus einer Variablen" "FAIL"

d="$(baue)"
{ printf 'fn f() {\n'
  printf '    conn.prepare("SELECT * FROM t WHERE a = ?1")?;\n'
  printf '    stmt.execute(params![wert])?;\n'
  printf '}\n'; } > "$d/crates/kern-db/src/suche.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "R5 Literal und gebundene Werte bestehen" 0 "$d" \
    "keine Abfrage setzt einen Wert ohne Begründung ein"

# ── SM-SEC-004: auch der unmittelbare Treiberzugriff umgeht die Fassade ────
d="$(baue)"
printf 'use rusqlite::Connection;\n' > "$d/crates/ui/src/direkt.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "S1 Treiberzugriff aus der Oberfläche blockiert" 1 "$d" \
    "Datenbanktreiber" "FAIL"

# ── .npmrc: Positivliste statt Stichprobe (SM-OSS-011, SM-SEC-007) ────────
d="$(baue)"
printf \'{ "lockfileVersion": 3, "packages": { "": {} } }\n\' > "$d/package-lock.json"
printf 'ignore-scripts=true\nregistry=https://paket.angreifer.example/\n' > "$d/.npmrc"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T1 Umleitung der Bezugsquelle blockiert" 1 "$d" "nicht freigegebene Direktiven" "FAIL"

d="$(baue)"
printf \'{ "lockfileVersion": 3, "packages": { "": {} } }\n\' > "$d/package-lock.json"
printf 'ignore-scripts=true\nstrict-ssl=false\n' > "$d/.npmrc"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T2 abgesenkte Transportsicherung blockiert" 1 "$d" "nicht freigegebene Direktiven" "FAIL"

TOK="_auth""Token"
d="$(baue)"
printf \'{ "lockfileVersion": 3, "packages": { "": {} } }\n\' > "$d/package-lock.json"
printf 'ignore-scripts=true\n//paket.example/:%s=geheim\n' "$TOK" > "$d/.npmrc"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T3 Zugangstoken in .npmrc blockiert" 1 "$d" "nicht freigegebene Direktiven" "FAIL"

d="$(baue)"
printf \'{ "lockfileVersion": 3, "packages": { "": {} } }\n\' > "$d/package-lock.json"
printf '# Begruendung\nignore-scripts=true\naudit=false\n' > "$d/.npmrc"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T4 freigegebene Direktiven bestehen" 0 "$d" "keine weiteren Direktiven"

# ── Treiber im Manifest der Oberfläche (SM-SEC-004) ───────────────────────
d="$(baue)"
printf '[dependencies]\nrusqlite = "0.32"\n' > "$d/crates/ui/Cargo.toml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T5 Treiber im Oberflaechen-Manifest blockiert" 1 "$d" \
    "Datenbanktreiber" "FAIL"

# ── Ein Eintrag mit Sonderzeichen darf die Prüfung nicht abschalten ───────
#
# Die Treiberliste wird zu einem regulären Ausdruck zusammengesetzt. War die
# Maskierung unvollständig, erzeugte ein Eintrag mit unbalancierter Klammer ein
# ungültiges Muster; `grep` brach mit Rückgabewert 2 ab, `2>/dev/null`
# verschluckte die Meldung, und die Prüfung meldete „kennt keinen Treiber",
# ohne eine Datei bewertet zu haben — fail-open für die einzige Schranke
# zwischen Oberfläche und Datenhaltung (SM-SEC-004).
d="$(baue)"
sed -i.bak 's/^treiber = .*/treiber = QSqlQuery( rusqlite/' "$d/.projektregeln.conf"
rm -f "$d/.projektregeln.conf.bak"
printf 'use rusqlite::Connection;\n' > "$d/crates/ui/src/direkt.rs"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T6 ein Eintrag mit Sonderzeichen schaltet die Pruefung nicht ab" 1 "$d" \
    "Datenbanktreiber" "FAIL"

# ── Rahmenstärke: 1 ist bei einer Länge ein Gestaltungswert ───────────────
#
# Zuvor galt die 0/1-Ausnahme für **alle** Maßeigenschaften. `border.width: 1`
# passierte damit ungeprüft, während der Schwesterwert `fokusstaerke: 2`
# gefunden wurde — zwei Maßstäbe für dieselbe Tabelle (DES Abschnitt 5).
BW="border.wid""th"
d="$(baue)"
printf 'Item { %s: 1 }\n' "$BW" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T7 Rahmenstaerke 1 ist ein Gestaltungswert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'Item { %s: aktiv ? 1.0 : 0.0 }\n' "$OPA" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "T8 Deckkraft 0 und 1 bleiben zulaessig" 0 "$d" \
    "keine Literale außerhalb der Gestaltungsquelle"

# ── Die von Turing benannten Klassen (DES Abschnitt 4, 5 und 8) ──────────
PROP="prop""erty"; DUR="dura""tion"; LH="lineHei""ght"; LW="lineWid""th"
SETSP="setSpa""cing"

d="$(baue)"
printf 'Item { %s int groesse: 16 }\n' "$PROP" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U1 eigene Maßdeklaration im Bauteil blockiert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'Item { %s int gewaehlteZeile: -1 }\n' "$PROP" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U1b ein Zustandsindex ist keine Gestaltung" 0 "$d" \
    "keine Literale außerhalb der Gestaltungsquelle"

d="$(baue)"
printf 'NumberAnimation { %s: 150 }\n' "$DUR" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U2 Bewegungsdauer blockiert (DES Abschnitt 8, SM-NFR-013)" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'Text { %s: 1.45 }\n' "$LH" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U3 Zeilenhoehe blockiert (DES Abschnitt 4)" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'function f(g) { g.%s = 2 }\n' "$LW" > "$d/crates/ui/qml/Kachel.qml"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U4 Strichstaerke ueber = blockiert (DES Abschnitt 5)" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

# Weg B: die Methodenform ist die uebliche Schreibweise unter PySide6.
d="$(baue)"
printf 'w.%s(8)\n' "$SETSP" > "$d/crates/ui/leiste.py"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U5 Weg B: Methodenform blockiert" 1 "$d" \
    "Abstands-, Maß- oder Schriftgradliteral" "FAIL"

d="$(baue)"
printf 'w.%s(kn.s2)\n' "$SETSP" > "$d/crates/ui/leiste.py"
git -C "$d" add -A >/dev/null 2>&1
pruefe "U5b Weg B: Bezeichner in der Methodenform besteht" 0 "$d" \
    "keine Literale außerhalb der Gestaltungsquelle"

echo
echo "Selbsttest Projektregeln: $bestanden bestanden, $fehlgeschlagen fehlgeschlagen"
[ "$fehlgeschlagen" -eq 0 ]
