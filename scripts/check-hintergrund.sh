#!/usr/bin/env bash
# Prüft SM-NFR-002 an der laufenden Anwendung: Import, Indizierung und
# Vorschauerzeugung dürfen die Bedienung nicht blockieren.
#
# Der Prüffall misst die Takte eines Zeitgebers im Qt-Faden gegen die
# Wanduhrzeit. Bleibt der Faden stehen, fallen Takte aus. Ohne den Bezug auf
# die Wanduhr wäre die Messung wertlos — ein blockierter Faden verzögert auch
# den Zeitgeber, der den Lauf beendet.
#
# ENTFÄLLT, solange die Anwendung nicht gebaut werden kann.
set -u

WURZEL="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WURZEL" || exit 3

# Die Bezugsmenge steht in SM-LIB-009, SM-NFR-001 und AK-01: **100.000**
# Einträge. Ein Lauf darunter belegt die Zusage nicht — er ist eine
# Kurzprüfung und wird als solche gekennzeichnet, damit sein Ergebnis nicht
# als Abnahme in die Rückverfolgbarkeitsmatrix wandert.
BEZUGSMENGE=100000
ANZAHL="${SM_PRUEFBESTAND_ANZAHL:-$BEZUGSMENGE}"
# Ein nicht-numerischer Wert ließ den Vergleich weiter unten fehlschlagen; ohne
# `set -e` wurde der Zweig übersprungen und das Werkzeug meldete „Bezugsmenge
# erreicht" für einen Lauf, der sie nicht erreicht hatte.
case "$ANZAHL" in
    ''|*[!0-9]*)
        echo "check-hintergrund: FAIL — SM_PRUEFBESTAND_ANZAHL ist keine Zahl: '$ANZAHL'"
        exit 1 ;;
esac
# Ein vorhersagbarer Pfad in einem allgemein beschreibbaren Verzeichnis ist
# angreifbar: Wer ihn vorher anlegt — als Verzeichnis oder als Symlink —,
# bestimmt, wohin dieser Lauf schreibt und was sein `rm -rf` trifft. `mktemp -d`
# erzeugt den Namen zufällig und mit engen Rechten; die Ablage wird beim
# Verlassen in **einem** Zug aufgeräumt, auch bei Abbruch.
ARBEIT="$(mktemp -d "${TMPDIR:-/tmp}/sm-hintergrund.XXXXXXXX")" || {
    echo "check-hintergrund: FAIL — keine Arbeitsablage anlegbar"; exit 1; }
aufraeumen() { rm -rf "$ARBEIT"; }
trap aufraeumen EXIT INT TERM
BESTAND="$ARBEIT/pruefbestand"
# Pfade und Befehle stehen in `.projektregeln.conf`, nicht hier: Dieselbe
# Frage zweimal zu beantworten — einmal in der Zuordnungsdatei, einmal fest im
# Skript — führt unter einem anderen Oberflächenweg zu zwei Wahrheiten.
konf() { sed -n "s/^[[:space:]]*$1[[:space:]]*=[[:space:]]*//p" .messwerkzeug.conf .projektregeln.conf 2>/dev/null | sed 's/[[:space:]]*$//' | head -1; }
# **`timeout` ist nicht überall vorhanden.** Auf macOS — dem Gerät, das der
# Messkopf nennt — fehlt es im Grundsystem. Ohne diese Prüfung endete der Lauf
# mit 127, und das Skript meldete einen Anwendungsabbruch statt eines
# fehlenden Werkzeugs: eine Fehldiagnose, die in die falsche Richtung schickt
# (S3 verlangt FAIL **mit Installationshinweis**).
# Die Ermittlung selbst steht **einmal** im Baum, in scripts/lib/pruefumgebung.sh.
if [ -r "$WURZEL/scripts/lib/pruefumgebung.sh" ]; then
    # shellcheck source=lib/pruefumgebung.sh
    . "$WURZEL/scripts/lib/pruefumgebung.sh"
else
    echo "check-hintergrund: FAIL — scripts/lib/pruefumgebung.sh fehlt (S3)."
    exit 1
fi
kn_zeitgrenze "check-hintergrund" || exit 1
ZEITGRENZE="$KN_ZEITGRENZE"

ANWENDUNG="$(konf anwendung)"

# **Auch `anwendung` ist ein ausführbarer Wert aus der Zuordnungsdatei.** Zwei
# der drei werden gegen Positivliste und Zeichenraster geprüft, dieser wurde
# unmittelbar gestartet — `anwendung = ../../../tmp/x` verließe den Baum.
# SM-SEC-001/002 verlangen Kanonisierung und Eingrenzung auf beiden Seiten.
anwendung_zulaessig() {
    case "$ANWENDUNG" in
        /*|*..*)
            echo "check-hintergrund: FAIL — 'anwendung' muss ein Pfad **im Baum** sein,"
            echo "  ohne führenden Schrägstrich und ohne '..' (SM-SEC-001/002)."
            return 1 ;;
    esac
    return 0
}
anwendung_zulaessig || exit 1
BAU="$(konf bau_befehl)"
ERZEUGE="$(konf pruefbestand_befehl)"

# **Die Zuordnungsdatei trägt ausführbaren Inhalt.** `bau_befehl` und
# `pruefbestand_befehl` sind vollständige Befehlszeilen, die unten unquotiert
# expandiert und ausgeführt werden. Ein zugelieferter Zweig, der die Datei
# ändert, brächte damit beliebigen Code zur Ausführung. Die Datei ist als
# verfolgte Datei prüfpflichtig; **in die Gate-Signatur geht sie nicht ein**,
# weil dieses Werkzeug nicht im Gate läuft. Prüfpflicht ist ohnehin keine
# Schranke. Deshalb: erstes Wort gegen eine Positivliste, Rest gegen ein enges
# Zeichenraster — und für `anwendung` eine Eingrenzung auf den Baum.
befehl_zulaessig() {   # $1 = Beschriftung, $2 = Befehlszeile
    local wort="${2%% *}"
    case "$wort" in
        cargo|make|cmake|ninja|meson|python3) ;;
        *)  echo "check-hintergrund: FAIL — $1 beginnt mit '$wort'; erlaubt sind"
            echo "  cargo, make, cmake, ninja, meson, python3 (.projektregeln.conf)."
            exit 1 ;;
    esac
    case "$2" in
        *[!A-Za-z0-9\ ._/=@:+-]*)
            echo "check-hintergrund: FAIL — $1 enthält unzulässige Zeichen."
            echo "  Erlaubt sind Buchstaben, Ziffern und . _ / = @ : + - (.projektregeln.conf)."
            exit 1 ;;
    esac
}
[ -n "$BAU" ]     && befehl_zulaessig "bau_befehl" "$BAU"
[ -n "$ERZEUGE" ] && befehl_zulaessig "pruefbestand_befehl" "$ERZEUGE"
OBERFLAECHE="$(konf oberflaeche)"

# Rückgabewert 3 heißt ENTFÄLLT — dieselbe Übereinkunft wie bei den
# Schwesterskripten. `exit 0` stünde als PASS im Protokoll, sobald das Skript je
# über `run_gate` liefe: „nicht geprüft" als „bestanden".
if [ -z "$ANWENDUNG" ] || [ -z "$OBERFLAECHE" ] || [ ! -d "$OBERFLAECHE" ]; then
    echo "check-hintergrund: ENTFÄLLT — keine Oberfläche im Baum oder keine Zuordnung"
    exit 3
fi

if [ -z "${QMAKE:-}" ] && command -v brew >/dev/null 2>&1; then
    QT="$(brew --prefix qt 2>/dev/null)"
    [ -x "$QT/bin/qmake" ] && export QMAKE="$QT/bin/qmake"
fi

# Zu jeder Messung gehören Gerät, Datenbestand und Bedingungen
# (Lastenheft Abschnitt 13.2). Ohne sie ist die Zahl in der
# Rückverfolgbarkeitsmatrix nicht einzuordnen.
QT_FASSUNG="$( "${QMAKE:-qmake}" -query QT_VERSION 2>/dev/null || echo "unbekannt" )"
echo "Messbedingungen"
echo "  Datum:        $(date '+%Y-%m-%d %H:%M %Z')"
echo "  Gerät:        $(uname -sm)"
echo "  Qt-Fassung:   $QT_FASSUNG"
echo "  Plattform:    offscreen (QT_QPA_PLATFORM)"
echo "  Prüfbestand:  $ANZAHL Dateien von $BEZUGSMENGE (Bezugsmenge)"
# **Ohne das Bauprofil ist die Zahl nicht einzuordnen.** Ein Debug-Bau läuft um
# ein Vielfaches langsamer als das ausgelieferte Erzeugnis; eine daraus
# abgeleitete Aussage über 100.000 Einträge trägt nicht.
case "$BAU" in
    *--release*) PROFIL="release" ;;
    *)           PROFIL="debug (nicht übertragbar auf das Erzeugnis)" ;;
esac
echo "  Bauprofil:    $PROFIL"
if ! $ERZEUGE -- "$BESTAND" "$ANZAHL" >/dev/null 2>&1; then
    echo "check-hintergrund: FAIL — der Prüfbestand ließ sich nicht erzeugen"
    exit 1
fi

if ! $BAU >/dev/null 2>&1; then
    echo "check-hintergrund: FAIL — die Anwendung ließ sich nicht bauen"
    exit 1
fi

# Der Zwischenspeicher liegt für den Prüflauf neben dem Prüfbestand — nie im
# dauerhaften Speicher der Nutzerin. Er wird geleert, damit die Vorschauen
# tatsächlich entstehen und nicht aus einem früheren Lauf stammen.
ABLAGE="$ARBEIT/vorschau"
DATEN="$ARBEIT/daten"
export SM_VORSCHAU_ABLAGE="$ABLAGE"
# Auch die Datenhaltung liegt für den Prüflauf beiseite — nie im Bestand der
# Nutzerin.
export SM_DATENABLAGE="$DATEN"

# Der Rückgabewert der Anwendung wird getrennt festgehalten: Ein **Absturz**
# ist etwas anderes als ein durchgefallener Lauf, und ein leerer Filter darf
# nicht wie „kein Ergebnis, also weiter" aussehen (S3).
#
# **Die Zuweisung darf nicht in einer Subshell stehen.** Zuvor lief die
# Funktion als `AUSGABE="$(lauf)"`; eine Kommandosubstitution ist eine
# Subshell, und das dort gesetzte `LAUF_RC` erreichte die Elternshell nie. Es
# blieb auf dem Anfangswert 0 — die Bedingung darunter war damit konstant
# falsch, und die Meldung nannte bei *jedem* Abbruch wörtlich „Rückgabewert 0".
# Ein Lauf, der seine Zeile schreibt und danach abstürzt oder in die
# Zeitüberschreitung läuft, galt als bestanden.
#
# Die Rohausgabe geht deshalb in eine Datei, und die Funktion gibt den echten
# Rückgabewert zurück. Damit bleibt auch die Fehlerausgabe erhalten: Zuvor warf
# `grep SELBSTTEST` sie weg, und wer das Werkzeug fuhr, stand ohne Diagnose da.
lauf() {
    # Der Pfad ist oben auf den Baum eingegrenzt; hier wird zusätzlich geprüft,
    # dass er auf eine reguläre, ausführbare Datei zeigt — ein Verzeichnis oder
    # ein Symlink nach außen käme sonst zur Ausführung.
    if [ ! -f "$WURZEL/$ANWENDUNG" ] || [ ! -x "$WURZEL/$ANWENDUNG" ]; then
        echo "check-hintergrund: FAIL — '$ANWENDUNG' ist keine ausführbare Datei im Baum."
        return 1
    fi
    QT_QPA_PLATFORM=offscreen SM_SELBSTTEST="$BESTAND" \
        "$ZEITGRENZE" 120 "$WURZEL/$ANWENDUNG" > "$ARBEIT/roh.txt" 2>&1
}

lauf; LAUF_RC=$?
AUSGABE="$(grep SELBSTTEST "$ARBEIT/roh.txt" || true)"
echo "$AUSGABE" | sed 's/^/  /'

if [ "$LAUF_RC" -ne 0 ] || [ -z "$AUSGABE" ]; then
    echo "check-hintergrund: FAIL — die Anwendung endete mit Rückgabewert $LAUF_RC"
    echo "  ohne verwertbare Selbsttestzeile. Das ist ein Abbruch, kein bestandener Lauf."
    echo "  Letzte Ausgabezeilen:"
    tail -20 "$ARBEIT/roh.txt" | sed 's/^/    /'
    exit 1
fi

if ! echo "$AUSGABE" | grep -q "ERGEBNIS=PASS"; then
    echo "check-hintergrund: FAIL — SM-NFR-002 nicht erfüllt"
    exit 1
fi

# --- SM-PRV-002: Der Zwischenspeicher übersteht das Programmende ---
ABGELEGT="$(find "$ABLAGE" -name '*.png' 2>/dev/null | wc -l | tr -d ' ')"
if [ "$ABGELEGT" -eq 0 ]; then
    echo "check-hintergrund: FAIL — es wurde keine Vorschau abgelegt (SM-PRV-002)"
    exit 1
fi
echo "  abgelegte Vorschauen nach Lauf 1: $ABGELEGT"

# Marke setzen: Kommt sie nach dem zweiten Lauf unverändert zurück, stammte die
# Vorschau aus dem Zwischenspeicher und wurde nicht neu gezeichnet.
MARKIERT="$(find "$ABLAGE" -name '*.png' | head -1)"
# `shasum` ist nicht auf allen Zielplattformen vorhanden (SM-PLT).
if command -v shasum >/dev/null 2>&1; then PS=shasum
elif command -v sha1sum >/dev/null 2>&1; then PS=sha1sum
else
    echo "check-hintergrund: FAIL — weder 'shasum' noch 'sha1sum' vorhanden."
    exit 1
fi
PRUEFSUMME_VORHER="$($PS "$MARKIERT" | awk '{print $1}')"
printf 'MARKE' > "$MARKIERT"

lauf; LAUF2_RC=$?
AUSGABE2="$(grep SELBSTTEST "$ARBEIT/roh.txt" || true)"
if [ "$LAUF2_RC" -ne 0 ] || [ -z "$AUSGABE2" ]; then
    echo "check-hintergrund: FAIL — der zweite Lauf endete mit Rückgabewert $LAUF2_RC"
    tail -20 "$ARBEIT/roh.txt" | sed 's/^/    /'
    exit 1
fi
echo "$AUSGABE2" | sed 's/^/  /'

if [ "$(cat "$MARKIERT")" != "MARKE" ]; then
    echo "check-hintergrund: FAIL — die Vorschau wurde neu gezeichnet statt wiederverwendet (SM-PRV-002)"
    exit 1
fi
echo "  Lauf 2 nutzte den Zwischenspeicher (Prüfsumme vorher $PRUEFSUMME_VORHER, Marke überlebte)"


if ! echo "$AUSGABE2" | grep -q "ERGEBNIS=PASS"; then
    echo "check-hintergrund: FAIL — der zweite Lauf schlug fehl"
    exit 1
fi

# --- SM-IMP-003: Der zweite Lauf verarbeitet nur, was sich geändert hat ---
if ! echo "$AUSGABE2" | grep -q "Bestand ist aktuell"; then
    echo "check-hintergrund: FAIL — der zweite Lauf war nicht inkrementell (SM-IMP-003)"
    exit 1
fi
echo "  Lauf 2 war inkrementell: nichts neu zu lesen"

if [ "$ANZAHL" -lt "$BEZUGSMENGE" ]; then
    echo "check-hintergrund: KURZPRÜFUNG BESTANDEN — $ANZAHL von $BEZUGSMENGE Einträgen"
    echo "  Das ist **keine Abnahme**: SM-LIB-009, SM-NFR-001 und AK-01 beziehen sich auf"
    echo "  $BEZUGSMENGE Einträge. Für den Nachweis je Veröffentlichung ohne"
    echo "  SM_PRUEFBESTAND_ANZAHL laufen lassen."
    # **Rückgabewert 4 = Kurzprüfung, keine Abnahme.** Zuvor stand hier `exit 0`
    # und die Unterscheidung lebte allein im deutschen Fließtext — maschinell
    # nicht auswertbar. Ein Aufrufer, der 0 als „bestanden" liest, trüge das
    # Ergebnis als Abnahme in die Rückverfolgbarkeitsmatrix.
    exit 4
fi
echo "check-hintergrund: PASS — Bedienung frei, Speicher dauerhaft, Lauf 2 inkrementell"
echo "  Bezugsmenge $BEZUGSMENGE erreicht; das Ergebnis trägt SM-NFR-002."
# **Nicht AK-01.** Das Kriterium verlangt zusätzlich eine *durchsuchbare*
# Bibliothek, und dieser Lauf setzt keine einzige Suche ab. Die Zusage stand
# hier, ohne dass die zweite Hälfte je gemessen worden wäre.
echo "  AK-01 ist damit **nicht** belegt: Die Durchsuchbarkeit misst dieser Lauf nicht."
exit 0
