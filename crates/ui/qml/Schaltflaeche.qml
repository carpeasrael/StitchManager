// Schaltfläche in Pillenform (DES Abschnitt 5).
//
// Text auf Terracotta steht ausschließlich in `aufBrand` — weiße Schrift
// verfehlt SM-NFR-007 in beiden Modi (DES 3.3, Regel 3).
import QtQuick

Item {
    id: knopf
    required property Gestaltung gestaltung
    property alias text: beschriftung.text
    property bool betont: false
    property bool aktiv: true
    property bool sichtbar: true

    signal geklickt

    visible: knopf.sichtbar
    // Sichtbar mindestens 26 px, Trefferfläche mindestens 32 px
    // (DES Abschnitt 7).
    implicitHeight: knopf.gestaltung.bedienTrefferflaeche
    implicitWidth: beschriftung.implicitWidth + knopf.gestaltung.s5

    Rectangle {
        id: flaeche
        anchors.centerIn: parent
        width: parent.width
        height: knopf.gestaltung.bedienSichtbar
        radius: height / 2
        color: knopf.betont ? knopf.gestaltung.brand : (zeiger.pressed ? knopf.gestaltung.surface3 : knopf.gestaltung.surface)
        border.width: knopf.gestaltung.rahmenstaerke
        border.color: knopf.betont ? knopf.gestaltung.brand : (zeiger.containsMouse ? knopf.gestaltung.borderStrong : knopf.gestaltung.border)

        Behavior on color {
            ColorAnimation {
                duration: knopf.gestaltung.dauerKurz
            }
        }

        Text {
            id: beschriftung
            anchors.centerIn: parent
            // Deaktiviert trägt `ink3` — DES-STM-001 Abschnitt 7 legt für
            // diesen Zustand keine Deckkraftänderung fest, und `ink3` ist genau
            // hierfür freigegeben (sonst für lesbaren Text gesperrt).
            color: !knopf.aktiv ? knopf.gestaltung.ink3 : (knopf.betont ? knopf.gestaltung.aufBrand : knopf.gestaltung.ink)
            font.family: knopf.gestaltung.schriftText
            font.pixelSize: knopf.gestaltung.tSm
            font.weight: Font.DemiBold
            textFormat: Text.PlainText
        }
    }

    // Der Fokusring wird nie unterdrückt (DES Abschnitt 7).
    Rectangle {
        anchors.fill: flaeche
        anchors.margins: -knopf.gestaltung.s1
        visible: knopf.activeFocus
        color: "transparent"
        radius: height / 2
        border.width: knopf.gestaltung.fokusstaerke
        border.color: knopf.gestaltung.brand
    }

    MouseArea {
        id: zeiger
        anchors.fill: parent
        hoverEnabled: true
        enabled: knopf.aktiv
        onClicked: knopf.geklickt()
    }

    focus: false
    activeFocusOnTab: knopf.aktiv && knopf.sichtbar
    Keys.onReturnPressed: if (knopf.aktiv)
        knopf.geklickt()
    Keys.onSpacePressed: if (knopf.aktiv)
        knopf.geklickt()

    Accessible.role: Accessible.Button
    Accessible.name: beschriftung.text
    Accessible.onPressAction: if (knopf.aktiv)
        knopf.geklickt()
}
