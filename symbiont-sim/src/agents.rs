//! Agent behavior models for simulation.

use crate::events::Event;
use rand::Rng;
use std::collections::HashMap;
use symbiont_core::node::Node;
use symbiont_core::types::{NodeId, Score, SignedScore};

/// Trait for agent behavior
pub trait Agent: Send + Sync {
    /// Called each tick to determine agent's actions
    fn act(&self, node: &Node, network: &HashMap<NodeId, Node>, tick: u64) -> Vec<Event>;

    /// Agent type name for logging
    fn agent_type(&self) -> &'static str;
}

/// An honest agent that follows the protocol faithfully
pub struct HonestAgent {
    /// Probability of initiating an interaction each tick
    interaction_rate: f64,
    /// Base quality of work produced
    base_quality: f64,
    /// Quality variance (for realism)
    quality_variance: f64,
}

impl HonestAgent {
    /// Create a new honest agent
    pub fn new(interaction_rate: f64) -> Self {
        Self {
            interaction_rate,
            base_quality: 0.8,
            quality_variance: 0.1,
        }
    }

    /// Create with custom quality settings
    pub fn with_quality(mut self, base: f64, variance: f64) -> Self {
        self.base_quality = base;
        self.quality_variance = variance;
        self
    }

    /// Select a partner using trust-weighted selection
    /// Prefers existing high-trust connections but may explore new nodes
    fn select_partner<R: Rng>(&self, node: &Node, network: &HashMap<NodeId, Node>, rng: &mut R) -> Option<NodeId> {
        // 80% chance to use existing connections, 20% to explore network
        if !node.connections.is_empty() && rng.gen::<f64>() < 0.8 {
            // Weight selection by connection strength
            let connections: Vec<_> = node.connections.iter().collect();
            let total_weight: f64 = connections.iter().map(|(_, c)| c.w.value()).sum();

            if total_weight > 0.0 {
                let mut pick = rng.gen::<f64>() * total_weight;
                for (id, conn) in &connections {
                    pick -= conn.w.value();
                    if pick <= 0.0 {
                        return Some(**id);
                    }
                }
            }
            // Fallback to random
            Some(*connections[rng.gen_range(0..connections.len())].0)
        } else {
            // Explore: pick a random node from the network we're not already connected to
            let candidates: Vec<_> = network.keys()
                .filter(|&id| *id != node.id && !node.connections.contains_key(id))
                .collect();

            if candidates.is_empty() {
                // Fall back to existing connections
                if node.connections.is_empty() {
                    None
                } else {
                    let partners: Vec<_> = node.connections.keys().collect();
                    Some(*partners[rng.gen_range(0..partners.len())])
                }
            } else {
                Some(*candidates[rng.gen_range(0..candidates.len())])
            }
        }
    }
}

impl Agent for HonestAgent {
    fn act(&self, node: &Node, network: &HashMap<NodeId, Node>, _tick: u64) -> Vec<Event> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        // Maybe initiate an interaction
        if rng.gen::<f64>() < self.interaction_rate {
            // Prefer partners with higher connection weight (trust-based selection)
            let partner = self.select_partner(node, network, &mut rng);

            if let Some(partner_id) = partner {
                // Generate quality with variance
                let quality = (self.base_quality + rng.gen_range(-self.quality_variance..self.quality_variance))
                    .clamp(0.0, 1.0);

                events.push(Event::Interaction {
                    from: node.id,
                    to: partner_id,
                    volume: 1.0,
                    quality: Score::new(quality),
                    tone: SignedScore::new(rng.gen_range(-0.2..0.3)),
                    capability: node.capabilities.keys().next().cloned(),
                });
            }
        }

        events
    }

    fn agent_type(&self) -> &'static str {
        "honest"
    }
}

/// A strategic adversary that builds trust then defects
pub struct StrategicAdversary {
    /// When to switch from good to bad behavior
    defection_tick: u64,
    /// Quality before defection
    good_quality: f64,
    /// Quality after defection
    bad_quality: f64,
    /// Interaction rate
    interaction_rate: f64,
}

impl StrategicAdversary {
    /// Create a new strategic adversary
    pub fn new(defection_tick: u64) -> Self {
        Self {
            defection_tick,
            good_quality: 0.95,
            bad_quality: 0.2,
            interaction_rate: 0.8,
        }
    }
}

impl Agent for StrategicAdversary {
    fn act(&self, node: &Node, _network: &HashMap<NodeId, Node>, tick: u64) -> Vec<Event> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        if rng.gen::<f64>() < self.interaction_rate && !node.connections.is_empty() {
            let partners: Vec<_> = node.connections.keys().collect();
            let partner = partners[rng.gen_range(0..partners.len())];

            // Switch behavior based on tick
            let quality = if tick < self.defection_tick {
                self.good_quality
            } else {
                self.bad_quality
            };

            events.push(Event::Interaction {
                from: node.id,
                to: *partner,
                volume: 1.0,
                quality: Score::new(quality),
                tone: SignedScore::ZERO,
                capability: node.capabilities.keys().next().cloned(),
            });
        }

        events
    }

    fn agent_type(&self) -> &'static str {
        "strategic_adversary"
    }
}

/// A free rider that takes but doesn't give quality work
pub struct FreeRider {
    /// Interaction rate
    interaction_rate: f64,
}

impl FreeRider {
    /// Create a new free rider
    pub fn new(interaction_rate: f64) -> Self {
        Self { interaction_rate }
    }
}

impl Agent for FreeRider {
    fn act(&self, node: &Node, _network: &HashMap<NodeId, Node>, _tick: u64) -> Vec<Event> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        if rng.gen::<f64>() < self.interaction_rate && !node.connections.is_empty() {
            let partners: Vec<_> = node.connections.keys().collect();
            let partner = partners[rng.gen_range(0..partners.len())];

            // Always low quality
            events.push(Event::Interaction {
                from: node.id,
                to: *partner,
                volume: 1.0,
                quality: Score::new(rng.gen_range(0.1..0.3)),
                tone: SignedScore::new(-0.3),
                capability: node.capabilities.keys().next().cloned(),
            });
        }

        events
    }

    fn agent_type(&self) -> &'static str {
        "free_rider"
    }
}

/// A Sybil cluster - coordinated fake identities
pub struct SybilCluster {
    /// IDs of other nodes in the cluster
    cluster_members: Vec<NodeId>,
    /// Interaction rate with cluster members
    internal_rate: f64,
    /// Interaction rate with outsiders
    external_rate: f64,
}

impl SybilCluster {
    /// Create a new Sybil node
    pub fn new(cluster_members: Vec<NodeId>) -> Self {
        Self {
            cluster_members,
            internal_rate: 0.9,
            external_rate: 0.1,
        }
    }
}

impl Agent for SybilCluster {
    fn act(&self, node: &Node, _network: &HashMap<NodeId, Node>, _tick: u64) -> Vec<Event> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        // High-quality interactions within cluster
        for member in &self.cluster_members {
            if *member != node.id && rng.gen::<f64>() < self.internal_rate {
                events.push(Event::Interaction {
                    from: node.id,
                    to: *member,
                    volume: 1.0,
                    quality: Score::new(0.99), // Suspiciously perfect
                    tone: SignedScore::new(0.9),
                    capability: node.capabilities.keys().next().cloned(),
                });
            }
        }

        // Low-quality interactions with outsiders
        let outsiders: Vec<_> = node
            .connections
            .keys()
            .filter(|id| !self.cluster_members.contains(id))
            .collect();

        for partner in outsiders {
            if rng.gen::<f64>() < self.external_rate {
                events.push(Event::Interaction {
                    from: node.id,
                    to: *partner,
                    volume: 1.0,
                    quality: Score::new(0.3),
                    tone: SignedScore::new(-0.2),
                    capability: node.capabilities.keys().next().cloned(),
                });
            }
        }

        events
    }

    fn agent_type(&self) -> &'static str {
        "sybil"
    }
}

/// A passive agent that rarely interacts
pub struct PassiveAgent {
    /// Very low interaction rate
    interaction_rate: f64,
}

impl PassiveAgent {
    /// Create a new passive agent
    pub fn new() -> Self {
        Self {
            interaction_rate: 0.05,
        }
    }
}

impl Default for PassiveAgent {
    fn default() -> Self {
        Self::new()
    }
}

impl Agent for PassiveAgent {
    fn act(&self, node: &Node, _network: &HashMap<NodeId, Node>, _tick: u64) -> Vec<Event> {
        let mut rng = rand::thread_rng();
        let mut events = Vec::new();

        if rng.gen::<f64>() < self.interaction_rate && !node.connections.is_empty() {
            let partners: Vec<_> = node.connections.keys().collect();
            let partner = partners[rng.gen_range(0..partners.len())];

            events.push(Event::Interaction {
                from: node.id,
                to: *partner,
                volume: 0.5,
                quality: Score::new(0.6),
                tone: SignedScore::ZERO,
                capability: node.capabilities.keys().next().cloned(),
            });
        }

        events
    }

    fn agent_type(&self) -> &'static str {
        "passive"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_honest_agent() {
        let agent = HonestAgent::new(1.0); // Always interact
        assert_eq!(agent.agent_type(), "honest");
    }

    #[test]
    fn test_strategic_adversary() {
        let agent = StrategicAdversary::new(100);
        assert_eq!(agent.agent_type(), "strategic_adversary");
    }
}
