//! Prüffälle des Kernbetriebs.
//!
//! Sie laufen **ohne Qt**: Der Rückkanal ist ein Verschluss. Genau deshalb ist
//! die Zusage aus SM-NFR-002 hier überhaupt prüfbar.

use super::*;

/// Schreibt eine echte Stickdatei über ein geprüftes Ziel (Schnittregel 5).
fn schreibe_muster(ziel: &std::path::Path, punkte: Vec<(f64, f64)>) {
    let wurzel = kern_security::Wurzel::neu(ziel.parent().unwrap()).unwrap();
    let geprueft = wurzel.schreibziel(ziel).unwrap();
    kern_parsers::writers::write_dst(
        &[kern_parsers::StitchSegment {
            color_index: 0,
            color_hex: Some("C8102E".into()),
            points: punkte,
        }],
        &geprueft,
    )
    .unwrap();
}
use std::sync::mpsc;
use std::time::{Duration, Instant};

/// Sammelt Antworten und lässt auf einzelne warten.
struct Horcher {
    empfang: mpsc::Receiver<Antwort>,
}

impl Horcher {
    fn neu() -> (Self, impl Fn(Antwort) + Send + 'static) {
        let (sender, empfang) = mpsc::channel();
        (Self { empfang }, move |a| {
            let _ = sender.send(a);
        })
    }

    /// Wartet, bis eine Antwort passt — oder die Frist abläuft.
    fn warte_auf<T>(&self, was: &str, mut pruefe: impl FnMut(&Antwort) -> Option<T>) -> T {
        let frist = Instant::now() + Duration::from_secs(20);
        while Instant::now() < frist {
            match self.empfang.recv_timeout(Duration::from_millis(200)) {
                Ok(a) => {
                    if let Some(t) = pruefe(&a) {
                        return t;
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
        panic!("Antwort {was} kam nicht innerhalb der Frist");
    }
}

/// Legt einen Prüfbestand aus echten, lesbaren Stickdateien an.
fn bestand_anlegen(n: usize) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    for i in 0..n {
        let punkte: Vec<(f64, f64)> = (0..40)
            .map(|k| (k as f64, ((k * (i + 1)) % 17) as f64))
            .collect();
        schreibe_muster(&tmp.path().join(format!("muster{i:04}.dst")), punkte);
    }
    tmp
}

/// Legt einen Betrieb an, dessen Zwischenspeicher **neben** dem Prüfbestand
/// liegt — nie im dauerhaften Speicher der Nutzerin.
fn betrieb_mit(wurzel: &std::path::Path) -> (Kernbetrieb, Horcher) {
    let fassade = Fassade::im_speicher().unwrap();
    let (horcher, melden) = Horcher::neu();
    let speicher =
        kern_render::Zwischenspeicher::an(wurzel.parent().unwrap().join("speicher")).unwrap();
    let betrieb = Kernbetrieb::starten(fassade, Some(speicher), melden);
    betrieb.beauftragen(Befehl::WurzelSetzen(wurzel.to_path_buf()));
    horcher.warte_auf("WurzelGesetzt", |a| match a {
        Antwort::WurzelGesetzt(_) => Some(()),
        _ => None,
    });
    (betrieb, horcher)
}

// --- SM-NFR-002 / SM-IMP-002: Import im Hintergrund mit Fortschritt ---

#[test]
fn einlesen_meldet_beginn_fortschritt_und_ende() {
    let tmp = bestand_anlegen(60);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::BestandEinlesen);

    let gesamt = horcher.warte_auf("EinlesenBegonnen", |a| match a {
        Antwort::EinlesenBegonnen { gesamt } => Some(*gesamt),
        _ => None,
    });
    assert_eq!(gesamt, 60);

    let mut zuletzt = 0usize;
    let (aufgenommen, abgebrochen) = horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::Fortschritt { erledigt, gesamt } => {
            assert!(
                *erledigt >= zuletzt,
                "Fortschritt lief rückwärts: {zuletzt} → {erledigt}"
            );
            assert!(erledigt <= gesamt, "Fortschritt über die Gesamtzahl hinaus");
            zuletzt = *erledigt;
            None
        }
        Antwort::EinlesenFertig {
            befund,
            abgebrochen,
        } => Some((befund.neu, *abgebrochen)),
        _ => None,
    });

    assert_eq!(aufgenommen, 60);
    assert!(!abgebrochen);
    assert_eq!(zuletzt, 60, "der Schlussstand wurde nicht gemeldet");
}

#[test]
fn der_aufrufende_faden_bleibt_frei() {
    // Die eigentliche Zusage aus SM-NFR-002: Der Auftrag kehrt sofort zurück,
    // der Lauf läuft daneben weiter.
    let tmp = bestand_anlegen(400);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    let vorher = Instant::now();
    betrieb.beauftragen(Befehl::BestandEinlesen);
    let dauer = vorher.elapsed();

    assert!(
        dauer < Duration::from_millis(50),
        "Der Auftrag blockierte den aufrufenden Faden {dauer:?} lang"
    );

    // Und der Lauf läuft tatsächlich.
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { .. } => Some(()),
        _ => None,
    });
}

// --- SM-BAT-005: Abbruch ---

#[test]
fn langer_lauf_ist_abbrechbar() {
    let tmp = bestand_anlegen(1500);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenBegonnen", |a| match a {
        Antwort::EinlesenBegonnen { .. } => Some(()),
        _ => None,
    });

    betrieb.abbrechen();

    let (aufgenommen, abgebrochen) = horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig {
            befund,
            abgebrochen,
        } => Some((befund.neu, *abgebrochen)),
        _ => None,
    });

    println!("nach dem Abbruch aufgenommen: {aufgenommen} von 1500");
    assert!(abgebrochen, "der Lauf meldete sich nicht als abgebrochen");
    // Gemessen greift der Abbruch nach der ersten Datei. Die Schranke liegt
    // bewusst weit darüber, prüft aber, dass er *zwischen* zwei Dateien wirkt
    // und nicht erst am Ende des Laufs.
    assert!(
        aufgenommen < 200,
        "der Abbruch wirkte zu spät — {aufgenommen} von 1500 Dateien liefen noch durch"
    );
}

// --- SM-NFR-005: defekte Dateien halten den Lauf nicht an ---

#[test]
fn defekte_dateien_halten_den_lauf_nicht_an() {
    // Zwei Zusagen in einem Fall. SM-NFR-005: Der Lauf läuft über die defekten
    // Dateien hinweg zu Ende. SM-IMP-009: Er **erfasst** sie dabei, statt sie
    // zu überspringen — eine still verschwundene Datei ist für die Nutzerin
    // unauffindbar. Der Fehlergrund an der Kachel ist der Beleg dafür; die
    // Zahl `abgewiesen` ist es seit SM-IMP-009 gerade **nicht** mehr, denn
    // eine erfasste Datei ist nicht abgewiesen.
    let tmp = bestand_anlegen(10);
    // Fünf unlesbare Dateien dazwischen.
    for i in 0..5 {
        std::fs::write(tmp.path().join(format!("kaputt{i}.dst")), vec![0xFFu8; 900]).unwrap();
    }

    let (_betrieb, _horcher, kacheln) = kacheln_aus_bestand(tmp.path());

    assert_eq!(kacheln.len(), 15, "der Lauf hat Dateien verloren");

    let defekt = kacheln.iter().filter(|k| k.fehlergrund.is_some()).count();
    assert_eq!(
        defekt, 5,
        "defekte Dateien tragen keinen Fehlergrund — SM-IMP-009 verfehlt"
    );
    assert!(
        kacheln
            .iter()
            .filter(|k| k.fehlergrund.is_none())
            .all(|k| k.breite_mm.is_some()),
        "eine gültige Datei wurde als unlesbar erfasst"
    );
}

// --- Vorrang und Schlangenverhalten ---

#[test]
fn befehle_haben_vorrang_vor_wartenden_vorschauen() {
    let tmp = bestand_anlegen(4);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { .. } => Some(()),
        _ => None,
    });

    // Die Schlange mit Vorschauen füllen …
    for i in 0..200 {
        betrieb.vorschau_anfordern(Vorschauauftrag {
            zeile: i,
            uid: format!("u{i}"),
            pfad: format!("muster{i:04}.dst"),
            breite_px: 320,
        });
    }
    // … und danach einen Befehl stellen.
    betrieb.beauftragen(Befehl::Ausschnitt {
        abfrage: Abfrage::default(),
        versatz: 0,
        anzahl: 10,
        mit_gesamtzahl: true,
    });

    // Der Ausschnitt muss kommen, ohne dass erst 200 Bilder entstehen.
    let anfang = Instant::now();
    horcher.warte_auf("Seite", |a| match a {
        Antwort::Seite(_) => Some(()),
        _ => None,
    });
    assert!(
        anfang.elapsed() < Duration::from_secs(5),
        "Der Ausschnitt kam erst nach {:?} — die Vorschauen hatten Vorrang",
        anfang.elapsed()
    );
}

#[test]
fn vorschauschlange_laeuft_nicht_ueber() {
    let tmp = bestand_anlegen(1);
    let (betrieb, _horcher) = betrieb_mit(tmp.path());

    for i in 0..(VORSCHAU_SCHLANGE_MAX * 3) {
        betrieb.vorschau_anfordern(Vorschauauftrag {
            zeile: i,
            uid: format!("u{i}"),
            // Ein Pfad, den es nicht gibt: Der Auftrag scheitert schnell und
            // die Schlange bleibt gefüllt, statt leergearbeitet zu werden.
            pfad: format!("/gibtsnicht/{i}.dst"),
            breite_px: 320,
        });
    }

    assert!(
        betrieb.wartende_vorschauen() <= VORSCHAU_SCHLANGE_MAX,
        "die Schlange wuchs auf {} Einträge",
        betrieb.wartende_vorschauen()
    );
}

// --- Ausschnittabruf über den Betrieb ---

#[test]
fn ausschnitt_kommt_ueber_den_rueckkanal() {
    let tmp = bestand_anlegen(25);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { .. } => Some(()),
        _ => None,
    });

    betrieb.beauftragen(Befehl::Ausschnitt {
        abfrage: Abfrage::default(),
        versatz: 0,
        anzahl: 10,
        mit_gesamtzahl: true,
    });

    let (zeilen, gesamt) = horcher.warte_auf("Seite", |a| match a {
        Antwort::Seite(s) => Some((s.kacheln.len(), s.gesamt)),
        _ => None,
    });
    assert_eq!(zeilen, 10);
    assert_eq!(gesamt, Some(25));
}

// --- AP-13: Detail-Lesepfad im Arbeitsfaden ---

#[test]
fn detail_meldet_laden_und_erfolg_mit_anfragenummer() {
    let tmp = bestand_anlegen(2);
    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(tmp.path());
    let uid = kacheln[0].uid.clone();

    betrieb.beauftragen(Befehl::DetailLaden {
        anfrage_id: 41,
        uid: uid.clone(),
    });

    let (lade_id, lade_uid) = horcher.warte_auf("DetailLaden", |a| match a {
        Antwort::DetailLaden { anfrage_id, uid } => Some((*anfrage_id, uid.clone())),
        _ => None,
    });
    assert_eq!((lade_id, lade_uid), (41, uid.clone()));

    let (antwort_id, detail) = horcher.warte_auf("DetailGeladen", |a| match a {
        Antwort::DetailGeladen { anfrage_id, detail } => Some((*anfrage_id, detail.clone())),
        _ => None,
    });
    assert_eq!(antwort_id, 41);
    assert_eq!(detail.uid, uid);
    assert_eq!(detail.format, "DST");
    assert!(detail.breite_mm.is_some());
    assert!(detail.hoehe_mm.is_some());
    assert!(detail.stichzahl.is_some());
    assert!(detail.farbzahl.is_some());
    assert_eq!(detail.thema, None);
    assert_eq!(detail.beschreibung, None);
    assert_eq!(detail.notizen, None);
    // DST enthält keine eingebettete, benannte Garnpalette. Der vollständige
    // Transport einer vorhandenen Palette wird im folgenden Abbildungstest
    // unabhängig vom Dateiformat geprüft.
    assert!(detail.garnfarben.is_empty());
}

#[test]
fn dienstabbildung_erhaelt_vollstaendigen_detaildatensatz() {
    let (horcher, melden) = Horcher::neu();
    let erwartet = EintragDetail {
        uid: "detail-vollstaendig".into(),
        name: "Blütenrand".into(),
        thema: Some("Botanik".into()),
        beschreibung: Some("Zweifarbige Bordüre".into()),
        notizen: Some("Auf Leinen geprüft".into()),
        format: "PES".into(),
        breite_mm: Some(84.2),
        hoehe_mm: Some(31.7),
        stichzahl: Some(4_212),
        farbzahl: Some(2),
        fehlerstatus: Some("warnung".into()),
        fehlergrund: Some("Farbzuordnung prüfen".into()),
        schlagworte: vec!["Rand".into(), "Blume".into()],
        garnfarben: vec![
            kern_typen::Garnfarbe {
                hex: "C8102E".into(),
                name: Some("Rot".into()),
                marke: Some("Prüfmarke".into()),
                markenschluessel: Some("R-01".into()),
            },
            kern_typen::Garnfarbe {
                hex: "0047AB".into(),
                name: Some("Blau".into()),
                marke: None,
                markenschluessel: None,
            },
        ],
    };

    detailantwort_melden(
        55,
        erwartet.uid.clone(),
        Ok(Some(erwartet.clone())),
        &melden,
    );

    let (anfrage_id, erhalten) = horcher.warte_auf("vollständiges Detail", |a| match a {
        Antwort::DetailGeladen { anfrage_id, detail } => Some((*anfrage_id, detail.clone())),
        _ => None,
    });
    assert_eq!(anfrage_id, 55);
    assert_eq!(*erhalten, erwartet);
}

#[test]
fn unbekannte_detailkennung_hat_eigene_antwort() {
    let tmp = bestand_anlegen(1);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::DetailLaden {
        anfrage_id: 77,
        uid: "nicht-da".into(),
    });

    let (anfrage_id, uid) = horcher.warte_auf("DetailNichtGefunden", |a| match a {
        Antwort::DetailNichtGefunden { anfrage_id, uid } => Some((*anfrage_id, uid.clone())),
        _ => None,
    });
    assert_eq!(anfrage_id, 77);
    assert_eq!(uid, "nicht-da");
}

#[test]
fn detailantworten_bleiben_bei_schnellem_auswahlwechsel_zuordenbar() {
    let tmp = bestand_anlegen(2);
    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(tmp.path());
    let erste_uid = kacheln[0].uid.clone();
    let zweite_uid = kacheln[1].uid.clone();

    betrieb.beauftragen(Befehl::DetailLaden {
        anfrage_id: 100,
        uid: erste_uid.clone(),
    });
    betrieb.beauftragen(Befehl::DetailLaden {
        anfrage_id: 101,
        uid: zweite_uid.clone(),
    });

    let mut antworten = std::collections::BTreeMap::new();
    while antworten.len() < 2 {
        let (id, uid) = horcher.warte_auf("DetailGeladen nach Auswahlwechsel", |a| match a {
            Antwort::DetailGeladen { anfrage_id, detail } => {
                Some((*anfrage_id, detail.uid.clone()))
            }
            _ => None,
        });
        antworten.insert(id, uid);
    }
    assert_eq!(antworten.get(&100), Some(&erste_uid));
    assert_eq!(antworten.get(&101), Some(&zweite_uid));
}

#[test]
fn datenfehler_hat_eigene_detailantwort() {
    let (horcher, melden) = Horcher::neu();
    detailantwort_melden(
        88,
        "detail-fehler".into(),
        Err(kern_typen::Fehler::Datenbank(
            "Die Bibliothek konnte den Vorgang nicht ausführen.".into(),
        )),
        &melden,
    );

    let (anfrage_id, uid, text) = horcher.warte_auf("DetailFehler", |a| match a {
        Antwort::DetailFehler {
            anfrage_id,
            uid,
            text,
        } => Some((*anfrage_id, uid.clone(), text.clone())),
        _ => None,
    });
    assert_eq!(anfrage_id, 88);
    assert_eq!(uid, "detail-fehler");
    assert!(text.contains("Bibliothek"));
}

#[test]
fn vorschau_wird_erzeugt_und_gemeldet() {
    let tmp = bestand_anlegen(3);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { .. } => Some(()),
        _ => None,
    });

    betrieb.vorschau_anfordern(Vorschauauftrag {
        zeile: 0,
        uid: "pruef-uid".into(),
        pfad: "muster0000.dst".into(),
        breite_px: 320,
    });

    let pfad = horcher.warte_auf("VorschauFertig", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });
    assert!(pfad.exists(), "die gemeldete Vorschau liegt nicht am Ort");
    let kopf = std::fs::read(&pfad).unwrap();
    assert_eq!(&kopf[1..4], b"PNG");
    let _ = std::fs::remove_file(&pfad);
}

#[test]
fn wurzel_ausserhalb_meldet_verstaendlichen_fehler() {
    let fassade = Fassade::im_speicher().unwrap();
    let (horcher, melden) = Horcher::neu();
    let betrieb = Kernbetrieb::starten(fassade, None, melden);

    betrieb.beauftragen(Befehl::WurzelSetzen(PathBuf::from("/etc")));

    let text = horcher.warte_auf("Fehler", |a| match a {
        Antwort::Fehler(t) => Some(t.clone()),
        _ => None,
    });
    // Der Text richtet sich an Endnutzer (SM-NFR-006).
    assert!(
        text.contains("Systemverzeichnisse") || text.contains("nicht"),
        "unverständlicher Text: {text}"
    );
}

#[test]
fn beenden_haelt_einen_laufenden_lauf_nicht_auf() {
    let tmp = bestand_anlegen(2000);
    let (betrieb, horcher) = betrieb_mit(tmp.path());
    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenBegonnen", |a| match a {
        Antwort::EinlesenBegonnen { .. } => Some(()),
        _ => None,
    });

    let anfang = Instant::now();
    drop(betrieb);
    assert!(
        anfang.elapsed() < Duration::from_secs(10),
        "das Beenden wartete {:?} auf den Importlauf",
        anfang.elapsed()
    );
}

// --- SM-PRV-002 / SM-PRV-003 im Zusammenspiel ---

/// Baut einen Bestand mit Unterordnern und liefert die Kacheln.
fn kacheln_aus_bestand(tmp: &std::path::Path) -> (Kernbetrieb, Horcher, Vec<kern_fassade::Kachel>) {
    let (betrieb, horcher) = betrieb_mit(tmp);
    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { .. } => Some(()),
        _ => None,
    });
    betrieb.beauftragen(Befehl::Ausschnitt {
        abfrage: Abfrage::default(),
        versatz: 0,
        anzahl: 50,
        mit_gesamtzahl: true,
    });
    let kacheln = horcher.warte_auf("Seite", |a| match a {
        Antwort::Seite(s) => Some(s.kacheln.clone()),
        _ => None,
    });
    (betrieb, horcher, kacheln)
}

#[test]
fn vorschau_entsteht_auch_fuer_dateien_in_unterordnern() {
    // Der Vorschauweg kannte zuvor nur den Dateinamen und löste ihn gegen die
    // Wurzel auf. Bei jeder Unterordnerstruktur (SM-LIB-002, SM-LIB-003) fand
    // er die Datei damit nicht, und die Kachel blieb dauerhaft leer.
    let wurzel = tempfile::tempdir().unwrap();
    let bestand = wurzel.path().join("bestand");
    let tief = bestand.join("tiere").join("voegel");
    std::fs::create_dir_all(&tief).unwrap();

    let punkte: Vec<(f64, f64)> = (0..40).map(|k| (k as f64, (k % 11) as f64)).collect();
    schreibe_muster(&tief.join("moewe.dst"), punkte);

    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(&bestand);
    assert_eq!(
        kacheln.len(),
        1,
        "die Datei im Unterordner wurde nicht erfasst"
    );

    betrieb.vorschau_anfordern(Vorschauauftrag {
        zeile: 0,
        uid: kacheln[0].uid.clone(),
        pfad: kacheln[0].pfad.clone(),
        breite_px: 320,
    });

    let pfad = horcher.warte_auf("VorschauFertig", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });
    assert!(
        pfad.is_file(),
        "für die Datei im Unterordner entstand keine Vorschau"
    );
}

#[test]
fn zweite_anforderung_kommt_aus_dem_zwischenspeicher() {
    // Beweisführung ohne Zeitmessung: Nach dem ersten Lauf wird die abgelegte
    // Datei durch eine erkennbare Marke ersetzt. Kommt sie zurück, stammt die
    // Antwort aus dem Zwischenspeicher und nicht aus einem zweiten Zeichenlauf.
    let wurzel = tempfile::tempdir().unwrap();
    let bestand = wurzel.path().join("bestand");
    std::fs::create_dir_all(&bestand).unwrap();
    let punkte: Vec<(f64, f64)> = (0..40).map(|k| (k as f64, (k % 9) as f64)).collect();
    schreibe_muster(&bestand.join("muster.dst"), punkte);

    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(&bestand);
    let auftrag = Vorschauauftrag {
        zeile: 0,
        uid: kacheln[0].uid.clone(),
        pfad: kacheln[0].pfad.clone(),
        breite_px: 320,
    };

    betrieb.vorschau_anfordern(auftrag.clone());
    let erst = horcher.warte_auf("VorschauFertig", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });
    assert_eq!(&std::fs::read(&erst).unwrap()[1..4], b"PNG");

    std::fs::write(&erst, b"MARKE-AUS-DEM-SPEICHER").unwrap();

    betrieb.vorschau_anfordern(auftrag);
    let zweit = horcher.warte_auf("VorschauFertig (zweiter Lauf)", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });

    assert_eq!(
        erst, zweit,
        "die zweite Antwort zeigte auf einen anderen Ort"
    );
    assert_eq!(
        std::fs::read(&zweit).unwrap(),
        b"MARKE-AUS-DEM-SPEICHER",
        "die Vorschau wurde neu gezeichnet, statt aus dem Zwischenspeicher zu kommen"
    );
}

#[test]
fn geaenderte_quelldatei_erzwingt_eine_neue_vorschau() {
    // SM-PRV-003 im Zusammenspiel: Die Marke aus dem Zwischenspeicher darf
    // nach einer Änderung der Quelldatei nicht mehr zurückkommen.
    let wurzel = tempfile::tempdir().unwrap();
    let bestand = wurzel.path().join("bestand");
    std::fs::create_dir_all(&bestand).unwrap();
    let quelle = bestand.join("muster.dst");
    let schreibe = |n: usize, ziel: &std::path::Path| {
        let punkte: Vec<(f64, f64)> = (0..n).map(|k| (k as f64, (k % 13) as f64)).collect();
        schreibe_muster(ziel, punkte);
    };
    schreibe(40, &quelle);

    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(&bestand);
    let auftrag = Vorschauauftrag {
        zeile: 0,
        uid: kacheln[0].uid.clone(),
        pfad: kacheln[0].pfad.clone(),
        breite_px: 320,
    };

    betrieb.vorschau_anfordern(auftrag.clone());
    let erst = horcher.warte_auf("VorschauFertig", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });
    std::fs::write(&erst, b"MARKE-AUS-DEM-SPEICHER").unwrap();

    // Die Quelldatei ändert sich — Länge und Änderungszeit weichen ab.
    schreibe(200, &quelle);

    betrieb.vorschau_anfordern(auftrag);
    let zweit = horcher.warte_auf("VorschauFertig (nach Änderung)", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });

    let inhalt = std::fs::read(&zweit).unwrap();
    assert_ne!(
        inhalt, b"MARKE-AUS-DEM-SPEICHER",
        "nach der Änderung kam die überholte Vorschau zurück — SM-PRV-003 verletzt"
    );
    assert_eq!(&inhalt[1..4], b"PNG", "es entstand kein neues Bild");
}

// --- SM-IMP-003: inkrementeller Lauf über den Kernbetrieb ---

fn einlesen_und_warten(betrieb: &Kernbetrieb, horcher: &Horcher) -> kern_fassade::Importbefund {
    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenFertig", |a| match a {
        Antwort::EinlesenFertig { befund, .. } => Some(*befund),
        _ => None,
    })
}

/// Pfade und Inhaltsdigests aller Quelldateien unterhalb der Bibliothek.
///
/// Der relative Pfad belegt, dass keine Datei verschoben oder umbenannt wurde;
/// der Digest belegt den unveränderten Inhalt. Größe allein genügt dafür nicht.
fn quelldaten_abgleichen(
    wurzel: &std::path::Path,
) -> std::collections::BTreeMap<std::path::PathBuf, u64> {
    use std::hash::{Hash, Hasher};

    fn besuchen(
        wurzel: &std::path::Path,
        ordner: &std::path::Path,
        abbild: &mut std::collections::BTreeMap<std::path::PathBuf, u64>,
    ) {
        let mut eintraege: Vec<_> = std::fs::read_dir(ordner)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        eintraege.sort();

        for pfad in eintraege {
            if pfad.is_dir() {
                besuchen(wurzel, &pfad, abbild);
                continue;
            }
            let daten = std::fs::read(&pfad).unwrap();
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            daten.hash(&mut hasher);
            abbild.insert(
                pfad.strip_prefix(wurzel).unwrap().to_path_buf(),
                hasher.finish(),
            );
        }
    }

    let mut abbild = std::collections::BTreeMap::new();
    besuchen(wurzel, wurzel, &mut abbild);
    abbild
}

fn quellbestand_ist_unveraendert(
    vorher: &std::collections::BTreeMap<std::path::PathBuf, u64>,
    wurzel: &std::path::Path,
    lauf: &str,
) {
    assert_eq!(
        &quelldaten_abgleichen(wurzel),
        vorher,
        "PF-MIG-05: Der Import hat Quelldateien beim {lauf} verändert"
    );
}

// --- PF-MIG-05 / SM-MIG-005: Quelldaten bleiben unverändert ---

#[test]
fn pf_mig_05_import_veraendert_keine_quelldatei() {
    let tmp = tempfile::tempdir().unwrap();
    let bestand = tmp.path().join("bestand");
    let tief = bestand.join("tiere").join("voegel");
    std::fs::create_dir_all(&tief).unwrap();

    // Genügend Arbeit, damit der Abbruch sicher zwischen zwei Dateien greift.
    // Ein Teil liegt in Unterordnern; die beschädigte Datei muss trotzdem als
    // sichtbarer Fehlereintrag in den Bestand gelangen.
    for i in 0..350 {
        let ordner = if i % 3 == 0 { &tief } else { &bestand };
        let punkte: Vec<(f64, f64)> = (0..40)
            .map(|k| (k as f64, ((k * (i + 1)) % 19) as f64))
            .collect();
        schreibe_muster(&ordner.join(format!("quelle{i:04}.dst")), punkte);
    }
    std::fs::write(tief.join("beschaedigt.pes"), vec![0xFFu8; 4096]).unwrap();

    let (betrieb, horcher) = betrieb_mit(&bestand);

    let vor_abbruch = quelldaten_abgleichen(&bestand);
    assert_eq!(vor_abbruch.len(), 351, "der Prüfbestand ist unvollständig");
    betrieb.beauftragen(Befehl::BestandEinlesen);
    horcher.warte_auf("EinlesenBegonnen", |a| match a {
        Antwort::EinlesenBegonnen { gesamt } => Some(*gesamt),
        _ => None,
    });
    betrieb.abbrechen();
    let abgebrochen = horcher.warte_auf("EinlesenFertig nach Abbruch", |a| match a {
        Antwort::EinlesenFertig { abgebrochen, .. } => Some(*abgebrochen),
        _ => None,
    });
    assert!(
        abgebrochen,
        "der Prüflauf wurde nicht als abgebrochen gemeldet"
    );
    quellbestand_ist_unveraendert(&vor_abbruch, &bestand, "abgebrochenen Lauf");

    let vor_erstlauf = quelldaten_abgleichen(&bestand);
    einlesen_und_warten(&betrieb, &horcher);
    quellbestand_ist_unveraendert(&vor_erstlauf, &bestand, "vollständigen Erstlauf");

    betrieb.beauftragen(Befehl::Ausschnitt {
        abfrage: Abfrage::default(),
        versatz: 0,
        anzahl: 400,
        mit_gesamtzahl: true,
    });
    let kacheln = horcher.warte_auf("Seite nach Erstlauf", |a| match a {
        Antwort::Seite(s) => Some(s.kacheln.clone()),
        _ => None,
    });
    assert_eq!(
        kacheln.len(),
        351,
        "der Erstlauf hat Quelldateien ausgelassen"
    );
    assert!(
        kacheln.iter().any(|k| {
            k.pfad.ends_with("tiere/voegel/beschaedigt.pes") && k.fehlergrund.is_some()
        }),
        "die beschädigte Quelldatei wurde nicht als Fehlereintrag erfasst"
    );

    let vor_zweitlauf = quelldaten_abgleichen(&bestand);
    let zweit = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(
        zweit.neu, 0,
        "der inkrementelle Lauf nahm Dateien erneut auf"
    );
    assert_eq!(
        zweit.geaendert, 0,
        "der inkrementelle Lauf meldete Änderungen"
    );
    assert_eq!(zweit.unveraendert, 351);
    quellbestand_ist_unveraendert(&vor_zweitlauf, &bestand, "inkrementellen Zweitlauf");

    // Neben dem unveränderten Quellbaum darf nur die ausdrücklich außerhalb
    // angelegte Vorschauablage entstehen; die Datenbank dieses Falls ist im
    // Speicher. Damit berührt der Lauf keinen weiteren dauerhaften Ort.
    let mut nachbarn: Vec<_> = std::fs::read_dir(tmp.path())
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    nachbarn.sort();
    assert_eq!(
        nachbarn,
        vec![
            std::ffi::OsString::from("bestand"),
            std::ffi::OsString::from("speicher")
        ]
    );
}

#[test]
fn zweiter_lauf_nimmt_nichts_auf_und_weist_nichts_ab() {
    let tmp = bestand_anlegen(30);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    let erst = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(erst.neu, 30);
    assert_eq!(erst.abgewiesen, 0);

    let zweit = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(zweit.neu, 0, "der zweite Lauf nahm erneut auf");
    assert_eq!(zweit.geaendert, 0);
    assert_eq!(
        zweit.abgewiesen, 0,
        "der zweite Lauf meldete Dateien als nicht lesbar"
    );
    assert_eq!(zweit.unveraendert, 30);
}

#[test]
fn ein_unveraenderter_lauf_liest_keine_datei() {
    // Beweisführung ohne Zählwerk: Nach dem ersten Lauf wird der **Inhalt**
    // jeder Datei durch Unsinn ersetzt, Größe und Änderungszeit aber
    // wiederhergestellt. Läse der zweite Lauf die Dateien, scheiterte der
    // Parser an jeder einzelnen und der Befund wiese sie als nicht lesbar aus.
    let tmp = bestand_anlegen(20);
    let (betrieb, horcher) = betrieb_mit(tmp.path());

    let erst = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(erst.neu, 20);

    for e in std::fs::read_dir(tmp.path()).unwrap().flatten() {
        let p = e.path();
        if p.extension().is_none_or(|x| x != "dst") {
            continue;
        }
        let m = std::fs::metadata(&p).unwrap();
        let zeit = m.modified().unwrap();
        std::fs::write(&p, vec![0xFFu8; m.len() as usize]).unwrap();
        std::fs::File::options()
            .write(true)
            .open(&p)
            .unwrap()
            .set_modified(zeit)
            .unwrap();
    }

    let zweit = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(
        zweit.abgewiesen, 0,
        "der Lauf hat die Dateien gelesen — SM-IMP-003 verfehlt"
    );
    assert_eq!(zweit.unveraendert, 20);
}

#[test]
fn entfernte_datei_wird_gemeldet_und_der_eintrag_bleibt() {
    let tmp = bestand_anlegen(10);
    let (betrieb, horcher) = betrieb_mit(tmp.path());
    einlesen_und_warten(&betrieb, &horcher);

    std::fs::remove_file(tmp.path().join("muster0003.dst")).unwrap();

    let befund = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(befund.vermisst, 1, "die Entfernung wurde nicht bemerkt");
    assert_eq!(befund.unveraendert, 9);

    // Der Eintrag steht noch: Der Bestand zählt weiterhin zehn.
    betrieb.beauftragen(Befehl::Ausschnitt {
        abfrage: Abfrage::default(),
        versatz: 0,
        anzahl: 50,
        mit_gesamtzahl: true,
    });
    let gesamt = horcher.warte_auf("Seite", |a| match a {
        Antwort::Seite(s) => Some(s.gesamt),
        _ => None,
    });
    assert_eq!(
        gesamt,
        Some(10),
        "der Eintrag wurde gelöscht statt gekennzeichnet"
    );
}

#[test]
fn geaenderte_datei_verwirft_ihre_vorschau() {
    // Die Änderungserkennung ist zugleich der Auslöser der Vorschau-Verwerfung
    // (AP-08 → AP-09, SM-PRV-003).
    let wurzel = tempfile::tempdir().unwrap();
    let bestand = wurzel.path().join("bestand");
    std::fs::create_dir_all(&bestand).unwrap();
    let quelle = bestand.join("muster.dst");
    let schreibe = |n: usize| {
        let punkte: Vec<(f64, f64)> = (0..n).map(|k| (k as f64, (k % 13) as f64)).collect();
        schreibe_muster(&quelle, punkte);
    };
    schreibe(40);

    let (betrieb, horcher, kacheln) = kacheln_aus_bestand(&bestand);
    betrieb.vorschau_anfordern(Vorschauauftrag {
        zeile: 0,
        uid: kacheln[0].uid.clone(),
        pfad: kacheln[0].pfad.clone(),
        breite_px: 320,
    });
    let vorschau = horcher.warte_auf("VorschauFertig", |a| match a {
        Antwort::VorschauFertig { pfad, .. } => Some(pfad.clone()),
        _ => None,
    });
    assert!(vorschau.is_file());

    schreibe(400);
    let befund = einlesen_und_warten(&betrieb, &horcher);
    assert_eq!(befund.geaendert, 1, "die Änderung wurde nicht erkannt");

    assert!(
        !vorschau.exists(),
        "die überholte Vorschau liegt noch am Ort — die Verwerfung griff nicht"
    );
}
