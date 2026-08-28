//! Pfadprüfung und Eingrenzung.
//!
//! Trägt SM-SEC-001 (Prüfung gegen Verzeichniswechsel), SM-SEC-002 (Eingrenzung
//! nach Kanonisierung **beider** Seiten, einschließlich Symlinks und
//! Schreibungsfaltung), SM-SEC-003 (Prüfung der Wurzel beim Setzen, Ablehnung
//! von Systemwurzeln) und SM-NFR-010 (Unicode- und lange Pfade).
//!
//! Schnittregel 5 aus IMP-STM-001 Abschnitt 3: **jeder** Schreibvorgang läuft
//! durch dieses Modul. Ein Modul, das selbst einen Pfad zusammensetzt, ist ein
//! Befund.

#![forbid(unsafe_code)]

use kern_typen::{Ergebnis, Fehler};
use std::path::{Component, Path, PathBuf};

/// Verzeichnisse, die nie als Wurzel zulässig sind (SM-SEC-003).
///
/// Die Liste ist bewusst plattformübergreifend: Ein Bestand, der unter macOS
/// angelegt und unter Linux geöffnet wird, darf nicht je nach Plattform eine
/// andere Antwort bekommen.
const SYSTEMWURZELN: &[&str] = &[
    "/",
    "/bin",
    "/boot",
    "/dev",
    "/etc",
    "/lib",
    "/lib64",
    "/proc",
    "/root",
    "/sbin",
    "/sys",
    "/usr",
    "/var",
    "/System",
    "/Library",
    "/Applications",
    "/Volumes",
    "/private",
    "/private/etc",
    "/private/var",
    "/private/tmp",
    "/opt",
    "C:\\",
    "C:\\Windows",
    "C:\\Program Files",
    "C:\\Program Files (x86)",
];

/// Prüft, ob Pfadvergleiche auf dieser Plattform die Schreibung falten.
///
/// macOS und Windows führen Dateisysteme im Regelfall ohne Rücksicht auf
/// Groß- und Kleinschreibung. Wird das übergangen, lässt sich die Eingrenzung
/// über eine abweichend geschriebene Wurzel umgehen (SM-SEC-002).
const FALTET_SCHREIBUNG: bool = cfg!(any(target_os = "macos", target_os = "windows"));

/// Eine geprüfte Wurzel. Ihr Vorhandensein ist der Nachweis, dass die Prüfung
/// nach SM-SEC-003 stattgefunden hat — ein ungeprüfter Pfad kann diesen Typ
/// nicht annehmen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Wurzel {
    kanonisch: PathBuf,
}

impl Wurzel {
    /// Setzt eine Wurzel und prüft sie (SM-SEC-003).
    ///
    /// Abgelehnt werden: nicht vorhandene Pfade, Dateien, Systemwurzeln und —
    /// nach Kanonisierung — jeder Pfad, der auf eine Systemwurzel zeigt. Die
    /// Kanonisierung vor der Prüfung ist wesentlich: Andernfalls ließe sich
    /// über einen Symlink oder über `/usr/../usr` an der Liste vorbeizielen.
    pub fn neu(pfad: impl AsRef<Path>) -> Ergebnis<Self> {
        let pfad = pfad.as_ref();

        if !pfad.exists() {
            return Err(Fehler::PfadUngueltig(
                "Das Verzeichnis gibt es nicht.".into(),
            ));
        }
        if !pfad.is_dir() {
            return Err(Fehler::PfadUngueltig(
                "Der Pfad zeigt auf eine Datei, nicht auf ein Verzeichnis.".into(),
            ));
        }

        let kanonisch = pfad
            .canonicalize()
            .map_err(|_| Fehler::PfadUngueltig("Das Verzeichnis ist nicht lesbar.".into()))?;

        // Beide Formen prüfen: `/etc` kanonisiert unter macOS zu `/private/etc`,
        // `/usr/..` zu `/`. Je nachdem trägt nur eine der beiden Formen den
        // Namen aus der Liste.
        if ist_systemwurzel(&kanonisch) || ist_systemwurzel(&normalisiere(&absolutiere(pfad))) {
            return Err(Fehler::PfadUngueltig(
                "Systemverzeichnisse sind als Bibliothekswurzel nicht zulässig.".into(),
            ));
        }

        Ok(Self { kanonisch })
    }

    /// Der kanonische Pfad der Wurzel.
    pub fn pfad(&self) -> &Path {
        &self.kanonisch
    }

    /// Prüft einen Schreibpfad und gibt ihn als [`Schreibziel`] zurück.
    ///
    /// Der einzige Weg, ein `Schreibziel` zu bekommen — und damit der einzige
    /// Weg, in dieser Anwendung eine Datei zu schreiben (Schnittregel 5).
    pub fn schreibziel(&self, kandidat: impl AsRef<Path>) -> Ergebnis<Schreibziel> {
        self.pruefe(kandidat).map(Schreibziel)
    }

    /// Prüft einen Kandidaten und gibt ihn kanonisiert zurück (SM-SEC-001/002).
    ///
    /// Der Kandidat darf noch nicht vorhanden sein — für Schreibziele ist das
    /// der Regelfall. Kanonisiert wird dann der tiefste vorhandene Vorfahre;
    /// der noch nicht vorhandene Rest kann keinen Symlink enthalten, weil es
    /// ihn nicht gibt.
    pub fn pruefe(&self, kandidat: impl AsRef<Path>) -> Ergebnis<PathBuf> {
        let kandidat = kandidat.as_ref();

        // Relative Pfade gelten gegen die Wurzel, nie gegen das
        // Arbeitsverzeichnis des Prozesses.
        let absolut = if kandidat.is_absolute() {
            kandidat.to_path_buf()
        } else {
            self.kanonisch.join(kandidat)
        };

        // Ein Pfad mit `..` wird nicht abgewiesen, sondern aufgelöst und
        // anschließend geprüft. Abweisen allein reichte nicht: `a/../../b`
        // und ein Symlink führen zum selben Ziel, nur eines davon trägt `..`.
        let aufgeloest = kanonisiere_teilweise(&absolut)?;

        if !enthaelt(&self.kanonisch, &aufgeloest) {
            return Err(Fehler::PfadAusserhalb);
        }

        Ok(aufgeloest)
    }
}

/// Kanonisiert so weit wie möglich und hängt den nicht vorhandenen Rest an.
fn kanonisiere_teilweise(pfad: &Path) -> Ergebnis<PathBuf> {
    // Zuerst lexikalisch normalisieren, damit `..` nicht über die Wurzel
    // hinausläuft, bevor das Dateisystem befragt wird.
    let normalisiert = normalisiere(pfad);

    let mut vorhanden = normalisiert.as_path();
    let mut rest: Vec<&std::ffi::OsStr> = Vec::new();

    loop {
        // `symlink_metadata` statt `exists`: `exists()` **folgt** dem Symlink
        // und meldet `false`, wenn dessen Ziel (noch) nicht da ist. Ein solcher
        // Verweis wanderte dadurch in `rest`, wurde nie aufgelöst, und die
        // Eingrenzung bejahte ihn — obwohl er auf `../../../.ssh/authorized_keys`
        // zeigen kann. `symlink_metadata` sieht den Verweis selbst; die
        // anschließende Kanonisierung scheitert an einem toten Ziel und der
        // Pfad wird abgewiesen (SM-SEC-002).
        if std::fs::symlink_metadata(vorhanden).is_ok() {
            break;
        }
        match (vorhanden.parent(), vorhanden.file_name()) {
            (Some(eltern), Some(name)) => {
                rest.push(name);
                vorhanden = eltern;
            }
            // Es gibt keinen vorhandenen Vorfahren mehr.
            _ => {
                return Err(Fehler::PfadUngueltig(
                    "Der Pfad ist nicht auflösbar.".into(),
                ))
            }
        }
    }

    let mut ergebnis = vorhanden
        .canonicalize()
        .map_err(|_| Fehler::PfadUngueltig("Der Pfad ist nicht auflösbar.".into()))?;

    for name in rest.iter().rev() {
        ergebnis.push(name);
    }

    Ok(ergebnis)
}

/// Macht einen Pfad absolut, ohne das Dateisystem zu befragen.
fn absolutiere(pfad: &Path) -> PathBuf {
    if pfad.is_absolute() {
        pfad.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("/"))
            .join(pfad)
    }
}

/// Löst `.` und `..` rein lexikalisch auf, ohne das Dateisystem zu befragen.
fn normalisiere(pfad: &Path) -> PathBuf {
    let mut aus = PathBuf::new();
    for teil in pfad.components() {
        match teil {
            Component::CurDir => {}
            Component::ParentDir => {
                // Über den Wurzelanteil hinaus wird nicht gestiegen.
                if aus
                    .components()
                    .next_back()
                    .is_some_and(|c| matches!(c, Component::Normal(_)))
                {
                    aus.pop();
                }
            }
            andere => aus.push(andere.as_os_str()),
        }
    }
    aus
}

/// Liegt `kind` in `wurzel` oder ist es die Wurzel selbst?
///
/// Verglichen wird komponentenweise, nicht über Zeichenketten: Ein
/// Präfixvergleich hielte `/bestand-alt` fälschlich für einen Teil von
/// `/bestand`.
fn enthaelt(wurzel: &Path, kind: &Path) -> bool {
    let mut w = wurzel.components();
    let mut k = kind.components();

    loop {
        match (w.next(), k.next()) {
            (None, _) => return true,
            (Some(_), None) => return false,
            (Some(a), Some(b)) => {
                if !teil_gleich(a.as_os_str(), b.as_os_str()) {
                    return false;
                }
            }
        }
    }
}

fn teil_gleich(a: &std::ffi::OsStr, b: &std::ffi::OsStr) -> bool {
    if !FALTET_SCHREIBUNG {
        return a == b;
    }
    match (a.to_str(), b.to_str()) {
        // `to_lowercase` faltet auch außerhalb von ASCII — nötig für
        // Unicode-Pfade nach SM-NFR-010.
        (Some(a), Some(b)) => a.to_lowercase() == b.to_lowercase(),
        // Nicht als UTF-8 darstellbare Namen werden Byte für Byte verglichen.
        _ => a == b,
    }
}

fn ist_systemwurzel(pfad: &Path) -> bool {
    SYSTEMWURZELN
        .iter()
        .any(|s| teil_gleich(pfad.as_os_str(), std::ffi::OsStr::new(s)))
}

/// Ein **geprüftes** Schreibziel.
///
/// Schnittregel 5 aus IMP-STM-001 Abschnitt 3: „Jeder Schreibvorgang läuft
/// durch `kern/security`. Ein Modul, das selbst einen Pfad zusammensetzt, ist
/// ein Befund." Dieser Typ macht die Regel zur Bauvorgabe statt zur Zusage:
/// Er lässt sich außerhalb dieser Kiste nicht herstellen, und wer schreiben
/// will, braucht ihn.
#[derive(Debug, Clone)]
pub struct Schreibziel(PathBuf);

impl Schreibziel {
    /// Der geprüfte Pfad.
    pub fn pfad(&self) -> &Path {
        &self.0
    }
}

/// Bereinigt einen aus Fremddaten stammenden Dateinamen.
///
/// Namen aus Stickdateien und Archiven sind Fremddaten. Sie dürfen weder einen
/// Verzeichniswechsel auslösen noch Trenner einschleusen (SM-SEC-001,
/// SM-FMT-012). Nicht darstellbare Steuerzeichen entfallen; die Länge wird
/// begrenzt, ohne eine Mehrbytefolge zu zerschneiden (SM-NFR-010).
pub fn bereinige_dateiname(roh: &str) -> String {
    const HOECHSTLAENGE: usize = 200;

    let gefiltert: String = roh
        .chars()
        .map(|z| match z {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            z if z.is_control() => '_',
            z => z,
        })
        .collect();

    // Aufeinanderfolgende Platzhalter zusammenziehen, damit aus
    // `../../etc/passwd` nicht `_.._.._etc_passwd` wird.
    let mut zusammengezogen = String::with_capacity(gefiltert.len());
    let mut zuletzt_platzhalter = false;
    for z in gefiltert.chars() {
        if z == '_' {
            if !zuletzt_platzhalter {
                zusammengezogen.push('_');
            }
            zuletzt_platzhalter = true;
        } else {
            zusammengezogen.push(z);
            zuletzt_platzhalter = false;
        }
    }
    let beschnitten = zusammengezogen.trim_matches(|z: char| z == '.' || z == '_' || z == ' ');

    // Reservierte Gerätenamen unter Windows — ein `CON.pes` ist dort nicht
    // anlegbar und träfe die Anwendung erst beim Schreiben.
    const RESERVIERT: &[&str] = &[
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    let stamm = beschnitten.split('.').next().unwrap_or("");
    let beschnitten = if RESERVIERT
        .iter()
        .any(|r| r.eq_ignore_ascii_case(stamm) && !stamm.is_empty())
    {
        format!("_{beschnitten}")
    } else {
        beschnitten.to_string()
    };

    if beschnitten.is_empty() {
        return "unbenannt".to_string();
    }

    // An einer Zeichengrenze kürzen, nie mitten in einer Mehrbytefolge.
    if beschnitten.len() <= HOECHSTLAENGE {
        beschnitten
    } else {
        let mut ende = HOECHSTLAENGE;
        while ende > 0 && !beschnitten.is_char_boundary(ende) {
            ende -= 1;
        }
        beschnitten[..ende].trim_end().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn wurzel_anlegen() -> (tempfile::TempDir, Wurzel) {
        let tmp = tempfile::tempdir().unwrap();
        let w = Wurzel::neu(tmp.path()).unwrap();
        (tmp, w)
    }

    // --- SM-SEC-003: Prüfung der Wurzel beim Setzen ---

    #[test]
    fn systemwurzeln_werden_abgelehnt() {
        for p in ["/", "/usr", "/etc"] {
            if Path::new(p).is_dir() {
                assert!(
                    Wurzel::neu(p).is_err(),
                    "Systemwurzel {p} wurde nicht abgelehnt"
                );
            }
        }
    }

    #[test]
    fn systemwurzel_ueber_umweg_wird_abgelehnt() {
        // `/usr/..` kanonisiert zu `/` — ohne Kanonisierung vor der Prüfung
        // liefe das an der Liste vorbei.
        if Path::new("/usr").is_dir() {
            assert!(Wurzel::neu("/usr/..").is_err());
        }
    }

    #[test]
    fn datei_ist_keine_wurzel() {
        let tmp = tempfile::tempdir().unwrap();
        let datei = tmp.path().join("a.pes");
        fs::write(&datei, b"x").unwrap();
        assert!(Wurzel::neu(&datei).is_err());
    }

    #[test]
    fn fehlendes_verzeichnis_ist_keine_wurzel() {
        assert!(Wurzel::neu("/gibt/es/nicht/hoffentlich").is_err());
    }

    // --- SM-SEC-001: Verzeichniswechsel ---

    #[test]
    fn verzeichniswechsel_wird_abgewiesen() {
        let (_tmp, w) = wurzel_anlegen();
        assert!(matches!(
            w.pruefe("../geheim.pes"),
            Err(Fehler::PfadAusserhalb)
        ));
        assert!(matches!(
            w.pruefe("unter/../../geheim.pes"),
            Err(Fehler::PfadAusserhalb)
        ));
    }

    #[test]
    fn absoluter_fremdpfad_wird_abgewiesen() {
        let (_tmp, w) = wurzel_anlegen();
        assert!(matches!(
            w.pruefe("/etc/passwd"),
            Err(Fehler::PfadAusserhalb)
        ));
    }

    #[test]
    fn punkt_punkt_innerhalb_bleibt_zulaessig() {
        // `a/../b` bleibt in der Wurzel und ist deshalb kein Befund.
        let (tmp, w) = wurzel_anlegen();
        fs::create_dir(tmp.path().join("a")).unwrap();
        let ziel = w.pruefe("a/../b.pes").unwrap();
        assert_eq!(ziel, w.pfad().join("b.pes"));
    }

    // --- SM-SEC-002: Eingrenzung, Symlinks, Schreibungsfaltung ---

    #[test]
    fn noch_nicht_vorhandenes_schreibziel_ist_zulaessig() {
        let (_tmp, w) = wurzel_anlegen();
        let ziel = w.pruefe("neu/tief/datei.pes").unwrap();
        assert!(ziel.starts_with(w.pfad()));
    }

    #[test]
    fn praefixnachbar_ist_nicht_enthalten() {
        // `/x/bestand-alt` darf nicht als Teil von `/x/bestand` gelten.
        let tmp = tempfile::tempdir().unwrap();
        let drin = tmp.path().join("bestand");
        let daneben = tmp.path().join("bestand-alt");
        fs::create_dir(&drin).unwrap();
        fs::create_dir(&daneben).unwrap();
        let w = Wurzel::neu(&drin).unwrap();
        assert!(matches!(
            w.pruefe(daneben.join("a.pes")),
            Err(Fehler::PfadAusserhalb)
        ));
    }

    #[cfg(unix)]
    #[test]
    fn symlink_aus_der_wurzel_heraus_wird_abgewiesen() {
        let tmp = tempfile::tempdir().unwrap();
        let drin = tmp.path().join("drin");
        let draussen = tmp.path().join("draussen");
        fs::create_dir(&drin).unwrap();
        fs::create_dir(&draussen).unwrap();
        fs::write(draussen.join("geheim.pes"), b"x").unwrap();

        std::os::unix::fs::symlink(&draussen, drin.join("brueckte")).unwrap();

        let w = Wurzel::neu(&drin).unwrap();
        assert!(
            matches!(w.pruefe("brueckte/geheim.pes"), Err(Fehler::PfadAusserhalb)),
            "Symlink führte aus der Wurzel heraus, wurde aber angenommen"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlink_ins_leere_wird_abgewiesen() {
        // Ein Verweis auf ein nicht vorhandenes Ziel außerhalb der Wurzel:
        // `exists()` folgt ihm und meldet `false` — ohne besondere Behandlung
        // gälte der Pfad als innerhalb der Wurzel.
        let tmp = tempfile::tempdir().unwrap();
        let drin = tmp.path().join("drin");
        fs::create_dir(&drin).unwrap();

        std::os::unix::fs::symlink("../../../.ssh/authorized_keys", drin.join("muster.dst"))
            .unwrap();

        let w = Wurzel::neu(&drin).unwrap();
        assert!(
            w.pruefe("muster.dst").is_err(),
            "ein ins Leere zeigender Symlink wurde als innerhalb der Wurzel angenommen"
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlink_auf_vorhandenes_ziel_in_der_wurzel_bleibt_zulaessig() {
        let tmp = tempfile::tempdir().unwrap();
        let drin = tmp.path().join("drin");
        fs::create_dir(&drin).unwrap();
        fs::write(drin.join("echt.dst"), b"x").unwrap();
        std::os::unix::fs::symlink(drin.join("echt.dst"), drin.join("verweis.dst")).unwrap();

        let w = Wurzel::neu(&drin).unwrap();
        assert!(w.pruefe("verweis.dst").is_ok());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn abweichende_schreibung_bleibt_eingegrenzt() {
        let (tmp, w) = wurzel_anlegen();
        fs::create_dir(tmp.path().join("Muster")).unwrap();
        // Auf einem faltenden Dateisystem zeigt `muster` auf dasselbe
        // Verzeichnis; die Eingrenzung muss es als innen erkennen.
        assert!(w.pruefe("muster/a.pes").is_ok());
    }

    #[test]
    fn unicode_pfad_wird_getragen() {
        let (_tmp, w) = wurzel_anlegen();
        let ziel = w.pruefe("Größe/Bär_süß_日本.pes").unwrap();
        assert!(ziel.starts_with(w.pfad()));
        assert!(ziel.to_string_lossy().contains("Bär_süß_日本"));
    }

    // --- Bereinigung von Fremdnamen ---

    #[test]
    fn fremdname_verliert_trenner_und_wechsel() {
        assert_eq!(bereinige_dateiname("../../etc/passwd"), "etc_passwd");
        assert_eq!(bereinige_dateiname("a/b\\c.pes"), "a_b_c.pes");
        assert_eq!(bereinige_dateiname(".."), "unbenannt");
        assert_eq!(bereinige_dateiname(""), "unbenannt");
    }

    #[test]
    fn fremdname_verliert_steuerzeichen() {
        assert_eq!(bereinige_dateiname("a\u{0}b\nc"), "a_b_c");
    }

    #[test]
    fn reservierter_geraetename_wird_entschaerft() {
        assert_eq!(bereinige_dateiname("CON.pes"), "_CON.pes");
        assert_eq!(bereinige_dateiname("con"), "_con");
        // `CONTUR` ist nicht reserviert und bleibt unverändert.
        assert_eq!(bereinige_dateiname("CONTUR.pes"), "CONTUR.pes");
    }

    #[test]
    fn langer_unicode_name_wird_an_zeichengrenze_gekuerzt() {
        let lang = "ä".repeat(300);
        let aus = bereinige_dateiname(&lang);
        assert!(aus.len() <= 200);
        // Entscheidend: das Ergebnis ist gültiges UTF-8 mit ganzen Zeichen.
        assert!(aus.chars().all(|z| z == 'ä'));
    }
}
