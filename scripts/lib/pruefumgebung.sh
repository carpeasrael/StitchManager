# shellcheck shell=bash
# ── Gemeinsame Umgebungsprüfungen der Selbsttests ───────────────────────────
#
# **Warum diese Datei existiert.** Dieselbe Frage — „womit begrenze ich die
# Laufzeit eines Prüffalls?" — stand in `check-projektregeln.test.sh` und
# `check-qml.test.sh` je einmal, wortgleich samt Begründung und Bezugshinweis.
# CLAUDE.md Abschnitt 11 benennt genau das als unzulässig: „Dieselbe Frage
# zweimal zu beantworten führt … zu zwei Wahrheiten." Dieselbe Erwägung hat
# `scripts/lib/gestaltung.sh` und `scripts/lib/dateien.sh` hervorgebracht.

# Setzt `KN_ZEITGRENZE` auf den absoluten Pfad von `timeout` oder `gtimeout`.
#
# **`timeout` gehört auf macOS nicht zum Grundsystem.** Ohne diese Prüfung
# lieferte *jeder* Prüffall 127, der Selbsttest wäre rot, Stufe 0b blockierte
# jeden Commit — und die Ausgabe zeigte auf den Prüfling statt auf das fehlende
# Werkzeug (S1/S3).
#
# **Absolut aufgelöst**, weil einzelne Fälle den Suchpfad verstellen: Ein
# Schattenpfad ohne `timeout` ließe den Aufruf sonst ins Leere laufen.
kn_zeitgrenze() {  # $1 = Name des Selbsttests für die Meldung
    if command -v timeout >/dev/null 2>&1; then
        KN_ZEITGRENZE="$(command -v timeout)"
    elif command -v gtimeout >/dev/null 2>&1; then
        KN_ZEITGRENZE="$(command -v gtimeout)"
    else
        echo "${1:-Selbsttest}: FAIL — weder 'timeout' noch 'gtimeout' vorhanden."
        echo "  Unter macOS: brew install coreutils (liefert gtimeout)."
        return 1
    fi
    return 0
}
