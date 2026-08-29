//! Prüffälle der Fassade — vor allem die Schnittregeln.

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
use kern_db::NeuerEintrag;

fn eintrag(uid: &str, name: &str) -> NeuerEintrag {
    NeuerEintrag {
        uid: uid.into(),
        pfad: std::path::PathBuf::from(format!("/bestand/{uid}.pes")),
        dateiname: format!("{uid}.pes"),
        format: Format::Pes,
        groesse_bytes: 1024,
        inhalt_hash: None,
        breite_mm: Some(100.0),
        hoehe_mm: Some(80.0),
        stichzahl: Some(9000),
        farbzahl: Some(3),
        name: name.into(),
        datei_geaendert_am: None,
        farben: vec![
            Garnfarbe {
                hex: "C8102E".into(),
                name: Some("Terracotta".into()),
                marke: None,
                markenschluessel: None,
            },
            Garnfarbe {
                hex: "1F4E79".into(),
                name: None,
                marke: None,
                markenschluessel: None,
            },
        ],
        fehlerstatus: None,
        fehlergrund: None,
    }
}

/// Baut einen Bestand über die Datenhaltung auf.
///
/// Die Fassade selbst nimmt nur über `aufnehmen` auf; für Messfälle wäre das
/// unnötig teuer, deshalb hier der kurze Weg über eine eigene Datenhaltung.
fn fassade_mit(n: usize) -> Fassade {
    let mut haltung = kern_db::Datenhaltung::im_speicher().unwrap();
    for i in 0..n {
        haltung
            .eintrag_anlegen(&eintrag(&format!("u{i:05}"), &format!("Muster {i:05}")))
            .unwrap();
    }
    Fassade {
        haltung,
        wurzel: None,
        speicher: None,
    }
}

// --- Schnittregel 4: Zahl der Abfragen hängt nicht an der Zeilenzahl ---

#[test]
fn abfragezahl_ist_von_der_ausschnittgroesse_unabhaengig() {
    let f = fassade_mit(200);
    let a = Abfrage::default();

    let klein = f.ausschnitt(&a, 0, 5, false).unwrap();
    let gross = f.ausschnitt(&a, 0, 100, false).unwrap();

    assert_eq!(klein.kacheln.len(), 5);
    assert_eq!(gross.kacheln.len(), 100);
    assert_eq!(
        klein.abfragen, gross.abfragen,
        "Die Abfragezahl wuchs mit dem Ausschnitt — genau die N+1-Abfrage, \
         die Schnittregel 4 ausschließt ({} gegen {})",
        klein.abfragen, gross.abfragen
    );
    // Genau eine Abfrage: der Ausschnitt. Die Garnfarben kommen über einen
    // eigenen Weg für den ausgewählten Eintrag (SM-DES-007, Newton-7).
    assert_eq!(gross.abfragen, 1);
}

#[test]
fn abfragezahl_ist_von_der_bestandsgroesse_unabhaengig() {
    let a = Abfrage::default();
    let klein = fassade_mit(10).ausschnitt(&a, 0, 10, true).unwrap();
    let gross = fassade_mit(500).ausschnitt(&a, 0, 10, true).unwrap();
    assert_eq!(klein.abfragen, gross.abfragen);
}

#[test]
fn garnfarben_kommen_je_auswahl_nicht_je_ausschnitt() {
    // SM-DES-007: Auf der Kachel stehen nur Bild, Marke, Name und Größe. Die
    // Farben lädt der Detailbereich für den ausgewählten Eintrag.
    let f = fassade_mit(3);
    let seite = f.ausschnitt(&Abfrage::default(), 0, 3, false).unwrap();
    let farben = f.garnfarben(seite.kacheln[0].id).unwrap();
    assert_eq!(farben.len(), 2);
    assert_eq!(farben[0].hex, "C8102E");
    assert_eq!(farben[1].hex, "1F4E79");
}

#[test]
fn detail_laeuft_durch_die_fassade_ohne_pfadpreisgabe() {
    let mut f = fassade_mit(1);
    let seite = f.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    let id = seite.kacheln[0].id;
    let uid = seite.kacheln[0].uid.clone();
    f.schlagworte_setzen(id, &["Herz".into(), "Fest".into()])
        .unwrap();

    let d = f.detail(&uid).unwrap().unwrap();
    assert_eq!(d.uid, uid);
    assert_eq!(d.name, "Muster 00000");
    assert_eq!(d.format, "PES");
    assert_eq!(d.breite_mm, Some(100.0));
    assert_eq!(d.hoehe_mm, Some(80.0));
    assert_eq!(d.stichzahl, Some(9000));
    assert_eq!(d.farbzahl, Some(3));
    assert_eq!(d.schlagworte, vec!["Fest", "Herz"]);
    assert_eq!(d.garnfarben.len(), 2);
    assert_eq!(d.thema, None, "fehlendes Thema wurde erfunden");
    assert_eq!(d.beschreibung, None, "fehlende Beschreibung wurde erfunden");
    assert_eq!(d.notizen, None, "fehlende Notizen wurden erfunden");
    assert_eq!(d.fehlerstatus, None);
    assert_eq!(d.fehlergrund, None);
}

#[test]
fn unbekannte_detailkennung_bleibt_unterscheidbar() {
    let f = fassade_mit(0);
    assert_eq!(f.detail("nicht-da").unwrap(), None);
}

// --- Schnittregel 3: Ausschnitt statt vollständiger Treffermenge ---

#[test]
fn gesamtzahl_nur_beim_ersten_ausschnitt() {
    let f = fassade_mit(30);
    let a = Abfrage::default();

    let erste = f.ausschnitt(&a, 0, 10, true).unwrap();
    assert_eq!(erste.gesamt, Some(30));
    // Der erste Ausschnitt zahlt die Zählabfrage zusätzlich: zwei Abfragen.
    assert_eq!(erste.abfragen, 2);

    let zweite = f.ausschnitt(&a, 10, 10, false).unwrap();
    assert_eq!(zweite.gesamt, None);
    assert_eq!(
        zweite.abfragen, 1,
        "Folgeausschnitt zahlte die Zählabfrage erneut"
    );
}

#[test]
fn treffermenge_wird_nie_vollstaendig_geliefert() {
    let f = fassade_mit(600);
    let seite = f.ausschnitt(&Abfrage::default(), 0, 10_000, false).unwrap();
    assert!(
        seite.kacheln.len() <= 500,
        "es wurden {} Zeilen auf einmal geliefert",
        seite.kacheln.len()
    );
}

#[test]
fn leerer_ausschnitt_ist_kein_fehler() {
    let f = fassade_mit(3);
    let seite = f.ausschnitt(&Abfrage::default(), 100, 10, true).unwrap();
    assert!(seite.kacheln.is_empty());
    assert_eq!(seite.gesamt, Some(3));
}

// --- SM-SEC-003 / Schnittregel 5: Pfade laufen durch kern-security ---

#[test]
fn aufnehmen_ohne_bibliothek_meldet_verstaendlich() {
    let mut f = Fassade::im_speicher().unwrap();
    let e = f.aufnehmen("/irgendwo/a.pes").unwrap_err();
    // Der Text richtet sich an Endnutzer (SM-NFR-006).
    assert!(e.to_string().contains("Bibliothek"), "Text: {e}");
}

#[test]
fn aufnehmen_ausserhalb_der_wurzel_wird_abgewiesen() {
    let tmp = tempfile::tempdir().unwrap();
    let drin = tmp.path().join("bestand");
    std::fs::create_dir(&drin).unwrap();
    let draussen = tmp.path().join("fremd.pes");
    std::fs::write(&draussen, b"x").unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(&drin).unwrap();

    assert!(matches!(
        f.aufnehmen(&draussen),
        Err(Fehler::PfadAusserhalb)
    ));
}

#[test]
fn systemwurzel_wird_als_bibliothek_abgelehnt() {
    let mut f = Fassade::im_speicher().unwrap();
    if std::path::Path::new("/etc").is_dir() {
        assert!(f.wurzel_setzen("/etc").is_err());
    }
    assert!(f.wurzel().is_none());
}

#[test]
fn unbekanntes_format_wird_abgewiesen() {
    let tmp = tempfile::tempdir().unwrap();
    let datei = tmp.path().join("liesmich.txt");
    std::fs::write(&datei, b"kein Stickmuster").unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(tmp.path()).unwrap();
    let e = f.aufnehmen(&datei).unwrap_err();
    assert!(e.to_string().contains("Dateiformat"), "Text: {e}");
}

#[test]
fn beschaedigte_datei_wird_mit_fehlerstatus_erfasst() {
    // SM-IMP-009 verlangt beides: den Lauf nicht abbrechen **und** die Datei
    // mit Fehlerstatus erfassen. Wird sie nur übersprungen, sieht die Nutzerin
    // eine Kachel weniger und erfährt nie, welche Datei fehlt.
    let tmp = tempfile::tempdir().unwrap();
    let datei = tmp.path().join("kaputt.pes");
    std::fs::write(&datei, vec![0xFFu8; 5000]).unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(tmp.path()).unwrap();

    // Der Vorgang kippt nicht …
    f.aufnehmen(&datei).unwrap();
    // … und die Datei ist als Eintrag da.
    assert_eq!(f.bestandsgroesse().unwrap(), 1);

    let seite = f.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    assert_eq!(seite.gesamt, Some(1));
    assert_eq!(seite.kacheln[0].name, "kaputt");

    // Die Fassade ist danach weiter benutzbar.
    assert!(f.ausschnitt(&Abfrage::default(), 0, 10, false).is_ok());
}

// --- Rundlauf über den echten Aufnahmeweg ---

#[test]
fn aufnehmen_und_wiederfinden() {
    let tmp = tempfile::tempdir().unwrap();
    let datei = tmp.path().join("herz.dst");

    // Eine echte, lesbare Datei über den Schreibpfad erzeugen.
    let punkte: Vec<(f64, f64)> = (0..40).map(|i| (i as f64, (i % 7) as f64)).collect();
    kern_parsers::writers::write_dst(
        &[kern_parsers::StitchSegment {
            color_index: 0,
            color_hex: Some("C8102E".into()),
            points: punkte,
        }],
        &kern_security::Wurzel::neu(datei.parent().unwrap())
            .unwrap()
            .schreibziel(&datei)
            .unwrap(),
    )
    .unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(tmp.path()).unwrap();
    f.aufnehmen(&datei).unwrap();

    assert_eq!(f.bestandsgroesse().unwrap(), 1);

    let seite = f.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    assert_eq!(seite.gesamt, Some(1));
    assert_eq!(seite.kacheln[0].format, "DST");
}

#[test]
fn vorschau_entsteht_aus_den_stichdaten() {
    let tmp = tempfile::tempdir().unwrap();
    let datei = tmp.path().join("muster.dst");
    let punkte: Vec<(f64, f64)> = (0..60).map(|i| (i as f64, ((i * 3) % 11) as f64)).collect();
    kern_parsers::writers::write_dst(
        &[kern_parsers::StitchSegment {
            color_index: 0,
            color_hex: Some("1F4E79".into()),
            points: punkte,
        }],
        &kern_security::Wurzel::neu(datei.parent().unwrap())
            .unwrap()
            .schreibziel(&datei)
            .unwrap(),
    )
    .unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(tmp.path()).unwrap();

    let png = f
        .vorschau(&datei, kern_render::Vorschauoption::default())
        .unwrap();
    assert_eq!(&png[1..4], b"PNG");
}

#[test]
fn vorschau_ausserhalb_der_wurzel_wird_abgewiesen() {
    let tmp = tempfile::tempdir().unwrap();
    let drin = tmp.path().join("bestand");
    std::fs::create_dir(&drin).unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(&drin).unwrap();
    assert!(matches!(
        f.vorschau("../fremd.dst", kern_render::Vorschauoption::default()),
        Err(Fehler::PfadAusserhalb)
    ));
}

// --- SM-LIB-010: Kennung hängt nicht am Pfad ---

#[test]
fn kennung_haengt_am_inhalt_nicht_am_pfad() {
    let daten = b"abc123";
    let a = kennung(std::path::Path::new("/erst/herz.pes"), daten);
    let b = kennung(std::path::Path::new("/ganz/woanders/herz.pes"), daten);
    assert_eq!(a, b, "Die Kennung änderte sich beim Verschieben");

    let c = kennung(std::path::Path::new("/erst/anders.pes"), daten);
    assert_ne!(a, c, "Verschiedene Dateien teilten sich eine Kennung");
}

// --- SM-IMP-003: Änderungserkennung ---

/// Schreibt eine echte, lesbare Stickdatei.
fn stickdatei(ziel: &std::path::Path, punkte: usize) {
    let p: Vec<(f64, f64)> = (0..punkte).map(|k| (k as f64, (k % 13) as f64)).collect();
    schreibe_muster(ziel, p);
}

fn bibliothek() -> (tempfile::TempDir, Fassade) {
    let tmp = tempfile::tempdir().unwrap();
    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(tmp.path()).unwrap();
    (tmp, f)
}

fn dateien(wurzel: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut v: Vec<_> = std::fs::read_dir(wurzel)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "dst"))
        .collect();
    v.sort();
    v
}

#[test]
fn erster_lauf_meldet_alles_als_neu() {
    let (tmp, f) = bibliothek();
    for i in 0..5 {
        stickdatei(&tmp.path().join(format!("m{i}.dst")), 40);
    }
    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(satz.neu.len(), 5);
    assert_eq!(satz.unveraendert, 0);
    assert!(satz.geaendert.is_empty());
    assert!(satz.vermisst.is_empty());
}

#[test]
fn zweiter_lauf_findet_nichts_zu_tun() {
    // Der Kern von SM-IMP-003: Ein Lauf über einen unveränderten Bestand hat
    // nichts zu lesen.
    let (tmp, mut f) = bibliothek();
    for i in 0..5 {
        let p = tmp.path().join(format!("m{i}.dst"));
        stickdatei(&p, 40);
        f.aufnehmen(&p).unwrap();
    }

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(satz.unveraendert, 5);
    assert_eq!(satz.zu_lesen(), 0, "es sollte nichts zu lesen geben");
    assert!(satz.leer(), "der Änderungssatz sollte leer sein");
}

#[test]
fn neue_datei_wird_erkannt_ohne_die_uebrigen_anzufassen() {
    let (tmp, mut f) = bibliothek();
    for i in 0..5 {
        let p = tmp.path().join(format!("m{i}.dst"));
        stickdatei(&p, 40);
        f.aufnehmen(&p).unwrap();
    }
    stickdatei(&tmp.path().join("neu.dst"), 60);

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(satz.neu.len(), 1);
    assert_eq!(satz.unveraendert, 5);
    assert_eq!(
        satz.zu_lesen(),
        1,
        "es sollte genau eine Datei zu lesen sein"
    );
}

#[test]
fn geaenderte_datei_behaelt_kennung_und_schlagworte() {
    // SM-LIB-010: Die Kennung übersteht die Änderung. Daran hängen die
    // Schlagworte, die die Nutzerin gepflegt hat.
    let (tmp, mut f) = bibliothek();
    let p = tmp.path().join("m.dst");
    stickdatei(&p, 40);
    let id = f.aufnehmen(&p).unwrap();
    f.schlagworte_setzen(id, &["Volksfest".into(), "Herz".into()])
        .unwrap();

    let vorher = f.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    let uid_vorher = vorher.kacheln[0].uid.clone();

    // Datei ändern — andere Länge, damit die Erkennung greift.
    stickdatei(&p, 400);

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(satz.geaendert.len(), 1, "die Änderung wurde nicht erkannt");
    assert!(satz.neu.is_empty(), "die Datei galt fälschlich als neu");

    let (datei, eintrag_id, _uid) = satz.geaendert[0].clone();
    assert_eq!(eintrag_id, id);
    f.erneuern(eintrag_id, &datei).unwrap();

    // Der Bestand hat weiterhin genau einen Eintrag …
    assert_eq!(f.bestandsgroesse().unwrap(), 1);
    // … mit derselben Kennung …
    let nachher = f.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    assert_eq!(nachher.kacheln[0].uid, uid_vorher, "die Kennung wechselte");
    // … und die Schlagworte stehen noch.
    let treffer = f
        .ausschnitt(
            &Abfrage {
                schlagworte: vec!["Volksfest".into()],
                ..Default::default()
            },
            0,
            10,
            true,
        )
        .unwrap();
    assert_eq!(treffer.gesamt, Some(1), "die Schlagworte gingen verloren");
}

#[test]
fn entfernte_datei_wird_gekennzeichnet_nicht_geloescht() {
    // SM-DAT-003 verlangt eine Bestätigung vor dem Löschen von Einträgen. Ein
    // Importlauf darf deshalb nicht stillschweigend löschen.
    let (tmp, mut f) = bibliothek();
    let p = tmp.path().join("m.dst");
    stickdatei(&p, 40);
    let id = f.aufnehmen(&p).unwrap();
    f.schlagworte_setzen(id, &["Wichtig".into()]).unwrap();

    std::fs::remove_file(&p).unwrap();

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(satz.vermisst.len(), 1);
    assert_eq!(f.vermisst_kennzeichnen(&[satz.vermisst[0].0]).unwrap(), 1);

    // Der Eintrag steht noch — samt gepflegter Metadaten.
    assert_eq!(
        f.bestandsgroesse().unwrap(),
        1,
        "der Eintrag wurde gelöscht"
    );
    let treffer = f
        .ausschnitt(
            &Abfrage {
                schlagworte: vec!["Wichtig".into()],
                ..Default::default()
            },
            0,
            10,
            true,
        )
        .unwrap();
    assert_eq!(treffer.gesamt, Some(1), "die Metadaten gingen verloren");
}

#[test]
fn eine_vermisste_datei_wird_nicht_zweimal_gemeldet() {
    let (tmp, mut f) = bibliothek();
    let p = tmp.path().join("m.dst");
    stickdatei(&p, 40);
    f.aufnehmen(&p).unwrap();
    std::fs::remove_file(&p).unwrap();

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    f.vermisst_kennzeichnen(&[satz.vermisst[0].0]).unwrap();

    // Im nächsten Lauf ist sie bereits gekennzeichnet und kein Vorgang mehr.
    let zweiter = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert!(zweiter.vermisst.is_empty(), "erneut als vermisst gemeldet");
}

#[test]
fn wieder_aufgetauchte_datei_verliert_die_marke() {
    let (tmp, mut f) = bibliothek();
    let p = tmp.path().join("m.dst");
    stickdatei(&p, 40);
    f.aufnehmen(&p).unwrap();

    let inhalt = std::fs::read(&p).unwrap();
    let zeit = std::fs::metadata(&p).unwrap().modified().unwrap();
    std::fs::remove_file(&p).unwrap();

    let satz = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    f.vermisst_kennzeichnen(&[satz.vermisst[0].0]).unwrap();

    // Dieselbe Datei kommt zurück — gleiche Länge, gleiche Änderungszeit.
    std::fs::write(&p, &inhalt).unwrap();
    std::fs::File::options()
        .write(true)
        .open(&p)
        .unwrap()
        .set_modified(zeit)
        .unwrap();

    let zweiter = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert_eq!(
        zweiter.wiedergefunden.len(),
        1,
        "das Wiederauftauchen wurde nicht bemerkt"
    );
    assert_eq!(f.vermisst_aufheben(&zweiter.wiedergefunden).unwrap(), 1);

    let dritter = f.aenderungen_ermitteln(&dateien(tmp.path())).unwrap();
    assert!(dritter.wiedergefunden.is_empty());
    assert!(dritter.vermisst.is_empty());
}

#[test]
fn der_inhaltshash_entsteht_beim_aufnehmen() {
    // Grundlage von SM-IMP-005. Er kostet keine zusätzliche Ein-/Ausgabe: Die
    // Datei liegt für den Parser ohnehin im Speicher.
    let a = inhaltshash(b"abc");
    let b = inhaltshash(b"abd");
    assert_ne!(a, b);
    assert_eq!(a.len(), 64, "kein SHA-256 in Hexschreibung");
    assert_eq!(inhaltshash(b"abc"), a, "der Hash ist nicht reproduzierbar");
}

// --- Tesla-1: Fremddateien werden begrenzt und typgeprüft gelesen ---

#[test]
fn uebergrosse_datei_wird_abgewiesen_statt_gelesen() {
    // SM-FMT-012: keine unbegrenzte Speicherbelegung. `fs::read` reserviert
    // anhand der Metadatengröße; eine sehr große Datei beendete den Prozess
    // über einen Allokationsfehler, den kein `Result` abfängt.
    let tmp = tempfile::tempdir().unwrap();
    let datei = tmp.path().join("riesig.dst");
    let f = std::fs::File::create(&datei).unwrap();
    // Dünn besetzt: kostet keinen Plattenplatz, meldet aber die volle Größe.
    f.set_len(super::HOECHSTGROESSE + 1).unwrap();
    drop(f);

    let mut fa = Fassade::im_speicher().unwrap();
    fa.wurzel_setzen(tmp.path()).unwrap();

    // Nicht gelesen — aber erfasst (SM-IMP-009).
    fa.aufnehmen(&datei).unwrap();
    let seite = fa.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    assert_eq!(seite.gesamt, Some(1));
    let grund = seite.kacheln[0].fehlergrund.as_deref().unwrap_or("");
    assert!(
        grund.contains("größer als"),
        "die übergroße Datei trägt keinen verständlichen Grund: {grund}"
    );
}

#[cfg(unix)]
#[test]
fn benannte_roehre_wird_nicht_geoeffnet() {
    // Eine FIFO in der Bibliothekswurzel ließe `fs::read` beim Öffnen
    // unbegrenzt blockieren — der Importlauf hinge ohne Fehlermeldung.
    let tmp = tempfile::tempdir().unwrap();
    let roehre = tmp.path().join("haenger.dst");
    // Über das Systemwerkzeug, nicht über einen unsicheren Block: Die Kiste
    // führt `#![forbid(unsafe_code)]`, und das gilt auch für Prüffälle.
    let erzeugt = std::process::Command::new("mkfifo")
        .arg(&roehre)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    // Kein stilles `return`: Ein nicht durchgeführter Fall ist kein
    // bestandener Fall (CLAUDE.md Abschnitt 13). Fehlt das Werkzeug, fällt der
    // Fall durch und benennt den Grund.
    assert!(
        erzeugt,
        "mkfifo ist nicht verfügbar — dieser Fall konnte nicht geprüft werden"
    );

    let mut fa = Fassade::im_speicher().unwrap();
    fa.wurzel_setzen(tmp.path()).unwrap();

    fa.aufnehmen(&roehre).unwrap();
    let seite = fa.ausschnitt(&Abfrage::default(), 0, 10, true).unwrap();
    let grund = seite.kacheln[0].fehlergrund.as_deref().unwrap_or("");
    assert!(
        grund.contains("gewöhnliche Datei"),
        "die Röhre wurde nicht als solche erkannt: {grund}"
    );
}

// --- Newton-1: der Anzeigepfad nutzt den Zwischenspeicher ---

#[test]
fn zweite_vorschau_kommt_aus_dem_zwischenspeicher() {
    // Beweis ohne Zeitmessung: Nach dem ersten Lauf wird die abgelegte Datei
    // durch eine erkennbare Marke ersetzt. Kommt sie zurück, stammt die
    // Antwort aus dem Speicher und nicht aus einem zweiten Zeichenlauf.
    let tmp = tempfile::tempdir().unwrap();
    let bestand = tmp.path().join("bestand");
    std::fs::create_dir(&bestand).unwrap();
    let quelle = bestand.join("muster.dst");
    let punkte: Vec<(f64, f64)> = (0..40).map(|k| (k as f64, (k % 11) as f64)).collect();
    kern_parsers::writers::write_dst(
        &[kern_parsers::StitchSegment {
            color_index: 0,
            color_hex: Some("C8102E".into()),
            points: punkte,
        }],
        &kern_security::Wurzel::neu(quelle.parent().unwrap())
            .unwrap()
            .schreibziel(&quelle)
            .unwrap(),
    )
    .unwrap();

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(&bestand).unwrap();
    f.speicher_setzen(kern_render::Zwischenspeicher::an(tmp.path().join("speicher")).unwrap());

    let erst = f
        .vorschau_gepuffert("u1", &quelle, kern_render::Stufe::Mittel)
        .unwrap();
    assert_eq!(&std::fs::read(&erst).unwrap()[1..4], b"PNG");

    std::fs::write(&erst, b"MARKE-AUS-DEM-SPEICHER").unwrap();

    let zweit = f
        .vorschau_gepuffert("u1", &quelle, kern_render::Stufe::Mittel)
        .unwrap();
    assert_eq!(erst, zweit);
    assert_eq!(
        std::fs::read(&zweit).unwrap(),
        b"MARKE-AUS-DEM-SPEICHER",
        "die Vorschau wurde neu gezeichnet statt aus dem Speicher genommen"
    );
}

#[test]
fn geaenderte_quelle_erzwingt_eine_neue_vorschau() {
    let tmp = tempfile::tempdir().unwrap();
    let bestand = tmp.path().join("bestand");
    std::fs::create_dir(&bestand).unwrap();
    let quelle = bestand.join("muster.dst");
    let schreibe = |n: usize| {
        let punkte: Vec<(f64, f64)> = (0..n).map(|k| (k as f64, (k % 13) as f64)).collect();
        kern_parsers::writers::write_dst(
            &[kern_parsers::StitchSegment {
                color_index: 0,
                color_hex: Some("C8102E".into()),
                points: punkte,
            }],
            &kern_security::Wurzel::neu(quelle.parent().unwrap())
                .unwrap()
                .schreibziel(&quelle)
                .unwrap(),
        )
        .unwrap();
    };
    schreibe(40);

    let mut f = Fassade::im_speicher().unwrap();
    f.wurzel_setzen(&bestand).unwrap();
    f.speicher_setzen(kern_render::Zwischenspeicher::an(tmp.path().join("speicher")).unwrap());

    let erst = f
        .vorschau_gepuffert("u1", &quelle, kern_render::Stufe::Mittel)
        .unwrap();
    std::fs::write(&erst, b"MARKE").unwrap();

    schreibe(400);
    let zweit = f
        .vorschau_gepuffert("u1", &quelle, kern_render::Stufe::Mittel)
        .unwrap();
    let inhalt = std::fs::read(&zweit).unwrap();
    assert_ne!(
        inhalt, b"MARKE",
        "die überholte Vorschau kam zurück — SM-PRV-003 verletzt"
    );
    assert_eq!(&inhalt[1..4], b"PNG");
}

// --- T-4: der billige Weg darf die Eingrenzung nicht aushebeln ---

#[test]
fn schnellweg_greift_nicht_bei_aufstieg_im_pfad() {
    let wurzel = std::path::Path::new("/bestand");
    // Beginnt lexikalisch mit der Wurzel, führt aber hinaus.
    assert!(!super::schnellweg_traegt(
        std::path::Path::new("/bestand/../fremd/x.dst"),
        wurzel
    ));
    assert!(!super::schnellweg_traegt(
        std::path::Path::new("/bestand-daneben/x.dst"),
        wurzel
    ));
}

#[cfg(unix)]
#[test]
fn schnellweg_greift_nicht_bei_symlink_am_ende() {
    let tmp = tempfile::tempdir().unwrap();
    let drin = tmp.path().join("drin");
    std::fs::create_dir(&drin).unwrap();
    std::fs::write(drin.join("echt.dst"), b"x").unwrap();
    std::os::unix::fs::symlink("/etc/passwd", drin.join("verweis.dst")).unwrap();

    assert!(super::schnellweg_traegt(&drin.join("echt.dst"), &drin));
    assert!(
        !super::schnellweg_traegt(&drin.join("verweis.dst"), &drin),
        "ein Symlink am Ende umging den Schnellweg"
    );
}
