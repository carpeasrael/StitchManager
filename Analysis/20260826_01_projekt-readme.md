# Analyse — produktorientierte Projekt-README

**Datum:** 26.08.2026  
**Änderungsklasse:** Dokumentation und Gestaltungsbestand

## Problem

Die vorhandene README richtet sich hauptsächlich an die Spezifikations- und
Prüfarbeit. Sie erklärt Endnutzern weder den vorgesehenen Funktionsumfang von
Version 1.0 noch die Installation auf macOS, Windows und Linux in einer
produktnahen Form. Zudem ist das Logo des Mockups nur als eingebettetes
SVG-Symbol in `Design/stitchmanager-mockup.html` vorhanden und deshalb nicht
direkt in einer Markdown-Datei verwendbar.

## Betroffene Komponenten

- `README.md`
- neues, aus dem Mockup abgeleitetes `Design/stitchmanager-logo.svg`

## Quellen und Abgrenzung

- Der Produktzweck folgt URS-STM-001, Kapitel 2 bis 4.
- Der Umfang von Version 1.0 folgt IMP-STM-001, Kapitel 1 und den Ergebnissen
  der Arbeitspakete AP-03 bis AP-21.
- Gestaltung, Farben und Kreuzstich-Signet folgen DES-STM-001 sowie
  `Design/stitchmanager-mockup.html`.
- Die Auslieferungsformate folgen TEC-STM-001, Abschnitt 5.
- Fertige Installationspakete werden nicht behauptet: AP-21 ist noch nicht
  umgesetzt. Bis dahin dokumentiert die README den Bau aus dem Quellcode.

## Betroffene Anforderungen

SM-DES-001 bis SM-DES-004, SM-PLT-001 bis SM-PLT-005, SM-PLT-010 sowie die in
IMP-STM-001 für Version 1.0 verplanten funktionalen Anforderungen. Die README
begründet keine neue Anforderung.

## Offene Punkte

OP-04 (Signaturinfrastruktur), OP-06 (Produktname) und OP-03 (Zielformate des
Schreibpfads) werden nicht vorweggenommen. Der Arbeitsname StitchManager wird
entsprechend dem Bestand verwendet; Warnhinweise zu unsignierten Paketen und
konkrete Zielformate werden nicht als fertige Produkteigenschaft dargestellt.

## Ansatz

1. Produktnutzen und Grenzen in Alltagssprache beschreiben.
2. Den geplanten Umfang von Version 1.0 vom heutigen Entwicklungsstand trennen.
3. Zielpakete je Plattform und derzeit ausführbare Quellinstallation getrennt
   dokumentieren.
4. Das vorhandene Kreuzstich-Signet als barrierearmes SVG-Wortbild übernehmen.
5. Links zu Lastenheft, Design, Technik und Implementierungsplan erhalten.
