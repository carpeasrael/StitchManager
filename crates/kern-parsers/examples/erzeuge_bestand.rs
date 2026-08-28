//! Erzeugt einen Prüfbestand aus echten, lesbaren Stickdateien.
//!
//! Das ist Stufe (a) des Prüfbestands aus AP-02: ein **erzeugter** Dateibestand,
//! reproduzierbar und ohne Datenbankschema. Er ersetzt im Repository abgelegte
//! Binärdateien, für die sonst eine Herkunftsangabe zu führen wäre
//! (CLAUDE.md Abschnitt 13, Stufe 0.4).
//!
//! ```text
//! cargo run --release -p kern-parsers --example erzeuge_bestand -- <ziel> <anzahl>
//! ```

fn main() {
    let ziel = std::env::args().nth(1).unwrap();
    let n: usize = std::env::args().nth(2).unwrap().parse().unwrap();
    std::fs::create_dir_all(&ziel).unwrap();
    // Auch der Prüfbestand wird über ein geprüftes Ziel geschrieben — die
    // Regel gilt für Werkzeuge wie für Anwendungscode (Schnittregel 5).
    let wurzel = kern_security::Wurzel::neu(&ziel).unwrap();
    // Über Unterordner streuen: Ein flacher Bestand verdeckt Mängel im
    // Pfadweg (SM-LIB-002, SM-LIB-003).
    for f in 0..16 {
        std::fs::create_dir_all(std::path::Path::new(&ziel).join(format!("gruppe{f:02}"))).unwrap();
    }
    for i in 0..n {
        let punkte: Vec<(f64, f64)> = (0..120)
            .map(|k| {
                (
                    (k % 40) as f64 + (i % 7) as f64,
                    ((k * (i + 3)) % 37) as f64,
                )
            })
            .collect();
        kern_parsers::writers::write_dst(
            &[kern_parsers::StitchSegment {
                color_index: 0,
                color_hex: Some("C8102E".into()),
                points: punkte,
            }],
            &wurzel
                .schreibziel(
                    std::path::Path::new(&ziel)
                        .join(format!("gruppe{:02}", i % 16))
                        .join(format!("muster{i:05}.dst")),
                )
                .unwrap(),
        )
        .unwrap();
    }
    println!("{n} Dateien erzeugt");
}
