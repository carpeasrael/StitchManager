//! Gemeinsame Typen der Kernschicht.
//!
//! Diese Kiste hängt von keiner anderen Kiste des Verbunds ab. Sie trägt die
//! Werte, die zwischen Parsern, Datenhaltung, Vorschau und Fassade wandern,
//! sowie den Fehlertyp des Kerns.

#![forbid(unsafe_code)]

use std::fmt;

/// Fehler der Kernschicht.
///
/// Die Texte sind für Endnutzer verständlich (SM-NFR-006); technische Angaben
/// gehören in das Protokoll, nicht in die Meldung.
#[derive(Debug, thiserror::Error)]
pub enum Fehler {
    /// Ein Ein-/Ausgabefehler, **für Endnutzer formuliert**.
    ///
    /// Bewusst ohne `#[from]`: Ein durchgereichter `std::io::Error` liefert die
    /// englische Systemmeldung („Input/output error (os error 5)") an die
    /// Oberfläche. Genau die Lagen aus SM-NFR-005 — fehlende Berechtigung,
    /// entfernter Wechseldatenträger — landeten so unübersetzt beim Nutzer.
    /// Die Aufrufstelle bildet ab; die Originalmeldung geht ins Protokoll
    /// (SM-SEC-010).
    #[error("{0}")]
    Ea(String),

    /// Der angezeigte Satz nennt **kein** technisches Detail.
    ///
    /// Die Parser formulieren ihre Meldungen englisch und technisch
    /// („PES: header too short", „DST triplet count exceeded …"). Sie gehören
    /// ins Protokoll, nicht auf den Bildschirm (SM-NFR-006, SM-SET-006); der
    /// Text bleibt im Wert erhalten und ist über [`Fehler::technisch`]
    /// abrufbar.
    #[error("Die Datei ist beschädigt oder kein gültiges {format}-Stickmuster.")]
    Parser { format: String, meldung: String },

    /// Die Datei ist zu groß, um eingelesen zu werden.
    ///
    /// Eigene Variante statt [`Fehler::ParserGrenze`]: Dort ist die Nutzlast
    /// englischer Parserjargon und wird bewusst verworfen — hier ist die
    /// Grenze selbst die Aussage, die die Nutzerin braucht.
    #[error("Die Datei ist größer als {grenze_mib} MiB und wurde nicht eingelesen.")]
    DateiZuGross { grenze_mib: u64 },

    /// Die Datei beschreibt mehr Stiche, als ausgewertet werden.
    ///
    /// SM-FMT-012 verlangt eine Grenze; DES-STM-001 Abschnitt 10 und AK-03
    /// verlangen eine **korrekte** Vorschau. Eine stillschweigend gekürzte
    /// Vorschau erfüllt das erste und verfehlt das zweite: Die Nutzerin sähe
    /// ein unvollständiges Muster, ohne dass etwas darauf hinweist. Deshalb
    /// eine sichtbare Meldung statt einer stillen Kürzung.
    #[error(
        "Die Datei beschreibt mehr als {grenze} Stiche und wurde nicht \
             ausgewertet. Eine unvollständige Vorschau wäre irreführend."
    )]
    ZuVieleStiche { grenze: usize },

    /// Das Format ist bekannt, aber in dieser Fassung nicht lesbar.
    ///
    /// EXP und XXX sind in SM-FMT-001 geführt; ihr Leseweg ist in Version 1.0
    /// zurückgestellt (IMP-STM-001 Kapitel 10). Diese Dateien als „beschädigt"
    /// zu melden wäre eine Falschaussage über eine intakte Datei — die
    /// Zurückstellung ist zulässig, die Falschaussage nicht.
    #[error(
        "Dateien im Format {format} kann diese Fassung noch nicht lesen. \
             Die Datei bleibt unverändert erhalten."
    )]
    FormatOhneLeseweg { format: String },

    /// Wie [`Fehler::Parser`]: Der angezeigte Satz nennt **kein** technisches
    /// Detail. Der DST-Parser füllt `was` mit Sätzen wie „DST triplet count
    /// exceeded 1000000 without End marker (pos=512)" — englischer
    /// Offset-Jargon, der über den Fehlereintrag bis auf die Kachel lief
    /// (SM-SET-006). `technisch()` gibt ihn fürs Protokoll heraus.
    #[error(
        "Die Datei überschreitet eine Grenze des Lesewegs und wurde nicht \
             vollständig ausgewertet."
    )]
    ParserGrenze { was: String },

    #[error("Der Pfad liegt außerhalb des zulässigen Bereichs.")]
    PfadAusserhalb,

    #[error("Der Pfad ist ungültig: {0}")]
    PfadUngueltig(String),

    #[error("{0}")]
    NichtGefunden(String),

    /// Ohne Vorsilbe: „Ungültige Eingabe:" stand auch vor Sätzen, die keine
    /// Eingabe der Nutzerin betreffen (etwa „Es ist keine Bibliothek gewählt.").
    #[error("{0}")]
    Eingabe(String),

    /// Die Nutzlast **ist** der Anzeigetext (SM-NFR-006). Zuvor verwarf
    /// `Display` sie und zeigte nur das Wort „Datenbankfehler" — der sorgfältig
    /// formulierte Satz erreichte den Nutzer nie.
    #[error("{0}")]
    Datenbank(String),

    #[error("{0}")]
    Intern(String),
}

impl Fehler {
    /// Der technische Text zu einem Fehler — für das Protokoll, nie für die
    /// Anzeige (SM-SEC-010).
    pub fn technisch(&self) -> Option<&str> {
        match self {
            Fehler::Parser { meldung, .. } => Some(meldung),
            Fehler::ParserGrenze { was } => Some(was),
            _ => None,
        }
    }

    /// Bildet einen Ein-/Ausgabefehler auf einen Nutzersatz ab.
    ///
    /// Der technische Text bleibt im Protokoll; der Nutzer bekommt einen Satz,
    /// der sagt, was geschehen ist **und** was er tun kann (SM-NFR-006,
    /// DES-STM-001 Abschnitt 10: „Grund im Klartext").
    pub fn aus_ea(e: std::io::Error, was: &str) -> Self {
        use std::io::ErrorKind::*;
        log::debug!("Ein-/Ausgabefehler bei {was}: {e}");
        let satz = match e.kind() {
            NotFound => {
                "Die Datei ist nicht mehr vorhanden. Möglicherweise wurde sie \
                 verschoben, oder der Wechseldatenträger ist nicht verbunden."
            }
            // Anredefreie Sachsprache, durchgehend (CLAUDE.md Abschnitt 1,
            // SM-SET-006). Zuvor siezten zwei der drei Sätze und der dritte
            // nicht — innerhalb **einer** Fallunterscheidung.
            PermissionDenied => {
                "Die Zugriffsrechte der Datei lassen kein Lesen zu. \
                                 Sie lassen sich im Dateiverwalter des Systems ändern."
            }
            _ => {
                "Die Datei ist zurzeit nicht lesbar. Möglicherweise ist der \
                 Wechseldatenträger nicht verbunden."
            }
        };
        Fehler::Ea(satz.to_string())
    }
}

/// Ergebnistyp der Kernschicht.
pub type Ergebnis<T> = Result<T, Fehler>;

/// Ein unterstütztes Stickformat (SM-FMT-001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Format {
    Pes,
    Dst,
    Jef,
    Vp3,
    Exp,
    Xxx,
}

impl Format {
    /// Erkennt das Format an der Dateiendung, ohne Rücksicht auf Schreibung.
    pub fn aus_endung(endung: &str) -> Option<Self> {
        match endung.to_ascii_lowercase().as_str() {
            "pes" => Some(Self::Pes),
            "dst" => Some(Self::Dst),
            "jef" => Some(Self::Jef),
            "vp3" => Some(Self::Vp3),
            "exp" => Some(Self::Exp),
            "xxx" => Some(Self::Xxx),
            _ => None,
        }
    }

    /// Kurzzeichen für die Formatmarke auf der Kachel (SM-DES-007).
    pub fn marke(&self) -> &'static str {
        match self {
            Self::Pes => "PES",
            Self::Dst => "DST",
            Self::Jef => "JEF",
            Self::Vp3 => "VP3",
            Self::Exp => "EXP",
            Self::Xxx => "XXX",
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.marke())
    }
}

/// Eine Garnfarbe aus der Datei.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Garnfarbe {
    /// Farbwert als sechsstelliger Hexwert ohne Doppelkreuz.
    pub hex: String,
    pub name: Option<String>,
    pub marke: Option<String>,
    pub markenschluessel: Option<String>,
}

/// Kennwerte einer eingelesenen Stickdatei.
#[derive(Debug, Clone, Default)]
pub struct Kennwerte {
    pub format_version: Option<String>,
    pub breite_mm: Option<f64>,
    pub hoehe_mm: Option<f64>,
    pub stichzahl: Option<i64>,
    pub farbzahl: Option<i64>,
    pub farben: Vec<Garnfarbe>,
    pub entwurfsname: Option<String>,
    pub spruenge: Option<i64>,
    pub schnitte: Option<i64>,
    pub rahmen_breite_mm: Option<f64>,
    pub rahmen_hoehe_mm: Option<f64>,
}

/// Ein Stichabschnitt einer Farblage — Grundlage der Vorschau (SM-PRV-001).
#[derive(Debug, Clone)]
pub struct Stichabschnitt {
    pub farbindex: usize,
    pub farbe_hex: Option<String>,
    /// Stichpunkte in Millimetern.
    pub punkte: Vec<(f64, f64)>,
}

/// Herkunft eines Wertes (SM-KIA-008, SM-DES-009).
///
/// Maschinell erzeugte Werte bleiben bis zur Bestätigung als solche
/// gekennzeichnet — über Farbe **und** Text, nie über Farbe allein.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Herkunft {
    /// Aus der Datei gelesen.
    Datei,
    /// Von einer Person eingetragen.
    Person,
    /// Maschinell erzeugt, noch nicht bestätigt.
    Maschinell,
    /// Maschinell erzeugt und bestätigt.
    MaschinellBestaetigt,
}

impl Herkunft {
    /// Textliche Kennzeichnung — das zweite Merkmal neben der Farbe.
    pub fn kennzeichnung(&self) -> Option<&'static str> {
        match self {
            Self::Maschinell => Some("maschinell erzeugt"),
            _ => None,
        }
    }
}

/// Arbeitsname des Ablageverzeichnisses.
///
/// Der endgültige Produktname und die Anwendungskennung stehen unter **OP-06**.
/// Eine spätere Änderung betrifft genau diese Zeile.
pub const ANWENDUNGSNAME: &str = "StitchManager";

/// Der dauerhafte Ablageort der Anwendung nach Plattformbrauch.
///
/// Hier liegen Datenhaltung und Vorschauen. Bewusst ohne zusätzliche
/// Fremdkiste: Die drei Regeln sind kurz, und jede Abhängigkeit ist nach
/// `CLAUDE.md` Abschnitt 13 eine prüfpflichtige Entscheidung.
///
/// `SM_DATENABLAGE` übersteuert den Ort — für Prüfläufe und für die
/// Unterstützung, wenn der Regelort nicht beschreibbar ist. Ohne diese
/// Übersteuerung schriebe jeder Prüflauf in den Bestand der Nutzerin.
pub fn anwendungsablage() -> std::path::PathBuf {
    use std::path::PathBuf;

    if let Some(ort) = std::env::var_os("SM_DATENABLAGE") {
        if !ort.is_empty() {
            return PathBuf::from(ort);
        }
    }

    let heimat = || std::env::var_os("HOME").map(PathBuf::from);

    if cfg!(target_os = "macos") {
        if let Some(h) = heimat() {
            return h
                .join("Library")
                .join("Application Support")
                .join(ANWENDUNGSNAME);
        }
    } else if cfg!(target_os = "windows") {
        if let Some(ort) = std::env::var_os("LOCALAPPDATA") {
            return PathBuf::from(ort).join(ANWENDUNGSNAME);
        }
    } else if let Some(ort) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(ort).join(ANWENDUNGSNAME);
    } else if let Some(h) = heimat() {
        return h.join(".local").join("share").join(ANWENDUNGSNAME);
    }

    // Letzter Ausweg. Er ist nicht dauerhaft und deshalb ausdrücklich der
    // Ausnahmefall, nicht der Regelweg.
    std::env::temp_dir().join(ANWENDUNGSNAME)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SM-NFR-006: Fehlertexte erreichen den Nutzer und sind verständlich ---

    #[test]
    fn fehlertexte_tragen_ihre_nutzlast() {
        // Zuvor verwarf `Display` die Nutzlast und zeigte nur „Datenbankfehler".
        let f = Fehler::Datenbank("Die Datenhaltung antwortet nicht.".into());
        assert_eq!(f.to_string(), "Die Datenhaltung antwortet nicht.");
        let f = Fehler::Intern("Etwas ist schiefgegangen.".into());
        assert_eq!(f.to_string(), "Etwas ist schiefgegangen.");
    }

    #[test]
    fn parsermeldung_zeigt_kein_technisches_detail() {
        let f = Fehler::Parser {
            format: "PES".into(),
            meldung: "header too short at offset 8".into(),
        };
        let text = f.to_string();
        assert!(text.contains("PES"), "das Format fehlt: {text}");
        assert!(
            !text.contains("offset"),
            "die technische Meldung erreicht die Anzeige: {text}"
        );
        // Für das Protokoll bleibt sie abrufbar.
        assert_eq!(f.technisch(), Some("header too short at offset 8"));
    }

    #[test]
    fn ea_fehler_sind_deutsch_und_anredefrei() {
        for art in [
            std::io::ErrorKind::NotFound,
            std::io::ErrorKind::PermissionDenied,
            std::io::ErrorKind::Other,
        ] {
            let f = Fehler::aus_ea(std::io::Error::new(art, "raw os message"), "Prüffall");
            let t = f.to_string();
            assert!(!t.contains("raw os message"), "Rohtext durchgereicht: {t}");
            assert!(!t.contains("Prüfen Sie"), "Anrede statt Sachsprache: {t}");
            assert!(t.ends_with('.'), "kein ganzer Satz: {t}");
            // Doppelte Satzaussagen wie „… ist … verbunden ist." fangen.
            assert!(
                !t.contains(" ist ist ") && !t.ends_with("verbunden ist."),
                "verstümmelter Satz: {t}"
            );
            assert!(t.split_whitespace().count() >= 5, "zu knapp: {t}");
        }
    }

    // --- SM-LIB-010 / Formate ---

    #[test]
    fn formate_werden_ohne_ruecksicht_auf_schreibung_erkannt() {
        assert_eq!(Format::aus_endung("PES"), Some(Format::Pes));
        assert_eq!(Format::aus_endung("pes"), Some(Format::Pes));
        assert_eq!(Format::aus_endung("PeS"), Some(Format::Pes));
        assert_eq!(Format::aus_endung("txt"), None);
        assert_eq!(Format::aus_endung(""), None);
    }

    #[test]
    fn jedes_format_traegt_eine_marke() {
        for f in [
            Format::Pes,
            Format::Dst,
            Format::Jef,
            Format::Vp3,
            Format::Exp,
            Format::Xxx,
        ] {
            assert_eq!(f.marke().len(), 3, "{f}: Marke nicht dreistellig");
            assert_eq!(
                Format::aus_endung(f.marke()),
                Some(f),
                "{f}: nicht rückführbar"
            );
        }
    }

    // --- SM-KIA-008 / SM-DES-009 ---

    #[test]
    fn nur_unbestaetigt_maschinelle_werte_tragen_eine_kennzeichnung() {
        assert!(Herkunft::Maschinell.kennzeichnung().is_some());
        assert!(Herkunft::Datei.kennzeichnung().is_none());
        assert!(Herkunft::Person.kennzeichnung().is_none());
        assert!(Herkunft::MaschinellBestaetigt.kennzeichnung().is_none());
    }

    #[test]
    fn die_anwendungsablage_traegt_den_anwendungsnamen() {
        let ort = anwendungsablage();
        assert!(
            ort.to_string_lossy().contains(ANWENDUNGSNAME),
            "Ablage ohne Anwendungsnamen: {}",
            ort.display()
        );
    }
}
