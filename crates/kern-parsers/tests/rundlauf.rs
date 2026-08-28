//! Rundlaufprüfungen Schreibpfad → Lesepfad.
//!
//! Sie ersetzen den externen Prüfbestand für die Fälle, die ohne ihn nicht
//! laufen können: Der Schreibpfad erzeugt die Datei, der Lesepfad liest sie
//! zurück. Das prüft beide Seiten gegeneinander und braucht keine im
//! Repository abgelegte Binärdatei (CLAUDE.md Abschnitt 13, Stufe 0.4).

use kern_parsers::{lies_kennwerte, lies_stichabschnitte, writers, StitchSegment};
use kern_typen::Format;

/// Ein kleines, aber nicht triviales Muster: zwei Farblagen, geschlossene Wege.
fn muster() -> Vec<StitchSegment> {
    let quadrat = |x0: f64, y0: f64, kante: f64, schritt: f64| {
        let mut punkte = Vec::new();
        let mut t = 0.0;
        while t < kante {
            punkte.push((x0 + t, y0));
            t += schritt;
        }
        t = 0.0;
        while t < kante {
            punkte.push((x0 + kante, y0 + t));
            t += schritt;
        }
        t = 0.0;
        while t < kante {
            punkte.push((x0 + kante - t, y0 + kante));
            t += schritt;
        }
        t = 0.0;
        while t < kante {
            punkte.push((x0, y0 + kante - t));
            t += schritt;
        }
        punkte
    };

    vec![
        StitchSegment {
            color_index: 0,
            color_hex: Some("C8102E".into()),
            points: quadrat(0.0, 0.0, 20.0, 1.0),
        },
        StitchSegment {
            color_index: 1,
            color_hex: Some("1F4E79".into()),
            points: quadrat(5.0, 5.0, 10.0, 1.0),
        },
    ]
}

fn schreibe_und_lies(zielformat: &str, format: Format) -> (i64, f64, f64) {
    let tmp = tempfile::tempdir().unwrap();
    let wurzel = kern_security::Wurzel::neu(tmp.path()).unwrap();
    let datei = wurzel
        .schreibziel(format!("muster.{}", zielformat.to_lowercase()))
        .unwrap();

    writers::convert_segments(&muster(), zielformat, &datei)
        .unwrap_or_else(|e| panic!("{zielformat}: Schreiben fehlgeschlagen: {e}"));

    let daten = std::fs::read(datei.pfad()).unwrap();
    assert!(!daten.is_empty(), "{zielformat}: leere Datei geschrieben");

    let kennwerte = lies_kennwerte(format, &daten)
        .unwrap_or_else(|e| panic!("{zielformat}: Zurücklesen fehlgeschlagen: {e}"));

    (
        kennwerte.stichzahl.unwrap_or(0),
        kennwerte.breite_mm.unwrap_or(0.0),
        kennwerte.hoehe_mm.unwrap_or(0.0),
    )
}

#[test]
fn dst_rundlauf_haelt_stichzahl_und_masse() {
    let (stiche, breite, hoehe) = schreibe_und_lies("DST", Format::Dst);

    let erwartet = muster().iter().map(|s| s.points.len()).sum::<usize>() as i64;
    // Der Schreibpfad setzt je Farbwechsel einen zusätzlichen Stich.
    assert!(
        (stiche - erwartet).abs() <= 2,
        "DST: Stichzahl {stiche} weicht von {erwartet} ab"
    );

    // Das äußere Quadrat ist 20 mm breit und hoch.
    assert!(
        (breite - 20.0).abs() < 1.5,
        "DST: Breite {breite} mm statt rund 20 mm"
    );
    assert!(
        (hoehe - 20.0).abs() < 1.5,
        "DST: Höhe {hoehe} mm statt rund 20 mm"
    );
}

#[test]
fn pes_rundlauf_liefert_lesbare_datei() {
    let (stiche, _b, _h) = schreibe_und_lies("PES", Format::Pes);
    assert!(stiche > 0, "PES: keine Stiche zurückgelesen");
}

#[test]
fn dst_rundlauf_haelt_die_farblagen() {
    let tmp = tempfile::tempdir().unwrap();
    let wurzel = kern_security::Wurzel::neu(tmp.path()).unwrap();
    let datei = wurzel.schreibziel("muster.dst").unwrap();
    writers::write_dst(&muster(), &datei).unwrap();
    let daten = std::fs::read(datei.pfad()).unwrap();

    let abschnitte = lies_stichabschnitte(Format::Dst, &daten).unwrap();
    assert!(
        abschnitte.len() >= 2,
        "DST: {} Farblagen statt mindestens zwei",
        abschnitte.len()
    );
    assert!(abschnitte.iter().all(|a| !a.punkte.is_empty()));
}

#[test]
fn leeres_muster_wird_abgewiesen_statt_geschrieben() {
    let tmp = tempfile::tempdir().unwrap();
    let wurzel = kern_security::Wurzel::neu(tmp.path()).unwrap();
    let datei = wurzel.schreibziel("leer.dst").unwrap();
    assert!(writers::write_dst(&[], &datei).is_err());
    assert!(
        !datei.pfad().exists(),
        "trotz Fehler wurde eine Datei angelegt"
    );
}

#[test]
fn unbekanntes_zielformat_wird_abgewiesen() {
    let tmp = tempfile::tempdir().unwrap();
    let wurzel = kern_security::Wurzel::neu(tmp.path()).unwrap();
    let datei = wurzel.schreibziel("x.abc").unwrap();
    assert!(writers::convert_segments(&muster(), "ABC", &datei).is_err());
}
