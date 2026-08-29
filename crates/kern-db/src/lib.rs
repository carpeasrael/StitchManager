//! Datenhaltung — SQLite mit WAL, additiven Migrationen und Volltextindex.
//!
//! Trägt SM-DAT-006 bis 008, SM-SEC-005, SM-SRC-001 bis 010 und SM-LIB-009/010.
//!
//! **Diese Kiste ist für die Oberfläche unerreichbar.** Schnittregel 1 aus
//! IMP-STM-001 Abschnitt 3 (SM-SEC-004) verlangt, dass jeder Zugriff über
//! `kern-fassade` läuft; der Werkstattverbund erzwingt das über die
//! Abhängigkeitsrichtung, nicht über eine Sichtprüfung.

#![forbid(unsafe_code)]

pub mod migrationen;

use kern_typen::{Ergebnis, Fehler, Format, Garnfarbe};
use rusqlite::{params, params_from_iter, Connection, OptionalExtension};
use std::path::{Path, PathBuf};

fn db_fehler(e: rusqlite::Error) -> Fehler {
    // Der technische Text gehört ins Protokoll, nicht in die Meldung
    // (SM-NFR-006, SM-SEC-010).
    log::error!("Datenbankfehler: {e}");
    // „Datenhaltung" ist ein Wort aus der Bauweise, nicht aus der
    // Begriffstabelle (Lastenheft Abschnitt 1.6). Die Nutzerin kennt ihre
    // Bibliothek.
    Fehler::Datenbank("Die Bibliothek konnte den Vorgang nicht ausführen.".into())
}

/// Ein neu aufzunehmender Eintrag.
#[derive(Debug, Clone)]
pub struct NeuerEintrag {
    pub uid: String,
    pub pfad: PathBuf,
    pub dateiname: String,
    pub format: Format,
    pub groesse_bytes: i64,
    pub inhalt_hash: Option<String>,
    pub breite_mm: Option<f64>,
    pub hoehe_mm: Option<f64>,
    pub stichzahl: Option<i64>,
    pub farbzahl: Option<i64>,
    pub name: String,
    pub datei_geaendert_am: Option<i64>,
    pub farben: Vec<Garnfarbe>,
    /// Gesetzt, wenn die Datei nicht lesbar war (SM-IMP-009).
    ///
    /// Der Eintrag entsteht trotzdem — mit Fehlerstatus und einem für
    /// Endnutzer verständlichen Grund. Eine übersprungene Datei wäre für die
    /// Nutzerin unsichtbar; sie soll sehen, **welche** Datei fehlt und warum.
    pub fehlerstatus: Option<String>,
    pub fehlergrund: Option<String>,
}

/// Eine abgewiesene Datei — mit **Namen**, nicht nur als Zahl.
///
/// „37 Dateien abgewiesen" sagt der Nutzerin nicht, **welche** fehlt. Der
/// vollständige Pfad bleibt draußen (SM-SEC-010); der Dateiname genügt zum
/// Wiederfinden.
#[derive(Debug, Clone)]
pub struct Abweisung {
    pub dateiname: String,
    pub grund: String,
}

/// Der Bestandsstand eines Eintrags für die Änderungserkennung (SM-IMP-003).
#[derive(Debug, Clone)]
pub struct Bestandszeile {
    pub id: i64,
    pub uid: String,
    pub pfad: String,
    pub groesse_bytes: i64,
    pub datei_geaendert_am: Option<i64>,
    pub vermisst: bool,
}

/// Eine Zeile der Treffermenge — genau die Felder, die die Kachel braucht.
///
/// Auf der Kachel stehen nur Bild, Formatmarke, Name und Größe (SM-DES-007).
/// Weitere Felder gehören in den Detailbereich und werden dort einzeln geholt;
/// sie hier mitzuliefern verteuerte jeden Bildlaufabschnitt.
#[derive(Debug, Clone)]
pub struct Trefferzeile {
    pub id: i64,
    pub uid: String,
    /// Der Ablageort der Quelldatei.
    ///
    /// Er dient dem Vorschau- und Detailweg, **nicht der Anzeige**: Ein
    /// unmaskierter Pfad gehört weder in die Oberfläche noch in ein Protokoll
    /// (SM-SEC-010). Ohne ihn fände der Vorschauweg jede Datei nur dann, wenn
    /// sie unmittelbar in der Bibliothekswurzel liegt.
    pub pfad: String,
    pub dateiname: String,
    pub name: String,
    pub format: String,
    pub breite_mm: Option<f64>,
    pub hoehe_mm: Option<f64>,
    /// Gesetzt, wenn die Quelldatei nicht lesbar war (SM-IMP-009).
    ///
    /// Kein zusätzliches Kachelfeld im Sinne von SM-DES-007, sondern der
    /// **Zustand** der Kachel: DES-STM-001 Abschnitt 10 verlangt „Datei nicht
    /// lesbar" als eigenen, ausformulierten Zustand.
    pub fehlergrund: Option<String>,
}

/// Vollständiger Lesedatensatz für den ausgewählten Eintrag (AP-13).
///
/// Leere optionale Textfelder werden als `None` geliefert. Der Detailbereich
/// darf fehlende Metadaten nicht mit erfundenen Werten auffüllen.
#[derive(Debug, Clone, PartialEq)]
pub struct Detailzeile {
    pub uid: String,
    pub name: String,
    pub thema: Option<String>,
    pub beschreibung: Option<String>,
    pub notizen: Option<String>,
    pub format: String,
    pub breite_mm: Option<f64>,
    pub hoehe_mm: Option<f64>,
    pub stichzahl: Option<i64>,
    pub farbzahl: Option<i64>,
    pub fehlerstatus: Option<String>,
    pub fehlergrund: Option<String>,
    pub schlagworte: Vec<String>,
    pub garnfarben: Vec<Garnfarbe>,
}

/// Ein Ausschnitt einer Treffermenge (Schnittregel 3).
///
/// `gesamt` trägt **nur der erste Ausschnitt** eines Suchlaufs. Folgeausschnitte
/// lassen das Feld leer — sonst liefe je Bildlaufabschnitt eine Zählabfrage über
/// die volle Treffermenge, und SM-SRC-007 fiele an der Schnittstelle statt im
/// Index.
#[derive(Debug, Clone)]
pub struct Ausschnitt {
    pub zeilen: Vec<Trefferzeile>,
    pub gesamt: Option<i64>,
    pub versatz: i64,
}

/// Sortierschlüssel (SM-SRC-005).
///
/// Ein Aufzählungstyp, kein Text: So kann kein Fremdwert in die
/// Sortierklausel gelangen, für die SQLite keine Platzhalter kennt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sortierung {
    #[default]
    Name,
    ImportDatum,
    DateiDatum,
    Groesse,
    Stichzahl,
    Relevanz,
}

impl Sortierung {
    /// Die Sortierklausel als feste Zeichenkette aus dem Programmtext.
    fn klausel(&self) -> &'static str {
        match self {
            Self::Name => "e.name COLLATE NOCASE ASC, e.id ASC",
            Self::ImportDatum => "e.importiert_am DESC, e.id ASC",
            Self::DateiDatum => "e.datei_geaendert_am DESC, e.id ASC",
            Self::Groesse => "e.groesse_bytes DESC, e.id ASC",
            Self::Stichzahl => "e.stichzahl DESC, e.id ASC",
            Self::Relevanz => "rang ASC, e.id ASC",
        }
    }
}

/// Eine Suchanfrage: Volltext und Filter, beliebig kombinierbar (SM-SRC-010).
#[derive(Debug, Clone, Default)]
pub struct Suchabfrage {
    /// Freitext. Wird als Wortfolge quotiert, nie als FTS5-Ausdruck übernommen.
    pub text: Option<String>,
    pub formate: Vec<Format>,
    pub schlagworte: Vec<String>,
    pub stichzahl_von: Option<i64>,
    pub stichzahl_bis: Option<i64>,
    pub farbzahl_von: Option<i64>,
    pub farbzahl_bis: Option<i64>,
    pub nur_favoriten: bool,
    pub sortierung: Sortierung,
}

/// Quotiert einen Suchtext für FTS5 (SM-SEC-005, Suchpfad).
///
/// Ein roher Nutzertext ist ein FTS5-*Ausdruck*: `NEAR`, `*`, `"` und `OR`
/// haben dort Bedeutung, und ein unpaariges Anführungszeichen lässt die
/// Abfrage scheitern. Jedes Wort wird deshalb einzeln in Anführungszeichen
/// gesetzt und ein enthaltenes Anführungszeichen verdoppelt.
fn quotiere_fts(roh: &str) -> Option<String> {
    let worte: Vec<String> = roh
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|w| format!("\"{}\"", w.replace('"', "\"\"")))
        .collect();
    if worte.is_empty() {
        None
    } else {
        Some(worte.join(" "))
    }
}

/// Die Datenhaltung.
pub struct Datenhaltung {
    conn: Connection,
    /// Zahl der abgesetzten Abfragen.
    ///
    /// Messgröße für PF-PRV-07.1: Schnittregel 4 verlangt eine von der
    /// Zeilenzahl **unabhängige** Zahl von Abfragen je Ausschnitt. Ohne diese
    /// Größe ließe sich die Regel nicht durchfallen — die Zahl der
    /// Fassadenaufrufe bliebe auch dann konstant 1, wenn darin je Zeile eine
    /// Abfrage liefe.
    abfragen: std::cell::Cell<u64>,
}

impl Datenhaltung {
    /// Öffnet die Datenhaltung, legt vor einer Anpassung eine Sicherung an und
    /// wendet fehlende Schritte an (SM-DAT-006, SM-DAT-008).
    pub fn oeffnen(pfad: impl AsRef<Path>) -> Ergebnis<Self> {
        let pfad = pfad.as_ref();
        let conn = Connection::open(pfad).map_err(db_fehler)?;
        let mut haltung = Self {
            conn,
            abfragen: std::cell::Cell::new(0),
        };
        haltung.grundeinstellungen()?;

        let vorhanden = haltung.schema_stand()?;
        let ziel = migrationen::SCHRITTE.last().map(|s| s.nummer).unwrap_or(0);

        // Vor jeder Anpassung einer *bestehenden* Datenhaltung eine Sicherung
        // (SM-DAT-008). Beim Erstanlegen gibt es nichts zu sichern.
        if vorhanden > 0 && vorhanden < ziel {
            haltung.sicherung_anlegen(pfad, vorhanden)?;
        }

        haltung.migrieren()?;
        Ok(haltung)
    }

    /// Eine Datenhaltung im Arbeitsspeicher — für Prüffälle.
    pub fn im_speicher() -> Ergebnis<Self> {
        let conn = Connection::open_in_memory().map_err(db_fehler)?;
        let mut haltung = Self {
            conn,
            abfragen: std::cell::Cell::new(0),
        };
        haltung.grundeinstellungen()?;
        haltung.migrieren()?;
        Ok(haltung)
    }

    /// Zahl der bisher abgesetzten Abfragen.
    pub fn abfragen_gesamt(&self) -> u64 {
        self.abfragen.get()
    }

    /// Setzt den Abfragezähler zurück — für Messaufbauten.
    pub fn abfragen_zuruecksetzen(&self) {
        self.abfragen.set(0);
    }

    fn zaehle(&self) {
        self.abfragen.set(self.abfragen.get() + 1);
    }

    fn grundeinstellungen(&mut self) -> Ergebnis<()> {
        // WAL trägt SM-DAT-006: Nach einem unerwarteten Programmende läuft die
        // Datenhaltung ohne Verlust wieder an.
        self.conn
            .pragma_update(None, "journal_mode", "WAL")
            .map_err(db_fehler)?;
        self.conn
            .pragma_update(None, "synchronous", "NORMAL")
            .map_err(db_fehler)?;
        self.conn
            .pragma_update(None, "foreign_keys", "ON")
            .map_err(db_fehler)?;
        // Ein Schreibvorgang wartet, statt sofort mit "database is locked"
        // abzubrechen.
        self.conn
            .busy_timeout(std::time::Duration::from_secs(5))
            .map_err(db_fehler)?;
        Ok(())
    }

    fn schema_stand(&self) -> Ergebnis<i64> {
        let da: bool = self
            .conn
            .query_row(
                "SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_schritt'",
                [],
                |_| Ok(true),
            )
            .optional()
            .map_err(db_fehler)?
            .unwrap_or(false);
        if !da {
            return Ok(0);
        }
        self.conn
            .query_row(
                "SELECT COALESCE(MAX(nummer), 0) FROM schema_schritt",
                [],
                |z| z.get(0),
            )
            .map_err(db_fehler)
    }

    fn sicherung_anlegen(&self, pfad: &Path, stand: i64) -> Ergebnis<()> {
        let ziel = pfad.with_extension(format!("vor-schritt-{stand}.sicherung"));
        log::info!("Sicherung vor Schemaanpassung wird angelegt");
        self.conn
            .backup(rusqlite::DatabaseName::Main, &ziel, None)
            .map_err(db_fehler)?;
        Ok(())
    }

    /// Wendet alle noch nicht angewandten Schritte an.
    fn migrieren(&mut self) -> Ergebnis<()> {
        self.conn
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS schema_schritt (
                     nummer      INTEGER PRIMARY KEY,
                     name        TEXT    NOT NULL,
                     angewandt_am INTEGER NOT NULL
                 ) STRICT;",
            )
            .map_err(db_fehler)?;

        let stand = self.schema_stand()?;

        for schritt in migrationen::SCHRITTE {
            if schritt.nummer <= stand {
                continue;
            }
            let tx = self.conn.transaction().map_err(db_fehler)?;
            // SM-SEC-005-Ausnahme: `schritt.sql` ist ein ausgelieferter Schemaschritt aus `migrationen.rs`, kein Fremdwert.
            tx.execute_batch(schritt.sql).map_err(|e| {
                log::error!(
                    "Schemaschritt {} ({}) scheiterte: {e}",
                    schritt.nummer,
                    schritt.name
                );
                db_fehler(e)
            })?;
            tx.execute(
                "INSERT INTO schema_schritt (nummer, name, angewandt_am) VALUES (?1, ?2, ?3)",
                params![schritt.nummer, schritt.name, jetzt()],
            )
            .map_err(db_fehler)?;
            tx.commit().map_err(db_fehler)?;
            log::info!(
                "Schemaschritt {} ({}) angewandt",
                schritt.nummer,
                schritt.name
            );
        }
        Ok(())
    }

    /// Nimmt einen Eintrag samt Garnfarben in einem Vorgang auf.
    pub fn eintrag_anlegen(&mut self, neu: &NeuerEintrag) -> Ergebnis<i64> {
        let tx = self.conn.transaction().map_err(db_fehler)?;
        let jetzt = jetzt();

        // Die Kennung wird eindeutig gemacht, falls sie schon vergeben ist —
        // gemeinsam mit dem Stapelweg, damit beide Aufnahmewege für denselben
        // Bestand dasselbe Ergebnis liefern (SM-IMP-003, SM-IMP-009).
        let uid = eindeutige_kennung(&tx, &neu.uid)?;

        tx.execute(
            "INSERT INTO eintrag (
                 uid, pfad, dateiname, format, groesse_bytes, inhalt_hash,
                 breite_mm, hoehe_mm, stichzahl, farbzahl, name,
                 importiert_am, datei_geaendert_am, geaendert_am,
                 fehlerstatus, fehlergrund)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
            params![
                uid,
                neu.pfad.to_string_lossy(),
                neu.dateiname,
                neu.format.marke(),
                neu.groesse_bytes,
                neu.inhalt_hash,
                neu.breite_mm,
                neu.hoehe_mm,
                neu.stichzahl,
                neu.farbzahl,
                neu.name,
                jetzt,
                neu.datei_geaendert_am,
                jetzt,
                neu.fehlerstatus,
                neu.fehlergrund,
            ],
        )
        .map_err(db_fehler)?;

        let id = tx.last_insert_rowid();

        {
            let mut einf = tx
                .prepare(
                    "INSERT INTO garnfarbe
                         (eintrag_id, reihenfolge, hex, name, marke, markenschluessel)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )
                .map_err(db_fehler)?;
            for (i, f) in neu.farben.iter().enumerate() {
                einf.execute(params![
                    id,
                    i as i64,
                    f.hex,
                    f.name,
                    f.marke,
                    f.markenschluessel
                ])
                .map_err(db_fehler)?;
            }
        }

        tx.commit().map_err(db_fehler)?;
        Ok(id)
    }

    /// Der Bestand, wie ihn die Änderungserkennung braucht (SM-IMP-003).
    ///
    /// Geliefert werden **nur** Pfad, Größe und Änderungszeit — nicht der ganze
    /// Eintrag. Bei 100.000 Einträgen ist der Unterschied erheblich, und mehr
    /// braucht der Vergleich nicht.
    pub fn bestandsuebersicht(&self) -> Ergebnis<std::collections::HashMap<String, Bestandszeile>> {
        self.zaehle();
        let mut stmt = self
            .conn
            .prepare(
                "SELECT id, uid, pfad, groesse_bytes, datei_geaendert_am, vermisst_seit
                 FROM eintrag",
            )
            .map_err(db_fehler)?;
        let zeilen = stmt
            .query_map([], |z| {
                Ok(Bestandszeile {
                    id: z.get(0)?,
                    uid: z.get(1)?,
                    pfad: z.get(2)?,
                    groesse_bytes: z.get(3)?,
                    datei_geaendert_am: z.get(4)?,
                    vermisst: z.get::<_, Option<i64>>(5)?.is_some(),
                })
            })
            .map_err(db_fehler)?;

        let mut nach_pfad = std::collections::HashMap::new();
        for zeile in zeilen {
            let zeile = zeile.map_err(db_fehler)?;
            nach_pfad.insert(zeile.pfad.clone(), zeile);
        }
        Ok(nach_pfad)
    }

    /// Schreibt einen geänderten Eintrag fort.
    ///
    /// **Die Kennung bleibt** (SM-LIB-010): Schlagworte, Notizen und Status
    /// hängen an ihr. Ein Löschen-und-Neuanlegen verlöre genau das, was die
    /// Nutzerin gepflegt hat.
    pub fn eintrag_fortschreiben(&mut self, id: i64, neu: &NeuerEintrag) -> Ergebnis<()> {
        let tx = self.conn.transaction().map_err(db_fehler)?;
        tx.execute(
            "UPDATE eintrag SET
                 pfad = ?1, dateiname = ?2, format = ?3, groesse_bytes = ?4,
                 inhalt_hash = ?5, breite_mm = ?6, hoehe_mm = ?7, stichzahl = ?8,
                 farbzahl = ?9, datei_geaendert_am = ?10, geaendert_am = ?11,
                 vermisst_seit = NULL
             WHERE id = ?12",
            params![
                neu.pfad.to_string_lossy(),
                neu.dateiname,
                neu.format.marke(),
                neu.groesse_bytes,
                neu.inhalt_hash,
                neu.breite_mm,
                neu.hoehe_mm,
                neu.stichzahl,
                neu.farbzahl,
                neu.datei_geaendert_am,
                jetzt(),
                id,
            ],
        )
        .map_err(db_fehler)?;

        // Die Garnfarben stammen aus der Datei und werden ersetzt; die
        // Schlagworte stammen von der Nutzerin und bleiben unberührt.
        tx.execute("DELETE FROM garnfarbe WHERE eintrag_id = ?1", params![id])
            .map_err(db_fehler)?;
        {
            let mut einf = tx
                .prepare(
                    "INSERT INTO garnfarbe
                         (eintrag_id, reihenfolge, hex, name, marke, markenschluessel)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )
                .map_err(db_fehler)?;
            for (i, f) in neu.farben.iter().enumerate() {
                einf.execute(params![
                    id,
                    i as i64,
                    f.hex,
                    f.name,
                    f.marke,
                    f.markenschluessel
                ])
                .map_err(db_fehler)?;
            }
        }
        tx.commit().map_err(db_fehler)?;
        Ok(())
    }

    /// Kennzeichnet Einträge als vermisst — sie werden **nicht** gelöscht.
    ///
    /// Löschen wäre nach SM-DAT-003 bestätigungspflichtig und nähme SM-DAT-004
    /// (Papierkorb) vorweg. Die gepflegten Metadaten bleiben erhalten.
    pub fn als_vermisst_kennzeichnen(&mut self, ids: &[i64]) -> Ergebnis<usize> {
        if ids.is_empty() {
            return Ok(0);
        }
        let tx = self.conn.transaction().map_err(db_fehler)?;
        let jetzt = jetzt();
        let mut gezaehlt = 0usize;
        {
            let mut stmt = tx
                .prepare(
                    "UPDATE eintrag SET vermisst_seit = ?1
                     WHERE id = ?2 AND vermisst_seit IS NULL",
                )
                .map_err(db_fehler)?;
            for id in ids {
                gezaehlt += stmt.execute(params![jetzt, id]).map_err(db_fehler)?;
            }
        }
        tx.commit().map_err(db_fehler)?;
        Ok(gezaehlt)
    }

    /// Hebt die Vermisstmarke auf — die Datei ist wieder da.
    pub fn vermisst_aufheben(&mut self, ids: &[i64]) -> Ergebnis<usize> {
        if ids.is_empty() {
            return Ok(0);
        }
        let tx = self.conn.transaction().map_err(db_fehler)?;
        let mut gezaehlt = 0usize;
        {
            let mut stmt = tx
                .prepare("UPDATE eintrag SET vermisst_seit = NULL WHERE id = ?1")
                .map_err(db_fehler)?;
            for id in ids {
                gezaehlt += stmt.execute(params![id]).map_err(db_fehler)?;
            }
        }
        tx.commit().map_err(db_fehler)?;
        Ok(gezaehlt)
    }

    /// Nimmt viele Einträge in **einem** Vorgang auf.
    ///
    /// Ein Vorgang je Datei kostet bei 100.000 Einträgen 100.000 Festschreibungen
    /// und ist damit für den Erstimport unbrauchbar (SM-NFR-001). Die
    /// vorbereitete Anweisung wird wiederverwendet, nicht je Zeile neu
    /// übersetzt.
    /// Gibt `(aufgenommen, abgewiesen)` zurück.
    ///
    /// Die Zahl der Abweisungen war zuvor nur ein `log::info!` — die Nutzerin
    /// sah 9.963 Kacheln statt 10.000 und erfuhr nie, dass 37 fehlen
    /// (SM-IMP-009).
    pub fn eintraege_anlegen(
        &mut self,
        neue: &[NeuerEintrag],
    ) -> Ergebnis<(usize, Vec<Abweisung>)> {
        let tx = self.conn.transaction().map_err(db_fehler)?;
        let jetzt = jetzt();
        let mut gezaehlt = 0usize;
        let mut abgewiesen: Vec<Abweisung> = Vec::new();
        {
            let mut einf = tx
                .prepare(
                    "INSERT INTO eintrag (
                         uid, pfad, dateiname, format, groesse_bytes, inhalt_hash,
                         breite_mm, hoehe_mm, stichzahl, farbzahl, name,
                         importiert_am, datei_geaendert_am, geaendert_am,
                         fehlerstatus, fehlergrund)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14,
                             ?15, ?16)",
                )
                .map_err(db_fehler)?;
            let mut farbe_einf = tx
                .prepare(
                    "INSERT INTO garnfarbe
                         (eintrag_id, reihenfolge, hex, name, marke, markenschluessel)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                )
                .map_err(db_fehler)?;

            for neu in neue {
                // Ein Konflikt überspringt den Eintrag, statt den Block zu
                // verwerfen. Getragen wird das von der Vorgabe-Konfliktbehandlung
                // `ABORT`, die **nur die Anweisung** zurücknimmt, nicht die
                // Transaktion — und von der Übersprungbehandlung unten, die
                // Eintrag **und** Garnfarben gemeinsam betrifft. SM-IMP-009
                // verlangt, dass eine problematische Datei den Lauf nicht
                // abbricht (bis zu 10.000 Einträge je Block).
                // Dieselbe Vereindeutigung wie im Einzelweg. Ohne sie fiele
                // eine inhaltsgleiche Datei still weg **und** würde bei jedem
                // Lauf erneut gelesen — und zwei Aufnahmewege lieferten für
                // denselben Bestand verschiedene Ergebnisse.
                let uid = match eindeutige_kennung(&tx, &neu.uid) {
                    Ok(u) => u,
                    Err(e) => {
                        log::warn!("Kennung nicht vereindeutigt: {e}");
                        abgewiesen.push(Abweisung {
                            dateiname: neu.dateiname.clone(),
                            grund: "Für diese Datei ließ sich keine eindeutige Kennung bilden."
                                .to_string(),
                        });
                        continue;
                    }
                };
                let ergebnis = einf.execute(params![
                    uid,
                    neu.pfad.to_string_lossy(),
                    neu.dateiname,
                    neu.format.marke(),
                    neu.groesse_bytes,
                    neu.inhalt_hash,
                    neu.breite_mm,
                    neu.hoehe_mm,
                    neu.stichzahl,
                    neu.farbzahl,
                    neu.name,
                    jetzt,
                    neu.datei_geaendert_am,
                    jetzt,
                    neu.fehlerstatus,
                    neu.fehlergrund,
                ]);
                if let Err(e) = ergebnis {
                    // Der Eintrag wird übersprungen, nicht der Block verworfen.
                    // `warn`, nicht `debug`: Eine abgewiesene Datei ist ein
                    // Vorgang, den die Nutzerin erfahren soll.
                    log::warn!("Eintrag nicht aufgenommen: {e}");
                    abgewiesen.push(Abweisung {
                        dateiname: neu.dateiname.clone(),
                        grund: "Der Eintrag konnte nicht angelegt werden.".to_string(),
                    });
                    continue;
                }
                let id = tx.last_insert_rowid();
                for (i, f) in neu.farben.iter().enumerate() {
                    farbe_einf
                        .execute(params![
                            id,
                            i as i64,
                            f.hex,
                            f.name,
                            f.marke,
                            f.markenschluessel
                        ])
                        .map_err(db_fehler)?;
                }
                gezaehlt += 1;
            }
        }
        tx.commit().map_err(db_fehler)?;
        Ok((gezaehlt, abgewiesen))
    }

    /// Setzt die Schlagworte eines Eintrags und zieht die Suchspalte nach.
    pub fn schlagworte_setzen(&mut self, eintrag_id: i64, worte: &[String]) -> Ergebnis<()> {
        let tx = self.conn.transaction().map_err(db_fehler)?;
        tx.execute(
            "DELETE FROM eintrag_schlagwort WHERE eintrag_id = ?1",
            params![eintrag_id],
        )
        .map_err(db_fehler)?;

        for wort in worte {
            let wort = wort.trim();
            if wort.is_empty() {
                continue;
            }
            tx.execute(
                "INSERT OR IGNORE INTO schlagwort (name) VALUES (?1)",
                params![wort],
            )
            .map_err(db_fehler)?;
            let sid: i64 = tx
                .query_row(
                    "SELECT id FROM schlagwort WHERE name = ?1",
                    params![wort],
                    |z| z.get(0),
                )
                .map_err(db_fehler)?;
            tx.execute(
                "INSERT OR IGNORE INTO eintrag_schlagwort (eintrag_id, schlagwort_id)
                 VALUES (?1, ?2)",
                params![eintrag_id, sid],
            )
            .map_err(db_fehler)?;
        }

        let zusammen = worte.join(" ");
        tx.execute(
            "UPDATE eintrag SET schlagworte_text = ?1, geaendert_am = ?2 WHERE id = ?3",
            params![zusammen, jetzt(), eintrag_id],
        )
        .map_err(db_fehler)?;

        tx.commit().map_err(db_fehler)?;
        Ok(())
    }

    /// Liefert einen Ausschnitt der Treffermenge (Schnittregel 3 und 4).
    ///
    /// `mit_gesamtzahl` wird **nur beim ersten Ausschnitt eines Suchlaufs**
    /// gesetzt. Die Zahl der Abfragen ist von der Zeilenzahl unabhängig: eine
    /// Abfrage für den Ausschnitt, höchstens eine für die Gesamtzahl.
    pub fn suche(
        &self,
        abfrage: &Suchabfrage,
        versatz: i64,
        anzahl: i64,
        mit_gesamtzahl: bool,
    ) -> Ergebnis<Ausschnitt> {
        let anzahl = anzahl.clamp(1, 500);
        let versatz = versatz.max(0);

        let (wo, werte) = self.bedingungen(abfrage);

        // Der Rumpf setzt sich ausschließlich aus festen Textstücken dieses
        // Programms zusammen; jeder Fremdwert steht als Platzhalter darin
        // (SM-SEC-005).
        let volltext = abfrage.text.as_deref().and_then(quotiere_fts);

        // Liegt ein Volltextausdruck vor, wird die Treffermenge des Index
        // zuerst gebildet und festgehalten (`MATERIALIZED`). Der Volltextindex
        // ist die weitaus engste Quelle; alles Weitere filtert nur noch.
        let kopf = if volltext.is_some() {
            "WITH treffer AS MATERIALIZED (
                 SELECT rowid AS id, rank AS rang FROM eintrag_fts
                 WHERE eintrag_fts MATCH ?1
             ) "
        } else {
            ""
        };
        let quelle = if volltext.is_some() {
            "eintrag e JOIN treffer t ON t.id = e.id"
        } else {
            "eintrag e"
        };

        let sortierung = if abfrage.sortierung == Sortierung::Relevanz && volltext.is_none() {
            // Ohne Volltext gibt es keinen Rang, nach dem sich sortieren ließe.
            Sortierung::Name
        } else {
            abfrage.sortierung
        };

        let rang_spalte = if volltext.is_some() {
            ", t.rang AS rang"
        } else {
            ""
        };

        // Eingesetzt werden ausschließlich feste Textstücke dieses Programms:
        // `kopf`, `quelle` und `wo` bestehen aus Literalen und
        // Platzhalternummern, `klausel()` liefert eine feste Zeichenkette aus
        // einem Aufzählungstyp.
        // SM-SEC-005-Ausnahme: jeder Fremdwert steht als `?N` und wird gebunden.
        let sql = format!(
            "{kopf}SELECT e.id, e.uid, e.dateiname, e.name, e.format,
                    e.breite_mm, e.hoehe_mm, e.pfad, e.fehlergrund{rang_spalte}
             FROM {quelle}
             {wo}
             ORDER BY {}
             LIMIT ?{} OFFSET ?{}",
            sortierung.klausel(),
            werte.len() + 1,
            werte.len() + 2,
        );

        let mut alle: Vec<rusqlite::types::Value> = werte.clone();
        alle.push(anzahl.into());
        alle.push(versatz.into());

        self.zaehle();
        // SM-SEC-005-Ausnahme: `sql` entsteht oben aus festen Textstücken; alle Werte stehen als `?N`.
        let mut stmt = self.conn.prepare(&sql).map_err(db_fehler)?;
        let zeilen = stmt
            .query_map(params_from_iter(alle.iter()), |z| {
                Ok(Trefferzeile {
                    id: z.get(0)?,
                    uid: z.get(1)?,
                    dateiname: z.get(2)?,
                    name: z.get(3)?,
                    format: z.get(4)?,
                    breite_mm: z.get(5)?,
                    hoehe_mm: z.get(6)?,
                    pfad: z.get(7)?,
                    fehlergrund: z.get(8)?,
                })
            })
            .map_err(db_fehler)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(db_fehler)?;

        let gesamt = if mit_gesamtzahl {
            self.zaehle();
            // Dieselben festen Textstücke wie oben.
            // SM-SEC-005-Ausnahme: die Werte kommen über `params_from_iter` an die Platzhalter.
            let zaehl_sql = format!("{kopf}SELECT COUNT(*) FROM {quelle} {wo}");
            let n: i64 = self
                .conn
                // SM-SEC-005-Ausnahme: `zaehl_sql` entsteht aus denselben festen Textstücken wie oben.
                .query_row(&zaehl_sql, params_from_iter(werte.iter()), |z| z.get(0))
                .map_err(db_fehler)?;
            Some(n)
        } else {
            None
        };

        Ok(Ausschnitt {
            zeilen,
            gesamt,
            versatz,
        })
    }

    /// Baut die WHERE-Klausel aus festen Textstücken und sammelt die Werte.
    fn bedingungen(&self, a: &Suchabfrage) -> (String, Vec<rusqlite::types::Value>) {
        let mut teile: Vec<String> = Vec::new();
        let mut werte: Vec<rusqlite::types::Value> = Vec::new();

        // Der Volltextausdruck steht bewusst **nicht** hier: Er treibt die
        // Abfrage aus dem gemeinsamen Tafelausdruck in `suche` heraus. Als
        // gewöhnliche WHERE-Bedingung wählt SQLite bei zusätzlichen Filtern den
        // Namensindex als Zugriffsweg und prüft den Volltextindex je Zeile —
        // gemessen 698 ms gegen 5 ms bei 100.000 Einträgen.
        // Der Platzhalter bleibt an erster Stelle (?1), damit die Nummerierung
        // beider Fassungen übereinstimmt.
        if let Some(q) = a.text.as_deref().and_then(quotiere_fts) {
            werte.push(q.into());
        }

        if !a.formate.is_empty() {
            let mut platzhalter = Vec::new();
            for f in &a.formate {
                werte.push(f.marke().to_string().into());
                platzhalter.push(format!("?{}", werte.len()));
            }
            teile.push(format!("e.format IN ({})", platzhalter.join(", ")));
        }

        for wort in &a.schlagworte {
            werte.push(wort.clone().into());
            // Eingesetzt wird die **Nummer** des Platzhalters, nicht das Schlagwort.
            // SM-SEC-005-Ausnahme: der Wert steht bereits in `werte` und wird gebunden.
            teile.push(format!(
                "EXISTS (SELECT 1 FROM eintrag_schlagwort es
                          JOIN schlagwort s ON s.id = es.schlagwort_id
                         WHERE es.eintrag_id = e.id AND s.name = ?{})",
                werte.len()
            ));
        }

        let spanne = |spalte: &str,
                      von: Option<i64>,
                      bis: Option<i64>,
                      teile: &mut Vec<String>,
                      werte: &mut Vec<rusqlite::types::Value>| {
            if let Some(v) = von {
                werte.push(v.into());
                teile.push(format!("{spalte} >= ?{}", werte.len()));
            }
            if let Some(b) = bis {
                werte.push(b.into());
                teile.push(format!("{spalte} <= ?{}", werte.len()));
            }
        };
        spanne(
            "e.stichzahl",
            a.stichzahl_von,
            a.stichzahl_bis,
            &mut teile,
            &mut werte,
        );
        spanne(
            "e.farbzahl",
            a.farbzahl_von,
            a.farbzahl_bis,
            &mut teile,
            &mut werte,
        );

        if a.nur_favoriten {
            teile.push("e.favorit = 1".to_string());
        }

        let wo = if teile.is_empty() {
            String::new()
        } else {
            // `teile` enthält ausschließlich feste Textstücke dieses Programms
            // mit Platzhalternummern.
            // SM-SEC-005-Ausnahme: jeder Fremdwert liegt in `werte` und wird gebunden.
            format!("WHERE {}", teile.join(" AND "))
        };
        (wo, werte)
    }

    /// Die Garnfarben eines Eintrags (SM-MET-010).
    pub fn garnfarben(&self, eintrag_id: i64) -> Ergebnis<Vec<Garnfarbe>> {
        self.zaehle();
        let mut stmt = self
            .conn
            .prepare(
                "SELECT hex, name, marke, markenschluessel
                 FROM garnfarbe WHERE eintrag_id = ?1 ORDER BY reihenfolge",
            )
            .map_err(db_fehler)?;
        let farben = stmt
            .query_map(params![eintrag_id], |z| {
                Ok(Garnfarbe {
                    hex: z.get(0)?,
                    name: z.get(1)?,
                    marke: z.get(2)?,
                    markenschluessel: z.get(3)?,
                })
            })
            .map_err(db_fehler)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(db_fehler)?;
        Ok(farben)
    }

    /// Liest den vollständigen Detaildatensatz über die dauerhafte Kennung.
    ///
    /// Dieser gezielte Abruf läuft nur für die Auswahl, nie für jede Kachel.
    /// Hauptzeile, Schlagworte und Garnfarben kosten zusammen konstant drei
    /// parametrisierte Abfragen, unabhängig von der Treffermenge.
    pub fn detail_nach_uid(&self, uid: &str) -> Ergebnis<Option<Detailzeile>> {
        self.zaehle();
        let gefunden = self
            .conn
            .query_row(
                "SELECT id, uid, name,
                        NULLIF(thema, ''), NULLIF(beschreibung, ''), NULLIF(notizen, ''),
                        format, breite_mm, hoehe_mm, stichzahl, farbzahl,
                        fehlerstatus, fehlergrund
                 FROM eintrag WHERE uid = ?1",
                params![uid],
                |z| {
                    Ok((
                        z.get::<_, i64>(0)?,
                        Detailzeile {
                            uid: z.get(1)?,
                            name: z.get(2)?,
                            thema: z.get(3)?,
                            beschreibung: z.get(4)?,
                            notizen: z.get(5)?,
                            format: z.get(6)?,
                            breite_mm: z.get(7)?,
                            hoehe_mm: z.get(8)?,
                            stichzahl: z.get(9)?,
                            farbzahl: z.get(10)?,
                            fehlerstatus: z.get(11)?,
                            fehlergrund: z.get(12)?,
                            schlagworte: Vec::new(),
                            garnfarben: Vec::new(),
                        },
                    ))
                },
            )
            .optional()
            .map_err(db_fehler)?;

        let Some((id, mut detail)) = gefunden else {
            return Ok(None);
        };

        self.zaehle();
        let mut stmt = self
            .conn
            .prepare(
                "SELECT s.name
                 FROM schlagwort s
                 JOIN eintrag_schlagwort es ON es.schlagwort_id = s.id
                 WHERE es.eintrag_id = ?1
                 ORDER BY s.name COLLATE NOCASE, s.id",
            )
            .map_err(db_fehler)?;
        detail.schlagworte = stmt
            .query_map(params![id], |z| z.get(0))
            .map_err(db_fehler)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(db_fehler)?;
        drop(stmt);

        detail.garnfarben = self.garnfarben(id)?;
        Ok(Some(detail))
    }

    /// Garnfarben **aller** Zeilen eines Ausschnitts in **einer** Abfrage.
    ///
    /// Schnittregel 4: Garnfarben werden mit dem Ausschnitt geliefert, nicht je
    /// Zeile nachgefragt. Ein Aufruf je Zeile wäre genau die N+1-Abfrage, die
    /// SM-SRC-007 bei 100.000 Einträgen reißt.
    pub fn garnfarben_mehrerer(
        &self,
        ids: &[i64],
    ) -> Ergebnis<std::collections::HashMap<i64, Vec<Garnfarbe>>> {
        let mut nach_eintrag: std::collections::HashMap<i64, Vec<Garnfarbe>> =
            std::collections::HashMap::new();
        if ids.is_empty() {
            return Ok(nach_eintrag);
        }

        // Die Platzhalterliste wächst mit der Ausschnittgröße, nicht die Zahl
        // der Abfragen. Werte stehen ausschließlich als Platzhalter darin.
        let platzhalter = (1..=ids.len())
            .map(|i| format!("?{i}"))
            .collect::<Vec<_>>()
            .join(", ");
        // `platzhalter` ist eine erzeugte Liste der Form `?1, ?2, …` —
        // Platzhalternummern, keine Werte.
        // SM-SEC-005-Ausnahme: die Kennungen selbst werden gebunden.
        let sql = format!(
            "SELECT eintrag_id, hex, name, marke, markenschluessel
             FROM garnfarbe WHERE eintrag_id IN ({platzhalter})
             ORDER BY eintrag_id, reihenfolge"
        );

        self.zaehle();
        // SM-SEC-005-Ausnahme: `sql` trägt nur die erzeugte Platzhalterliste; die Kennungen werden gebunden.
        let mut stmt = self.conn.prepare(&sql).map_err(db_fehler)?;
        let zeilen = stmt
            .query_map(params_from_iter(ids.iter()), |z| {
                Ok((
                    z.get::<_, i64>(0)?,
                    Garnfarbe {
                        hex: z.get(1)?,
                        name: z.get(2)?,
                        marke: z.get(3)?,
                        markenschluessel: z.get(4)?,
                    },
                ))
            })
            .map_err(db_fehler)?;

        for zeile in zeilen {
            let (id, farbe) = zeile.map_err(db_fehler)?;
            nach_eintrag.entry(id).or_default().push(farbe);
        }
        Ok(nach_eintrag)
    }

    /// Gesamtzahl der Einträge — für die Statusleiste.
    pub fn bestandsgroesse(&self) -> Ergebnis<i64> {
        self.zaehle();
        self.conn
            .query_row("SELECT COUNT(*) FROM eintrag", [], |z| z.get(0))
            .map_err(db_fehler)
    }

    /// Findet einen Eintrag über seine dauerhafte Kennung (SM-LIB-010).
    pub fn eintrag_nach_uid(&self, uid: &str) -> Ergebnis<Option<i64>> {
        self.zaehle();
        self.conn
            .query_row("SELECT id FROM eintrag WHERE uid = ?1", params![uid], |z| {
                z.get(0)
            })
            .optional()
            .map_err(db_fehler)
    }

    /// Schreibt einen neuen Pfad fort, ohne die Kennung zu berühren
    /// (SM-LIB-010: die Kennung übersteht Umbenennung und Verschiebung).
    pub fn pfad_fortschreiben(
        &self,
        uid: &str,
        neuer_pfad: &Path,
        dateiname: &str,
    ) -> Ergebnis<()> {
        let betroffen = self
            .conn
            .execute(
                "UPDATE eintrag SET pfad = ?1, dateiname = ?2, geaendert_am = ?3 WHERE uid = ?4",
                params![neuer_pfad.to_string_lossy(), dateiname, jetzt(), uid],
            )
            .map_err(db_fehler)?;
        if betroffen == 0 {
            return Err(Fehler::NichtGefunden(
                "Der Eintrag ist nicht mehr vorhanden.".into(),
            ));
        }
        Ok(())
    }
}

/// Macht eine Kennung eindeutig, falls sie schon vergeben ist.
///
/// `kennung()` bildet sie aus Dateiname, Länge und den ersten Bytes — zwei
/// **inhaltsgleiche** Dateien in verschiedenen Ordnern bekommen dieselbe. Die
/// Kennung bleibt dauerhaft (SM-LIB-010); dass zwei Dateien denselben Inhalt
/// haben, bleibt über `inhalt_hash` erkennbar (SM-IMP-005).
fn eindeutige_kennung(tx: &rusqlite::Transaction<'_>, vorschlag: &str) -> Ergebnis<String> {
    // **Eine** Abfrage für den Startwert, nicht eine je Versuch.
    //
    // Aufsteigend zu probieren kostet für die k-te inhaltsgleiche Datei k
    // Abfragen — über den Bestand also O(k²). Bei 1.000 Kopien desselben
    // Musters, in Stickbibliotheken durch Sicherungsordner alltäglich, sind das
    // rund 500.000 Indexabfragen allein für die Kennungsvergabe (SM-LIB-009,
    // SM-NFR-001).
    //
    // Die Zählung liefert den Startwert unmittelbar. Sie kann zu niedrig
    // liegen, wenn zwischendurch gelöscht wurde; die Schleife darunter fängt
    // das ab und läuft dann typischerweise einen Schritt.
    let belegt: i64 = tx
        .query_row(
            "SELECT COUNT(*) FROM eintrag WHERE uid = ?1 OR uid GLOB ?2",
            params![vorschlag, format!("{vorschlag}-*")],
            |z| z.get(0),
        )
        .map_err(db_fehler)?;

    if belegt == 0 {
        return Ok(vorschlag.to_string());
    }

    let mut vorhanden = tx
        .prepare("SELECT 1 FROM eintrag WHERE uid = ?1")
        .map_err(db_fehler)?;
    let mut lauf = belegt as u64;
    loop {
        let uid = format!("{vorschlag}-{lauf}");
        if !vorhanden.exists(params![uid]).map_err(db_fehler)? {
            return Ok(uid);
        }
        lauf += 1;
        if lauf > belegt as u64 + 10_000 {
            return Err(Fehler::Intern(
                "Für diese Datei ließ sich keine eindeutige Kennung bilden.".into(),
            ));
        }
    }
}

fn jetzt() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests;
