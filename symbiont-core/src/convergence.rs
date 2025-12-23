//! Convergence tracking and agree-to-disagree protocol for Symbiont.

use crate::math::variance;
use crate::types::{NodeId, Score, TaskId, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// State of convergence for a decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConvergenceState {
    /// Positions are widely dispersed
    Polarized,
    /// Low convergence, making no progress
    Stuck,
    /// Actively exploring options
    Exploring,
    /// Moving toward agreement
    Converging,
    /// Agreement reached
    Converged,
}

impl ConvergenceState {
    /// Determine state from convergence score
    pub fn from_score(score: Score) -> Self {
        match score.value() {
            v if v > 0.85 => ConvergenceState::Converged,
            v if v > 0.60 => ConvergenceState::Converging,
            v if v > 0.40 => ConvergenceState::Exploring,
            v if v > 0.20 => ConvergenceState::Stuck,
            _ => ConvergenceState::Polarized,
        }
    }
}

/// Trend of convergence over time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConvergenceTrend {
    /// Score is increasing
    Improving,
    /// Score is stable
    Stable,
    /// Score is decreasing
    Declining,
}

/// A position held by a node on a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Node holding this position
    pub node_id: NodeId,
    /// Position value (interpretation depends on task)
    pub value: f64,
    /// Confidence in this position
    pub confidence: Score,
    /// When this position was stated
    pub timestamp: Timestamp,
}

/// Tracker for convergence on a specific task
#[derive(Debug, Clone)]
pub struct ConvergenceTracker {
    /// Task being tracked
    pub task_id: TaskId,
    /// Current positions by node
    positions: HashMap<NodeId, Position>,
    /// History of convergence scores
    score_history: Vec<(Timestamp, Score)>,
    /// Number of rounds
    pub round_count: u32,
    /// Maximum variance (for normalization)
    max_variance: f64,
    /// Current convergence score
    current_score: Score,
}

impl ConvergenceTracker {
    /// Create a new tracker
    pub fn new(task_id: TaskId, max_variance: f64) -> Self {
        Self {
            task_id,
            positions: HashMap::new(),
            score_history: Vec::new(),
            round_count: 0,
            max_variance,
            current_score: Score::ZERO,
        }
    }

    /// Record a position from a node
    pub fn record_position(&mut self, position: Position) {
        self.positions.insert(position.node_id, position);
        self.update_score();
    }

    /// Update the convergence score
    fn update_score(&mut self) {
        if self.positions.len() < 2 {
            self.current_score = Score::ONE;
            return;
        }

        let values: Vec<f64> = self.positions.values().map(|p| p.value).collect();
        let var = variance(&values);

        // Conv = 1 - (Var / Var_max)
        let score = 1.0 - (var / self.max_variance).min(1.0);
        self.current_score = Score::new(score);

        self.score_history.push((Timestamp::now(), self.current_score));
    }

    /// Get current convergence score
    pub fn score(&self) -> Score {
        self.current_score
    }

    /// Get current state
    pub fn state(&self) -> ConvergenceState {
        ConvergenceState::from_score(self.current_score)
    }

    /// Get trend over recent history
    pub fn trend(&self, window: usize) -> ConvergenceTrend {
        if self.score_history.len() < window + 1 {
            return ConvergenceTrend::Stable;
        }

        let recent = &self.score_history[self.score_history.len() - window..];
        let early_avg: f64 = recent[..window / 2]
            .iter()
            .map(|(_, s)| s.value())
            .sum::<f64>()
            / (window / 2) as f64;
        let late_avg: f64 = recent[window / 2..]
            .iter()
            .map(|(_, s)| s.value())
            .sum::<f64>()
            / (window - window / 2) as f64;

        let diff = late_avg - early_avg;

        if diff > 0.05 {
            ConvergenceTrend::Improving
        } else if diff < -0.05 {
            ConvergenceTrend::Declining
        } else {
            ConvergenceTrend::Stable
        }
    }

    /// Advance to next round
    pub fn advance_round(&mut self) {
        self.round_count += 1;
    }

    /// Get current positions
    pub fn positions(&self) -> &HashMap<NodeId, Position> {
        &self.positions
    }

    /// Get the mean position value
    pub fn mean_position(&self) -> f64 {
        if self.positions.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.positions.values().map(|p| p.value).sum();
        sum / self.positions.len() as f64
    }
}

/// Result of agree-to-disagree protocol
#[derive(Debug, Clone)]
pub struct AgreeToDisagreeResult {
    /// The chosen path forward
    pub chosen_position: f64,
    /// Nodes that agree with the chosen path
    pub agreeing_nodes: Vec<NodeId>,
    /// Nodes that disagree (dissent recorded, not penalized)
    pub dissenting_nodes: Vec<NodeId>,
    /// Confidence in the decision
    pub confidence: Score,
}

/// Check if agree-to-disagree should be invoked
pub fn should_invoke_atd(tracker: &ConvergenceTracker, task_criticality: Score) -> bool {
    // Conditions for invoking ATD
    tracker.round_count >= 5
        && matches!(
            tracker.state(),
            ConvergenceState::Stuck | ConvergenceState::Polarized
        )
        && tracker.trend(5) == ConvergenceTrend::Stable
        && task_criticality.value() < 0.8
}

/// Execute agree-to-disagree protocol
pub fn execute_atd(
    tracker: &ConvergenceTracker,
    node_trusts: &HashMap<NodeId, Score>,
) -> AgreeToDisagreeResult {
    let positions = tracker.positions();

    if positions.len() < 2 {
        return AgreeToDisagreeResult {
            chosen_position: tracker.mean_position(),
            agreeing_nodes: positions.keys().cloned().collect(),
            dissenting_nodes: Vec::new(),
            confidence: Score::ONE,
        };
    }

    // Cluster positions (simple: above/below mean)
    let mean = tracker.mean_position();

    let mut above: Vec<(NodeId, f64)> = Vec::new();
    let mut below: Vec<(NodeId, f64)> = Vec::new();

    for (id, pos) in positions {
        if pos.value >= mean {
            above.push((*id, pos.value));
        } else {
            below.push((*id, pos.value));
        }
    }

    // Weight clusters by trust
    let above_weight: f64 = above
        .iter()
        .map(|(id, _)| node_trusts.get(id).map(|s| s.value()).unwrap_or(0.5))
        .sum();
    let below_weight: f64 = below
        .iter()
        .map(|(id, _)| node_trusts.get(id).map(|s| s.value()).unwrap_or(0.5))
        .sum();

    // Choose winning cluster
    let (winners, losers) = if above_weight >= below_weight {
        (above, below)
    } else {
        (below, above)
    };

    // Compute chosen position as weighted mean of winners
    let chosen = if winners.is_empty() {
        mean
    } else {
        let weighted_sum: f64 = winners
            .iter()
            .map(|(id, v)| v * node_trusts.get(id).map(|s| s.value()).unwrap_or(0.5))
            .sum();
        let weight_sum: f64 = winners
            .iter()
            .map(|(id, _)| node_trusts.get(id).map(|s| s.value()).unwrap_or(0.5))
            .sum();
        weighted_sum / weight_sum
    };

    let agreeing: Vec<NodeId> = winners.into_iter().map(|(id, _)| id).collect();
    let dissenting: Vec<NodeId> = losers.into_iter().map(|(id, _)| id).collect();

    // Confidence based on margin
    let total_weight = above_weight + below_weight;
    let margin = (above_weight - below_weight).abs() / total_weight;
    let confidence = Score::new(0.5 + margin * 0.5);

    AgreeToDisagreeResult {
        chosen_position: chosen,
        agreeing_nodes: agreeing,
        dissenting_nodes: dissenting,
        confidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_position(node_idx: u64, value: f64) -> Position {
        Position {
            node_id: NodeId::from_index(node_idx),
            value,
            confidence: Score::new(0.9),
            timestamp: Timestamp::now(),
        }
    }

    #[test]
    fn test_convergence_state() {
        assert_eq!(
            ConvergenceState::from_score(Score::new(0.9)),
            ConvergenceState::Converged
        );
        assert_eq!(
            ConvergenceState::from_score(Score::new(0.7)),
            ConvergenceState::Converging
        );
        assert_eq!(
            ConvergenceState::from_score(Score::new(0.1)),
            ConvergenceState::Polarized
        );
    }

    #[test]
    fn test_convergence_tracker() {
        let mut tracker = ConvergenceTracker::new(TaskId::random(), 100.0);

        // All same position = converged
        tracker.record_position(create_position(1, 50.0));
        tracker.record_position(create_position(2, 50.0));
        tracker.record_position(create_position(3, 50.0));

        assert!(tracker.score().value() > 0.95);
        assert_eq!(tracker.state(), ConvergenceState::Converged);
    }

    #[test]
    fn test_convergence_polarized() {
        let mut tracker = ConvergenceTracker::new(TaskId::random(), 100.0);

        // Very different positions
        tracker.record_position(create_position(1, 0.0));
        tracker.record_position(create_position(2, 100.0));

        // Variance = (50² + 50²) / 2 = 2500
        // Score = 1 - 2500/100 = very low (clamped)
        assert!(tracker.score().value() < 0.5);
    }

    #[test]
    fn test_agree_to_disagree() {
        let mut tracker = ConvergenceTracker::new(TaskId::random(), 100.0);

        // Two clusters
        tracker.record_position(create_position(1, 10.0));
        tracker.record_position(create_position(2, 12.0));
        tracker.record_position(create_position(3, 88.0));
        tracker.record_position(create_position(4, 90.0));

        let mut trusts = HashMap::new();
        trusts.insert(NodeId::from_index(1), Score::new(0.9));
        trusts.insert(NodeId::from_index(2), Score::new(0.9));
        trusts.insert(NodeId::from_index(3), Score::new(0.5));
        trusts.insert(NodeId::from_index(4), Score::new(0.5));

        let result = execute_atd(&tracker, &trusts);

        // Higher trust cluster (1, 2) should win
        assert!(result.agreeing_nodes.contains(&NodeId::from_index(1)));
        assert!(result.agreeing_nodes.contains(&NodeId::from_index(2)));
        assert!(result.dissenting_nodes.contains(&NodeId::from_index(3)));
        assert!(result.dissenting_nodes.contains(&NodeId::from_index(4)));

        // Chosen position should be near the low cluster
        assert!(result.chosen_position < 50.0);
    }

    #[test]
    fn test_should_invoke_atd() {
        let mut tracker = ConvergenceTracker::new(TaskId::random(), 100.0);

        // Not enough rounds
        assert!(!should_invoke_atd(&tracker, Score::new(0.5)));

        // Simulate stuck situation
        tracker.round_count = 10;
        tracker.record_position(create_position(1, 0.0));
        tracker.record_position(create_position(2, 100.0));

        // Add stable history
        for _ in 0..10 {
            tracker.score_history.push((Timestamp::now(), Score::new(0.2)));
        }

        // Should invoke for non-critical task
        assert!(should_invoke_atd(&tracker, Score::new(0.5)));

        // Should NOT invoke for critical task
        assert!(!should_invoke_atd(&tracker, Score::new(0.9)));
    }
}
