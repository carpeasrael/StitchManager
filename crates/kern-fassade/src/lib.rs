//! Kernfassade — die **einzige** Zugriffsschicht der Oberfläche auf den Kern.
//!
//! Trägt SM-SEC-004 und SM-DTA-001 sowie die Schnittregeln 1, 3 und 4 aus
//! IMP-STM-001 Abschnitt 3.
//!
//! Die Oberfläche hängt an dieser Kiste und **nicht** an `kern-db`. Der
//! Werkstattverbund erzwingt das: `crates/ui/Cargo.toml` führt `kern-db` nicht,
//! ein direkter Zugriff findet den Namen daher nicht und der Bau bricht ab.
//! Diese Regel ist nachträglich praktisch nicht mehr einziehbar — sie entsteht
//! deshalb mit dem ersten Oberflächenmodul.

#![forbid(unsafe_code)]

use kern_db::{Ausschnitt, Bestandszeile, Datenhaltung, NeuerEintrag, Trefferzeile};
use kern_render::{Stufe, Zwischenspeicher};
use kern_security::Wurzel;
use kern_typen::{Ergebnis, Fehler, Format, Garnfarbe};
use std::path::Path;

/// Obergrenze für eine einzulesende Fremddatei.
///
/// SM-FMT-012 verlangt „kein Absturz, keine unbegrenzte Speicherbelegung, keine
/// Endlosschleife". `std::fs::read` reserviert anhand der Metadatengröße — eine
/// 40-GB-Datei in der Bibliothekswurzel beendet den Prozess über einen
/// Allokationsfehler, den kein `Result` abfängt. Die Härtung in
/// `kern_parsers::sicher` greift erst, wenn die Bytes schon im Speicher stehen.
///
/// 256 MiB ist begründet großzügig: Die größten bekannten Stickdateien liegen
/// im niedrigen zweistelligen Megabytebereich, PDF-Schnittmuster darunter.
const HOECHSTGROESSE: u64 = 256 * 1024 * 1024;

/// Liest eine Fremddatei begrenzt und nur, wenn sie eine gewöhnliche Datei ist.
///
/// Der Typtest fängt benannte Röhren ab: Ein `mkfifo x.dst` in der
/// Bibliothekswurzel ließe `fs::read` beim Öffnen unbegrenzt blockieren — der
/// Importlauf hinge ohne Fehlermeldung.
fn lies_fremddatei(pfad: &Path) -> Ergebnis<Vec<u8>> {
    let m = std::fs::metadata(pfad).map_err(|e| Fehler::aus_ea(e, "Lesen der Dateiangaben"))?;
    if !m.is_file() {
        return Err(Fehler::Eingabe(
            "Das ist keine gewöhnliche Datei und wird nicht gelesen.".into(),
        ));
    }
    if m.len() > HOECHSTGROESSE {
        return Err(Fehler::DateiZuGross {
            grenze_mib: HOECHSTGROESSE / (1024 * 1024),
        });
    }
    std::fs::read(pfad).map_err(|e| Fehler::aus_ea(e, "Lesen einer Stickdatei"))
}

pub use kern_db::{Sortierung, Suchabfrage as Abfrage};

/// Eine Kachelzeile — genau das, was SM-DES-007 auf der Kachel zulässt.
///
/// **Ohne Garnfarben.** Schnittregel 4 verlangt, dass eine Ansicht mit einer
/// von der Zeilenzahl unabhängigen Zahl von Abfragen beantwortet wird — sie
/// verlangt nicht, Daten zu laden, die niemand zeichnet. Auf der Kachel stehen
/// nur Bild, Formatmarke, Name und Größe; die Farben eines Ausschnitts von 500
/// Zeilen wären bis zu 7.500 Ergebniszeilen, die über die Schnittstelle wandern
/// und verworfen werden. Sie kommen über [`Fassade::garnfarben`] für den
/// **ausgewählten** Eintrag.
#[derive(Debug, Clone)]
pub struct Kachel {
    pub id: i64,
    pub uid: String,
    pub name: String,
    pub dateiname: String,
    /// Ablageort der Quelldatei — für den Vorschauweg, nicht für die Anzeige
    /// (SM-SEC-010).
    pub pfad: String,
    pub format: String,
    pub breite_mm: Option<f64>,
    pub hoehe_mm: Option<f64>,
    /// Der Fehlerzustand der Kachel (SM-IMP-009, DES-STM-001 Abschnitt 10).
    pub fehlergrund: Option<String>,
}

// **Was hier bewusst fehlt.** DES-STM-001 Abschnitt 6.3 zählt den Kachelaufbau
// abschließend auf — Vorschaufläche, Formatmarke, Herkunftsmarke, Name, Größe —
// und schreibt „Mehr steht nicht auf der Kachel". Stichzahl, Farbzahl und
// Favoritenmarke sind Filter- und Sortiergrößen; sie wirken in der Abfrage,
// nicht in der Anzeige, und wurden hier nur mitgeschleppt.
//
// Die **Herkunftsmarke** fehlt ebenfalls, und zwar begründet: Sie war fest auf
// `Herkunft::Datei` gesetzt, es gibt keine Schemaspalte dafür und keinen Weg,
// sie zu setzen — eine Kennzeichnung, die nie etwas kennzeichnet, ist
// schlechter als keine (SM-KIA-008, SM-DES-009). Sie kommt mit dem Paket, das
// maschinell erzeugte Werte überhaupt erst erzeugt (AP-18).

/// Die Antwort auf einen Ausschnittabruf.
#[derive(Debug, Clone)]
pub struct Seite {
    pub kacheln: Vec<Kachel>,
    /// Nur beim **ersten** Ausschnitt eines Suchlaufs gesetzt (Schnittregel 3).
    pub gesamt: Option<i64>,
    pub versatz: i64,
    /// Zahl der Datenbankabfragen, die dieser Abruf gekostet hat.
    ///
    /// Messgröße für PF-PRV-07.1. Sie muss bei wachsender Ausschnitt- und
    /// Bestandsgröße **konstant** bleiben.
    pub abfragen: u64,
}

/// Was ein inkrementeller Lauf zu tun hat (SM-IMP-003).
#[derive(Debug, Default)]
pub struct Aenderungssatz {
    /// Im Dateisystem, nicht im Bestand.
    pub neu: Vec<std::path::PathBuf>,
    /// In beiden, aber Größe oder Änderungszeit weichen ab.
    pub geaendert: Vec<(std::path::PathBuf, i64, String)>,
    /// Im Bestand, aber nicht mehr im Dateisystem.
    pub vermisst: Vec<(i64, String)>,
    /// Wieder aufgetaucht: im Bestand als vermisst gekennzeichnet, aber da.
    pub wiedergefunden: Vec<i64>,
    /// Zahl der Dateien, die keiner Arbeit bedürfen.
    pub unveraendert: usize,
}

impl Aenderungssatz {
    /// Zahl der Dateien, die tatsächlich gelesen werden müssen.
    pub fn zu_lesen(&self) -> usize {
        self.neu.len() + self.geaendert.len()
    }

    /// Ist nichts zu tun?
    pub fn leer(&self) -> bool {
        self.zu_lesen() == 0 && self.vermisst.is_empty() && self.wiedergefunden.is_empty()
    }
}

/// Das Ergebnis eines Laufs.
#[derive(Debug, Default, Clone, Copy)]
pub struct Importbefund {
    pub neu: usize,
    pub geaendert: usize,
    pub unveraendert: usize,
    pub vermisst: usize,
    pub wiedergefunden: usize,
    pub abgewiesen: usize,
}

/// Die Fassade.
pub struct Fassade {
    haltung: Datenhaltung,
    wurzel: Option<Wurzel>,
    /// Der dauerhafte Vorschauspeicher (SM-PRV-002).
    ///
    /// Er gehört hierher, nicht in eine Schicht daneben: Die Oberfläche
    /// erreicht den Kern ausschließlich über diese Kiste (SM-SEC-004). Lag der
    /// Speicher außerhalb, war er über die einzige zulässige Schnittstelle
    /// nicht erreichbar — jede sichtbar werdende Kachel hätte die Quelldatei
    /// erneut gelesen, geparst und ein PNG erneut komprimiert.
    speicher: Option<Zwischenspeicher>,
}

impl Fassade {
    /// Öffnet den Bestand an einem Ort.
    pub fn oeffnen(datenbank: impl AsRef<Path>) -> Ergebnis<Self> {
        Ok(Self {
            haltung: Datenhaltung::oeffnen(datenbank)?,
            wurzel: None,
            speicher: Zwischenspeicher::am_standardort().ok(),
        })
    }

    /// Eine Fassade im Arbeitsspeicher — für Prüffälle und Messaufbauten.
    pub fn im_speicher() -> Ergebnis<Self> {
        Ok(Self {
            haltung: Datenhaltung::im_speicher()?,
            wurzel: None,
            speicher: None,
        })
    }

    /// Legt den Ablageort des Vorschauspeichers fest (SM-PRV-002).
    pub fn speicher_setzen(&mut self, speicher: Zwischenspeicher) {
        self.speicher = Some(speicher);
    }

    /// Setzt die Bibliothekswurzel; sie wird dabei geprüft (SM-SEC-003).
    pub fn wurzel_setzen(&mut self, pfad: impl AsRef<Path>) -> Ergebnis<()> {
        self.wurzel = Some(Wurzel::neu(pfad)?);
        Ok(())
    }

    /// Die gesetzte Wurzel, falls es eine gibt.
    pub fn wurzel(&self) -> Option<&Wurzel> {
        self.wurzel.as_ref()
    }

    /// Liefert einen Ausschnitt der Treffermenge.
    ///
    /// `mit_gesamtzahl` gehört zum **ersten** Ausschnitt eines Suchlaufs; jeder
    /// Folgeausschnitt setzt ihn auf `false`, sonst läuft je Bildlaufabschnitt
    /// eine Zählabfrage über die volle Treffermenge (Schnittregel 3).
    ///
    /// Die Zahl der Abfragen hängt **nicht** an der Zeilenzahl: eine für den
    /// Ausschnitt, eine für die Garnfarben aller Zeilen, höchstens eine für die
    /// Gesamtzahl (Schnittregel 4).
    pub fn ausschnitt(
        &self,
        abfrage: &Abfrage,
        versatz: i64,
        anzahl: i64,
        mit_gesamtzahl: bool,
    ) -> Ergebnis<Seite> {
        self.haltung.abfragen_zuruecksetzen();

        let Ausschnitt {
            zeilen,
            gesamt,
            versatz,
        } = self
            .haltung
            .suche(abfrage, versatz, anzahl, mit_gesamtzahl)?;

        let kacheln = zeilen
            .into_iter()
            .map(|z: Trefferzeile| Kachel {
                id: z.id,
                uid: z.uid,
                name: z.name,
                dateiname: z.dateiname,
                pfad: z.pfad,
                format: z.format,
                breite_mm: z.breite_mm,
                hoehe_mm: z.hoehe_mm,
                fehlergrund: z.fehlergrund,
            })
            .collect();

        Ok(Seite {
            kacheln,
            gesamt,
            versatz,
            abfragen: self.haltung.abfragen_gesamt(),
        })
    }

    /// Die Garnfarben **eines** Eintrags (SM-MET-010).
    ///
    /// Eine Abfrage je Auswahlwechsel, nicht je Zeile eines Ausschnitts: Der
    /// Detailbereich zeigt sie, die Kachel nicht (SM-DES-007).
    pub fn garnfarben(&self, eintrag_id: i64) -> Ergebnis<Vec<Garnfarbe>> {
        self.haltung.garnfarben(eintrag_id)
    }

    /// Ermittelt, was ein Lauf zu tun hat — **ohne eine Datei zu lesen**
    /// (SM-IMP-003).
    ///
    /// Verglichen wird über den Pfad, entschieden über **Größe und
    /// Änderungszeit** — dieselben zwei Größen wie beim Vorschau-Zwischenspeicher
    /// und aus demselben Grund: Ein Inhaltsvergleich läse die ganze Bibliothek
    /// und wäre genau der Aufwand, den SM-IMP-003 ausschließt.
    pub fn aenderungen_ermitteln(
        &self,
        gefunden: &[std::path::PathBuf],
    ) -> Ergebnis<Aenderungssatz> {
        let bestand = self.haltung.bestandsuebersicht()?;
        let mut satz = Aenderungssatz::default();

        let mut gesehen: std::collections::HashSet<&str> = std::collections::HashSet::new();

        // Zwei Anforderungen, die sich zu widersprechen scheinen — und ihre
        // Auflösung.
        //
        // SM-IMP-003 will den Abgleich billig: Ein `canonicalize` je Datei
        // kostet bei 100.000 Einträgen 100.000 zusätzliche Systemaufrufe.
        // SM-SEC-002 will ihn sicher: Ein rein **lexikalischer** Präfixvergleich
        // ist keine Prüfung — `<wurzel>/../fremd` beginnt lexikalisch mit der
        // Wurzel, und ein Symlink am Ende zeigt, wohin er will.
        //
        // Beides zusammen geht, wenn der billige Weg nur dort greift, wo er
        // **beweisbar** dasselbe leistet:
        //   · kein `..` im Pfad — rein lexikalisch, kostenlos;
        //   · der letzte Bestandteil ist kein Symlink — `symlink_metadata`,
        //     ein Systemaufruf, den der Abgleich für Größe und Änderungszeit
        //     ohnehin macht;
        //   · der Pfad liegt unter der kanonisierten Wurzel.
        // Trifft eines davon nicht zu, wird vollständig aufgelöst.
        let wurzelpfad = self
            .wurzel
            .as_ref()
            .map(|w| w.pfad().to_path_buf())
            .unwrap_or_default();

        for datei in gefunden {
            let datei = if schnellweg_traegt(datei, &wurzelpfad) {
                datei.clone()
            } else {
                match self.pfad_pruefen(datei) {
                    Ok(p) => p,
                    // Außerhalb der Bibliothek — kein Gegenstand dieses Laufs.
                    Err(_) => continue,
                }
            };
            let schluessel = datei.to_string_lossy().to_string();
            let Some(zeile) = bestand.get(&schluessel) else {
                satz.neu.push(datei.clone());
                continue;
            };
            gesehen.insert(zeile.pfad.as_str());

            if kennzeichen_gleich(&datei, zeile) {
                satz.unveraendert += 1;
                // Eine als vermisst gekennzeichnete Datei, die wieder da ist.
                if zeile.vermisst {
                    satz.wiedergefunden.push(zeile.id);
                }
            } else {
                satz.geaendert.push((datei, zeile.id, zeile.uid.clone()));
            }
        }

        for (pfad, zeile) in &bestand {
            if !gesehen.contains(pfad.as_str()) && !zeile.vermisst {
                satz.vermisst.push((zeile.id, zeile.uid.clone()));
            }
        }

        Ok(satz)
    }

    /// Schreibt einen geänderten Eintrag fort; die Kennung bleibt (SM-LIB-010).
    pub fn erneuern(&mut self, id: i64, datei: impl AsRef<Path>) -> Ergebnis<()> {
        let neu = self.einlesen(datei)?;
        self.haltung.eintrag_fortschreiben(id, &neu)
    }

    /// Kennzeichnet Einträge als vermisst (nicht löschen — SM-DAT-003).
    pub fn vermisst_kennzeichnen(&mut self, ids: &[i64]) -> Ergebnis<usize> {
        self.haltung.als_vermisst_kennzeichnen(ids)
    }

    /// Hebt die Vermisstmarke auf.
    pub fn vermisst_aufheben(&mut self, ids: &[i64]) -> Ergebnis<usize> {
        self.haltung.vermisst_aufheben(ids)
    }

    /// Nimmt eine Datei in den Bestand auf.
    ///
    /// Der Pfad läuft durch `kern-security`, bevor er irgendwo landet
    /// (Schnittregel 5). Die Datei selbst wird **nicht** verändert (RB-04):
    /// Sie wird gelesen, nie geschrieben.
    pub fn aufnehmen(&mut self, datei: impl AsRef<Path>) -> Ergebnis<i64> {
        let neu = self.einlesen(datei)?;
        self.haltung.eintrag_anlegen(&neu)
    }

    /// Liest eine Datei und bildet daraus einen Eintrag.
    ///
    /// Der Pfad läuft durch `kern-security`, bevor er irgendwo landet
    /// (Schnittregel 5). Die Datei selbst wird **nicht** verändert (RB-04):
    /// Sie wird gelesen, nie geschrieben.
    fn einlesen(&self, datei: impl AsRef<Path>) -> Ergebnis<NeuerEintrag> {
        let wurzel = self
            .wurzel
            .as_ref()
            .ok_or_else(|| Fehler::Eingabe("Es ist keine Bibliothek gewählt.".into()))?;

        let pfad = wurzel.pruefe(datei)?;

        let endung = pfad
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let format = Format::aus_endung(endung)
            .ok_or_else(|| Fehler::Eingabe("Dieses Dateiformat wird nicht unterstützt.".into()))?;

        let dateiname_fuer_fehler = pfad
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        // SM-IMP-009: Eine nicht lesbare Datei wird **erfasst**, nicht
        // übersprungen. Die Nutzerin soll sehen, welche Datei fehlt und warum;
        // eine still verschwundene Datei ist für sie unauffindbar.
        let daten = match lies_fremddatei(&pfad) {
            Ok(d) => d,
            Err(e) => return Ok(fehlereintrag(&pfad, &dateiname_fuer_fehler, format, &e)),
        };
        let groesse = daten.len() as i64;

        // Der Import ist der Fremddatenpfad: gehärtet lesen (SM-FMT-012).
        let kennwerte = match kern_parsers::lies_kennwerte(format, &daten) {
            Ok(k) => k,
            Err(e) => return Ok(fehlereintrag(&pfad, &dateiname_fuer_fehler, format, &e)),
        };

        let dateiname = pfad
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        Ok(NeuerEintrag {
            uid: kennung(&pfad, &daten),
            pfad: pfad.clone(),
            dateiname: dateiname.clone(),
            format,
            groesse_bytes: groesse,
            // Der Inhaltshash entsteht **nur hier** — also nur für Dateien, die
            // die Änderungserkennung als neu oder geändert ausweist (AP-08).
            // Er kostet keine zusätzliche Ein-/Ausgabe: Die Datei liegt für den
            // Parser ohnehin im Speicher.
            inhalt_hash: Some(inhaltshash(&daten)),
            breite_mm: kennwerte.breite_mm,
            hoehe_mm: kennwerte.hoehe_mm,
            stichzahl: kennwerte.stichzahl,
            farbzahl: kennwerte.farbzahl,
            name: kennwerte
                .entwurfsname
                .filter(|n| !n.trim().is_empty())
                .unwrap_or_else(|| {
                    pfad.file_stem()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or(dateiname)
                }),
            datei_geaendert_am: aenderungszeit(&pfad),
            farben: kennwerte.farben,
            fehlerstatus: None,
            fehlergrund: None,
        })
    }

    /// Prüft einen Pfad gegen die Bibliothekswurzel und gibt ihn aufgelöst
    /// zurück (Schnittregel 5, SM-SEC-001 bis 003).
    ///
    /// Wer die Quelldatei anfassen will — sei es nur, um Größe und
    /// Änderungszeit zu lesen —, holt sich den Pfad hier. Ein relativer Pfad
    /// gilt gegen die Wurzel, nie gegen das Arbeitsverzeichnis des Prozesses.
    pub fn pfad_pruefen(&self, pfad: impl AsRef<Path>) -> Ergebnis<std::path::PathBuf> {
        let wurzel = self
            .wurzel
            .as_ref()
            .ok_or_else(|| Fehler::Eingabe("Es ist keine Bibliothek gewählt.".into()))?;
        wurzel.pruefe(pfad)
    }

    /// Liefert die Vorschau eines Eintrags — aus dem Zwischenspeicher, wenn
    /// sie dort gültig vorliegt (SM-PRV-002, SM-PRV-003).
    ///
    /// Gültig heißt: Größe und Änderungszeit der Quelldatei stimmen mit dem
    /// überein, was beim Ablegen galt. Die Prüfung kostet ein `stat` und ein
    /// `exists` — kein Lesen der Quelldatei, kein Zeichnen, keine
    /// PNG-Kodierung. Das ist der Weg, den die Oberfläche nimmt.
    pub fn vorschau_gepuffert(
        &self,
        kennung: &str,
        datei: impl AsRef<Path>,
        stufe: Stufe,
    ) -> Ergebnis<std::path::PathBuf> {
        let pfad = self.pfad_pruefen(datei)?;

        if let Some(sp) = self.speicher.as_ref() {
            if let Some(fertig) = sp.holen(kennung, &pfad, stufe) {
                return Ok(fertig);
            }
        }

        let opt = kern_render::Vorschauoption {
            breite_px: stufe.kantenlaenge(),
            hoehe_px: stufe.kantenlaenge(),
            ..Default::default()
        };
        let png = self.vorschau(&pfad, opt)?;

        let Some(sp) = self.speicher.as_ref() else {
            return Err(Fehler::Intern("Kein Vorschauspeicher gesetzt.".into()));
        };
        sp.ablegen(kennung, &pfad, stufe, &png)
    }

    /// Erzeugt die Vorschau eines Eintrags aus seinen Stichdaten — **ohne**
    /// Zwischenspeicher.
    ///
    /// Der ungepufferte Sonderweg für Druck und Export, wo eine freie
    /// Auflösung gebraucht wird. Der Anzeigepfad nimmt
    /// [`Fassade::vorschau_gepuffert`].
    pub fn vorschau(
        &self,
        datei: impl AsRef<Path>,
        opt: kern_render::Vorschauoption,
    ) -> Ergebnis<Vec<u8>> {
        let wurzel = self
            .wurzel
            .as_ref()
            .ok_or_else(|| Fehler::Eingabe("Es ist keine Bibliothek gewählt.".into()))?;
        let pfad = wurzel.pruefe(datei)?;

        let endung = pfad
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or_default();
        let format = Format::aus_endung(endung)
            .ok_or_else(|| Fehler::Eingabe("Dieses Dateiformat wird nicht unterstützt.".into()))?;

        let daten = lies_fremddatei(&pfad)?;
        let abschnitte = kern_parsers::lies_stichabschnitte(format, &daten)?;
        kern_render::zeichne(&abschnitte, opt)?.als_png()
    }

    /// Setzt die Schlagworte eines Eintrags.
    pub fn schlagworte_setzen(&mut self, eintrag_id: i64, worte: &[String]) -> Ergebnis<()> {
        self.haltung.schlagworte_setzen(eintrag_id, worte)
    }

    /// Bestandsgröße für die Statusleiste.
    pub fn bestandsgroesse(&self) -> Ergebnis<i64> {
        self.haltung.bestandsgroesse()
    }
}

/// Darf der billige Weg statt der vollen Auflösung greifen?
///
/// Nur wenn er beweisbar dasselbe leistet: kein Aufstieg im Pfad, kein Symlink
/// am Ende, und unterhalb der kanonisierten Wurzel (SM-SEC-002, SM-IMP-003).
fn schnellweg_traegt(datei: &Path, wurzel: &Path) -> bool {
    if !datei.starts_with(wurzel) {
        return false;
    }
    if datei
        .components()
        .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return false;
    }
    match std::fs::symlink_metadata(datei) {
        Ok(m) => !m.file_type().is_symlink(),
        Err(_) => false,
    }
}

/// Größe und Änderungszeit stimmen mit dem Bestand überein.
fn kennzeichen_gleich(datei: &Path, zeile: &Bestandszeile) -> bool {
    // **Ein** Systemaufruf je Datei. Die frühere Fassung las die Metadaten
    // zweimal — einmal für die Größe, einmal über `aenderungszeit`. Bei
    // 100.000 unveränderten Dateien sind das 100.000 vermeidbare Aufrufe auf
    // genau dem Weg, den SM-IMP-003 billig halten soll.
    let Ok(m) = std::fs::metadata(datei) else {
        return false;
    };
    if m.len() as i64 != zeile.groesse_bytes {
        return false;
    }
    aenderungszeit_aus(&m) == zeile.datei_geaendert_am
}

/// Änderungszeit aus bereits gelesenen Metadaten.
fn aenderungszeit_aus(m: &std::fs::Metadata) -> Option<i64> {
    m.modified()
        .ok()
        .and_then(|z| z.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs() as i64)
}

/// Änderungszeit einer Datei in Sekunden seit 1970.
fn aenderungszeit(pfad: &Path) -> Option<i64> {
    std::fs::metadata(pfad)
        .ok()
        .as_ref()
        .and_then(aenderungszeit_aus)
}

/// Hash des Dateiinhalts — Grundlage der Duplikaterkennung (SM-IMP-005).
fn inhaltshash(daten: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(daten);
    format!("{:x}", h.finalize())
}

/// Bildet einen Eintrag für eine nicht lesbare Datei (SM-IMP-009).
///
/// Der Eintrag trägt den Fehlerstatus und einen für Endnutzer verständlichen
/// Grund; die Kachel kann ihn nach DES-STM-001 Abschnitt 10 als „Fehler"
/// darstellen. Die Alternative — überspringen — nähme der Nutzerin jede
/// Möglichkeit, die Ursache zu finden.
fn fehlereintrag(pfad: &Path, dateiname: &str, format: Format, grund: &Fehler) -> NeuerEintrag {
    NeuerEintrag {
        uid: format!("fehler-{}", kennung(pfad, dateiname.as_bytes())),
        pfad: pfad.to_path_buf(),
        dateiname: dateiname.to_string(),
        format,
        groesse_bytes: std::fs::metadata(pfad).map(|m| m.len() as i64).unwrap_or(0),
        inhalt_hash: None,
        breite_mm: None,
        hoehe_mm: None,
        stichzahl: None,
        farbzahl: None,
        name: pfad
            .file_stem()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| dateiname.to_string()),
        datei_geaendert_am: aenderungszeit(pfad),
        farben: Vec::new(),
        fehlerstatus: Some("nicht_lesbar".to_string()),
        fehlergrund: Some(grund.to_string()),
    }
}

/// Eine dauerhafte Kennung, die Umbenennung und Verschiebung übersteht
/// (SM-LIB-010).
fn kennung(pfad: &Path, daten: &[u8]) -> String {
    // Aus Inhalt und Dateinamen, nicht aus dem Pfad: Ein Verschieben ändert den
    // Pfad, nicht die Kennung.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let name = pfad
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    for b in name.as_bytes().iter().chain(daten.iter().take(4096)) {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    format!("{h:016x}-{}", daten.len())
}

#[cfg(test)]
mod tests;
