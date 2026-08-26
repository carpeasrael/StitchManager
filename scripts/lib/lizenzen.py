#!/usr/bin/env python3
"""Lizenz- und Herkunftsprüfung der npm-Abhängigkeitskette.

SM-OSS-009 (Lizenzen) und SM-OSS-011 (Herkunft und Bauweg).
Aufruf aus ``scripts/check-docs.sh``; Ausgabe je Befund **eine** Zeile
``LIZENZ<TAB>text`` oder ``HERKUNFT<TAB>text``.

Eigene Datei, nicht eingebettet: Ein Python-Rumpf im Innern einer
Kommandosubstitution bringt Bashs Klammer- und Anführungszählung
durcheinander -- das Skript war dadurch nicht mehr parsebar.

**Rückgabewerte.** 0 heißt „geprüft" -- unabhängig davon, ob Befunde
anfielen. 2 heißt „nicht durchführbar" und ist für den Aufrufer ein
Befund, kein Übergehen: Ein Abbruch, der nur eine leere Ausgabe
hinterlässt, sähe von außen wie „nichts zu beanstanden" aus, und die
Meldung „N Pakete geprüft" wäre unwahr (S3, CLAUDE.md Abschnitt 13).
"""
import json, re, sys

# Die Positivliste steht in `.lizenzen.conf`, nicht hier: Jede Erweiterung
# trägt ihre Begründung neben dem Eintrag, und die Prüflogik bleibt unberührt.
erlaubt = set()
# Begründete Herkunfts-Ausnahmen. Ohne sie hätte die Herkunftsprüfung keinen
# begehbaren Weg (S1): Wer hinter einer Firmenspiegelung arbeitet oder ein
# Lockfile älterer Fassung im Baum hat, käme aus Stufe 0c nur noch über den
# Notfall-Ausstieg heraus. Form wie bei `D-05-Ausnahme:` — die Begründung ist
# Pflicht, eine nackte Zeile zählt nicht.
herkunft_ausnahme = set()
try:
    with open(".lizenzen.conf", encoding="utf-8") as f:
        for rohzeile in f:
            teile = rohzeile.split("#", 1)
            zeile = teile[0].strip()
            if not zeile:
                continue
            if zeile.startswith("herkunft "):
                grund = teile[1].strip() if len(teile) > 1 else ""
                if not grund:
                    print(
                        f"Herkunfts-Ausnahme ohne Begründung: {zeile}",
                        file=sys.stderr,
                    )
                    sys.exit(2)
                herkunft_ausnahme.add(zeile.split(None, 1)[1].strip())
            else:
                erlaubt.add(zeile)
except Exception as e:
    print(f"Positivliste nicht lesbar: {e}", file=sys.stderr)
    sys.exit(2)
if not erlaubt:
    print("Positivliste ist leer — keine Lizenz wäre zulässig", file=sys.stderr)
    sys.exit(2)

def zulaessig(ausdruck):
    """Zerlegt einen SPDX-Ausdruck.

    `OR` heißt: **ein** zulässiger Teil genügt — die Wahl steht uns zu.
    `AND` heißt: **alle** Teile müssen zulässig sein.
    `WITH` bindet eine Ausnahme an eine Lizenz; maßgeblich ist die Lizenz,
    die Ausnahme erweitert die Rechte nur.

    Zuvor blockierte jeder zusammengesetzte Ausdruck ohne dokumentierten
    Ausweg — auch dann, wenn eine seiner Alternativen auf der Liste stand.
    """
    a = ausdruck.strip()
    while a.startswith("(") and a.endswith(")"):
        a = a[1:-1].strip()
    teile = re.split(r"\s+OR\s+", a)
    if len(teile) > 1:
        return any(zulaessig(t) for t in teile)
    teile = re.split(r"\s+AND\s+", a)
    if len(teile) > 1:
        return all(zulaessig(t) for t in teile)
    a = re.split(r"\s+WITH\s+", a)[0].strip()
    return a in erlaubt

try:
    with open("package-lock.json", encoding="utf-8") as f:
        daten = json.load(f)
except Exception as e:
    # **Nicht Rückgabewert 0.** Zuvor endete dieser Zweig mit 0 und einer
    # Zeile ohne Tabulator; der Aufrufer verwarf sie und meldete grün. Ein
    # unlesbares Lockfile ist aber gerade der Fall, in dem nichts geprüft
    # werden konnte.
    print(f"Lockfile nicht lesbar: {e}", file=sys.stderr)
    sys.exit(2)

# **Die Fassung entscheidet, wo die Pakete stehen.** Bei `lockfileVersion` 1
# liegen sie unter `dependencies`, nicht unter `packages` — die Schleife wäre
# leer, es fiele kein Befund an, und die Prüfung meldete „0 Pakete geprüft" als
# Bestehen. Damit wäre die Prüfung, die nach AK-11 ein eingeschleustes
# Fremdpaket scheitern lassen soll, über den Aufbau der geprüften Datei
# abschaltbar. Dasselbe gilt für ein Lockfile, das schlicht `{}` ist.
version = daten.get("lockfileVersion")
if not isinstance(version, int) or version < 2:
    print(
        f"Lockfile-Fassung {version!r} — nur 2 und 3 führen 'packages'. "
        "Mit 'npm install --package-lock-only' neu erzeugen.",
        file=sys.stderr,
    )
    sys.exit(2)

pakete = daten.get("packages") or {}
if not [k for k in pakete if k]:
    print("Lockfile führt keinen einzigen Paketeintrag", file=sys.stderr)
    sys.exit(2)

fremd, herkunft = {}, []
for name, eintrag in pakete.items():
    if not name:
        continue
    lizenz = eintrag.get("license")
    if lizenz is None:
        fremd.setdefault("(ohne Angabe)", []).append(name)
    elif not isinstance(lizenz, str) or not zulaessig(lizenz):
        fremd.setdefault(str(lizenz), []).append(name)

    # **Herkunft, nicht nur Lizenz** (SM-OSS-011: „Herkunft und Bauweg
    # nachvollziehbar"). Ein Eintrag mit fremder Bezugsquelle und ohne
    # Prüfsumme passierte die Prüfung, und das folgende `npm ci` bezog das
    # Paket von dort.
    # Ein Verweis auf eine Kiste desselben Baums (`link`) wird nicht bezogen.
    if eintrag.get("link"):
        continue

    # **Beide Bedingungen unabhängig auswerten.** Zuvor stieg der Zweig bei
    # fehlendem `resolved` mit `continue` aus und übersprang damit *auch* die
    # Prüfsummenbedingung: Ein Eintrag, dem beide Felder fehlen, passierte die
    # Prüfung vollständig — und `npm ci` löste ihn anschließend ungebunden auf.
    if name in herkunft_ausnahme:
        continue

    aufgeloest = eintrag.get("resolved")
    if aufgeloest is None:
        herkunft.append(f"{name}: ohne Bezugsquelle (kein 'resolved' im Lockfile)")
    elif not str(aufgeloest).startswith("https://registry.npmjs.org/"):
        herkunft.append(f"{name}: Bezugsquelle {aufgeloest}")

    pruefsumme = str(eintrag.get("integrity") or "")
    if not pruefsumme.startswith(("sha512-", "sha256-")):
        herkunft.append(f"{name}: ohne Prüfsumme (integrity)")

for lizenz, namen in sorted(fremd.items()):
    kurz = ", ".join(sorted(namen)[:3])
    rest = f" und {len(namen) - 3} weitere" if len(namen) > 3 else ""
    print(f"LIZENZ\t{lizenz}: {kurz}{rest}")
for z in sorted(set(herkunft))[:8]:
    print(f"HERKUNFT\t{z}")
