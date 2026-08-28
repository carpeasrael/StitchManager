//! Additive, versionierte Schemaschritte (SM-DAT-007).
//!
//! **Ein einmal ausgelieferter Schritt wird nie wieder verändert.** Wer eine
//! Spalte braucht, hängt einen neuen Schritt an. Diese Zusage ist hier nicht
//! nur ein Kommentar: [`super::tests::bestehende_schritte_sind_unveraendert`]
//! prüft die Prüfsumme jedes ausgelieferten Schritts gegen einen festen Wert
//! und schlägt fehl, sobald jemand einen bestehenden Schritt anfasst.

/// Ein Schemaschritt. `nummer` ist aufsteigend und lückenlos.
pub struct Schritt {
    pub nummer: i64,
    pub name: &'static str,
    pub sql: &'static str,
}

/// Alle Schritte in Anwendungsreihenfolge.
pub const SCHRITTE: &[Schritt] = &[
    Schritt {
        nummer: 1,
        name: "grundgeruest",
        sql: r#"
CREATE TABLE eintrag (
    id                INTEGER PRIMARY KEY,
    -- Dauerhafte Kennung, die Umbenennung und Verschiebung übersteht
    -- (SM-LIB-010). Sie hängt nicht am Pfad.
    uid               TEXT    NOT NULL UNIQUE,
    pfad              TEXT    NOT NULL,
    dateiname         TEXT    NOT NULL,
    format            TEXT    NOT NULL,
    groesse_bytes     INTEGER NOT NULL DEFAULT 0,
    inhalt_hash       TEXT,
    breite_mm         REAL,
    hoehe_mm          REAL,
    stichzahl         INTEGER,
    farbzahl          INTEGER,
    name              TEXT    NOT NULL DEFAULT '',
    thema             TEXT    NOT NULL DEFAULT '',
    beschreibung      TEXT    NOT NULL DEFAULT '',
    notizen           TEXT    NOT NULL DEFAULT '',
    -- Denormalisierte Schlagwortfolge, damit die Volltextsuche eine
    -- Tabelle befragt statt einer Verknüpfung je Zeile (SM-SRC-007).
    schlagworte_text  TEXT    NOT NULL DEFAULT '',
    status            TEXT    NOT NULL DEFAULT 'nicht_begonnen',
    favorit           INTEGER NOT NULL DEFAULT 0,
    importiert_am     INTEGER NOT NULL,
    datei_geaendert_am INTEGER,
    geaendert_am      INTEGER NOT NULL
) STRICT;

CREATE UNIQUE INDEX idx_eintrag_pfad ON eintrag(pfad);
CREATE INDEX idx_eintrag_format     ON eintrag(format);
CREATE INDEX idx_eintrag_stichzahl  ON eintrag(stichzahl);
CREATE INDEX idx_eintrag_farbzahl   ON eintrag(farbzahl);
CREATE INDEX idx_eintrag_importiert ON eintrag(importiert_am);
CREATE INDEX idx_eintrag_name       ON eintrag(name);
CREATE INDEX idx_eintrag_favorit    ON eintrag(favorit) WHERE favorit = 1;
CREATE INDEX idx_eintrag_hash       ON eintrag(inhalt_hash);

CREATE TABLE schlagwort (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE eintrag_schlagwort (
    eintrag_id    INTEGER NOT NULL REFERENCES eintrag(id)    ON DELETE CASCADE,
    schlagwort_id INTEGER NOT NULL REFERENCES schlagwort(id) ON DELETE CASCADE,
    PRIMARY KEY (eintrag_id, schlagwort_id)
) STRICT;

CREATE INDEX idx_es_schlagwort ON eintrag_schlagwort(schlagwort_id);

CREATE TABLE garnfarbe (
    id               INTEGER PRIMARY KEY,
    eintrag_id       INTEGER NOT NULL REFERENCES eintrag(id) ON DELETE CASCADE,
    reihenfolge      INTEGER NOT NULL,
    hex              TEXT    NOT NULL,
    name             TEXT,
    marke            TEXT,
    markenschluessel TEXT,
    stichanteil      REAL
) STRICT;

CREATE INDEX idx_garnfarbe_eintrag ON garnfarbe(eintrag_id, reihenfolge);
"#,
    },
    Schritt {
        nummer: 2,
        name: "volltextindex",
        sql: r#"
-- Volltextindex über die Textfelder (SM-SRC-001). `content=` bindet den
-- Index an die Haupttabelle, damit die Texte nicht doppelt liegen.
CREATE VIRTUAL TABLE eintrag_fts USING fts5(
    name,
    thema,
    beschreibung,
    notizen,
    schlagworte_text,
    content='eintrag',
    content_rowid='id',
    tokenize='unicode61 remove_diacritics 2'
);

CREATE TRIGGER eintrag_fts_ein AFTER INSERT ON eintrag BEGIN
    INSERT INTO eintrag_fts(rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES (new.id, new.name, new.thema, new.beschreibung, new.notizen, new.schlagworte_text);
END;

CREATE TRIGGER eintrag_fts_weg AFTER DELETE ON eintrag BEGIN
    INSERT INTO eintrag_fts(eintrag_fts, rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES ('delete', old.id, old.name, old.thema, old.beschreibung, old.notizen, old.schlagworte_text);
END;

CREATE TRIGGER eintrag_fts_neu AFTER UPDATE ON eintrag BEGIN
    INSERT INTO eintrag_fts(eintrag_fts, rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES ('delete', old.id, old.name, old.thema, old.beschreibung, old.notizen, old.schlagworte_text);
    INSERT INTO eintrag_fts(rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES (new.id, new.name, new.thema, new.beschreibung, new.notizen, new.schlagworte_text);
END;
"#,
    },
    Schritt {
        nummer: 3,
        name: "ordner_und_sammlungen",
        sql: r#"
CREATE TABLE ordner (
    id        INTEGER PRIMARY KEY,
    pfad      TEXT    NOT NULL UNIQUE,
    name      TEXT    NOT NULL,
    eltern_id INTEGER REFERENCES ordner(id) ON DELETE CASCADE
) STRICT;

CREATE INDEX idx_ordner_eltern ON ordner(eltern_id);

CREATE TABLE sammlung (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE sammlung_eintrag (
    sammlung_id INTEGER NOT NULL REFERENCES sammlung(id) ON DELETE CASCADE,
    eintrag_id  INTEGER NOT NULL REFERENCES eintrag(id)  ON DELETE CASCADE,
    PRIMARY KEY (sammlung_id, eintrag_id)
) STRICT;
"#,
    },
    Schritt {
        nummer: 4,
        name: "vermisste_eintraege",
        sql: r#"
-- Ein Eintrag, dessen Quelldatei nicht mehr auffindbar ist, wird
-- gekennzeichnet statt gelöscht (SM-IMP-003). Löschen wäre nach SM-DAT-003
-- bestätigungspflichtig und nähme SM-DAT-004 (Papierkorb) vorweg; die
-- gepflegten Metadaten blieben dabei auf der Strecke.
ALTER TABLE eintrag ADD COLUMN vermisst_seit INTEGER;

CREATE INDEX idx_eintrag_vermisst ON eintrag(vermisst_seit)
    WHERE vermisst_seit IS NOT NULL;
"#,
    },
    Schritt {
        nummer: 5,
        name: "suchindizes_und_gezielter_auslöser",
        sql: r#"
-- Der Auslöser aus Schritt 2 feuerte bei **jeder** Spaltenänderung. Eine
-- Kennzeichnung als vermisst oder ein fortgeschriebener Pfad schrieb damit
-- den Volltextindex über fünf Textspalten neu, obwohl sich kein Suchtext
-- geändert hat: Bei 100.000 vermissten Einträgen sind das 200.000
-- Indexvorgänge ohne Gegenwert (SM-SRC-007, SM-IMP-003).
--
-- Schritt 2 bleibt unverändert (SM-DAT-007); der Auslöser wird hier ersetzt.
DROP TRIGGER IF EXISTS eintrag_fts_neu;

CREATE TRIGGER eintrag_fts_neu
AFTER UPDATE OF name, thema, beschreibung, notizen, schlagworte_text ON eintrag
BEGIN
    INSERT INTO eintrag_fts(eintrag_fts, rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES ('delete', old.id, old.name, old.thema, old.beschreibung, old.notizen, old.schlagworte_text);
    INSERT INTO eintrag_fts(rowid, name, thema, beschreibung, notizen, schlagworte_text)
    VALUES (new.id, new.name, new.thema, new.beschreibung, new.notizen, new.schlagworte_text);
END;

-- Sortierklauseln brauchen einen benutzbaren Index. `idx_eintrag_name` trägt
-- die Kollation BINARY, die Klausel verlangt NOCASE — SQLite baute deshalb je
-- Ausschnittabruf einen temporären Sortierer über die volle Treffermenge.
-- Die Richtung gehört in den Index: Die Klauseln sortieren absteigend nach
-- dem Wert und aufsteigend nach der Kennung. Ein rein aufsteigender Index
-- kann das nicht bedienen; gemessen kostete `Importdatum` am tiefsten Versatz
-- 175 ms gegen 2 ms bei passendem Index.
CREATE INDEX idx_eintrag_name_nocase   ON eintrag(name COLLATE NOCASE ASC, id ASC);
CREATE INDEX idx_eintrag_groesse_sort  ON eintrag(groesse_bytes DESC, id ASC);
CREATE INDEX idx_eintrag_datei_sort    ON eintrag(datei_geaendert_am DESC, id ASC);
CREATE INDEX idx_eintrag_import_sort   ON eintrag(importiert_am DESC, id ASC);
CREATE INDEX idx_eintrag_stichzahl_sort ON eintrag(stichzahl DESC, id ASC);
"#,
    },
    Schritt {
        nummer: 6,
        name: "fehlerstatus",
        sql: r#"
-- SM-IMP-009 verlangt, defekte Dateien **mit Fehlerstatus zu erfassen**, ohne
-- den Lauf abzubrechen. Ohne diese Spalten war nur die zweite Hälfte umgesetzt:
-- Die Datei wurde übersprungen und verschwand spurlos — bei 10.000 Dateien mit
-- 37 defekten sah die Nutzerin 9.963 Kacheln und keinen Hinweis, welche fehlen
-- und warum. Die Kachel mit der Marke „Fehler" aus DES-STM-001 Abschnitt 10
-- war damit unerreichbar.
ALTER TABLE eintrag ADD COLUMN fehlerstatus TEXT;
ALTER TABLE eintrag ADD COLUMN fehlergrund  TEXT;

CREATE INDEX idx_eintrag_fehlerstatus ON eintrag(fehlerstatus)
    WHERE fehlerstatus IS NOT NULL;
"#,
    },
];
