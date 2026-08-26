#!/usr/bin/env bash
# Konsistenzprüfungen für den Implementierungsplan IMP-STM-001 — Teil der Stufe 0c.
#
# Der Plan verteilt dieselbe Aussage über viele Stellen: Umfangstabelle, Reibungsstellen,
# Arbeitspakete, Prüffallschema, Matrix, Zurückstellung. Jede Korrektur an einer Stelle kann
# an einer anderen einen Widerspruch hinterlassen. Diese Prüfung findet genau diese Klasse.
#
# Exit 0 = grün · 1 = Befund · 2 = Gegenstand da, Prüfung nicht durchführbar (FAIL)
# Exit 3 = ENTFÄLLT (kein Implementierungsplan im Baum) — nie als PASS zu werten.
# --selftest prüft jede Bedingung gegen ein eigenes Negativbeispiel.

set -uo pipefail
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2

# Der Kopf sagt --selftest zu; ohne diese Zeile war die Zusage unerfüllt und der
# Aufruf lief als gewöhnliche Prüfung durch (Befund C-M2 der Runde 14).
if [ "${1:-}" = --selftest ]; then exec bash "$ROOT/scripts/check-plan.test.sh"; fi

PLAN="${CHECK_PLAN_FILE:-Implementation/StitchManager_Implementierungsplan.md}"
URS="${CHECK_PLAN_URS:-Requirements/StitchManager_Lastenheft.md}"
ANA="${CHECK_PLAN_ANA:-Analysis/*_implementierungsplan.md}"

# Gegenstand vor Werkzeug prüfen (S3): Ohne Plan ist die Prüfung ENTFÄLLT, nicht FAIL —
# auch dann, wenn python3 fehlt.
[ -f "$PLAN" ] || exit 3
command -v python3 >/dev/null 2>&1 || { echo "  ✗ Planprüfungen: python3 fehlt"; exit 2; }

python3 - "$PLAN" "$URS" "$ANA" <<'PYENDE'
import os, re, sys, glob
from pathlib import Path

plan_pfad, urs_pfad, ana_glob = sys.argv[1], sys.argv[2], sys.argv[3]
t = Path(plan_pfad).read_text(encoding="utf-8")

# Geglättete Fassung für satzbezogene Bedingungen.
#
# Der Plan ist bei 100 Zeichen hart umbrochen. Jede Bedingung, die eine
# **Wortfolge** sucht, geht deshalb an ihrem Gegenstand vorbei, sobald er über
# eine Zeilengrenze läuft — und meldet grün, ohne je etwas gesehen zu haben.
# Gemessen am vorliegenden Plan traf das zwei Bedingungen: von drei
# Zählwortstellen prüfte Bedingung 8 null.
flach = re.sub(r'\s+', ' ', t)

urs = Path(urs_pfad).read_text(encoding="utf-8") if Path(urs_pfad).exists() else ""
fehler = []

def block(a, b, wozu):
    """Abschnitt zwischen zwei Ankern. Fehlt ein Anker, ist die Prüfung nicht durchführbar —
    fail-closed, damit eine umbenannte Überschrift keine Prüfung stillschweigend abschaltet."""
    try:
        return t[t.index(a):t.index(b)]
    except ValueError:
        fehler.append(f"Ankerüberschrift für {wozu} nicht gefunden — Prüfung nicht durchführbar")
        return None

# 1 · keine Kennung zugleich im Umfang und zurückgestellt
umfang = set(re.findall(r'^\| (SM-[A-Z]{3}-[0-9]{3}) \| [MS]', t, re.M))
b = block("### 10.1 Zurückgestellte", "### 10.2 Kann", "Zurückstellung")
if b is not None:
    zurueck = set()
    for z in b.splitlines():
        sp = z.split("|")
        if len(sp) > 2: zurueck |= set(re.findall(r'SM-[A-Z]{3}-[0-9]{3}', sp[2]))
    if umfang & zurueck:
        fehler.append(f"zugleich im Umfang und zurückgestellt: {sorted(umfang & zurueck)}")

# 2 · Arbeitspakete und Matrix deckungsgleich
matrix, kap4 = {}, {}
for k, ap in re.findall(r'^\| (SM-[A-Z]{3}-[0-9]{3}) \| [MS]¹? \| [TDIA] \| (AP-[0-9]{2}) \|', t, re.M):
    matrix.setdefault(ap, set()).add(k)
teile = re.split(r'^### (AP-[0-9]{2}) · ', t, flags=re.M)
if len(teile) < 3:
    fehler.append("keine Arbeitspaket-Überschriften gefunden — Prüfung nicht durchführbar")
for i in range(1, len(teile), 2):
    m = re.search(r'- \*\*Zugeordnet:\*\*(.*?)(?=\n- \*\*)', teile[i+1], re.S)
    if m:
        kap4[teile[i]] = set() if "keine" in m.group(1).split(".")[0] else set(re.findall(r'SM-[A-Z]{3}-[0-9]{3}', m.group(1)))
for ap in sorted(set(matrix) | set(kap4)):
    if kap4.get(ap, set()) != matrix.get(ap, set()):
        fehler.append(f"{ap}: Arbeitspaket und Matrix weichen ab {sorted(kap4.get(ap,set()) ^ matrix.get(ap,set()))}")

# 3 · nichts zugleich zugeordnet und Mitwirkung desselben Pakets
for i in range(1, len(teile), 2):
    z = re.search(r'- \*\*Zugeordnet:\*\*(.*?)(?=\n- \*\*)', teile[i+1], re.S)
    w = re.search(r'- \*\*Mitwirkung:\*\*(.*?)(?=\n- \*\*)', teile[i+1], re.S)
    if z and w:
        ue = set(re.findall(r'SM-[A-Z]{3}-[0-9]{3}', z.group(1))) & set(re.findall(r'SM-[A-Z]{3}-[0-9]{3}', w.group(1)))
        if ue: fehler.append(f"{teile[i]}: zugleich zugeordnet und Mitwirkung {sorted(ue)}")

# 4 · Zählwerte gegen die Matrix; Gesamtzahl aus dem Lastenheft, nicht als Literal
n = len(umfang)
# Gestrichene Kennungen (~~SM-…~~) zählen nicht zum Umfang — sie bleiben nur als Verweisziel
gesamt = len(set(re.findall(r'^\| (SM-[A-Z]{3}-[0-9]{3}) \|', urs, re.M)))
for tr in set(re.findall(r'Jede der (\d+) verplanten', flach)) | set(re.findall(r'Über die (\d+) verplanten', flach)):
    if int(tr) != n: fehler.append(f"Zählwert {tr} statt {n}")
if gesamt:
    for a, e in re.findall(r'\| \*\*Summe\*\* \| (\d+) \| \*\*(\d+)\*\*', t):
        if int(e) != n: fehler.append(f"Summenzeile: {e} verplant statt {n}")
        if int(a) != gesamt: fehler.append(f"Summenzeile: {a} Anforderungen, Lastenheft führt {gesamt}")

# 5 · jeder in einem Arbeitspaket genannte Prüffall ist im Schema oder in der Matrix geführt
# Unterfälle zählen mit. Ohne sie schnitt das Muster ".N" ab, und eine Matrixzeile
# durfte "PF-NFR-05.1 bis .4" führen, während Abschnitt 6.1 fünf Unterfälle kennt —
# genau der Befund, der in Runde 13 dreifach gemeldet wurde. Die Schreibweisen
# "X.1 bis .5", "X.1, .2" und "X.1 und .2" werden dafür ausgeschrieben.
PF_BASIS = r'PF-[A-Z0-9]{3,5}-[0-9]{2}'
def pf_menge(txt):
    """Menge der Prüffallkennungen; Bereiche und Aufzählungen ausgeschrieben."""
    menge = set()
    for m in re.finditer(PF_BASIS + r'(?:\.[0-9])?((?:\s*(?:,|bis|und|·)\s*\.[0-9])*)', txt):
        voll = m.group(0)
        basis = re.match(PF_BASIS, voll).group(0)
        erste = re.match(PF_BASIS + r'\.([0-9])', voll)
        folge = [int(x) for x in re.findall(r'\.([0-9])', voll)]
        if not folge:
            menge.add(basis); continue
        menge.add(f"{basis}.{folge[0]}")
        # "bis" spannt einen Bereich, Komma und "und" zählen einzeln auf
        for treffer in re.finditer(r'(,|bis|und|·)\s*\.([0-9])', voll):
            wort, nr = treffer.group(1), int(treffer.group(2))
            if wort == "bis":
                vorher = folge[folge.index(nr) - 1] if nr in folge else int(erste.group(1))
                for k in range(vorher, nr + 1): menge.add(f"{basis}.{k}")
            else:
                menge.add(f"{basis}.{nr}")
    return menge

b1 = block("### 6.1 Prüffallschema", "### 6.2", "Prüffallschema")
b2 = block("## 7. Rückverfolgbarkeitsmatrix", "## 8. Mitwachsende", "Matrix")
if b1 is not None and b2 is not None:
    kap4_pf = pf_menge("".join(teile[i+1] for i in range(1, len(teile), 2)))
    bekannt = pf_menge(b1) | pf_menge(b2)
    if kap4_pf - bekannt:
        fehler.append(f"Prüffall ohne Eintrag in Abschnitt 6.1 oder Matrix: {sorted(kap4_pf - bekannt)}")

    # 5b · Unterfallmenge je Kennung: Abschnitt 6.1 gegen die Matrix.
    # Ein in 6.1 geführter Unterfall, den die Matrix nicht nennt, ist bei der
    # Abnahme unsichtbar — die Matrix ist ihr Träger (URS-STM-001 Abschnitt 13.3).
    def nach_basis(menge):
        d = {}
        for k in menge: d.setdefault(k.split(".")[0], set()).add(k)
        return d
    s61, s7 = nach_basis(pf_menge(b1)), nach_basis(pf_menge(b2))
    for basis in sorted(set(s61) & set(s7)):
        fehlend = s61[basis] - s7[basis]
        if fehlend:
            fehler.append(f"Matrix führt {basis} ohne Unterfall {sorted(fehlend)} — Abschnitt 6.1 kennt sie")

# 6 · Zeilenlänge im Fließtext (Tabellen und Codeblöcke ausgenommen)
inb = False
for nr, l in enumerate(t.splitlines(), 1):
    if l.startswith("```"): inb = not inb; continue
    if inb or l.lstrip().startswith("|"): continue
    if len(l) > 100: fehler.append(f"{plan_pfad}:{nr}: {len(l)} Zeichen")

# 7 · offene Punkte: definiert, referenziert, im Entscheidungsgatter
op_def = set(re.findall(r'^\| \*\*(OP-[0-9]{2})\*\* \|', urs, re.M))
if urs and not op_def:
    fehler.append("keine offenen Punkte im Lastenheft gefunden — Prüfung nicht durchführbar")
for op in sorted(set(re.findall(r'OP-[0-9]{2}', t)) - op_def):
    fehler.append(f"OP-Verweis ohne Definition im Lastenheft: {op}")
bg = block("## 2. Entscheidungsgatter", "## 3. Zielarchitektur", "Entscheidungsgatter")
if bg is not None:
    for op in sorted(op_def - set(re.findall(r'OP-[0-9]{2}', bg))):
        fehler.append(f"{op} im Lastenheft geführt, aber nicht im Entscheidungsgatter")

# 8 · ausgeschriebene Zahlwörter für die Zahl der offenen Punkte, abgeleitet statt aufgezählt
EINER = ["null","ein","zwei","drei","vier","fünf","sechs","sieben","acht","neun"]
ZEHNER = {2:"zwanzig",3:"dreißig",4:"vierzig",5:"fünfzig"}
def zahlwort(z):
    if z < 13: return {10:"zehn",11:"elf",12:"zwölf"}.get(z, EINER[z] if z < 10 else None)
    if z < 20: return {16:"sechzehn",17:"siebzehn"}.get(z, EINER[z-10] + "zehn")
    e, t_ = z % 10, z // 10
    return ZEHNER.get(t_, "") if e == 0 else f"{EINER[e]}und{ZEHNER.get(t_, '')}"
wort = zahlwort(len(op_def)) if op_def else None
# Nur echte Zahlwörter prüfen. Das Muster erfasst sonst auch gewöhnlichen
# Fließtext („die offenen Punkte"), und die Bedingung meldete einen Befund,
# wo keiner ist — der sichere Weg, eine Prüfung wieder loszuwerden.
zahlworte = {zahlwort(i) for i in range(1, 100)} - {None}
for m in re.finditer(r'\b([a-zäöüß]+)\s+offene[nr]?\s+Punkte', flach):
    gefunden = m.group(1)
    if gefunden not in zahlworte:
        continue
    if wort and gefunden != wort:
        fehler.append(f"Zahlwort '{gefunden} offene Punkte' — es sind {len(op_def)} ({wort})")

# 9 · Auszeichnungsschäden, je Absatz
dateien = [Path(plan_pfad)] + [Path(x) for x in sorted(glob.glob(ana_glob))]
for datei in dateien:
    if not datei.exists(): continue
    inb, absatz, start = False, [], 0
    def pruef(ab, nr, d=datei):
        txt = " ".join(ab)
        if txt.count("**") % 2: fehler.append(f"{d}:{nr}: ungerade Zahl von **")
        if "****" in txt: fehler.append(f"{d}:{nr}: vierfaches Sternchen")
        # Stehen gebliebener Artikel vor einer fett gesetzten Zusage: "… Ein **Das
        # Sicherungsarchiv …**". Zwei Artikel hintereinander über die Auszeichnungs-
        # grenze hinweg sind in Sachsprache immer ein Bearbeitungsschaden.
        ART = r'(?:Ein|Eine|Einen|Einem|Der|Die|Das|Den|Dem)'
        if re.search(ART + r' \*\*' + ART + r'\b', txt):
            fehler.append(f"{d}:{nr}: Artikel doppelt über eine Auszeichnungsgrenze — "
                          "vermutlich ein stehen gebliebenes Wort")
        w = txt.split()
        for k in range(len(w) - 5):
            if w[k:k+3] == w[k+3:k+6] and len(" ".join(w[k:k+3])) > 12:
                fehler.append(f"{d}:{nr}: wiederholte Wortfolge"); break
    for nr, l in enumerate(datei.read_text(encoding="utf-8").splitlines(), 1):
        if l.startswith("```"): inb = not inb; continue
        if inb: continue
        if not l.strip() or l.lstrip().startswith("|"):
            if absatz: pruef(absatz, start); absatz = []
            continue
        if not absatz: start = nr
        absatz.append(l)
    if absatz: pruef(absatz, start)

# 9b · Kopfschema: die Metadatenfelder stehen je auf eigener Zeile wie in den
# Schwesterdokumenten. Ohne diese Bedingung zieht ein Umbruchlauf den Kopf zu einem Absatz
# zusammen — dann findet der Versionsabgleich in check-docs.sh die Version nicht mehr am
# Zeilenanfang und überspringt sie still (fail-open, in Runde 16 aufgetreten).
kopf = t.split("\n---\n", 1)[0].splitlines()
for feld in ("Kennung", "Version", "Datum", "Führendes Dokument", "Mitgeltend"):
    if not any(l.startswith("**" + feld + ":**") for l in kopf):
        fehler.append("Kopffeld '" + feld + "' steht nicht am Zeilenanfang — der Versionsabgleich greift dort nicht")

# 10 · jede beschriebene Prüfrunde hat ihre Befundtabelle — Nummern, nicht Zahlwörter
# Die Rundennummer kommt aus dem Ordinalwort der Überschrift, nicht aus einer
# beliebigen Erwähnung im Fließtext. Vorher ordnete die Bedingung "8.24 Zwölfte
# Runde" wegen eines Rückverweises der Runde 11 zu, fand deren Tabelle und meldete
# grün — sie erkannte ihren eigenen Gegenstand nicht (fail-open).
ORDINAL = {"erste": 1, "zweite": 2, "dritte": 3, "vierte": 4, "fünfte": 5, "sechste": 6,
           "siebte": 7, "achte": 8, "neunte": 9, "zehnte": 10, "elfte": 11, "zwölfte": 12,
           "dreizehnte": 13, "vierzehnte": 14, "fünfzehnte": 15, "sechzehnte": 16,
           "siebzehnte": 17, "achtzehnte": 18, "neunzehnte": 19, "zwanzigste": 20}
for datei in sorted(glob.glob(ana_glob)):
    a = Path(datei).read_text(encoding="utf-8")
    beschrieben = set(re.findall(r'Runde (\d+)\*\* ?—|\*\*Runde (\d+)\*\*', a))
    beschrieben = {x or y for x, y in beschrieben}
    for m in re.finditer(r'\*\*8\.\d+[a-z]? ([A-ZÄÖÜ][a-zäöüß]+) Runde', a):
        wort = m.group(1).lower()
        if wort in ORDINAL:
            beschrieben.add(str(ORDINAL[wort]))
        else:
            fehler.append(f"{datei}: Rundenüberschrift mit unbekanntem Ordinalwort '{m.group(1)}' — "
                          "die Zuordnung zur Befundtabelle ist damit nicht prüfbar")
    mit_tabelle = set(re.findall(r'Befunde der Runde (\d+)', a))
    mit_tabelle |= set(re.findall(r'\*\*Runde (\d+)\*\* — \d+ Befunde', a))
    fehlt = beschrieben - mit_tabelle
    if fehlt:
        fehler.append(f"{datei}: Runde(n) ohne Befundtabelle: {sorted(fehlt, key=int)}")

# 11 · Zählwerte der README gegen ihre Quellen
# Die README nennt Anforderungs-, Paket- und Punktzahlen. Ohne diese Bedingung altern
# sie unbemerkt: Prüfung 5 in check-docs.sh deckt nur Versionen ab, nicht Zahlen
# (Befund T-8 der Runde 13). Fehlt die README oder eine Zeile, entfällt der Vergleich
# für diese Zeile — geprüft wird, was dasteht.
readme = Path(os.environ.get("CHECK_PLAN_README", "README.md"))
if readme.exists():
    r = readme.read_text(encoding="utf-8")
    erwartet = [
        ("Anforderungen im Lastenheft", gesamt, "Anforderungen des Lastenhefts"),
        ("Für Version 1.0 verplant", n, "verplante Anforderungen"),
        ("Arbeitspakete", len(kap4), "Arbeitspakete"),
        ("Offene Punkte", len(op_def), "offene Punkte"),
    ]
    for etikett, soll, was in erwartet:
        m = re.search(r'^\| ' + re.escape(etikett) + r' \| [^0-9]*([0-9]+)', r, re.M)
        if not m:
            # Still übergehen hieße: Wer die Zeile löscht, entzieht sie der Prüfung.
            fehler.append(f"{readme}: Zeile „{etikett}“ fehlt oder nennt keine Zahl — "
                          "die Zählwerte der README stehen unter Prüfung")
            continue
        if soll and int(m.group(1)) != soll:
            fehler.append(f"{readme}: Zeile „{etikett}“ nennt {m.group(1)} {was}, "
                          f"tatsächlich sind es {soll}")

if fehler:
    for f in fehler: print(f"  ✗ {f}")
    sys.exit(1)
print(f"  ✓ Planprüfungen: {n} Anforderungen, {len(kap4)} Arbeitspakete, {len(op_def)} offene Punkte")
PYENDE
