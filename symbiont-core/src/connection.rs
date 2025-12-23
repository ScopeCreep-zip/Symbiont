//! Connection structure and dynamics for Symbiont.
//!
//! Connections represent relationships between nodes, governed by the Physarum equation.

use crate::constants::{ALPHA, DELTA, GAMMA, LAMBDA, MU, THETA};
use crate::math::{
    exchange_ratio_log, quality_multiplier, reciprocity_sigmoid, tone_multiplier,
};
use crate::types::{CapabilityId, NodeId, Score, SignedScore, Timestamp, Weight};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A connection between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Partner node ID
    pub partner_id: NodeId,
    /// Connection weight w ∈ [W_MIN, W_MAX]
    pub w: Weight,
    /// Reciprocity score r (unbounded, but typically in [-3, 3])
    pub r: f64,
    /// Quality score q ∈ [0, 1] (global)
    pub q: Score,
    /// Per-capability quality scores
    pub capability_qualities: HashMap<CapabilityId, Score>,
    /// Tone score τ ∈ [-1, 1]
    pub tau: SignedScore,
    /// Priming level π ∈ [0, 1]
    pub pi: Score,
    /// Last active timestamp
    pub last_active: Timestamp,
    /// Interaction count
    pub count: u32,
}

impl Connection {
    /// Create a new connection with default values
    pub fn new(partner_id: NodeId) -> Self {
        Self {
            partner_id,
            w: Weight::INIT,
            r: 0.0,
            q: Score::HALF,
            capability_qualities: HashMap::new(),
            tau: SignedScore::ZERO,
            pi: Score::ZERO,
            last_active: Timestamp::now(),
            count: 0,
        }
    }

    /// Create a connection with a specific initial weight
    pub fn with_weight(partner_id: NodeId, weight: Weight) -> Self {
        Self {
            partner_id,
            w: weight,
            r: 0.0,
            q: Score::HALF,
            capability_qualities: HashMap::new(),
            tau: SignedScore::ZERO,
            pi: Score::ZERO,
            last_active: Timestamp::now(),
            count: 0,
        }
    }

    /// Update reciprocity based on exchange ratio
    ///
    /// r_new = λ × r + (1 - λ) × (log(ρ + ε) + θ × (q - 0.5))
    pub fn update_reciprocity(&mut self, exchange_in: f64, exchange_out: f64, quality: Score) {
        let log_rho = exchange_ratio_log(exchange_in, exchange_out);
        let quality_adj = THETA * (quality.value() - 0.5);
        self.r = LAMBDA * self.r + (1.0 - LAMBDA) * (log_rho + quality_adj);
    }

    /// Update quality score using EMA
    pub fn update_quality(&mut self, observed_quality: Score) {
        let new_q = LAMBDA * self.q.value() + (1.0 - LAMBDA) * observed_quality.value();
        self.q = Score::new(new_q);
    }

    /// Update per-capability quality score using EMA
    pub fn update_capability_quality(&mut self, capability: CapabilityId, observed_quality: Score) {
        let current = self.capability_qualities.get(&capability).copied().unwrap_or(Score::HALF);
        let new_q = LAMBDA * current.value() + (1.0 - LAMBDA) * observed_quality.value();
        self.capability_qualities.insert(capability, Score::new(new_q));
    }

    /// Get quality for a specific capability (falls back to global quality)
    pub fn capability_quality(&self, capability: CapabilityId) -> Score {
        self.capability_qualities.get(&capability).copied().unwrap_or(self.q)
    }

    /// Update tone score using EMA
    pub fn update_tone(&mut self, observed_tone: SignedScore) {
        let new_tau = LAMBDA * self.tau.value() + (1.0 - LAMBDA) * observed_tone.value();
        self.tau = SignedScore::new(new_tau);
    }

    /// Compute reinforcement term Φ
    ///
    /// Φ = γ × |Q|^μ × σ(r) × ψ(q) × φ(τ)
    pub fn compute_reinforcement(&self, volume: f64) -> f64 {
        let flow = volume.abs().powf(MU);
        let sigma_r = reciprocity_sigmoid(self.r);
        let psi_q = quality_multiplier(self.q);
        let phi_tau = tone_multiplier(self.tau);

        GAMMA * flow * sigma_r.value() * psi_q * phi_tau
    }

    /// Update connection weight based on interaction
    ///
    /// w_new = clamp(w + Δt × (Φ - α×w - D), W_MIN, W_MAX)
    pub fn update_weight(&mut self, volume: f64, threat_level: f64, dt: f64) {
        let phi = self.compute_reinforcement(volume);
        let decay = ALPHA * self.w.value();
        let defense = DELTA * threat_level;

        let delta_w = dt * (phi - decay - defense);
        self.w = self.w + delta_w;
    }

    /// Full update from an interaction outcome
    pub fn process_interaction(
        &mut self,
        volume: f64,
        exchange_in: f64,
        exchange_out: f64,
        quality: Score,
        tone: SignedScore,
        threat_level: f64,
    ) {
        // Update reciprocity
        self.update_reciprocity(exchange_in, exchange_out, quality);

        // Update quality
        self.update_quality(quality);

        // Update tone
        self.update_tone(tone);

        // Update weight (dt = 1.0 for discrete updates)
        self.update_weight(volume, threat_level, 1.0);

        // Update metadata
        self.last_active = Timestamp::now();
        self.count += 1;
    }

    /// Apply passive decay (for idle connections)
    pub fn apply_decay(&mut self, dt: f64) {
        let decay = ALPHA * self.w.value() * dt;
        self.w = Weight::new(self.w.value() - decay);
    }

    /// Check if connection is considered idle
    pub fn is_idle(&self, threshold_ms: u64) -> bool {
        self.last_active.is_older_than(threshold_ms)
    }

    /// Increase priming level
    pub fn increase_priming(&mut self, boost: f64) {
        self.pi = Score::new((self.pi.value() + boost).min(1.0));
    }

    /// Decay priming level
    pub fn decay_priming(&mut self, decay_factor: f64) {
        self.pi = Score::new(self.pi.value() * decay_factor);
    }
}

/// Statistics about a set of connections
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// Number of connections
    pub count: usize,
    /// Mean weight
    pub mean_weight: f64,
    /// Mean reciprocity
    pub mean_reciprocity: f64,
    /// Mean quality
    pub mean_quality: f64,
    /// Total interaction count
    pub total_interactions: u32,
}

impl ConnectionStats {
    /// Compute statistics from a collection of connections
    pub fn from_connections<'a>(connections: impl Iterator<Item = &'a Connection>) -> Self {
        let conns: Vec<_> = connections.collect();

        if conns.is_empty() {
            return Self::default();
        }

        let count = conns.len();
        let mean_weight = conns.iter().map(|c| c.w.value()).sum::<f64>() / count as f64;
        let mean_reciprocity = conns.iter().map(|c| c.r).sum::<f64>() / count as f64;
        let mean_quality = conns.iter().map(|c| c.q.value()).sum::<f64>() / count as f64;
        let total_interactions = conns.iter().map(|c| c.count).sum();

        Self {
            count,
            mean_weight,
            mean_reciprocity,
            mean_quality,
            total_interactions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::W_INIT;

    #[test]
    fn test_new_connection() {
        let id = NodeId::from_index(1);
        let conn = Connection::new(id);

        assert_eq!(conn.w.value(), W_INIT);
        assert_eq!(conn.r, 0.0);
        assert_eq!(conn.q.value(), 0.5);
        assert_eq!(conn.count, 0);
    }

    #[test]
    fn test_reciprocity_update() {
        let id = NodeId::from_index(1);
        let mut conn = Connection::new(id);

        // Balanced exchange → reciprocity stays near 0
        conn.update_reciprocity(1.0, 1.0, Score::HALF);
        assert!(conn.r.abs() < 0.1);

        // Receiving more → positive reciprocity
        conn.update_reciprocity(2.0, 1.0, Score::HALF);
        assert!(conn.r > 0.0);
    }

    #[test]
    fn test_quality_ema() {
        let id = NodeId::from_index(1);
        let mut conn = Connection::new(id);

        // Start at 0.5, observe high quality
        conn.update_quality(Score::new(1.0));
        // New quality should be between 0.5 and 1.0
        assert!(conn.q.value() > 0.5 && conn.q.value() < 1.0);

        // Multiple high observations should push it higher
        // With LAMBDA=0.9 (90% weight on history), need more iterations to converge
        for _ in 0..30 {
            conn.update_quality(Score::ONE);
        }
        // After 30+ iterations with perfect quality, should exceed 0.9
        assert!(conn.q.value() > 0.9);
    }

    #[test]
    fn test_reinforcement_signs() {
        let id = NodeId::from_index(1);
        let mut conn = Connection::new(id);

        // Neutral reciprocity → near-zero reinforcement
        let phi_neutral = conn.compute_reinforcement(1.0);
        assert!(phi_neutral.abs() < 0.01);

        // Positive reciprocity → positive reinforcement
        conn.r = 1.0;
        let phi_pos = conn.compute_reinforcement(1.0);
        assert!(phi_pos > 0.0);

        // Negative reciprocity → negative reinforcement
        conn.r = -1.0;
        let phi_neg = conn.compute_reinforcement(1.0);
        assert!(phi_neg < 0.0);
    }

    #[test]
    fn test_weight_update() {
        let id = NodeId::from_index(1);
        let mut conn = Connection::new(id);
        let initial_w = conn.w.value();

        // Positive reciprocity should increase weight
        conn.r = 1.0;
        conn.q = Score::ONE;
        conn.tau = SignedScore::ONE;
        conn.update_weight(1.0, 0.0, 1.0);

        assert!(conn.w.value() > initial_w);

        // High threat level should decrease weight
        let mut conn2 = Connection::new(id);
        conn2.update_weight(0.0, 1.0, 1.0);
        assert!(conn2.w.value() < W_INIT);
    }

    #[test]
    fn test_process_interaction() {
        let id = NodeId::from_index(1);
        let mut conn = Connection::new(id);

        conn.process_interaction(
            1.0,                 // volume
            2.0,                 // exchange_in
            1.0,                 // exchange_out
            Score::new(0.9),     // quality
            SignedScore::new(0.8), // tone
            0.0,                 // threat_level
        );

        assert!(conn.r > 0.0); // Positive reciprocity
        assert!(conn.q.value() > 0.5); // Quality increased
        assert!(conn.tau.value() > 0.0); // Positive tone
        assert_eq!(conn.count, 1);
    }

    #[test]
    fn test_connection_stats() {
        let conns = vec![
            Connection::with_weight(NodeId::from_index(1), Weight::new(0.5)),
            Connection::with_weight(NodeId::from_index(2), Weight::new(0.7)),
        ];

        let stats = ConnectionStats::from_connections(conns.iter());
        assert_eq!(stats.count, 2);
        assert!((stats.mean_weight - 0.6).abs() < 0.01);
    }
}
