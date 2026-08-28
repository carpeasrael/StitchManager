//! Fuzzing-Ziel für den DST-Parser (SM-SEC-011, SM-FMT-012).
//!
//! Geprüft wird nicht ein Ergebnis, sondern eine Abwesenheit: kein Absturz,
//! keine unbegrenzte Speicherbelegung, keine Endlosschleife — gleich, welche
//! Bytes ankommen. Der Import ist der Fremddatenpfad dieses Programms.
#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|daten: &[u8]| {
    let _ = kern_parsers::lies_kennwerte(kern_typen::Format::Dst, daten);
    let _ = kern_parsers::lies_stichabschnitte(kern_typen::Format::Dst, daten);
});
