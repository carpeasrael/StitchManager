# shellcheck shell=bash
# ── Gemeinsame Dateiliste der Prüfskripte ───────────────────────────────────
#
# **Warum diese Datei existiert.** Dieselbe Frage — „welche Dateien gehören zum
# Baum?" — wurde zuvor in `check-docs.sh`, `check-projektregeln.sh` und
# `check-qml.sh` je einmal beantwortet. CLAUDE.md Abschnitt 11 benennt genau das
# als unzulässig: „Dieselbe Frage zweimal zu beantworten führt … zu zwei
# Wahrheiten." Dieselbe Erwägung hat `scripts/lib/gestaltung.sh` hervorgebracht.
#
# Wer diese Datei ändert, ändert den Prüfumfang. Sie steht deshalb in der
# Gate-Signatur; ein geänderter Bereich gibt kein altes Ergebnis aus dem Cache.
#
# **Drei Festlegungen, die zusammen eine Antwort ergeben:**
#
#  · `--cached --others --exclude-standard` — der **Arbeitsbaum**, nicht nur der
#    Index, aber ohne alles, was `.gitignore` ausschließt. Ein erzeugtes
#    Erzeugnis unter `target/` kann damit keine Prüfung ohne begehbaren Weg
#    blockieren (S1), und der Abstieg in ein gebautes Bauverzeichnis mit
#    fünfstelliger Eintragszahl entfällt.
#  · `core.quotePath=false` — sonst liefert git Nicht-ASCII-Pfade escapt
#    (`"a/\303\234b.md"`), und jede Datei mit Umlaut fiele lautlos aus der
#    Prüfung (SM-NFR-010).
#  · `sort -u` — `--cached` und `--others` können dieselbe Datei nennen.

# ── Arbeitsbaumprüfung, einmal beim Einbinden ────────────────────────────────
#
# **Sie steht auf Skriptebene, nicht in der Funktion.** Sie startet einen
# eigenen git-Prozess, und die Aufrufer rufen `kn_dateien` ausnahmslos in
# Kommandoersetzungen und Pipelines auf — also in Unterschalen. Eine Zuweisung
# *innerhalb* der Funktion ginge dort jedes Mal verloren, und `git rev-parse`
# liefe je Aufruf. Genau diesen Fehler dokumentiert `check-docs.sh` für seine
# eigenen Listen als behoben; er darf hier nicht wieder entstehen.
if git rev-parse --is-inside-work-tree >/dev/null 2>&1; then
    KN_ARBEITSBAUM=ja
else
    KN_ARBEITSBAUM=nein
fi

# Gibt die Dateien des Arbeitsbaums aus, gefiltert über die übergebenen
# Pfadangaben (git-Pathspecs). Ohne Angabe: alle.
#
# **Rückgabewert 1, wenn kein Git-Arbeitsbaum vorliegt.** Ein unterdrückter
# git-Fehler ergäbe eine leere Liste, und die riefe bei jedem Aufrufer
# „Gegenstand nicht vorhanden" hervor — ENTFÄLLT statt FAIL, obwohl der Baum
# voll davon ist. Das ist genau die Verwechslung, die S3 ausschließt. **Jeder**
# Aufrufer wertet diesen Rückgabewert aus; ein Aufrufer, der ihn verwirft, macht
# aus der einen Antwort im Baum wieder zwei Lesarten.
#
# **Auch jeder andere git-Fehler ist FAIL.** Zuvor wurde nur „kein Arbeitsbaum"
# ausgewertet; ein beschädigter Index, eine unlesbare `.gitignore` oder ein
# abgebrochenes `ls-files` ergaben eine leere Liste bei Rückgabewert 0 — der
# Aufrufer sähe einen leeren Prüfbereich und meldete grün. Der Status wird
# deshalb **vor** dem Sortieren eingefangen; eine Pipeline verdeckte ihn.
kn_dateien() {  # $1… = Pathspecs
    kn_arbeitsbaum || return 1
    local roh
    if [ "$#" -eq 0 ]; then
        roh="$(git -c core.quotePath=false ls-files \
                 --cached --others --exclude-standard 2>/dev/null)" || return 1
    else
        roh="$(git -c core.quotePath=false ls-files \
                 --cached --others --exclude-standard -- "$@" 2>/dev/null)" || return 1
    fi
    printf '%s\n' "$roh" | sort -u | grep -v '^$' || true
}

# Sagt, ob überhaupt ein Git-Arbeitsbaum vorliegt.
#
# **Für Aufrufer, die nur das wissen wollen.** Wer dafür `kn_dateien >/dev/null`
# schreibt, zieht einen vollständigen Abzug des Baums samt `sort` und wirft ihn
# weg — je Lauf beider Prüfskripte, im Selbsttest der Stufe 0b also einmal je
# Prüffall. Die Antwort steht seit dem Einbinden fest (CLAUDE.md Abschnitt 13,
# „billig vor teuer").
kn_arbeitsbaum() { [ "$KN_ARBEITSBAUM" = ja ]; }
