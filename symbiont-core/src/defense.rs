//! Defense signaling and threat response for Symbiont.
//!
//! Nodes can emit and receive defense signals that propagate through
//! trusted connections with decay.

use crate::constants::{
    ACTION_THRESHOLD, DECAY_PER_HOP, MAX_HOPS, MIN_SIGNAL, PRIMING_SENSITIVITY,
    PROPAGATE_THRESHOLD,
};
use crate::node::{DefenseState, Node, ThreatType};
use crate::types::{Hash, NodeId, Score, Signature, Timestamp};
use serde::{Deserialize, Serialize};

/// Type of defense signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SignalType {
    /// General alert (be vigilant)
    GeneralAlert,
    /// Specific threat identified
    SpecificThreat,
    /// Network-wide broadcast
    Broadcast,
}

/// A defense signal emitted by a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefenseSignal {
    /// Type of signal
    pub signal_type: SignalType,
    /// Node that sent this signal (may not be originator)
    pub sender: NodeId,
    /// Node that originally detected the threat
    pub origin: NodeId,
    /// The suspected threat (target node)
    pub threat: NodeId,
    /// Type of threat detected
    pub threat_type: ThreatType,
    /// Confidence in the threat assessment
    pub confidence: Score,
    /// Hash of evidence
    pub evidence: Hash,
    /// Number of hops from origin
    pub hops: u8,
    /// When the signal was created
    pub timestamp: Timestamp,
    /// Signature from sender
    pub signature: Signature,
}

impl DefenseSignal {
    /// Create a new defense signal
    pub fn new(
        origin: NodeId,
        threat: NodeId,
        threat_type: ThreatType,
        confidence: Score,
        evidence: Hash,
    ) -> Self {
        Self {
            signal_type: SignalType::SpecificThreat,
            sender: origin,
            origin,
            threat,
            threat_type,
            confidence,
            evidence,
            hops: 0,
            timestamp: Timestamp::now(),
            signature: Signature::new([0u8; 64]), // Placeholder
        }
    }

    /// Create a forwarded version of this signal
    pub fn forward(&self, new_sender: NodeId, connection_weight: f64) -> Option<Self> {
        // Check if we should propagate
        if self.confidence.value() < PROPAGATE_THRESHOLD {
            return None;
        }
        if self.hops >= MAX_HOPS {
            return None;
        }

        // Attenuate confidence
        let new_confidence = self.confidence.value() * DECAY_PER_HOP * connection_weight;
        if new_confidence < MIN_SIGNAL {
            return None;
        }

        Some(Self {
            signal_type: self.signal_type,
            sender: new_sender,
            origin: self.origin,
            threat: self.threat,
            threat_type: self.threat_type,
            confidence: Score::new(new_confidence),
            evidence: self.evidence,
            hops: self.hops + 1,
            timestamp: Timestamp::now(),
            signature: Signature::new([0u8; 64]),
        })
    }

    /// Check if signal is still valid (not too old)
    pub fn is_valid(&self, max_age_ms: u64) -> bool {
        !self.timestamp.is_older_than(max_age_ms)
    }
}

/// An affirmation of good behavior from one node to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Affirmation {
    /// Node giving the affirmation
    pub from: NodeId,
    /// Node receiving the affirmation
    pub to: NodeId,
    /// Type of affirmation
    pub affirmation_type: AffirmationType,
    /// Strength of affirmation
    pub strength: Score,
    /// Timestamp
    pub timestamp: Timestamp,
    /// Signature from sender
    pub signature: Signature,
}

/// Type of affirmation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AffirmationType {
    /// Affirming quality of work
    Quality,
    /// Affirming reliability
    Reliability,
    /// Affirming collaboration
    Collaboration,
    /// Affirming growth/improvement
    Growth,
}

impl Affirmation {
    /// Create a new affirmation
    pub fn new(from: NodeId, to: NodeId, affirmation_type: AffirmationType, strength: Score) -> Self {
        Self {
            from,
            to,
            affirmation_type,
            strength,
            timestamp: Timestamp::now(),
            signature: Signature::new([0u8; 64]),
        }
    }
}

/// Handler for defense signals
pub struct DefenseHandler {
    /// Signals to propagate
    pending_signals: Vec<DefenseSignal>,
    /// Affirmations to send
    pending_affirmations: Vec<Affirmation>,
}

impl DefenseHandler {
    /// Create a new defense handler
    pub fn new() -> Self {
        Self {
            pending_signals: Vec::new(),
            pending_affirmations: Vec::new(),
        }
    }

    /// Process an incoming defense signal for a node
    pub fn handle_signal(&mut self, node: &mut Node, signal: &DefenseSignal) -> SignalResult {
        // Don't process signals about ourselves
        if signal.threat == node.id {
            return SignalResult::Ignored;
        }

        // Don't process signals from ourselves
        if signal.sender == node.id {
            return SignalResult::Ignored;
        }

        // Get sender's trust level (if known)
        let sender_trust = node
            .get_connection(&signal.sender)
            .map(|c| c.w.value())
            .unwrap_or(0.3);

        // Update threat belief (Bayesian)
        let weight = sender_trust * signal.confidence.value();
        node.update_threat_belief(signal.threat, signal.threat_type, weight, Some(signal.evidence));

        // Increase priming
        let boost = signal.confidence.value() * PRIMING_SENSITIVITY;
        node.increase_priming(boost);

        // Check if we should take action
        let threat_level = node.get_threat_level(&signal.threat);
        if threat_level > ACTION_THRESHOLD {
            // Take defensive action
            self.take_defensive_action(node, signal.threat);
        }

        // Maybe propagate
        if signal.confidence.value() > PROPAGATE_THRESHOLD && signal.hops < MAX_HOPS {
            self.queue_propagation(node, signal);
        }

        SignalResult::Processed {
            new_threat_level: Score::new(threat_level),
            priming_boost: boost,
        }
    }

    /// Take defensive action against a threat
    fn take_defensive_action(&mut self, node: &mut Node, threat: NodeId) {
        node.defense_state = DefenseState::Defending;

        // Reduce connection weight to threat
        if let Some(conn) = node.get_connection_mut(&threat) {
            conn.w = crate::types::Weight::MIN;
        }

        // Could also: block interactions, notify other systems, etc.
    }

    /// Queue signal propagation to neighbors
    fn queue_propagation(&mut self, node: &Node, signal: &DefenseSignal) {
        for (partner_id, conn) in &node.connections {
            // Don't send back to sender or origin
            if *partner_id == signal.sender || *partner_id == signal.origin {
                continue;
            }

            // Don't send to the threat
            if *partner_id == signal.threat {
                continue;
            }

            // Forward with attenuation based on connection strength
            if let Some(forwarded) = signal.forward(node.id, conn.w.value()) {
                self.pending_signals.push(forwarded);
            }
        }
    }

    /// Send an affirmation
    pub fn send_affirmation(&mut self, affirmation: Affirmation) {
        self.pending_affirmations.push(affirmation);
    }

    /// Check if we should send an affirmation based on interaction quality
    pub fn maybe_affirm(&mut self, node: &Node, partner: NodeId, quality: Score, tone: Score) {
        if quality.value() > 0.8 && tone.value() > 0.5 {
            let strength = Score::new((quality.value() + tone.value()) / 2.0);
            let affirmation = Affirmation::new(
                node.id,
                partner,
                AffirmationType::Quality,
                strength,
            );
            self.send_affirmation(affirmation);
        }
    }

    /// Drain pending signals
    pub fn take_pending_signals(&mut self) -> Vec<DefenseSignal> {
        std::mem::take(&mut self.pending_signals)
    }

    /// Drain pending affirmations
    pub fn take_pending_affirmations(&mut self) -> Vec<Affirmation> {
        std::mem::take(&mut self.pending_affirmations)
    }
}

impl Default for DefenseHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of processing a defense signal
#[derive(Debug, Clone)]
pub enum SignalResult {
    /// Signal was ignored (irrelevant or invalid)
    Ignored,
    /// Signal was processed
    Processed {
        new_threat_level: Score,
        priming_boost: f64,
    },
}

/// Emit a defense signal about a detected threat
pub fn emit_defense_signal(
    from: &Node,
    threat: NodeId,
    threat_type: ThreatType,
    confidence: Score,
    evidence_data: &[u8],
) -> DefenseSignal {
    let evidence = Hash::compute(evidence_data);
    DefenseSignal::new(from.id, threat, threat_type, confidence, evidence)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_signal() -> DefenseSignal {
        DefenseSignal::new(
            NodeId::from_index(1),
            NodeId::from_index(99),
            ThreatType::Strategic,
            Score::new(0.8),
            Hash::compute(b"test evidence"),
        )
    }

    #[test]
    fn test_defense_signal_creation() {
        let signal = create_test_signal();

        assert_eq!(signal.hops, 0);
        assert_eq!(signal.origin, NodeId::from_index(1));
        assert_eq!(signal.threat, NodeId::from_index(99));
        assert!(signal.confidence.value() > 0.7);
    }

    #[test]
    fn test_signal_forwarding() {
        let signal = create_test_signal();
        let new_sender = NodeId::from_index(2);

        let forwarded = signal.forward(new_sender, 0.8).unwrap();

        assert_eq!(forwarded.hops, 1);
        assert_eq!(forwarded.sender, new_sender);
        assert_eq!(forwarded.origin, signal.origin); // Origin unchanged
        assert!(forwarded.confidence.value() < signal.confidence.value()); // Decayed
    }

    #[test]
    fn test_signal_forwarding_stops() {
        let mut signal = create_test_signal();
        signal.hops = MAX_HOPS;

        // Should not forward at max hops
        assert!(signal.forward(NodeId::from_index(2), 1.0).is_none());

        // Low confidence should also stop propagation
        signal.hops = 0;
        signal.confidence = Score::new(0.1);
        assert!(signal.forward(NodeId::from_index(2), 1.0).is_none());
    }

    #[test]
    fn test_defense_handler() {
        let mut node = Node::new(NodeId::from_index(1));

        // Add a connection to the sender
        let sender = NodeId::from_index(2);
        node.get_or_create_connection(sender);

        let mut handler = DefenseHandler::new();
        let signal = DefenseSignal::new(
            sender,
            NodeId::from_index(99),
            ThreatType::Strategic,
            Score::new(0.8),
            Hash::compute(b"evidence"),
        );

        let result = handler.handle_signal(&mut node, &signal);

        match result {
            SignalResult::Processed { new_threat_level, priming_boost } => {
                assert!(new_threat_level.value() > 0.0);
                assert!(priming_boost > 0.0);
            }
            SignalResult::Ignored => panic!("Signal should have been processed"),
        }

        // Node's priming should have increased
        assert!(node.priming.value() > 0.0);
    }

    #[test]
    fn test_affirmation() {
        let affirmation = Affirmation::new(
            NodeId::from_index(1),
            NodeId::from_index(2),
            AffirmationType::Quality,
            Score::new(0.9),
        );

        assert_eq!(affirmation.from, NodeId::from_index(1));
        assert_eq!(affirmation.to, NodeId::from_index(2));
        assert!(affirmation.strength.value() > 0.8);
    }
}
