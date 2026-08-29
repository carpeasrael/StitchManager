//! Brücke zwischen der Oberfläche und dem Kern.
//!
//! Sie ist die **einzige** Stelle, an der die Oberfläche den Kern erreicht, und
//! sie erreicht ihn ausschließlich über `kern-fassade` (SM-SEC-004,
//! Schnittregel 1). Die Kiste führt `kern-db` nicht in ihren Abhängigkeiten;
//! ein unmittelbarer Zugriff fände den Namen nicht und bräche den Bau.
//!
//! Die Liste ist ein `QAbstractListModel`. Nur so bleibt die Darstellung
//! virtualisiert: Qt fragt ausschließlich die sichtbaren Zeilen ab, und der
//! Bestand wird nie vollständig in ein Anzeigemodell übertragen
//! (SM-PRV-007, SM-NFR-003).
//!
//! **Kein Kernzugriff läuft im Qt-Faden** (SM-NFR-002). Die Fassade gehört dem
//! Arbeitsfaden in `kern-services`; diese Kiste stellt Aufträge und nimmt
//! Antworten entgegen, die über `CxxQtThread` in den Qt-Faden zurückkehren.

use cxx_qt_lib::{
    QByteArray, QHash, QHashPair_i32_QByteArray, QList, QModelIndex, QString, QVariant,
};
use kern_fassade::{Abfrage, Fassade, Kachel, Sortierung};
use kern_services::{Antwort, Befehl, Kernbetrieb, Vorschauauftrag};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

/// Rollennummern. Qt vergibt eigene Rollen ab `Qt::UserRole` (256).
mod rolle {
    // Die Rollen bilden **genau** ab, was DES-STM-001 Abschnitt 6.3 auf der
    // Kachel zulässt: Vorschaufläche, Formatmarke, Name, Größe (SM-DES-007).
    // `UID` trägt keine Anzeige, sondern verknüpft Zeile und Vorschauauftrag.
    //
    // Stichzahl, Farbzahl und Favoritenmarke sind Filter- und Sortiergrößen;
    // sie wirken in der Abfrage, nicht in der Anzeige, und stehen deshalb
    // nicht mehr auf `Kachel`. Die Herkunftsmarke kommt mit AP-18 wieder —
    // erst dort entstehen maschinell erzeugte Werte, die sie kennzeichnen
    // könnte (SM-KIA-008, SM-DES-009).
    //
    // Die Zahlenwerte sind Qt::UserRole-Versätze und bleiben stabil; eine
    // entfernte Rolle gibt ihre Zahl nicht an eine andere weiter.
    pub const NAME: i32 = 256;
    pub const FORMAT: i32 = 257;
    pub const MASSE: i32 = 258;
    pub const UID: i32 = 262;
    pub const VORSCHAU: i32 = 265;
}

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
        include!("cxx-qt-lib/qmodelindex.h");
        type QModelIndex = cxx_qt_lib::QModelIndex;
        include!("cxx-qt-lib/qhash.h");
        type QHash_i32_QByteArray = cxx_qt_lib::QHash<cxx_qt_lib::QHashPair_i32_QByteArray>;
        include!("cxx-qt-lib/qlist.h");
        type QList_i32 = cxx_qt_lib::QList<i32>;
    }

    unsafe extern "C++" {
        include!(<QtCore/QAbstractListModel>);
        /// Basisklasse der virtualisierten Liste.
        type QAbstractListModel;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[base = QAbstractListModel]
        #[qproperty(i32, gesamt)]
        #[qproperty(i32, geladen)]
        #[qproperty(bool, laedt)]
        #[qproperty(bool, bibliothek_gewaehlt)]
        #[qproperty(QString, fehlertext)]
        #[qproperty(QString, bibliothekspfad)]
        #[qproperty(i32, fortschritt)]
        #[qproperty(i32, fortschritt_gesamt)]
        #[qproperty(i32, vorschauen_fertig)]
        type Musterliste = super::MusterlisteRust;
    }

    extern "RustQt" {
        /// Wählt die Bibliothekswurzel. Der Pfad wird geprüft (SM-SEC-003).
        #[qinvokable]
        fn bibliothek_waehlen(self: Pin<&mut Musterliste>, pfad: &QString);

        /// Stößt das Einlesen an. Kehrt sofort zurück (SM-NFR-002).
        #[qinvokable]
        fn bestand_einlesen(self: Pin<&mut Musterliste>);

        /// Bricht einen laufenden Einlesevorgang ab (SM-BAT-005).
        #[qinvokable]
        fn einlesen_abbrechen(self: Pin<&mut Musterliste>);

        /// Setzt den Suchtext und startet einen neuen Suchlauf.
        #[qinvokable]
        fn suchen(self: Pin<&mut Musterliste>, text: &QString);

        /// Schränkt auf ein Format ein; leerer Text hebt die Einschränkung auf.
        #[qinvokable]
        fn format_filtern(self: Pin<&mut Musterliste>, marke: &QString);

        /// Ändert die Sortierung (SM-SRC-005).
        #[qinvokable]
        fn sortieren(self: Pin<&mut Musterliste>, schluessel: &QString);

        /// Lädt den nächsten Ausschnitt nach (Schnittregel 3).
        #[qinvokable]
        fn mehr_laden(self: Pin<&mut Musterliste>);

        /// Löscht Suchtext und Filter.
        #[qinvokable]
        fn filter_leeren(self: Pin<&mut Musterliste>);

        /// Verzeichnis für den Selbsttest, aus der Umgebung gelesen.
        ///
        /// Leer im Regelbetrieb. Ist `SM_SELBSTTEST` gesetzt, fährt die
        /// Oberfläche einen Durchlauf ohne Bedienung und beendet sich; damit
        /// ist die Fadenverdrahtung maschinell prüfbar (SM-NFR-012).
        #[qinvokable]
        fn selbsttest_verzeichnis(self: Pin<&mut Musterliste>) -> QString;
    }

    extern "RustQt" {
        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "rowCount"]
        fn row_count(self: &Musterliste, parent: &QModelIndex) -> i32;

        #[qinvokable]
        #[cxx_override]
        fn data(self: &Musterliste, index: &QModelIndex, role: i32) -> QVariant;

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "roleNames"]
        fn role_names(self: &Musterliste) -> QHash_i32_QByteArray;

        #[qinvokable]
        #[cxx_override]
        #[cxx_name = "canFetchMore"]
        fn can_fetch_more(self: &Musterliste, parent: &QModelIndex) -> bool;
    }

    // Geerbte Methoden der Basisklasse. Sie klammern jede Änderung an der
    // Zeilenmenge, damit Qt die Ansicht gezielt nachführt statt neu aufzubauen.
    unsafe extern "RustQt" {
        #[inherit]
        #[cxx_name = "beginResetModel"]
        fn begin_reset_model(self: Pin<&mut Musterliste>);

        #[inherit]
        #[cxx_name = "endResetModel"]
        fn end_reset_model(self: Pin<&mut Musterliste>);

        #[inherit]
        #[cxx_name = "beginInsertRows"]
        fn begin_insert_rows(
            self: Pin<&mut Musterliste>,
            parent: &QModelIndex,
            first: i32,
            last: i32,
        );

        #[inherit]
        #[cxx_name = "endInsertRows"]
        fn end_insert_rows(self: Pin<&mut Musterliste>);

        /// Meldet Qt, dass sich Rollen einer Zeile geändert haben.
        #[inherit]
        #[cxx_name = "dataChanged"]
        fn data_changed(
            self: Pin<&mut Musterliste>,
            top_left: &QModelIndex,
            bottom_right: &QModelIndex,
            roles: &QList_i32,
        );

        #[inherit]
        fn index(self: &Musterliste, row: i32, column: i32, parent: &QModelIndex) -> QModelIndex;
    }

    impl cxx_qt::Threading for Musterliste {}
}

/// Größe eines Ausschnitts. Klein genug, dass der erste Bildaufbau nicht wartet,
/// groß genug, dass Bildlauf nicht je Zeile nachlädt.
const AUSSCHNITT: i64 = 60;

/// Gewünschte Kantenlänge der Kachelvorschau.
///
/// Sie wird auf eine feste Auflösungsstufe aufgerundet; der Zwischenspeicher
/// hält je Stufe, nicht je beliebiger Pixelgröße (AP-09).
const KACHEL_VORSCHAU_PX: u32 = 320;

#[derive(Default)]
pub struct MusterlisteRust {
    gesamt: i32,
    geladen: i32,
    laedt: bool,
    bibliothek_gewaehlt: bool,
    fehlertext: QString,
    bibliothekspfad: QString,
    fortschritt: i32,
    fortschritt_gesamt: i32,
    vorschauen_fertig: i32,

    /// Der Arbeitsfaden. Er besitzt die Fassade; diese Kiste hat keine.
    betrieb: Option<Kernbetrieb>,
    zeilen: Vec<Kachel>,
    abfrage: Abfrage,

    /// Fertige Vorschauen je Kennung (SM-PRV-002).
    vorschauen: RefCell<HashMap<String, String>>,
    /// Bereits gestellte Vorschauaufträge — jeder wird genau einmal gestellt.
    angefordert: RefCell<HashSet<String>>,
    /// Ein Ausschnittabruf ist unterwegs; ein zweiter würde ihn verdoppeln.
    abruf_laeuft: bool,
}

use core::pin::Pin;
use cxx_qt::{CxxQtType, Threading};

impl qobject::Musterliste {
    /// Setzt einen Fehlertext für die Oberfläche (SM-NFR-006).
    fn melde(mut self: Pin<&mut Self>, text: &str) {
        self.as_mut().set_fehlertext(QString::from(text));
    }

    fn melde_frei(mut self: Pin<&mut Self>) {
        self.as_mut().set_fehlertext(QString::from(""));
    }

    /// Startet den Arbeitsfaden beim ersten Bedarf.
    ///
    /// Nicht in `Default`, weil der Rückkanal den Fadenzeiger des QObjects
    /// braucht — den gibt es erst, wenn das Objekt steht.
    fn betrieb_sichern(mut self: Pin<&mut Self>) {
        if self.rust().betrieb.is_some() {
            return;
        }

        // Die Datenhaltung liegt dauerhaft (SM-DAT-006): Ein Bestand im
        // Arbeitsspeicher wäre nach jedem Programmende verloren, und der
        // inkrementelle Import (SM-IMP-003) läse bei jedem Start die gesamte
        // Bibliothek erneut ein.
        let ablage = kern_typen::anwendungsablage();
        if let Err(e) = std::fs::create_dir_all(&ablage) {
            log::error!("Ablage nicht anlegbar: {e}");
            self.melde("Der Ablageort für die Bibliothek ist nicht beschreibbar.");
            return;
        }

        let Ok(fassade) = Fassade::oeffnen(ablage.join("bestand.db")) else {
            self.melde("Die Bibliothek konnte nicht geöffnet werden.");
            return;
        };

        // Der dauerhafte Ablageort der Vorschauen (SM-PRV-002). Lässt er sich
        // nicht anlegen, arbeitet die Anwendung weiter und zeichnet jedes Mal
        // neu — eine fehlende Ablage hält sie nicht an.
        // Ohne Ablage wird jedes Mal neu gezeichnet; das ist langsamer, aber
        // kein Grund, die Anwendung anzuhalten.
        let speicher = kern_render::Zwischenspeicher::am_standardort().ok();

        let faden = self.qt_thread();
        let betrieb = Kernbetrieb::starten(fassade, speicher, move |antwort| {
            // Aus dem Arbeitsfaden zurück in den Qt-Faden. Erst dort wird ein
            // Eigenschaftswert gesetzt oder das Modell angefasst.
            let _ = faden.queue(move |qobject| {
                qobject.antwort_verarbeiten(antwort);
            });
        });

        self.as_mut().rust_mut().betrieb = Some(betrieb);
    }

    /// Nimmt eine Antwort des Arbeitsfadens entgegen — im Qt-Faden.
    fn antwort_verarbeiten(mut self: Pin<&mut Self>, antwort: Antwort) {
        match antwort {
            Antwort::WurzelGesetzt(pfad) => {
                self.as_mut().set_bibliothek_gewaehlt(true);
                self.as_mut()
                    .set_bibliothekspfad(QString::from(&pfad.display().to_string()));
                self.as_mut().melde_frei();
                self.neu_laden();
            }

            Antwort::EinlesenBegonnen { gesamt } => {
                self.as_mut().set_laedt(true);
                self.as_mut().set_fortschritt(0);
                self.as_mut().set_fortschritt_gesamt(gesamt as i32);
                self.as_mut().melde_frei();
            }

            Antwort::Fortschritt { erledigt, gesamt } => {
                self.as_mut().set_fortschritt(erledigt as i32);
                self.as_mut().set_fortschritt_gesamt(gesamt as i32);
            }

            Antwort::EinlesenFertig {
                befund,
                abgebrochen,
            } => {
                self.as_mut().set_laedt(false);
                // Nach einem Lauf sind die zwischengespeicherten Zuordnungen
                // überholt: Die Zeilen entstehen neu.
                self.as_mut().rust_mut().angefordert.borrow_mut().clear();

                let mut teile: Vec<String> = Vec::new();
                if befund.neu > 0 {
                    teile.push(format!("{} neu aufgenommen", befund.neu));
                }
                if befund.geaendert > 0 {
                    teile.push(format!("{} aktualisiert", befund.geaendert));
                }
                if befund.unveraendert > 0 {
                    teile.push(format!("{} unverändert", befund.unveraendert));
                }
                if befund.vermisst > 0 {
                    teile.push(format!("{} nicht mehr auffindbar", befund.vermisst));
                }
                if befund.wiedergefunden > 0 {
                    teile.push(format!("{} wieder aufgetaucht", befund.wiedergefunden));
                }
                if befund.abgewiesen > 0 {
                    teile.push(format!("{} nicht lesbar", befund.abgewiesen));
                }

                let text = if abgebrochen {
                    format!("Abgebrochen. {}.", teile.join(", "))
                } else if teile.is_empty() {
                    "In diesem Verzeichnis wurden keine Stickdateien gefunden.".to_string()
                } else if befund.neu == 0
                    && befund.geaendert == 0
                    && befund.vermisst == 0
                    && befund.abgewiesen == 0
                {
                    // Der Regelfall eines inkrementellen Laufs: nichts zu tun.
                    format!("Bestand ist aktuell — {} unverändert.", befund.unveraendert)
                } else {
                    format!("{}.", teile.join(", "))
                };

                self.as_mut().melde(&text);
                self.neu_laden();
            }

            Antwort::Seite(seite) => {
                self.as_mut().rust_mut().abruf_laeuft = false;
                if let Some(g) = seite.gesamt {
                    self.as_mut().set_gesamt(g as i32);
                }
                let von = self.rust().zeilen.len() as i32;
                let anzahl = seite.kacheln.len() as i32;
                if anzahl > 0 {
                    let eltern = QModelIndex::default();
                    self.as_mut()
                        .begin_insert_rows(&eltern, von, von + anzahl - 1);
                    self.as_mut().rust_mut().zeilen.extend(seite.kacheln);
                    self.as_mut().end_insert_rows();
                }
                let geladen = self.rust().zeilen.len() as i32;
                self.as_mut().set_geladen(geladen);
            }

            // SP-05 stellt nur den kernseitigen Detail-Leseweg bereit. Die
            // QML-Eigenschaften und Darstellung entstehen erst im folgenden
            // AP-13-Schnitt; bis dahin fordert die Brücke keine Details an.
            Antwort::DetailLaden { .. }
            | Antwort::DetailGeladen { .. }
            | Antwort::DetailNichtGefunden { .. }
            | Antwort::DetailFehler { .. } => {}

            Antwort::VorschauFertig { zeile, uid, pfad } => {
                self.rust()
                    .vorschauen
                    .borrow_mut()
                    .insert(uid, format!("file://{}", pfad.display()));

                let fertig = *self.vorschauen_fertig() + 1;
                self.as_mut().set_vorschauen_fertig(fertig);

                // Genau diese eine Zeile nachführen — nicht die ganze Liste.
                if (zeile as i32) < self.rust().zeilen.len() as i32 {
                    let stelle = self.index(zeile as i32, 0, &QModelIndex::default());
                    let mut rollen = QList::<i32>::default();
                    rollen.append(rolle::VORSCHAU);
                    self.as_mut().data_changed(&stelle, &stelle, &rollen);
                }
            }

            Antwort::Fehler(text) => {
                self.as_mut().rust_mut().abruf_laeuft = false;
                self.as_mut().set_laedt(false);
                self.as_mut().melde(&text);
            }
        }
    }

    pub fn bibliothek_waehlen(mut self: Pin<&mut Self>, pfad: &QString) {
        let pfad = pfad.to_string();
        // Ein leerer Pfad entsteht, wenn der Auswahldialog abgebrochen wird.
        if pfad.trim().is_empty() {
            return;
        }
        self.as_mut().betrieb_sichern();
        if let Some(b) = self.rust().betrieb.as_ref() {
            b.beauftragen(Befehl::WurzelSetzen(std::path::PathBuf::from(pfad)));
        }
    }

    pub fn bestand_einlesen(mut self: Pin<&mut Self>) {
        if !*self.bibliothek_gewaehlt() {
            self.melde("Wählen Sie zuerst eine Bibliothek.");
            return;
        }
        self.as_mut().betrieb_sichern();
        // Der Auftrag kehrt sofort zurück; der Lauf läuft daneben (SM-NFR-002).
        if let Some(b) = self.rust().betrieb.as_ref() {
            b.beauftragen(Befehl::BestandEinlesen);
        }
    }

    pub fn einlesen_abbrechen(self: Pin<&mut Self>) {
        if let Some(b) = self.rust().betrieb.as_ref() {
            b.abbrechen();
        }
    }

    pub fn suchen(mut self: Pin<&mut Self>, text: &QString) {
        let text = text.to_string();
        self.as_mut().rust_mut().abfrage.text = if text.trim().is_empty() {
            None
        } else {
            Some(text)
        };
        self.neu_laden();
    }

    pub fn format_filtern(mut self: Pin<&mut Self>, marke: &QString) {
        let marke = marke.to_string();
        let formate = match kern_typen::Format::aus_endung(&marke) {
            Some(f) => vec![f],
            None => Vec::new(),
        };
        self.as_mut().rust_mut().abfrage.formate = formate;
        self.neu_laden();
    }

    pub fn sortieren(mut self: Pin<&mut Self>, schluessel: &QString) {
        let s = match schluessel.to_string().as_str() {
            "importdatum" => Sortierung::ImportDatum,
            "dateidatum" => Sortierung::DateiDatum,
            "groesse" => Sortierung::Groesse,
            "stichzahl" => Sortierung::Stichzahl,
            "relevanz" => Sortierung::Relevanz,
            _ => Sortierung::Name,
        };
        self.as_mut().rust_mut().abfrage.sortierung = s;
        self.neu_laden();
    }

    pub fn selbsttest_verzeichnis(self: Pin<&mut Self>) -> QString {
        QString::from(&std::env::var("SM_SELBSTTEST").unwrap_or_default())
    }

    pub fn filter_leeren(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().abfrage = Abfrage::default();
        self.neu_laden();
    }

    /// Verwirft die Liste und holt den ersten Ausschnitt samt Gesamtzahl.
    fn neu_laden(mut self: Pin<&mut Self>) {
        self.as_mut().begin_reset_model();
        {
            let mut inneres = self.as_mut().rust_mut();
            inneres.zeilen.clear();
            inneres.angefordert.borrow_mut().clear();
        }
        self.as_mut().end_reset_model();
        self.as_mut().set_geladen(0);
        self.as_mut().set_gesamt(0);
        self.hole_ausschnitt(true);
    }

    pub fn mehr_laden(self: Pin<&mut Self>) {
        self.hole_ausschnitt(false);
    }

    /// Stellt einen Ausschnittauftrag. Die Gesamtzahl nur beim ersten
    /// (Schnittregel 3).
    fn hole_ausschnitt(mut self: Pin<&mut Self>, mit_gesamtzahl: bool) {
        // Ein zweiter Auftrag, während der erste unterwegs ist, lieferte
        // denselben Versatz doppelt.
        if self.rust().abruf_laeuft {
            return;
        }

        let versatz = self.rust().zeilen.len() as i64;
        if !mit_gesamtzahl && versatz >= *self.gesamt() as i64 {
            return;
        }

        self.as_mut().betrieb_sichern();
        let abfrage = self.rust().abfrage.clone();
        self.as_mut().rust_mut().abruf_laeuft = true;

        if let Some(b) = self.rust().betrieb.as_ref() {
            b.beauftragen(Befehl::Ausschnitt {
                abfrage,
                versatz,
                anzahl: AUSSCHNITT,
                mit_gesamtzahl,
            });
        }
    }

    // --- QAbstractListModel ---

    pub fn row_count(&self, _parent: &QModelIndex) -> i32 {
        self.rust().zeilen.len() as i32
    }

    pub fn can_fetch_more(&self, _parent: &QModelIndex) -> bool {
        (self.rust().zeilen.len() as i32) < *self.gesamt()
    }

    pub fn data(&self, index: &QModelIndex, role: i32) -> QVariant {
        let zeile = index.row() as usize;
        let Some(k) = self.rust().zeilen.get(zeile) else {
            return QVariant::default();
        };

        // Fremdtext — Dateinamen und Metadaten — wird ausschließlich als
        // Nur-Text geliefert (SM-SEC-008). Die Oberfläche stellt ihn in
        // Elementen ohne Auszeichnungsfähigkeit dar.
        match role {
            rolle::NAME => QVariant::from(&QString::from(&k.name)),
            rolle::FORMAT => QVariant::from(&QString::from(&k.format)),
            rolle::MASSE => {
                let text = match (k.breite_mm, k.hoehe_mm) {
                    (Some(b), Some(h)) => format!("{b:.0} × {h:.0} mm"),
                    _ => String::from("—"),
                };
                QVariant::from(&QString::from(&text))
            }
            rolle::UID => QVariant::from(&QString::from(&k.uid)),
            // Die Kachel zeigt keine Garnfarben (SM-DES-007); sie werden für
            // den ausgewählten Eintrag im Detailbereich geladen.
            rolle::VORSCHAU => {
                // Diese Rolle wird im Zeichenpfad abgefragt. Sie **wartet
                // nie** auf Ein-/Ausgabe (SM-NFR-002): Entweder das Bild liegt
                // schon vor, oder es wird angefordert und die Kachel bleibt so
                // lange leer. Ist es fertig, meldet der Rückkanal
                // `dataChanged` für genau diese Zeile.
                if let Some(p) = self.rust().vorschauen.borrow().get(&k.uid) {
                    return QVariant::from(&QString::from(p));
                }
                if self.rust().angefordert.borrow_mut().insert(k.uid.clone()) {
                    if let Some(b) = self.rust().betrieb.as_ref() {
                        b.vorschau_anfordern(Vorschauauftrag {
                            zeile,
                            uid: k.uid.clone(),
                            pfad: k.pfad.clone(),
                            breite_px: KACHEL_VORSCHAU_PX,
                        });
                    }
                }
                QVariant::from(&QString::from(""))
            }
            _ => QVariant::default(),
        }
    }

    pub fn role_names(&self) -> QHash<QHashPair_i32_QByteArray> {
        let mut rollen = QHash::<QHashPair_i32_QByteArray>::default();
        rollen.insert(rolle::NAME, QByteArray::from("name"));
        rollen.insert(rolle::FORMAT, QByteArray::from("format"));
        rollen.insert(rolle::MASSE, QByteArray::from("masse"));
        rollen.insert(rolle::UID, QByteArray::from("uid"));
        rollen.insert(rolle::VORSCHAU, QByteArray::from("vorschau"));
        rollen
    }
}

// **Was hier bewusst nicht mehr steht.** Die Hilfe `tausender` setzte
// Tausenderpunkte nach deutscher Schreibung (SM-SET-006). Ihr einziger
// Aufrufer war die Kachelrolle `stichzahl`, und die Stichzahl steht nach
// SM-DES-007 nicht auf der Kachel. Eine Hilfe ohne Aufrufer ist toter Code;
// sie entsteht in AP-13 neu, wo der Detailbereich Stichzahl und Farbzahl
// tatsächlich anzeigt — dann zusammen mit ihrem Prüffall.
