//! Messaufbau zu SM-SRC-007 und SM-LIB-009.
//!
//! Der Fall ist bewusst ausgesetzt (`#[ignore]`) und wird gezielt gestartet:
//!
//! ```text
//! cargo test -p kern-db --release --test messung -- --ignored --nocapture
//! ```
//!
//! **Was er belegt und was nicht.** Er belegt, dass Index und Abfrageform bei
//! 100.000 Einträgen tragen. Er ist **keine Abnahme**: Solange OP-08 kein
//! Referenzgerät benennt, ist der Wert eine Regressionsschwelle, keine
//! Abnahmezusage (IMP-STM-001 Abschnitt 2.2). Gerät, Datenbestand und
//! Bedingungen gehören zu jeder Messung ins Protokoll.

use kern_db::{Datenhaltung, NeuerEintrag, Suchabfrage};
use kern_typen::{Format, Garnfarbe};

const BESTAND: usize = 100_000;

fn bestand_erzeugen(n: usize) -> Vec<NeuerEintrag> {
    let formate = [Format::Pes, Format::Dst, Format::Jef, Format::Vp3];
    let woerter = [
        "Herz", "Rose", "Anker", "Stern", "Blume", "Kranz", "Ranke", "Falter", "Hirsch", "Feder",
        "Welle", "Zweig", "Distel", "Moewe", "Segel",
    ];
    (0..n)
        .map(|i| {
            let a = woerter[i % woerter.len()];
            let b = woerter[(i / woerter.len()) % woerter.len()];
            NeuerEintrag {
                uid: format!("u{i:07}"),
                pfad: std::path::PathBuf::from(format!("/bestand/{:03}/{i:07}.pes", i % 500)),
                dateiname: format!("{i:07}.pes"),
                format: formate[i % formate.len()],
                groesse_bytes: 2048 + (i as i64 % 90_000),
                inhalt_hash: None,
                breite_mm: Some(40.0 + (i % 260) as f64),
                hoehe_mm: Some(30.0 + (i % 190) as f64),
                stichzahl: Some(800 + (i as i64 % 90_000)),
                farbzahl: Some(1 + (i as i64 % 12)),
                // „Muster" steht in jedem Namen. Ein Wort, das nahezu den
                // Gesamtbestand trifft, ist der ungünstigste Volltextfall und
                // in gewachsenen Bibliotheken der Regelfall (ein Ladenname, ein
                // Anlass, eine Sammlungsbezeichnung im Dateinamen).
                name: format!("Muster {a} {b} {i:07}"),
                datei_geaendert_am: Some(1_600_000_000 + i as i64),
                farben: vec![Garnfarbe {
                    hex: "C8102E".into(),
                    name: Some("Terracotta".into()),
                    marke: None,
                    markenschluessel: None,
                }],
                fehlerstatus: None,
                fehlergrund: None,
            }
        })
        .collect()
}

#[test]
#[ignore = "Messfall — gezielt mit --ignored starten"]
fn suche_unter_einer_sekunde_bei_hunderttausend() {
    let tmp = tempfile::tempdir().unwrap();
    let mut h = Datenhaltung::oeffnen(tmp.path().join("mess.db")).unwrap();

    let anfang = std::time::Instant::now();
    let eintraege = bestand_erzeugen(BESTAND);
    for block in eintraege.chunks(10_000) {
        let _ = h.eintraege_anlegen(block).unwrap();
    }
    let aufbau = anfang.elapsed();
    println!("Aufbau von {BESTAND} Eintraegen: {aufbau:?}");
    assert_eq!(h.bestandsgroesse().unwrap(), BESTAND as i64);

    // Der Index wird warm gelesen — SM-SRC-007 spricht vom warmen Index.
    for _ in 0..3 {
        let _ = h.suche(
            &Suchabfrage {
                text: Some("Herz".into()),
                ..Default::default()
            },
            0,
            60,
            true,
        );
    }

    let faelle: Vec<(&str, Suchabfrage)> = vec![
        (
            "Volltext, ein Wort",
            Suchabfrage {
                text: Some("Anker".into()),
                ..Default::default()
            },
        ),
        (
            "Volltext, zwei Woerter",
            Suchabfrage {
                text: Some("Rose Stern".into()),
                ..Default::default()
            },
        ),
        (
            "Volltext ohne Treffer",
            Suchabfrage {
                text: Some("Zeppelin".into()),
                ..Default::default()
            },
        ),
        (
            "Formatfilter",
            Suchabfrage {
                formate: vec![Format::Dst],
                ..Default::default()
            },
        ),
        (
            "Volltext und Filter",
            Suchabfrage {
                text: Some("Herz".into()),
                formate: vec![Format::Pes],
                stichzahl_von: Some(5_000),
                ..Default::default()
            },
        ),
        (
            "Volltext, fast Gesamtbestand",
            Suchabfrage {
                text: Some("Muster".into()),
                ..Default::default()
            },
        ),
        (
            "Volltext breit, sortiert",
            Suchabfrage {
                text: Some("Muster".into()),
                sortierung: kern_db::Sortierung::Groesse,
                ..Default::default()
            },
        ),
        ("Ohne Einschraenkung", Suchabfrage::default()),
    ];

    let mut schlechteste = std::time::Duration::ZERO;

    for (name, abfrage) in &faelle {
        // Dreimal messen: Der erste Wert enthaelt das Warmlesen der beteiligten
        // Indexseiten, SM-SRC-007 spricht ausdruecklich vom *warmen* Index.
        let mut werte = Vec::new();
        let mut aus = h.suche(abfrage, 0, 60, true).unwrap();
        for _ in 0..3 {
            let t = std::time::Instant::now();
            aus = h.suche(abfrage, 0, 60, true).unwrap();
            werte.push(t.elapsed());
        }
        let dauer = *werte.iter().min().unwrap();
        let erst = werte[0];
        schlechteste = schlechteste.max(dauer);
        println!(
            "  {name:<24} erster Ausschnitt {dauer:>12.2?} (kalt {erst:>10.2?})  Treffer {:>7}  Zeilen {}",
            aus.gesamt.unwrap_or(-1),
            aus.zeilen.len()
        );

        // Der tiefste Versatz wird je Fall aus seiner Trefferzahl bestimmt.
        // Ein fester Wert (vormals 5.000) misst am eigentlichen Fall vorbei:
        // Für einen Volltextbegriff mit wenigen Treffern liegt er hinter dem
        // Ende und liefert eine leere, trivial schnelle Seite — der teure Fall
        // ist die *letzte gefuellte* Seite, weil der Tafelausdruck dafuer den
        // gesamten Treffersatz materialisiert und sortiert.
        for versatz in [5_000i64, aus.gesamt.unwrap_or(0) - 60] {
            if versatz <= 0 {
                continue;
            }
            let t = std::time::Instant::now();
            let tief = h.suche(abfrage, versatz, 60, false).unwrap();
            let dauer_tief = t.elapsed();
            schlechteste = schlechteste.max(dauer_tief);
            println!(
                "  {:<24} Versatz {versatz:>6}    {dauer_tief:>12.2?}  Zeilen {}",
                "",
                tief.zeilen.len()
            );
        }
    }

    // Jeder Sortierschluessel bekommt seinen Index erst mit Schritt 5; ohne
    // Messung je Schluessel truege der Nachweis nur einen von sechs.
    println!("  --- je Sortierschluessel, tiefster Versatz ---");
    for (name, sortierung) in [
        ("Name", kern_db::Sortierung::Name),
        ("Groesse", kern_db::Sortierung::Groesse),
        ("Dateidatum", kern_db::Sortierung::DateiDatum),
        ("Stichzahl", kern_db::Sortierung::Stichzahl),
        ("Importdatum", kern_db::Sortierung::ImportDatum),
    ] {
        // Je Schluessel zweimal: ohne Volltext (reiner Indexweg) und mit einem
        // Volltextbegriff, der nahezu alles trifft (Tafelausdruck ueber den
        // Gesamtbestand). Nur die zweite Form belegt SM-SRC-007 fuer die
        // Verbindung aus Volltext und tiefstem Versatz.
        for (art, text) in [("ohne Volltext", None), ("mit Volltext", Some("Muster"))] {
            let a = Suchabfrage {
                sortierung,
                text: text.map(str::to_string),
                ..Default::default()
            };
            let t = std::time::Instant::now();
            let aus = h.suche(&a, (BESTAND as i64) - 60, 60, false).unwrap();
            let dauer = t.elapsed();
            schlechteste = schlechteste.max(dauer);
            println!(
                "  {name:<12} {art:<14} {dauer:>12.2?}  Zeilen {}",
                aus.zeilen.len()
            );
        }
    }

    println!("Schlechtester Einzelwert: {schlechteste:?}");
    assert!(
        schlechteste < std::time::Duration::from_secs(1),
        "SM-SRC-007 verfehlt: schlechteste Suche {schlechteste:?} ueber einer Sekunde"
    );
}

#[test]
#[ignore = "Messfall — gezielt mit --ignored starten"]
fn abfragezahl_bleibt_bei_hunderttausend_konstant() {
    let tmp = tempfile::tempdir().unwrap();
    let mut h = Datenhaltung::oeffnen(tmp.path().join("mess2.db")).unwrap();
    for block in bestand_erzeugen(BESTAND).chunks(10_000) {
        let _ = h.eintraege_anlegen(block).unwrap();
    }

    let a = Suchabfrage::default();
    for anzahl in [10, 60, 200, 500] {
        h.abfragen_zuruecksetzen();
        let aus = h.suche(&a, 0, anzahl, false).unwrap();
        let farben = h
            .garnfarben_mehrerer(&aus.zeilen.iter().map(|z| z.id).collect::<Vec<_>>())
            .unwrap();
        println!(
            "  Ausschnitt {anzahl:>3}: {} Abfragen, {} Zeilen, {} Farbsaetze",
            h.abfragen_gesamt(),
            aus.zeilen.len(),
            farben.len()
        );
        assert_eq!(
            h.abfragen_gesamt(),
            2,
            "Die Abfragezahl haengt an der Zeilenzahl — Schnittregel 4 verletzt"
        );
    }
}
