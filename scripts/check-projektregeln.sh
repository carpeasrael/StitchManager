#!/usr/bin/env bash
# Projektregelprüfungen am Quellbaum (CLAUDE.md Abschnitt 11).
#
# Geprüft werden die Regeln, die kein Sprachwerkzeug prüft:
#   1. D-05 / SM-DES-003 — kein Farb-, Schrift- oder Abstandsliteral außerhalb
#      der Gestaltungsquelle.
#   2. SM-SEC-004 — die Oberfläche kennt die Datenhaltung nicht (Schnittregel 1).
#   3. SM-SEC-005 — keine Zeichenkettenverkettung in Abfragen.
#   4. SM-OSS-011 — die Installationsskripte der Abhängigkeitskette sind
#      abgeschaltet (`.npmrc` mit `ignore-scripts=true`).
#   5. SM-PLT-007 — Versionsangaben in Projekt- und Paketdateien stimmen überein.
#
# Rückgabewerte: 0 = PASS · 1 = FAIL · 3 = ENTFÄLLT.
#
# **Das Skript legt keinen Oberflächenweg fest.** Weg A (cxx-qt) und Weg B
# (PySide6 mit PyO3) sind offen (IMP-STM-001 Abschnitt 2.1); ein Skript, das
# QML oder ein Kistenlayout fest kodiert, entschiede den offenen Punkt und
# blockierte unter dem anderen Weg jeden Commit ohne begehbaren Weg (S1).
# Deshalb gilt:
#   · Die Gestaltungsquelle **deklariert sich selbst** über die Markierung
#     `GESTALTUNGSQUELLE` in der Datei, statt hier benannt zu werden.
#   · Die Zuordnung logischer Module auf Pfade steht in `.projektregeln.conf`.
#   · Fehlt der Gegenstand einer Prüfung, ist das ENTFÄLLT (S3), nie FAIL.
set -u

WURZEL="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$WURZEL" || exit 3

CONF=".projektregeln.conf"

# Ein Quellbaum liegt vor, wenn eine Projektdatei irgendeines Wegs da ist.
quellbaum_vorhanden() {
    [ -f Cargo.toml ] || [ -f pyproject.toml ] || return 1
}

if [ ! -f "$CONF" ]; then
    if quellbaum_vorhanden; then
        # Gegenstand da, Prüfung nicht durchführbar → FAIL (S3).
        echo "check-projektregeln: FAIL — Quellbaum vorhanden, aber $CONF fehlt."
        echo "  Ohne Zuordnung der logischen Module auf Pfade ist keine der"
        echo "  Regeln prüfbar. Ein Quellbaum ohne Zuordnung ist kein geprüfter Quellbaum."
        exit 1
    fi
    echo "check-projektregeln: ENTFÄLLT — kein Quellbaum und keine Zuordnung"
    exit 3
fi

# ── Zuordnung lesen ──────────────────────────────────────────────────────────
konf() {  # $1 = Schlüssel
    sed -n "s/^[[:space:]]*$1[[:space:]]*=[[:space:]]*//p" "$CONF" \
        | sed 's/[[:space:]]*$//' | head -1
}
OBERFLAECHE="$(konf oberflaeche)"
DATENHALTUNG="$(konf datenhaltung)"
ABFRAGEN="$(konf abfragen)"
# Datenbanktreiber, an denen die Oberfläche die Fassade umgehen könnte. Aus der
# Konfiguration, nicht aus dem Skript — sonst entschiede der Prüfer den offenen
# Punkt der Wegwahl mit (CLAUDE.md Abschnitt 11).
TREIBER="$(konf treiber)"

# Zahl der Prüfbedingungen. Sie steht **einmal** hier und wird von der
# Schlussmeldung und der ENTFÄLLT-Schwelle gelesen; eine an zwei Stellen
# gepflegte Zahl läuft auseinander, sobald eine Bedingung hinzukommt.
PRUEFUNGEN=5

TAB="$(printf '\t')"   # Trennzeichen der Befundzeilen aus kn_literale

# Die Regeln zu SM-DES-003/D-05 stehen **einmal** im Baum — gemeinsam mit
# check-docs.sh. Zuvor lagen Muster, Erweiterungsliste und Ausnahmeform in
# beiden Skripten und waren bereits auseinandergelaufen.
if [ -r "$WURZEL/scripts/lib/gestaltung.sh" ]; then
    # shellcheck source=lib/gestaltung.sh
    . "$WURZEL/scripts/lib/gestaltung.sh"
else
    echo "check-projektregeln: FAIL — scripts/lib/gestaltung.sh fehlt (SM-DES-003)" >&2
    exit 1
fi
fehler=0
entfallen=0
melde()   { echo "  ✗ $*"; fehler=$((fehler + 1)); }
gut()     { echo "  ✓ $*"; }
entfaellt() { echo "  – ENTFÄLLT: $*"; entfallen=$((entfallen + 1)); }

# Nur Quelltextarten der Oberfläche können die Gestaltungsquelle sein. Eine
# Schaltdatei ist es nie — und eine Prüfdatei, die eine Deklaration in einem
# Heredoc erzeugt, erst recht nicht. Ohne diese Schranke befreite sich
# `check-projektregeln.test.sh` über den Text seiner eigenen Prüfattrappe von
# D-05, und die Farbliterale seiner Attrappen blieben ungeprüft.
# Wortkopie entfernt: `kn_quelltextart` aus scripts/lib/gestaltung.sh ist die
# eine Antwort auf diese Frage. Eine zweite Fassung daneben hieße, die
# Bibliothek ändern zu können, ohne dass sich das Prüfergebnis ändert — genau
# die Drift, gegen die sie angelegt wurde.
gestaltungsquelle_moeglich() { kn_quelltextart "$1"; }

# Quelltextdateien beider Wege. Nicht nur `*.qml` — sonst prüfte das Skript
# unter Weg B nichts und meldete trotzdem grün.
quellen_unter() {  # $1 = Pfad
    [ -d "$1" ] || return 0
    git -c core.quotePath=false ls-files --cached --others --exclude-standard \
        -- "$1" 2>/dev/null \
        | grep -E "\.($KN_PRUEF_EXT)$" | sort -u
}

# ── 1 · D-05 / SM-DES-003 ────────────────────────────────────────────────────
echo "Prüfung 1 — Gestaltungsliterale (D-05, SM-DES-003)"
if [ -z "$OBERFLAECHE" ] || [ ! -d "$OBERFLAECHE" ]; then
    entfaellt "keine Oberflächenschicht im Baum"
else
    dateien="$(quellen_unter "$OBERFLAECHE")"
    if [ -z "$dateien" ]; then
        entfaellt "Oberflächenschicht ohne Quelltextdateien"
    else
        # Die Gestaltungsquelle deklariert sich selbst. SM-DES-003 verlangt
        # **genau eine**; null und mehrere sind beides ein Befund.
        # **Deklaration, nicht Erwähnung.** Gesucht wird eine Zeile, die nur
        # aus Kommentarzeichen und dem Wort besteht. Ein `grep` auf das bloße
        # Vorkommen machte jede Datei, die das Wort irgendwo nennt — auch diese
        # Skripte und das Regelwerk —, zum dateiweiten Freibrief gegen D-05.
        quellen=""
        while IFS= read -r kandidat; do
            [ -n "$kandidat" ] || continue
            if kn_ist_gestaltungsquelle "$kandidat"; then
                quellen="$quellen$kandidat
"
            fi
        done < <(printf '%s\n' "$dateien")
        quellen="$(printf '%s' "$quellen" | grep . || true)"
        anzahl="$(printf '%s' "$quellen" | grep -c . || true)"
        if [ "${anzahl:-0}" -eq 0 ]; then
            melde "keine Datei trägt die Markierung GESTALTUNGSQUELLE — SM-DES-003 verlangt genau eine"
        elif [ "${anzahl:-0}" -gt 1 ]; then
            melde "$anzahl Dateien tragen GESTALTUNGSQUELLE; SM-DES-003 verlangt genau eine:"
            printf '%s\n' "$quellen" | sed 's/^/      /'
        else
            gut "Gestaltungsquelle: $quellen"
        fi

        # **Ein Durchlauf für den ganzen Prüfbereich**, nicht einer je
        # Datei. Zuvor startete je Quelldatei ein eigener `perl`; bei einem
        # Quellbaum in der angekündigten Größenordnung sind das Tausende
        # Prozessstarts je Commit („billig vor teuer").
        #
        # Der Durchlauf liest jede Datei im Ganzen und folgt Zuweisungen über
        # Zeilengrenzen: QML bricht sie regelmäßig um, und eine zeilenweise
        # Bedingung meldet dann grün, ohne je etwas gesehen zu haben.
        treffer=0
        _pruef=()
        while IFS= read -r f; do
            [ -n "$f" ] || continue
            case " $quellen " in *" $f "*) continue ;; esac
            _pruef+=("$f")
        done < <(printf '%s\n' "$dateien")
        if [ "${#_pruef[@]}" -gt 0 ]; then
            if ! befunde="$(kn_literale "${_pruef[@]}")"; then
                melde "Literalprüfung nicht durchführbar (perl-Rückgabewert) — nicht geprüft heißt nicht bestanden (S3)"
                treffer=1
            elif [ -n "$befunde" ]; then
                # Keine Pipe: `melde` erhöht einen Zähler; in einer Subshell
                # ginge er verloren und die Prüfung meldete grün, nachdem sie
                # Befunde gedruckt hat.
                while IFS="$TAB" read -r klasse datei nr _rest; do
                    [ -n "$klasse" ] || continue
                    melde "$datei:$nr: $(kn_klassentext "$klasse") außerhalb der Gestaltungsquelle. Wert als Bezeichner in die Gestaltungsquelle aufnehmen oder in derselben Zeile 'D-05-Ausnahme: <Grund>' setzen"
                    treffer=1
                done <<< "$befunde"
            fi
        fi
        [ "$treffer" = 0 ] && gut "keine Literale außerhalb der Gestaltungsquelle"
    fi
fi

# ── 2 · SM-SEC-004 — Schichttrennung ─────────────────────────────────────────
echo "Prüfung 2 — Schichttrennung (SM-SEC-004, Schnittregel 1)"
if [ -z "$OBERFLAECHE" ] || [ ! -d "$OBERFLAECHE" ] || [ -z "$DATENHALTUNG" ]; then
    entfaellt "keine Oberflächenschicht oder keine Datenhaltung benannt"
else
    treffer=0
    # Der Name der Datenhaltung in beiden üblichen Schreibungen: `kern-db` in
    # Projektdateien, `kern_db` im Programmtext.
    dh_strich="$DATENHALTUNG"
    dh_unter="$(printf '%s' "$DATENHALTUNG" | tr '-' '_')"
    for manifest in "$OBERFLAECHE/Cargo.toml" "$OBERFLAECHE/pyproject.toml"; do
        [ -f "$manifest" ] || continue
        if grep -qE -- "^[[:space:]]*[\"']?${dh_strich}[\"']?[[:space:]]*(=|\\.)" "$manifest"; then
            melde "$manifest führt $dh_strich — die Oberfläche greift an der Fassade vorbei"
            treffer=1
        fi
        # **Auch der Treiber gehört in den Manifestzweig.** Der Kommentar
        # sagte zu, der Name der Fassade genüge als Prüfung nicht — geprüft
        # wurde im Manifest aber nur er. Ein `rusqlite = "…"` in
        # `crates/ui/Cargo.toml` passierte, und der Quelltextzweig greift auf
        # `Cargo.toml` gar nicht zu (nicht in `quellen_unter`).
        #
        # Beide Schreibweisen: Kistennamen tragen Bindestriche, Modulpfade
        # Unterstriche.
        set -f
        for t in $TREIBER; do
            tn="$(printf '%s' "$t" | tr '_' '-')"
            tu="$(printf '%s' "$t" | tr '-' '_')"
            if grep -qE -- "^[[:space:]]*[\"']?(${tn}|${tu})[\"']?[[:space:]]*(=|\\.)" "$manifest"; then
                melde "$manifest führt den Datenbanktreiber $t — die Oberfläche greift an der Fassade vorbei"
                treffer=1
            fi
        done
        set +f
    done
    # **Der Name der Fassade genügt nicht.** Geprüft wurde allein, ob die
    # Oberfläche `kern-db` nennt. Ein Bauteil, das statt dessen den Treiber
    # unmittelbar zieht (`rusqlite`, `sqlite3`, `QSqlDatabase`), umgeht die
    # Fassade genauso — und blieb unbemerkt. Die Treibernamen stehen in
    # `.projektregeln.conf`, nicht im Skript: Dieselbe Frage zweimal zu
    # beantworten führt unter einem anderen Oberflächenweg zu zwei Wahrheiten.
    # **Vollständig maskieren, sonst kippt die Prüfung ins Fail-open.** Ein
    # Eintrag mit unbalancierter Klammer erzeugt ein ungültiges Muster; `grep`
    # bricht dann mit Rückgabewert 2 ab, `2>/dev/null` verschluckt die Meldung,
    # und die Bedingung ist dauerhaft falsch — die Prüfung meldet „kennt keinen
    # Treiber", ohne eine Datei bewertet zu haben.
    #
    # `set -f` um die Schleife: Ein Eintrag `*` expandierte sonst gegen das
    # Arbeitsverzeichnis.
    treiber_muster=""
    set -f
    for t in $TREIBER; do
        treiber_muster="$treiber_muster${treiber_muster:+|}$(printf '%s' "$t" | sed 's/[][\\.^$*+?(){}|]/\\&/g')"
    done
    set +f
    while IFS= read -r f; do
        [ -n "$f" ] || continue
        if grep -qE -- "\\b${dh_unter}\\b" "$f" 2>/dev/null; then
            melde "$f verweist unmittelbar auf die Datenhaltung ($dh_unter)"
            treffer=1
        fi
        if [ -n "$treiber_muster" ]; then
            grep -qE -- "\\b(${treiber_muster})\\b" "$f" 2>/dev/null
            grc=$?
            if [ "$grc" -eq 0 ]; then
                melde "$f greift unmittelbar auf einen Datenbanktreiber zu — die Oberfläche geht an der Fassade vorbei"
                treffer=1
            elif [ "$grc" -gt 1 ]; then
                # Rückgabewert > 1 heißt „grep konnte nicht auswerten", nicht
                # „nichts gefunden". Das als Bestehen zu lesen wäre genau der
                # Fehler, den S3 ausschließt.
                melde "Treibermuster nicht auswertbar (grep-Rückgabewert $grc) — Prüfung nicht durchführbar, Eintrag 'treiber' in .projektregeln.conf prüfen (S3)"
                treffer=1
            fi
        fi
    done < <(quellen_unter "$OBERFLAECHE")
    if [ "$treffer" = 0 ]; then
        gut "die Oberfläche kennt $DATENHALTUNG nicht${TREIBER:+ und keinen Datenbanktreiber}"
    fi
fi

# ── 3 · SM-SEC-005 — parametrisierte Abfragen ────────────────────────────────
echo "Prüfung 3 — parametrisierte Abfragen (SM-SEC-005)"
if [ -z "$ABFRAGEN" ] || [ ! -d "$ABFRAGEN" ]; then
    entfaellt "kein Pfad mit Abfragen benannt"
else
    treffer=0
    sql='SELECT|INSERT|UPDATE|DELETE|WHERE|VALUES'
    # Eine Abfrage, die einen Wert in den Text einsetzt statt ihn zu binden.
    #
    # Ob eine eingesetzte Größe ein **fester Textbaustein** (zulässig: Rumpf,
    # Sortierklausel, Platzhalterliste) oder ein **Fremdwert** ist (unzulässig),
    # entscheidet kein Muster — das steht im umgebenden Programm. Die Prüfung
    # meldet deshalb jede Einsetzung in einer Abfragezeichenkette und verlangt
    # eine Begründung in derselben Zeile. Dasselbe Muster schreibt CLAUDE.md
    # Abschnitt 12 für bewusste Ausnahmen beim Secret-Scan vor; es zwingt zur
    # Begründung, statt die Regel zu lockern.
    while IFS= read -r f; do
        [ -n "$f" ] || continue
        befunde="$(perl -0777 -ne '
            my @zeilen = split /\n/, $_;
            # `write!` und `writeln!` bauen Zeichenketten genauso zusammen
            # wie `format!`; der Kopf sagt „keine Zeichenkettenverkettung in
            # Abfragen" zu und muss sie deshalb mitnehmen. Zwischen Klammer und
            # Zeichenkette steht bei `write!` das Ziel (`q, `).
            while (/(?:format|write|writeln)!\s*\([^"]{0,80}?(?:r?#*)?"((?:[^"\\]|\\.)*)"/gs) {
                # Anfang des Aufrufs, nicht Ende der Zeichenkette: Bei einer
                # mehrzeiligen Abfrage läge die Begründung sonst außerhalb des
                # Fensters und die Prüfung meldete einen Befund, den es nicht gibt.
                my ($s, $anfang) = ($1, $-[0]);
                next unless $s =~ /\b(?:SELECT|INSERT|UPDATE|DELETE|WHERE|VALUES)\b/i;
                # Jede Einsetzung zählt, nicht nur die benannte. Die
                # kanonische Rust-Schreibweise ist `{}` mit Positionsargument
                # und fiel zuvor vollständig durch; `{{` ist eine geschützte
                # geschweifte Klammer und keine Einsetzung.
                my $ohne_escape = $s;
                $ohne_escape =~ s/\{\{|\}\}//g;
                next unless $ohne_escape =~ /\{[^{}]*\}/;
                my $nr = (substr($_, 0, $anfang) =~ tr/\n//) + 1;
                # **Dieselbe Zeile oder die unmittelbar vorangehende
                # Kommentarzeile — mehr nicht.** Zuvor galt die Markierung
                # irgendwo in einem beliebig langen Kommentarblock darüber.
                # Damit überdauerte eine einmal gegebene Begründung den
                # Austausch der Abfrage darunter: Aus einem festen Rumpf wurde
                # ein eingesetzter Fremdwert, und das Gate meldete weiter grün.
                # CLAUDE.md Abschnitt 11 lässt genau eine Zeile zu.
                #
                # Und sie **verlangt einen Grund**: Die nackte Markierung ist
                # ein Freibrief, keine Begründung.
                my $begruendet = qr/SM-SEC-005-Ausnahme:\s*\S/;
                my $ok = ($zeilen[$nr - 1] // "") =~ $begruendet;
                if (!$ok && $nr >= 2) {
                    my $davor = $zeilen[$nr - 2] // "";
                    $ok = 1 if $davor =~ m{^\s*(?://|\#)} && $davor =~ $begruendet;
                }
                print "$nr\n" unless $ok;
            }' < "$f" 2>/dev/null || true)"
        n="$(printf '%s' "$befunde" | grep -c . || true)"
        if [ "${n:-0}" -gt 0 ]; then
            melde "$f: $n Abfrage(n) setzen einen Wert ein, ohne Begründung (Zeilen: $(printf '%s' "$befunde" | tr '\n' ' '))"
            treffer=1
        fi
        # **Der Abfragetext selbst, nicht nur seine Entstehung.** Die beiden
        # Raster oben sehen `format!` mit SQL-Schlüsselwort und Verkettung in
        # Dateien, die SQL enthalten. Eine Abfrage, deren Text als Konstante in
        # einer anderen Datei liegt und hier nur noch zusammengesetzt
        # übergeben wird, entzog sich beiden — und genau das ist der Weg, auf
        # dem ein Fremdwert in den Abfragetext gerät (SM-SEC-005).
        #
        # Geprüft wird deshalb die **Übergabestelle**: Ist das erste Argument
        # einer Abfragefunktion kein Zeichenkettenliteral, ist es
        # begründungspflichtig.
        befunde="$(perl -0777 -ne '
            my @zeilen = split /\n/, $_;
            my $begruendet = qr/SM-SEC-005-Ausnahme:\s*\S/;
            while (/\.\s*(?:execute|execute_batch|batch|query|query_row|query_map|query_and_then|prepare|prepare_cached)\s*\(\s*([^\s,)]*)/gs) {
                my ($arg, $anfang) = ($1, $-[0]);
                # Zeichenkettenliteral in jeder Rust-Schreibweise — zulässig.
                next if $arg =~ /^(?:r?\#*")/;
                next if $arg eq "";                      # mehrzeilig, gleich folgt "
                # `stmt.execute(params![…])` ist ein Aufruf auf einer bereits
                # **vorbereiteten** Anweisung: Das erste Argument sind die
                # Werte, nicht der Abfragetext. Der Text dieser Anweisung wurde
                # an der `prepare`-Stelle geprüft; hier noch einmal zu melden
                # hieße, die Bindung von Werten zu beanstanden — also genau
                # das, was SM-SEC-005 verlangt.
                next if $arg =~ /^(?:params!|params_from_iter|\[|\(|&\[|NO_PARAMS|\|)/;
                my $nr = (substr($_, 0, $anfang) =~ tr/\n//) + 1;
                my $ok = ($zeilen[$nr - 1] // "") =~ $begruendet;
                if (!$ok && $nr >= 2) {
                    my $davor = $zeilen[$nr - 2] // "";
                    $ok = 1 if $davor =~ m{^\s*(?://|\#)} && $davor =~ $begruendet;
                }
                print "$nr\n" unless $ok;
            }' < "$f" 2>/dev/null || true)"
        n="$(printf '%s' "$befunde" | grep -c . || true)"
        if [ "${n:-0}" -gt 0 ]; then
            melde "$f: $n Abfrage(n) erhalten ihren Text aus einer Variablen, ohne Begründung (Zeilen: $(printf '%s' "$befunde" | tr '\n' ' '))"
            treffer=1
        fi
        # `+=` nur mit `&` oder Anführungszeichen dahinter: `zahl += 1` ist
        # Ganzzahladdition und hat mit Zeichenketten nichts zu tun.
        # Verkettung. Die frühere Fassung verlangte das Schlüsselwort in
        # derselben Zeile und ging deshalb an der häufigsten Form vorbei: Die
        # Abfrage steht in einer Zwischenvariablen, angehängt wird woanders.
        # Geprüft wird daher: Enthält die Datei überhaupt eine
        # Abfragezeichenkette, ist jedes Anhängen begründungspflichtig.
        if grep -qE "\"[^\"]*($sql)[^\"]*\"" "$f" 2>/dev/null; then
            n="$(grep -nE "push_str\\(|\\bformat_args!|\"[^\"]*($sql)[^\"]*\"[[:space:]]*\\+|\\+=[[:space:]]*[&\"]" "$f" 2>/dev/null \
                 | grep -vc 'SM-SEC-005-Ausnahme:' || true)"
            if [ "${n:-0}" -gt 0 ]; then
                melde "$f: $n Zeile(n) hängen an eine Zeichenkette an, während die Datei eine Abfrage trägt"
                treffer=1
            fi
        fi
    done < <(quellen_unter "$ABFRAGEN")
    [ "$treffer" = 0 ] && gut "keine Abfrage setzt einen Wert ohne Begründung ein"
fi

# ── 3b · SM-OSS-011 — Installationsskripte Dritter sind abgeschaltet ─────────
echo "Prüfung 3b — Installationsskripte der Abhängigkeitskette (SM-OSS-011)"
if [ ! -f package-lock.json ]; then
    entfaellt "keine npm-Abhängigkeitskette im Baum"
elif [ ! -f .npmrc ]; then
    # Die Zusage im Regelwerk braucht ein Skript, das sie durchsetzt. Ohne
    # `.npmrc` führt `npm` die preinstall-, install- und postinstall-Skripte
    # jedes Pakets mit den Rechten der Nutzerin aus.
    melde ".npmrc fehlt, während eine Abhängigkeitskette im Baum liegt — die Installationsskripte Dritter wären aktiv (SM-OSS-011)"
elif ! grep -qE '^[[:space:]]*ignore-scripts[[:space:]]*=[[:space:]]*true[[:space:]]*$' .npmrc; then
    melde ".npmrc setzt kein 'ignore-scripts=true'. Zeile ergänzen (SM-OSS-011)"
else
    # **Positivliste, nicht Stichprobe.** Die Prüfung fragte nur, *ob* eine
    # Zeile vorhanden ist. Jede weitere Direktive derselben Datei blieb
    # unbeanstandet — und `.npmrc` ist seit diesem Änderungssatz eine im Gate
    # gedeckte Konfigurationsquelle für jeden `npm`-Lauf. Eine Zeile
    # `registry=https://…` daneben lenkt den Bezug der gesamten Kette um
    # (SM-SEC-007, SM-NFR-014: ausgehende Verbindungen begrenzt), ein
    # `strict-ssl=false` senkt die Transportsicherung, und ein `_authToken`
    # wäre ein Geheimnis im Baum (CLAUDE.md Abschnitt 12).
    #
    # Zulässig sind deshalb nur Leerzeilen, Kommentare und ausdrücklich
    # freigegebene Direktiven. Wer eine weitere braucht, nimmt sie hier auf —
    # mit Begründung, wie bei der Lizenz-Positivliste.
    unzulaessig="$(grep -vnE '^[[:space:]]*(#|;|$|ignore-scripts[[:space:]]*=[[:space:]]*true[[:space:]]*$|audit[[:space:]]*=[[:space:]]*(true|false)[[:space:]]*$|fund[[:space:]]*=[[:space:]]*(true|false)[[:space:]]*$)' .npmrc || true)"
    if [ -n "$unzulaessig" ]; then
        melde ".npmrc enthält nicht freigegebene Direktiven — eine Umleitung der Bezugsquelle oder ein Zugangstoken wäre hier nicht zu sehen (SM-OSS-011, SM-SEC-007):"
        printf '%s\n' "$unzulaessig" | sed 's/^/      /'
    else
        gut "Installationsskripte Dritter sind abgeschaltet, keine weiteren Direktiven"
    fi
fi

# ── 4 · SM-PLT-007 — Versionsgleichheit ──────────────────────────────────────
echo "Prüfung 4 — Versionsgleichheit (SM-PLT-007)"
# Versionsangaben aus allen Projekt- und Paketdateien, die es im Baum gibt.
# Nicht am Zeilenanfang verankert: `{ "version": "1.0" }` steht oft in einer
# Zeile mit der Klammer, und `version = "…"` kann eingerückt sein.
versionen=""
if [ -f Cargo.toml ]; then
    v="$(sed -n '/^\[workspace\.package\]/,/^\[.*\]/p' Cargo.toml \
         | grep -oE '^[[:space:]]*version[[:space:]]*=[[:space:]]*"[^"]*"' \
         | sed 's/.*"\([^"]*\)"$/\1/' | head -1)"
    [ -n "$v" ] && versionen="$versionen
Cargo.toml $v"
fi
if [ -f package.json ]; then
    v="$(grep -oE '"version"[[:space:]]*:[[:space:]]*"[^"]*"' package.json \
         | sed 's/.*"\([^"]*\)"$/\1/' | head -1)"
    [ -n "$v" ] && versionen="$versionen
package.json $v"
fi
if [ -f pyproject.toml ]; then
    v="$(grep -oE '^[[:space:]]*version[[:space:]]*=[[:space:]]*"[^"]*"' pyproject.toml \
         | sed 's/.*"\([^"]*\)"$/\1/' | head -1)"
    [ -n "$v" ] && versionen="$versionen
pyproject.toml $v"
fi
werte="$(printf '%s\n' "$versionen" | grep -c . || true)"
if [ "${werte:-0}" -lt 2 ]; then
    entfaellt "weniger als zwei Versionsangaben im Baum — nichts zu vergleichen"
else
    verschieden="$(printf '%s\n' "$versionen" | grep . | awk '{print $2}' | sort -u | wc -l | tr -d ' ')"
    if [ "$verschieden" -gt 1 ]; then
        melde "Versionsangaben weichen voneinander ab:"
        printf '%s\n' "$versionen" | grep . | sed 's/^/      /'
    else
        gut "alle $werte Versionsangaben stimmen überein"
    fi
fi

echo
if [ "$fehler" -eq 0 ]; then
    # Ist **keine** der Prüfungen gelaufen, ist das kein Bestehen. Ein
    # PASS ohne durchgeführte Prüfung ist genau der Satz, den CLAUDE.md
    # Abschnitt 13 ausschließt: „ein nicht durchgeführter Test ist kein
    # bestandener Test".
    if [ "$entfallen" -ge "$PRUEFUNGEN" ]; then
        echo "check-projektregeln: ENTFÄLLT — keine der $PRUEFUNGEN Prüfungen hatte einen Gegenstand"
        exit 3
    fi
    if [ "$entfallen" -gt 0 ]; then
        echo "check-projektregeln: PASS — ohne Befund, $entfallen Prüfung(en) entfallen"
    else
        echo "check-projektregeln: PASS — $PRUEFUNGEN Prüfungen ohne Befund"
    fi
    exit 0
fi
echo "check-projektregeln: FAIL — $fehler Befund(e)"
exit 1
