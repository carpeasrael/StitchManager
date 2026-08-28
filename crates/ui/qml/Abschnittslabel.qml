// Abschnittslabel der Navigationsspalte.
//
// Die Nachordnung entsteht über Größe, Versalien und Sperrung — NICHT über
// eine dritte Textfarbe: `ink3` ist für lesbaren Text gesperrt (DES 3.3).
import QtQuick

Text {
    id: label
    required property Gestaltung gestaltung
    color: label.gestaltung.ink2
    font.family: label.gestaltung.schriftAuszeichnung
    font.pixelSize: label.gestaltung.tXs
    font.capitalization: Font.AllUppercase
    font.letterSpacing: label.gestaltung.labelSperrung
    font.weight: Font.DemiBold
    textFormat: Text.PlainText
}
