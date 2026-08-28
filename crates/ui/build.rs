fn main() {
    cxx_qt_build::CxxQtBuilder::new()
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qml_module(cxx_qt_build::QmlModule {
            uri: "de.stitchmanager",
            rust_files: &["src/bruecke.rs"],
            qml_files: &[
                "qml/Haupt.qml",
                "qml/Gestaltung.qml",
                "qml/Naht.qml",
                "qml/Kreuzstich.qml",
                "qml/Abschnittslabel.qml",
                "qml/Marke.qml",
                "qml/Schaltflaeche.qml",
                "qml/Chip.qml",
            ],
            ..Default::default()
        })
        .build();
}
