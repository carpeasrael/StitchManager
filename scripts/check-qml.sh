#!/usr/bin/env bash
# Prüfung der QML-Oberflächenschicht (CLAUDE.md Abschnitt 11, Zeile „QML").
#
# Rückgabewerte nach der Übereinkunft der hauseigenen Skripte:
#   0  PASS       — geprüft, kein Befund
#   1  FAIL       — Befund, oder der Gegenstand ist da und ein Werkzeug fehlt
#   3  ENTFÄLLT   — kein `*.qml` im Baum (S3)
#
# **Warum ENTFÄLLT und nicht PASS, wenn nichts da ist:** Die
# Anwendbarkeitstabelle in CLAUDE.md Abschnitt 13 führt das QML-Gate mit
# „`*.qml` im Baum → sonst ENTFÄLLT"; ein nicht durchgeführter Test ist kein
# bestandener Test.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT" || exit 1

WORKTMP="$(mktemp -d)"
trap 'rm -rf "$WORKTMP"' EXIT

# ── Dateiauswahl ─────────────────────────────────────────────────────────────
#
# **Baumweit, und über `kn_dateien` aus scripts/lib/dateien.sh.** Der Gegenstand
# ist nach der Anwendbarkeitstabelle „`*.qml` **im Baum**"; eine Auswahl entlang
# des `oberflaeche`-Pfads meldete PASS, während eine QML-Datei daneben ungeprüft
# bliebe. Die Bibliothek trägt die drei Festlegungen dazu (Arbeitsbaum ohne
# Ignoriertes, Nicht-ASCII-Pfade, Doppelnennungen) — einmal für alle drei
# Prüfskripte.
if [ -r "$ROOT/scripts/lib/dateien.sh" ]; then
  # shellcheck source=lib/dateien.sh
  . "$ROOT/scripts/lib/dateien.sh"
else
  echo "QML-Prüfung: FAIL — scripts/lib/dateien.sh fehlt, die Dateiliste ist nicht bildbar (S3)."
  exit 1
fi

# Gelesen wird in einer Schleife, nicht mit dem Sammelbefehl aus bash 4: macOS
# liefert bash 3.2 aus, und Fall H5 in `scripts/review-gate.test.sh` hält den
# Baum bewusst frei von bash-4-Konstrukten.
#
# **Ein gescheitertes `kn_dateien` ist FAIL, nicht ENTFÄLLT.** Ohne diese
# Unterscheidung ergäbe ein fehlender Git-Arbeitsbaum eine leere Liste — und die
# meldete „kein *.qml im Baum", obwohl der Baum voll davon ist. Genau die
# Verwechslung, die S3 ausschließt.
DATEIEN=()
liste="$(kn_dateien '*.qml')" || {
  echo "QML-Prüfung: FAIL — kein Git-Arbeitsbaum, die Dateiliste ist nicht bildbar."
  echo "  Ein nicht durchgeführter Test ist kein bestandener Test (S3)."
  echo "  Behebung: aus einem Klon heraus aufrufen, nicht aus einem entpackten Archiv."
  exit 1
}
while IFS= read -r f; do
  [ -n "$f" ] && DATEIEN+=("$f")
done <<< "$liste"

if [ "${#DATEIEN[@]}" -eq 0 ]; then
  echo "QML-Prüfung: kein *.qml im Baum — ENTFÄLLT."
  exit 3
fi

werkzeug_fehlt() {  # $1 = Name
  echo "QML-Prüfung: $1 fehlt, aber ${#DATEIEN[@]} QML-Datei(en) liegen im Baum."
  echo "Bezugsweg: Qt 6 mit den Entwicklungswerkzeugen installieren — etwa"
  echo "  'brew install qt' oder 'apt install qt6-declarative-dev'; $1 gehört dazu."
  echo "Ein vorhandener Gegenstand ohne Werkzeug ist FAIL, nicht ENTFÄLLT (S3)."
}

# **Beide Werkzeuge werden vorab geprüft, nicht erst beim Aufruf.** Sonst
# scheiterten die Fälle des Formatteils am Prüfling, und die Ausgabe zeigte auf
# ihn statt auf die fehlende Fassung — dieselbe Vorkehrung, die der Selbsttest
# für `timeout` trifft.
for w in qmllint qmlformat; do
  if ! command -v "$w" >/dev/null 2>&1; then
    werkzeug_fehlt "$w"
    exit 1
  fi
done

echo "QML-Prüfung: ${#DATEIEN[@]} Datei(en), $(qmllint --version 2>&1 | tr -d '\n')"

# ── 1 · qmllint ──────────────────────────────────────────────────────────────
#
# **Die erzeugten QML-Module gehören auf den Suchpfad.** `Musterliste` entsteht
# aus `crates/ui/src/bruecke.rs`; ohne das von cxx-qt erzeugte
# `plugin.qmltypes` sähe qmllint den Typ nicht und meldete jede Verwendung als
# unaufgelöst. Übergeben wird **je Modulname genau ein Verzeichnis** — das
# jüngste; mehrere verschiedene Module ergeben mehrere `-I`. Die Begründung
# steht bei der Auswahl weiter unten.
# **Der Suchpfad steht in einer Variablen, damit die Meldung ihn nennen kann.**
# Ist `CARGO_TARGET_DIR` gesetzt — der Grund, aus dem die Variable überhaupt
# beachtet wird —, suchte der Nutzer sonst im falschen Verzeichnis und führte
# ein `cargo build` aus, das er schon ausgeführt hat (SM-NFR-006).
SUCHPFAD="${CARGO_TARGET_DIR:-$ROOT/target}"
# **Ausgegeben wird der Pfad relativ zur Wurzel.** SM-SEC-010 hält Protokolle
# frei von unmaskierten vollständigen Pfaden; die Gate-Ausgabe landet über
# `run_gate` genau dort. Liegt der Pfad außerhalb der Wurzel — bei gesetztem
# `CARGO_TARGET_DIR` möglich —, bleibt er absolut, weil eine relative Angabe
# dann nicht mehr auffindbar wäre; der Nutzer hat ihn in diesem Fall selbst
# gesetzt.
ANZEIGEPFAD="${SUCHPFAD#"$ROOT"/}"

# **Je Modulnamen genau ein Verzeichnis, und zwar das jüngste.**
#
# Ein gebauter Baum trägt das erzeugte Modul mehrfach: je Bauprofil und je
# Baulauf ein eigenes `qml_modules` mit demselben Modulnamen — im
# Entwicklungsbaum waren es vier für `de/stitchmanager`. Übergäbe man alle als
# `-I`, entschiede die Fundreihenfolge, welche Typbeschreibung gilt, und die
# hängt am Bau-Hash, nicht an der Aktualität. Die Wirkungsrichtung ist genau
# die, gegen die dieses Gate angelegt ist: Wird eine Rolle aus der Brücke
# entfernt, die eine QML-Datei weiter liest, führt ein **älterer**
# `plugin.qmltypes` sie noch — das Gate meldete PASS über die Regression, die
# es finden soll. Umgekehrt erzeugte ein zu alter Stand Falschbefunde. Und ein
# frisch geklonter Baum (ein Verzeichnis) käme zu einem anderen Ergebnis als
# ein Entwicklungsgerät (vier): dasselbe Gate, zwei Wahrheiten.
#
# `-prune` beschneidet den Abstieg: Ohne ihn liefe `find` zusätzlich durch
# jedes gefundene Modulverzeichnis, und `target/` trägt bei zwei gebauten
# Profilen eine fünfstellige Eintragszahl (CLAUDE.md Abschnitt 13, „billig vor
# teuer").
# **Die teuren Zweige werden benannt, nicht über die Tiefe gehofft.** Sie liegen
# flacher als jede sinnvolle Tiefengrenze: `<profil>/deps` auf Ebene 3,
# `<profil>/incremental/<kiste>/<sitzung>` auf Ebene 5. Bei zwei gebauten
# Profilen sind das sechsstellig viele Einträge je Lauf, und Stufe 0c läuft bei
# jedem Änderungssatz mit Codebezug. Die erzeugten Module liegen bauartbedingt
# auf sechs Ebenen (`<profil>/build/<kiste>-<hash>/out/qt-build-utils/qml_modules`);
# acht lässt Luft.
find "$SUCHPFAD" -maxdepth 8 \
     -type d \( -name deps -o -name incremental -o -name .fingerprint \) -prune -o \
     -type d -name qml_modules -prune -print 2>/dev/null \
  | sort > "$WORKTMP/modulwurzeln"

: > "$WORKTMP/qmltypes"
while IFS= read -r w; do
  [ -n "$w" ] || continue
  find "$w" -name plugin.qmltypes -type f 2>/dev/null >> "$WORKTMP/qmltypes"
done < "$WORKTMP/modulwurzeln"

# **Sortiert wird je Modulname, nicht über die Gesamtliste.** `xargs` teilt eine
# überlange Argumentliste auf; `ls -t` sortierte dann nur innerhalb eines
# Stapels, und über die Stapel hinweg gälte wieder die Fundreihenfolge — genau
# das Fehlerbild, gegen das die Auswahl angelegt ist. Eine feste Obergrenze
# löste das nicht: Maßgeblich ist die Byte-Länge, nicht die Anzahl, und ein
# `cargo clean` als einziger Ausweg wäre ein voller Neubau als Preis für ein
# Formatgate. Je Modulname bleibt die Liste durch die Zahl der Stände **eines**
# Moduls beschränkt.
IMPORTPFAD=()
MEHRFACH=0
: > "$WORKTMP/gewaehlte_typdateien"
if [ -s "$WORKTMP/qmltypes" ]; then
  # Modulname ist der Pfad unterhalb von `qml_modules`, etwa `de/stitchmanager`.
  sed 's|.*/qml_modules/||; s|/plugin\.qmltypes$||' "$WORKTMP/qmltypes" \
    | sort -u > "$WORKTMP/modulnamen"
  gewaehlt=""
  while IFS= read -r modul; do
    [ -n "$modul" ] || continue
    : > "$WORKTMP/stand"
    while IFS= read -r f; do
      case "$f" in */qml_modules/"$modul"/plugin.qmltypes) printf '%s\n' "$f" >> "$WORKTMP/stand" ;; esac
    done < "$WORKTMP/qmltypes"
    anzahl="$(wc -l < "$WORKTMP/stand" | tr -d ' ')"
    [ "${anzahl:-0}" -gt 1 ] && MEHRFACH=$((MEHRFACH + anzahl - 1))
    # `ls -t` sortiert nach Änderungszeit, jüngste zuerst — portabel, anders als
    # `find -printf` oder `stat`, die sich zwischen BSD und GNU unterscheiden.
    # Bei **gleicher** Änderungszeit ist die Reihenfolge nicht festgelegt; zwei
    # gleich alte Stände desselben Moduls hatten denselben Bauschritt.
    juengste="$(tr '\n' '\0' < "$WORKTMP/stand" | xargs -0 ls -t 2>/dev/null | head -1)"
    [ -n "$juengste" ] || continue
    printf '%s\n' "$juengste" >> "$WORKTMP/gewaehlte_typdateien"
    wurzel="${juengste%/qml_modules/*}/qml_modules"
    case " $gewaehlt " in
      *" $wurzel "*) ;;
      *) gewaehlt="$gewaehlt $wurzel"; IMPORTPFAD+=(-I "$wurzel") ;;
    esac
  done < "$WORKTMP/modulnamen"
fi

if [ "$MEHRFACH" -gt 0 ]; then
  # **Sichtbar, nicht still.** Veraltete Baureste sind kein Befund am QML, aber
  # der Leser des Protokolls muss wissen, dass eine Auswahl stattgefunden hat.
  echo "  ! $MEHRFACH überholte(s) Modulverzeichnis(se) übergangen — je Modulname"
  echo "    gilt das jüngste. 'cargo clean' entfernt die Baureste."
fi

if [ "${#IMPORTPFAD[@]}" -eq 0 ]; then
  echo "  ! Kein erzeugtes QML-Modul unter $ANZEIGEPFAD gefunden."
  # **Der Rat hängt am Vorhandensein eines Cargo-Projekts.** Unter Weg B läge
  # QML im Baum und kein `target/`; ein unbedingtes „zuerst cargo build" wäre
  # dort ein Ratschlag, der ins Leere zeigt (S1, OP-13/Wegwahl).
  if [ -f "$ROOT/Cargo.toml" ]; then
    echo "    Zuerst 'cargo build' ausführen. Ohne das von cxx-qt erzeugte"
    echo "    plugin.qmltypes meldet qmllint die Brückentypen als fehlenden"
    echo "    Import — ein Befund, der nicht der geprüften Datei gilt."
  else
    echo "    Ohne die Typbeschreibung der Oberflächenbrücke meldet qmllint"
    echo "    deren Typen als fehlenden Import — ein Befund, der nicht der"
    echo "    geprüften Datei gilt."
  fi
fi

# **Die Brückentypen werden abgeleitet, nicht verdrahtet.** Aus den oben
# ausgewählten `plugin.qmltypes` kommen die Typnamen, die cxx-qt dort ausführt.
# Sie sind der einzige Grund, aus dem qmllint hier etwas nicht auflösen kann —
# und der Name steht damit im Baum, nicht im Skript. Käme eine zweite Brücke
# hinzu, wüchse die Liste von selbst mit.
# **Gelesen werden die oben gewählten Dateien, nicht die Wurzeln erneut.**
# Ein zweites `find` über die Wurzeln nähme auch Module mit, deren jüngster
# Stand in einer anderen Wurzel liegt — ein überholter Typstand erweiterte die
# Ausnahmeliste dann um einen Namen, den es nicht mehr gibt, und der Filter
# schluckte eine echte Meldung. Dieselbe Auswahl, eine Antwort.
BRUECKENTYPEN=""
while IFS= read -r t; do
  [ -n "$t" ] || continue
  BRUECKENTYPEN="${BRUECKENTYPEN}${BRUECKENTYPEN:+|}$t"
done < <(
  if [ -s "$WORKTMP/gewaehlte_typdateien" ]; then
    while IFS= read -r f; do
      [ -n "$f" ] || continue
      sed -n 's/.*exports: \["[^/]*\/\([A-Za-z_][A-Za-z0-9_]*\) .*/\1/p' "$f"
    done < "$WORKTMP/gewaehlte_typdateien" | sort -u
  fi
)

# `"${ARR[@]}"` auf einem leeren Array bricht unter `set -u` in bash 3.2 ab.
# Die Form `${ARR[@]+"${ARR[@]}"}` liefert dort eine leere Argumentliste.
#
# **Die Ausgabe wird einmal weggeschrieben, nicht je Befund neu durchsucht.**
# Ein `grep` je Befundzeile über die Gesamtausgabe kostet B Prozesse und B·A
# gelesene Zeilen; bei einem Massenbefund über acht Dateien sind das dreistellig
# viele Prozesse im Fehlerpfad des Gates. Und `grep -F` träfe bei mehrfach
# gleichlautender Meldung stets das **erste** Vorkommen — der abgedruckte
# Quellkontext gehörte dann zu einer anderen Datei als der Befund darüber.
# **`--` vor der Dateiliste.** Ein Dateiname mit führendem Bindestrich käme
# sonst als Schalter an — die Liste stammt aus dem Baum, also aus Fremddaten.
qmllint ${IMPORTPFAD[@]+"${IMPORTPFAD[@]}"} -- "${DATEIEN[@]}" > "$WORKTMP/lint.out" 2>&1
QMLLINT_RC=$?

MELDUNGEN="$(grep -cE '^(Warning|Error)' "$WORKTMP/lint.out" || true)"

# **Ein Werkzeugfehler ist kein bestandener Lauf.** Bricht qmllint ab, ohne
# eine einzige Meldung zu schreiben — falscher Schalter, fehlende Bibliothek,
# geändertes Ausgabeformat einer neueren Fassung —, stünde ohne diese Abfrage
# ein grünes Gate im Protokoll, das nichts geprüft hat. Fail-closed nach
# CLAUDE.md Abschnitt 13.
if [ "$QMLLINT_RC" -ne 0 ] && [ "${MELDUNGEN:-0}" -eq 0 ]; then
  cat "$WORKTMP/lint.out"
  echo
  echo "QML-Prüfung: qmllint brach mit Rückgabewert $QMLLINT_RC ab, ohne eine"
  echo "  erkennbare Meldung zu schreiben — oder in einem unbekannten Format."
  echo "  Erwartet werden Zeilen ab 'Warning:' oder 'Error:' (geprüft gegen 6.11)."
  exit 1
fi

# ── Befunde herausfiltern und mit Kontext ausgeben ──────────────────────────
#
# **Eine Ausnahme, als Muster — keine abgeschaltete Prüfart.**
# cxx-qt schreibt in `plugin.qmltypes` die C++-Schreibweise seiner Typen
# (`::std::int32_t`, `::rust::cxxqt1::CxxQtThreading<…>`). qmllint kennt diese
# Schreibweise nicht und meldet daraufhin zweierlei: die Typen selbst als
# unaufgelöst, und den Brückentyp, dessen Grundtyp darunter liegt. Beides prüft
# der Rust- und C++-Übersetzer; die Prüfung geht nicht verloren, sie liegt
# woanders. Eine Prüfart ganz abzuschalten hieße dagegen, einen echten Befund
# derselben Art nicht mehr zu sehen — eine fehlende `import`-Zeile ebenso wie
# einen wirklich unauflösbaren Eigenschaftstyp.
#
# **Die Ausnahme greift doppelt eingegrenzt:** nur bei einem doppelten
# Doppelpunkt (C++-Geltungsbereich, in QML kein gültiger Typname) oder bei einem
# der oben **abgeleiteten** Brückentypen — **und** nur in den Arten
# `[unresolved-type]` und `[import]`, in denen diese Typen überhaupt auftauchen.
# Ohne die zweite Eingrenzung verwürfe sie jede Meldung, die einen
# qualifizierten Qt-Typ zitiert (`Qt::Alignment`). Belegt ist beides — Enge und
# Wirkung — in der Fallgruppe zur cxx-qt-Ausnahme in `check-qml.test.sh`; die
# Fallnamen stehen dort, damit eine Umbenennung hier keinen toten Verweis
# hinterlässt.
#
# **Verglichen wird der Meldungstext, nicht die ganze Zeile.** Vorn stehen Datei,
# Zeile und Spalte; ein Pfad, der zufällig `::` enthält, machte den Filter sonst
# breiter, als er begründet ist.
#
# **Filter und Kontext laufen in einem Durchlauf.** Ein `grep` je Meldungszeile
# und ein `sed` je Befund ergäben bei einem Massenbefund über acht Dateien
# dreistellig viele Prozesse im Fehlerpfad. `awk` liest die Datei einmal,
# entscheidet je Zeile und druckt die beiden Folgezeilen (Quellzeile und Zeiger)
# gleich mit.
awk -v typen="$BRUECKENTYPEN" '
  function ist_meldung(z) { return z ~ /^(Warning|Error): / }
  {
    zeilen[NR] = $0
  }
  END {
    befunde = 0
    for (n = 1; n <= NR; n++) {
      z = zeilen[n]
      if (!ist_meldung(z)) continue
      # Ortsanteil abtrennen: "Warning: datei:zeile:spalte: text"
      text = z
      sub(/^(Warning|Error): /, "", text)
      # Der Ortsanteil endet am **letzten** ":<zeile>:<spalte>: " vor dem Text.
      # Ein Muster ohne Leerzeichen (`[^ ]*`) verfehlte jeden Pfad mit
      # Leerzeichen; er bliebe dann Teil des Textes, und ein Pfad, der ein `::`
      # enthält, machte die Ausnahme breiter, als sie begründet ist.
      while (match(text, /^.*:[0-9]+:[0-9]+: /)) {
        text = substr(text, RSTART + RLENGTH)
      }
      art = (text ~ /\[(unresolved-type|import)\]$/)
      ausnahme = (text ~ /::/)
      if (!ausnahme && typen != "" \
          && text ~ ("Type (" typen ") is used but it is not resolved")) ausnahme = 1
      if (art && ausnahme) continue
      befunde++
      print z
      if (n + 1 <= NR) print zeilen[n + 1]
      if (n + 2 <= NR) print zeilen[n + 2]
    }
  }
' "$WORKTMP/lint.out" > "$WORKTMP/befunde.out" 2> "$WORKTMP/befunde.fehler"
AWK_RC=$?

# **Ein abgebrochener Filterlauf ist FAIL, nie PASS.** Ohne diese Abfrage
# meldete das Gate „ohne Befund", obwohl qmllint bereits Meldungen geschrieben
# hat — der Filter dazwischen wäre der einzige Schritt ohne diese Vorkehrung.
if [ "$AWK_RC" -ne 0 ]; then
  cat "$WORKTMP/befunde.fehler"
  echo
  echo "QML-Prüfung: der Filterlauf brach mit Rückgabewert $AWK_RC ab."
  echo "  qmllint hatte ${MELDUNGEN:-0} Meldung(en) geschrieben; ungeprüft ist"
  echo "  nicht bestanden (S3)."
  exit 1
fi

# **Die Zahl kommt aus der Ausgabe selbst, nicht über einen zweiten Kanal.**
# Jeder Befund beginnt mit seiner Meldungszeile; sie zu zählen braucht weder
# eine Hilfsdatei noch eine `awk`-Fassung, die Dateinamen im `print` versteht.
BEFUNDZAHL="$(grep -cE '^(Warning|Error): ' "$WORKTMP/befunde.out" || true)"
BEFUNDZAHL="${BEFUNDZAHL:-0}"

if [ "$BEFUNDZAHL" -gt 0 ]; then
  cat "$WORKTMP/befunde.out"
  echo
  echo "QML-Prüfung: $BEFUNDZAHL Befund(e)."
  # **Der Hinweis steht am Anfang und am Ende.** Ins Protokoll übernimmt
  # `run_gate` bei FAIL nur die letzten Zeilen der Ausgabe; ein Hinweis, der
  # ganz oben steht, fällt bei mehr als einer Handvoll Befunden heraus, und der
  # Leser sucht am falschen Ort (SM-NFR-006).
  if [ "${#IMPORTPFAD[@]}" -eq 0 ]; then
    echo "  Hinweis: kein erzeugtes QML-Modul unter $ANZEIGEPFAD."
    echo "  Die Brückentypen gelten sonst als unaufgelöst; die Befunde oben können daher rühren."
  fi
  exit 1
fi
echo "  ✓ qmllint ohne Befund"

# ── 2 · qmlformat ────────────────────────────────────────────────────────────
#
# **Ohne `-n`, und das ist eine Festlegung, keine Nachlässigkeit.** Der
# Schalter *normalize* sortiert die Eigenschaften eines Objekts alphabetisch.
# Auf die Gestaltungsquelle angewandt zerreißt er die Gliederung nach den
# Abschnitten von DES-STM-001 — Farben nach 3.1/3.2, Grundraster nach 5 — und
# löst jeden Begründungskommentar von dem Wert, zu dem er gehört. Die Quelle
# verlöre damit die Nachvollziehbarkeit gegen das Gestaltungsdokument, um
# derentwillen SM-DES-003 sie zu **einer** Datei macht.
#
# Ohne den Schalter vereinheitlicht `qmlformat` Einrückung, Leerraum und
# Strichpunkte — genau das, was nach CLAUDE.md Abschnitt 11 im Review nicht zu
# diskutieren ist. Entschieden am 26.08.2026,
# `Analysis/20260826_02_bestandsaufnahme.md`.
#
# `qmlformat` kennt keinen Prüfmodus; der Vergleich gegen die formatierte
# Fassung ist der übliche Weg.
# **Abbruch und Abweichung sind zu trennen.** Verwürfe man Rückgabewert und
# Fehlerausgabe, sähe ein abgebrochenes `qmlformat` genauso aus wie eine
# unformatierte Datei: Der Nutzer läse „nicht formatiert" und führte
# `qmlformat -i` aus — einen Befehl, der die Ursache nicht berührt. Dieselbe
# Trennung wie oben bei `qmllint`; belegt in der Fallgruppe zum
# Werkzeugabbruch in `check-qml.test.sh`.
abweichend=0
gescheitert=0
for f in "${DATEIEN[@]}"; do
  fehlerausgabe="$WORKTMP/fmt.err"
  if ! qmlformat -- "$f" > "$WORKTMP/fmt.out" 2> "$fehlerausgabe"; then
    echo "  ✗ qmlformat brach ab bei: $f"
    sed 's/^/      /' "$fehlerausgabe" | head -3
    gescheitert=$((gescheitert + 1))
    continue
  fi
  if ! diff -q "$WORKTMP/fmt.out" "$f" >/dev/null 2>&1; then
    echo "  ✗ nicht formatiert: $f"
    abweichend=$((abweichend + 1))
  fi
done
if [ "$gescheitert" -gt 0 ]; then
  echo
  echo "QML-Prüfung: qmlformat brach bei $gescheitert Datei(en) ab."
  echo "  Das ist kein Formatbefund — 'qmlformat -i' behebt es nicht."
  exit 1
fi
if [ "$abweichend" -gt 0 ]; then
  echo
  echo "QML-Prüfung: $abweichend Datei(en) weichen vom Format ab."
  echo "Behebung: qmlformat -i <datei>"
  exit 1
fi
echo "  ✓ qmlformat ohne Abweichung"
exit 0
