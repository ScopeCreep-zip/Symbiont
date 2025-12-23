//! Simulated network of Symbiont nodes.

use crate::agents::{Agent, FreeRider, HonestAgent, PassiveAgent, StrategicAdversary, SybilCluster};
use crate::events::{AgentType, Event};
use rand::SeedableRng;
use symbiont_core::constants::ADVERSARY_INTERVAL;
use symbiont_core::defense::DefenseSignal;
use symbiont_core::detection::{detect_all_threats, detect_collusion};
use symbiont_core::types::Hash;
use crate::metrics::MetricsCollector;
use rand::Rng;
use std::collections::HashMap;
use symbiont_core::capability::Capability;
use symbiont_core::node::Node;
use symbiont_core::trust::compute_trust;
use symbiont_core::types::NodeId;

/// Configuration for network creation
#[derive(Debug, Clone)]
pub struct NetworkConfig {
    /// Number of initial nodes
    pub node_count: usize,
    /// Capabilities to assign to nodes
    pub capabilities: Vec<Capability>,
    /// Probability of connection between any two nodes
    pub connection_probability: f64,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            node_count: 10,
            capabilities: Vec::new(),
            connection_probability: 0.3,
            seed: None,
        }
    }
}

impl NetworkConfig {
    /// Create a new config with specified node count
    pub fn with_nodes(mut self, count: usize) -> Self {
        self.node_count = count;
        self
    }

    /// Add a capability
    pub fn with_capability(mut self, cap: Capability) -> Self {
        self.capabilities.push(cap);
        self
    }

    /// Set connection probability
    pub fn with_connection_prob(mut self, prob: f64) -> Self {
        self.connection_probability = prob;
        self
    }

    /// Set random seed
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

/// A simulated network of Symbiont nodes
pub struct Network {
    /// All nodes in the network
    nodes: HashMap<NodeId, Node>,
    /// Agent behaviors for each node
    agents: HashMap<NodeId, Box<dyn Agent>>,
    /// Current simulation tick
    pub tick: u64,
    /// Pending events
    event_queue: Vec<Event>,
    /// Metrics collector
    pub metrics: MetricsCollector,
}

impl Network {
    /// Create a new empty network
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            agents: HashMap::new(),
            tick: 0,
            event_queue: Vec::new(),
            metrics: MetricsCollector::new(),
        }
    }

    /// Create a network from configuration
    pub fn from_config(config: NetworkConfig) -> Self {
        let mut network = Self::new();
        let mut rng = match config.seed {
            Some(seed) => rand::rngs::StdRng::seed_from_u64(seed),
            None => rand::rngs::StdRng::from_entropy(),
        };

        // Create nodes
        for i in 0..config.node_count {
            let id = NodeId::from_index(i as u64);
            let mut node = Node::new(id);

            // Assign capabilities
            for cap in &config.capabilities {
                node.add_capability(cap.clone());
            }

            network.add_node(node);
        }

        // Create random connections
        let node_ids: Vec<_> = network.nodes.keys().cloned().collect();
        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                if rng.gen::<f64>() < config.connection_probability {
                    network.connect(node_ids[i], node_ids[j]);
                }
            }
        }

        network
    }

    /// Add a node to the network
    pub fn add_node(&mut self, node: Node) {
        let id = node.id;
        self.nodes.insert(id, node);
    }

    /// Add a node with a specific agent behavior
    pub fn add_node_with_agent(&mut self, node: Node, agent: Box<dyn Agent>) {
        let id = node.id;
        self.nodes.insert(id, node);
        self.agents.insert(id, agent);
    }

    /// Get a node by ID
    pub fn get_node(&self, id: &NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Get a mutable node by ID
    pub fn get_node_mut(&mut self, id: &NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Get all nodes
    pub fn nodes(&self) -> &HashMap<NodeId, Node> {
        &self.nodes
    }

    /// Get mutable reference to all nodes
    pub fn nodes_mut(&mut self) -> &mut HashMap<NodeId, Node> {
        &mut self.nodes
    }

    /// Number of nodes in the network
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Create a connection between two nodes
    pub fn connect(&mut self, a: NodeId, b: NodeId) {
        if let Some(node_a) = self.nodes.get_mut(&a) {
            node_a.get_or_create_connection(b);
        }
        if let Some(node_b) = self.nodes.get_mut(&b) {
            node_b.get_or_create_connection(a);
        }
    }

    /// Set agent behavior for a node
    pub fn set_agent(&mut self, id: NodeId, agent: Box<dyn Agent>) {
        self.agents.insert(id, agent);
    }

    /// Create an agent from an AgentType specification
    fn create_agent_from_type(&self, agent_type: AgentType) -> Box<dyn Agent> {
        match agent_type {
            AgentType::Honest { interaction_rate, base_quality } => {
                Box::new(HonestAgent::new(interaction_rate).with_quality(base_quality, 0.1))
            }
            AgentType::Strategic { defection_tick } => {
                Box::new(StrategicAdversary::new(defection_tick))
            }
            AgentType::FreeRider { interaction_rate } => {
                Box::new(FreeRider::new(interaction_rate))
            }
            AgentType::Sybil { cluster_members } => {
                Box::new(SybilCluster::new(cluster_members))
            }
            AgentType::Passive => {
                Box::new(PassiveAgent::new())
            }
        }
    }

    /// Queue an event for processing
    pub fn queue_event(&mut self, event: Event) {
        self.event_queue.push(event);
    }

    /// Process all queued events
    pub fn process_events(&mut self) {
        let events = std::mem::take(&mut self.event_queue);
        for event in events {
            self.handle_event(event);
        }
    }

    /// Handle a single event
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Interaction {
                from,
                to,
                volume,
                quality,
                tone,
                capability,
            } => {
                // Update both nodes
                if let Some(from_node) = self.nodes.get_mut(&from) {
                    from_node.handle_outgoing_interaction(
                        to, volume, 1.0, 1.0, quality, tone, capability,
                    );
                }
                if let Some(to_node) = self.nodes.get_mut(&to) {
                    to_node.handle_incoming_interaction(
                        from, volume, 1.0, 1.0, quality, tone, capability,
                    );
                }

                self.metrics.record_interaction(from, to, quality);
            }
            Event::DefenseSignal { signal } => {
                // Propagate signal to target's connections
                if let Some(target) = self.nodes.get_mut(&signal.threat) {
                    target.update_threat_belief(
                        signal.threat,
                        signal.threat_type,
                        signal.confidence.value(),
                        Some(signal.evidence),
                    );
                }
            }
            Event::NodeJoin { node, agent_type } => {
                let node_id = node.id;
                self.add_node(node);

                // Create and attach agent if specified
                if let Some(at) = agent_type {
                    let agent: Box<dyn Agent> = self.create_agent_from_type(at);
                    self.agents.insert(node_id, agent);

                    // Connect to existing nodes
                    let existing_ids: Vec<_> = self.nodes.keys()
                        .filter(|&&id| id != node_id)
                        .cloned()
                        .collect();
                    for other_id in existing_ids {
                        self.connect(node_id, other_id);
                    }
                }
            }
            Event::NodeLeave { node_id } => {
                self.nodes.remove(&node_id);
                self.agents.remove(&node_id);
            }
        }
    }

    /// Advance simulation by one tick
    pub fn tick(&mut self) {
        self.tick += 1;

        // Have each agent act
        let node_ids: Vec<_> = self.agents.keys().cloned().collect();
        for id in node_ids {
            if let Some(agent) = self.agents.get(&id) {
                // Get node state
                if let Some(node) = self.nodes.get(&id) {
                    // Generate events based on agent behavior
                    let events = agent.act(node, &self.nodes, self.tick);
                    for event in events {
                        self.event_queue.push(event);
                    }
                }
            }
        }

        // Process all events
        self.process_events();

        // Apply periodic maintenance
        for node in self.nodes.values_mut() {
            node.decay_priming();
            node.decay_idle_connections();
            node.check_diversity();
        }

        // JOB 6: Scan for adversaries (periodic)
        if self.tick % ADVERSARY_INTERVAL == 0 {
            self.scan_for_adversaries();
        }

        // Update trust scores
        self.update_trust_scores();

        // Collect metrics
        self.collect_metrics();
    }

    /// Update trust scores for all nodes
    fn update_trust_scores(&mut self) {
        for node in self.nodes.values_mut() {
            node.trust = compute_trust(node);
        }
    }

    /// Collect metrics at this tick
    fn collect_metrics(&mut self) {
        let trust_scores: Vec<_> = self.nodes.values().map(|n| n.trust).collect();
        self.metrics.record_trust_distribution(self.tick, &trust_scores);
    }

    /// Scan for adversaries across all nodes (JOB 6)
    fn scan_for_adversaries(&mut self) {
        // Collect detection results without mutating nodes yet
        let mut signals_to_emit: Vec<DefenseSignal> = Vec::new();

        // Check each node for individual threats
        for node in self.nodes.values() {
            let threats = detect_all_threats(node, &node.history);
            for detection in threats {
                if detection.is_threat() {
                    if let Some(threat_type) = detection.threat_type {
                        // Hash the reason as evidence
                        let evidence = Hash::compute(detection.reason.as_bytes());
                        let signal = DefenseSignal::new(
                            node.id, // emitter (self-detection)
                            detection.node_id,
                            threat_type,
                            detection.confidence,
                            evidence,
                        );
                        signals_to_emit.push(signal);
                    }
                }
            }
        }

        // Check for collusion clusters
        let collusion_clusters = detect_collusion(&self.nodes);
        for cluster in collusion_clusters {
            if cluster.is_suspicious() {
                // Emit signals for each member of the cluster
                for member_id in &cluster.members {
                    let evidence_str = format!(
                        "Collusion cluster detected: density={:.2}, mutual_rating={:.2}",
                        cluster.internal_density, cluster.mutual_rating
                    );
                    let evidence = Hash::compute(evidence_str.as_bytes());
                    let signal = DefenseSignal::new(
                        *member_id, // emitter (could be any node that detected it)
                        *member_id,
                        symbiont_core::node::ThreatType::Sybil,
                        cluster.confidence,
                        evidence,
                    );
                    signals_to_emit.push(signal);
                }
            }
        }

        // Process all emitted signals
        for signal in signals_to_emit {
            self.queue_event(Event::DefenseSignal { signal });
        }
    }

    /// Get network statistics
    pub fn stats(&self) -> NetworkStats {
        let trust_values: Vec<f64> = self.nodes.values().map(|n| n.trust.value()).collect();
        let connection_counts: Vec<usize> =
            self.nodes.values().map(|n| n.connections.len()).collect();

        let mean_trust = if trust_values.is_empty() {
            0.0
        } else {
            trust_values.iter().sum::<f64>() / trust_values.len() as f64
        };

        let mean_connections = if connection_counts.is_empty() {
            0.0
        } else {
            connection_counts.iter().sum::<usize>() as f64 / connection_counts.len() as f64
        };

        let total_connections: usize = connection_counts.iter().sum::<usize>() / 2;

        NetworkStats {
            node_count: self.nodes.len(),
            connection_count: total_connections,
            mean_trust,
            mean_connections,
            tick: self.tick,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about the network
#[derive(Debug, Clone)]
pub struct NetworkStats {
    /// Number of nodes
    pub node_count: usize,
    /// Number of connections
    pub connection_count: usize,
    /// Mean trust score
    pub mean_trust: f64,
    /// Mean connections per node
    pub mean_connections: f64,
    /// Current tick
    pub tick: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::HonestAgent;
    use symbiont_core::capability::common;

    #[test]
    fn test_network_creation() {
        let config = NetworkConfig::default()
            .with_nodes(5)
            .with_capability(common::analysis())
            .with_connection_prob(0.5)
            .with_seed(42);

        let network = Network::from_config(config);

        assert_eq!(network.node_count(), 5);
        assert!(network.stats().connection_count > 0);
    }

    #[test]
    fn test_network_tick() {
        let mut network = Network::from_config(
            NetworkConfig::default()
                .with_nodes(3)
                .with_seed(42),
        );

        // Add honest agents
        let node_ids: Vec<_> = network.nodes.keys().cloned().collect();
        for id in node_ids {
            network.set_agent(id, Box::new(HonestAgent::new(0.5)));
        }

        let initial_tick = network.tick;
        network.tick();

        assert_eq!(network.tick, initial_tick + 1);
    }
}
