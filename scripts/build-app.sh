#!/usr/bin/env bash
# Baut das macOS-App-Bündel und ein Plattenabbild (.dmg) für Probeläufe.
#
# Ergebnis ist ein **Entwicklungsbau**, keine Veröffentlichung: Er ist weder
# Universal-Paket (SM-PLT-001) noch notarisiert (SM-PLT-002, OP-04), und die
# Anwendungskennung ist vorläufig (OP-06). Herleitung und Grenzen stehen in
# `Analysis/20261017_01_app-paket.md`.
#
# Läuft **nicht** im Gate: ein vollständiger Bau mit rund 135 MB Ergebnis je
# Commit würde umgangen statt befolgt — dieselbe Abwägung wie bei
# `check-hintergrund.sh`.
#
# Rückgabewerte: 0 PASS · 1 FAIL · 3 ENTFÄLLT (nicht unter macOS).
#
# Stellschrauben:
#   SM_APP_KENNUNG  CFBundleIdentifier  (invalid.stitchmanager.entwicklungsbau)
#   SM_APP_NAME     Bündel- und Abbildname (StitchManager, Arbeitsname)
#   SM_SIGNATUR     Signaturkennung für codesign (-, also ad hoc)
#
# Aufruf: bash scripts/build-app.sh [--ohne-startprobe]
set -u

WURZEL="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WURZEL" || exit 1

fail() { echo "build-app: FAIL — $*"; exit 1; }
hinweis() { echo "build-app: Hinweis — $*"; }
schritt() { echo "build-app: $*"; }

STARTPROBE=1
for arg in "$@"; do
    case "$arg" in
        --ohne-startprobe) STARTPROBE=0 ;;
        -h|--hilfe) sed -n '2,23p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'; exit 0 ;;
        *) fail "unbekannter Schalter '$arg' (--ohne-startprobe, --hilfe)" ;;
    esac
done

# ── Anwendbarkeit (S3) ───────────────────────────────────────────────────────
# App-Bündel und Plattenabbild gibt es nur unter macOS. Anderswo ist der
# Gegenstand nicht vorhanden — das ist ENTFÄLLT, nicht FAIL.
if [ "$(uname -s)" != "Darwin" ]; then
    echo "build-app: ENTFÄLLT — Bündel und .dmg entstehen nur unter macOS."
    exit 3
fi

# ── Werkzeuge ────────────────────────────────────────────────────────────────
# Fehlt ein Werkzeug, ist das FAIL **mit Bezugsweg**: Der Gegenstand liegt im
# Baum, nur der Bau kann nicht laufen.
BREW="$(brew --prefix 2>/dev/null || echo /opt/homebrew)"
MACDEPLOYQT="$(command -v macdeployqt || true)"
[ -n "$MACDEPLOYQT" ] || [ ! -x "$BREW/opt/qt/bin/macdeployqt" ] || MACDEPLOYQT="$BREW/opt/qt/bin/macdeployqt"
[ -n "$MACDEPLOYQT" ] || fail "macdeployqt fehlt — Bezugsweg: brew install qt"
for w in cargo python3 codesign hdiutil otool install_name_tool lipo shasum; do
    command -v "$w" >/dev/null 2>&1 || fail "$w fehlt — Bezugsweg: Xcode-Befehlszeilenwerkzeuge (xcode-select --install) bzw. rustup"
done

# ── Angaben aus dem Baum ─────────────────────────────────────────────────────
# Der Pfad der Oberfläche steht in `.projektregeln.conf`, nicht hier
# (CLAUDE.md Abschnitt 11): dieselbe Frage zweimal beantwortet ergäbe unter
# einem anderen Oberflächenweg zwei Wahrheiten.
konf() { sed -n "s/^[[:space:]]*$1[[:space:]]*=[[:space:]]*//p" .projektregeln.conf 2>/dev/null | sed 's/[[:space:]]*$//' | head -1; }
OBERFLAECHE="$(konf oberflaeche)"
[ -n "$OBERFLAECHE" ] || fail "'oberflaeche' fehlt in .projektregeln.conf"
case "$OBERFLAECHE" in
    /*|*..*) fail "'oberflaeche' muss ein Pfad im Baum sein, ohne '/' am Anfang und ohne '..' (SM-SEC-001/002)" ;;
esac
QMLDIR="$OBERFLAECHE/qml"
[ -f "$OBERFLAECHE/Cargo.toml" ] || fail "$OBERFLAECHE/Cargo.toml fehlt"
[ -d "$QMLDIR" ] || fail "$QMLDIR fehlt"

# Paketname, Version und Programmname liefert cargo selbst. Die Version steht
# damit genau einmal im Baum (SM-PLT-007).
META="$(cargo metadata --format-version 1 --no-deps --locked 2>/dev/null)" \
    || fail "cargo metadata scheitert — Cargo.lock nicht aktuell?"
read -r PAKET VERSION PROGRAMM < <(printf '%s' "$META" | python3 -I -c '
import json, os, sys
ziel = os.path.realpath(sys.argv[1])
for p in json.load(sys.stdin)["packages"]:
    if os.path.realpath(p["manifest_path"]) == ziel:
        bins = [t["name"] for t in p["targets"] if "bin" in t["kind"]]
        if len(bins) == 1:
            print(p["name"], p["version"], bins[0])
' "$WURZEL/$OBERFLAECHE/Cargo.toml")
[ -n "${PROGRAMM:-}" ] || fail "kein eindeutiges Programm in $OBERFLAECHE/Cargo.toml"

# Die Werte landen in Info.plist und in Dateinamen. Ein enges Zeichenraster
# schließt XML-Einschleusung und Pfadsprünge zugleich aus.
KENNUNG="${SM_APP_KENNUNG:-invalid.stitchmanager.entwicklungsbau}"
NAME="${SM_APP_NAME:-StitchManager}"
SIGNATUR="${SM_SIGNATUR:--}"
[[ "$KENNUNG" =~ ^[A-Za-z0-9.-]+$ ]] || fail "SM_APP_KENNUNG enthält unzulässige Zeichen"
[[ "$NAME" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]] || fail "SM_APP_NAME enthält unzulässige Zeichen"
[[ "$VERSION" =~ ^[0-9A-Za-z.+-]+$ ]] || fail "Version '$VERSION' unzulässig"
[[ "$PROGRAMM" =~ ^[A-Za-z0-9_-]+$ ]] || fail "Programmname '$PROGRAMM' unzulässig"
case "$KENNUNG" in
    invalid.*) hinweis "vorläufige Anwendungskennung '$KENNUNG' — OP-06 ist offen" ;;
esac

# Der LGPL-3-Text liegt im Baum, weil Homebrew-Qt ihn nicht mitliefert. Die
# Prüfsumme ist die des unveränderten FSF-Textes: Ein veränderter Lizenztext
# erfüllt SM-OSS-006 nicht.
LGPL="Lizenzen/LGPL-3.0.txt"
LGPL_SHA256="e3a994d82e644b03a792a930f574002658412f62407f5fee083f2555c5f23118"
[ -f "$LGPL" ] || fail "$LGPL fehlt (SM-OSS-006)"
[ "$(shasum -a 256 "$LGPL" | cut -d' ' -f1)" = "$LGPL_SHA256" ] \
    || fail "$LGPL weicht vom FSF-Text ab (SM-OSS-006)"
[ -f LICENSE ] || fail "LICENSE fehlt (SM-OSS-005)"

# ── Bau ──────────────────────────────────────────────────────────────────────
# `--locked` bindet den Bau an Cargo.lock (SM-PLT-009).
schritt "baue $PAKET $VERSION (release) …"
cargo build --release --locked -p "$PAKET" || fail "cargo build"
ZIELDIR="$(printf '%s' "$META" | python3 -I -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
BINAER="$ZIELDIR/release/$PROGRAMM"
[ -x "$BINAER" ] || fail "$BINAER nicht gebaut"
ARCH="$(lipo -archs "$BINAER" | tr ' ' '+')"

AUSGABE="$ZIELDIR/paket"
APP="$AUSGABE/$NAME.app"
mkdir -p "$AUSGABE" || fail "$AUSGABE nicht anlegbar"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources" || fail "Bündel nicht anlegbar"
cp "$BINAER" "$APP/Contents/MacOS/$PROGRAMM" || fail "Programm nicht kopierbar"

cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key><string>$NAME</string>
    <key>CFBundleDisplayName</key><string>$NAME</string>
    <key>CFBundleIdentifier</key><string>$KENNUNG</string>
    <key>CFBundleExecutable</key><string>$PROGRAMM</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>$VERSION</string>
    <key>CFBundleVersion</key><string>$VERSION</string>
    <key>CFBundleDevelopmentRegion</key><string>de</string>
    <key>NSHighResolutionCapable</key><true/>
    <key>LSMinimumSystemVersion</key><string>12.0</string>
</dict>
</plist>
EOF

# ── Qt einbetten ─────────────────────────────────────────────────────────────
# macdeployqt kopiert die Frameworks nach Contents/Frameworks; die Bindung
# bleibt dynamisch und austauschbar (SM-OSS-006).
schritt "bette Qt ein …"
PROTOKOLL="$AUSGABE/macdeployqt.log"
"$MACDEPLOYQT" "$APP" -qmldir="$QMLDIR" -verbose=1 >"$PROTOKOLL" 2>&1
# Der Rückgabewert von macdeployqt sagt wenig: Es meldet unauflösbare
# Verweise optionaler Module (in Homebrew getrennt verpackt) als ERROR und
# beendet sich trotzdem mit 0. Maßgeblich sind die Prüfungen darunter.
N_FEHLER="$(grep -c '^ERROR' "$PROTOKOLL" || true)"
[ "$N_FEHLER" = 0 ] || hinweis "macdeployqt meldet $N_FEHLER Zeilen ERROR (Protokoll: $PROTOKOLL) — bewertet wird unten"

macho_dateien() {
    find "$APP/Contents" -type f \( -perm -u+x -o -name '*.dylib' \) -print0 \
        | while IFS= read -r -d '' f; do
            file -b "$f" | grep -q 'Mach-O' && printf '%s\n' "$f"
        done
}
fremd() { grep -E "^($BREW|/usr/local)/"; }
abhaengigkeiten() { otool -L "$1" | tail -n +2 | sed 's/^[[:space:]]*//; s/ (compatibility.*//'; }
suchpfade() { otool -l "$1" | awk '/LC_RPATH/{r=1} r&&/ path /{print $2; r=0}'; }
FW="$APP/Contents/Frameworks"

# **Restverweise auf Homebrew.** macdeployqt lässt einzelne absolute Verweise
# und Suchpfade stehen. Auf dem Bauplatz fällt das nicht auf — die Bibliothek
# liegt ja dort —, auf jedem anderen Gerät startet das Bündel nicht. Jeder
# Verweis wird auf die Kopie in Frameworks umgehängt, fehlt sie, wird sie
# nachgeholt. Das wiederholt sich, bis keine Änderung mehr eintritt, weil eine
# nachgeholte Bibliothek ihrerseits Verweise mitbringt.
schritt "hänge Restverweise auf Homebrew um …"
for _durchgang in 1 2 3 4 5 6; do
    geaendert=0
    while IFS= read -r f; do
        # Eine Bibliothek nennt in `otool -L` auch ihre eigene Kennung. Zeigt
        # die noch auf Homebrew, wird sie neu gesetzt, nicht umgehängt.
        eigen="$(otool -D "$f" | tail -n +2 | fremd)"
        if [ -n "$eigen" ]; then
            install_name_tool -id "@executable_path/../Frameworks/$(basename "$eigen")" "$f" 2>/dev/null \
                || fail "Kennung von ${f#"$APP"/} nicht setzbar"
            geaendert=1
        fi
        for dep in $(abhaengigkeiten "$f" | fremd); do
            base="$(basename "$dep")"
            if [ ! -e "$FW/$base" ]; then
                cp -L "$dep" "$FW/$base" || fail "$dep nicht kopierbar"
                chmod u+w "$FW/$base"
                install_name_tool -id "@executable_path/../Frameworks/$base" "$FW/$base" 2>/dev/null
            fi
            install_name_tool -change "$dep" "@executable_path/../Frameworks/$base" "$f" 2>/dev/null \
                || fail "Verweis in ${f#"$APP"/} nicht umhängbar"
            geaendert=1
        done
        for rp in $(suchpfade "$f" | fremd); do
            install_name_tool -delete_rpath "$rp" "$f" 2>/dev/null \
                || fail "Suchpfad in ${f#"$APP"/} nicht entfernbar"
            geaendert=1
        done
    done < <(macho_dateien)
    [ "$geaendert" = 0 ] && break
done

# **Unauflösbare @rpath-Verweise.** Ein Erweiterungsmodul, dessen Bibliothek
# nicht im Bündel liegt, lädt Qt ohnehin nicht. Es wird entfernt und benannt,
# statt als totes Gewicht mitzureisen. Fehlt dagegen einer Bibliothek unter
# Frameworks oder dem Programm selbst ein Verweisziel, ist das Bündel defekt.
ENTFERNT=()
while IFS= read -r f; do
    for dep in $(abhaengigkeiten "$f" | grep '^@rpath/'); do
        [ -e "$FW/${dep#@rpath/}" ] && continue
        case "$f" in
            "$APP/Contents/PlugIns/"*|"$APP/Contents/Resources/qml/"*)
                rm -f "$f"; ENTFERNT+=("${f#"$APP/Contents/"}"); break ;;
            *) fail "${f#"$APP"/} verweist auf fehlendes ${dep#@rpath/}" ;;
        esac
    done
done < <(macho_dateien)
[ "${#ENTFERNT[@]}" = 0 ] || hinweis "${#ENTFERNT[@]} Erweiterungsmodule ohne Bibliothek entfernt: ${ENTFERNT[*]}"

# Die QML-Module unter Resources/qml verweisen per Symlink auf ihre Bibliothek
# unter PlugIns. Nach dem Entfernen zeigt der Verweis ins Leere, und
# `codesign --strict` lehnt das Bündel ab. Ein Modul ohne Bibliothek ist nicht
# ladbar — es geht als Ganzes.
while IFS= read -r -d '' l; do
    [ -e "$l" ] || rm -rf "$(dirname "$l")"
done < <(find "$APP/Contents/Resources/qml" -type l -print0 2>/dev/null)
TOT="$(find "$APP" -type l ! -exec test -e {} \; -print)"
[ -z "$TOT" ] || { printf '%s\n' "$TOT" | head -5; fail "verwaiste Verweise im Bündel"; }

# Abschlussprüfung: kein einziger Verweis und kein Suchpfad zeigt noch aus dem
# Bündel heraus auf eine Paketverwaltung.
REST="$(macho_dateien | while IFS= read -r f; do
    { abhaengigkeiten "$f"; suchpfade "$f"; } | fremd | sed "s|^|${f#"$APP"/}: |"
done)"
[ -z "$REST" ] || { printf '%s\n' "$REST" | head -10; fail "Bündel verweist noch auf $BREW bzw. /usr/local"; }

# ── Lizenzen und Herkunft ────────────────────────────────────────────────────
# Qt steht unter LGPL-3, deren Text die GPL-3 voraussetzt; beide reisen mit
# (SM-OSS-006). Für die übrigen Bibliotheken werden die Lizenzdateien aus
# ihrem Homebrew-Paket übernommen; die Herkunft nennt Formel und Fassung
# (SM-OSS-011).
schritt "lege Lizenzen und Herkunft bei …"
LIZ="$APP/Contents/Resources/Lizenzen"
mkdir -p "$LIZ"
cp LICENSE "$LIZ/GPL-3.0.txt"
cp "$LGPL" "$LIZ/LGPL-3.0.txt"
HERKUNFT="$LIZ/HERKUNFT.txt"
{
    echo "$NAME $VERSION — Entwicklungsbau ($ARCH)"
    echo "Projektlizenz: GPL-3.0-only (GPL-3.0.txt)"
    echo "Qt: LGPL-3.0 (LGPL-3.0.txt), dynamisch gebunden unter Contents/Frameworks"
    echo
    echo "Mitgelieferte Bibliotheken — Bezugsquelle Homebrew (homebrew/core):"
} > "$HERKUNFT"
OHNE_LIZENZ=()
for f in "$FW"/*; do
    base="$(basename "$f")"
    stamm="${base%.framework}"
    # Die Formel ergibt sich aus dem Ort der Originaldatei im Cellar.
    orig="$(find "$BREW/Cellar" -maxdepth 4 -path "*/lib/$base" -print -quit 2>/dev/null)"
    [ -n "$orig" ] || { echo "  $stamm  (Herkunft nicht ermittelbar)" >> "$HERKUNFT"; OHNE_LIZENZ+=("$stamm"); continue; }
    rel="${orig#"$BREW/Cellar/"}"
    formel="${rel%%/*}"; rest="${rel#*/}"; fassung="${rest%%/*}"
    echo "  $stamm  ←  $formel $fassung" >> "$HERKUNFT"
    [ -d "$LIZ/$formel" ] && continue
    mkdir -p "$LIZ/$formel"
    for t in "$BREW/Cellar/$formel/$fassung"/{LICENSE,LICENCE,COPYING,COPYRIGHT,NOTICE,AUTHORS,sbom.spdx.json}*; do
        [ -f "$t" ] && cp "$t" "$LIZ/$formel/"
    done
    [ -n "$(ls -A "$LIZ/$formel")" ] || { rmdir "$LIZ/$formel"; OHNE_LIZENZ+=("$formel"); }
done
[ "${#OHNE_LIZENZ[@]}" = 0 ] || hinweis "ohne mitgelieferte Lizenzdatei: ${OHNE_LIZENZ[*]} — Nachtrag in AP-21 (SM-OSS-008)"

# ── Signatur ─────────────────────────────────────────────────────────────────
# Jedes Umhängen hat die Signatur ungültig gemacht; ohne neue Signatur startet
# das Bündel auf Apple Silicon nicht. Ad hoc genügt dem Bauplatz, eine echte
# Kennung steht unter OP-04.
schritt "signiere ($([ "$SIGNATUR" = - ] && echo 'ad hoc' || echo "$SIGNATUR")) …"
codesign --force --deep -s "$SIGNATUR" "$APP" >/dev/null 2>&1 || fail "codesign"
codesign --verify --deep --strict "$APP" 2>&1 || fail "Signaturprüfung"

# ── Startprobe ───────────────────────────────────────────────────────────────
# Fünf Sekunden Lauf. Ein vorzeitiges Ende ist FAIL, ebenso eine Ladefehlermeldung
# der QML-Maschine: Ohne Hauptfenster läuft die Anwendung weiter, bedienbar
# ist sie nicht.
if [ "$STARTPROBE" = 1 ]; then
    schritt "Startprobe (5 s) …"
    LAUF="$AUSGABE/startprobe.log"
    "$APP/Contents/MacOS/$PROGRAMM" >"$LAUF" 2>&1 &
    pid=$!
    for _ in 1 2 3 4 5 6 7 8 9 10; do
        kill -0 "$pid" 2>/dev/null || break
        sleep 0.5
    done
    if kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null; wait "$pid" 2>/dev/null
    else
        wait "$pid"; rc=$?
        tail -15 "$LAUF"
        fail "Anwendung endete vorzeitig mit $rc (Protokoll: $LAUF)"
    fi
    if grep -Eq 'failed to load component|is not installed|\.qml:[0-9]+' "$LAUF"; then
        grep -E 'failed to load component|is not installed|\.qml:[0-9]+' "$LAUF" | head -10
        fail "QML-Ladefehler bei der Startprobe (Protokoll: $LAUF)"
    fi
else
    hinweis "Startprobe abgeschaltet (--ohne-startprobe)"
fi

# ── Plattenabbild ────────────────────────────────────────────────────────────
# Installiert wird durch Ziehen nach „Programme" — ohne Installer und ohne
# Administratorrechte (SM-PLT-010).
schritt "erzeuge Plattenabbild …"
DMG="$AUSGABE/$NAME-$VERSION-entwicklungsbau-$ARCH.dmg"
STAGE="$(mktemp -d "${TMPDIR:-/tmp}/sm-dmg.XXXXXXXX")" || fail "keine Arbeitsablage anlegbar"
trap 'rm -rf "$STAGE"' EXIT INT TERM
cp -R "$APP" "$STAGE/" || fail "Bündel nicht in die Arbeitsablage kopierbar"
ln -s /Applications "$STAGE/Programme"
rm -f "$DMG" "$DMG.sha256"
hdiutil create -quiet -volname "$NAME" -srcfolder "$STAGE" -format UDZO -ov "$DMG" || fail "hdiutil create"
hdiutil verify -quiet "$DMG" || fail "hdiutil verify"
( cd "$AUSGABE" && shasum -a 256 "$(basename "$DMG")" > "$(basename "$DMG").sha256" ) || fail "Prüfsumme"

echo "build-app: PASS"
echo "  Bündel:  $APP"
echo "  Abbild:  $DMG ($(du -h "$DMG" | cut -f1))"
echo "  SHA-256: $(cut -d' ' -f1 "$DMG.sha256")"
exit 0
