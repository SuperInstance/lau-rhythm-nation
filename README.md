# lau-rhythm-nation

**Unified cultural rhythm systems through a tensor MIDI interface.**

Tala (Indian), Tala (African), Ma (Japanese silence), Palaver tempo, Songline cadence, Islamic geometric rhythm, Western meter — all expressed through the same `RhythmicSignature` abstraction and driven by a shared `PolyrhythmEngine`. Agents in a multi-agent conversation each get assigned a cultural rhythm that determines their turn-taking cadence.

---

## What This Does

This crate provides a **unified rhythmic framework** that treats every cultural rhythm tradition as a first-class citizen:

- **Western** — Common-practice time signatures (4/4, 3/4, etc.) with downbeats and off-beat swing positions
- **Carnatic Tala** — Indian rhythmic cycles using *jaati* groupings (Tisra-3, Chatusra-4, Khanda-5, Misra-7, Sankirna-9)
- **Japanese Ma** — Intentional silence as part of rhythm, with configurable silence ratio
- **African Palaver** — Consensus-driven rhythm with tempo-derived subdivision
- **Aboriginal Songline** — Walking cadence tied to landscape steps, with natural swing
- **Islamic Girih** — Geometric n-fold symmetry folded into rhythmic accent patterns
- **African Ceremonial** — Phase-based cyclical rhythms with decaying strength

Each tradition produces a `RhythmicSignature` — a tick-level sequence of accent strengths, silence markers, and swing positions. Multiple signatures compose via the `PolyrhythmEngine`, which tracks sync points, harmonic alignment, and dominant traditions. The `ConversationRhythm` layer maps agents to rhythms and enforces an energy conservation budget.

---

## Key Idea

Different cultures organize time differently, but they all produce **periodic accent patterns**. By normalizing every tradition into a common `(accent_pattern, silence_pattern, swing)` representation, we can:

1. Run multiple cultural rhythms simultaneously (polyrhythm)
2. Detect when they align (sync points, harmony)
3. Use rhythm to control agent turn-taking in conversations
4. Serialize and deserialize the entire state for network transport

The math is unified: every rhythm is a vector of strengths over a period, and polyrhythmic composition is just element-wise aggregation.

---

## Install

```toml
[dependencies]
lau-rhythm-nation = "0.1"
```

Or via git:

```toml
[dependencies]
lau-rhythm-nation = { git = "https://github.com/SuperInstance/lau-rhythm-nation" }
```

Requires **Rust 2021 edition** (1.56+). Only dependency: `serde` for serialization.

---

## Quick Start

```rust
use lau_rhythm_nation::*;

// Create a conversation rhythm with energy budget
let mut cr = ConversationRhythm::new(10000.0);

// Assign agents to different cultural traditions
cr.add_agent("alice", RhythmTradition::Western { time_sig: (4, 4) });
cr.add_agent("bob", RhythmTradition::Tala { beats: 7, jaati: Jaati::Misra });
cr.add_agent("carol", RhythmTradition::Ma { silence_ratio: 0.25 });

// Advance ticks and check who should speak
for _ in 0..20 {
    let speakers = cr.current_speakers();
    println!("Tick {}: speakers = {:?}", cr.engine.tick, speakers);
    cr.tick();
}

// Check conservation
println!("Energy used: {:.2} / {:.2}", cr.total_energy_expended, cr.conservation_budget);
println!("Conserved: {}", cr.is_conserved());

// Use a pre-built multi-tradition conversation
let east_west = east_meets_west();
println!("LCC: {}", east_west.engine.lowest_common_cycle());
```

---

## API Reference

### `Jaati`

Carnatic rhythmic grouping counts:

| Variant | Counts |
|---------|--------|
| `Tisra` | 3 |
| `Chatusra` | 4 |
| `Khanda` | 5 |
| `Misra` | 7 |
| `Sankirna` | 9 |

---

### `RhythmTradition`

The cultural tradition that defines how time is organized:

```rust
RhythmTradition::Western { time_sig: (4, 4) }
RhythmTradition::Tala { beats: 7, jaati: Jaati::Misra }
RhythmTradition::Ma { silence_ratio: 0.25 }
RhythmTradition::Palaver { tempo: 90.0 }
RhythmTradition::Songline { steps: 8 }
RhythmTradition::Girih { symmetry: 6 }
RhythmTradition::Ceremonial { phase_count: 4 }
```

---

### `RhythmicSignature`

A concrete rhythmic pattern derived from a tradition.

| Field | Type | Description |
|-------|------|-------------|
| `tradition` | `RhythmTradition` | Source tradition |
| `base_period` | `u64` | Ticks per cycle |
| `accent_pattern` | `Vec<f64>` | Strength at each tick (0.0–1.0) |
| `silence_pattern` | `Vec<bool>` | Whether each tick is a rest |
| `swing` | `f64` | Swing factor (0.0 = straight, 1.0 = max) |
| `swing_positions` | `Vec<usize>` | Tick indices that receive swing |

**Factory constructors:**

| Method | Tradition | Period Formula |
|--------|-----------|----------------|
| `from_western(beats, subdivision)` | Western | `beats × subdivision` |
| `from_tala(beats, jaati)` | Carnatic | `beats × jaati.counts()` |
| `from_ma(beats, silence_ratio)` | Japanese | `beats × 4` |
| `from_palaver(tempo)` | African | `4 × clamp(tempo/15, 2, 12)` |
| `from_songline(steps)` | Aboriginal | `steps × 2` |
| `from_girih(symmetry)` | Islamic | `symmetry × 3` |
| `from_ceremonial(phase_count)` | Ceremonial | `phase_count × 6` |

**Query methods:**

| Method | Returns | Description |
|--------|---------|-------------|
| `beat_strength(position)` | `f64` | Accent strength at tick (wraps modulo period) |
| `is_silence(position)` | `bool` | Whether tick is a rest |
| `cycle_length()` | `u64` | Same as `base_period` |
| `accent_positions()` | `Vec<usize>` | Positions with strength ≥ 0.6 |

---

### `PolyrhythmEngine`

Drives multiple rhythmic signatures simultaneously.

```rust
let mut engine = PolyrhythmEngine::new();
engine.add_rhythm("western", RhythmicSignature::from_western(4, 4));
engine.add_rhythm("girih", RhythmicSignature::from_girih(6));
engine.tick();
```

| Method | Returns | Description |
|--------|---------|-------------|
| `add_rhythm(name, sig)` | — | Register a named rhythm |
| `tick()` | — | Advance one tick |
| `current_state()` | `HashMap<String, f64>` | Strength of each rhythm at current tick |
| `is_harmonic()` | `bool` | All rhythms above threshold |
| `sync_points(duration)` | `Vec<u64>` | Ticks where all rhythms align |
| `lowest_common_cycle()` | `u64` | LCM of all cycle lengths |
| `energy_at(tick)` | `f64` | Sum of all strengths at tick |
| `silence_at(tick)` | `bool` | Whether all rhythms are silent |
| `dominant_tradition()` | `Option<&str>` | Rhythm with highest current strength |
| `polyrhythm_summary()` | `String` | Human-readable state dump |

---

### `ConversationRhythm`

Maps agents to rhythms with energy conservation.

```rust
let mut cr = ConversationRhythm::new(10000.0);
cr.add_agent("alice", RhythmTradition::Western { time_sig: (4, 4) });
cr.tick();
println!("Speaking: {:?}", cr.current_speakers());
```

| Method | Returns | Description |
|--------|---------|-------------|
| `add_agent(name, tradition)` | — | Assign a rhythm to an agent |
| `should_speak(agent)` | `bool` | Agent's strength ≥ 0.5 and not silent |
| `tick()` | — | Advance, accumulating energy |
| `is_conserved()` | `bool` | Energy used ≤ budget |
| `current_speakers()` | `Vec<&str>` | Agents who should speak now |
| `rhythm_state()` | `String` | Human-readable state |

---

### Pre-built Conversations

| Function | Agents | Description |
|----------|--------|-------------|
| `east_meets_west()` | Western 4/4 + Tala Misra-7 + Ma | Cultural polyrhythm |
| `consensus_circle()` | Palaver at 60/90/120 BPM | Multi-tempo consensus |
| `songline_journey()` | Songline 8-step + Girih-6 + Ceremonial-4 | Walking + geometry + ceremony |

---

## How It Works

### Architecture

```
┌──────────────────────────────────────────────┐
│            ConversationRhythm                 │
│  (agents → rhythms, energy conservation)      │
│                                               │
│  ┌─────────────────────────────────────────┐  │
│  │          PolyrhythmEngine                │  │
│  │  (tick driver, harmony, sync, LCC)      │  │
│  │                                          │  │
│  │  ┌────────┐ ┌──────┐ ┌──────────────┐   │  │
│  │  │ Western│ │ Tala │ │ Ma / Girih... │   │  │
│  │  │  4/4   │ │ 7/M  │ │              │   │  │
│  │  │ 16 ticks│ │49tck│ │ varies       │   │  │
│  │  └────────┘ └──────┘ └──────────────┘   │  │
│  └─────────────────────────────────────────┘  │
└──────────────────────────────────────────────┘
```

### Accent Pattern Generation

Each tradition has its own accent generation logic:

- **Western**: Downbeat = 1.0, even beats = 0.7, odd beats = 0.4, subdivisions = 30% of parent
- **Tala**: Sam (first beat of first group) = 1.0, group starts = 0.8, interior decays linearly
- **Ma**: Beat onsets at 0.75–1.0, silence distributed evenly at ratio, silenced positions = 0.0
- **Palaver**: First beat = 1.0, subsequent decay as `0.5 + 0.3/(beat+1)`, subdivisions = 20%
- **Songline**: First step = 1.0, decay toward end, off-steps = 0.15
- **Girih**: Main vertex = base strength, edge midpoint = 50%, secondary = 25%, repeated per fold
- **Ceremonial**: Phase strength decays as `1.0 - 0.5 × (phase/total)`, internal pattern: 100%/60%/30%/80%/20%/10%

### Polyrhythmic Composition

Multiple rhythms run against a shared tick counter. Harmony occurs when all rhythms simultaneously exceed the `harmony_threshold`. Sync points within a duration are all ticks where this holds. The lowest common cycle is the LCM of all individual periods.

---

## The Math

### Period and LCM

For rhythms with periods `p₁, p₂, ..., pₙ`, the **lowest common cycle** is:

```
LCC = lcm(p₁, p₂, ..., pₙ) = p₁ × p₂ / gcd(p₁, p₂) (extended iteratively)
```

At tick `LCC`, all rhythms simultaneously return to their initial state.

### GCD via Euclidean Algorithm

```
gcd(a, b):
    while b ≠ 0:
        (a, b) = (b, a mod b)
    return a
```

### Swing

Swing delays off-beat positions. A swing factor `s ∈ [0, 1]` applied at positions in `swing_positions` proportionally delays those ticks. The `RhythmicSignature` records *where* swing applies; actual timing offset is `s × tick_duration`.

### Energy Conservation

Each tick accumulates energy:

```
E(tick) = Σᵢ beat_strengthᵢ(tick)
total_energy = Σ_tick E(tick)
```

The conversation is "conserved" while `total_energy ≤ budget`.

### Ma Silence Distribution

For a period `P` with silence ratio `r`:

```
n_silent = round(P × r)
step = P / n_silent
silent positions: [round(step), round(2×step), ..., round(n×step)]
```

Silence is distributed **evenly** — reflecting the Japanese aesthetic of *ma* as structured emptiness.

---

## Testing

56 tests covering all traditions, the engine, conversations, serialization, and edge cases:

```bash
cargo test
```

Test categories:
- **Jaati** (2): counts and discriminant values
- **RhythmicSignature construction** (16): period calculation, accent patterns, silence, swing for all 7 traditions
- **Query methods** (4): beat strength, silence, accent positions, cycle length
- **PolyrhythmEngine** (14): add/tick, current state, harmony, LCM, energy, silence, dominant tradition, sync points, summary
- **ConversationRhythm** (9): add agent, should speak, tick energy, conservation, current speakers, state output
- **Pre-built conversations** (4): east_meets_west, consensus_circle, songline_journey, LCC
- **Serde round-trip** (4): tradition, signature, engine, conversation
- **Math helpers** (2): gcd, lcm
- **Edge cases** (3): zero silence ratio, high silence ratio, beat wrapping, default trait

---

## License

MIT © SuperInstance
