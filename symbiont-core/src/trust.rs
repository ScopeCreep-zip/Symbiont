//! Trust computation for Symbiont nodes.
//!
//! Trust is computed from quality, reciprocity, social proof, and diversity,
//! with a diversity cap to prevent high trust without broad interaction.

use crate::constants::{
    CONFIDENCE_MEMORY, TRUST_WEIGHT_DIVERSITY, TRUST_WEIGHT_QUALITY, TRUST_WEIGHT_RECIPROCITY,
    TRUST_WEIGHT_SOCIAL,
};
use crate::math::{apply_diversity_cap, sigmoid};
use crate::node::Node;
use crate::types::{NodeId, Score};
use std::collections::HashMap;

/// Compute global trust for a node
///
/// T(n) = (w_Q × Q_agg + w_R × σ(R_agg) + w_S × S_social + w_D × D_diversity) / Σw
///
/// With diversity cap: T_final = min(T(n), D_diversity + 0.3)
pub fn compute_trust(node: &Node) -> Score {
    // Aggregate quality (weighted by capability volume)
    let q_agg = node.aggregate_capability_quality();

    // Aggregate reciprocity from connections
    let r_agg = if node.connections.is_empty() {
        0.0
    } else {
        let sum: f64 = node.connections.values().map(|c| c.r).sum();
        sum / node.connections.len() as f64
    };

    // Social proof (could be computed from affirmations, for now use connection quality)
    let s_social = compute_social_proof(node);

    // Diversity
    let d_diversity = node.diversity_score();

    // Weighted combination
    let total_weight = TRUST_WEIGHT_QUALITY
        + TRUST_WEIGHT_RECIPROCITY
        + TRUST_WEIGHT_SOCIAL
        + TRUST_WEIGHT_DIVERSITY;

    let trust = (TRUST_WEIGHT_QUALITY * q_agg.value()
        + TRUST_WEIGHT_RECIPROCITY * sigmoid(r_agg).value()
        + TRUST_WEIGHT_SOCIAL * s_social.value()
        + TRUST_WEIGHT_DIVERSITY * d_diversity.value())
        / total_weight;

    let base_trust = Score::new(trust);

    // Apply diversity cap and trust cap
    let capped = apply_diversity_cap(base_trust, d_diversity);
    Score::new(capped.value().min(node.trust_cap.value()))
}

/// Compute social proof score
///
/// Based on the quality of connections and their trust levels
fn compute_social_proof(node: &Node) -> Score {
    if node.connections.is_empty() {
        return Score::ZERO;
    }

    // Use mean connection quality as proxy for social proof
    let sum: f64 = node.connections.values().map(|c| c.q.value()).sum();
    Score::new(sum / node.connections.len() as f64)
}

/// Update self-confidence based on affirmations
///
/// S_conf_new = α_conf × S_conf + (1 - α_conf) × A_mean
///
/// WHERE:
///     A_mean = Σ(T_affirmer × strength) / Σ(T_affirmer)
pub fn update_confidence(current: Score, affirmation_strength: f64) -> Score {
    let new_conf =
        CONFIDENCE_MEMORY * current.value() + (1.0 - CONFIDENCE_MEMORY) * affirmation_strength;
    Score::new(new_conf)
}

/// Compute trust for all nodes in a network
pub fn compute_network_trust(nodes: &HashMap<NodeId, Node>) -> HashMap<NodeId, Score> {
    nodes
        .iter()
        .map(|(&id, node)| (id, compute_trust(node)))
        .collect()
}

/// Trust metrics for a network
#[derive(Debug, Clone, Default)]
pub struct TrustMetrics {
    /// Mean trust across all nodes
    pub mean_trust: f64,
    /// Variance of trust scores
    pub trust_variance: f64,
    /// Minimum trust
    pub min_trust: f64,
    /// Maximum trust
    pub max_trust: f64,
    /// Number of nodes
    pub node_count: usize,
    /// Number of high-trust nodes (> 0.7)
    pub high_trust_count: usize,
    /// Number of low-trust nodes (< 0.3)
    pub low_trust_count: usize,
}

impl TrustMetrics {
    /// Compute metrics from a collection of trust scores
    pub fn from_scores(scores: impl Iterator<Item = Score>) -> Self {
        let scores: Vec<_> = scores.collect();

        if scores.is_empty() {
            return Self::default();
        }

        let values: Vec<f64> = scores.iter().map(|s| s.value()).collect();
        let n = values.len() as f64;

        let mean_trust = values.iter().sum::<f64>() / n;
        let trust_variance = values.iter().map(|v| (v - mean_trust).powi(2)).sum::<f64>() / n;
        let min_trust = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_trust = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let high_trust_count = values.iter().filter(|&&v| v > 0.7).count();
        let low_trust_count = values.iter().filter(|&&v| v < 0.3).count();

        Self {
            mean_trust,
            trust_variance,
            min_trust,
            max_trust,
            node_count: values.len(),
            high_trust_count,
            low_trust_count,
        }
    }

    /// Compute metrics from a network
    pub fn from_network(nodes: &HashMap<NodeId, Node>) -> Self {
        let trust_scores = compute_network_trust(nodes);
        Self::from_scores(trust_scores.values().cloned())
    }
}

/// Check if a node has achieved stable trust
///
/// A node has stable trust if its trust variance over recent history is low
pub fn is_trust_stable(node: &Node, variance_threshold: f64) -> bool {
    let quality_variance = node.history.quality_variance(50);
    quality_variance < variance_threshold
}

/// Compute trust delta (change from previous value)
pub fn trust_delta(current: Score, previous: Score) -> f64 {
    current.value() - previous.value()
}

/// Categories of trust level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustLevel {
    /// Very low trust (0 - 0.2)
    VeryLow,
    /// Low trust (0.2 - 0.4)
    Low,
    /// Medium trust (0.4 - 0.6)
    Medium,
    /// High trust (0.6 - 0.8)
    High,
    /// Very high trust (0.8 - 1.0)
    VeryHigh,
}

impl TrustLevel {
    /// Categorize a trust score
    pub fn from_score(score: Score) -> Self {
        match score.value() {
            v if v < 0.2 => TrustLevel::VeryLow,
            v if v < 0.4 => TrustLevel::Low,
            v if v < 0.6 => TrustLevel::Medium,
            v if v < 0.8 => TrustLevel::High,
            _ => TrustLevel::VeryHigh,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::common;
    use crate::connection::Connection;

    #[test]
    fn test_compute_trust_new_node() {
        let id = NodeId::from_index(1);
        let node = Node::new(id);

        let trust = compute_trust(&node);
        // New node with no connections should have low-medium trust
        assert!(trust.value() >= 0.0 && trust.value() <= 1.0);
    }

    #[test]
    fn test_compute_trust_with_good_connections() {
        let id = NodeId::from_index(1);
        let mut node = Node::with_capabilities(id, vec![common::analysis()]);

        // Add many good connections
        for i in 0..50 {
            let partner = NodeId::from_index(100 + i);
            let mut conn = Connection::new(partner);
            conn.r = 1.0; // Good reciprocity
            conn.q = Score::new(0.9); // High quality
            node.connections.insert(partner, conn);
        }

        // Add interaction history for diversity (50 unique partners = 50% diversity)
        for i in 0..50 {
            let partner = NodeId::from_index(100 + i);
            node.handle_outgoing_interaction(
                partner,
                1.0,
                1.0,
                1.0,
                Score::new(0.9),
                crate::types::SignedScore::ZERO,
                None,
            );
        }

        // Diversity = 50/100 = 0.5, so cap = 0.5 + 0.3 = 0.8
        let trust = compute_trust(&node);
        assert!(trust.value() > 0.5);
    }

    #[test]
    fn test_diversity_cap() {
        let id = NodeId::from_index(1);
        let mut node = Node::new(id);

        // Only interact with one partner (low diversity)
        for _ in 0..100 {
            node.handle_outgoing_interaction(
                NodeId::from_index(2),
                1.0,
                1.0,
                1.0,
                Score::ONE,
                crate::types::SignedScore::ONE,
                None,
            );
        }

        // Diversity is 1/100 = 0.01
        let diversity = node.diversity_score();
        assert!(diversity.value() < 0.1);

        // Trust should be capped at diversity + 0.3
        let trust = compute_trust(&node);
        assert!(trust.value() <= diversity.value() + 0.31); // Small epsilon for float
    }

    #[test]
    fn test_update_confidence() {
        let current = Score::HALF;

        // High affirmation should increase confidence
        let updated = update_confidence(current, 0.9);
        assert!(updated.value() > current.value());

        // Low affirmation should decrease confidence
        let decreased = update_confidence(Score::new(0.8), 0.2);
        assert!(decreased.value() < 0.8);
    }

    #[test]
    fn test_trust_level_categorization() {
        assert_eq!(TrustLevel::from_score(Score::new(0.1)), TrustLevel::VeryLow);
        assert_eq!(TrustLevel::from_score(Score::new(0.3)), TrustLevel::Low);
        assert_eq!(TrustLevel::from_score(Score::new(0.5)), TrustLevel::Medium);
        assert_eq!(TrustLevel::from_score(Score::new(0.7)), TrustLevel::High);
        assert_eq!(TrustLevel::from_score(Score::new(0.9)), TrustLevel::VeryHigh);
    }

    #[test]
    fn test_trust_metrics() {
        let scores = vec![
            Score::new(0.2),
            Score::new(0.4),
            Score::new(0.6),
            Score::new(0.8),
        ];

        let metrics = TrustMetrics::from_scores(scores.into_iter());

        assert_eq!(metrics.node_count, 4);
        assert!((metrics.mean_trust - 0.5).abs() < 0.01);
        assert_eq!(metrics.low_trust_count, 1);
        assert_eq!(metrics.high_trust_count, 1);
    }
}
