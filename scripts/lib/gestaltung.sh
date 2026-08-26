# shellcheck shell=bash
# ── Gemeinsame Regeln zu SM-DES-003 / D-05 ──────────────────────────────────
#
# **Warum diese Datei existiert.** Dieselbe Designregel lag zuvor in
# `check-docs.sh` und `check-projektregeln.sh` je einmal — und war bereits
# auseinandergelaufen: Das eine Skript meldete nur *mehr als eine*
# Gestaltungsquelle, das andere zusätzlich *keine*. Für eine Anforderung, die
# „genau eine" verlangt, galten damit zwei Maßstäbe, je nachdem welches Gate
# zog. CLAUDE.md Abschnitt 11 benennt genau das als unzulässig: „Dieselbe Frage
# zweimal zu beantworten führt … zu zwei Wahrheiten."
#
# Wer diese Datei ändert, ändert das Prüfergebnis. Sie steht deshalb in
# `gate_signature` (scripts/review-gate.sh) — sonst gäbe der Cache nach einer
# Regeländerung ein altes Ergebnis zurück.

# **`perl` ist Voraussetzung, nicht Zufall.** Die drei tragenden Bedingungen
# dieses Prüfstands — D-05, die Gestaltungswerte und SM-SEC-005 — laufen über
# `perl`. Fehlt es (schlanke CI-Abbilder, Alpine, minimale Container), lieferte
# ein ungeprüfter Aufruf leer, und das Gate meldete „keine Literale außerhalb
# der Gestaltungsquelle" — ohne eine Zeile gelesen zu haben. Der Gegenstand
# liegt im Baum, also ist das FAIL mit Installationshinweis, nicht ENTFÄLLT
# (S3, CLAUDE.md Abschnitt 13).
if ! command -v perl >/dev/null 2>&1; then
    echo "  ✗ perl fehlt — D-05 (SM-DES-003) und SM-SEC-005 sind nicht prüfbar (S3)." >&2
    echo "    Debian/Ubuntu: apt install perl · macOS: im Grundsystem enthalten" >&2
    exit 1
fi

# Drei-, sechs- und achtstelliges Hex.
KN_HEX='#[0-9a-fA-F]{3}([0-9a-fA-F]{3})?([0-9a-fA-F]{2})?'

# Quelltextarten, die überhaupt Gestaltungsquelle sein können. Eine Schaltdatei
# ist es nie — sonst befreite sich jede Datei, die die Markierung auch nur
# erzeugt (etwa in einem Heredoc), selbst von D-05.
KN_QUELL_EXT='qml|py|css|qss|ui|js|ts'

# Die Ausnahmemarkierung **verlangt einen Grund**. `D-05-Ausnahme:` allein ist
# keine Begründung, sondern ein Freibrief; CLAUDE.md Abschnitt 11 schreibt
# `D-05-Ausnahme: <Grund>` vor. Dasselbe gilt für SM-SEC-005.
KN_AUSNAHME='D-05-Ausnahme:[[:space:]]*[^[:space:]]'
KN_SEC005_AUSNAHME='SM-SEC-005-Ausnahme:[[:space:]]*[^[:space:]]'

# **Deklaration, nicht Erwähnung.** Eine Zeile, die nur aus Kommentarzeichen und
# dem Wort besteht.
KN_DEKLARATION='^[[:space:]]*(//|#|\*)[[:space:]]*GESTALTUNGSQUELLE[[:space:]]*$'

# **Aus `KN_QUELL_EXT` abgeleitet, nicht daneben geschrieben.** Ein zweites
# `case`-Muster derselben Liste war genau die Doppelung, gegen die diese Datei
# angelegt wurde: Wer die Variable ergänzt, hätte den Prüfumfang der einen
# Stelle geändert und den der anderen nicht.
kn_quelltextart() {
    printf '%s' "$1" | grep -qE "\.($KN_QUELL_EXT)$"
}

# Dateiliste eines Verzeichnisbaums, die D-05 unterliegt — dieselbe Liste,
# ergänzt um Rust. Auch sie steht damit nur einmal im Baum.
KN_PRUEF_EXT="$KN_QUELL_EXT|rs"

kn_ist_gestaltungsquelle() {
    kn_quelltextart "$1" || return 1
    grep -qE "$KN_DEKLARATION" "$1" 2>/dev/null
}

# ── Ein Durchlauf je Datei, alle Literalklassen ─────────────────────────────
#
# Zuvor liefen bis zu neun Prozesse je Quelldatei (mehrere `grep`, ein `perl`).
# Bei einem Quellbaum in der angekündigten Größenordnung sind das Tausende je
# Commit. Ausgabe: je Fund eine Zeile `Klasse<TAB>Zeilennummer<TAB>Auszug`.
#
# Der Durchlauf liest die Datei **im Ganzen** (`-0777`), nicht zeilenweise:
# QML bricht Zuweisungen regelmäßig um, und eine zeilenweise Bedingung geht an
# ihrem Gegenstand vorbei, sobald er über eine Zeilengrenze läuft — sie meldet
# dann grün, ohne je etwas gesehen zu haben.
# `kn_literale <datei> [datei …]` — **ein** perl-Prozess für alle übergebenen
# Dateien. Zuvor startete je Datei einer; bei einem Quellbaum in der
# angekündigten Größenordnung sind das Tausende je Commit („billig vor teuer").
#
# Über `KN_NUR_KLASSE` lässt sich auf eine Klasse einschränken, statt das
# Ergebnis hinterher zu filtern — für Markdown wird sonst der vollständige
# Mehrklassendurchlauf gefahren und wieder verworfen.
#
# Der Rückgabewert wird vom Aufrufer ausgewertet: Ein Abbruch ist kein „nichts
# gefunden".
kn_literale() {
    [ "$#" -gt 0 ] || return 0
    KN_NUR_KLASSE="${KN_NUR_KLASSE:-}" perl -0777 -ne '
        my $datei = $ARGV;
        my $nur   = $ENV{KN_NUR_KLASSE} // "";
        my @zeilen = split /\n/, $_, -1;

        # In Markdown ist `#` das Überschriftenzeichen, der Vorgangsbezug
        # (Vorgangsbezug) und der Anker. Ein Farbwert wird dort
        # zitiert, nicht gesetzt. Ohne diese Ausnahme wäre die einzige
        # vorgesehene Befreiung `D-05-Ausnahme: <Grund>` — mitten im Fließtext,
        # im gerenderten Dokument sichtbar. Eine gestellte Falle für den
        # nächsten Autor.
        my $ist_md = ($datei =~ /\.md$/i);

        my $farbworte = qr/\b(?:white|black|red|green|blue|yellow|orange|purple|
                              pink|brown|gray|grey|cyan|magenta|silver|gold|
                              beige|ivory|navy|teal|olive|maroon|lime|aqua|
                              fuchsia|coral|salmon|khaki|lavender|crimson|
                              indigo|violet|turquoise|tan|plum|orchid)\b/xi;
        my $farbfunktion = qr/(?:\bQt\.(?:rgba?|hsla?|hsva|tint|darker|lighter)\s*\(
                               |(?<![\w.-])(?:rgba?|hsla?)\s*\(
                               |\.(?:darker|lighter)\s*\()/x;

        # Maß-, Abstands-, Schrift- und Bewegungsgrößen. Ergänzt gegenüber der
        # ersten Fassung um die Klassen, die DES-STM-001 ausdrücklich festlegt
        # und die durch das Raster fielen: Strichstärke (Abschnitt 5),
        # Zeilenhöhe (Abschnitt 4) und Bewegungsdauer (Abschnitt 8). Die
        # Dauerwerte hängen zusätzlich an SM-NFR-013: Ein fest geschriebenes
        # `duration: 150` umgeht den Schalter für reduzierte Bewegung.
        #
        # **`interval` steht bewusst nicht hier.** Der Takt eines Zeitgebers
        # ist Verhalten, nicht Gestaltung: DES-STM-001 Abschnitt 8 legt
        # Übergangsdauern fest, keine Abfrageintervalle. Die Klasse mit
        # aufzunehmen erzeugte Befunde gegen Werte, für die es keinen
        # Bezeichner in der Gestaltungsquelle gibt und geben soll.
        my $mass = qr/(?:radius
                      |[a-zA-Z]*[Ss]pacing
                      |[a-zA-Z]*[Pp]adding
                      |[a-zA-Z]*[Mm]argins?
                      |implicit(?:Width|Height)
                      |(?:preferred|maximum|minimum)(?:Width|Height)
                      |pixelSize|letterSpacing|lineHeight
                      |font\.(?:weight|pointSize|pixelSize)
                      |border\.width|lineWidth|penWidth
                      |cell(?:Width|Height)
                      |duration
                      |opacity|width|height)/x;

        # Methodenform unter Weg B (PySide6): `setSpacing(8)`,
        # `setFixedWidth(320)`. Der Skriptkopf sagt Neutralität gegenüber
        # beiden Wegen zu; ohne diese Form deckte die Bibliothek für `.py`
        # faktisch nur Farbliterale ab — die Zusage war breiter als ihre
        # Deckung.
        my $setter = qr/\.\s*set(?:Spacing|ContentsMargins|FixedWidth|FixedHeight|
                                  FixedSize|MinimumWidth|MinimumHeight|
                                  MaximumWidth|MaximumHeight|LineWidth|
                                  PenWidth|Duration|PointSize|PixelSize|
                                  Weight)\s*\(\s*(-?\d+(?:\.\d+)?)/x;

        my $begruendet = qr/D-05-Ausnahme:\s*\S/;

        my $melde = sub {
            my ($klasse, $nr, $text) = @_;
            return if $nur ne "" && $klasse ne $nur;
            print "$klasse\t$datei\t$nr\t$text\n";
        };

        # Eine Zahl, die einen Ausdruck **skaliert**, ist Rechnung — gleich ob
        # der Operator davor oder dahinter steht (`kn.s6 * 2` wie `2 * kn.s6`).
        # `0` ist nie ein Gestaltungswert; `1` nur bei den Anteilsgrößen
        # (Deckkraft, Skalierung), denn bei einer Länge ist 1 px der in
        # DES Abschnitt 5 festgelegte Rahmenwert.
        my $wert_ist_gestaltung = sub {
            my ($wert, $anteil) = @_;
            while ($wert =~ /(?<![\w.])(\d+(?:\.\d+)?)/g) {
                my $z = $1;
                my $davor  = substr($wert, 0, $-[1]);
                my $danach = substr($wert, $+[1]);
                next if $davor  =~ m{[*/][\s(]*$};
                next if $danach =~ m{^\s*[*/]};
                next if $z =~ /^(?:0|0\.0+)$/;
                next if $anteil && $z =~ /^(?:1|1\.0+)$/;
                return 1;
            }
            return 0;
        };

        for my $i (0 .. $#zeilen) {
            my $z = $zeilen[$i];
            my $nr = $i + 1;
            next if $z =~ $begruendet;
            my $ohne = $z; $ohne =~ s{//.*$}{};

            # **Farbe nur, wo ein Wert gesetzt wird.** Ein `#` gefolgt von
            # Hexziffern ist erst dann ein Farbliteral, wenn es hinter einer
            # Zuweisung oder in einer Zeichenkette steht. In Markdown bleiben
            # reine Dezimalfolgen ohnehin außen vor.
            my $hex = qr/\#[0-9a-fA-F]{3}(?:[0-9a-fA-F]{3})?(?:[0-9a-fA-F]{2})?\b/;
            if ($ohne =~ /$hex/) {
                my $treffer = $&;
                my $nur_ziffern = ($treffer =~ /^\#[0-9]+$/);
                my $gesetzt = ($ohne =~ /(?::|=|,|\()\s*"?\s*$hex/);
                unless ($ist_md && ($nur_ziffern || !$gesetzt)) {
                    $melde->("farbe", $nr, $z);
                }
            }
            $melde->("farbwort", $nr, $z)
                if $ohne =~ /(?:color|colour|background|fill|stroke)\s*[:=]\s*"?$farbworte/i;
            $melde->("farbfunktion", $nr, $z) if $ohne =~ /$farbfunktion/;
            $melde->("schrift", $nr, $z)
                if $ohne =~ /(?:font\.family|font-family|setFamily|QFont\()\s*[:(]?\s*"/;

            # Methodenform (Weg B).
            if ($ohne =~ /$setter/) {
                my $wert = $1;
                $melde->("mass", $nr, $z) if $wert_ist_gestaltung->($wert, 0);
            }

            # **Eigene Maßdeklaration im Bauteil.** In QML ist
            # `property int groesse: 16` die natürlichste Stelle für einen
            # Gestaltungswert — und sie trug keinen Eigenschaftsnamen aus der
            # Liste, fiel also vollständig durch.
            # Der **Name** entscheidet mit. `property int gewaehlteZeile: -1`
            # ist ein Zustandsindex, kein Maß; ihn zu melden erzeugte einen
            # Befund gegen einen Wert, für den es in der Gestaltungsquelle
            # keinen Bezeichner geben soll. Erfasst sind Namen, die eine
            # Gestaltungsgröße benennen.
            my $massname = qr/(?:groesse|größe|breite|hoehe|höhe|staerke|stärke
                              |radius|abstand|dauer|rand|einzug
                              |width|height|size|weight|duration|spacing
                              |padding|margin|opacity)/xi;
            if ($ohne =~ /\bproperty\s+(?:int|real|double)\s+(\w*$massname\w*)\s*:\s*(.+)$/) {
                my $wert = $2;
                $melde->("mass", $nr, $z) if $wert_ist_gestaltung->($wert, 0);
            }

            # **Zuweisung mit `=`.** `g.lineWidth = 2` ist eine Strichstärke;
            # DES Abschnitt 5 lässt 2 px nur beim Fokusring zu.
            if ($ohne =~ /\b$mass\s*=\s*([^=].*)$/) {
                my $wert = $1;
                my $anteil = ($ohne =~ /\b(?:opacity|scale|z)\s*=/);
                $melde->("mass", $nr, $z) if $wert_ist_gestaltung->($wert, $anteil);
            }
        }

        # Maßzuweisungen mit `:` — **über Zeilengrenzen hinweg**. QML bricht sie
        # regelmäßig um, und eine zeilenweise Bedingung meldet dann grün, ohne
        # je etwas gesehen zu haben.
        for my $i (0 .. $#zeilen) {
            my $nr = $i + 1;
            next unless $zeilen[$i] =~ /\b$mass\s*:\s*(.*)$/;
            my $wert = $1;
            my $j = $i;
            while ($j + 1 <= $#zeilen && $zeilen[$j + 1] =~ /^\s*(?:[?:+*\/]|&&|\|\||\))/) {
                $j++;
                $wert .= " " . $zeilen[$j];
            }
            # Eine begründete Ausnahme darf in jeder Zeile der Zuweisung
            # stehen — die Zuweisung ist eine Einheit.
            next if join("\n", @zeilen[$i .. $j]) =~ $begruendet;
            $wert =~ s{//.*$}{}mg;
            my $anteil = ($zeilen[$i] =~ /\b(?:opacity|scale|z)\s*:/);
            $melde->("mass", $nr, $zeilen[$i]) if $wert_ist_gestaltung->($wert, $anteil);
        }
    ' "$@" || return 2
}

kn_klassentext() {
    case "$1" in
        farbe)        echo "Farbliteral" ;;
        farbwort)     echo "benannte Farbe" ;;
        farbfunktion) echo "Farbfunktion oder Farbfaktor" ;;
        schrift)      echo "Schriftliteral" ;;
        mass)         echo "Abstands-, Maß- oder Schriftgradliteral" ;;
        *)            echo "Gestaltungsliteral" ;;
    esac
}
