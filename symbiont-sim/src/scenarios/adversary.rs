//! Adversary injection scenario - test detection mechanisms.

use crate::agents::HonestAgent;
use crate::events::{AgentType, Event, EventScheduler};
use crate::network::Network;
use crate::scenarios::Scenario;
use symbiont_core::capability::common;
use symbiont_core::node::Node;
use symbiont_core::types::NodeId;

/// Type of adversary to inject
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdversaryType {
    /// Strategic adversary (builds trust then defects)
    Strategic,
    /// Free rider (takes but doesn't give)
    FreeRider,
    /// Sybil cluster (coordinated fake identities)
    Sybil,
}

/// Scenario for testing adversary detection
pub struct AdversaryScenario {
    /// Type of adversary
    pub adversary_type: AdversaryType,
    /// When to inject adversaries
    pub inject_at_tick: u64,
    /// Number of adversaries to inject
    pub adversary_count: usize,
    /// For strategic adversaries: when they defect
    pub defection_tick: u64,
    /// Interaction rate for honest nodes
    pub honest_interaction_rate: f64,
}

impl Default for AdversaryScenario {
    fn default() -> Self {
        Self {
            adversary_type: AdversaryType::Strategic,
            inject_at_tick: 0,
            adversary_count: 3,
            defection_tick: 100,
            honest_interaction_rate: 0.5,
        }
    }
}

impl AdversaryScenario {
    /// Create a new scenario
    pub fn new(adversary_type: AdversaryType) -> Self {
        Self {
            adversary_type,
            ..Default::default()
        }
    }

    /// Set injection timing
    pub fn inject_at(mut self, tick: u64) -> Self {
        self.inject_at_tick = tick;
        self
    }

    /// Set adversary count
    pub fn with_count(mut self, count: usize) -> Self {
        self.adversary_count = count;
        self
    }

    /// Set defection tick (for strategic adversaries)
    pub fn defect_at(mut self, tick: u64) -> Self {
        self.defection_tick = tick;
        self
    }
}

impl Scenario for AdversaryScenario {
    fn setup(&self, network: &mut Network, scheduler: &mut EventScheduler) {
        // Add honest agents to existing nodes
        let node_ids: Vec<_> = network.nodes().keys().cloned().collect();
        for id in node_ids {
            let agent = HonestAgent::new(self.honest_interaction_rate);
            network.set_agent(id, Box::new(agent));
        }

        // Schedule adversary injection
        let base_id = network.node_count() as u64 + 1000;

        match self.adversary_type {
            AdversaryType::Strategic => {
                for i in 0..self.adversary_count {
                    let id = NodeId::from_index(base_id + i as u64);
                    let mut node = Node::new(id);
                    node.add_capability(common::analysis());

                    // Schedule node join with strategic adversary agent
                    let agent_type = AgentType::Strategic {
                        defection_tick: self.defection_tick,
                    };
                    scheduler.schedule(
                        self.inject_at_tick,
                        Event::node_join_with_agent(node, agent_type),
                    );
                }
            }
            AdversaryType::FreeRider => {
                for i in 0..self.adversary_count {
                    let id = NodeId::from_index(base_id + i as u64);
                    let mut node = Node::new(id);
                    node.add_capability(common::analysis());

                    // Schedule node join with free rider agent
                    let agent_type = AgentType::FreeRider {
                        interaction_rate: 0.5,
                    };
                    scheduler.schedule(
                        self.inject_at_tick,
                        Event::node_join_with_agent(node, agent_type),
                    );
                }
            }
            AdversaryType::Sybil => {
                // Create a cluster of Sybil nodes
                let cluster_ids: Vec<NodeId> = (0..self.adversary_count)
                    .map(|i| NodeId::from_index(base_id + i as u64))
                    .collect();

                for id in &cluster_ids {
                    let mut node = Node::new(*id);
                    node.add_capability(common::analysis());

                    // Schedule node join with Sybil cluster agent
                    let agent_type = AgentType::Sybil {
                        cluster_members: cluster_ids.clone(),
                    };
                    scheduler.schedule(
                        self.inject_at_tick,
                        Event::node_join_with_agent(node, agent_type),
                    );
                }
            }
        }
    }

    fn name(&self) -> &'static str {
        match self.adversary_type {
            AdversaryType::Strategic => "adversary_strategic",
            AdversaryType::FreeRider => "adversary_free_rider",
            AdversaryType::Sybil => "adversary_sybil",
        }
    }

    fn description(&self) -> &'static str {
        match self.adversary_type {
            AdversaryType::Strategic => "Inject strategic adversaries that build trust then defect",
            AdversaryType::FreeRider => "Inject free riders that take but don't contribute",
            AdversaryType::Sybil => "Inject a coordinated Sybil cluster",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::NetworkConfig;
    use crate::runner::{SimulationConfig, SimulationRunner};

    #[test]
    fn test_strategic_adversary_scenario() {
        let config = SimulationConfig::default()
            .with_ticks(200)
            .with_network(
                NetworkConfig::default()
                    .with_nodes(10)
                    .with_capability(common::analysis())
                    .with_seed(42),
            );

        let scenario = AdversaryScenario::new(AdversaryType::Strategic)
            .inject_at(0)
            .with_count(2)
            .defect_at(100);

        let result = SimulationRunner::run_scenario(config, &scenario);

        assert!(result.completed);
    }

    #[test]
    fn test_free_rider_scenario() {
        let config = SimulationConfig::default()
            .with_ticks(100)
            .with_network(
                NetworkConfig::default()
                    .with_nodes(8)
                    .with_capability(common::analysis())
                    .with_seed(123),
            );

        let scenario = AdversaryScenario::new(AdversaryType::FreeRider)
            .inject_at(10)
            .with_count(2);

        let result = SimulationRunner::run_scenario(config, &scenario);

        assert!(result.completed);
    }
}
