//! StitchManager — Einstiegspunkt der Anwendung.
//!
//! Weg A nach TEC-STM-001: Rust-Kern mit Qt-6-Oberfläche über cxx-qt.
//! Die Anwendung arbeitet offline und verändert Originaldateien nie (RB-04).

// Das Fenster ist die Oberfläche; unter Windows soll keine Konsole aufgehen.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bruecke;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};

fn main() {
    let mut app = QGuiApplication::new();

    // Der Basisstil trägt die Gestaltung; die Farben kommen ausschließlich
    // aus `ui/gestaltung` (SM-DES-003).
    QQuickStyle::set_style(&QString::from("Basic"));

    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/de/stitchmanager/qml/Haupt.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
