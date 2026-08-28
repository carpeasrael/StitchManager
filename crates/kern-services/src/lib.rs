//! Kernbetrieb — der Arbeitsfaden, der die Fassade besitzt.
//!
//! Trägt SM-NFR-002 („Import, Indizierung und Vorschauerzeugung laufen im
//! Hintergrund und blockieren die Bedienung nicht"), SM-IMP-002
//! (Fortschrittsanzeige) und den Fortschritts- und Abbruchteil von SM-BAT-005.
//!
//! **Warum ein Fadenwechsel und keine Nachbesserung einzelner Aufrufe.** Die
//! Fassade hält eine SQLite-Verbindung. Die ist `Send`, aber nicht `Sync` — sie
//! lässt sich verschieben, nicht teilen. Bleibt sie im Qt-Faden, läuft jeder
//! Kernzugriff dort. Deshalb wechselt die Fassade den Faden und wird zum
//! alleinigen Eigentum dieses Betriebs.
//!
//! **Die Kiste kennt Qt nicht.** Der Rückkanal ist ein gewöhnlicher Verschluss.
//! Dadurch ist der gesamte Ablauf ohne Oberfläche prüfbar.

#![forbid(unsafe_code)]

use kern_fassade::{Abfrage, Fassade, Importbefund, Seite};
use kern_render::{Stufe, Vorschauoption, Zwischenspeicher};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};

/// Höchstzahl wartender Vorschauaufträge.
///
/// Wer schnell blättert, fordert Vorschauen für Kacheln an, die längst wieder
/// aus dem Bild sind. Ohne Grenze arbeitete der Faden minutenlang an Bildern,
/// die niemand mehr sieht.
const VORSCHAU_SCHLANGE_MAX: usize = 256;

/// Nach so vielen Dateien wird ein Fortschritt gemeldet.
///
/// Je Datei zu melden überflutete genau die Ereignisschlange, die frei zu
/// halten der Zweck dieser Kiste ist.
const FORTSCHRITT_BUENDEL: usize = 25;

/// Ein Befehl an den Kernbetrieb.
#[derive(Debug, Clone)]
pub enum Befehl {
    /// Setzt die Bibliothekswurzel; sie wird dabei geprüft (SM-SEC-003).
    WurzelSetzen(PathBuf),
    /// Liest alle Stickdateien unterhalb der Wurzel ein.
    BestandEinlesen,
    /// Holt einen Ausschnitt der Treffermenge.
    Ausschnitt {
        abfrage: Abfrage,
        versatz: i64,
        anzahl: i64,
        mit_gesamtzahl: bool,
    },
}

/// Ein Vorschauauftrag. Er liegt in einer eigenen, begrenzten Schlange.
#[derive(Debug, Clone)]
pub struct Vorschauauftrag {
    pub zeile: usize,
    pub uid: String,
    /// Ablageort der Quelldatei. Er kommt aus der Fassade und wird vor jedem
    /// Zugriff erneut gegen die Bibliothekswurzel geprüft (Schnittregel 5).
    pub pfad: String,
    /// Gewünschte Kantenlänge; sie wird auf eine feste Stufe aufgerundet.
    pub breite_px: u32,
}

/// Eine Antwort des Kernbetriebs.
#[derive(Debug)]
pub enum Antwort {
    WurzelGesetzt(PathBuf),
    /// Der Lauf hat begonnen; `gesamt` ist die Zahl gefundener Dateien.
    EinlesenBegonnen {
        gesamt: usize,
    },
    Fortschritt {
        erledigt: usize,
        gesamt: usize,
    },
    EinlesenFertig {
        befund: Importbefund,
        abgebrochen: bool,
    },
    Seite(Box<Seite>),
    VorschauFertig {
        zeile: usize,
        uid: String,
        pfad: PathBuf,
    },
    /// Ein für Endnutzer verständlicher Fehlertext (SM-NFR-006).
    Fehler(String),
}

/// Aufträge, die der Arbeitsfaden abarbeitet.
struct Schlange {
    befehle: VecDeque<Befehl>,
    vorschauen: VecDeque<Vorschauauftrag>,
    beenden: bool,
}

struct Geteilt {
    schlange: Mutex<Schlange>,
    wecker: Condvar,
}

/// Der Kernbetrieb.
pub struct Kernbetrieb {
    geteilt: Arc<Geteilt>,
    abbruch: Arc<AtomicBool>,
    faden: Option<std::thread::JoinHandle<()>>,
}

impl Kernbetrieb {
    /// Startet den Arbeitsfaden. Die Fassade geht dabei in dessen Eigentum über.
    ///
    /// `melden` wird **aus dem Arbeitsfaden** aufgerufen. Die Oberfläche reicht
    /// die Antwort von dort in ihren eigenen Faden weiter.
    /// `speicher` ist der Ablageort der Vorschauen. Er wird **übergeben**,
    /// nicht hier gewählt: Sonst schriebe jeder Prüffall in den dauerhaften
    /// Speicher der Nutzerin.
    pub fn starten<F>(fassade: Fassade, speicher: Option<Zwischenspeicher>, melden: F) -> Self
    where
        F: Fn(Antwort) + Send + 'static,
    {
        let geteilt = Arc::new(Geteilt {
            schlange: Mutex::new(Schlange {
                befehle: VecDeque::new(),
                vorschauen: VecDeque::new(),
                beenden: false,
            }),
            wecker: Condvar::new(),
        });
        let abbruch = Arc::new(AtomicBool::new(false));

        let faden = {
            let geteilt = Arc::clone(&geteilt);
            let abbruch = Arc::clone(&abbruch);
            std::thread::Builder::new()
                .name("stitchmanager-kern".into())
                .spawn(move || arbeiten(fassade, speicher, geteilt, abbruch, melden))
                .expect("Arbeitsfaden konnte nicht gestartet werden")
        };

        Self {
            geteilt,
            abbruch,
            faden: Some(faden),
        }
    }

    /// Stellt einen Befehl ein.
    pub fn beauftragen(&self, befehl: Befehl) {
        let mut s = self
            .geteilt
            .schlange
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        s.befehle.push_back(befehl);
        drop(s);
        self.geteilt.wecker.notify_one();
    }

    /// Stellt einen Vorschauauftrag ein.
    ///
    /// Läuft die Schlange über, fällt der **älteste** Auftrag heraus: Er ist am
    /// wahrscheinlichsten nicht mehr sichtbar. Er wird neu gestellt, sobald die
    /// Zeile wieder erfragt wird.
    pub fn vorschau_anfordern(&self, auftrag: Vorschauauftrag) {
        let mut s = self
            .geteilt
            .schlange
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        if s.vorschauen.len() >= VORSCHAU_SCHLANGE_MAX {
            s.vorschauen.pop_front();
        }
        s.vorschauen.push_back(auftrag);
        drop(s);
        self.geteilt.wecker.notify_one();
    }

    /// Bricht einen laufenden Importlauf ab (SM-BAT-005).
    pub fn abbrechen(&self) {
        self.abbruch.store(true, Ordering::Relaxed);
    }

    /// Zahl der wartenden Vorschauaufträge — für Prüffälle und Messungen.
    pub fn wartende_vorschauen(&self) -> usize {
        self.geteilt
            .schlange
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .vorschauen
            .len()
    }
}

impl Drop for Kernbetrieb {
    fn drop(&mut self) {
        {
            let mut s = self
                .geteilt
                .schlange
                .lock()
                .unwrap_or_else(|e| e.into_inner());
            s.beenden = true;
        }
        // Ein laufender Importlauf soll das Beenden nicht aufhalten.
        self.abbruch.store(true, Ordering::Relaxed);
        self.geteilt.wecker.notify_all();
        if let Some(f) = self.faden.take() {
            let _ = f.join();
        }
    }
}

enum Arbeit {
    Befehl(Befehl),
    Vorschau(Vorschauauftrag),
}

/// Die Schleife des Arbeitsfadens.
fn arbeiten<F>(
    mut fassade: Fassade,
    speicher: Option<Zwischenspeicher>,
    geteilt: Arc<Geteilt>,
    abbruch: Arc<AtomicBool>,
    melden: F,
) where
    F: Fn(Antwort) + Send + 'static,
{
    loop {
        let arbeit = {
            let mut s = geteilt.schlange.lock().unwrap_or_else(|e| e.into_inner());
            loop {
                if s.beenden {
                    return;
                }
                // Vorrang: erst ein Befehl, dann eine Vorschau. Sonst
                // verzögerte ein Schwall von Vorschauen den nächsten
                // Ausschnittabruf und der Bildlauf ruckelte.
                if let Some(b) = s.befehle.pop_front() {
                    break Arbeit::Befehl(b);
                }
                // Von hinten: zuletzt angefordert heißt zuletzt sichtbar.
                if let Some(v) = s.vorschauen.pop_back() {
                    break Arbeit::Vorschau(v);
                }
                s = geteilt.wecker.wait(s).unwrap_or_else(|e| e.into_inner());
            }
        };

        match arbeit {
            Arbeit::Befehl(b) => befehl_ausfuehren(
                &mut fassade,
                b,
                speicher.as_ref(),
                &abbruch,
                &geteilt,
                &melden,
            ),
            Arbeit::Vorschau(v) => vorschau_erzeugen(&fassade, v, speicher.as_ref(), &melden),
        }
    }
}

fn befehl_ausfuehren<F>(
    fassade: &mut Fassade,
    befehl: Befehl,
    speicher: Option<&Zwischenspeicher>,
    abbruch: &AtomicBool,
    geteilt: &Geteilt,
    melden: &F,
) where
    F: Fn(Antwort),
{
    match befehl {
        Befehl::WurzelSetzen(pfad) => match fassade.wurzel_setzen(&pfad) {
            Ok(()) => melden(Antwort::WurzelGesetzt(pfad)),
            Err(e) => melden(Antwort::Fehler(e.to_string())),
        },

        Befehl::BestandEinlesen => {
            let Some(wurzel) = fassade.wurzel().map(|w| w.pfad().to_path_buf()) else {
                melden(Antwort::Fehler(
                    "Wählen Sie zuerst eine Bibliothek.".to_string(),
                ));
                return;
            };

            abbruch.store(false, Ordering::Relaxed);

            let dateien = sammle_stickdateien(&wurzel);

            // Zuerst ermitteln, was überhaupt zu tun ist — ohne eine Datei zu
            // lesen (SM-IMP-003).
            let satz = match fassade.aenderungen_ermitteln(&dateien) {
                Ok(s) => s,
                Err(e) => {
                    melden(Antwort::Fehler(e.to_string()));
                    return;
                }
            };

            let mut befund = Importbefund {
                unveraendert: satz.unveraendert,
                ..Default::default()
            };

            // Der Fortschritt zählt die **Arbeit**, nicht die Fundmenge. Ein
            // Lauf über 100.000 unveränderte Dateien meldet „0 von 0" — und das
            // ist die richtige Aussage.
            let gesamt = satz.zu_lesen();
            melden(Antwort::EinlesenBegonnen { gesamt });

            let mut abgebrochen = false;
            let mut erledigt = 0usize;

            for datei in &satz.neu {
                if abbruch.load(Ordering::Relaxed) {
                    abgebrochen = true;
                    break;
                }
                match fassade.aufnehmen(datei) {
                    Ok(_) => befund.neu += 1,
                    // Eine defekte Datei hält den Lauf nicht an (SM-NFR-005).
                    Err(e) => {
                        log::debug!("Datei nicht aufgenommen: {e}");
                        befund.abgewiesen += 1;
                    }
                }
                erledigt += 1;
                if erledigt % FORTSCHRITT_BUENDEL == 0 {
                    melden(Antwort::Fortschritt { erledigt, gesamt });
                }
            }

            if !abgebrochen {
                for (datei, id, uid) in &satz.geaendert {
                    if abbruch.load(Ordering::Relaxed) {
                        abgebrochen = true;
                        break;
                    }
                    match fassade.erneuern(*id, datei) {
                        Ok(()) => {
                            befund.geaendert += 1;
                            // Die Änderungserkennung ist zugleich der Auslöser
                            // der Vorschau-Verwerfung (AP-08 → AP-09).
                            if let Some(sp) = speicher {
                                sp.verwerfen(uid);
                            }
                        }
                        Err(e) => {
                            log::debug!("Datei nicht fortgeschrieben: {e}");
                            befund.abgewiesen += 1;
                        }
                    }
                    erledigt += 1;
                    if erledigt % FORTSCHRITT_BUENDEL == 0 {
                        melden(Antwort::Fortschritt { erledigt, gesamt });
                    }
                }
            }

            // Vermisste und wiedergefundene Einträge auch nach einem Abbruch
            // fortschreiben: Beides ist bereits ermittelt und kostet nichts.
            let vermisste: Vec<i64> = satz.vermisst.iter().map(|(id, _)| *id).collect();
            match fassade.vermisst_kennzeichnen(&vermisste) {
                Ok(n) => befund.vermisst = n,
                Err(e) => log::debug!("Vermisstmarke nicht gesetzt: {e}"),
            }
            // Eine vermisste Datei, die wieder auftaucht, verliert die Marke.
            match fassade.vermisst_aufheben(&satz.wiedergefunden) {
                Ok(n) => befund.wiedergefunden = n,
                Err(e) => log::debug!("Vermisstmarke nicht aufgehoben: {e}"),
            }

            melden(Antwort::Fortschritt { erledigt, gesamt });
            melden(Antwort::EinlesenFertig {
                befund,
                abgebrochen,
            });

            // Nach einem Lauf sind wartende Vorschauen überholt: Die Zeilen
            // werden neu geladen und neu angefordert.
            let mut s = geteilt.schlange.lock().unwrap_or_else(|e| e.into_inner());
            s.vorschauen.clear();
        }

        Befehl::Ausschnitt {
            abfrage,
            versatz,
            anzahl,
            mit_gesamtzahl,
        } => match fassade.ausschnitt(&abfrage, versatz, anzahl, mit_gesamtzahl) {
            Ok(seite) => melden(Antwort::Seite(Box::new(seite))),
            Err(e) => melden(Antwort::Fehler(e.to_string())),
        },
    }
}

fn vorschau_erzeugen<F>(
    fassade: &Fassade,
    auftrag: Vorschauauftrag,
    speicher: Option<&Zwischenspeicher>,
    melden: &F,
) where
    F: Fn(Antwort),
{
    // Der Pfad wird zuerst gegen die Bibliothekswurzel geprüft und aufgelöst
    // (Schnittregel 5). Erst danach darf er das Dateisystem berühren: Ein
    // relativer Pfad gälte sonst gegen das Arbeitsverzeichnis des Prozesses,
    // und die Gültigkeitsprüfung läse die Kennzeichen der falschen Datei.
    let Ok(quelle) = fassade.pfad_pruefen(&auftrag.pfad) else {
        log::debug!("Vorschau abgelehnt: Pfad außerhalb der Bibliothek");
        return;
    };
    let stufe = Stufe::fuer_breite(auftrag.breite_px);

    // Gültigkeitsprüfung gegen Größe und Änderungszeit der Quelldatei
    // (SM-PRV-003). Sie läuft hier, im Arbeitsfaden — nie im Zeichenpfad
    // (AP-09, AP-12, SM-NFR-002).
    if let Some(sp) = speicher {
        if let Some(pfad) = sp.holen(&auftrag.uid, &quelle, stufe) {
            melden(Antwort::VorschauFertig {
                zeile: auftrag.zeile,
                uid: auftrag.uid,
                pfad,
            });
            return;
        }
    }

    let opt = Vorschauoption {
        breite_px: stufe.kantenlaenge(),
        hoehe_px: stufe.kantenlaenge(),
        ..Default::default()
    };

    let daten = match fassade.vorschau(&quelle, opt) {
        Ok(d) => d,
        // Eine Kachel ohne Vorschau ist kein Fehler, den die Nutzerin sehen
        // muss; sie zeigt den leeren Zustand.
        Err(e) => {
            log::debug!("Vorschau nicht erzeugt: {e}");
            return;
        }
    };

    let Some(sp) = speicher else {
        return;
    };

    match sp.ablegen(&auftrag.uid, &quelle, stufe, &daten) {
        Ok(pfad) => melden(Antwort::VorschauFertig {
            zeile: auftrag.zeile,
            uid: auftrag.uid,
            pfad,
        }),
        Err(e) => log::debug!("Vorschau nicht abgelegt: {e}"),
    }
}

/// Sammelt alle Stickdateien unterhalb eines Verzeichnisses.
pub fn sammle_stickdateien(wurzel: &Path) -> Vec<PathBuf> {
    fn rekursiv(pfad: &Path, aus: &mut Vec<PathBuf>, tiefe: u32) {
        // Die Tiefenbegrenzung fängt Symlinkschleifen ab.
        if tiefe > 32 {
            return;
        }
        let Ok(eintraege) = std::fs::read_dir(pfad) else {
            return;
        };
        for e in eintraege.flatten() {
            let p = e.path();
            if p.is_dir() {
                rekursiv(&p, aus, tiefe + 1);
            } else if p
                .extension()
                .and_then(|x| x.to_str())
                .and_then(kern_typen::Format::aus_endung)
                .is_some()
            {
                aus.push(p);
            }
        }
    }
    let mut aus = Vec::new();
    rekursiv(wurzel, &mut aus, 0);
    aus.sort();
    aus
}

#[cfg(test)]
mod tests;
