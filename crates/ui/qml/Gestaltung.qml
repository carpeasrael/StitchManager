// GESTALTUNGSQUELLE
//
// ui/gestaltung — die EINZIGE Quelle der Farb-, Schrift- und Abstandswerte.
//
// Die Markierung oben ist kein Schmuck: Die Projektregelprüfung findet die
// Gestaltungsquelle daran, statt ihren Pfad fest zu kodieren. So bleibt die
// Prüfung unabhängig vom gewählten Oberflächenweg (IMP-STM-001 Abschnitt 2.1),
// und SM-DES-003 („genau eine Datei") ist maschinell prüfbar: keine und
// mehrere Markierungen sind beides ein Befund.
//
// SM-DES-003 und Prüfpunkt D-05: Kein Farb-, Schrift- oder Abstandsliteral
// steht außerhalb dieser Datei. Jede andere Datei entwickelt gegen die
// **Bezeichner** hier, nie gegen Werte.
//
// Die Werte sind nach DES-STM-001 Abschnitt 3 aus dem Markenstandard
// rekonstruiert und stehen unter OP-09. Der Abgleich ändert diese Datei —
// kein Bauteil. Die Bezeichner ändern sich dabei nie.
import QtQuick

QtObject {
    id: kn

    // Dunkelmodus ist Espresso, nie neutrales Grau (DES Abschnitt 3.2).
    property bool dunkel: false

    // --- Farben (DES Abschnitt 3.1 / 3.2) ---
    readonly property color bg: dunkel ? "#221b17" : "#faf6f0"
    readonly property color surface: dunkel ? "#2b231e" : "#ffffff"
    readonly property color surface2: dunkel ? "#251e19" : "#f4ede2"
    readonly property color surface3: dunkel ? "#3a2f27" : "#ebe1d3"
    readonly property color border: dunkel ? "#443830" : "#e0d4c2"
    readonly property color borderStrong: dunkel ? "#5c4c40" : "#c9b9a2"
    readonly property color ink: dunkel ? "#f3e9dd" : "#2e2a27"
    readonly property color ink2: dunkel ? "#b3a294" : "#6e6259"
    // ink3 ist für lesbaren Text GESPERRT — nur Deaktiviert und Platzhalter.
    readonly property color ink3: dunkel ? "#91816f" : "#8d8177"
    readonly property color brand: dunkel ? "#f0836e" : "#e85d5d"
    // brand erreicht als Textfarbe im Hellmodus nur 3,17:1 — für Text gilt
    // ausschließlich brandInk.
    readonly property color brandInk: dunkel ? "#f0836e" : "#c2452f"
    readonly property color brandSoft: dunkel ? "#402620" : "#fbe7e2"
    readonly property color amber: dunkel ? "#e5b463" : "#d98c1f"
    readonly property color amberSoft: dunkel ? "#3b2d18" : "#fbeed6"
    readonly property color ok: dunkel ? "#7fb98c" : "#3b754a"
    readonly property color okSoft: dunkel ? "#23331f" : "#e3f0e5"
    readonly property color warn: dunkel ? "#e0ac5a" : "#955e0e"
    readonly property color ki: dunkel ? "#b3a3d6" : "#6b5b8f"
    readonly property color kiSoft: dunkel ? "#2f2839" : "#eee9f5"
    // Einzige Textfarbe auf Terracotta-Flächen, in BEIDEN Modi.
    // Heißt in DES-STM-001 `--kn-on-brand`; QML deutet einen Bezeichner mit
    // „on"-Vorsilbe als Signalempfänger, deshalb hier `aufBrand`.
    // Weiße Schrift auf brand verfehlt SM-NFR-007 (3,41:1 hell, 2,57:1 dunkel).
    readonly property color aufBrand: "#2b1a15"
    readonly property color seam: dunkel ? "#5c4c40" : "#cbbaa4"

    // --- Grundraster 4 px (DES Abschnitt 5) ---
    readonly property int raster: 4
    readonly property int s1: raster        //  4
    readonly property int s2: raster * 2    //  8
    readonly property int s3: raster * 3    // 12
    readonly property int s4: raster * 4    // 16
    readonly property int s5: raster * 5    // 20
    readonly property int s6: raster * 6    // 24

    // --- Radien ---
    readonly property int radiusKlein: 4
    readonly property int radiusMittel: 7
    readonly property int radiusGross: 11
    // „Pille" ist voll gerundet; der Wert wird je Element aus der Höhe gebildet.

    readonly property int rahmenstaerke: 1
    readonly property int fokusstaerke: 2

    // --- Bereichsmaße (DES Abschnitt 5) ---
    readonly property int werkzeugleisteHoehe: 46
    readonly property int statusleisteHoehe: 26
    readonly property int navigationMin: 190
    readonly property int navigationVorgabe: 226
    readonly property int auswahlMin: 300
    readonly property int detailMin: 290
    readonly property int trennerSichtbar: 1
    // Die Ziehfläche ist größer als die sichtbare Linie, sonst ist der Trenner
    // nicht greifbar.
    readonly property int trennerZiehflaeche: 6
    readonly property int fensterMindestbreite: 860

    // Fenstermaße. Die Mindestbreite steht in DES Abschnitt 5; die
    // Vorgabegrößen sind Gestaltung und gehören deshalb ebenfalls hierher.
    readonly property int fensterBreiteVorgabe: 1280
    readonly property int fensterHoeheVorgabe: 820
    readonly property int fensterMindesthoehe: 560

    // Musterauswahl: Kachelmaße. Die Höhe steht **vor** dem Laden der Vorschau
    // fest, damit nichts springt (SM-PRV-009).
    readonly property int kachelBreite: 168
    readonly property int kachelHoehe: 208
    readonly property int kachelBildHoehe: 116

    // Größte Breite einer Hinweisspalte (leerer, ladender, fehlerhafter Zustand).
    readonly property int hinweisMaxBreite: 380

    // Größte Breite des Suchfelds in der Werkzeugleiste.
    readonly property int suchfeldMaxBreite: 420

    // Sperrung der Abschnittslabels. Die Nachordnung entsteht über Größe,
    // Versalien und Sperrung — nicht über eine dritte Textfarbe (DES 3.3).
    readonly property int labelSperrung: 1

    // Mindestmaße von Bedienelementen (DES Abschnitt 7).
    readonly property int bedienSichtbar: 26
    readonly property int bedienTrefferflaeche: 32

    // --- Schrift (DES Abschnitt 4) ---
    // Die Festbreitenschrift steht unter OP-07; bis zur Freigabe wird die
    // Festbreitenschrift des Systems verwendet, nicht eine mitgelieferte.
    readonly property string schriftAuszeichnung: "Josefin Sans"
    readonly property string schriftText: "Lato"
    readonly property string schriftFestbreite: "IBM Plex Mono"

    readonly property int tXs: 11
    readonly property int tSm: 12
    readonly property int tMd: 13
    readonly property int tLg: 15
    readonly property int tXl: 19

    // --- Bewegung ---
    // SM-NFR-013: Bei reduzierter Bewegung entfallen ALLE Übergänge, ohne
    // Ausnahme. Jede Animation liest diese Dauer, keine setzt eine eigene.
    property bool bewegungReduziert: false
    readonly property int dauerKurz: bewegungReduziert ? 0 : 120
    readonly property int dauerMittel: bewegungReduziert ? 0 : 200
}
