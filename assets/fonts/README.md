# Fonts

The typefaces both apps ship with, as named in `docs/design/tokens.json`.
All but PxPlus IBM VGA 8x16 are under the [SIL Open Font License 1.1](https://openfontlicense.org);
PxPlus IBM VGA 8x16 is under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/)
(see `CC-BY-SA-PxPlus.txt`). Each license sits next to its font and is packaged with it.

| File | Family | Used for |
|---|---|---|
| `Figtree.ttf` | Figtree (variable, weights 300 to 900) | All text in Bloom; body text in Graphite |
| `InstrumentSerif-Regular.ttf` | Instrument Serif | Graphite headings |
| `SpaceMono-Regular.ttf`, `SpaceMono-Bold.ttf` | Space Mono | Codes, IDs and Graphite labels |
| `PxPlus_IBM_VGA_8x16.ttf` | PxPlus IBM VGA 8x16 | The bundled EGA custom theme (from VileR's [Oldschool PC Font Pack](https://int10h.org/oldschool-pc-fonts/), desktop only) |

Taken unmodified from [google/fonts](https://github.com/google/fonts) at
commit `6e8069ff8ba3dab2a397fb30e7fbd243aba9b57a` (`ofl/figtree`,
`ofl/instrumentserif`, `ofl/spacemono`).

The Windows app compiles them into its resources (`desktop/app/build.rs`);
the Android app packages this folder as assets (`android/app/build.gradle.kts`).
