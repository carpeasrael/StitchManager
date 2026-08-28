// ui/fenster · ui/navigation · ui/auswahl · ui/detail
//
// Dreispaltiges Hauptfenster nach DES-STM-001 Abschnitt 6. Die Spalten stehen
// in **jeder** Fensterbreite senkrecht nebeneinander; es gibt keine gestapelte
// Ersatzdarstellung (SM-DES-005/006, AK-12). Unterhalb der Mindestbreite wird
// waagerecht gescrollt.
//
// Kein Farb-, Schrift- oder Abstandswert steht in dieser Datei — alles kommt
// aus `Gestaltung.qml` (SM-DES-003, D-05).

// Bezeichner aus dem umgebenden Dokument sind in geschachtelten Bauteilen
// gebunden statt kontextabhängig aufgelöst. Zwei Gründe: Die Kachel greift im
// **Zeichenpfad** auf `kn` und auf ihre Modellrollen zu, und eine gebundene
// Auflösung kostet dort keine Namenssuche je Bild (SM-PRV-007, SM-NFR-003).
// Und sie macht die Zugriffe für `qmllint` prüfbar — ohne die Bindung ist
// jeder von ihnen ein unqualifizierter Zugriff, den das Werkzeug meldet, aber
// nicht nachvollziehen kann.
pragma ComponentBehavior: Bound
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import de.stitchmanager

ApplicationWindow {
    id: fenster
    visible: true
    width: kn.fensterBreiteVorgabe
    height: kn.fensterHoeheVorgabe
    minimumWidth: kn.fensterMindestbreite
    minimumHeight: kn.fensterMindesthoehe
    title: qsTr("StitchManager")
    color: kn.bg

    // Die einzige Quelle der Gestaltungswerte.
    Gestaltung {
        id: kn
    }

    Musterliste {
        id: bestand
    }

    // Auswahl der Musterauswahl-Spalte.
    property int gewaehlteZeile: -1

    // --- Selbsttest der Fadenverdrahtung (SM-NFR-002) ---
    //
    // Nur aktiv, wenn `SM_SELBSTTEST` gesetzt ist. Der Zeitgeber tickt im
    // Qt-Faden. Bliebe die Bedienung während des Einlesens stehen, blieben
    // auch die Ticks aus — genau das ist die Messgröße.
    property int ticksGesamt: 0
    property int ticksWaehrendLauf: 0
    property bool selbsttestLaeuft: false
    property bool selbsttestEingelesen: false
    // Ohne Wanduhrzeit ist der Tickzaehler wertlos: Ein blockierter Faden
    // verzoegert auch den Zeitgeber, der den Selbsttest beendet — die Ticks
    // saehen dann selbst bei voller Blockade vollzaehlig aus.
    property double startZeit: 0

    Timer {
        id: pulsschlag
        interval: 100
        repeat: true
        onTriggered: {
            fenster.ticksGesamt++;
            if (bestand.laedt)
                fenster.ticksWaehrendLauf++;

            // Das Einlesen beginnt erst, wenn die Wurzel bestaetigt ist. Die
            // Bestaetigung kommt aus dem Arbeitsfaden und ist deshalb nicht
            // schon im selben Zug da, in dem sie beauftragt wurde.
            if (fenster.selbsttestLaeuft && !fenster.selbsttestEingelesen && bestand.bibliothek_gewaehlt) {
                fenster.selbsttestEingelesen = true;
                bestand.bestand_einlesen();
            }
        }
    }

    Timer {
        id: selbsttestEnde
        interval: 12000
        onTriggered: {
            var verstrichen = Date.now() - fenster.startZeit;
            var erwartet = Math.round(verstrichen / pulsschlag.interval);
            console.log("SELBSTTEST" + " ticks_erwartet=" + erwartet + " verstrichen_ms=" + verstrichen + " gesamt=" + bestand.gesamt + " geladen=" + bestand.geladen + " laeuft_noch=" + bestand.laedt + " fortschritt=" + bestand.fortschritt + "/" + bestand.fortschritt_gesamt + " vorschauen=" + bestand.vorschauen_fertig + " ticks_gesamt=" + fenster.ticksGesamt + " ticks_waehrend_lauf=" + fenster.ticksWaehrendLauf + " pfad='" + bestand.bibliothekspfad + "'" + " fehler='" + bestand.fehlertext + "'");
            // Der Selbsttest urteilt selbst. Ein Mensch, der Zahlen
            // vergleicht, ist keine automatisierte Pruefung (SM-NFR-012).
            //
            // Fuenf Prozent Nachsicht fangen die Ungenauigkeit des Zeitgebers;
            // eine Blockade kostet ein Vielfaches davon.
            var genug = bestand.gesamt > 0;
            var fluessig = fenster.ticksGesamt >= erwartet * 0.95;
            if (genug && fluessig) {
                console.log("SELBSTTEST ERGEBNIS=PASS");
                Qt.exit(0);
            } else {
                console.log("SELBSTTEST ERGEBNIS=FAIL" + (genug ? "" : " (kein Bestand eingelesen)") + (fluessig ? "" : " (Bedienung stockte: " + fenster.ticksGesamt + " von " + erwartet + " Takten)"));
                Qt.exit(1);
            }
        }
    }

    Component.onCompleted: {
        var pfad = bestand.selbsttest_verzeichnis();
        if (pfad.length > 0) {
            fenster.selbsttestLaeuft = true;
            fenster.startZeit = Date.now();
            pulsschlag.start();
            selbsttestEnde.start();
            bestand.bibliothek_waehlen(pfad);
        }
    }

    // --- Tastaturkürzel (SM-NFR-008) ---
    Shortcut {
        sequences: [StandardKey.Find]
        onActivated: suchfeld.forceActiveFocus()
    }
    Shortcut {
        sequence: "Ctrl+O"
        onActivated: ordnerdialog.open()
    }
    Shortcut {
        sequence: "Ctrl+D"
        onActivated: kn.dunkel = !kn.dunkel
    }
    Shortcut {
        sequence: "Escape"
        onActivated: {
            if (suchfeld.text.length > 0) {
                suchfeld.clear();
                bestand.suchen("");
            }
        }
    }

    FolderDialog {
        id: ordnerdialog
        title: qsTr("Bibliothek wählen")
        onAccepted: {
            // `selectedFolder` ist eine URL; der Kern erwartet einen Pfad.
            var pfad = selectedFolder.toString().replace("file://", "");
            bestand.bibliothek_waehlen(decodeURIComponent(pfad));
        }
    }

    // ------------------------------------------------------------------
    // Werkzeugleiste
    // ------------------------------------------------------------------
    header: Rectangle {
        height: kn.werkzeugleisteHoehe
        color: kn.surface2

        Naht {
            anchors.bottom: parent.bottom
            width: parent.width
            farbe: kn.seam
            gestaltung: kn
        }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: kn.s4
            anchors.rightMargin: kn.s4
            spacing: kn.s3

            Text {
                text: qsTr("StitchManager")
                font.family: kn.schriftAuszeichnung
                font.pixelSize: kn.tLg
                font.weight: Font.DemiBold
                color: kn.brandInk
                textFormat: Text.PlainText
            }

            Schaltflaeche {
                text: qsTr("Bibliothek wählen…")
                gestaltung: kn
                onGeklickt: ordnerdialog.open()
            }

            Schaltflaeche {
                text: bestand.laedt ? qsTr("Einlesen abbrechen") : qsTr("Bestand einlesen")
                gestaltung: kn
                betont: !bestand.laedt
                aktiv: bestand.bibliothek_gewaehlt
                onGeklickt: bestand.laedt ? bestand.einlesen_abbrechen() : bestand.bestand_einlesen()
            }

            // Suchfeld
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: kn.bedienSichtbar
                Layout.maximumWidth: kn.suchfeldMaxBreite
                radius: kn.radiusKlein
                color: kn.surface
                border.width: kn.rahmenstaerke
                border.color: suchfeld.activeFocus ? kn.brand : kn.border

                TextInput {
                    id: suchfeld
                    anchors.fill: parent
                    anchors.leftMargin: kn.s2
                    anchors.rightMargin: kn.s2
                    verticalAlignment: TextInput.AlignVCenter
                    font.family: kn.schriftText
                    font.pixelSize: kn.tMd
                    color: kn.ink
                    clip: true
                    // Entprellt, damit die Eingabe nicht blockiert
                    // (SM-SRC-008). Der Schwellwert steht unter OP-18.
                    onTextChanged: entprellung.restart()

                    Text {
                        anchors.verticalCenter: parent.verticalCenter
                        visible: suchfeld.text.length === 0
                        text: qsTr("Suchen…")
                        // Platzhalter ist der einzige zulässige Ort für ink3.
                        color: kn.ink3
                        font.family: kn.schriftText
                        font.pixelSize: kn.tMd
                        textFormat: Text.PlainText
                    }
                }

                Timer {
                    id: entprellung
                    interval: 250
                    onTriggered: bestand.suchen(suchfeld.text)
                }
            }

            Item {
                Layout.fillWidth: true
            }

            Schaltflaeche {
                text: kn.dunkel ? qsTr("Hell") : qsTr("Dunkel")
                gestaltung: kn
                onGeklickt: kn.dunkel = !kn.dunkel
            }
        }
    }

    // ------------------------------------------------------------------
    // Drei Spalten — immer senkrecht, waagerecht scrollbar
    // ------------------------------------------------------------------
    ScrollView {
        anchors.fill: parent
        contentWidth: Math.max(availableWidth, kn.fensterMindestbreite)
        ScrollBar.vertical.policy: ScrollBar.AlwaysOff

        RowLayout {
            width: Math.max(fenster.width, kn.fensterMindestbreite)
            height: fenster.contentItem.height
            spacing: 0

            // --- Navigationsspalte ---
            Rectangle {
                Layout.preferredWidth: kn.navigationVorgabe
                Layout.minimumWidth: kn.navigationMin
                Layout.fillHeight: true
                color: kn.surface2

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: kn.s3
                    spacing: kn.s2

                    Abschnittslabel {
                        text: qsTr("Bibliothek")
                        gestaltung: kn
                    }

                    Text {
                        Layout.fillWidth: true
                        // Fremdtext: ausschließlich Nur-Text (SM-SEC-008).
                        textFormat: Text.PlainText
                        text: bestand.bibliothek_gewaehlt ? bestand.bibliothekspfad : qsTr("Noch keine Bibliothek gewählt")
                        color: bestand.bibliothek_gewaehlt ? kn.ink : kn.ink2
                        font.family: kn.schriftText
                        font.pixelSize: kn.tSm
                        wrapMode: Text.Wrap
                        elide: Text.ElideMiddle
                        maximumLineCount: 3
                    }

                    Abschnittslabel {
                        text: qsTr("Format")
                        gestaltung: kn
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: kn.s1

                        Repeater {
                            model: ["Alle", "PES", "DST", "JEF", "VP3"]
                            Chip {
                                id: formatchip
                                // `pragma ComponentBehavior: Bound` verlangt,
                                // dass eine Modellrolle deklariert wird, statt
                                // aus dem Kontext zu fallen.
                                required property string modelData

                                text: formatchip.modelData
                                gestaltung: kn
                                gewaehlt: fenster.aktivesFormat === formatchip.modelData
                                onGeklickt: {
                                    fenster.aktivesFormat = formatchip.modelData;
                                    bestand.format_filtern(formatchip.modelData === "Alle" ? "" : formatchip.modelData);
                                }
                            }
                        }
                    }

                    Abschnittslabel {
                        text: qsTr("Sortierung")
                        gestaltung: kn
                    }

                    Flow {
                        Layout.fillWidth: true
                        spacing: kn.s1
                        Repeater {
                            model: [
                                {
                                    s: "name",
                                    t: qsTr("Name")
                                },
                                {
                                    s: "stichzahl",
                                    t: qsTr("Stiche")
                                },
                                {
                                    s: "groesse",
                                    t: qsTr("Größe")
                                },
                                {
                                    s: "importdatum",
                                    t: qsTr("Import")
                                }
                            ]
                            Chip {
                                id: sortierchip
                                required property var modelData

                                text: sortierchip.modelData.t
                                gestaltung: kn
                                gewaehlt: fenster.aktiveSortierung === sortierchip.modelData.s
                                onGeklickt: {
                                    fenster.aktiveSortierung = sortierchip.modelData.s;
                                    bestand.sortieren(sortierchip.modelData.s);
                                }
                            }
                        }
                    }

                    Item {
                        Layout.fillHeight: true
                    }
                }

                Naht {
                    anchors.right: parent.right
                    height: parent.height
                    senkrecht: true
                    farbe: kn.seam
                    gestaltung: kn
                }
            }

            // --- Musterauswahl ---
            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumWidth: kn.auswahlMin

                ColumnLayout {
                    anchors.fill: parent
                    spacing: 0

                    // Kopfzeile mit Trefferzahl
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: kn.werkzeugleisteHoehe
                        color: kn.bg

                        RowLayout {
                            anchors.fill: parent
                            anchors.leftMargin: kn.s4
                            anchors.rightMargin: kn.s4

                            Text {
                                text: bestand.gesamt === 1 ? qsTr("1 Muster") : qsTr("%1 Muster").arg(bestand.gesamt)
                                color: kn.ink2
                                font.family: kn.schriftFestbreite
                                font.pixelSize: kn.tSm
                                textFormat: Text.PlainText
                            }

                            Item {
                                Layout.fillWidth: true
                            }

                            Schaltflaeche {
                                text: qsTr("Filter zurücksetzen")
                                gestaltung: kn
                                sichtbar: fenster.aktivesFormat !== "Alle" || suchfeld.text.length > 0
                                onGeklickt: {
                                    suchfeld.clear();
                                    fenster.aktivesFormat = "Alle";
                                    bestand.filter_leeren();
                                }
                            }
                        }

                        Naht {
                            anchors.bottom: parent.bottom
                            width: parent.width
                            farbe: kn.seam
                            gestaltung: kn
                        }
                    }

                    // --- Die drei Zustände nach DES Abschnitt 10 ---

                    // Fehler
                    Rectangle {
                        Layout.fillWidth: true
                        Layout.margins: kn.s4
                        Layout.preferredHeight: fehlertext.implicitHeight + kn.s4
                        visible: bestand.fehlertext.length > 0
                        color: kn.amberSoft
                        radius: kn.radiusMittel
                        border.width: kn.rahmenstaerke
                        border.color: kn.border

                        Text {
                            id: fehlertext
                            anchors.fill: parent
                            anchors.margins: kn.s2
                            text: bestand.fehlertext
                            // Meldungstexte können Fremdanteile tragen.
                            textFormat: Text.PlainText
                            color: kn.warn
                            font.family: kn.schriftText
                            font.pixelSize: kn.tSm
                            wrapMode: Text.Wrap
                        }
                    }

                    // Ladend
                    Item {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        visible: bestand.laedt

                        Column {
                            anchors.centerIn: parent
                            spacing: kn.s3
                            width: Math.min(parent.width - kn.s6 * 2, kn.hinweisMaxBreite)

                            Text {
                                width: parent.width
                                horizontalAlignment: Text.AlignHCenter
                                text: qsTr("Der Bestand wird eingelesen…")
                                color: kn.ink
                                font.family: kn.schriftText
                                font.pixelSize: kn.tMd
                                textFormat: Text.PlainText
                            }

                            // Fortschrittsanzeige (SM-IMP-002). Sie trägt den
                            // Stand zusätzlich als Zahl — ein Balken allein ist
                            // kein zweites Merkmal (SM-NFR-009).
                            Rectangle {
                                width: parent.width
                                height: kn.s2
                                radius: height / 2
                                color: kn.surface3

                                Rectangle {
                                    height: parent.height
                                    radius: parent.radius
                                    color: kn.brand
                                    width: bestand.fortschritt_gesamt > 0 ? parent.width * (bestand.fortschritt / bestand.fortschritt_gesamt) : 0
                                    Behavior on width {
                                        NumberAnimation {
                                            duration: kn.dauerKurz
                                        }
                                    }
                                }
                            }

                            Text {
                                width: parent.width
                                horizontalAlignment: Text.AlignHCenter
                                text: qsTr("%1 von %2 Dateien").arg(bestand.fortschritt).arg(bestand.fortschritt_gesamt)
                                color: kn.ink2
                                font.family: kn.schriftFestbreite
                                font.pixelSize: kn.tSm
                                textFormat: Text.PlainText
                            }

                            Schaltflaeche {
                                anchors.horizontalCenter: parent.horizontalCenter
                                text: qsTr("Abbrechen")
                                gestaltung: kn
                                onGeklickt: bestand.einlesen_abbrechen()
                            }
                        }
                    }

                    // Leer
                    Item {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        visible: !bestand.laedt && bestand.gesamt === 0

                        Column {
                            anchors.centerIn: parent
                            spacing: kn.s3
                            width: Math.min(parent.width - kn.s6 * 2, kn.hinweisMaxBreite)

                            Text {
                                width: parent.width
                                horizontalAlignment: Text.AlignHCenter
                                text: bestand.bibliothek_gewaehlt ? qsTr("In dieser Bibliothek ist noch kein Muster erfasst.") : qsTr("Wählen Sie eine Bibliothek, um zu beginnen.")
                                color: kn.ink
                                font.family: kn.schriftText
                                font.pixelSize: kn.tMd
                                wrapMode: Text.Wrap
                                textFormat: Text.PlainText
                            }

                            Text {
                                width: parent.width
                                horizontalAlignment: Text.AlignHCenter
                                text: bestand.bibliothek_gewaehlt ? qsTr("„Bestand einlesen“ nimmt alle Stickdateien unterhalb des Verzeichnisses auf. Die Originaldateien werden dabei nicht verändert.") : qsTr("StitchManager liest Ihre Stickdateien, ohne sie zu verändern.")
                                color: kn.ink2
                                font.family: kn.schriftText
                                font.pixelSize: kn.tSm
                                wrapMode: Text.Wrap
                                textFormat: Text.PlainText
                            }

                            Schaltflaeche {
                                anchors.horizontalCenter: parent.horizontalCenter
                                text: bestand.bibliothek_gewaehlt ? qsTr("Bestand einlesen") : qsTr("Bibliothek wählen…")
                                gestaltung: kn
                                betont: true
                                onGeklickt: bestand.bibliothek_gewaehlt ? bestand.bestand_einlesen() : ordnerdialog.open()
                            }
                        }
                    }

                    // Gefüllt — virtualisiertes Kachelraster
                    GridView {
                        id: raster
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.margins: kn.s4
                        visible: !bestand.laedt && bestand.gesamt > 0
                        clip: true

                        // Die Kachelhöhe steht VOR dem Laden der Vorschau fest —
                        // kein Layoutsprung (SM-PRV-009).
                        readonly property int kachelBreite: kn.kachelBreite
                        readonly property int kachelHoehe: kn.kachelHoehe

                        cellWidth: kachelBreite + kn.s3
                        cellHeight: kachelHoehe + kn.s3

                        model: bestand
                        // Qt hält nur die sichtbaren Kacheln vor; der Bestand
                        // wandert nie vollständig in das Anzeigemodell
                        // (SM-PRV-007, SM-NFR-003).
                        cacheBuffer: kachelHoehe * 2

                        ScrollBar.vertical: ScrollBar {}

                        onContentYChanged: {
                            // Nachladen, bevor das Ende erreicht ist.
                            if (contentY + height > contentHeight - cellHeight * 2)
                                bestand.mehr_laden();
                        }

                        delegate: Item {
                            id: kacheleintrag

                            width: raster.kachelBreite
                            height: raster.kachelHoehe

                            required property int index
                            required property string name
                            required property string format
                            // Genau die Rollen, die das Modell führt. Eine
                            // `required property` ohne Rolle bricht die
                            // Kachel zur Laufzeit — sie ist damit die Probe
                            // darauf, dass Modell und Kachel denselben
                            // Kachelaufbau nach SM-DES-007 kennen.
                            required property string masse
                            required property string vorschau

                            Rectangle {
                                id: kachel
                                anchors.fill: parent
                                radius: kn.radiusMittel
                                color: kn.surface
                                border.width: gewaehlt ? kn.fokusstaerke : kn.rahmenstaerke
                                border.color: gewaehlt ? kn.brand : (zeiger.containsMouse ? kn.borderStrong : kn.border)

                                readonly property bool gewaehlt: fenster.gewaehlteZeile === kacheleintrag.index

                                Behavior on border.color {
                                    ColorAnimation {
                                        duration: kn.dauerKurz
                                    }
                                }

                                ColumnLayout {
                                    anchors.fill: parent
                                    anchors.margins: kn.s2
                                    spacing: kn.s1

                                    // Bild — feste Höhe, damit nichts springt.
                                    Rectangle {
                                        Layout.fillWidth: true
                                        Layout.preferredHeight: kn.kachelBildHoehe
                                        radius: kn.radiusKlein
                                        color: kn.bg

                                        // Die Quelle kommt aus der Modellrolle. Sie
                                        // ist leer, solange das Bild noch nicht
                                        // vorliegt — der Zeichenpfad wartet nie auf
                                        // Ein-/Ausgabe (SM-NFR-002).
                                        Image {
                                            anchors.fill: parent
                                            anchors.margins: kn.s1
                                            fillMode: Image.PreserveAspectFit
                                            asynchronous: true
                                            cache: true
                                            source: kacheleintrag.vorschau
                                            visible: kacheleintrag.vorschau.length > 0
                                        }

                                        // Ladender Zustand der Kachel
                                        // (DES Abschnitt 10). Die Kachelhöhe
                                        // ändert sich dabei nicht (SM-PRV-009).
                                        Text {
                                            anchors.centerIn: parent
                                            visible: kacheleintrag.vorschau.length === 0
                                            text: qsTr("Vorschau entsteht…")
                                            color: kn.ink3
                                            font.family: kn.schriftText
                                            font.pixelSize: kn.tXs
                                            textFormat: Text.PlainText
                                        }
                                    }

                                    // Marken. Die Formatmarke steht heute
                                    // allein: Die Herkunftsmarke für maschinell
                                    // erzeugte Werte (SM-KIA-008, SM-DES-009)
                                    // kommt mit AP-18 zurück — erst dort
                                    // entstehen solche Werte überhaupt. Eine
                                    // Kennzeichnung, die nie etwas
                                    // kennzeichnet, ist schlechter als keine.
                                    RowLayout {
                                        Layout.fillWidth: true
                                        spacing: kn.s1

                                        Marke {
                                            text: kacheleintrag.format
                                            gestaltung: kn
                                        }

                                        Item {
                                            Layout.fillWidth: true
                                        }
                                    }

                                    // Name — Fremdtext, Nur-Text (SM-SEC-008).
                                    Text {
                                        Layout.fillWidth: true
                                        text: kacheleintrag.name
                                        textFormat: Text.PlainText
                                        color: kn.ink
                                        font.family: kn.schriftText
                                        font.pixelSize: kn.tMd
                                        elide: Text.ElideRight
                                        maximumLineCount: 2
                                        wrapMode: Text.Wrap
                                    }

                                    // Größe. Mehr steht auf der Kachel nicht —
                                    // SM-DES-007 lässt nur Bild, Marke, Name
                                    // und Größe zu.
                                    Text {
                                        Layout.fillWidth: true
                                        text: kacheleintrag.masse
                                        textFormat: Text.PlainText
                                        color: kn.ink2
                                        font.family: kn.schriftFestbreite
                                        font.pixelSize: kn.tXs
                                    }
                                }

                                // Auswahlmarke: Kreuzstich, nicht nur Farbe
                                // (SM-NFR-009 — kein Zustand allein über Farbe).
                                Kreuzstich {
                                    visible: kachel.gewaehlt
                                    gestaltung: kn
                                    anchors.top: parent.top
                                    anchors.right: parent.right
                                    anchors.margins: kn.s1
                                    farbe: kn.brand
                                    groesse: kn.s4
                                }

                                MouseArea {
                                    id: zeiger
                                    anchors.fill: parent
                                    hoverEnabled: true
                                    onClicked: fenster.gewaehlteZeile = kacheleintrag.index
                                }
                            }

                            // Der Fokusring wird nie unterdrückt, auch nicht bei
                            // Zeigerbedienung (DES Abschnitt 7).
                            Rectangle {
                                anchors.fill: parent
                                anchors.margins: -kn.s1
                                visible: parent.activeFocus
                                color: "transparent"
                                radius: kn.radiusMittel
                                border.width: kn.fokusstaerke
                                border.color: kn.brand
                            }

                            focus: true
                            Keys.onReturnPressed: fenster.gewaehlteZeile = index
                            Keys.onSpacePressed: fenster.gewaehlteZeile = index
                        }
                    }
                }

                Naht {
                    anchors.right: parent.right
                    height: parent.height
                    senkrecht: true
                    farbe: kn.seam
                    gestaltung: kn
                }
            }

            // --- Detailbereich ---
            Rectangle {
                Layout.preferredWidth: kn.detailMin
                Layout.minimumWidth: kn.detailMin
                Layout.fillHeight: true
                color: kn.surface

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: kn.s4
                    spacing: kn.s2

                    Text {
                        Layout.fillWidth: true
                        text: qsTr("Detail")
                        font.family: kn.schriftAuszeichnung
                        font.pixelSize: kn.tXl
                        font.weight: Font.DemiBold
                        color: kn.ink
                        textFormat: Text.PlainText
                    }

                    Text {
                        Layout.fillWidth: true
                        visible: fenster.gewaehlteZeile < 0
                        text: qsTr("Wählen Sie ein Muster, um seine Angaben zu sehen.")
                        color: kn.ink2
                        font.family: kn.schriftText
                        font.pixelSize: kn.tSm
                        wrapMode: Text.Wrap
                        textFormat: Text.PlainText
                    }

                    Item {
                        Layout.fillHeight: true
                    }
                }
            }
        }
    }

    property string aktivesFormat: "Alle"
    property string aktiveSortierung: "name"

    // ------------------------------------------------------------------
    // Statusleiste
    // ------------------------------------------------------------------
    footer: Rectangle {
        height: kn.statusleisteHoehe
        color: kn.surface2

        Naht {
            anchors.top: parent.top
            width: parent.width
            farbe: kn.seam
            gestaltung: kn
        }

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: kn.s4
            anchors.rightMargin: kn.s4
            spacing: kn.s4

            Text {
                text: qsTr("Bestand: %1").arg(bestand.gesamt)
                color: kn.ink2
                font.family: kn.schriftFestbreite
                font.pixelSize: kn.tSm
                textFormat: Text.PlainText
            }
            Text {
                text: qsTr("Geladen: %1").arg(bestand.geladen)
                color: kn.ink2
                font.family: kn.schriftFestbreite
                font.pixelSize: kn.tSm
                textFormat: Text.PlainText
            }
            Text {
                visible: fenster.gewaehlteZeile >= 0
                text: qsTr("Auswahl: 1")
                color: kn.ink2
                font.family: kn.schriftFestbreite
                font.pixelSize: kn.tSm
                textFormat: Text.PlainText
            }
            Item {
                Layout.fillWidth: true
            }
        }
    }
}
