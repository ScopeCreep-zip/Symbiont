//! Trust emergence scenario - watch trust dynamics evolve in an honest network.

use crate::agents::HonestAgent;
use crate::events::EventScheduler;
use crate::network::Network;
use crate::scenarios::Scenario;

/// Scenario for observing trust emergence in an honest network
pub struct TrustEmergenceScenario {
    /// Interaction rate for agents
    pub interaction_rate: f64,
    /// Base quality for agents
    pub base_quality: f64,
    /// Quality variance
    pub quality_variance: f64,
}

impl Default for TrustEmergenceScenario {
    fn default() -> Self {
        Self {
            interaction_rate: 0.5,
            base_quality: 0.8,
            quality_variance: 0.1,
        }
    }
}

impl TrustEmergenceScenario {
    /// Create a new scenario
    pub fn new() -> Self {
        Self::default()
    }

    /// Set interaction rate
    pub fn with_interaction_rate(mut self, rate: f64) -> Self {
        self.interaction_rate = rate;
        self
    }

    /// Set quality parameters
    pub fn with_quality(mut self, base: f64, variance: f64) -> Self {
        self.base_quality = base;
        self.quality_variance = variance;
        self
    }
}

impl Scenario for TrustEmergenceScenario {
    fn setup(&self, network: &mut Network, _scheduler: &mut EventScheduler) {
        // Add honest agents to all nodes
        let node_ids: Vec<_> = network.nodes().keys().cloned().collect();

        for id in node_ids {
            let agent = HonestAgent::new(self.interaction_rate)
                .with_quality(self.base_quality, self.quality_variance);
            network.set_agent(id, Box::new(agent));
        }
    }

    fn name(&self) -> &'static str {
        "trust_emergence"
    }

    fn description(&self) -> &'static str {
        "Observe trust dynamics emergence in a network of honest agents"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::NetworkConfig;
    use crate::runner::{SimulationConfig, SimulationRunner};
    use symbiont_core::capability::common;

    #[test]
    fn test_trust_emergence_scenario() {
        let config = SimulationConfig::default()
            .with_ticks(100)
            .with_network(
                NetworkConfig::default()
                    .with_nodes(10)
                    .with_capability(common::analysis())
                    .with_connection_prob(0.4)
                    .with_seed(42),
            );

        let scenario = TrustEmergenceScenario::new()
            .with_interaction_rate(0.6)
            .with_quality(0.85, 0.05);

        let result = SimulationRunner::run_scenario(config, &scenario);

        assert!(result.completed);
        // In an honest network, trust should generally increase
        assert!(result.summary.final_mean_trust > 0.3);
    }
}
