//! Dauerhafter Zwischenspeicher der Vorschaubilder und dessen Verwerfung.
//!
//! Trägt SM-PRV-002 (dauerhaft) und SM-PRV-003 (verwerfen, sobald sich die
//! Quelldatei ändert). Der Modulschnitt weist `kern/render` beides zu:
//! „Vorschau aus Stichdaten, Zwischenspeicher und dessen Verwerfung".
//!
//! **Der Schlüssel trägt die Prüfung.** Größe und Änderungszeit der Quelldatei
//! stehen im Dateinamen. Die Gültigkeitsprüfung ist damit ein `stat` der Quelle
//! und ein `exists` — kein Nebenaktenlesen und **kein Inhaltshash**. Der
//! Inhaltshash bleibt dem Importlauf vorbehalten (SM-IMP-005): Im Lesepfad
//! müsste er je angezeigter Kachel die vollständige Quelldatei lesen.
//!
//! **Keine Obergrenze, keine Verdrängung.** Ob SM-PRV-002 beides deckt, ist
//! offen (OP-21). Verworfen wird ausschließlich, was nachweislich überholt ist;
//! das folgt aus SM-PRV-003 und nicht aus einer Größenannahme.

use kern_typen::{Ergebnis, Fehler};
use std::path::{Path, PathBuf};

/// Diskrete Auflösungsstufen.
///
/// AP-09: Der Zwischenspeicher hält je **diskret gestufter** Auflösung — nicht
/// je beliebiger Pixelgröße. Andernfalls entstünde bei jeder Fensterbreite ein
/// neuer Stand, und der Speicher wüchse ohne Nutzen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Stufe {
    Klein,
    Mittel,
    Gross,
    Sehrgross,
}

impl Stufe {
    /// Kantenlänge der Stufe in Bildpunkten.
    pub fn kantenlaenge(&self) -> u32 {
        match self {
            Self::Klein => 160,
            Self::Mittel => 320,
            Self::Gross => 640,
            Self::Sehrgross => 1280,
        }
    }

    /// Die kleinste Stufe, die die gewünschte Breite noch trägt.
    ///
    /// Aufgerundet, nie abgerundet: Ein hochskaliertes Bild ist unscharf, und
    /// DES-STM-001 Abschnitt 11 verlangt Neuzeichnen statt Hochskalieren.
    pub fn fuer_breite(px: u32) -> Self {
        for s in [Self::Klein, Self::Mittel, Self::Gross] {
            if px <= s.kantenlaenge() {
                return s;
            }
        }
        Self::Sehrgross
    }

    fn kuerzel(&self) -> &'static str {
        match self {
            Self::Klein => "k",
            Self::Mittel => "m",
            Self::Gross => "g",
            Self::Sehrgross => "s",
        }
    }
}

/// Die Kennzeichen der Quelldatei, gegen die geprüft wird.
///
/// Ausschließlich Größe und Änderungszeit — die beiden Größen, die AP-09
/// vorschreibt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Quellstand {
    groesse: u64,
    geaendert: i64,
}

impl Quellstand {
    fn lesen(quelle: &Path) -> Option<Self> {
        let m = std::fs::metadata(quelle).ok()?;
        let geaendert = m
            .modified()
            .ok()
            .and_then(|z| z.duration_since(std::time::UNIX_EPOCH).ok())
            // Vor 1970 datierte Dateien gibt es; sie dürfen nicht alle
            // denselben Stand bekommen.
            .map(|d| d.as_secs() as i64)
            .unwrap_or(-1);
        Some(Self {
            groesse: m.len(),
            geaendert,
        })
    }
}

/// Der Zwischenspeicher.
pub struct Zwischenspeicher {
    wurzel: PathBuf,
}

impl Zwischenspeicher {
    /// Öffnet den Zwischenspeicher an einem Ort und legt ihn bei Bedarf an.
    pub fn an(wurzel: impl Into<PathBuf>) -> Ergebnis<Self> {
        let wurzel = wurzel.into();
        std::fs::create_dir_all(&wurzel)
            .map_err(|e| Fehler::aus_ea(e, "Anlegen des Vorschauspeichers"))?;
        Ok(Self { wurzel })
    }

    /// Öffnet den Zwischenspeicher am dauerhaften Ort des Betriebssystems.
    pub fn am_standardort() -> Ergebnis<Self> {
        Self::an(standardort())
    }

    /// Der Ablageort.
    pub fn ort(&self) -> &Path {
        &self.wurzel
    }

    /// Liefert eine **gültige** Vorschau oder nichts (SM-PRV-003).
    ///
    /// Gültig heißt: Größe und Änderungszeit der Quelldatei stimmen mit dem
    /// überein, was beim Ablegen galt. Ändert sich eines von beidem, ändert
    /// sich der Schlüssel und dieser Aufruf liefert nichts — der überholte
    /// Stand wird nicht mehr gefunden.
    ///
    /// **Läuft nicht im Zeichenpfad.** Der Aufruf kostet ein `stat` und ein
    /// `exists`; beides gehört in den Arbeitsfaden (SM-NFR-002, AP-12).
    pub fn holen(&self, kennung: &str, quelle: &Path, stufe: Stufe) -> Option<PathBuf> {
        let stand = Quellstand::lesen(quelle)?;
        let ziel = self.ablage(kennung, stand, stufe);
        ziel.is_file().then_some(ziel)
    }

    /// Legt eine Vorschau ab und **verwirft überholte Stände derselben
    /// Kennung** (SM-PRV-003).
    pub fn ablegen(
        &self,
        kennung: &str,
        quelle: &Path,
        stufe: Stufe,
        png: &[u8],
    ) -> Ergebnis<PathBuf> {
        let stand = Quellstand::lesen(quelle).ok_or_else(|| {
            Fehler::NichtGefunden("Die Quelldatei ist nicht mehr vorhanden.".into())
        })?;

        let ziel = self.ablage(kennung, stand, stufe);
        if let Some(fach) = ziel.parent() {
            std::fs::create_dir_all(fach)
                .map_err(|e| Fehler::aus_ea(e, "Anlegen eines Speicherfachs"))?;
        }

        // Erst schreiben, dann Überholtes entfernen: Bräche der Vorgang
        // dazwischen ab, stünde sonst weder das alte noch das neue Bild da.
        std::fs::write(&ziel, png).map_err(|e| Fehler::aus_ea(e, "Ablegen einer Vorschau"))?;
        self.ueberholte_entfernen(kennung, stand);

        Ok(ziel)
    }

    /// Verwirft **alle** Stände einer Kennung, gültige eingeschlossen.
    ///
    /// Für den Fall, dass ein Eintrag den Bestand verlässt.
    pub fn verwerfen(&self, kennung: &str) -> usize {
        self.staende(kennung)
            .into_iter()
            .filter(|p| std::fs::remove_file(p).is_ok())
            .count()
    }

    /// Entfernt die Stände dieser Kennung, die zu einem **überholten
    /// Quellstand** gehören.
    ///
    /// Die Auflösungsstufen desselben Quellstands bleiben nebeneinander
    /// bestehen — AP-09 verlangt ausdrücklich, dass der Zwischenspeicher je
    /// diskret gestufter Auflösung hält. Verworfen wird, was zu einer
    /// **anderen** Fassung der Quelldatei gehört.
    fn ueberholte_entfernen(&self, kennung: &str, gueltig: Quellstand) -> usize {
        let behalten = format!("{}_{}_", gueltig.groesse, gueltig.geaendert);
        self.staende(kennung)
            .into_iter()
            .filter(|p| {
                !p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with(&behalten))
            })
            .filter(|p| std::fs::remove_file(p).is_ok())
            .count()
    }

    /// Alle abgelegten Stände einer Kennung.
    ///
    /// Liest nur das Verzeichnis **dieser** Kennung — höchstens vier Stufen mal
    /// wenige Stände, unabhängig von der Bestandsgröße.
    fn staende(&self, kennung: &str) -> Vec<PathBuf> {
        let Ok(eintraege) = std::fs::read_dir(self.kennungsverzeichnis(kennung)) else {
            return Vec::new();
        };
        eintraege
            .flatten()
            .map(|e| e.path())
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.ends_with(".png"))
            })
            .collect()
    }

    /// Ablageort eines Stands.
    ///
    /// Je Kennung ein **eigenes Verzeichnis**. Lägen alle Stände flach im Fach,
    /// müsste `staende` das ganze Fach durchlaufen: Bei 100.000 Einträgen und
    /// 256 Fächern sind das rund 1.560 Verzeichniseinträge je Aufruf, und
    /// `ablegen` ruft es jedes Mal — der Erstlauf der Vorschauerzeugung läse
    /// dann rund 1,6 × 10⁸ Verzeichniseinträge. So liest er höchstens die
    /// wenigen Stände **einer** Kennung.
    fn kennungsverzeichnis(&self, kennung: &str) -> PathBuf {
        self.wurzel.join(fachname(kennung)).join(kennung)
    }

    fn ablage(&self, kennung: &str, stand: Quellstand, stufe: Stufe) -> PathBuf {
        self.kennungsverzeichnis(kennung).join(format!(
            "{}_{}_{}.png",
            stand.groesse,
            stand.geaendert,
            stufe.kuerzel()
        ))
    }
}

/// Streut die Kennungen über 256 Fächer.
fn fachname(kennung: &str) -> String {
    let mut zeichen = kennung.chars().filter(|z| z.is_ascii_alphanumeric());
    match (zeichen.next(), zeichen.next()) {
        (Some(a), Some(b)) => format!("{a}{b}"),
        (Some(a), None) => format!("{a}_"),
        _ => "__".to_string(),
    }
}

/// Der dauerhafte Ablageort nach Plattformbrauch (SM-PRV-002).
///
/// Bewusst ohne zusätzliche Fremdkiste: Die drei Regeln sind kurz, und jede
/// Abhängigkeit ist nach `CLAUDE.md` Abschnitt 13 eine prüfpflichtige
/// Entscheidung.
pub fn standardort() -> PathBuf {
    // Ausdrückliche Übersteuerung — für Prüfläufe und für die Unterstützung,
    // wenn der Regelort nicht beschreibbar ist. Ein Prüflauf darf nicht in den
    // dauerhaften Speicher der Nutzerin schreiben.
    if let Some(ort) = std::env::var_os("SM_VORSCHAU_ABLAGE") {
        if !ort.is_empty() {
            return PathBuf::from(ort);
        }
    }
    kern_typen::anwendungsablage().join("vorschau")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn quelle_anlegen(verzeichnis: &Path, name: &str, inhalt: &[u8]) -> PathBuf {
        let p = verzeichnis.join(name);
        let mut d = std::fs::File::create(&p).unwrap();
        d.write_all(inhalt).unwrap();
        d.sync_all().unwrap();
        p
    }

    /// Setzt die Änderungszeit einer Datei um Sekunden vor.
    ///
    /// Warten wäre der Prüfung nicht würdig: Sie soll die Wirkung der
    /// Änderungszeit belegen, nicht die Auflösung der Uhr abwarten.
    fn aendere_zeit(pfad: &Path, sekunden: i64) {
        let m = std::fs::metadata(pfad).unwrap();
        let neu = m.modified().unwrap() + std::time::Duration::from_secs(sekunden as u64);
        let datei = std::fs::OpenOptions::new().write(true).open(pfad).unwrap();
        datei.set_modified(neu).unwrap();
        datei.sync_all().unwrap();
    }

    struct Aufbau {
        _tmp: tempfile::TempDir,
        quellen: PathBuf,
        speicher: Zwischenspeicher,
    }

    fn aufbau() -> Aufbau {
        let tmp = tempfile::tempdir().unwrap();
        let quellen = tmp.path().join("quellen");
        std::fs::create_dir_all(&quellen).unwrap();
        let speicher = Zwischenspeicher::an(tmp.path().join("speicher")).unwrap();
        Aufbau {
            _tmp: tmp,
            quellen,
            speicher,
        }
    }

    // --- SM-PRV-002: dauerhaft ---

    #[test]
    fn abgelegtes_wird_wiedergefunden() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");

        let ziel = a
            .speicher
            .ablegen("u1", &q, Stufe::Mittel, b"PNG-Daten")
            .unwrap();
        assert!(ziel.is_file());

        let gefunden = a.speicher.holen("u1", &q, Stufe::Mittel).unwrap();
        assert_eq!(gefunden, ziel);
        assert_eq!(std::fs::read(&gefunden).unwrap(), b"PNG-Daten");
    }

    #[test]
    fn ein_neuer_speicher_am_selben_ort_findet_den_stand() {
        // Das ist die eigentliche Zusage aus SM-PRV-002: Der Stand übersteht
        // das Programmende, nicht nur die Lebensdauer eines Objekts.
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();

        let ort = a.speicher.ort().to_path_buf();
        drop(a.speicher);

        let wieder = Zwischenspeicher::an(&ort).unwrap();
        assert!(
            wieder.holen("u1", &q, Stufe::Mittel).is_some(),
            "der Stand überlebte das Neuöffnen nicht"
        );
    }

    #[test]
    fn standardort_liegt_nicht_im_temporaerverzeichnis() {
        // Ein Ablageort, den das Betriebssystem leert, ist nicht dauerhaft.
        if std::env::var_os("HOME").is_some() || std::env::var_os("LOCALAPPDATA").is_some() {
            let ort = standardort();
            assert!(
                !ort.starts_with(std::env::temp_dir()),
                "der Standardort liegt im Temporärverzeichnis: {}",
                ort.display()
            );
            assert!(ort.ends_with("vorschau"));
        }
    }

    // --- SM-PRV-003: Verwerfung ---

    #[test]
    fn geaenderte_groesse_verwirft_den_stand() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();
        assert!(a.speicher.holen("u1", &q, Stufe::Mittel).is_some());

        quelle_anlegen(&a.quellen, "muster.dst", b"abcdefghij");

        assert!(
            a.speicher.holen("u1", &q, Stufe::Mittel).is_none(),
            "nach geänderter Größe wurde der alte Stand weiter geliefert"
        );
    }

    #[test]
    fn geaenderte_zeit_bei_gleicher_groesse_verwirft_den_stand() {
        // Der schwierigere Fall: gleiche Länge, anderer Inhalt. Ohne die
        // Änderungszeit bliebe er unbemerkt.
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();
        assert!(a.speicher.holen("u1", &q, Stufe::Mittel).is_some());

        quelle_anlegen(&a.quellen, "muster.dst", b"xyz");
        aendere_zeit(&q, 120);

        assert!(
            a.speicher.holen("u1", &q, Stufe::Mittel).is_none(),
            "nach geänderter Änderungszeit wurde der alte Stand weiter geliefert"
        );
    }

    #[test]
    fn ueberholte_staende_werden_entfernt_statt_angehaeuft() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");

        for i in 0..5 {
            a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();
            // Jedes Mal ein neuer Quellstand.
            aendere_zeit(&q, 60 * (i + 1));
        }
        a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();

        assert_eq!(
            a.speicher.staende("u1").len(),
            1,
            "überholte Stände wurden angehäuft"
        );
    }

    #[test]
    fn geloeschte_quelle_liefert_keinen_treffer() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        a.speicher.ablegen("u1", &q, Stufe::Mittel, b"PNG").unwrap();

        std::fs::remove_file(&q).unwrap();
        assert!(a.speicher.holen("u1", &q, Stufe::Mittel).is_none());
    }

    #[test]
    fn ablegen_ohne_quelle_meldet_statt_zu_schreiben() {
        let a = aufbau();
        let fehlt = a.quellen.join("gibtsnicht.dst");
        assert!(a
            .speicher
            .ablegen("u1", &fehlt, Stufe::Mittel, b"PNG")
            .is_err());
    }

    #[test]
    fn verwerfen_raeumt_alle_staende_einer_kennung() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        for stufe in [Stufe::Klein, Stufe::Mittel, Stufe::Gross] {
            a.speicher.ablegen("u1", &q, stufe, b"PNG").unwrap();
        }
        assert_eq!(a.speicher.staende("u1").len(), 3);
        assert_eq!(a.speicher.verwerfen("u1"), 3);
        assert!(a.speicher.staende("u1").is_empty());
    }

    // --- Stufen ---

    #[test]
    fn stufen_bestehen_nebeneinander() {
        let a = aufbau();
        let q = quelle_anlegen(&a.quellen, "muster.dst", b"abc");
        a.speicher.ablegen("u1", &q, Stufe::Klein, b"K").unwrap();
        a.speicher.ablegen("u1", &q, Stufe::Gross, b"G").unwrap();

        assert_eq!(
            std::fs::read(a.speicher.holen("u1", &q, Stufe::Klein).unwrap()).unwrap(),
            b"K"
        );
        assert_eq!(
            std::fs::read(a.speicher.holen("u1", &q, Stufe::Gross).unwrap()).unwrap(),
            b"G"
        );
    }

    #[test]
    fn stufe_wird_aufgerundet_nie_abgerundet() {
        // Hochskalieren ist unzulässig (DES-STM-001 Abschnitt 11).
        assert_eq!(Stufe::fuer_breite(1), Stufe::Klein);
        assert_eq!(Stufe::fuer_breite(160), Stufe::Klein);
        assert_eq!(Stufe::fuer_breite(161), Stufe::Mittel);
        assert_eq!(Stufe::fuer_breite(320), Stufe::Mittel);
        assert_eq!(Stufe::fuer_breite(321), Stufe::Gross);
        assert_eq!(Stufe::fuer_breite(9999), Stufe::Sehrgross);
    }

    // --- OP-21: keine Obergrenze vorwegnehmen ---

    #[test]
    fn der_speicher_kennt_keine_obergrenze() {
        // OP-21 ist offen. Solange er offen ist, verdrängt der Speicher
        // nichts, was noch gültig ist.
        let a = aufbau();
        for i in 0..300 {
            let q = quelle_anlegen(&a.quellen, &format!("m{i}.dst"), b"abc");
            a.speicher
                .ablegen(&format!("u{i:04}"), &q, Stufe::Mittel, b"PNG")
                .unwrap();
        }
        for i in 0..300 {
            let q = a.quellen.join(format!("m{i}.dst"));
            assert!(
                a.speicher
                    .holen(&format!("u{i:04}"), &q, Stufe::Mittel)
                    .is_some(),
                "Stand {i} wurde verdrängt — das nähme OP-21 vorweg"
            );
        }
    }

    #[test]
    fn kennungen_werden_gestreut() {
        assert_eq!(fachname("abcdef"), "ab");
        assert_eq!(fachname("a"), "a_");
        assert_eq!(fachname(""), "__");
        // Ein Trennerzeichen darf nicht in den Fachnamen geraten.
        assert_eq!(fachname("../x"), "x_");
    }
}
