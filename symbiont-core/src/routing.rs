//! Task routing for Symbiont.
//!
//! Routes tasks to the best-suited nodes based on trust, capability quality,
//! availability, and connection strength.

use crate::constants::W_INIT;
use crate::node::Node;
use crate::types::{CapabilityId, NodeId, Score, TaskId, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Priority level for tasks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[derive(Default)]
pub enum Priority {
    Low,
    #[default]
    Normal,
    High,
    Critical,
}


/// Constraints for task routing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TaskConstraints {
    /// Maximum time to complete
    pub timeout_ms: Option<u64>,
    /// Task priority
    pub priority: Priority,
    /// Minimum trust required
    pub min_trust: Option<Score>,
    /// Minimum capability quality required
    pub min_quality: Option<Score>,
    /// Preferred nodes (hints, not requirements)
    pub preferred_nodes: Vec<NodeId>,
    /// Nodes to exclude
    pub excluded_nodes: Vec<NodeId>,
}

impl TaskConstraints {
    /// Create new constraints
    pub fn new() -> Self {
        Self::default()
    }

    /// Set minimum trust
    pub fn with_min_trust(mut self, trust: Score) -> Self {
        self.min_trust = Some(trust);
        self
    }

    /// Set minimum quality
    pub fn with_min_quality(mut self, quality: Score) -> Self {
        self.min_quality = Some(quality);
        self
    }

    /// Add preferred node
    pub fn prefer(mut self, node_id: NodeId) -> Self {
        self.preferred_nodes.push(node_id);
        self
    }

    /// Exclude a node
    pub fn exclude(mut self, node_id: NodeId) -> Self {
        self.excluded_nodes.push(node_id);
        self
    }

    /// Set priority
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }

    /// Check if a node meets the constraints
    pub fn is_acceptable(&self, node: &Node, capability: CapabilityId) -> bool {
        // Check exclusions
        if self.excluded_nodes.contains(&node.id) {
            return false;
        }

        // Check minimum trust
        if let Some(min_trust) = self.min_trust {
            if node.trust.value() < min_trust.value() {
                return false;
            }
        }

        // Check minimum quality
        if let Some(min_quality) = self.min_quality {
            if node.capability_quality(capability).value() < min_quality.value() {
                return false;
            }
        }

        true
    }
}

/// A task to be routed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// Unique identifier
    pub id: TaskId,
    /// Required capabilities
    pub required_caps: Vec<CapabilityId>,
    /// Task constraints
    pub constraints: TaskConstraints,
    /// Node that originated this task
    pub origin: NodeId,
    /// Creation timestamp
    pub created: Timestamp,
}

impl Task {
    /// Create a new task
    pub fn new(id: TaskId, origin: NodeId, required_cap: CapabilityId) -> Self {
        Self {
            id,
            required_caps: vec![required_cap],
            constraints: TaskConstraints::default(),
            origin,
            created: Timestamp::now(),
        }
    }

    /// Add a required capability
    pub fn require_cap(mut self, cap: CapabilityId) -> Self {
        self.required_caps.push(cap);
        self
    }

    /// Set constraints
    pub fn with_constraints(mut self, constraints: TaskConstraints) -> Self {
        self.constraints = constraints;
        self
    }
}

/// Scoring for a candidate node
#[derive(Debug, Clone)]
pub struct CandidateScore {
    /// Node ID
    pub node_id: NodeId,
    /// Total routing score
    pub score: f64,
    /// Component breakdown
    pub components: ScoreComponents,
}

/// Breakdown of routing score components
#[derive(Debug, Clone, Default)]
pub struct ScoreComponents {
    /// Trust contribution
    pub trust: f64,
    /// Capability quality contribution
    pub capability_quality: f64,
    /// Availability (1 - load) contribution
    pub availability: f64,
    /// Connection weight contribution
    pub connection: f64,
    /// Defense (1 - threat) contribution
    pub defense: f64,
    /// Preference bonus
    pub preference_bonus: f64,
}

/// Compute routing score for a candidate
///
/// S_route = T(n) × q_cap(n) × (1 - load) × w_conn × (1 - threat) × pref_bonus
pub fn compute_routing_score(
    from_node: &Node,
    candidate: &Node,
    capability: CapabilityId,
    constraints: &TaskConstraints,
) -> CandidateScore {
    // Trust
    let trust = candidate.trust.value();

    // Capability quality
    let cap_quality = candidate.capability_quality(capability).value();

    // Availability
    let availability = 1.0 - candidate.load.value();

    // Connection weight
    let connection = from_node
        .get_connection(&candidate.id)
        .map(|c| c.w.value())
        .unwrap_or(W_INIT);

    // Defense factor
    let threat = from_node.get_threat_level(&candidate.id);
    let defense = 1.0 - threat;

    // Preference bonus
    let preference_bonus = if constraints.preferred_nodes.contains(&candidate.id) {
        1.2
    } else {
        1.0
    };

    let score = trust * cap_quality * availability * connection * defense * preference_bonus;

    CandidateScore {
        node_id: candidate.id,
        score,
        components: ScoreComponents {
            trust,
            capability_quality: cap_quality,
            availability,
            connection,
            defense,
            preference_bonus,
        },
    }
}

/// Result of routing attempt
#[derive(Debug, Clone)]
pub enum RoutingResult {
    /// Successfully found a candidate
    Success(CandidateScore),
    /// No candidates available
    NoCandidates,
    /// All candidates were filtered out by constraints
    ConstraintsNotMet,
}

impl RoutingResult {
    /// Check if routing succeeded
    pub fn is_success(&self) -> bool {
        matches!(self, RoutingResult::Success(_))
    }

    /// Get the selected node if successful
    pub fn selected_node(&self) -> Option<NodeId> {
        match self {
            RoutingResult::Success(score) => Some(score.node_id),
            _ => None,
        }
    }
}

/// Route a task to the best candidate
pub fn route_task(
    from_node: &Node,
    task: &Task,
    candidates: &HashMap<NodeId, Node>,
) -> RoutingResult {
    if task.required_caps.is_empty() {
        return RoutingResult::NoCandidates;
    }

    let required_cap = task.required_caps[0];

    // Find candidates with the required capability
    let mut scored: Vec<CandidateScore> = candidates
        .values()
        .filter(|node| {
            // Must have capability and be able to accept work
            node.has_capability(required_cap)
                && node.can_accept_capability_work(required_cap)
                && task.constraints.is_acceptable(node, required_cap)
                && node.id != from_node.id // Don't route to self
        })
        .map(|node| compute_routing_score(from_node, node, required_cap, &task.constraints))
        .collect();

    if scored.is_empty() {
        return RoutingResult::NoCandidates;
    }

    // Sort by score descending
    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    RoutingResult::Success(scored.remove(0))
}

/// Get top-k candidates for ensemble routing
pub fn route_ensemble(
    from_node: &Node,
    task: &Task,
    candidates: &HashMap<NodeId, Node>,
    k: usize,
) -> Vec<CandidateScore> {
    if task.required_caps.is_empty() || k == 0 {
        return Vec::new();
    }

    let required_cap = task.required_caps[0];

    let mut scored: Vec<CandidateScore> = candidates
        .values()
        .filter(|node| {
            node.has_capability(required_cap)
                && node.can_accept_capability_work(required_cap)
                && task.constraints.is_acceptable(node, required_cap)
                && node.id != from_node.id
        })
        .map(|node| compute_routing_score(from_node, node, required_cap, &task.constraints))
        .collect();

    scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    scored.into_iter().take(k).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::common;
    use crate::node::NodeBuilder;
    use crate::types::Weight;

    fn setup_test_network() -> (Node, HashMap<NodeId, Node>) {
        let from_id = NodeId::from_index(0);
        let from_node = NodeBuilder::new(from_id)
            .trust(Score::new(0.8))
            .capability(common::analysis())
            .build();

        let mut candidates = HashMap::new();

        // High quality candidate
        let high_quality = NodeBuilder::new(NodeId::from_index(1))
            .trust(Score::new(0.9))
            .capability(common::analysis())
            .build();

        // Low quality candidate
        let low_quality = NodeBuilder::new(NodeId::from_index(2))
            .trust(Score::new(0.3))
            .capability(common::analysis())
            .build();

        // No capability
        let no_cap = NodeBuilder::new(NodeId::from_index(3))
            .trust(Score::new(0.9))
            .build();

        candidates.insert(high_quality.id, high_quality);
        candidates.insert(low_quality.id, low_quality);
        candidates.insert(no_cap.id, no_cap);

        (from_node, candidates)
    }

    #[test]
    fn test_routing_score() {
        let (from_node, candidates) = setup_test_network();
        let high_quality = candidates.get(&NodeId::from_index(1)).unwrap();

        let score = compute_routing_score(
            &from_node,
            high_quality,
            common::analysis().id,
            &TaskConstraints::default(),
        );

        assert!(score.score > 0.0);
        assert!(score.components.trust > 0.5);
    }

    #[test]
    fn test_route_task() {
        let (from_node, candidates) = setup_test_network();

        let task = Task::new(
            TaskId::random(),
            from_node.id,
            common::analysis().id,
        );

        let result = route_task(&from_node, &task, &candidates);

        assert!(result.is_success());
        // Should select the high-quality node
        assert_eq!(result.selected_node(), Some(NodeId::from_index(1)));
    }

    #[test]
    fn test_route_with_constraints() {
        let (from_node, candidates) = setup_test_network();

        let task = Task::new(
            TaskId::random(),
            from_node.id,
            common::analysis().id,
        ).with_constraints(
            TaskConstraints::new()
                .with_min_trust(Score::new(0.2))  // Lower min_trust so NodeId 2 (0.3) passes
                .exclude(NodeId::from_index(1)), // Exclude best candidate
        );

        let result = route_task(&from_node, &task, &candidates);

        // Should still succeed with fallback candidate
        assert!(result.is_success());
        // But not the excluded node
        assert_ne!(result.selected_node(), Some(NodeId::from_index(1)));
        // Should select NodeId 2 (the only remaining candidate with capability)
        assert_eq!(result.selected_node(), Some(NodeId::from_index(2)));
    }

    #[test]
    fn test_route_no_capability() {
        let (from_node, candidates) = setup_test_network();

        // Request a capability no one has
        let task = Task::new(
            TaskId::random(),
            from_node.id,
            CapabilityId::new(99999),
        );

        let result = route_task(&from_node, &task, &candidates);
        assert!(!result.is_success());
    }

    #[test]
    fn test_route_ensemble() {
        let (from_node, candidates) = setup_test_network();

        let task = Task::new(
            TaskId::random(),
            from_node.id,
            common::analysis().id,
        );

        let top_k = route_ensemble(&from_node, &task, &candidates, 3);

        // Should get 2 (the ones with the capability)
        assert_eq!(top_k.len(), 2);
        // First should be highest scored
        assert!(top_k[0].score >= top_k[1].score);
    }

    #[test]
    fn test_connection_weight_affects_routing() {
        use crate::connection::Connection;

        let from_id = NodeId::from_index(0);
        let mut from_node = NodeBuilder::new(from_id)
            .trust(Score::new(0.8))
            .capability(common::analysis())
            .build();

        // Create two identical candidates
        let candidate1 = NodeBuilder::new(NodeId::from_index(1))
            .trust(Score::new(0.8))
            .capability(common::analysis())
            .build();

        let candidate2 = NodeBuilder::new(NodeId::from_index(2))
            .trust(Score::new(0.8))
            .capability(common::analysis())
            .build();

        // Establish connections with different weights
        let mut conn1 = Connection::new(candidate1.id);
        conn1.w = Weight::new(0.9); // High weight connection

        let mut conn2 = Connection::new(candidate2.id);
        conn2.w = Weight::new(0.2); // Low weight connection

        from_node.connections.insert(candidate1.id, conn1);
        from_node.connections.insert(candidate2.id, conn2);

        let mut candidates = HashMap::new();
        candidates.insert(candidate1.id, candidate1);
        candidates.insert(candidate2.id, candidate2);

        // Route - should prefer candidate1 due to higher connection weight
        let task = Task::new(
            TaskId::random(),
            from_node.id,
            common::analysis().id,
        );

        let result = route_task(&from_node, &task, &candidates);
        assert!(result.is_success());
        assert_eq!(result.selected_node(), Some(NodeId::from_index(1)));

        // Verify the connection weight component
        let score1 = compute_routing_score(&from_node, candidates.get(&NodeId::from_index(1)).unwrap(), common::analysis().id, &TaskConstraints::default());
        let score2 = compute_routing_score(&from_node, candidates.get(&NodeId::from_index(2)).unwrap(), common::analysis().id, &TaskConstraints::default());

        assert!(score1.components.connection > score2.components.connection);
        assert!((score1.components.connection - 0.9).abs() < 0.01);
        assert!((score2.components.connection - 0.2).abs() < 0.01);
    }
}
