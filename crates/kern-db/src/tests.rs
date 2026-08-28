//! Prüffälle der Datenhaltung.

use super::*;

fn eintrag(uid: &str, name: &str, format: Format) -> NeuerEintrag {
    NeuerEintrag {
        uid: uid.into(),
        pfad: PathBuf::from(format!("/bestand/{uid}.{}", format.marke().to_lowercase())),
        dateiname: format!("{uid}.{}", format.marke().to_lowercase()),
        format,
        groesse_bytes: 4096,
        inhalt_hash: Some(format!("hash-{uid}")),
        breite_mm: Some(100.0),
        hoehe_mm: Some(80.0),
        stichzahl: Some(12_000),
        farbzahl: Some(4),
        name: name.into(),
        datei_geaendert_am: Some(1_700_000_000),
        farben: vec![Garnfarbe {
            hex: "C8102E".into(),
            name: Some("Terracotta".into()),
            marke: Some("Madeira".into()),
            markenschluessel: Some("1147".into()),
        }],
        fehlerstatus: None,
        fehlergrund: None,
    }
}

// --- SM-DAT-007: Additive, unveränderliche Schritte ---

/// Diese Prüfung ist die maschinelle Fassung der Zusage aus SM-DAT-007.
///
/// Ändert jemand einen **bestehenden** Schritt, ändert sich dessen Prüfsumme
/// und dieser Fall schlägt fehl. Ein **neuer** Schritt am Ende lässt ihn grün —
/// genau das ist der zulässige Weg. Wird ein Schritt bewusst neu ausgeliefert,
/// ist die Zeile hier mit Begründung im Commit zu ändern.
#[test]
fn bestehende_schritte_sind_unveraendert() {
    fn pruefsumme(s: &str) -> u64 {
        // FNV-1a — reicht, um eine Veränderung zu bemerken.
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in s.as_bytes() {
            h ^= *b as u64;
            h = h.wrapping_mul(0x100_0000_01b3);
        }
        h
    }

    let erwartet: &[(i64, &str, u64)] = &[
        (1, "grundgeruest", 0x8511314a30ca4226),
        (2, "volltextindex", 0x1b9b86c1a0ec4b9c),
        (3, "ordner_und_sammlungen", 0x4307e46b028dbfeb),
        (4, "vermisste_eintraege", 0x2a12a950e7cedd12),
        (5, "suchindizes_und_gezielter_auslöser", 0xb2b55825d6b318ff),
        (6, "fehlerstatus", 0x4370b3cef1b652eb),
    ];

    // Die Nummern sind aufsteigend und lückenlos.
    for (i, s) in migrationen::SCHRITTE.iter().enumerate() {
        assert_eq!(s.nummer, i as i64 + 1, "Schrittnummern haben eine Lücke");
    }

    assert_eq!(
        migrationen::SCHRITTE.len(),
        erwartet.len(),
        "Es kam ein Schritt hinzu — die Erwartungsliste ist mitzuführen"
    );

    for (s, (nr, name, summe)) in migrationen::SCHRITTE.iter().zip(erwartet) {
        assert_eq!(s.nummer, *nr);
        assert_eq!(s.name, *name);
        assert_eq!(
            pruefsumme(s.sql),
            *summe,
            "Schritt {nr} ({name}) wurde nachträglich verändert — SM-DAT-007 verbietet das"
        );
    }
}

#[test]
fn migration_laeuft_und_ist_wiederholbar() {
    let tmp = tempfile::tempdir().unwrap();
    let pfad = tmp.path().join("bestand.db");

    // Der erwartete Stand ist der letzte Schritt — nicht eine Zahl, die bei
    // jedem neuen Schritt nachzuziehen wäre.
    let ziel = migrationen::SCHRITTE.last().unwrap().nummer;

    let h = Datenhaltung::oeffnen(&pfad).unwrap();
    assert_eq!(h.schema_stand().unwrap(), ziel);
    drop(h);

    // Zweites Öffnen wendet nichts erneut an und scheitert nicht.
    let h = Datenhaltung::oeffnen(&pfad).unwrap();
    assert_eq!(h.schema_stand().unwrap(), ziel);
}

#[test]
fn wal_ist_eingeschaltet() {
    // SM-DAT-006: Ohne WAL kein verlustfreier Wiederanlauf.
    let tmp = tempfile::tempdir().unwrap();
    let h = Datenhaltung::oeffnen(tmp.path().join("b.db")).unwrap();
    let modus: String = h
        .conn
        .query_row("PRAGMA journal_mode", [], |z| z.get(0))
        .unwrap();
    assert_eq!(modus.to_lowercase(), "wal");
}

// --- SM-LIB-010: dauerhafte Kennung ---

#[test]
fn kennung_uebersteht_verschieben() {
    let mut h = Datenhaltung::im_speicher().unwrap();
    let id = h
        .eintrag_anlegen(&eintrag("u1", "Herz", Format::Pes))
        .unwrap();

    h.pfad_fortschreiben("u1", Path::new("/bestand/neu/herz.pes"), "herz.pes")
        .unwrap();

    assert_eq!(h.eintrag_nach_uid("u1").unwrap(), Some(id));
}

#[test]
fn unbekannte_kennung_meldet_klar() {
    let h = Datenhaltung::im_speicher().unwrap();
    assert!(matches!(
        h.pfad_fortschreiben("gibtsnicht", Path::new("/a/b.pes"), "b.pes"),
        Err(Fehler::NichtGefunden(_))
    ));
}

// --- SM-SRC-001 ff.: Suche ---

fn bestand() -> Datenhaltung {
    let mut h = Datenhaltung::im_speicher().unwrap();
    let id1 = h
        .eintrag_anlegen(&eintrag("u1", "Bayrisches Herz", Format::Pes))
        .unwrap();
    let id2 = h
        .eintrag_anlegen(&eintrag("u2", "Rosenranke", Format::Dst))
        .unwrap();
    h.eintrag_anlegen(&eintrag("u3", "Anker maritim", Format::Jef))
        .unwrap();
    h.schlagworte_setzen(id1, &["Volksfest".into(), "Herz".into()])
        .unwrap();
    h.schlagworte_setzen(id2, &["Blume".into()]).unwrap();
    h
}

#[test]
fn volltext_findet_ueber_den_namen() {
    let h = bestand();
    let a = Suchabfrage {
        text: Some("Rosenranke".into()),
        ..Default::default()
    };
    let aus = h.suche(&a, 0, 50, true).unwrap();
    assert_eq!(aus.gesamt, Some(1));
    assert_eq!(aus.zeilen[0].name, "Rosenranke");
}

#[test]
fn volltext_findet_ueber_schlagworte() {
    let h = bestand();
    let a = Suchabfrage {
        text: Some("Volksfest".into()),
        ..Default::default()
    };
    let aus = h.suche(&a, 0, 50, true).unwrap();
    assert_eq!(aus.gesamt, Some(1));
    assert_eq!(aus.zeilen[0].uid, "u1");
}

#[test]
fn volltext_faltet_diakritika() {
    let mut h = Datenhaltung::im_speicher().unwrap();
    h.eintrag_anlegen(&eintrag("u9", "Größe Bär", Format::Pes))
        .unwrap();
    // `remove_diacritics 2` — „Bar" findet „Bär".
    let a = Suchabfrage {
        text: Some("Bar".into()),
        ..Default::default()
    };
    assert_eq!(h.suche(&a, 0, 50, true).unwrap().gesamt, Some(1));
}

#[test]
fn filter_und_volltext_wirken_zusammen() {
    let h = bestand();
    let a = Suchabfrage {
        text: Some("Herz".into()),
        formate: vec![Format::Dst],
        ..Default::default()
    };
    // „Herz" gibt es, aber nicht als DST.
    assert_eq!(h.suche(&a, 0, 50, true).unwrap().gesamt, Some(0));
}

#[test]
fn formatfilter_grenzt_ein() {
    let h = bestand();
    let a = Suchabfrage {
        formate: vec![Format::Pes, Format::Dst],
        ..Default::default()
    };
    assert_eq!(h.suche(&a, 0, 50, true).unwrap().gesamt, Some(2));
}

#[test]
fn schlagwortfilter_grenzt_ein() {
    let h = bestand();
    let a = Suchabfrage {
        schlagworte: vec!["Blume".into()],
        ..Default::default()
    };
    let aus = h.suche(&a, 0, 50, true).unwrap();
    assert_eq!(aus.gesamt, Some(1));
    assert_eq!(aus.zeilen[0].uid, "u2");
}

// --- SM-SEC-005: Einschleusung über den Suchpfad ---

#[test]
fn sonderzeichen_im_suchtext_schleusen_nicht_ein() {
    let h = bestand();
    // Diese Texte sind für FTS5 bedeutungstragend oder syntaktisch falsch.
    // Keiner darf die Abfrage scheitern lassen oder ihre Struktur ändern.
    for boes in [
        "\"",
        "\" OR 1=1 --",
        "a\" NEAR b",
        "*",
        "NEAR(a b)",
        "'; DROP TABLE eintrag; --",
        "Rosen*",
        "^abc",
        "a AND b OR c",
    ] {
        let a = Suchabfrage {
            text: Some(boes.into()),
            ..Default::default()
        };
        let ergebnis = h.suche(&a, 0, 50, true);
        assert!(
            ergebnis.is_ok(),
            "Suchtext {boes:?} ließ die Abfrage scheitern"
        );
    }
    // Der Bestand steht noch.
    assert_eq!(h.bestandsgroesse().unwrap(), 3);
}

#[test]
fn quotierung_macht_aus_wortfolge_eine_phrasenfolge() {
    assert_eq!(quotiere_fts("a b"), Some("\"a\" \"b\"".into()));
    assert_eq!(quotiere_fts("a\"b"), Some("\"a\"\"b\"".into()));
    assert_eq!(quotiere_fts("   "), None);
}

// --- Schnittregel 3: Ausschnitt und Gesamtzahl ---

#[test]
fn gesamtzahl_nur_beim_ersten_ausschnitt() {
    let h = bestand();
    let a = Suchabfrage::default();

    let erster = h.suche(&a, 0, 2, true).unwrap();
    assert_eq!(erster.gesamt, Some(3));
    assert_eq!(erster.zeilen.len(), 2);

    let zweiter = h.suche(&a, 2, 2, false).unwrap();
    assert_eq!(
        zweiter.gesamt, None,
        "Folgeausschnitt trug die Gesamtzahl erneut — das ist je Bildlaufabschnitt eine Zählabfrage"
    );
    assert_eq!(zweiter.zeilen.len(), 1);
}

#[test]
fn ausschnitt_ueberschneidet_sich_nicht() {
    let h = bestand();
    let a = Suchabfrage::default();
    let e = h.suche(&a, 0, 2, false).unwrap();
    let z = h.suche(&a, 2, 2, false).unwrap();
    let ids: Vec<i64> = e
        .zeilen
        .iter()
        .chain(z.zeilen.iter())
        .map(|r| r.id)
        .collect();
    let mut sortiert = ids.clone();
    sortiert.sort_unstable();
    sortiert.dedup();
    assert_eq!(ids.len(), sortiert.len(), "Ausschnitte überschnitten sich");
}

#[test]
fn ausschnittgroesse_ist_gedeckelt() {
    let h = bestand();
    // Ein Fremdwert darf keinen vollständigen Ergebnistransfer auslösen.
    let aus = h
        .suche(&Suchabfrage::default(), 0, 1_000_000, false)
        .unwrap();
    assert!(aus.zeilen.len() <= 500);
}

#[test]
fn negativer_versatz_wird_gefangen() {
    let h = bestand();
    let aus = h.suche(&Suchabfrage::default(), -5, 10, false).unwrap();
    assert_eq!(aus.versatz, 0);
}

// --- Sortierung ---

#[test]
fn sortierung_nach_name_ist_stabil_und_alphabetisch() {
    let h = bestand();
    let a = Suchabfrage {
        sortierung: Sortierung::Name,
        ..Default::default()
    };
    let namen: Vec<String> = h
        .suche(&a, 0, 50, false)
        .unwrap()
        .zeilen
        .into_iter()
        .map(|z| z.name)
        .collect();
    let mut erwartet = namen.clone();
    erwartet.sort_by_key(|s| s.to_lowercase());
    assert_eq!(namen, erwartet);
}

#[test]
fn relevanz_ohne_volltext_faellt_auf_name_zurueck() {
    let h = bestand();
    // Ohne Volltext gibt es keinen Rang — die Abfrage darf nicht scheitern.
    let a = Suchabfrage {
        sortierung: Sortierung::Relevanz,
        ..Default::default()
    };
    assert!(h.suche(&a, 0, 50, false).is_ok());
}

// --- Garnfarben ---

#[test]
fn garnfarben_kommen_in_reihenfolge_zurueck() {
    let mut h = Datenhaltung::im_speicher().unwrap();
    let mut e = eintrag("u1", "Herz", Format::Pes);
    e.farben = vec![
        Garnfarbe {
            hex: "AAAAAA".into(),
            name: None,
            marke: None,
            markenschluessel: None,
        },
        Garnfarbe {
            hex: "BBBBBB".into(),
            name: None,
            marke: None,
            markenschluessel: None,
        },
    ];
    let id = h.eintrag_anlegen(&e).unwrap();
    let farben = h.garnfarben(id).unwrap();
    assert_eq!(farben.len(), 2);
    assert_eq!(farben[0].hex, "AAAAAA");
    assert_eq!(farben[1].hex, "BBBBBB");
}

#[test]
fn geloeschter_eintrag_nimmt_seine_farben_mit() {
    let mut h = Datenhaltung::im_speicher().unwrap();
    let id = h
        .eintrag_anlegen(&eintrag("u1", "Herz", Format::Pes))
        .unwrap();
    h.conn
        .execute("DELETE FROM eintrag WHERE id = ?1", params![id])
        .unwrap();
    assert!(h.garnfarben(id).unwrap().is_empty());
}

// --- Newton-6: ein Konflikt verwirft den Block nicht ---

#[test]
fn ein_konflikt_verwirft_den_block_nicht() {
    // SM-IMP-009: Eine problematische Datei darf den Lauf nicht abbrechen.
    // Zwei Kopien derselben Datei in verschiedenen Ordnern bekommen nach
    // `kennung()` dieselbe Kennung und verletzen `uid UNIQUE`.
    let mut h = Datenhaltung::im_speicher().unwrap();
    let mut block: Vec<NeuerEintrag> = (0..10)
        .map(|i| eintrag(&format!("u{i}"), &format!("Muster {i}"), Format::Pes))
        .collect();
    // Ein Doppelgänger mittendrin.
    let mut doppelt = eintrag("u3", "Kopie", Format::Pes);
    doppelt.pfad = std::path::PathBuf::from("/bestand/anderer/ordner/u3.pes");
    block.insert(5, doppelt);

    let (aufgenommen, abgewiesen) = h.eintraege_anlegen(&block).unwrap();
    // Der Doppelgänger wird jetzt **vereindeutigt**, nicht abgewiesen — beide
    // Aufnahmewege liefern für denselben Bestand dasselbe Ergebnis (M-5).
    assert_eq!(
        aufgenommen, 11,
        "der Doppelgänger fiel weg statt vereindeutigt"
    );
    assert!(
        abgewiesen.is_empty(),
        "unerwartete Abweisung: {:?}",
        abgewiesen
    );
    assert_eq!(h.bestandsgroesse().unwrap(), 11);
    // Beide Dateien sind auffindbar.
    assert!(h.eintrag_nach_uid("u3").unwrap().is_some());
    assert!(h.eintrag_nach_uid("u3-1").unwrap().is_some());
}

// --- Newton-2: der Volltextauslöser feuert nur bei Textänderungen ---

#[test]
fn vermisstmarke_schreibt_den_volltextindex_nicht_um() {
    let mut h = Datenhaltung::im_speicher().unwrap();
    let id = h
        .eintrag_anlegen(&eintrag("u1", "Bayrisches Herz", Format::Pes))
        .unwrap();

    // Der Eintrag ist über den Volltext auffindbar …
    let a = Suchabfrage {
        text: Some("Bayrisches".into()),
        ..Default::default()
    };
    assert_eq!(h.suche(&a, 0, 10, true).unwrap().gesamt, Some(1));

    // … und bleibt es, nachdem er als vermisst gekennzeichnet wurde.
    h.als_vermisst_kennzeichnen(&[id]).unwrap();
    assert_eq!(
        h.suche(&a, 0, 10, true).unwrap().gesamt,
        Some(1),
        "die Vermisstmarke hat den Volltextindex angetastet"
    );

    // Eine echte Textänderung wirkt weiterhin.
    h.schlagworte_setzen(id, &["Volksfest".into()]).unwrap();
    let b = Suchabfrage {
        text: Some("Volksfest".into()),
        ..Default::default()
    };
    assert_eq!(h.suche(&b, 0, 10, true).unwrap().gesamt, Some(1));
}

// --- Newton-2: inhaltsgleiche Dateien fallen nicht still weg ---

#[test]
fn inhaltsgleiche_dateien_bekommen_beide_einen_eintrag() {
    // Zwei Kopien derselben Datei in verschiedenen Ordnern erzeugen nach
    // `kennung()` dieselbe Kennung. Ohne Vereindeutigung fiele die zweite
    // still weg und würde bei **jedem** Lauf erneut gelesen — SM-IMP-003.
    let mut h = Datenhaltung::im_speicher().unwrap();
    let erst = eintrag("gleich", "Herz", Format::Pes);
    let mut zweit = eintrag("gleich", "Herz", Format::Pes);
    zweit.pfad = std::path::PathBuf::from("/bestand/anderer/ordner/gleich.pes");

    let a = h.eintrag_anlegen(&erst).unwrap();
    let b = h.eintrag_anlegen(&zweit).unwrap();

    assert_ne!(a, b);
    assert_eq!(h.bestandsgroesse().unwrap(), 2, "die zweite Datei fiel weg");
    // Die erste behält ihre Kennung; die zweite bekommt eine eigene.
    assert_eq!(h.eintrag_nach_uid("gleich").unwrap(), Some(a));
    // Der Zusatz zählt ab der Zahl der bereits belegten Kennungen.
    assert_eq!(h.eintrag_nach_uid("gleich-1").unwrap(), Some(b));
}

#[test]
fn viele_inhaltsgleiche_dateien_bleiben_bezahlbar() {
    // Aufsteigend zu probieren kostete für die k-te Kopie k Abfragen — über
    // den Bestand O(k²). Der Fall ist in Stickbibliotheken alltäglich
    // (Sicherungsordner) und war bis dahin unmessbar: Der Messaufbau erzeugt
    // ausschließlich eindeutige Kennungen.
    let mut h = Datenhaltung::im_speicher().unwrap();
    let anzahl = 300;
    for i in 0..anzahl {
        let mut e = eintrag("gleich", "Kopie", Format::Pes);
        e.pfad = std::path::PathBuf::from(format!("/bestand/ordner{i}/gleich.pes"));
        h.eintrag_anlegen(&e).unwrap();
    }
    assert_eq!(h.bestandsgroesse().unwrap(), anzahl as i64);

    // Die letzte Aufnahme kostet nicht mehr Abfragen als die erste.
    h.abfragen_zuruecksetzen();
    let mut e = eintrag("gleich", "Kopie", Format::Pes);
    e.pfad = std::path::PathBuf::from("/bestand/noch-einer/gleich.pes");
    h.eintrag_anlegen(&e).unwrap();
    assert!(
        h.abfragen_gesamt() <= 3,
        "die Kennungsvergabe kostete {} Abfragen",
        h.abfragen_gesamt()
    );
}
