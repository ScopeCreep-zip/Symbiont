//! Workflow routing scenario - test multi-step task routing.

use crate::agents::HonestAgent;
use crate::events::EventScheduler;
use crate::network::Network;
use crate::scenarios::Scenario;
use symbiont_core::capability::{common, Capability, CapabilityCategory};

/// Type of workflow to test
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowType {
    /// Simple chain: A → B → C
    Chain,
    /// Fan-out then fan-in
    FanOutFanIn,
    /// Complex DAG
    Dag,
}

/// Scenario for testing workflow routing
pub struct WorkflowScenario {
    /// Type of workflow
    pub workflow_type: WorkflowType,
    /// Number of steps in chain workflows
    pub chain_length: usize,
    /// Parallelism for fan-out workflows
    pub parallelism: usize,
    /// Interaction rate for agents
    pub interaction_rate: f64,
}

impl Default for WorkflowScenario {
    fn default() -> Self {
        Self {
            workflow_type: WorkflowType::Chain,
            chain_length: 3,
            parallelism: 3,
            interaction_rate: 0.6,
        }
    }
}

impl WorkflowScenario {
    /// Create a new scenario
    pub fn new(workflow_type: WorkflowType) -> Self {
        Self {
            workflow_type,
            ..Default::default()
        }
    }

    /// Set chain length
    pub fn with_chain_length(mut self, length: usize) -> Self {
        self.chain_length = length;
        self
    }

    /// Set parallelism
    pub fn with_parallelism(mut self, parallelism: usize) -> Self {
        self.parallelism = parallelism;
        self
    }

    /// Create specialized capabilities for workflow testing
    fn create_capabilities(&self) -> Vec<Capability> {
        match self.workflow_type {
            WorkflowType::Chain => {
                vec![
                    Capability::from_name("intake", CapabilityCategory::Analysis),
                    Capability::from_name("process", CapabilityCategory::Transformation),
                    Capability::from_name("output", CapabilityCategory::Generation),
                ]
            }
            WorkflowType::FanOutFanIn => {
                vec![
                    Capability::from_name("split", CapabilityCategory::Analysis),
                    Capability::from_name("worker", CapabilityCategory::Transformation),
                    Capability::from_name("merge", CapabilityCategory::Generation),
                ]
            }
            WorkflowType::Dag => {
                vec![
                    common::analysis(),
                    common::transformation(),
                    common::generation(),
                    common::validation(),
                ]
            }
        }
    }
}

impl Scenario for WorkflowScenario {
    fn setup(&self, network: &mut Network, _scheduler: &mut EventScheduler) {
        let capabilities = self.create_capabilities();

        // Assign capabilities to nodes (round-robin for variety)
        let node_ids: Vec<_> = network.nodes().keys().cloned().collect();

        for (i, id) in node_ids.iter().enumerate() {
            // Give each node a subset of capabilities
            let cap_index = i % capabilities.len();
            let cap = capabilities[cap_index].clone();

            if let Some(node) = network.nodes_mut().get_mut(id) {
                node.add_capability(cap);
            }

            // Add honest agent
            let agent = HonestAgent::new(self.interaction_rate)
                .with_quality(0.8, 0.1);
            network.set_agent(*id, Box::new(agent));
        }
    }

    fn name(&self) -> &'static str {
        match self.workflow_type {
            WorkflowType::Chain => "workflow_chain",
            WorkflowType::FanOutFanIn => "workflow_fan_out_fan_in",
            WorkflowType::Dag => "workflow_dag",
        }
    }

    fn description(&self) -> &'static str {
        match self.workflow_type {
            WorkflowType::Chain => "Test sequential chain workflow routing",
            WorkflowType::FanOutFanIn => "Test parallel fan-out/fan-in workflow routing",
            WorkflowType::Dag => "Test complex DAG workflow routing",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::NetworkConfig;
    use crate::runner::{SimulationConfig, SimulationRunner};

    #[test]
    fn test_chain_workflow_scenario() {
        let config = SimulationConfig::default()
            .with_ticks(100)
            .with_network(
                NetworkConfig::default()
                    .with_nodes(12)
                    .with_connection_prob(0.4)
                    .with_seed(42),
            );

        let scenario = WorkflowScenario::new(WorkflowType::Chain)
            .with_chain_length(3);

        let result = SimulationRunner::run_scenario(config, &scenario);

        assert!(result.completed);
    }

    #[test]
    fn test_fan_out_workflow_scenario() {
        let config = SimulationConfig::default()
            .with_ticks(100)
            .with_network(
                NetworkConfig::default()
                    .with_nodes(15)
                    .with_connection_prob(0.3)
                    .with_seed(123),
            );

        let scenario = WorkflowScenario::new(WorkflowType::FanOutFanIn)
            .with_parallelism(4);

        let result = SimulationRunner::run_scenario(config, &scenario);

        assert!(result.completed);
    }
}
