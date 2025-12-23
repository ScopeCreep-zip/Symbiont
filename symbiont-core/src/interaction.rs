//! Interaction structures for recording exchanges between nodes.

use crate::constants::{OMEGA_ACC, OMEGA_HELP, OMEGA_REL, OMEGA_TIME, REUSE_BOOST, REUSE_PENALTY};
use crate::constants::{TONE_WEIGHT_COLLABORATION, TONE_WEIGHT_ENGAGEMENT, TONE_WEIGHT_FRIENDLINESS};
use crate::types::{CapabilityId, NodeId, Score, SignedScore, Timestamp};
use serde::{Deserialize, Serialize};

/// An interaction between two nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// Node that initiated the interaction
    pub initiator: NodeId,
    /// Node that responded
    pub responder: NodeId,
    /// Task volume (amount of work/value)
    pub volume: f64,
    /// Which capability was used (if any)
    pub capability: Option<CapabilityId>,
    /// Measured quality
    pub quality: Score,
    /// Measured tone
    pub tone: SignedScore,
    /// Value received by initiator
    pub exchange_in: f64,
    /// Value given by initiator
    pub exchange_out: f64,
    /// When the interaction occurred
    pub timestamp: Timestamp,
}

impl Interaction {
    /// Create a new interaction
    pub fn new(initiator: NodeId, responder: NodeId) -> Self {
        Self {
            initiator,
            responder,
            volume: 1.0,
            capability: None,
            quality: Score::HALF,
            tone: SignedScore::ZERO,
            exchange_in: 1.0,
            exchange_out: 1.0,
            timestamp: Timestamp::now(),
        }
    }

    /// Set the volume
    pub fn with_volume(mut self, volume: f64) -> Self {
        self.volume = volume;
        self
    }

    /// Set the capability
    pub fn with_capability(mut self, cap: CapabilityId) -> Self {
        self.capability = Some(cap);
        self
    }

    /// Set quality and tone
    pub fn with_outcome(mut self, quality: Score, tone: SignedScore) -> Self {
        self.quality = quality;
        self.tone = tone;
        self
    }

    /// Set exchange values
    pub fn with_exchange(mut self, received: f64, given: f64) -> Self {
        self.exchange_in = received;
        self.exchange_out = given;
        self
    }

    /// Create from feedback
    pub fn from_feedback(
        initiator: NodeId,
        responder: NodeId,
        capability: Option<CapabilityId>,
        feedback: &Feedback,
        volume: f64,
    ) -> Self {
        let quality = feedback.compute_quality();
        let tone = SignedScore::ZERO; // Tone computed separately

        Self {
            initiator,
            responder,
            volume,
            capability,
            quality,
            tone,
            exchange_in: 1.0,
            exchange_out: 1.0,
            timestamp: Timestamp::now(),
        }
    }
}

/// Feedback from a user about an interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    /// Helpfulness rating (1-5)
    pub helpfulness: u8,
    /// Accuracy rating (1-5)
    pub accuracy: u8,
    /// Relevance rating (1-5)
    pub relevance: u8,
    /// Timeliness rating (1-5)
    pub timeliness: u8,
    /// Would the user reuse this service?
    pub would_reuse: bool,
}

impl Feedback {
    /// Create new feedback
    pub fn new(
        helpfulness: u8,
        accuracy: u8,
        relevance: u8,
        timeliness: u8,
        would_reuse: bool,
    ) -> Self {
        Self {
            helpfulness: helpfulness.clamp(1, 5),
            accuracy: accuracy.clamp(1, 5),
            relevance: relevance.clamp(1, 5),
            timeliness: timeliness.clamp(1, 5),
            would_reuse,
        }
    }

    /// Create perfect feedback
    pub fn perfect() -> Self {
        Self::new(5, 5, 5, 5, true)
    }

    /// Create poor feedback
    pub fn poor() -> Self {
        Self::new(1, 1, 1, 1, false)
    }

    /// Create neutral feedback
    pub fn neutral() -> Self {
        Self::new(3, 3, 3, 3, false)
    }

    /// Compute quality score from feedback
    ///
    /// Q_raw = (ω_HELP × helpfulness + ω_ACC × accuracy +
    ///          ω_REL × relevance + ω_TIME × timeliness) / 4
    /// Q_multiplied = Q_raw × (REUSE_BOOST if would_reuse else REUSE_PENALTY)
    /// Q_normalized = (Q_multiplied - 1) / 4  // Maps to [0, 1]
    pub fn compute_quality(&self) -> Score {
        let q_raw = OMEGA_HELP * self.helpfulness as f64
            + OMEGA_ACC * self.accuracy as f64
            + OMEGA_REL * self.relevance as f64
            + OMEGA_TIME * self.timeliness as f64;

        let multiplier = if self.would_reuse {
            REUSE_BOOST
        } else {
            REUSE_PENALTY
        };

        let q_multiplied = q_raw * multiplier;

        // Map from [0.8, 6.0] range to [0, 1]
        // Min: 1 * 0.8 = 0.8
        // Max: 5 * 1.2 = 6.0
        let q_normalized = (q_multiplied - 0.8) / (6.0 - 0.8);

        Score::new(q_normalized)
    }
}

/// Signals for computing tone score
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToneSignals {
    // Engagement signals
    /// Response latency score (0 = slow, 1 = fast)
    pub latency_score: f64,
    /// Level of elaboration in response
    pub elaboration: f64,
    /// Questions asked (shows engagement)
    pub questions: f64,

    // Friendliness signals
    /// Affirmative language (0 to 1)
    pub affirmative: f64,
    /// Hedging language (0 to 1, inverted in calc)
    pub hedging: f64,
    /// Acknowledgment of other's points
    pub acknowledgment: f64,

    // Collaboration signals
    /// Offering alternatives
    pub alternatives: f64,
    /// Building on others' ideas
    pub build_on: f64,
    /// Giving credit
    pub credit_giving: f64,
}

impl ToneSignals {
    /// Create positive tone signals
    pub fn positive() -> Self {
        Self {
            latency_score: 0.8,
            elaboration: 0.8,
            questions: 0.5,
            affirmative: 0.8,
            hedging: 0.2,
            acknowledgment: 0.8,
            alternatives: 0.6,
            build_on: 0.7,
            credit_giving: 0.6,
        }
    }

    /// Create negative tone signals
    pub fn negative() -> Self {
        Self {
            latency_score: 0.0,  // Very slow response
            elaboration: 0.0,   // Minimal elaboration
            questions: 0.0,     // No questions
            affirmative: 0.0,   // No affirmative language
            hedging: 1.0,       // Maximum hedging
            acknowledgment: 0.0, // No acknowledgment
            alternatives: 0.0,  // No alternatives
            build_on: 0.0,      // Not building on ideas
            credit_giving: 0.0, // No credit given
        }
    }

    /// Create neutral tone signals
    pub fn neutral() -> Self {
        Self {
            latency_score: 0.5,
            elaboration: 0.5,
            questions: 0.3,
            affirmative: 0.5,
            hedging: 0.5,
            acknowledgment: 0.5,
            alternatives: 0.3,
            build_on: 0.3,
            credit_giving: 0.3,
        }
    }

    /// Compute tone score
    ///
    /// τ = tanh(ω_e × E + ω_f × F + ω_c × C)
    ///
    /// WHERE:
    ///     E = 0.4 × latency_score + 0.4 × elaboration + 0.2 × questions
    ///     F = 0.5 × (affirmative - 0.5) + 0.3 × (0.5 - hedging) + 0.2 × acknowledgment
    ///     C = 0.5 × alternatives + 0.3 × build_on + 0.2 × credit_giving
    pub fn compute_tone(&self) -> SignedScore {
        // Engagement
        let e = 0.4 * self.latency_score + 0.4 * self.elaboration + 0.2 * self.questions;

        // Friendliness (centered around 0)
        let f = 0.5 * (self.affirmative - 0.5)
            + 0.3 * (0.5 - self.hedging)
            + 0.2 * self.acknowledgment;

        // Collaboration
        let c = 0.5 * self.alternatives + 0.3 * self.build_on + 0.2 * self.credit_giving;

        let raw = TONE_WEIGHT_ENGAGEMENT * e
            + TONE_WEIGHT_FRIENDLINESS * f
            + TONE_WEIGHT_COLLABORATION * c;

        SignedScore::new(raw.tanh())
    }
}

/// History of interactions for a connection
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InteractionHistory {
    /// Recent interactions (most recent first)
    interactions: Vec<Interaction>,
    /// Maximum history size
    max_size: usize,
}

impl InteractionHistory {
    /// Create with default max size (100)
    pub fn new() -> Self {
        Self::with_max_size(100)
    }

    /// Create with specific max size
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            interactions: Vec::with_capacity(max_size),
            max_size,
        }
    }

    /// Add an interaction to history
    pub fn add(&mut self, interaction: Interaction) {
        self.interactions.insert(0, interaction);
        if self.interactions.len() > self.max_size {
            self.interactions.pop();
        }
    }

    /// Get recent interactions
    pub fn recent(&self, count: usize) -> &[Interaction] {
        let end = count.min(self.interactions.len());
        &self.interactions[..end]
    }

    /// Get all interactions
    pub fn all(&self) -> &[Interaction] {
        &self.interactions
    }

    /// Number of interactions in history
    pub fn len(&self) -> usize {
        self.interactions.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.interactions.is_empty()
    }

    /// Mean quality of recent interactions
    pub fn mean_quality(&self, count: usize) -> Score {
        let recent = self.recent(count);
        if recent.is_empty() {
            return Score::HALF;
        }
        let sum: f64 = recent.iter().map(|i| i.quality.value()).sum();
        Score::new(sum / recent.len() as f64)
    }

    /// Variance of quality scores
    pub fn quality_variance(&self, count: usize) -> f64 {
        let recent = self.recent(count);
        if recent.len() < 2 {
            return 0.0;
        }

        let mean = self.mean_quality(count).value();
        let sum_sq: f64 = recent
            .iter()
            .map(|i| (i.quality.value() - mean).powi(2))
            .sum();
        sum_sq / recent.len() as f64
    }

    /// Count unique partners in recent history
    pub fn unique_partners(&self, count: usize) -> usize {
        use std::collections::HashSet;
        let recent = self.recent(count);
        let partners: HashSet<_> = recent.iter().map(|i| i.responder).collect();
        partners.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_quality() {
        // Perfect feedback
        let perfect = Feedback::perfect();
        let q = perfect.compute_quality();
        assert!(q.value() > 0.9);

        // Poor feedback
        let poor = Feedback::poor();
        let q = poor.compute_quality();
        assert!(q.value() < 0.1);

        // Neutral
        let neutral = Feedback::neutral();
        let q = neutral.compute_quality();
        assert!(q.value() > 0.3 && q.value() < 0.7);
    }

    #[test]
    fn test_tone_computation() {
        let positive = ToneSignals::positive();
        let tone = positive.compute_tone();
        assert!(tone.value() > 0.0);

        let negative = ToneSignals::negative();
        let tone = negative.compute_tone();
        assert!(tone.value() < 0.0);

        let neutral = ToneSignals::neutral();
        let tone = neutral.compute_tone();
        assert!(tone.value().abs() < 0.3);
    }

    #[test]
    fn test_interaction_history() {
        let mut history = InteractionHistory::with_max_size(10);

        for i in 0..15 {
            let interaction = Interaction::new(
                NodeId::from_index(0),
                NodeId::from_index(i as u64),
            ).with_outcome(Score::new(0.5 + i as f64 * 0.03), SignedScore::ZERO);
            history.add(interaction);
        }

        // Should only keep last 10
        assert_eq!(history.len(), 10);

        // Most recent should have highest quality
        let recent = history.recent(1);
        assert!(recent[0].quality.value() > 0.8);
    }

    #[test]
    fn test_unique_partners() {
        let mut history = InteractionHistory::new();

        // Add interactions with 3 unique partners, some repeated
        for _ in 0..5 {
            history.add(Interaction::new(
                NodeId::from_index(0),
                NodeId::from_index(1),
            ));
        }
        for _ in 0..3 {
            history.add(Interaction::new(
                NodeId::from_index(0),
                NodeId::from_index(2),
            ));
        }
        history.add(Interaction::new(
            NodeId::from_index(0),
            NodeId::from_index(3),
        ));

        assert_eq!(history.unique_partners(100), 3);
    }
}
