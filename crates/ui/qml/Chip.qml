// Filterchip (SM-SRC-009: aktive Filter sind sichtbar und einzeln entfernbar).
import QtQuick

Item {
    id: chip
    required property Gestaltung gestaltung
    property alias text: beschriftung.text
    property bool gewaehlt: false

    signal geklickt

    implicitHeight: chip.gestaltung.bedienTrefferflaeche
    implicitWidth: beschriftung.implicitWidth + chip.gestaltung.s4

    Rectangle {
        id: flaeche
        anchors.centerIn: parent
        width: parent.width
        height: chip.gestaltung.bedienSichtbar
        radius: height / 2
        color: chip.gewaehlt ? chip.gestaltung.brandSoft : chip.gestaltung.surface
        border.width: chip.gestaltung.rahmenstaerke
        border.color: chip.gewaehlt ? chip.gestaltung.brand : chip.gestaltung.border

        Behavior on color {
            ColorAnimation {
                duration: chip.gestaltung.dauerKurz
            }
        }

        Row {
            anchors.centerIn: parent
            spacing: chip.gestaltung.s1

            // Zweites Merkmal neben der Farbe (SM-NFR-009): Der gewählte
            // Zustand trägt zusätzlich eine Marke.
            Text {
                visible: chip.gewaehlt
                text: "✓"
                color: chip.gestaltung.brandInk
                font.pixelSize: chip.gestaltung.tXs
                anchors.verticalCenter: parent.verticalCenter
            }

            Text {
                id: beschriftung
                anchors.verticalCenter: parent.verticalCenter
                color: chip.gewaehlt ? chip.gestaltung.brandInk : chip.gestaltung.ink2
                font.family: chip.gestaltung.schriftText
                font.pixelSize: chip.gestaltung.tSm
                font.weight: chip.gewaehlt ? Font.DemiBold : Font.Normal
                textFormat: Text.PlainText
            }
        }
    }

    Rectangle {
        anchors.fill: flaeche
        anchors.margins: -chip.gestaltung.s1
        visible: chip.activeFocus
        color: "transparent"
        radius: height / 2
        border.width: chip.gestaltung.fokusstaerke
        border.color: chip.gestaltung.brand
    }

    MouseArea {
        anchors.fill: parent
        onClicked: chip.geklickt()
    }

    activeFocusOnTab: true
    Keys.onReturnPressed: chip.geklickt()
    Keys.onSpacePressed: chip.geklickt()

    Accessible.role: Accessible.RadioButton
    Accessible.name: beschriftung.text
    Accessible.checked: chip.gewaehlt
}
