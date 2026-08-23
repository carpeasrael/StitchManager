#!/usr/bin/env bash
# Installiert die Git-Hooks des Commit-Freigabe-Gates.
#   bash scripts/install-hooks.sh              installieren
#   bash scripts/install-hooks.sh --uninstall  entfernen
#   bash scripts/install-hooks.sh --status     Zustand anzeigen
#
# Die Hooks sind client-seitig und je Klon von Hand zu installieren. Sie erzeugen
# einen Sorgfaltsnachweis, keinen Freigabenachweis — siehe CLAUDE.md.

set -uo pipefail

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || { echo "Kein Git-Repository." >&2; exit 2; }
GIT_COMMON="$(git rev-parse --git-common-dir)"
case "$GIT_COMMON" in /*) ;; *) GIT_COMMON="$ROOT/$GIT_COMMON" ;; esac
HOOKS="$GIT_COMMON/hooks"
MARKER="# stitchmanager-review-gate"
MODE="${1:-install}"

hook_path() { printf '%s/%s' "$HOOKS" "$1"; }

show_status() {
  local h p
  for h in pre-commit pre-push post-commit; do
    p="$(hook_path "$h")"
    if [ -f "$p" ] && grep -q "$MARKER" "$p" 2>/dev/null; then echo "  ✓ $h  (Gate)"
    elif [ -f "$p" ]; then echo "  ! $h  (fremder Hook, unangetastet)"
    else echo "  – $h  (nicht installiert)"; fi
  done
}

if [ "$MODE" = "--status" ]; then echo "Hooks in $HOOKS:"; show_status; exit 0; fi

if [ "$MODE" = "--uninstall" ]; then
  for h in pre-commit pre-push post-commit; do
    p="$(hook_path "$h")"
    if [ -f "$p" ] && grep -q "$MARKER" "$p" 2>/dev/null; then
      rm -f "$p"; echo "  entfernt: $h"
      [ -f "$p.vor-gate" ] && { mv "$p.vor-gate" "$p"; echo "  vorheriger Hook wiederhergestellt: $h"; }
    fi
  done
  exit 0
fi

[ -x "$ROOT/scripts/review-gate.sh" ] || { echo "scripts/review-gate.sh fehlt oder ist nicht ausführbar." >&2; exit 2; }
mkdir -p "$HOOKS"

write_hook() {   # $1 = Name, $2 = Rumpf
  local p; p="$(hook_path "$1")"
  if [ -f "$p" ] && ! grep -q "$MARKER" "$p" 2>/dev/null; then
    mv "$p" "$p.vor-gate"
    echo "  vorhandener Hook gesichert: $1 → $1.vor-gate"
  fi
  printf '%s\n' "$2" > "$p"
  chmod +x "$p"
  echo "  installiert: $1"
}

write_hook pre-commit "#!/usr/bin/env bash
$MARKER — Stufen 0, 0b, 0c und 1 über den Index.
set -uo pipefail
root=\"\$(git rev-parse --show-toplevel)\"
REVIEW_GATE_TIER=commit exec bash \"\$root/scripts/review-gate.sh\""

write_hook pre-push "#!/usr/bin/env bash
$MARKER — zusätzlich Stufe 2 über merge-base(main)..HEAD.
set -uo pipefail
root=\"\$(git rev-parse --show-toplevel)\"
REVIEW_GATE_TIER=push exec bash \"\$root/scripts/review-gate.sh\""

write_hook post-commit "#!/usr/bin/env bash
$MARKER — Nachweisbindung: Commit im Protokoll nachtragen.
set -uo pipefail
root=\"\$(git rev-parse --show-toplevel)\"
bash \"\$root/scripts/review-gate.sh\" __record-commit || true"

cat <<'HINT'

Installiert. Hinweise:
  · Das Gate ist client-seitig und hat dokumentierte Ausstiege (CLAUDE.md, Notfall-Ausstiege).
  · Stufe 1 startet vier Reviewer über die claude-CLI; Zeitbudget je Reviewer AGENT_TIMEOUT (900 s).
  · Trockenlauf ohne Reviewer (Stufen 0 bis 0c, kein Protokoll, keine Freigabe):
      GATE_DRY_RUN=1 bash scripts/review-gate.sh
  · Zustand prüfen:  bash scripts/install-hooks.sh --status
HINT
