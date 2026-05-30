//! # lau-rhythm-nation
//!
//! Unifies ALL cultural rhythm systems into one bridge crate.
//! Tala (Indian), Tala (African), Ma (Japanese silence), Palaver tempo,
//! Songline cadence, Islamic geometric rhythm, Western meter — all expressed
//! through the same tensor MIDI interface.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Jaati — Carnatic rhythmic grouping
// ---------------------------------------------------------------------------

/// Carnatic *jaati* (rhythmic grouping counts).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Jaati {
    Tisra = 3,
    Chatusra = 4,
    Khanda = 5,
    Misra = 7,
    Sankirna = 9,
}

impl Jaati {
    /// Number of counts in this jaati.
    pub fn counts(&self) -> u8 {
        match self {
            Jaati::Tisra => 3,
            Jaati::Chatusra => 4,
            Jaati::Khanda => 5,
            Jaati::Misra => 7,
            Jaati::Sankirna => 9,
        }
    }
}

// ---------------------------------------------------------------------------
// RhythmTradition
// ---------------------------------------------------------------------------

/// The cultural / rhythmic tradition that defines how time is organised.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RhythmTradition {
    /// Western common-practice time signature.
    Western { time_sig: (u8, u8) },
    /// Indian (Carnatic) tala.
    Tala { beats: u8, jaati: Jaati },
    /// Japanese *ma* — intentional silence as part of rhythm.
    Ma { silence_ratio: f64 },
    /// African palaver — consensus rhythm at a given tempo.
    Palaver { tempo: f64 },
    /// Aboriginal songline — walking cadence tied to landscape.
    Songline { steps: u8 },
    /// Islamic *girih* — geometric symmetry folded into rhythm.
    Girih { symmetry: u8 },
    /// African ceremonial — phase-based cyclical rhythm.
    Ceremonial { phase_count: u8 },
}

// ---------------------------------------------------------------------------
// RhythmicSignature
// ---------------------------------------------------------------------------

/// A concrete rhythmic signature derived from a tradition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RhythmicSignature {
    pub tradition: RhythmTradition,
    /// Ticks per cycle.
    pub base_period: u64,
    /// Strength of each beat position (0.0–1.0).
    pub accent_pattern: Vec<f64>,
    /// Whether a position is a rest.
    pub silence_pattern: Vec<bool>,
    /// Swing factor (0.0 = straight, 1.0 = maximum swing).
    pub swing: f64,
    /// Beat indices that receive swing.
    pub swing_positions: Vec<usize>,
}

impl RhythmicSignature {
    // -- Factory constructors ------------------------------------------------

    /// Build a standard Western time-signature rhythm.
    pub fn from_western(beats: u8, subdivision: u8) -> Self {
        let period = (beats as u64) * (subdivision as u64);
        let mut accents = Vec::with_capacity(period as usize);
        let mut silences = Vec::with_capacity(period as usize);
        let mut swing_pos = Vec::new();

        for b in 0..beats {
            let strength = if b == 0 { 1.0 } else if b % 2 == 0 { 0.7 } else { 0.4 };
            for s in 0..subdivision {
                let idx = (b as u64 * subdivision as u64 + s as u64) as usize;
                let beat_strength = if s == 0 { strength } else { strength * 0.3 };
                accents.push(beat_strength);
                silences.push(false);
                // swing on off-beats
                if s == 1 {
                    swing_pos.push(idx);
                }
            }
        }

        Self {
            tradition: RhythmTradition::Western {
                time_sig: (beats, subdivision),
            },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: swing_pos,
        }
    }

    /// Build a Carnatic tala rhythm with accent pattern.
    pub fn from_tala(beats: u8, jaati: Jaati) -> Self {
        let counts = jaati.counts();
        let period = (beats as u64) * (counts as u64);
        let mut accents = Vec::with_capacity(period as usize);
        let silences = vec![false; period as usize];

        for b in 0..beats {
            for c in 0..counts {
                let strength = if b == 0 && c == 0 {
                    1.0 // sam
                } else if c == 0 {
                    0.8 // beat start
                } else {
                    0.3 + 0.4 * (1.0 - (c as f64) / (counts as f64))
                };
                accents.push(strength);
            }
        }

        Self {
            tradition: RhythmTradition::Tala { beats, jaati },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: Vec::new(),
        }
    }

    /// Build a Japanese *ma* rhythm with intentional silence.
    pub fn from_ma(beats: u8, silence_ratio: f64) -> Self {
        let period = beats as u64 * 4; // 4 subdivisions per beat
        let num_silent = ((period as f64) * silence_ratio).round() as usize;
        let mut accents = vec![0.5; period as usize];
        let mut silences = vec![false; period as usize];

        // Place accents on beat onsets
        for b in 0..beats {
            accents[b as usize * 4] = if b == 0 { 1.0 } else { 0.75 };
        }

        // Distribute silence evenly (ma)
        if period > 0 && num_silent > 0 {
            let step = (period as f64) / (num_silent as f64);
            let mut pos = step;
            let mut i = 0usize;
            while i < num_silent {
                let idx = (pos.round() as usize).min(period as usize - 1);
                silences[idx] = true;
                accents[idx] = 0.0;
                pos += step;
                i += 1;
            }
        }

        Self {
            tradition: RhythmTradition::Ma { silence_ratio },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: Vec::new(),
        }
    }

    /// Build an African palaver (consensus) rhythm.
    pub fn from_palaver(tempo: f64) -> Self {
        // Normalise tempo into a cycle length (e.g. 60 bpm → 48 ticks)
        let beats = 4u64;
        let subdivision = ((tempo / 15.0).round() as u64).clamp(2, 12);
        let period = beats * subdivision;

        let mut accents = Vec::with_capacity(period as usize);
        let silences = vec![false; period as usize];

        for b in 0..beats {
            let strength = if b == 0 { 1.0 } else { 0.5 + 0.3 * (1.0 / (b as f64 + 1.0)) };
            for s in 0..subdivision {
                accents.push(if s == 0 { strength } else { strength * 0.2 });
            }
        }

        Self {
            tradition: RhythmTradition::Palaver { tempo },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: Vec::new(),
        }
    }

    /// Build an Aboriginal songline walking rhythm.
    pub fn from_songline(steps: u8) -> Self {
        let period = steps as u64 * 2; // 2 subdivisions per step
        let mut accents = Vec::with_capacity(period as usize);
        let silences = vec![false; period as usize];

        for s in 0..steps {
            // Walking accent: first step strong, second softer
            accents.push(if s == 0 { 1.0 } else { 0.6 + 0.3 * ((steps - s) as f64 / steps as f64) });
            accents.push(0.15); // off-step
        }

        Self {
            tradition: RhythmTradition::Songline { steps },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.2,
            swing_positions: (0..period as usize).filter(|i| i % 2 == 1).collect(),
        }
    }

    /// Build an Islamic *girih* geometric rhythm with *n*-fold symmetry.
    pub fn from_girih(symmetry: u8) -> Self {
        let period = symmetry as u64 * 3; // 3 layers per symmetry fold
        let mut accents = Vec::with_capacity(period as usize);
        let silences = vec![false; period as usize];

        for s in 0..symmetry {
            let base = if s == 0 { 1.0 } else { 0.7 };
            accents.push(base); // main vertex
            accents.push(base * 0.5); // edge midpoint
            accents.push(base * 0.25); // secondary vertex
        }

        Self {
            tradition: RhythmTradition::Girih { symmetry },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: Vec::new(),
        }
    }

    /// Build a ceremonial phase-based rhythm.
    pub fn from_ceremonial(phase_count: u8) -> Self {
        let ticks_per_phase: u64 = 6;
        let period = phase_count as u64 * ticks_per_phase;
        let mut accents = Vec::with_capacity(period as usize);
        let silences = vec![false; period as usize];

        for p in 0..phase_count {
            let phase_strength = 1.0 - (p as f64 / phase_count as f64) * 0.5;
            for t in 0..ticks_per_phase {
                let s = match t {
                    0 => phase_strength,
                    1 => phase_strength * 0.6,
                    2 => phase_strength * 0.3,
                    3 => phase_strength * 0.8,
                    4 => phase_strength * 0.2,
                    _ => phase_strength * 0.1,
                };
                accents.push(s);
            }
        }

        Self {
            tradition: RhythmTradition::Ceremonial { phase_count },
            base_period: period,
            accent_pattern: accents,
            silence_pattern: silences,
            swing: 0.0,
            swing_positions: Vec::new(),
        }
    }

    // -- Query methods -------------------------------------------------------

    /// Beat strength at an absolute tick position.
    pub fn beat_strength(&self, position: u64) -> f64 {
        if self.base_period == 0 {
            return 0.0;
        }
        let idx = (position % self.base_period) as usize;
        if idx < self.accent_pattern.len() {
            self.accent_pattern[idx]
        } else {
            0.0
        }
    }

    /// Whether the given tick position is a rest.
    pub fn is_silence(&self, position: u64) -> bool {
        if self.base_period == 0 {
            return false;
        }
        let idx = (position % self.base_period) as usize;
        if idx < self.silence_pattern.len() {
            self.silence_pattern[idx]
        } else {
            false
        }
    }

    /// Length of one full cycle in ticks.
    pub fn cycle_length(&self) -> u64 {
        self.base_period
    }

    /// Positions where accents (strength ≥ 0.6) fall within one cycle.
    pub fn accent_positions(&self) -> Vec<usize> {
        self.accent_pattern
            .iter()
            .enumerate()
            .filter(|(_, &s)| s >= 0.6)
            .map(|(i, _)| i)
            .collect()
    }
}

// ---------------------------------------------------------------------------
// PolyrhythmEngine
// ---------------------------------------------------------------------------

/// Engine that drives multiple rhythmic signatures simultaneously.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolyrhythmEngine {
    pub signatures: HashMap<String, RhythmicSignature>,
    pub tick: u64,
    pub harmony_threshold: f64,
}

impl PolyrhythmEngine {
    pub fn new() -> Self {
        Self {
            signatures: HashMap::new(),
            tick: 0,
            harmony_threshold: 0.5,
        }
    }

    pub fn add_rhythm(&mut self, name: &str, sig: RhythmicSignature) {
        self.signatures.insert(name.to_string(), sig);
    }

    /// Advance one tick.
    pub fn tick(&mut self) {
        self.tick += 1;
    }

    /// Current beat strength for each named rhythm.
    pub fn current_state(&self) -> HashMap<String, f64> {
        self.signatures
            .iter()
            .map(|(name, sig)| (name.clone(), sig.beat_strength(self.tick)))
            .collect()
    }

    /// True when *all* rhythms are above the harmony threshold at this tick.
    pub fn is_harmonic(&self) -> bool {
        if self.signatures.is_empty() {
            return true;
        }
        self.signatures
            .values()
            .all(|sig| sig.beat_strength(self.tick) >= self.harmony_threshold)
    }

    /// Tick positions within `duration` where every rhythm aligns (strength ≥ threshold).
    pub fn sync_points(&self, duration: u64) -> Vec<u64> {
        (0..duration)
            .filter(|&t| {
                self.signatures
                    .values()
                    .all(|sig| sig.beat_strength(t) >= self.harmony_threshold)
            })
            .collect()
    }

    /// Lowest common multiple of all cycle lengths.
    pub fn lowest_common_cycle(&self) -> u64 {
        self.signatures
            .values()
            .map(|s| s.base_period)
            .fold(1u64, lcm)
    }

    /// Sum of all beat strengths at the given tick.
    pub fn energy_at(&self, tick: u64) -> f64 {
        self.signatures
            .values()
            .map(|sig| sig.beat_strength(tick))
            .sum()
    }

    /// True when every rhythm is silent at the given tick.
    pub fn silence_at(&self, tick: u64) -> bool {
        if self.signatures.is_empty() {
            return true;
        }
        self.signatures
            .values()
            .all(|sig| sig.is_silence(tick) || sig.beat_strength(tick) == 0.0)
    }

    /// Name of the rhythm with the highest current energy.
    pub fn dominant_tradition(&self) -> Option<&str> {
        self.signatures
            .iter()
            .max_by(|a, b| {
                a.1.beat_strength(self.tick)
                    .partial_cmp(&b.1.beat_strength(self.tick))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(name, _)| name.as_str())
    }

    /// Human-readable summary.
    pub fn polyrhythm_summary(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Tick: {}", self.tick));
        for (name, sig) in &self.signatures {
            let strength = sig.beat_strength(self.tick);
            let silent = sig.is_silence(self.tick);
            lines.push(format!(
                "  {} → strength={:.2} silent={} period={}",
                name, strength, silent, sig.base_period
            ));
        }
        lines.push(format!("Harmonic: {}", self.is_harmonic()));
        lines.push(format!("LCC: {}", self.lowest_common_cycle()));
        if let Some(dom) = self.dominant_tradition() {
            lines.push(format!("Dominant: {}", dom));
        }
        lines.join("\n")
    }
}

impl Default for PolyrhythmEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// ConversationRhythm
// ---------------------------------------------------------------------------

/// Maps agents to rhythms and enforces an energy conservation budget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConversationRhythm {
    pub engine: PolyrhythmEngine,
    pub agents: HashMap<String, String>,
    pub conservation_budget: f64,
    pub total_energy_expended: f64,
}

impl ConversationRhythm {
    pub fn new(budget: f64) -> Self {
        Self {
            engine: PolyrhythmEngine::new(),
            agents: HashMap::new(),
            conservation_budget: budget,
            total_energy_expended: 0.0,
        }
    }

    /// Assign a rhythm tradition to an agent.
    pub fn add_agent(&mut self, agent: &str, tradition: RhythmTradition) {
        let sig = signature_from_tradition(&tradition);
        let rhythm_name = format!("{}_rhythm", agent);
        self.engine.add_rhythm(&rhythm_name, sig);
        self.agents.insert(agent.to_string(), rhythm_name);
    }

    /// Whether the agent's rhythm says it is their turn to speak.
    pub fn should_speak(&self, agent: &str) -> bool {
        if let Some(rhythm_name) = self.agents.get(agent) {
            if let Some(sig) = self.engine.signatures.get(rhythm_name) {
                let strength = sig.beat_strength(self.engine.tick);
                return strength >= 0.5 && !sig.is_silence(self.engine.tick);
            }
        }
        false
    }

    /// Advance one tick, accounting energy.
    pub fn tick(&mut self) {
        self.engine.tick();
        let energy: f64 = self
            .engine
            .signatures
            .values()
            .map(|sig| sig.beat_strength(self.engine.tick))
            .sum();
        self.total_energy_expended += energy;
    }

    /// True when total energy expended is within budget.
    pub fn is_conserved(&self) -> bool {
        self.total_energy_expended <= self.conservation_budget
    }

    /// Agents whose rhythms are currently active.
    pub fn current_speakers(&self) -> Vec<&str> {
        self.agents
            .keys()
            .filter(|agent| self.should_speak(agent))
            .map(|s| s.as_str())
            .collect()
    }

    /// Human-readable state.
    pub fn rhythm_state(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("Tick: {}  Energy: {:.2}/{:.2}", 
            self.engine.tick, self.total_energy_expended, self.conservation_budget));
        lines.push(format!("Conserved: {}", self.is_conserved()));
        let speakers = self.current_speakers();
        lines.push(format!("Speaking: {:?}", speakers));
        lines.push(self.engine.polyrhythm_summary());
        lines.join("\n")
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { a / gcd(a, b) * b }
}

fn signature_from_tradition(tradition: &RhythmTradition) -> RhythmicSignature {
    match tradition {
        RhythmTradition::Western { time_sig } => {
            RhythmicSignature::from_western(time_sig.0, time_sig.1)
        }
        RhythmTradition::Tala { beats, jaati } => {
            RhythmicSignature::from_tala(*beats, *jaati)
        }
        RhythmTradition::Ma { silence_ratio } => {
            RhythmicSignature::from_ma(4, *silence_ratio)
        }
        RhythmTradition::Palaver { tempo } => {
            RhythmicSignature::from_palaver(*tempo)
        }
        RhythmTradition::Songline { steps } => {
            RhythmicSignature::from_songline(*steps)
        }
        RhythmTradition::Girih { symmetry } => {
            RhythmicSignature::from_girih(*symmetry)
        }
        RhythmTradition::Ceremonial { phase_count } => {
            RhythmicSignature::from_ceremonial(*phase_count)
        }
    }
}

// ---------------------------------------------------------------------------
// Pre-built conversation rhythms
// ---------------------------------------------------------------------------

/// "East meets West": Western 4/4 + Tala Misra (7) + Ma silence.
pub fn east_meets_west() -> ConversationRhythm {
    let mut cr = ConversationRhythm::new(10000.0);
    cr.add_agent("western", RhythmTradition::Western { time_sig: (4, 4) });
    cr.add_agent("tala", RhythmTradition::Tala { beats: 7, jaati: Jaati::Misra });
    cr.add_agent("ma", RhythmTradition::Ma { silence_ratio: 0.25 });
    cr
}

/// "Consensus Circle": Palaver — 3 agents with different tempos.
pub fn consensus_circle() -> ConversationRhythm {
    let mut cr = ConversationRhythm::new(8000.0);
    cr.add_agent("elder", RhythmTradition::Palaver { tempo: 60.0 });
    cr.add_agent("speaker", RhythmTradition::Palaver { tempo: 90.0 });
    cr.add_agent("youth", RhythmTradition::Palaver { tempo: 120.0 });
    cr
}

/// "Songline Journey": Songline (8 steps) + Girih (6-fold) + Ceremonial (4 phases).
pub fn songline_journey() -> ConversationRhythm {
    let mut cr = ConversationRhythm::new(12000.0);
    cr.add_agent("walker", RhythmTradition::Songline { steps: 8 });
    cr.add_agent("geometer", RhythmTradition::Girih { symmetry: 6 });
    cr.add_agent("ceremony", RhythmTradition::Ceremonial { phase_count: 4 });
    cr
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Jaati tests ---

    #[test]
    fn jaati_counts() {
        assert_eq!(Jaati::Tisra.counts(), 3);
        assert_eq!(Jaati::Chatusra.counts(), 4);
        assert_eq!(Jaati::Khanda.counts(), 5);
        assert_eq!(Jaati::Misra.counts(), 7);
        assert_eq!(Jaati::Sankirna.counts(), 9);
    }

    #[test]
    fn jaati_discriminant() {
        assert_eq!(Jaati::Tisra as u8, 3);
        assert_eq!(Jaati::Sankirna as u8, 9);
    }

    // --- RhythmicSignature construction tests ---

    #[test]
    fn western_4_4_period() {
        let sig = RhythmicSignature::from_western(4, 4);
        assert_eq!(sig.base_period, 16);
        assert_eq!(sig.accent_pattern.len(), 16);
        assert_eq!(sig.silence_pattern.len(), 16);
    }

    #[test]
    fn western_downbeat_strength() {
        let sig = RhythmicSignature::from_western(4, 4);
        assert_eq!(sig.beat_strength(0), 1.0);
    }

    #[test]
    fn western_cycle_repeats() {
        let sig = RhythmicSignature::from_western(4, 4);
        assert_eq!(sig.beat_strength(0), sig.beat_strength(16));
    }

    #[test]
    fn western_3_4() {
        let sig = RhythmicSignature::from_western(3, 4);
        assert_eq!(sig.base_period, 12);
    }

    #[test]
    fn tala_period() {
        let sig = RhythmicSignature::from_tala(7, Jaati::Misra);
        assert_eq!(sig.base_period, 49); // 7 * 7
    }

    #[test]
    fn tala_sam_strength() {
        let sig = RhythmicSignature::from_tala(5, Jaati::Chatusra);
        // First beat of first group = sam
        assert_eq!(sig.beat_strength(0), 1.0);
    }

    #[test]
    fn tala_beat_starts() {
        let sig = RhythmicSignature::from_tala(3, Jaati::Tisra);
        // Beat 1 start (index 3) should be 0.8
        let strength = sig.beat_strength(3);
        assert!((strength - 0.8).abs() < 1e-6);
    }

    #[test]
    fn ma_silence() {
        let sig = RhythmicSignature::from_ma(4, 0.25);
        assert_eq!(sig.base_period, 16);
        // Some positions should be silent
        let silent_count = sig.silence_pattern.iter().filter(|&&s| s).count();
        assert!(silent_count > 0);
    }

    #[test]
    fn ma_downbeat() {
        let sig = RhythmicSignature::from_ma(4, 0.1);
        assert_eq!(sig.beat_strength(0), 1.0);
    }

    #[test]
    fn palaver_period() {
        let sig = RhythmicSignature::from_palaver(60.0);
        assert!(sig.base_period > 0);
        assert!(sig.accent_pattern.len() as u64 == sig.base_period);
    }

    #[test]
    fn songline_period() {
        let sig = RhythmicSignature::from_songline(8);
        assert_eq!(sig.base_period, 16);
    }

    #[test]
    fn songline_swing() {
        let sig = RhythmicSignature::from_songline(6);
        assert!(sig.swing > 0.0);
        assert!(!sig.swing_positions.is_empty());
    }

    #[test]
    fn girih_period() {
        let sig = RhythmicSignature::from_girih(6);
        assert_eq!(sig.base_period, 18); // 6 * 3
    }

    #[test]
    fn girih_symmetry_accent() {
        let sig = RhythmicSignature::from_girih(4);
        // First position should be strongest
        assert_eq!(sig.beat_strength(0), 1.0);
    }

    #[test]
    fn ceremonial_period() {
        let sig = RhythmicSignature::from_ceremonial(4);
        assert_eq!(sig.base_period, 24); // 4 * 6
    }

    #[test]
    fn ceremonial_first_phase_strongest() {
        let sig = RhythmicSignature::from_ceremonial(3);
        assert!(sig.beat_strength(0) > sig.beat_strength(6));
    }

    // --- Query method tests ---

    #[test]
    fn is_silence_default() {
        let sig = RhythmicSignature::from_western(4, 4);
        assert!(!sig.is_silence(0));
    }

    #[test]
    fn accent_positions() {
        let sig = RhythmicSignature::from_western(4, 4);
        let accents = sig.accent_positions();
        assert!(accents.contains(&0)); // downbeat
        assert!(accents.len() >= 1);
    }

    #[test]
    fn cycle_length_matches_period() {
        let sig = RhythmicSignature::from_tala(5, Jaati::Khanda);
        assert_eq!(sig.cycle_length(), sig.base_period);
    }

    // --- PolyrhythmEngine tests ---

    #[test]
    fn engine_new() {
        let e = PolyrhythmEngine::new();
        assert_eq!(e.tick, 0);
        assert!(e.signatures.is_empty());
    }

    #[test]
    fn engine_add_and_tick() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("western", RhythmicSignature::from_western(4, 4));
        e.tick();
        assert_eq!(e.tick, 1);
    }

    #[test]
    fn engine_current_state() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        let state = e.current_state();
        assert!(state.contains_key("w"));
        assert_eq!(state["w"], 1.0); // downbeat
    }

    #[test]
    fn engine_is_harmonic_at_origin() {
        let mut e = PolyrhythmEngine::new();
        e.harmony_threshold = 0.5;
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        e.add_rhythm("t", RhythmicSignature::from_tala(3, Jaati::Tisra));
        // At tick 0 both have strong beats
        assert!(e.is_harmonic());
    }

    #[test]
    fn engine_is_harmonic_empty() {
        let e = PolyrhythmEngine::new();
        assert!(e.is_harmonic());
    }

    #[test]
    fn engine_lcm() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4)); // period 16
        e.add_rhythm("g", RhythmicSignature::from_girih(6)); // period 18
        let lcc = e.lowest_common_cycle();
        assert_eq!(lcc, 144); // lcm(16,18)
    }

    #[test]
    fn engine_energy_at() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        let energy = e.energy_at(0);
        assert!(energy > 0.0);
    }

    #[test]
    fn engine_silence_at() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        // Western rhythm has no explicit silences
        assert!(!e.silence_at(0));
    }

    #[test]
    fn engine_dominant_tradition() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        e.add_rhythm("g", RhythmicSignature::from_girih(4));
        let dom = e.dominant_tradition();
        assert!(dom.is_some());
    }

    #[test]
    fn engine_dominant_tradition_empty() {
        let e = PolyrhythmEngine::new();
        assert!(e.dominant_tradition().is_none());
    }

    #[test]
    fn engine_sync_points() {
        let mut e = PolyrhythmEngine::new();
        e.harmony_threshold = 0.5;
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        let sp = e.sync_points(16);
        assert!(!sp.is_empty());
        assert!(sp.contains(&0));
    }

    #[test]
    fn engine_polyrhythm_summary() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        let summary = e.polyrhythm_summary();
        assert!(summary.contains("Tick: 0"));
        assert!(summary.contains("w"));
    }

    // --- ConversationRhythm tests ---

    #[test]
    fn conversation_new() {
        let cr = ConversationRhythm::new(500.0);
        assert_eq!(cr.conservation_budget, 500.0);
        assert_eq!(cr.total_energy_expended, 0.0);
        assert!(cr.agents.is_empty());
    }

    #[test]
    fn conversation_add_agent() {
        let mut cr = ConversationRhythm::new(1000.0);
        cr.add_agent("alice", RhythmTradition::Western { time_sig: (4, 4) });
        assert!(cr.agents.contains_key("alice"));
        assert!(cr.engine.signatures.contains_key("alice_rhythm"));
    }

    #[test]
    fn conversation_should_speak_at_downbeat() {
        let mut cr = ConversationRhythm::new(10000.0);
        cr.add_agent("alice", RhythmTradition::Western { time_sig: (4, 4) });
        // At tick 0, downbeat = strength 1.0
        assert!(cr.should_speak("alice"));
    }

    #[test]
    fn conversation_should_not_speak_unknown() {
        let cr = ConversationRhythm::new(1000.0);
        assert!(!cr.should_speak("nobody"));
    }

    #[test]
    fn conversation_tick_advances_energy() {
        let mut cr = ConversationRhythm::new(10000.0);
        cr.add_agent("a", RhythmTradition::Western { time_sig: (4, 4) });
        cr.tick();
        assert!(cr.total_energy_expended > 0.0);
        assert_eq!(cr.engine.tick, 1);
    }

    #[test]
    fn conversation_conserved() {
        let mut cr = ConversationRhythm::new(100000.0);
        cr.add_agent("a", RhythmTradition::Western { time_sig: (4, 4) });
        for _ in 0..10 {
            cr.tick();
        }
        assert!(cr.is_conserved());
    }

    #[test]
    fn conversation_not_conserved() {
        let mut cr = ConversationRhythm::new(0.001);
        cr.add_agent("a", RhythmTradition::Western { time_sig: (4, 4) });
        for _ in 0..100 {
            cr.tick();
        }
        assert!(!cr.is_conserved());
    }

    #[test]
    fn conversation_current_speakers() {
        let mut cr = ConversationRhythm::new(10000.0);
        cr.add_agent("a", RhythmTradition::Western { time_sig: (4, 4) });
        let speakers = cr.current_speakers();
        assert!(speakers.contains(&"a"));
    }

    #[test]
    fn conversation_rhythm_state() {
        let mut cr = ConversationRhythm::new(10000.0);
        cr.add_agent("a", RhythmTradition::Western { time_sig: (4, 4) });
        let state = cr.rhythm_state();
        assert!(state.contains("Tick:"));
        assert!(state.contains("Conserved:"));
    }

    // --- Pre-built rhythm tests ---

    #[test]
    fn east_meets_west_builds() {
        let cr = east_meets_west();
        assert_eq!(cr.agents.len(), 3);
        assert!(cr.agents.contains_key("western"));
        assert!(cr.agents.contains_key("tala"));
        assert!(cr.agents.contains_key("ma"));
    }

    #[test]
    fn consensus_circle_builds() {
        let cr = consensus_circle();
        assert_eq!(cr.agents.len(), 3);
    }

    #[test]
    fn songline_journey_builds() {
        let cr = songline_journey();
        assert_eq!(cr.agents.len(), 3);
    }

    #[test]
    fn east_meets_west_lcc() {
        let cr = east_meets_west();
        let lcc = cr.engine.lowest_common_cycle();
        assert!(lcc > 0);
    }

    // --- Serde round-trip tests ---

    #[test]
    fn serde_tradition() {
        let t = RhythmTradition::Tala { beats: 7, jaati: Jaati::Misra };
        let json = serde_json::to_string(&t).unwrap();
        let t2: RhythmTradition = serde_json::from_str(&json).unwrap();
        assert_eq!(t, t2);
    }

    #[test]
    fn serde_signature() {
        let sig = RhythmicSignature::from_western(4, 4);
        let json = serde_json::to_string(&sig).unwrap();
        let sig2: RhythmicSignature = serde_json::from_str(&json).unwrap();
        assert_eq!(sig, sig2);
    }

    #[test]
    fn serde_engine() {
        let mut e = PolyrhythmEngine::new();
        e.add_rhythm("w", RhythmicSignature::from_western(4, 4));
        let json = serde_json::to_string(&e).unwrap();
        let e2: PolyrhythmEngine = serde_json::from_str(&json).unwrap();
        assert_eq!(e, e2);
    }

    #[test]
    fn serde_conversation() {
        let cr = east_meets_west();
        let json = serde_json::to_string(&cr).unwrap();
        let cr2: ConversationRhythm = serde_json::from_str(&json).unwrap();
        assert_eq!(cr, cr2);
    }

    // --- GCD/LCM tests ---

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(16, 18), 144);
        assert_eq!(lcm(0, 5), 0);
    }

    // --- Ma silence pattern tests ---

    #[test]
    fn ma_silence_ratio_zero() {
        let sig = RhythmicSignature::from_ma(4, 0.0);
        let silent = sig.silence_pattern.iter().filter(|&&s| s).count();
        assert_eq!(silent, 0);
    }

    #[test]
    fn ma_silence_ratio_high() {
        let sig = RhythmicSignature::from_ma(4, 0.5);
        let silent = sig.silence_pattern.iter().filter(|&&s| s).count();
        assert!(silent >= 6); // ~8 out of 16
    }

    // --- Beat strength at out-of-bounds wraps ---

    #[test]
    fn beat_strength_wraps() {
        let sig = RhythmicSignature::from_western(4, 4);
        assert_eq!(sig.beat_strength(0), sig.beat_strength(16));
        assert_eq!(sig.beat_strength(1), sig.beat_strength(17));
    }

    // --- Default trait ---

    #[test]
    fn engine_default() {
        let e = PolyrhythmEngine::default();
        assert_eq!(e.tick, 0);
    }
}
