//! Metrics collection for simulation analysis.

use std::collections::HashMap;
use std::io::Write;
use symbiont_core::types::{NodeId, Score};

/// Collected metrics from simulation
#[derive(Debug, Clone, Default)]
pub struct MetricsCollector {
    /// Trust distribution over time
    pub trust_history: Vec<TrustSnapshot>,
    /// Interaction counts per node pair
    pub interaction_counts: HashMap<(NodeId, NodeId), u64>,
    /// Quality history per node
    pub quality_history: HashMap<NodeId, Vec<Score>>,
    /// Detection events
    pub detection_events: Vec<DetectionEvent>,
}

/// Snapshot of trust distribution at a tick
#[derive(Debug, Clone)]
pub struct TrustSnapshot {
    /// Tick number
    pub tick: u64,
    /// Mean trust
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum trust
    pub min: f64,
    /// Maximum trust
    pub max: f64,
    /// Number of high-trust nodes (> 0.7)
    pub high_trust_count: usize,
    /// Number of low-trust nodes (< 0.3)
    pub low_trust_count: usize,
}

/// A detection event (adversary detected)
#[derive(Debug, Clone)]
pub struct DetectionEvent {
    /// When detected
    pub tick: u64,
    /// Who was detected
    pub node_id: NodeId,
    /// What type of threat
    pub threat_type: String,
    /// Confidence level
    pub confidence: Score,
}

impl MetricsCollector {
    /// Create a new collector
    pub fn new() -> Self {
        Self::default()
    }

    /// Record trust distribution at a tick
    pub fn record_trust_distribution(&mut self, tick: u64, trusts: &[Score]) {
        if trusts.is_empty() {
            return;
        }

        let values: Vec<f64> = trusts.iter().map(|s| s.value()).collect();
        let n = values.len() as f64;

        let mean = values.iter().sum::<f64>() / n;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        let high_trust_count = values.iter().filter(|&&v| v > 0.7).count();
        let low_trust_count = values.iter().filter(|&&v| v < 0.3).count();

        self.trust_history.push(TrustSnapshot {
            tick,
            mean,
            std_dev,
            min,
            max,
            high_trust_count,
            low_trust_count,
        });
    }

    /// Record an interaction
    pub fn record_interaction(&mut self, from: NodeId, to: NodeId, quality: Score) {
        let key = if from < to { (from, to) } else { (to, from) };
        *self.interaction_counts.entry(key).or_insert(0) += 1;

        self.quality_history
            .entry(from)
            .or_default()
            .push(quality);
    }

    /// Record a detection event
    pub fn record_detection(&mut self, tick: u64, node_id: NodeId, threat_type: &str, confidence: Score) {
        self.detection_events.push(DetectionEvent {
            tick,
            node_id,
            threat_type: threat_type.to_string(),
            confidence,
        });
    }

    /// Get summary statistics
    pub fn summary(&self) -> MetricsSummary {
        let total_interactions: u64 = self.interaction_counts.values().sum();

        let final_trust = self.trust_history.last().map(|s| s.mean).unwrap_or(0.0);

        let trust_convergence = if self.trust_history.len() >= 2 {
            let early = &self.trust_history[..self.trust_history.len() / 2];
            let late = &self.trust_history[self.trust_history.len() / 2..];

            let early_std: f64 = early.iter().map(|s| s.std_dev).sum::<f64>() / early.len() as f64;
            let late_std: f64 = late.iter().map(|s| s.std_dev).sum::<f64>() / late.len() as f64;

            early_std - late_std // Positive = converging
        } else {
            0.0
        };

        MetricsSummary {
            total_ticks: self.trust_history.len() as u64,
            total_interactions,
            final_mean_trust: final_trust,
            trust_convergence,
            detection_count: self.detection_events.len(),
        }
    }

    /// Export trust history to CSV
    pub fn export_trust_csv<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writeln!(writer, "tick,mean,std_dev,min,max,high_trust,low_trust")?;

        for snapshot in &self.trust_history {
            writeln!(
                writer,
                "{},{:.4},{:.4},{:.4},{:.4},{},{}",
                snapshot.tick,
                snapshot.mean,
                snapshot.std_dev,
                snapshot.min,
                snapshot.max,
                snapshot.high_trust_count,
                snapshot.low_trust_count,
            )?;
        }

        Ok(())
    }

    /// Export detection events to CSV
    pub fn export_detections_csv<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writeln!(writer, "tick,node_id,threat_type,confidence")?;

        for event in &self.detection_events {
            writeln!(
                writer,
                "{},{},{},{:.4}",
                event.tick,
                event.node_id,
                event.threat_type,
                event.confidence.value(),
            )?;
        }

        Ok(())
    }
}

/// Summary of simulation metrics
#[derive(Debug, Clone)]
pub struct MetricsSummary {
    /// Total simulation ticks
    pub total_ticks: u64,
    /// Total interactions
    pub total_interactions: u64,
    /// Final mean trust
    pub final_mean_trust: f64,
    /// Trust convergence (positive = more converged)
    pub trust_convergence: f64,
    /// Number of adversaries detected
    pub detection_count: usize,
}

impl std::fmt::Display for MetricsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Simulation Summary:")?;
        writeln!(f, "  Ticks: {}", self.total_ticks)?;
        writeln!(f, "  Interactions: {}", self.total_interactions)?;
        writeln!(f, "  Final Mean Trust: {:.3}", self.final_mean_trust)?;
        writeln!(f, "  Trust Convergence: {:.3}", self.trust_convergence)?;
        writeln!(f, "  Detections: {}", self.detection_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let mut collector = MetricsCollector::new();

        let trusts = vec![Score::new(0.5), Score::new(0.6), Score::new(0.7)];
        collector.record_trust_distribution(1, &trusts);

        assert_eq!(collector.trust_history.len(), 1);
        assert!((collector.trust_history[0].mean - 0.6).abs() < 0.01);
    }

    #[test]
    fn test_interaction_recording() {
        let mut collector = MetricsCollector::new();

        let n1 = NodeId::from_index(1);
        let n2 = NodeId::from_index(2);

        collector.record_interaction(n1, n2, Score::new(0.8));
        collector.record_interaction(n1, n2, Score::new(0.9));

        assert_eq!(*collector.interaction_counts.get(&(n1, n2)).unwrap(), 2);
    }
}
