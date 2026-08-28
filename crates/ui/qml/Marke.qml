// Format- und Herkunftsmarke der Kachel.
//
// Das Lastenheft nennt das Element in SM-DES-007 „Herkunftsmarke“,
// DES-STM-001 Abschnitt 6.3 „KI-Marke“. Maßgeblich ist der Wortlaut des
// führenden Dokuments (CLAUDE.md Abschnitt 3): Herkunftsmarke.
import QtQuick

Rectangle {
    id: marke
    required property Gestaltung gestaltung
    property alias text: beschriftung.text
    property color flaeche: marke.gestaltung.surface3
    property color schrift: marke.gestaltung.ink2

    implicitWidth: beschriftung.implicitWidth + marke.gestaltung.s2
    implicitHeight: beschriftung.implicitHeight + marke.gestaltung.s1
    radius: marke.gestaltung.radiusKlein
    color: marke.flaeche

    Text {
        id: beschriftung
        anchors.centerIn: parent
        color: marke.schrift
        font.family: marke.gestaltung.schriftFestbreite
        font.pixelSize: marke.gestaltung.tXs
        font.weight: Font.Medium
        textFormat: Text.PlainText
    }
}
