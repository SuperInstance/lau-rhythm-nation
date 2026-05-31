# lau-rhythm-nation

One crate, seven rhythm traditions. Indian tala, African polyrhythm, Japanese ma, Islamic geometric rhythm, Aboriginal songline cadence, Akan drum patterns, Western meter — all expressed through the same tensor MIDI interface.

## The concept in 60 seconds

Every culture developed rhythm independently, and every culture discovered the same mathematical truths: cyclic patterns, polyrhythmic layering, tension and release, silence as structure. This crate unifies them:

- **Indian tala:** Carnatic jaati (3, 4, 5, 7, 9 beats) with anga structure (laghu, drutam, anudrutam)
- **African polyrhythm:** simultaneous meters with cross-beat emphasis
- **Japanese ma:** silence as rhythm — the space between notes IS the music
- **Islamic geometric rhythm:** symmetry groups mapped to rhythmic patterns
- **Songline cadence:** walking speed as tempo, landscape as score
- **Western meter:** 4/4, 3/4, 6/8 with emphasis patterns
- **Unified output:** everything renders through tensor MIDI

## Quick start

```rust
use lau_rhythm_nation::{Tala, Polyrhythm, MaRhythm, RhythmFusion};

// Carnatic tala: Adi (4+2+2 = 8 beats)
let adi = Tala::adi();
let pattern = adi.pattern(); // [L, D, D, L, D, L, D, D]

// African polyrhythm: 3 against 4
let poly = Polyrhythm::new(&[3, 4]);
let lcm_beats = poly.total_beats(); // 12

// Japanese ma — silence-weighted rhythm
let ma = MaRhythm::new(4).with_ma_weight(0.6); // 60% silence

// Fuse traditions into one tensor MIDI output
let fusion = RhythmFusion::new()
    .add_layer(adi, 120.0)   // 120 BPM
    .add_layer(poly, 90.0)   // 90 BPM
    .add_layer(ma, 60.0);    // 60 BPM
let tensor = fusion.to_tensor_midi();
```

## Key types

| Type | What it is |
|------|-----------|
| `Tala` | Carnatic rhythmic cycle with jaati and anga |
| `Polyrhythm` | Simultaneous conflicting meters |
| `MaRhythm` | Silence-weighted rhythm (Japanese) |
| `GeometricRhythm` | Symmetry-group derived patterns (Islamic) |
| `SonglineCadence` | Walking-speed tempo with landscape triggers |
| `RhythmFusion` | Unified multi-tradition output via tensor MIDI |

## Contributing

[Open an issue](https://github.com/SuperInstance/lau-rhythm-nation/issues) or PR.
