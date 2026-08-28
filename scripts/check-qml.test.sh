#!/usr/bin/env bash
# Selbsttest der QML-Prüfung.
#
# Je Prüfbedingung ein **Negativfall**: Ein Prüfer ohne Negativfall degradiert
# unbemerkt zum No-Op und meldet dabei weiter grün. Dazu je ein Positivfall,
# damit die Prüfung nicht einfach alles rot färbt.
set -u

WURZEL="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Zeitgrenze und ihre Begründung stehen **einmal** im Baum.
if [ -r "$WURZEL/scripts/lib/pruefumgebung.sh" ]; then
    # shellcheck source=lib/pruefumgebung.sh
    . "$WURZEL/scripts/lib/pruefumgebung.sh"
else
    echo "Selbsttest QML: FAIL — scripts/lib/pruefumgebung.sh fehlt (S3)."
    exit 1
fi
kn_zeitgrenze "Selbsttest QML" || exit 1
ZEITGRENZE="$KN_ZEITGRENZE"

TMPWURZEL="$(mktemp -d)"
trap 'rm -rf "$TMPWURZEL"' EXIT

# **Die Prüfbäume dürfen keinen Elternbaum sehen.** Läge `$TMPDIR` innerhalb
# eines Git-Arbeitsbaums, fände `git rev-parse` in Fall A4 den Elternbaum,
# `kn_dateien` gelänge, und der Fall schlüge ohne Sachgrund fehl.
export GIT_CEILING_DIRECTORIES="$TMPWURZEL"

# ── Schattenpfad: ein Suchpfad ohne einzelne Werkzeuge ──────────────────────
#
# **Die Abwesenheit wird hergestellt, nicht vorausgesetzt.** Ein früherer Stand
# verengte dafür den Suchpfad auf `/usr/bin:/bin:…` und nahm an, Qt liege nicht
# dort. Auf jeder Ablage, die `qmllint` dorthin legt, war der Fall nicht
# darstellbar, wurde aber als Fehlschlag gezählt — Stufe 0b rot, jeder Commit
# gesperrt, einziger Ausweg ein Notfall-Ausstieg als Dauerzustand (S1).
#
# **Der Spiegel trägt den vollständigen Suchpfad**, aus dem einzelne Werkzeuge
# entfernt werden — keine gepflegte Positivliste. Eine ausgeschriebene
# Werkzeugliste ist eine zweite Antwort auf „was braucht der Prüfling?"; sie
# altert, und ein fehlender Eintrag ließe einen Fall am Prüfling scheitern statt
# an der Sache, die er belegen soll.
#
# **Und er entsteht einmal, nicht je Fall.** Je Fall neu angelegt kostete er
# mehr Laufzeit als der gesamte übrige Selbsttest — und Stufe 0b ist nicht
# änderungsbezogen, zahlt also auch eine Markdown-Korrektur. Die Fälle laufen
# nacheinander: Ein Werkzeug wird vor dem Fall entfernt und danach zurückgelegt.
SCHATTEN=""
SPIEGEL=""
schatten_bereitstellen() {
    if [ -n "$SPIEGEL" ]; then SCHATTEN="$SPIEGEL"; return 0; fi
    SPIEGEL="$TMPWURZEL/pfadspiegel"
    SCHATTEN="$SPIEGEL"
    mkdir -p "$SCHATTEN"
    # **Ein `ln`-Aufruf je Verzeichnis, nicht je Datei.** Der Suchpfad trägt auf
    # einem Entwicklungsgerät regelmäßig vierstellig viele Programme; je Datei
    # ein Prozess kostete mehr als der gesamte übrige Selbsttest. `ln` **ohne**
    # `-f` überschreibt nichts — die Vorrangfolge des Suchpfads bleibt damit
    # gewahrt, der erste Treffer gewinnt wie beim echten Aufruf.
    local verz
    local altes_ifs="$IFS"; IFS=:
    for verz in $PATH; do
        IFS="$altes_ifs"
        # `ln` meldet für jeden bereits vorhandenen Namen einen Fehler — das ist
        # der Regelfall und gewollt (der erste Treffer des Suchpfads gewinnt).
        # Eine Bewertung je Meldung wäre deshalb nicht aussagekräftig; geprüft
        # wird stattdessen das Ergebnis, unten über den leeren Spiegel.
        [ -d "$verz" ] && ln -s "$verz"/* "$SCHATTEN"/ 2>/dev/null
        IFS=:
    done
    IFS="$altes_ifs"
    # Ein leerer Spiegel hieße: Kein Fall, der ihn benutzt, prüft noch etwas.
    if [ -z "$(ls -A "$SCHATTEN" 2>/dev/null)" ]; then
        echo "Selbsttest QML: FAIL — der Suchpfadspiegel blieb leer."
        echo "  Ohne ihn stellen die Fälle B1, B2, E4, E5, F1 und F2 nichts nach."
        exit 1
    fi
    return 0
}

# Entfernt die genannten Werkzeuge aus dem Spiegel.
#
# **Sie gibt den Pfad nicht aus, sondern setzt `SCHATTEN`.** Ein Aufruf in
# einer Kommandoersetzung liefe in einer Unterschale; die Merkliste `ENTFERNT`
# ginge dort verloren, und `schatten_zuruecksetzen` legte nichts zurück — die
# Attrappe eines Falls bliebe im Spiegel und verfälschte den nächsten. Genau
# der Unterschalen-Fehler, den `scripts/lib/dateien.sh` für sich selbst
# vermeidet.
schattenpfad() {  # $1… = Werkzeuge, die fehlen sollen
    # **Kein Spiegel, wo es nichts zu verbergen gibt.** Liegt keines der
    # genannten Werkzeuge auf dem Suchpfad, stellt der unveränderte Pfad
    # denselben Fall dar — und der Spiegel kostet auf einem Gerät ohne Qt
    # Laufzeit für zwanzig entfallende Fälle. Stufe 0b ist nicht
    # änderungsbezogen; die Kosten zahlt jeder Commit.
    local w vorhanden=0
    for w in "$@"; do
        command -v "$w" >/dev/null 2>&1 && vorhanden=1
    done
    if [ "$vorhanden" = 0 ]; then
        # Der unveränderte Suchpfad **ist** hier der Fall: Was fehlen soll,
        # fehlt bereits. `SCHATTEN` trägt dann den Pfad selbst, nicht ein
        # Verzeichnis — die Aufrufer setzen ihn als `PATH`.
        SCHATTEN="$PATH"
        ENTFERNT=""
        return 0
    fi
    schatten_bereitstellen
    ENTFERNT=""
    for w in "$@"; do
        rm -f "$SCHATTEN/$w"
        ENTFERNT="$ENTFERNT $w"
    done
}

# Legt zurück, was `schattenpfad` entfernt hat — samt etwaiger Attrappen.
schatten_zuruecksetzen() {
    local w q
    for w in $ENTFERNT; do
        rm -f "$SCHATTEN/$w"
        q="$(command -v "$w" 2>/dev/null)" || continue
        [ -n "$q" ] && ln -sf "$q" "$SCHATTEN/$w"
    done
    ENTFERNT=""
}
ENTFERNT=""

bestanden=0
fehlgeschlagen=0

# Baut einen Prüfbaum mit dem Prüfling.
#
# **Das Verzeichnis kommt aus `mktemp`, nicht aus einem Zähler.** Ein Zähler
# stiege in dieser Kommandoersetzung nie: Sie läuft in einer eigenen Schale.
# Alle Fälle teilten dann ein Verzeichnis, sammelten die QML-Dateien der
# Vorgänger ein — und ein Fall bewiese nicht mehr, was er behauptet.
#
# **Und es ist ein Git-Baum**: Der Prüfling bildet seine Dateiliste über
# `git ls-files`, damit `.gitignore` gilt. Dasselbe Muster wie in
# scripts/check-projektregeln.test.sh.
baue() {
    local d
    d="$(mktemp -d "$TMPWURZEL/fallXXXXXX")"
    mkdir -p "$d/scripts/lib" "$d/crates/ui/qml"
    cp "$WURZEL/scripts/check-qml.sh" "$d/scripts/"
    # Die gemeinsame Dateiliste gehört mit: Ohne sie meldete jeder Fall FAIL
    # wegen der fehlenden Bibliothek statt der Sache, um die es ihm geht.
    cp "$WURZEL/scripts/lib/dateien.sh" "$d/scripts/lib/"
    git -C "$d" init -q >/dev/null 2>&1
    git -C "$d" config user.email pruef@beispiel.invalid >/dev/null 2>&1
    git -C "$d" config user.name Pruefung >/dev/null 2>&1
    printf '%s' "$d"
}

pruefe() {  # $1 = Name, $2 = erwarteter Rückgabewert, $3 = Verzeichnis, $4… = erwarteter Text
    local name="$1" erwartet="$2" d="$3"; shift 3
    local out rc ok=1 muster
    out="$(cd "$d" && PATH="${PRUEFPFAD:-$PATH}" "$ZEITGRENZE" 120 bash "$d/scripts/check-qml.sh" 2>&1)"
    rc=$?
    [ "$rc" = "$erwartet" ] || ok=0
    for muster in "$@"; do
        case "$out" in *"$muster"*) ;; *) ok=0 ;; esac
    done
    if [ "$ok" = 1 ]; then
        bestanden=$((bestanden + 1)); echo "  ✓ $name"
    else
        fehlgeschlagen=$((fehlgeschlagen + 1))
        echo "  ✗ $name"
        echo "     erwarteter Rückgabewert: $erwartet · erhalten: $rc"
        printf '%s\n' "$out" | sed 's/^/     /'
    fi
}

# ── Anwendbarkeit — je Fallgruppe, nicht global ─────────────────────────────
#
# **Zwei Sackgassen, beide erlebt, beide hier ausgeschlossen.**
#
# Die erste: Ein früherer Stand brach mit FAIL ab, sobald ein Qt-Werkzeug
# fehlte, und begründete das mit „Der Gegenstand liegt im Baum" — behauptet,
# nicht geprüft. Ein Klon eines Zwischenstands ohne Oberflächenschicht trägt
# kein `*.qml`; der Prüfling meldete dort ENTFÄLLT, sein Selbsttest FAIL.
#
# Die zweite: Sobald `*.qml` **doch** im Baum liegt, sperrte derselbe Abbruch
# auf jedem Gerät ohne Qt **jeden** Commit — auch die Korrektur einer Zeile im
# Lastenheft. Denn **Stufe 0b ist nicht änderungsbezogen**, anders als Stufe 0c,
# die für dieselbe Änderung „0c QML — ENTFÄLLT, vom Änderungssatz nicht
# betroffen" protokollierte. Einziger Ausweg wäre der Notfall-Ausstieg als
# Dauerzustand (S1).
#
# Beides löst dieselbe Unterscheidung: **Der Gegenstand dieses Selbsttests ist
# die Verdrahtung des Prüflings, nicht die Qt-Werkzeugkette.** Ein großer Teil
# davon ist ohne Qt prüfbar — die Anwendbarkeitszweige laufen vor der
# Werkzeugabfrage, und das Fehlen eines Werkzeugs stellen B1/B2 selbst her.
# Diese Fälle laufen immer. Nur die Fälle, die eine echte Prüfung brauchen,
# entfallen namentlich und werden gezählt.
#
# **Der Baum spielt für diese Entscheidung keine Rolle**, und das ist kein
# Versehen: Der Selbsttest baut seine Prüfbäume selbst. Was er nicht selbst
# herstellen kann, ist eine vorhandene Qt-Werkzeugkette — und nur daran hängt
# die Fallgruppe. Ein früherer Stand fragte hier zusätzlich den Baum ab und
# beantwortete die Frage mit einer Wortkopie des Bibliotheksidioms; die Kopie
# ist ersatzlos entfallen, weil die Frage selbst nicht gebraucht wird.
FEHLENDE=""
for w in qmllint qmlformat; do
    command -v "$w" >/dev/null 2>&1 || FEHLENDE="$FEHLENDE $w"
done
QMLLINT_DA=0; command -v qmllint  >/dev/null 2>&1 && QMLLINT_DA=1
QT_DA=0;      [ -z "$FEHLENDE" ]                  && QT_DA=1

entfallen=0
entfaellt() {  # $1 = Name, $2 = Grund
    entfallen=$((entfallen + 1))
    echo "  – $1 — entfällt ($2)"
}

# ── Fallnamen stehen einmal ─────────────────────────────────────────────────
#
# Der Entfallzweig zählt die Qt-abhängigen Fälle namentlich auf. Stünden die
# Namen dort ein zweites Mal ausgeschrieben, liefen sie auseinander — sie taten
# es bereits —, und ein neuer Fall im Prüfzweig fiele auf einem Gerät ohne Qt
# lautlos aus der Zählung. Wer einen Fall hinzufügt, legt hier seinen Namen an
# und benutzt ihn an **beiden** Stellen.
F_A3="A3 *.qml außerhalb der Oberflächenschicht wird mitgeprüft"
F_B2="B2 Gegenstand da, qmlformat fehlt — FAIL mit Bezugshinweis"
F_C1="C1 sauberes QML besteht"
F_C2="C2 Pfad mit Leerzeichen besteht"
F_C3="C3 Umlautpfad besteht (SM-NFR-010)"
F_C4="C4 Dateiname mit führendem Bindestrich besteht"
F_D1="D1 unqualifizierter Zugriff blockiert"
F_D2="D2 Befund nennt die Quellzeile"
F_D3="D3 Modulhinweis rät mit Cargo.toml zu cargo build"
F_D4="D4 Modulhinweis bleibt ohne Cargo.toml wegneutral"
F_E1="E1 echter Import-Befund blockiert trotz der cxx-qt-Ausnahme"
F_E2="E2 die Brückenmeldung wird geschluckt, der Lauf besteht"
F_E2b="E2b ein abgeleiteter Brückentypname wird geschluckt"
F_E2c="E2c ein fremder Typname derselben Form blockiert"
F_E3="E3 echter unauflösbarer Eigenschaftstyp blockiert"
F_E4="E4 je Modulname gilt das jüngste Verzeichnis"
F_E5="E5 verschiedene Module bleiben beide auf dem Importpfad"
F_F1="F1 stiller Abbruch des Werkzeugs blockiert"
F_F2="F2 Abbruch von qmlformat wird nicht als Formatbefund gemeldet"
F_G1="G1 abweichende Einrückung blockiert"
F_G2="G2 Reihenfolge der Eigenschaften ist kein Befund (kein -n)"

echo "Selbsttest QML"
if [ "$QT_DA" = 0 ]; then
    echo "  Hinweis:$FEHLENDE fehlt — die Qt-abhängigen Fälle entfallen namentlich."
    echo "  Bezugsweg: 'brew install qt' oder 'apt install qt6-declarative-dev'."
fi
if command -v qmllint >/dev/null 2>&1; then
    echo "  Geprüft gegen $(qmllint --version 2>&1 | tr -d '\n')."
fi

# --- A · Anwendbarkeit (S3) ------------------------------------------------
# Ohne Gegenstand ist die Prüfung ENTFÄLLT, nie PASS. Ein PASS hier hieße:
# „QML geprüft" — ohne eine einzige Datei gesehen zu haben.
d="$(baue)"
pruefe "A1 kein *.qml im Baum ergibt ENTFÄLLT, nicht PASS" 3 "$d" \
    "ENTFÄLLT"

# Eine ignorierte Datei ist kein Gegenstand: Sonst blockierte ein erzeugtes
# QML unterhalb von target/ ohne begehbaren Weg (S1).
d="$(baue)"
printf 'target/\n' > "$d/.gitignore"
mkdir -p "$d/target"
printf 'import QtQuick\nItem {\n    Item {\n        width: nichts\n    }\n}\n' \
    > "$d/target/Erzeugt.qml"
pruefe "A2 ignorierte *.qml ist kein Gegenstand" 3 "$d" \
    "ENTFÄLLT"

# Der Gegenstand ist der **Baum**, nicht die Oberflächenschicht: Eine QML-Datei
# außerhalb von crates/ui muss mitgeprüft werden, sonst meldete das Gate PASS
# über eine Datei, die es nie gesehen hat.
# **Braucht die ganze Werkzeugkette.** Der Fall erwartet einen echten Befund,
# nicht den Werkzeughinweis — und der Prüfling prüft **beide** Werkzeuge vorab.
# Auf einem Gerät mit `qmllint`, aber ohne `qmlformat` — die Lage, die Fall B2
# eigens herstellt — schlüge er sonst ohne Sachgrund fehl, und Stufe 0b wäre
# dauerhaft rot (S1).
if [ "$QT_DA" = 1 ]; then
d="$(baue)"
mkdir -p "$d/woanders"
printf 'import QtQuick\nItem {\n    Item {\n        width: nichts\n    }\n}\n' \
    > "$d/woanders/Abseits.qml"
pruefe "$F_A3" 1 "$d" \
    "unqualified" "Befund"
else
    entfaellt "$F_A3" "Qt-Werkzeugkette fehlt"
fi

# Ein gescheiterter Aufbau der Dateiliste ist FAIL, nicht ENTFÄLLT: Sonst
# meldete ein fehlender Git-Arbeitsbaum „kein *.qml im Baum", obwohl der Baum
# voll davon ist — ein nicht durchgeführter Test als „nicht anwendbar" (S3).
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
rm -rf "$d/.git"
pruefe "A4 kein Git-Arbeitsbaum ergibt FAIL, nicht ENTFÄLLT" 1 "$d" \
    "kein Git-Arbeitsbaum" "kein bestandener Test"

# Fehlt die gemeinsame Bibliothek, ist der Prüfbereich nicht bildbar. Auch das
# ist FAIL, nicht ENTFÄLLT — ein Prüfer ohne Bereich hat nichts geprüft.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
rm -f "$d/scripts/lib/dateien.sh"
pruefe "A5 fehlende Bibliothek ergibt FAIL, nicht ENTFÄLLT" 1 "$d" \
    "dateien.sh fehlt"

# --- B · Fehlendes Werkzeug bei vorhandenem Gegenstand ist FAIL (S3) -------
d="$(baue)"
# Formatgerecht nach qmlformat, damit der Fall nicht an der Formatprüfung
# scheitert statt an dem, was er beweisen soll.
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
schattenpfad qmllint
PRUEFPFAD="$SCHATTEN" \
pruefe "B1 Gegenstand da, qmllint fehlt — FAIL mit Bezugshinweis" 1 "$d" \
    "qmllint fehlt" "Qt 6" "nicht ENTFÄLLT"
schatten_zuruecksetzen
unset PRUEFPFAD

# Dieselbe Zusage für das zweite Werkzeug. Ohne diesen Fall scheiterten auf
# einem System mit qmllint, aber ohne qmlformat die Formatfälle am Prüfling,
# und die Ausgabe zeigte auf ihn statt auf die fehlende Fassung.
# **B2 braucht ein vorhandenes qmllint.** Der Prüfling meldet das *erste*
# fehlende Werkzeug; ohne qmllint bewiese der Fall nicht, was er behauptet.
if [ "$QMLLINT_DA" = 1 ]; then
    d="$(baue)"
    # Formatgerecht nach qmlformat, damit der Fall nicht an der Formatprüfung
    # scheitert statt an dem, was er beweisen soll.
    printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
    schattenpfad qmlformat
    PRUEFPFAD="$SCHATTEN" \
    pruefe "$F_B2" 1 "$d" \
        "qmlformat fehlt" "Qt 6"
    schatten_zuruecksetzen
    unset PRUEFPFAD
else
    entfaellt "$F_B2" "qmllint fehlt ebenfalls"
fi

# ── Ab hier braucht jeder Fall eine echte Qt-Werkzeugkette ──────────────────
if [ "$QT_DA" = 0 ]; then
    # **Die Liste wird abgeleitet, nicht gepflegt.** Eine ausgeschriebene
    # Zweitliste der Fallnamen lief bereits zweimal auseinander: Ein neuer Fall
    # im Zweig darunter fiel auf einem Gerät ohne Qt lautlos aus der Zählung.
    # Gelesen werden deshalb die `pruefe`-Aufrufe des Zweigs selbst; `${!v}`
    # löst den Namen zum Fallnamen auf.
    while IFS= read -r v; do
        [ -n "$v" ] || continue
        entfaellt "${!v}" "Qt-Werkzeugkette fehlt"
    done < <(awk '/^# ── Ab hier braucht jeder Fall/,0' "$0" \
               | grep -oE 'pruefe "\$F_[A-Za-z0-9_]+"' \
               | sed 's/.*\$//; s/"//')
else
# --- C · Positivfall -------------------------------------------------------
d="$(baue)"
cat > "$d/crates/ui/qml/Sauber.qml" <<'QML'
import QtQuick

Item {
    id: sauber

    property int breite: 10

    width: sauber.breite
}
QML
# **Das Muster wird formatiert, nicht als formatgerecht angenommen.** Sonst
# färbte eine Qt-Aktualisierung Stufe 0b rot, ohne dass sich eine Zeile im Baum
# geändert hätte — dieselbe Fehlerrichtung, die MD060 vorgeführt hat und
# derentwegen die Linter-Fassung gebunden ist (CLAUDE.md Abschnitt 11). G1
# bleibt davon unberührt: Der Fall verstellt die Einrückung mit Absicht.
qmlformat -i "$d/crates/ui/qml/Sauber.qml" >/dev/null 2>&1
pruefe "$F_C1" 0 "$d" "qmllint ohne Befund" "qmlformat ohne Abweichung"

# Ein Pfad mit Leerzeichen darf nichts verschieben (SM-NFR-010): Der Prüfling
# reicht Dateinamen an zwei Werkzeuge und an `diff` weiter.
d="$(baue)"
mkdir -p "$d/crates/ui/qml/mit Leerzeichen"
cat > "$d/crates/ui/qml/mit Leerzeichen/Sauber.qml" <<'QML'
import QtQuick

Item {
    id: sauber

    property int breite: 10

    width: sauber.breite
}
QML
qmlformat -i "$d/crates/ui/qml/mit Leerzeichen/Sauber.qml" >/dev/null 2>&1
pruefe "$F_C2" 0 "$d" "qmllint ohne Befund"

# Und ein Nicht-ASCII-Pfad (SM-NFR-010). Ohne `core.quotePath=false` in
# scripts/lib/dateien.sh liefert git den Namen mit Oktalfluchten in
# Anführungszeichen; der Prüfling reichte dann einen Pfad an qmllint weiter,
# den es nicht gibt, und der Befund zeigte auf die falsche Stelle. Genau diese
# Wirkung belegt der Fall — `review-gate.test.sh` führt mit H10 den Vorläufer.
d="$(baue)"
cat > "$d/crates/ui/qml/Größenübersicht.qml" <<'QML'
import QtQuick

Item {
    id: groessen

    property int breite: 10

    width: groessen.breite
}
QML
qmlformat -i "$d/crates/ui/qml/Größenübersicht.qml" >/dev/null 2>&1
pruefe "$F_C3" 0 "$d" "1 Datei(en)" "qmllint ohne Befund"

# C4 — ein Dateiname mit führendem Bindestrich. Die Liste stammt aus dem Baum,
# also aus Fremddaten; ohne `--` käme der Name als Schalter an, und das Werkzeug
# meldete etwas anderes als einen Befund an der Datei.
d="$(baue)"
cat > "$d/crates/ui/qml/-Sonderling.qml" <<'QML'
import QtQuick

Item {
    id: sonderling

    property int breite: 10

    width: sonderling.breite
}
QML
qmlformat -i -- "$d/crates/ui/qml/-Sonderling.qml" >/dev/null 2>&1
pruefe "$F_C4" 0 "$d" "1 Datei(en)" "qmllint ohne Befund" "qmlformat ohne Abweichung"

# --- D · Negativfall: unqualifizierter Zugriff ------------------------------
d="$(baue)"
cat > "$d/crates/ui/qml/Unsauber.qml" <<'QML'
import QtQuick

Item {
    id: unsauber

    property int breite: 10

    Item {
        // Greift auf die Eigenschaft des **umgebenden** Elements zu, ohne sie
        // über dessen Bezeichner zu benennen. Genau das meldet qmllint.
        width: breite
    }
}
QML
# Der Prüfbaum trägt kein erzeugtes QML-Modul; der Hinweis darauf muss
# erscheinen — und zwar auch in der Schlusszeile, weil das Protokoll nur die
# letzten Ausgabezeilen übernimmt. Zweimal als Befund entstanden, bis hierher
# ohne Prüffall.
pruefe "$F_D1" 1 "$d" \
    "unqualified" "Befund" "Kein erzeugtes QML-Modul" "Hinweis: kein erzeugtes QML-Modul"

# Der Befund trägt seinen Kontext: nur `datei:zeile:spalte` zwänge den Leser,
# die Stelle selbst aufzuschlagen.
pruefe "$F_D2" 1 "$d" "width: breite"

# D3/D4 — der Modulhinweis kennt zwei Fassungen. Mit `Cargo.toml` rät er zu
# `cargo build`; ohne — der unter Weg B vorgesehene Zustand — wäre dieser Rat
# ein Weg, den es nicht gibt (S1). Beide Hälften brauchen ihren Fall, sonst
# fiele die Unterscheidung beim nächsten Umbau lautlos weg.
d="$(baue)"
printf 'import QtQuick\nItem {\n    Item {\n        width: nichts\n    }\n}\n' \
    > "$d/crates/ui/qml/Unsauber.qml"
printf '[workspace]\n' > "$d/Cargo.toml"
pruefe "$F_D3" 1 "$d" "Zuerst 'cargo build' ausführen"

d="$(baue)"
printf 'import QtQuick\nItem {\n    Item {\n        width: nichts\n    }\n}\n' \
    > "$d/crates/ui/qml/Unsauber.qml"
pruefe "$F_D4" 1 "$d" "Ohne die Typbeschreibung der Oberflächenbrücke"

# --- E · Die cxx-qt-Ausnahme: eng, aber wirksam -----------------------------
# E1 — sie darf **nur** die Meldungen der Brücke schlucken. Eine wirklich
# fehlende Importzeile trägt dieselbe Art `[import]` und muss weiter
# blockieren, sonst wäre aus der begründeten Ausnahme eine abgeschaltete
# Prüfart geworden.
d="$(baue)"
cat > "$d/crates/ui/qml/Fremd.qml" <<'QML'
import QtQuick

Item {
    GibtEsNicht {
    }
}
QML
pruefe "$F_E1" 1 "$d" \
    "Befund"

# E2 — der Gegenbeweis. E1 zeigt nur, dass die Ausnahme **eng** ist; ohne
# diesen Fall wäre nicht belegt, dass sie überhaupt **wirkt**. Ein Prüfling,
# der die Brückenmeldung nicht mehr filterte, bliebe hier unauffällig: Der
# echte qmllint erzeugt sie nur an einer cxx-qt-Brücke, die im Prüfbaum nicht
# steht. Deshalb wird sie hier nachgestellt.
d="$(baue)"
# Formatgerecht nach qmlformat, damit der Fall nicht an der Formatprüfung
# scheitert statt an dem, was er beweisen soll.
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
schattenpfad qmllint
cat > "$SCHATTEN/qmllint" <<'ATTRAPPE'
#!/bin/sh
# Nur die Meldung der cxx-qt-Brücke, mit dem Rückgabewert, den qmllint dafür
# setzt. Sie darf den Lauf nicht blockieren — die Typen prüft der Übersetzer.
echo "Warning: q.qml:1:1: ::rust::cxxqt1::CxxQtThreading<X> was not found. [import]"
exit 1
ATTRAPPE
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_E2" 0 "$d" \
    "qmllint ohne Befund"
schatten_zuruecksetzen
unset PRUEFPFAD

# E2b — die **zweite** Hälfte der Ausnahme: der aus `plugin.qmltypes`
# abgeleitete Brückentypname. E2 belegt nur die `::`-Hälfte; bräche die
# Ableitung (geändertes Format von `plugin.qmltypes`), bliebe Stufe 0b grün,
# während Stufe 0c mit Falschbefunden jeden Codecommit sperrte — der Nutzer
# suchte dann am QML statt an der Ableitung.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
m="$d/target/bau/qt-build-utils/qml_modules/de/probe"
mkdir -p "$m"
printf 'module de.probe\ntypeinfo plugin.qmltypes\n' > "$m/qmldir"
printf 'Module { Component { name: "Bruecke"; exports: ["de.probe/Bruecke 1.0"] } }\n' \
    > "$m/plugin.qmltypes"
printf 'target/\n' > "$d/.gitignore"
schattenpfad qmllint
cat > "$SCHATTEN/qmllint" <<'ATTRAPPE'
#!/bin/sh
case "$1" in --version) echo "qmllint attrappe"; exit 0 ;; esac
echo "Warning: q.qml:1:1: Type Bruecke is used but it is not resolved [unresolved-type]"
exit 1
ATTRAPPE
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_E2b" 0 "$d" "qmllint ohne Befund"
schatten_zuruecksetzen
unset PRUEFPFAD

# E2c — dieselbe Meldung mit einem Namen, der **nicht** in `exports` steht,
# blockiert weiter. Ohne diese Gegenprobe wäre nicht belegt, dass die Ableitung
# überhaupt etwas eingrenzt.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
m="$d/target/bau/qt-build-utils/qml_modules/de/probe"
mkdir -p "$m"
printf 'module de.probe\ntypeinfo plugin.qmltypes\n' > "$m/qmldir"
printf 'Module { Component { name: "Bruecke"; exports: ["de.probe/Bruecke 1.0"] } }\n' \
    > "$m/plugin.qmltypes"
printf 'target/\n' > "$d/.gitignore"
schattenpfad qmllint
cat > "$SCHATTEN/qmllint" <<'ATTRAPPE'
#!/bin/sh
case "$1" in --version) echo "qmllint attrappe"; exit 0 ;; esac
echo "Warning: q.qml:1:1: Type Fremdtyp is used but it is not resolved [unresolved-type]"
exit 1
ATTRAPPE
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_E2c" 1 "$d" "Fremdtyp" "Befund"
schatten_zuruecksetzen
unset PRUEFPFAD

# E3 — die Gegenprobe zur **anderen** Hälfte der Ausnahme. Der Musterfilter
# lässt Meldungen mit doppeltem Doppelpunkt und die abgeleiteten Brückentypen
# durch. Ein wirklich unauflösbarer Eigenschaftstyp trägt dieselbe Art
# `[unresolved-type]`, aber keines von beidem — er muss weiter blockieren.
# Ohne diesen Fall bliebe unbemerkt, wenn aus dem Muster wieder eine
# abgeschaltete Prüfart würde.
d="$(baue)"
mkdir -p "$d/target/build/x/out/qt-build-utils/qml_modules/de/probe"
cat > "$d/target/build/x/out/qt-build-utils/qml_modules/de/probe/qmldir" <<'QMLDIR'
module de.probe
typeinfo plugin.qmltypes
QMLDIR
cat > "$d/target/build/x/out/qt-build-utils/qml_modules/de/probe/plugin.qmltypes" <<'TYPEN'
import QtQuick.tooling 1.2
Module {
    Component {
        file: "x.h"
        name: "Echt"
        accessSemantics: "reference"
        prototype: "QObject"
        exports: ["de.probe/Echt 1.0"]
        exportMetaObjectRevisions: [256]
        Property { name: "wert"; type: "GibtEsNichtTyp"; read: "w"; index: 0 }
    }
}
TYPEN
cat > "$d/crates/ui/qml/Unaufloesbar.qml" <<'QML'
import QtQuick
import de.probe

Item {
    id: u

    Echt {
        id: e
    }

    width: e.wert
}
QML
printf 'target/\n' > "$d/.gitignore"
pruefe "$F_E3" 1 "$d" \
    "unresolved-type" "Befund"

# E4 — die Modulauswahl. Ein gebauter Baum trägt dasselbe Modul mehrfach; ohne
# Auswahl entschiede die Fundreihenfolge, welche Typbeschreibung gilt, und ein
# überholter Stand ließe eine entfernte Rolle als vorhanden erscheinen. Der
# Fall legt zwei Stände desselben Moduls an, setzt ihre Änderungszeit
# eindeutig und liest über eine Attrappe mit, welche `-I` tatsächlich gesetzt
# werden.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
for stand in alt neu; do
    m="$d/target/$stand/qt-build-utils/qml_modules/de/probe"
    mkdir -p "$m"
    printf 'module de.probe\ntypeinfo plugin.qmltypes\n' > "$m/qmldir"
    printf 'Module { Component { name: "%s"; exports: ["de.probe/%s 1.0"] } }\n' \
        "Typ_$stand" "Typ_$stand" > "$m/plugin.qmltypes"
done
printf 'target/\n' > "$d/.gitignore"
# Änderungszeit eindeutig setzen: `alt` älter als `neu`.
touch -t 202601010000 "$d/target/alt/qt-build-utils/qml_modules/de/probe/plugin.qmltypes"
touch -t 202606010000 "$d/target/neu/qt-build-utils/qml_modules/de/probe/plugin.qmltypes"
schattenpfad qmllint
# **Die Attrappe meldet als Befund, nicht auf stderr.** Der Prüfling schreibt
# die Werkzeugausgabe in eine Datei und druckt sie nur bei Befunden; eine
# stille Meldung erschiene nirgends, und der Fall bewiese nichts.
cat > "$SCHATTEN/qmllint" <<'ATTRAPPE'
#!/bin/sh
case "$1" in --version) echo "qmllint attrappe"; exit 0 ;; esac
pfade=""
for a in "$@"; do
  case "$a" in */qml_modules) pfade="$pfade $a" ;; esac
done
echo "Warning: probe.qml:1:1: GEWAEHLT$pfade [import]"
exit 1
ATTRAPPE
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_E4" 1 "$d" \
    "1 überholte(s) Modulverzeichnis" \
    "GEWAEHLT $d/target/neu/qt-build-utils/qml_modules"
schatten_zuruecksetzen
unset PRUEFPFAD

# E5 — der Gegenbeweis: **verschiedene** Module bleiben beide auf dem Pfad.
# Ohne diesen Fall könnte die Auswahl auf „genau ein Verzeichnis insgesamt"
# zusammenschrumpfen, und eine zweite Brücke fiele still heraus.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
for modul in eins zwei; do
    m="$d/target/bau-$modul/qt-build-utils/qml_modules/de/$modul"
    mkdir -p "$m"
    printf 'module de.%s\ntypeinfo plugin.qmltypes\n' "$modul" > "$m/qmldir"
    printf 'Module { Component { name: "T"; exports: ["de.%s/T 1.0"] } }\n' \
        "$modul" > "$m/plugin.qmltypes"
done
printf 'target/\n' > "$d/.gitignore"
schattenpfad qmllint
cat > "$SCHATTEN/qmllint" <<'ATTRAPPE'
#!/bin/sh
case "$1" in --version) echo "qmllint attrappe"; exit 0 ;; esac
n=0
for a in "$@"; do
  case "$a" in */qml_modules) n=$((n + 1)) ;; esac
done
echo "Warning: probe.qml:1:1: IMPORTPFADE=$n [import]"
exit 1
ATTRAPPE
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_E5" 1 "$d" "IMPORTPFADE=2"
schatten_zuruecksetzen
unset PRUEFPFAD

# --- F · Ein Werkzeugfehler ist kein bestandener Lauf -----------------------
# Bricht qmllint ohne erkennbare Meldung ab, stünde ohne diese Abfrage ein
# grünes Gate im Protokoll, das nichts geprüft hat.
d="$(baue)"
# Formatgerecht nach qmlformat, damit der Fall nicht an der Formatprüfung
# scheitert statt an dem, was er beweisen soll.
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
schattenpfad qmllint
printf '#!/bin/sh\nexit 7\n' > "$SCHATTEN/qmllint"
chmod +x "$SCHATTEN/qmllint"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_F1" 1 "$d" \
    "ohne eine" "unbekannten Format"
schatten_zuruecksetzen
unset PRUEFPFAD

# F2 — dieselbe Zusage für das zweite Werkzeug. Verwürfe der Prüfling
# Rückgabewert und Fehlerausgabe von `qmlformat`, sähe ein Abbruch genauso aus
# wie eine unformatierte Datei: Der Nutzer läse „nicht formatiert" und führte
# `qmlformat -i` aus — einen Befehl, der die Ursache nicht berührt.
d="$(baue)"
printf 'import QtQuick\n\nItem {}\n' > "$d/crates/ui/qml/Leer.qml"
schattenpfad qmlformat
printf '#!/bin/sh\necho "Bibliothek fehlt" >&2\nexit 5\n' > "$SCHATTEN/qmlformat"
chmod +x "$SCHATTEN/qmlformat"
PRUEFPFAD="$SCHATTEN" \
pruefe "$F_F2" 1 "$d" \
    "qmlformat brach ab" "kein Formatbefund"
schatten_zuruecksetzen
unset PRUEFPFAD

# --- G · Formatprüfung -----------------------------------------------------
# Sie läuft **ohne** `-n`: Der Schalter sortierte die Eigenschaften alphabetisch
# und zerrisse die Gliederung der Gestaltungsquelle nach den Abschnitten von
# DES-STM-001 (Analysis/20260826_02_bestandsaufnahme.md).
d="$(baue)"
cat > "$d/crates/ui/qml/Schief.qml" <<'QML'
import QtQuick

Item {
    id: schief

        property int breite: 10

    width: schief.breite
}
QML
pruefe "$F_G1" 1 "$d" \
    "nicht formatiert" "qmlformat -i"

# Und der Gegenbeweis: Die Reihenfolge der Eigenschaften ist **kein** Befund.
# Liefe die Prüfung mit `-n`, blockierte dieser Fall — und mit ihm die
# Gliederung der Gestaltungsquelle.
d="$(baue)"
cat > "$d/crates/ui/qml/Gruppiert.qml" <<'QML'
import QtQuick

Item {
    id: gruppiert

    // --- Erste Gruppe ---
    // Absichtlich nicht alphabetisch: Genau das wäre unter `-n` ein Befund.
    // Farbwerte stehen hier bewusst nicht — sie wären Gestaltungsliterale
    // außerhalb der Gestaltungsquelle und blockierten den eigenen Commit
    // (D-05, dieselbe Vorkehrung wie in check-projektregeln.test.sh).
    property int zwei: 2
    property int eins: 1

    // --- Zweite Gruppe ---
    property int breite: 10

    width: gruppiert.breite
}
QML
qmlformat -i "$d/crates/ui/qml/Gruppiert.qml" >/dev/null 2>&1
pruefe "$F_G2" 0 "$d" \
    "ohne Abweichung"

fi

# **Die Gesamtzahl wird geprüft, nicht geglaubt.** Die Entfallliste wird aus
# den `pruefe`-Aufrufen des Zweigs abgeleitet; findet die Ableitung nichts —
# umformulierter Marker, Aufruf über `bash < datei` —, meldete der Selbsttest
# „0 entfallen" und endete grün, und die dokumentierte Rechnung wanderte
# lautlos auseinander. Genau die Drift, gegen die die Ableitung angetreten ist,
# nur in einen Kommentar verlagert.
FAELLE_GESAMT=26
if [ $((bestanden + fehlgeschlagen + entfallen)) -ne "$FAELLE_GESAMT" ]; then
    echo
    echo "Selbsttest QML: FAIL — $((bestanden + fehlgeschlagen + entfallen)) Fälle abgerechnet," \
         "erwartet $FAELLE_GESAMT."
    echo "  Entweder ist ein Fall hinzugekommen, ohne dass FAELLE_GESAMT nachgezogen wurde,"
    echo "  oder die Ableitung der Entfallliste greift nicht mehr."
    exit 1
fi

echo
if [ "$entfallen" -gt 0 ]; then
    echo "Selbsttest QML: $bestanden bestanden, $fehlgeschlagen fehlgeschlagen," \
         "$entfallen entfallen (Qt-Werkzeugkette fehlt —$FEHLENDE)"
else
    echo "Selbsttest QML: $bestanden bestanden, $fehlgeschlagen fehlgeschlagen"
fi
[ "$fehlgeschlagen" -eq 0 ]
