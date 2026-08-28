// Trennlinie als gestrichelte Naht (DES Abschnitt 2: Trennlinien sind Nähte,
// keine durchgezogenen Linien).
import QtQuick

Canvas {
    id: naht
    property color farbe: "transparent"
    property bool senkrecht: false
    // **Die Gestaltungsquelle wird hereingereicht** — dasselbe Muster wie in
    // `Chip.qml` und `Schaltflaeche.qml`. Sichtbare Stärke; die Ziehfläche
    // eines Trenners ist größer und wird von der aufrufenden Stelle gesetzt.
    // Der Wert steht in der Gestaltungsquelle (DES-STM-001 Abschnitt 5:
    // Rahmenstärke 1 px), nicht hier (SM-DES-003, D-05).
    required property Gestaltung gestaltung
    property int staerke: naht.gestaltung.rahmenstaerke

    width: naht.senkrecht ? naht.staerke : parent.width
    height: naht.senkrecht ? parent.height : naht.staerke

    onFarbeChanged: requestPaint()
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()

    onPaint: {
        var g = getContext("2d");
        g.reset();
        g.strokeStyle = naht.farbe;
        g.lineWidth = naht.staerke;
        g.setLineDash([4, 3]);
        g.beginPath();
        if (naht.senkrecht) {
            g.moveTo(naht.staerke / 2, 0);
            g.lineTo(naht.staerke / 2, height);
        } else {
            g.moveTo(0, naht.staerke / 2);
            g.lineTo(width, naht.staerke / 2);
        }
        g.stroke();
    }
}
