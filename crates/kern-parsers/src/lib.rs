//! Formatparser — lesend und gehärtet.
//!
//! Trägt SM-FMT-001 bis 013 und SM-SEC-011. Übernommen aus dem Kern des
//! Vorgängers (OP-13, Entscheidung vom 2026-08-25) und von der abgelösten
//! Rahmenbibliothek gelöst.
//!
//! **Der Import ist der Upload-Pfad dieses Programms.** Jede eingelesene Datei
//! ist Fremddaten, auch wenn sie aus dem Dateisystem der Nutzerin stammt. Die
//! Parser laufen deshalb ausschließlich über die Hüllen in [`sicher`], die eine
//! Panik in einen behandelbaren Fehler wandeln (SM-FMT-012).

#![forbid(unsafe_code)]

mod dst;
mod jef;
mod pes;
mod vp3;
pub mod writers;

use kern_typen::{Ergebnis, Fehler, Format, Garnfarbe, Kennwerte, Stichabschnitt};

/// Obergrenze für die Zahl der Stichpunkte je Datei.
///
/// SM-FMT-012 verlangt „keine unbegrenzte Speicherbelegung" für **jeden**
/// Parser. DST und PEC tragen je eine eigene Grenze; JEF und VP3 hatten keine.
/// Eine Datei innerhalb der Lesegrenze von 256 MiB kann über hundert Millionen
/// Stichpunkte beschreiben — bei 16 Byte je Punkt sind das mehrere Gigabyte.
///
/// Zwei Millionen Punkte sind großzügig: Das größte bekannte Stickmuster liegt
/// im niedrigen sechsstelligen Bereich.
pub(crate) const HOECHSTZAHL_PUNKTE: usize = 2_000_000;

/// Kennwerte, wie die Parser sie liefern.
///
/// Bewusst kisteneigen: Die Parserlogik ist unverändert übernommen, die
/// Umsetzung auf die Kerntypen geschieht an der Kistengrenze. So bleibt der
/// Formatcode frei von Anpassungen, die sich nur schwer gegenlesen lassen.
#[derive(Debug, Clone)]
pub struct ParsedFileInfo {
    pub format: String,
    pub format_version: Option<String>,
    pub width_mm: Option<f64>,
    pub height_mm: Option<f64>,
    pub stitch_count: Option<i32>,
    pub color_count: Option<i32>,
    pub colors: Vec<ParsedColor>,
    pub design_name: Option<String>,
    pub jump_count: Option<i32>,
    pub trim_count: Option<i32>,
    pub hoop_width_mm: Option<f64>,
    pub hoop_height_mm: Option<f64>,
    pub category: Option<String>,
    pub author: Option<String>,
    pub keywords: Option<String>,
    pub comments: Option<String>,
    pub page_count: Option<i32>,
    pub paper_size: Option<String>,
}

/// Eine Garnfarbe, wie die Parser sie liefern.
#[derive(Debug, Clone)]
pub struct ParsedColor {
    pub hex: String,
    pub name: Option<String>,
    pub brand: Option<String>,
    pub brand_code: Option<String>,
}

/// Ein Stichabschnitt einer Farblage.
#[derive(Debug, Clone)]
pub struct StitchSegment {
    pub color_index: usize,
    pub color_hex: Option<String>,
    pub points: Vec<(f64, f64)>,
}

impl From<ParsedColor> for Garnfarbe {
    fn from(f: ParsedColor) -> Self {
        Garnfarbe {
            hex: f.hex,
            name: f.name,
            marke: f.brand,
            markenschluessel: f.brand_code,
        }
    }
}

impl From<StitchSegment> for Stichabschnitt {
    fn from(s: StitchSegment) -> Self {
        Stichabschnitt {
            farbindex: s.color_index,
            farbe_hex: s.color_hex,
            punkte: s.points,
        }
    }
}

impl From<ParsedFileInfo> for Kennwerte {
    fn from(p: ParsedFileInfo) -> Self {
        Kennwerte {
            format_version: p.format_version,
            breite_mm: p.width_mm,
            hoehe_mm: p.height_mm,
            stichzahl: p.stitch_count.map(i64::from),
            farbzahl: p.color_count.map(i64::from),
            farben: p.colors.into_iter().map(Garnfarbe::from).collect(),
            entwurfsname: p.design_name,
            spruenge: p.jump_count.map(i64::from),
            schnitte: p.trim_count.map(i64::from),
            rahmen_breite_mm: p.hoop_width_mm,
            rahmen_hoehe_mm: p.hoop_height_mm,
        }
    }
}

/// Gemeinsame Schnittstelle aller Formatparser.
pub trait EmbroideryParser: Send + Sync {
    fn supported_extensions(&self) -> &[&str];
    fn parse(&self, data: &[u8]) -> Result<ParsedFileInfo, Fehler>;
    fn extract_thumbnail(&self, data: &[u8]) -> Result<Option<Vec<u8>>, Fehler>;
    fn extract_stitch_segments(&self, data: &[u8]) -> Result<Vec<StitchSegment>, Fehler>;
}

/// Panikisolierende Hüllen um die Parseraufrufe (SM-FMT-012, SM-SEC-011).
///
/// Ein Parser, der über einer manipulierten Datei in Panik gerät, darf den
/// Prozess nicht mitnehmen: Die Bibliothek und der Schlüsselspeicher hängen
/// daran. Die Panik wird zu [`Fehler::Parser`].
pub mod sicher {
    use super::{EmbroideryParser, Fehler, Format, ParsedFileInfo, StitchSegment};
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn panik_zu_fehler<T>(
        format: Format,
        ergebnis: Result<Result<T, Fehler>, Box<dyn std::any::Any + Send>>,
    ) -> Result<T, Fehler> {
        match ergebnis {
            Ok(inneres) => inneres,
            Err(nutzlast) => {
                let meldung = nutzlast
                    .downcast_ref::<&'static str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| nutzlast.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "unbekannt".to_string());
                // Der technische Text gehört ins Protokoll, nicht in die
                // Meldung an die Nutzerin (SM-NFR-006).
                log::warn!("Panik im Parser abgefangen: {meldung}");
                // Das Format ist an der Aufrufstelle bekannt. Ohne es lautete
                // der Satz „… kein gültiges unbekannt-Stickmuster." — kein
                // richtiges Deutsch, und er erscheint genau dort, wo er nach
                // SM-IMP-009 sichtbar werden soll.
                Err(Fehler::Parser {
                    format: format.marke().to_string(),
                    meldung: "Die Datei ist beschädigt oder kein gültiges Stickmuster.".to_string(),
                })
            }
        }
    }

    pub fn parse(
        format: Format,
        parser: &dyn EmbroideryParser,
        daten: &[u8],
    ) -> Result<ParsedFileInfo, Fehler> {
        panik_zu_fehler(
            format,
            catch_unwind(AssertUnwindSafe(|| parser.parse(daten))),
        )
    }

    pub fn extract_thumbnail(
        format: Format,
        parser: &dyn EmbroideryParser,
        daten: &[u8],
    ) -> Result<Option<Vec<u8>>, Fehler> {
        panik_zu_fehler(
            format,
            catch_unwind(AssertUnwindSafe(|| parser.extract_thumbnail(daten))),
        )
    }

    pub fn extract_stitch_segments(
        format: Format,
        parser: &dyn EmbroideryParser,
        daten: &[u8],
    ) -> Result<Vec<StitchSegment>, Fehler> {
        panik_zu_fehler(
            format,
            catch_unwind(AssertUnwindSafe(|| parser.extract_stitch_segments(daten))),
        )
    }
}

/// Liefert den Parser zu einem Format.
pub fn parser_fuer(format: Format) -> Option<Box<dyn EmbroideryParser>> {
    match format {
        Format::Pes => Some(Box::new(pes::PesParser)),
        Format::Dst => Some(Box::new(dst::DstParser)),
        Format::Jef => Some(Box::new(jef::JefParser)),
        Format::Vp3 => Some(Box::new(vp3::Vp3Parser)),
        // EXP und XXX sind im Lesepfad der Version 1.0 nicht verplant
        // (IMP-STM-001 Kapitel 10). Kein stiller Rückfall auf einen
        // fremden Parser: Ein falsch geratenes Format liefert falsche Maße.
        Format::Exp | Format::Xxx => None,
    }
}

/// Liest die Kennwerte einer Datei, gehärtet und auf die Kerntypen umgesetzt.
pub fn lies_kennwerte(format: Format, daten: &[u8]) -> Ergebnis<Kennwerte> {
    let parser = parser_fuer(format).ok_or_else(|| Fehler::FormatOhneLeseweg {
        format: format.marke().to_string(),
    })?;
    sicher::parse(format, parser.as_ref(), daten).map(Kennwerte::from)
}

/// Liest die Stichabschnitte einer Datei, gehärtet und umgesetzt.
pub fn lies_stichabschnitte(format: Format, daten: &[u8]) -> Ergebnis<Vec<Stichabschnitt>> {
    let parser = parser_fuer(format).ok_or_else(|| Fehler::FormatOhneLeseweg {
        format: format.marke().to_string(),
    })?;
    let abschnitte = sicher::extract_stitch_segments(format, parser.as_ref(), daten)?;

    // Die Dekodierer brechen an der Punktgrenze ab. Wurde sie erreicht, ist
    // das Ergebnis **unvollständig** — und eine unvollständige Vorschau ohne
    // Hinweis ist irreführend (AK-03, DES-STM-001 Abschnitt 10). Statt still
    // zu kürzen, wird der Fall gemeldet; die Datei bleibt unverändert.
    let punkte: usize = abschnitte.iter().map(|a| a.points.len()).sum();
    if punkte >= HOECHSTZAHL_PUNKTE {
        return Err(Fehler::ZuVieleStiche {
            grenze: HOECHSTZAHL_PUNKTE,
        });
    }

    Ok(abschnitte.into_iter().map(Stichabschnitt::from).collect())
}

#[cfg(test)]
mod haertung {
    use super::*;

    /// Kein Format darf über Zufallsdaten den Prozess mitnehmen (SM-FMT-012).
    #[test]
    fn zufallsdaten_erzeugen_fehler_statt_panik() {
        let formate = [Format::Pes, Format::Dst, Format::Jef, Format::Vp3];
        // Deterministischer Pseudozufall — der Fall ist reproduzierbar.
        let mut zustand: u64 = 0x5DEE_CE66_D1CE_4B9F;
        for durchgang in 0..64 {
            let mut daten = vec![0u8; 64 + durchgang * 16];
            for b in daten.iter_mut() {
                zustand = zustand
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                *b = (zustand >> 33) as u8;
            }
            for f in formate {
                // Entscheidend ist allein: kein Absturz, kein Hänger.
                let _ = lies_kennwerte(f, &daten);
                let _ = lies_stichabschnitte(f, &daten);
            }
        }
    }

    #[test]
    fn leere_und_abgeschnittene_dateien_erzeugen_fehler() {
        for f in [Format::Pes, Format::Dst, Format::Jef, Format::Vp3] {
            assert!(
                lies_kennwerte(f, &[]).is_err(),
                "{f}: leere Datei angenommen"
            );
            assert!(lies_kennwerte(f, &[0x00]).is_err());
        }
    }

    /// EXP und XXX sind **geführte** Formate ohne Leseweg in dieser Fassung
    /// (IMP-STM-001 Kapitel 10). Sie als „beschädigt" zu melden wäre eine
    /// Falschaussage über eine intakte Datei.
    #[test]
    fn gefuehrte_formate_ohne_leseweg_werden_nicht_als_beschaedigt_gemeldet() {
        for f in [Format::Exp, Format::Xxx] {
            let e = lies_kennwerte(f, &[0u8; 128]).unwrap_err();
            assert!(
                matches!(e, Fehler::FormatOhneLeseweg { .. }),
                "{f}: falsche Fehlerart — {e}"
            );
            let text = e.to_string();
            assert!(
                !text.contains("beschädigt"),
                "{f}: intakte Datei als beschädigt gemeldet — {text}"
            );
            assert!(
                text.contains(f.marke()),
                "{f}: Format nicht genannt — {text}"
            );
        }
    }
}
