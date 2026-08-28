//! Vorschauerzeugung aus Stichdaten (SM-PRV-001 bis 003).
//!
//! Die Vorschau entsteht aus den **Stichdaten**, nicht aus einem in der Datei
//! mitgelieferten Vorschaubild: Nur so zeigt sie, was die Maschine tatsächlich
//! stickt, und nur so gibt es sie für jedes Format.
//!
//! Das Modul kennt keine Themenfarben. Der Ausdruck übernimmt keine
//! Themenfarben (SM-PRN-015), und dieselbe Vorschau trägt Hell- und
//! Dunkelmodus — sie darf deshalb an keiner Stelle an `ui/gestaltung` hängen.

#![forbid(unsafe_code)]

pub mod zwischenspeicher;
pub use zwischenspeicher::{Stufe, Zwischenspeicher};

use kern_typen::{Ergebnis, Fehler, Stichabschnitt};

/// Maße und Hintergrund der zu zeichnenden Vorschau.
#[derive(Debug, Clone, Copy)]
pub struct Vorschauoption {
    pub breite_px: u32,
    pub hoehe_px: u32,
    /// Randabstand in Bildpunkten.
    pub rand_px: u32,
    /// Hintergrund als RGBA. Durchsichtig lässt die Kachel durchscheinen.
    pub hintergrund: [u8; 4],
    /// Strichstärke in Bildpunkten.
    pub stichstaerke: u32,
}

impl Default for Vorschauoption {
    fn default() -> Self {
        Self {
            breite_px: 512,
            hoehe_px: 512,
            rand_px: 16,
            hintergrund: [0, 0, 0, 0],
            stichstaerke: 1,
        }
    }
}

/// Eine gezeichnete Vorschau als RGBA-Punktfeld.
pub struct Vorschau {
    pub breite: u32,
    pub hoehe: u32,
    pub punkte: Vec<u8>,
}

impl Vorschau {
    /// Kodiert die Vorschau als PNG — die Form des Zwischenspeichers.
    pub fn als_png(&self) -> Ergebnis<Vec<u8>> {
        let puffer = image::RgbaImage::from_raw(self.breite, self.hoehe, self.punkte.clone())
            .ok_or_else(|| Fehler::Intern("Punktfeld passt nicht zu den Maßen".into()))?;
        let mut aus = std::io::Cursor::new(Vec::new());
        puffer
            .write_to(&mut aus, image::ImageFormat::Png)
            .map_err(|e| Fehler::Intern(format!("PNG-Kodierung: {e}")))?;
        Ok(aus.into_inner())
    }
}

/// Umgrenzung der Stichdaten in Millimetern.
fn umgrenzung(abschnitte: &[Stichabschnitt]) -> Option<(f64, f64, f64, f64)> {
    let (mut x0, mut y0) = (f64::MAX, f64::MAX);
    let (mut x1, mut y1) = (f64::MIN, f64::MIN);
    let mut gesehen = false;

    for a in abschnitte {
        for &(x, y) in &a.punkte {
            // Nicht darstellbare Werte aus einer beschädigten Datei dürfen die
            // Umgrenzung nicht vergiften (SM-FMT-012).
            if !x.is_finite() || !y.is_finite() {
                continue;
            }
            gesehen = true;
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
    }
    gesehen.then_some((x0, y0, x1, y1))
}

fn farbe_aus_hex(hex: &Option<String>) -> [u8; 4] {
    // Der Rückfallton muss in **beiden** Modi tragen, ohne eine Themenfarbe zu
    // sein — die Vorschau ist dieselbe für Hell und Dunkel, und der DST-Parser
    // liefert für jede Farblage `None` (das Format trägt keine Farbe).
    //
    // Gerechnet gegen die Flächen aus DES-STM-001 Abschnitt 3 (WCAG 2.x,
    // relative Luminanz):
    //
    //   Ton       gegen --kn-bg hell   gegen --kn-bg dunkel
    //   #333333          11,74:1               1,34:1   ← unbrauchbar  D-05-Ausnahme: verworfener Kandidat der Kontrastrechnung, keine Themenfarbe.
    //   #787878           4,10:1               3,84:1   ← gewählt      D-05-Ausnahme: Rückfallton der Vorschau, bewusst außerhalb des Themas.
    //   #999999           2,65:1               5,96:1                  D-05-Ausnahme: verworfener Kandidat der Kontrastrechnung, keine Themenfarbe.
    //
    // Der zuvor gewählte Ton war im Dunkelmodus praktisch unsichtbar; betroffen
    // war jede DST-Vorschau (SM-PRV-001, AK-03, SM-NFR-007).
    let vorgabe = [0x78, 0x78, 0x78, 0xFF];
    let Some(h) = hex else { return vorgabe };
    let h = h.trim().trim_start_matches('#');
    if h.len() != 6 {
        return vorgabe;
    }
    match (
        u8::from_str_radix(&h[0..2], 16),
        u8::from_str_radix(&h[2..4], 16),
        u8::from_str_radix(&h[4..6], 16),
    ) {
        (Ok(r), Ok(g), Ok(b)) => [r, g, b, 0xFF],
        _ => vorgabe,
    }
}

/// Zeichnet die Vorschau maßstabsgetreu und mittig.
///
/// Das Seitenverhältnis bleibt erhalten — ein verzerrtes Muster wäre als
/// Vorschau wertlos. Die Kachelhöhe steht dadurch **vor** dem Zeichnen fest
/// (SM-PRV-009): Sie hängt an der Option, nicht an den Daten.
pub fn zeichne(abschnitte: &[Stichabschnitt], opt: Vorschauoption) -> Ergebnis<Vorschau> {
    let breite = opt.breite_px.max(1);
    let hoehe = opt.hoehe_px.max(1);

    let mut punkte = Vec::new();
    punkte
        .try_reserve((breite as usize) * (hoehe as usize) * 4)
        .map_err(|_| Fehler::Intern("Die Vorschau ist zu groß für den Arbeitsspeicher".into()))?;
    for _ in 0..(breite as usize) * (hoehe as usize) {
        punkte.extend_from_slice(&opt.hintergrund);
    }

    let mut bild = Bild {
        breite,
        hoehe,
        punkte,
    };

    let Some((x0, y0, x1, y1)) = umgrenzung(abschnitte) else {
        // Keine darstellbaren Stichdaten: leere Vorschau statt Fehler. Der
        // leere Zustand ist eine Anzeigefrage, kein Programmfehler.
        return Ok(Vorschau {
            breite: bild.breite,
            hoehe: bild.hoehe,
            punkte: bild.punkte,
        });
    };

    let rand = opt.rand_px.min(breite / 4).min(hoehe / 4) as f64;
    let nutz_b = (breite as f64 - 2.0 * rand).max(1.0);
    let nutz_h = (hoehe as f64 - 2.0 * rand).max(1.0);

    let spanne_x = (x1 - x0).max(1e-6);
    let spanne_y = (y1 - y0).max(1e-6);
    let massstab = (nutz_b / spanne_x).min(nutz_h / spanne_y);

    // Mittig setzen.
    let versatz_x = rand + (nutz_b - spanne_x * massstab) / 2.0;
    let versatz_y = rand + (nutz_h - spanne_y * massstab) / 2.0;

    let auf_bild = |x: f64, y: f64| -> (i64, i64) {
        (
            (versatz_x + (x - x0) * massstab).round() as i64,
            // Die Bildachse zeigt nach unten, die Stickachse nach oben.
            (versatz_y + (y1 - y) * massstab).round() as i64,
        )
    };

    for a in abschnitte {
        let farbe = farbe_aus_hex(&a.farbe_hex);
        let mut vorher: Option<(i64, i64)> = None;
        for &(x, y) in &a.punkte {
            if !x.is_finite() || !y.is_finite() {
                vorher = None;
                continue;
            }
            let jetzt = auf_bild(x, y);
            if let Some(v) = vorher {
                bild.linie(v, jetzt, farbe, opt.stichstaerke.max(1));
            }
            vorher = Some(jetzt);
        }
    }

    Ok(Vorschau {
        breite: bild.breite,
        hoehe: bild.hoehe,
        punkte: bild.punkte,
    })
}

struct Bild {
    breite: u32,
    hoehe: u32,
    punkte: Vec<u8>,
}

impl Bild {
    fn setze(&mut self, x: i64, y: i64, farbe: [u8; 4]) {
        if x < 0 || y < 0 || x >= self.breite as i64 || y >= self.hoehe as i64 {
            return;
        }
        let i = ((y as usize) * (self.breite as usize) + (x as usize)) * 4;
        self.punkte[i..i + 4].copy_from_slice(&farbe);
    }

    /// Bresenham — ganzzahlig, ohne Gleitkommafehler auf langen Strecken.
    fn linie(&mut self, a: (i64, i64), b: (i64, i64), farbe: [u8; 4], staerke: u32) {
        let (mut x0, mut y0) = a;
        let (x1, y1) = b;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut fehler = dx + dy;

        // Eine sehr lange Strecke aus einer beschädigten Datei darf die
        // Schleife nicht unbegrenzt laufen lassen (SM-FMT-012).
        let hoechstschritte = (self.breite as i64 + self.hoehe as i64) * 4;
        let mut schritte = 0i64;

        loop {
            if staerke <= 1 {
                self.setze(x0, y0, farbe);
            } else {
                let r = (staerke / 2) as i64;
                for oy in -r..=r {
                    for ox in -r..=r {
                        self.setze(x0 + ox, y0 + oy, farbe);
                    }
                }
            }

            if x0 == x1 && y0 == y1 {
                break;
            }
            schritte += 1;
            if schritte > hoechstschritte {
                break;
            }

            let e2 = 2 * fehler;
            if e2 >= dy {
                fehler += dy;
                x0 += sx;
            }
            if e2 <= dx {
                fehler += dx;
                y0 += sy;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quadrat() -> Vec<Stichabschnitt> {
        vec![Stichabschnitt {
            farbindex: 0,
            farbe_hex: Some("C8102E".into()),
            punkte: vec![
                (0.0, 0.0),
                (50.0, 0.0),
                (50.0, 50.0),
                (0.0, 50.0),
                (0.0, 0.0),
            ],
        }]
    }

    fn gesetzte_punkte(v: &Vorschau) -> usize {
        v.punkte.chunks_exact(4).filter(|p| p[3] > 0).count()
    }

    #[test]
    fn masse_stehen_vor_den_daten_fest() {
        // SM-PRV-009: kein Layoutsprung — die Maße hängen an der Option.
        let opt = Vorschauoption {
            breite_px: 300,
            hoehe_px: 200,
            ..Default::default()
        };
        let v = zeichne(&quadrat(), opt).unwrap();
        assert_eq!((v.breite, v.hoehe), (300, 200));

        // Auch ohne jede Stichdatei bleiben die Maße dieselben.
        let leer = zeichne(&[], opt).unwrap();
        assert_eq!((leer.breite, leer.hoehe), (300, 200));
    }

    #[test]
    fn leere_stichdaten_liefern_leere_vorschau_statt_fehler() {
        let v = zeichne(&[], Vorschauoption::default()).unwrap();
        assert_eq!(gesetzte_punkte(&v), 0);
    }

    #[test]
    fn quadrat_wird_gezeichnet() {
        let v = zeichne(&quadrat(), Vorschauoption::default()).unwrap();
        assert!(
            gesetzte_punkte(&v) > 100,
            "es wurde praktisch nichts gezeichnet"
        );
    }

    #[test]
    fn seitenverhaeltnis_bleibt_erhalten() {
        // Ein 50 × 10 mm breites Muster darf in einem quadratischen Feld nicht
        // auf Quadratform gezogen werden.
        let breit = vec![Stichabschnitt {
            farbindex: 0,
            farbe_hex: None,
            punkte: vec![
                (0.0, 0.0),
                (50.0, 0.0),
                (50.0, 10.0),
                (0.0, 10.0),
                (0.0, 0.0),
            ],
        }];
        let v = zeichne(
            &breit,
            Vorschauoption {
                breite_px: 400,
                hoehe_px: 400,
                rand_px: 0,
                ..Default::default()
            },
        )
        .unwrap();

        let mut min_y = u32::MAX;
        let mut max_y = 0u32;
        for y in 0..v.hoehe {
            for x in 0..v.breite {
                let i = ((y * v.breite + x) * 4) as usize;
                if v.punkte[i + 3] > 0 {
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                }
            }
        }
        let gezeichnete_hoehe = max_y - min_y;
        // 400 px Breite bei Verhältnis 5:1 ergibt rund 80 px Höhe.
        assert!(
            (60..=100).contains(&gezeichnete_hoehe),
            "Höhe {gezeichnete_hoehe} px deutet auf Verzerrung"
        );
    }

    #[test]
    fn nicht_darstellbare_werte_werden_uebergangen() {
        // SM-FMT-012: eine beschädigte Datei darf die Vorschau nicht kippen.
        let kaputt = vec![Stichabschnitt {
            farbindex: 0,
            farbe_hex: Some("ZZZZZZ".into()),
            punkte: vec![
                (0.0, 0.0),
                (f64::NAN, 5.0),
                (f64::INFINITY, f64::NEG_INFINITY),
                (10.0, 10.0),
            ],
        }];
        let v = zeichne(&kaputt, Vorschauoption::default()).unwrap();
        assert_eq!((v.breite, v.hoehe), (512, 512));
    }

    #[test]
    fn ungueltiger_farbwert_faellt_auf_neutral_zurueck() {
        assert_eq!(farbe_aus_hex(&None), [0x78, 0x78, 0x78, 0xFF]);
        assert_eq!(
            farbe_aus_hex(&Some("nonsens".into())),
            [0x78, 0x78, 0x78, 0xFF]
        );
        assert_eq!(
            farbe_aus_hex(&Some("#C8102E".into())), // D-05-Ausnahme: Eingabewert eines Prüffalls, keine Themenfarbe.
            [0xC8, 0x10, 0x2E, 0xFF]
        );
    }

    /// Der Rückfallton trägt in **beiden** Modi — gerechnet, nicht geschätzt.
    ///
    /// Die Vorschau ist für Hell- und Dunkelmodus dieselbe; ein Ton, der nur
    /// gegen eine der beiden Flächen trägt, macht jede DST-Vorschau in der
    /// anderen unsichtbar (SM-NFR-007, AK-03).
    #[test]
    fn rueckfallton_traegt_in_beiden_modi() {
        fn linear(k: u8) -> f64 {
            let c = k as f64 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        }
        fn luminanz(f: [u8; 3]) -> f64 {
            0.2126 * linear(f[0]) + 0.7152 * linear(f[1]) + 0.0722 * linear(f[2])
        }
        fn kontrast(a: [u8; 3], b: [u8; 3]) -> f64 {
            let (la, lb) = (luminanz(a), luminanz(b));
            let (hoch, tief) = if la > lb { (la, lb) } else { (lb, la) };
            (hoch + 0.05) / (tief + 0.05)
        }

        let ton = farbe_aus_hex(&None);
        let ton = [ton[0], ton[1], ton[2]];
        // Die Flächenwerte stammen aus DES-STM-001 Abschnitt 3 (`--kn-bg`).
        let hell = [0xFA, 0xF6, 0xF0];
        let dunkel = [0x22, 0x1B, 0x17];

        let k_hell = kontrast(ton, hell);
        let k_dunkel = kontrast(ton, dunkel);
        assert!(k_hell >= 3.0, "gegen den Hellmodus nur {k_hell:.2}:1");
        assert!(k_dunkel >= 3.0, "gegen den Dunkelmodus nur {k_dunkel:.2}:1");
    }

    #[test]
    fn png_kodierung_traegt() {
        let v = zeichne(&quadrat(), Vorschauoption::default()).unwrap();
        let png = v.als_png().unwrap();
        assert_eq!(&png[1..4], b"PNG", "kein PNG-Kopf");
    }
}
