// Auswahlmarke als Kreuzstich (DES Abschnitt 2).
//
// Sie trägt die Auswahl als **Form**, nicht allein über Farbe — SM-NFR-009
// verlangt ein zweites Merkmal neben der Farbe.
import QtQuick

Canvas {
    id: marke
    property color farbe: "transparent"
    // **Die Gestaltungsquelle wird hereingereicht** — dasselbe Muster wie in
    // `Chip.qml`, `Schaltflaeche.qml` und `Abschnittslabel.qml`. Ein Bauteil
    // in eigener Datei sieht `kn` aus `Haupt.qml` nicht; ein Zahlenwert hier
    // wäre ein Gestaltungsliteral am Designsystem vorbei (SM-DES-003, D-05).
    required property Gestaltung gestaltung
    property int groesse: marke.gestaltung.s4

    width: marke.groesse
    height: marke.groesse

    onFarbeChanged: requestPaint()

    onPaint: {
        var g = getContext("2d");
        g.reset();
        g.strokeStyle = marke.farbe;
        g.lineWidth = marke.gestaltung.fokusstaerke;
        g.lineCap = "round";
        g.beginPath();
        g.moveTo(2, 2);
        g.lineTo(width - 2, height - 2);
        g.moveTo(width - 2, 2);
        g.lineTo(2, height - 2);
        g.stroke();
    }
}
