//! Adversary detection for Symbiont.
//!
//! Detects strategic adversaries, Sybil attacks, and collusion rings.

use crate::constants::{ADVERSARY_DROP, COLLUSION_THRESHOLD, DIVERSITY_THRESHOLD};
use crate::interaction::InteractionHistory;
use crate::node::{Node, ThreatType};
use crate::types::{NodeId, Score};
use std::collections::{HashMap, HashSet};

/// Result of adversary detection
#[derive(Debug, Clone)]
pub struct DetectionResult {
    /// Node being analyzed
    pub node_id: NodeId,
    /// Detected threat type (if any)
    pub threat_type: Option<ThreatType>,
    /// Confidence in detection
    pub confidence: Score,
    /// Reason for detection
    pub reason: String,
}

impl DetectionResult {
    /// Create a clean result (no threat detected)
    pub fn clean(node_id: NodeId) -> Self {
        Self {
            node_id,
            threat_type: None,
            confidence: Score::ZERO,
            reason: String::from("No anomalies detected"),
        }
    }

    /// Create a threat detection result
    pub fn threat(node_id: NodeId, threat_type: ThreatType, confidence: Score, reason: String) -> Self {
        Self {
            node_id,
            threat_type: Some(threat_type),
            confidence,
            reason,
        }
    }

    /// Check if a threat was detected
    pub fn is_threat(&self) -> bool {
        self.threat_type.is_some() && self.confidence.value() > 0.5
    }
}

/// Detect strategic adversary behavior
///
/// Strategic adversaries build trust through good behavior early,
/// then defect once they've gained sufficient trust.
///
/// Detection signals:
/// - Suspiciously perfect early behavior (quality > 0.95, variance < 0.01)
/// - Quality drop after building trust
pub fn detect_strategic_adversary(node: &Node, history: &InteractionHistory) -> DetectionResult {
    let interactions = history.all();

    if interactions.len() < 100 {
        return DetectionResult::clean(node.id);
    }

    // Split history into early and recent
    let mid = interactions.len() / 2;
    let early = &interactions[mid..]; // Older interactions (at end due to insert order)
    let recent = &interactions[..mid]; // Recent interactions (at start)

    // Calculate early stats
    let early_quality: f64 = early.iter().map(|i| i.quality.value()).sum::<f64>() / early.len() as f64;
    let early_variance: f64 = {
        let mean = early_quality;
        early.iter().map(|i| (i.quality.value() - mean).powi(2)).sum::<f64>() / early.len() as f64
    };

    // Calculate recent stats
    let recent_quality: f64 = recent.iter().map(|i| i.quality.value()).sum::<f64>() / recent.len() as f64;

    // Check for suspiciously perfect early behavior
    if early_quality > 0.95 && early_variance < 0.01 {
        return DetectionResult::threat(
            node.id,
            ThreatType::Strategic,
            Score::new(0.7),
            String::from("Suspiciously perfect early behavior"),
        );
    }

    // Check for quality drop after building trust
    let quality_drop = early_quality - recent_quality;
    if node.trust.value() > 0.7 && quality_drop > ADVERSARY_DROP {
        let confidence = Score::new((quality_drop / 0.5).min(1.0));
        return DetectionResult::threat(
            node.id,
            ThreatType::Strategic,
            confidence,
            format!("Quality dropped by {quality_drop:.2} after building trust"),
        );
    }

    DetectionResult::clean(node.id)
}

/// Detect low diversity (potential Sybil or isolation)
pub fn detect_low_diversity(node: &Node) -> DetectionResult {
    let diversity = node.diversity_score();

    if diversity.value() < DIVERSITY_THRESHOLD {
        // Confidence increases as diversity decreases
        let confidence = 0.5 + 0.3 * (1.0 - diversity.value() / DIVERSITY_THRESHOLD);
        DetectionResult::threat(
            node.id,
            ThreatType::Sybil,
            Score::new(confidence),
            format!("Low interaction diversity: {:.2}", diversity.value()),
        )
    } else {
        DetectionResult::clean(node.id)
    }
}

/// A suspected collusion cluster
#[derive(Debug, Clone)]
pub struct CollusionCluster {
    /// Nodes in the cluster
    pub members: HashSet<NodeId>,
    /// Internal density (how interconnected)
    pub internal_density: f64,
    /// External ratio (connections outside)
    pub external_ratio: f64,
    /// Mean mutual rating
    pub mutual_rating: f64,
    /// Confidence in detection
    pub confidence: Score,
}

impl CollusionCluster {
    /// Check if this looks like collusion
    pub fn is_suspicious(&self) -> bool {
        self.internal_density > COLLUSION_THRESHOLD
            && self.external_ratio < 1.0
            && self.mutual_rating > 0.9
    }
}

/// Detect collusion rings in the network
///
/// Looks for clusters of nodes that:
/// - Have high internal connection density
/// - Low external connections relative to size
/// - Suspiciously high mutual ratings
pub fn detect_collusion(nodes: &HashMap<NodeId, Node>) -> Vec<CollusionCluster> {
    // Build interaction graph
    let mut graph: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();

    for (id, node) in nodes {
        let neighbors: HashSet<NodeId> = node.connections.keys().cloned().collect();
        graph.insert(*id, neighbors);
    }

    // Find communities using simple connected component analysis
    // (In production, use more sophisticated community detection)
    let communities = find_connected_components(&graph);

    let mut suspicious = Vec::new();

    for community in communities {
        if community.len() < 3 {
            continue;
        }

        // Calculate internal density
        let max_edges = community.len() * (community.len() - 1) / 2;
        let mut actual_edges = 0;

        for node_id in &community {
            if let Some(neighbors) = graph.get(node_id) {
                actual_edges += neighbors.iter().filter(|n| community.contains(n)).count();
            }
        }
        actual_edges /= 2; // Each edge counted twice

        let internal_density = if max_edges > 0 {
            actual_edges as f64 / max_edges as f64
        } else {
            0.0
        };

        // Calculate external connections
        let mut external_edges = 0;
        for node_id in &community {
            if let Some(neighbors) = graph.get(node_id) {
                external_edges += neighbors.iter().filter(|n| !community.contains(n)).count();
            }
        }
        let expected_external = community.len() as f64 * 0.5;
        let external_ratio = if expected_external > 0.0 {
            external_edges as f64 / expected_external
        } else {
            0.0
        };

        // Calculate mutual ratings
        let mut rating_sum = 0.0;
        let mut rating_count = 0;

        for node_id in &community {
            if let Some(node) = nodes.get(node_id) {
                for (partner_id, conn) in &node.connections {
                    if community.contains(partner_id) {
                        rating_sum += conn.q.value();
                        rating_count += 1;
                    }
                }
            }
        }

        let mutual_rating = if rating_count > 0 {
            rating_sum / rating_count as f64
        } else {
            0.0
        };

        let cluster = CollusionCluster {
            members: community,
            internal_density,
            external_ratio,
            mutual_rating,
            confidence: Score::new(if internal_density > 0.8 { 0.7 } else { 0.3 }),
        };

        if cluster.is_suspicious() {
            suspicious.push(cluster);
        }
    }

    suspicious
}

/// Simple connected components finder
fn find_connected_components(graph: &HashMap<NodeId, HashSet<NodeId>>) -> Vec<HashSet<NodeId>> {
    let mut visited: HashSet<NodeId> = HashSet::new();
    let mut components = Vec::new();

    for &start in graph.keys() {
        if visited.contains(&start) {
            continue;
        }

        let mut component = HashSet::new();
        let mut stack = vec![start];

        while let Some(node) = stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);
            component.insert(node);

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }

        components.push(component);
    }

    components
}

/// Detect quality fraud (fake positive reviews)
pub fn detect_quality_fraud(node: &Node) -> DetectionResult {
    // Check for unusual patterns in quality scores
    let quality_variance = node.history.quality_variance(50);

    // Very low variance might indicate coordinated fake reviews
    if quality_variance < 0.001 && node.history.len() > 30 {
        let mean_quality = node.history.mean_quality(50);
        if mean_quality.value() > 0.95 {
            return DetectionResult::threat(
                node.id,
                ThreatType::QualityFraud,
                Score::new(0.6),
                String::from("Suspiciously uniform high quality scores"),
            );
        }
    }

    DetectionResult::clean(node.id)
}

/// Run all detection checks on a node
pub fn detect_all_threats(node: &Node, history: &InteractionHistory) -> Vec<DetectionResult> {
    let results = vec![
        detect_strategic_adversary(node, history),
        detect_low_diversity(node),
        detect_quality_fraud(node),
    ];

    // Filter to only actual threats
    results.into_iter().filter(|r| r.is_threat()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interaction::Interaction;
    use crate::types::SignedScore;

    #[test]
    fn test_detection_result() {
        let clean = DetectionResult::clean(NodeId::from_index(1));
        assert!(!clean.is_threat());

        let threat = DetectionResult::threat(
            NodeId::from_index(1),
            ThreatType::Strategic,
            Score::new(0.8),
            String::from("Test threat"),
        );
        assert!(threat.is_threat());
    }

    #[test]
    fn test_strategic_adversary_not_enough_history() {
        let node = Node::new(NodeId::from_index(1));
        let history = InteractionHistory::new();

        let result = detect_strategic_adversary(&node, &history);
        assert!(!result.is_threat());
    }

    #[test]
    fn test_strategic_adversary_quality_drop() {
        let mut node = Node::new(NodeId::from_index(1));
        node.trust = Score::new(0.8);

        let mut history = InteractionHistory::new();

        // Add good early history
        for _ in 0..50 {
            let interaction = Interaction::new(NodeId::from_index(1), NodeId::from_index(2))
                .with_outcome(Score::new(0.9), SignedScore::ZERO);
            history.add(interaction);
        }

        // Add poor recent history
        for _ in 0..50 {
            let interaction = Interaction::new(NodeId::from_index(1), NodeId::from_index(2))
                .with_outcome(Score::new(0.3), SignedScore::ZERO);
            history.add(interaction);
        }

        let result = detect_strategic_adversary(&node, &history);
        assert!(result.is_threat());
        assert_eq!(result.threat_type, Some(ThreatType::Strategic));
    }

    #[test]
    fn test_low_diversity_detection() {
        let node = Node::new(NodeId::from_index(1));
        // New node has no history, so low diversity

        let result = detect_low_diversity(&node);
        assert!(result.is_threat());
    }

    #[test]
    fn test_connected_components() {
        let mut graph: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();

        // Create two disconnected clusters
        // Cluster 1: nodes 1, 2, 3
        let n1 = NodeId::from_index(1);
        let n2 = NodeId::from_index(2);
        let n3 = NodeId::from_index(3);

        graph.insert(n1, [n2, n3].into_iter().collect());
        graph.insert(n2, [n1, n3].into_iter().collect());
        graph.insert(n3, [n1, n2].into_iter().collect());

        // Cluster 2: nodes 4, 5
        let n4 = NodeId::from_index(4);
        let n5 = NodeId::from_index(5);

        graph.insert(n4, [n5].into_iter().collect());
        graph.insert(n5, [n4].into_iter().collect());

        let components = find_connected_components(&graph);
        assert_eq!(components.len(), 2);

        let sizes: Vec<_> = components.iter().map(|c| c.len()).collect();
        assert!(sizes.contains(&3));
        assert!(sizes.contains(&2));
    }
}
