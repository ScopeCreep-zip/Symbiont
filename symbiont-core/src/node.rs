//! Node structure representing an agent in the Symbiont network.

use crate::capability::{Capability, CapabilityState};
use crate::connection::{Connection, ConnectionStats};
use crate::constants::{
    DIVERSITY_THRESHOLD, IDLE_THRESHOLD, LAMBDA, PRIMING_DECAY, PROBATION_COUNT,
    PROBATION_THRESHOLD, SWIFT_TRUST_BASE,
};
use crate::interaction::{Interaction, InteractionHistory};
use crate::types::{CapabilityId, Hash, NodeId, Score, SignedScore, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Node status in the network
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeStatus {
    /// New node, still proving itself
    Probationary,
    /// Passed probation, regular member
    Member,
    /// Long-standing trusted member
    Established,
    /// Highly connected hub node
    Hub,
    /// Expelled from network
    Expelled,
}

impl NodeStatus {
    /// Check if node can participate in network
    pub fn is_active(&self) -> bool {
        !matches!(self, NodeStatus::Expelled)
    }
}

/// Threat belief about another node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatBelief {
    /// Belief level (0 = trusted, 1 = definitely adversary)
    pub level: Score,
    /// Type of threat suspected
    pub threat_type: ThreatType,
    /// Evidence hashes
    pub evidence: Vec<Hash>,
    /// Last updated
    pub updated: Timestamp,
}

impl ThreatBelief {
    /// Create a new threat belief
    pub fn new(threat_type: ThreatType, initial_level: Score) -> Self {
        Self {
            level: initial_level,
            threat_type,
            evidence: Vec::new(),
            updated: Timestamp::now(),
        }
    }

    /// Update belief with new evidence (Bayesian update)
    pub fn update(&mut self, weight: f64, evidence: Option<Hash>) {
        // belief_new = belief_old + weight × (1 - belief_old)
        let new_level = self.level.value() + weight * (1.0 - self.level.value());
        self.level = Score::new(new_level);
        self.updated = Timestamp::now();

        if let Some(hash) = evidence {
            self.evidence.push(hash);
        }
    }
}

/// Type of threat
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatType {
    /// Cheating on quality or reciprocity
    Cheating,
    /// Sybil attack (fake identities)
    Sybil,
    /// Collusion ring
    Collusion,
    /// Quality fraud (fake reviews)
    QualityFraud,
    /// Strategic adversary (builds trust then defects)
    Strategic,
}

/// Defense state of a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DefenseState {
    /// Normal operation
    Normal,
    /// Heightened alertness due to signals
    Primed,
    /// Actively defending against threat
    Defending,
}

/// Flags that can be set on a node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NodeFlag {
    /// Low interaction diversity
    LowDiversity,
    /// Suspiciously high quality variance
    QualityAnomaly,
    /// Rapid trust changes
    TrustVolatility,
}

/// A node in the Symbiont network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Unique identifier
    pub id: NodeId,
    /// Current status
    pub status: NodeStatus,
    /// Global trust score
    pub trust: Score,
    /// Trust cap (can be reduced for various reasons)
    pub trust_cap: Score,
    /// Self-confidence score
    pub confidence: Score,
    /// Priming level (alertness)
    pub priming: Score,
    /// Connections to other nodes
    pub connections: HashMap<NodeId, Connection>,
    /// Available capabilities
    pub capabilities: HashMap<CapabilityId, CapabilityState>,
    /// Threat beliefs about other nodes
    pub threat_beliefs: HashMap<NodeId, ThreatBelief>,
    /// Current quality score (aggregate)
    pub quality_score: Score,
    /// Active flags
    pub flags: HashSet<NodeFlag>,
    /// Defense state
    pub defense_state: DefenseState,
    /// Interaction history
    pub history: InteractionHistory,
    /// Probation interaction count
    pub probation_count: u32,
    /// Creation timestamp
    pub created: Timestamp,
    /// Current load (0 = idle, 1 = maxed)
    pub load: Score,
}

impl Node {
    /// Create a new node with default values
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            status: NodeStatus::Probationary,
            trust: Score::new(SWIFT_TRUST_BASE),
            trust_cap: Score::ONE,
            confidence: Score::HALF,
            priming: Score::ZERO,
            connections: HashMap::new(),
            capabilities: HashMap::new(),
            threat_beliefs: HashMap::new(),
            quality_score: Score::HALF,
            flags: HashSet::new(),
            defense_state: DefenseState::Normal,
            history: InteractionHistory::new(),
            probation_count: 0,
            created: Timestamp::now(),
            load: Score::ZERO,
        }
    }

    /// Create a node with initial capabilities
    pub fn with_capabilities(id: NodeId, capabilities: Vec<Capability>) -> Self {
        let mut node = Self::new(id);
        for cap in capabilities {
            node.add_capability(cap);
        }
        node
    }

    /// Add a capability to this node
    pub fn add_capability(&mut self, capability: Capability) {
        let state = CapabilityState::new(capability.clone());
        self.capabilities.insert(capability.id, state);
    }

    /// Check if node has a specific capability
    pub fn has_capability(&self, cap_id: CapabilityId) -> bool {
        self.capabilities
            .get(&cap_id)
            .map(|s| s.available)
            .unwrap_or(false)
    }

    /// Get quality for a specific capability
    pub fn capability_quality(&self, cap_id: CapabilityId) -> Score {
        self.capabilities
            .get(&cap_id)
            .map(|s| s.quality)
            .unwrap_or(Score::ZERO)
    }

    /// Get or create a connection to another node
    pub fn get_or_create_connection(&mut self, partner_id: NodeId) -> &mut Connection {
        self.connections
            .entry(partner_id)
            .or_insert_with(|| Connection::new(partner_id))
    }

    /// Get connection to a specific node
    pub fn get_connection(&self, partner_id: &NodeId) -> Option<&Connection> {
        self.connections.get(partner_id)
    }

    /// Get mutable connection
    pub fn get_connection_mut(&mut self, partner_id: &NodeId) -> Option<&mut Connection> {
        self.connections.get_mut(partner_id)
    }

    /// Get threat level for a node
    pub fn get_threat_level(&self, node_id: &NodeId) -> f64 {
        self.threat_beliefs
            .get(node_id)
            .map(|b| b.level.value())
            .unwrap_or(0.0)
    }

    /// Handle an outgoing interaction (we initiated)
    #[allow(clippy::too_many_arguments)]
    pub fn handle_outgoing_interaction(
        &mut self,
        partner_id: NodeId,
        volume: f64,
        exchange_in: f64,
        exchange_out: f64,
        quality: Score,
        tone: SignedScore,
        capability: Option<CapabilityId>,
    ) {
        let threat_level = self.get_threat_level(&partner_id);
        let conn = self.get_or_create_connection(partner_id);

        conn.process_interaction(
            volume,
            exchange_in,
            exchange_out,
            quality,
            tone,
            threat_level,
        );

        // Update per-capability quality if this interaction involved a specific capability
        if let Some(cap_id) = capability {
            let conn = self.get_or_create_connection(partner_id);
            conn.update_capability_quality(cap_id, quality);
        }

        // Record in history
        let interaction = Interaction::new(self.id, partner_id)
            .with_volume(volume)
            .with_outcome(quality, tone)
            .with_exchange(exchange_in, exchange_out);
        self.history.add(interaction);

        // Check for probation progress
        if self.status == NodeStatus::Probationary {
            self.probation_count += 1;
            self.check_probation_status();
        }
    }

    /// Handle an incoming interaction (someone else initiated with us)
    #[allow(clippy::too_many_arguments)]
    pub fn handle_incoming_interaction(
        &mut self,
        initiator_id: NodeId,
        volume: f64,
        exchange_in: f64,
        exchange_out: f64,
        quality: Score,
        tone: SignedScore,
        capability: Option<CapabilityId>,
    ) {
        let threat_level = self.get_threat_level(&initiator_id);
        let conn = self.get_or_create_connection(initiator_id);

        // For incoming, exchange direction is flipped
        conn.process_interaction(
            volume,
            exchange_out, // We received what they gave
            exchange_in,  // We gave what they received
            quality,
            tone,
            threat_level,
        );

        // Update our capability quality if we used it
        if let Some(cap_id) = capability {
            if let Some(cap_state) = self.capabilities.get_mut(&cap_id) {
                cap_state.record_usage(quality, LAMBDA);
            }
        }

        // Record in history (we're the responder)
        let interaction = Interaction::new(initiator_id, self.id)
            .with_volume(volume)
            .with_outcome(quality, tone)
            .with_exchange(exchange_out, exchange_in);
        self.history.add(interaction);
    }

    /// Check and update probation status
    fn check_probation_status(&mut self) {
        if self.status != NodeStatus::Probationary {
            return;
        }

        if self.probation_count >= PROBATION_COUNT {
            let mean_quality = self.history.mean_quality(PROBATION_COUNT as usize);

            if mean_quality.value() >= PROBATION_THRESHOLD {
                // Passed probation
                self.status = NodeStatus::Member;
                self.trust = Score::new((self.trust.value() * 1.5).min(0.8));
            } else {
                // Failed probation - extend or expel
                self.trust = Score::new(self.trust.value() * 0.8);
                self.probation_count = 0; // Reset for another try
                // TODO: track failure count for expulsion
            }
        }
    }

    /// Update threat belief about a node
    pub fn update_threat_belief(
        &mut self,
        target: NodeId,
        threat_type: ThreatType,
        weight: f64,
        evidence: Option<Hash>,
    ) {
        let belief = self
            .threat_beliefs
            .entry(target)
            .or_insert_with(|| ThreatBelief::new(threat_type, Score::ZERO));

        belief.update(weight, evidence);
    }

    /// Compute current diversity score
    pub fn diversity_score(&self) -> Score {
        let unique = self.history.unique_partners(100);
        Score::new(unique as f64 / 100.0)
    }

    /// Apply periodic decay to priming
    pub fn decay_priming(&mut self) {
        self.priming = Score::new(self.priming.value() * PRIMING_DECAY);

        // Update defense state based on priming
        if self.priming.value() < 0.1 {
            self.defense_state = DefenseState::Normal;
        }
    }

    /// Increase priming due to threat signal
    pub fn increase_priming(&mut self, boost: f64) {
        self.priming = Score::new((self.priming.value() + boost).min(1.0));

        if self.priming.value() > 0.3 {
            self.defense_state = DefenseState::Primed;
        }
    }

    /// Apply decay to idle connections
    pub fn decay_idle_connections(&mut self) {
        let mut to_remove = Vec::new();

        for (id, conn) in self.connections.iter_mut() {
            if conn.is_idle(IDLE_THRESHOLD) {
                conn.apply_decay(1.0);
                if conn.w.value() <= crate::constants::W_MIN {
                    to_remove.push(*id);
                }
            }
        }

        for id in to_remove {
            self.connections.remove(&id);
        }
    }

    /// Check and update diversity flag
    pub fn check_diversity(&mut self) {
        let diversity = self.diversity_score();

        if diversity.value() < DIVERSITY_THRESHOLD {
            self.flags.insert(NodeFlag::LowDiversity);
            self.trust_cap = Score::new(0.7);
        } else {
            self.flags.remove(&NodeFlag::LowDiversity);
            self.trust_cap = Score::ONE;
        }
    }

    /// Get connection statistics
    pub fn connection_stats(&self) -> ConnectionStats {
        ConnectionStats::from_connections(self.connections.values())
    }

    /// Get aggregate capability quality (weighted by volume)
    pub fn aggregate_capability_quality(&self) -> Score {
        if self.capabilities.is_empty() {
            return Score::HALF;
        }

        let (sum, weight_sum): (f64, f64) = self
            .capabilities
            .values()
            .map(|cap| (cap.quality.value() * cap.volume as f64, cap.volume as f64))
            .fold((0.0, 0.0), |(s, w), (v, weight)| (s + v, w + weight));

        if weight_sum < 0.001 {
            return Score::HALF;
        }

        Score::new(sum / weight_sum)
    }

    /// Check if this node can accept work
    pub fn can_accept_work(&self) -> bool {
        self.status.is_active() && self.load.value() < 0.95
    }

    /// Check if this node can accept work for a specific capability
    pub fn can_accept_capability_work(&self, cap_id: CapabilityId) -> bool {
        self.can_accept_work()
            && self
                .capabilities
                .get(&cap_id)
                .map(|c| c.can_accept_work())
                .unwrap_or(false)
    }
}

/// Builder for creating test nodes
#[derive(Debug, Clone)]
pub struct NodeBuilder {
    id: NodeId,
    status: NodeStatus,
    trust: Score,
    capabilities: Vec<Capability>,
    connections: Vec<(NodeId, Connection)>,
}

impl NodeBuilder {
    /// Start building a node with a given ID
    pub fn new(id: NodeId) -> Self {
        Self {
            id,
            status: NodeStatus::Probationary,
            trust: Score::new(SWIFT_TRUST_BASE),
            capabilities: Vec::new(),
            connections: Vec::new(),
        }
    }

    /// Set status
    pub fn status(mut self, status: NodeStatus) -> Self {
        self.status = status;
        self
    }

    /// Set trust
    pub fn trust(mut self, trust: Score) -> Self {
        self.trust = trust;
        self
    }

    /// Add a capability
    pub fn capability(mut self, cap: Capability) -> Self {
        self.capabilities.push(cap);
        self
    }

    /// Add a connection
    pub fn connection(mut self, partner_id: NodeId, conn: Connection) -> Self {
        self.connections.push((partner_id, conn));
        self
    }

    /// Build the node
    pub fn build(self) -> Node {
        let mut node = Node::with_capabilities(self.id, self.capabilities);
        node.status = self.status;
        node.trust = self.trust;

        for (id, conn) in self.connections {
            node.connections.insert(id, conn);
        }

        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::common;

    #[test]
    fn test_node_creation() {
        let id = NodeId::from_index(1);
        let node = Node::new(id);

        assert_eq!(node.status, NodeStatus::Probationary);
        assert_eq!(node.trust.value(), SWIFT_TRUST_BASE);
        assert!(node.connections.is_empty());
    }

    #[test]
    fn test_node_with_capabilities() {
        let id = NodeId::from_index(1);
        let node = Node::with_capabilities(
            id,
            vec![common::analysis(), common::generation()],
        );

        assert_eq!(node.capabilities.len(), 2);
        assert!(node.has_capability(common::analysis().id));
        assert!(node.has_capability(common::generation().id));
        assert!(!node.has_capability(CapabilityId::new(999)));
    }

    #[test]
    fn test_connection_management() {
        let id = NodeId::from_index(1);
        let partner = NodeId::from_index(2);
        let mut node = Node::new(id);

        // Get or create should create
        let conn = node.get_or_create_connection(partner);
        assert_eq!(conn.partner_id, partner);

        // Second call should return existing
        let conn2 = node.get_or_create_connection(partner);
        assert_eq!(conn2.count, 0); // Same connection

        assert_eq!(node.connections.len(), 1);
    }

    #[test]
    fn test_threat_belief_update() {
        let id = NodeId::from_index(1);
        let adversary = NodeId::from_index(99);
        let mut node = Node::new(id);

        // Initially no threat
        assert_eq!(node.get_threat_level(&adversary), 0.0);

        // Update belief
        node.update_threat_belief(adversary, ThreatType::Strategic, 0.5, None);
        assert!(node.get_threat_level(&adversary) > 0.0);

        // Multiple updates increase belief
        for _ in 0..5 {
            node.update_threat_belief(adversary, ThreatType::Strategic, 0.3, None);
        }
        assert!(node.get_threat_level(&adversary) > 0.8);
    }

    #[test]
    fn test_probation_status() {
        let id = NodeId::from_index(1);
        let partner = NodeId::from_index(2);
        let mut node = Node::new(id);

        // Simulate good interactions
        for _ in 0..PROBATION_COUNT {
            node.handle_outgoing_interaction(
                partner,
                1.0,
                1.0,
                1.0,
                Score::new(0.8),
                SignedScore::ZERO,
                None,
            );
        }

        // Should have passed probation
        assert_eq!(node.status, NodeStatus::Member);
        assert!(node.trust.value() > SWIFT_TRUST_BASE);
    }

    #[test]
    fn test_node_builder() {
        let id = NodeId::from_index(1);
        let partner = NodeId::from_index(2);

        let node = NodeBuilder::new(id)
            .status(NodeStatus::Established)
            .trust(Score::new(0.9))
            .capability(common::analysis())
            .connection(partner, Connection::new(partner))
            .build();

        assert_eq!(node.status, NodeStatus::Established);
        assert_eq!(node.trust.value(), 0.9);
        assert!(node.has_capability(common::analysis().id));
        assert!(node.connections.contains_key(&partner));
    }
}
