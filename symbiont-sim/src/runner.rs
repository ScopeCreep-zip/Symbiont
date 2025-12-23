//! Simulation runner for executing scenarios.

use crate::events::EventScheduler;
use crate::metrics::MetricsSummary;
use crate::network::{Network, NetworkConfig};
use crate::scenarios::Scenario;
use std::time::Instant;

/// Configuration for simulation run
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Maximum ticks to run
    pub max_ticks: u64,
    /// Network configuration
    pub network_config: NetworkConfig,
    /// Whether to collect detailed metrics
    pub detailed_metrics: bool,
    /// Progress reporting interval (ticks)
    pub progress_interval: Option<u64>,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            max_ticks: 1000,
            network_config: NetworkConfig::default(),
            detailed_metrics: true,
            progress_interval: Some(100),
        }
    }
}

impl SimulationConfig {
    /// Create with specific tick count
    pub fn with_ticks(mut self, ticks: u64) -> Self {
        self.max_ticks = ticks;
        self
    }

    /// Set network config
    pub fn with_network(mut self, config: NetworkConfig) -> Self {
        self.network_config = config;
        self
    }

    /// Enable/disable detailed metrics
    pub fn with_detailed_metrics(mut self, enabled: bool) -> Self {
        self.detailed_metrics = enabled;
        self
    }
}

/// Result of a simulation run
#[derive(Debug)]
pub struct SimulationResult {
    /// Summary metrics
    pub summary: MetricsSummary,
    /// Wall-clock duration
    pub duration_ms: u64,
    /// Final tick reached
    pub final_tick: u64,
    /// Whether completed normally
    pub completed: bool,
    /// Any error message
    pub error: Option<String>,
}

/// Callback for progress updates
pub type ProgressCallback = Box<dyn Fn(u64, u64) + Send>;

/// Simulation runner
pub struct SimulationRunner {
    /// Configuration
    config: SimulationConfig,
    /// The network being simulated
    network: Network,
    /// Event scheduler
    scheduler: EventScheduler,
    /// Progress callback
    progress_callback: Option<ProgressCallback>,
}

impl SimulationRunner {
    /// Create a new runner with configuration
    pub fn new(config: SimulationConfig) -> Self {
        let network = Network::from_config(config.network_config.clone());

        Self {
            config,
            network,
            scheduler: EventScheduler::new(),
            progress_callback: None,
        }
    }

    /// Set progress callback
    pub fn on_progress(mut self, callback: ProgressCallback) -> Self {
        self.progress_callback = Some(callback);
        self
    }

    /// Get mutable access to network for setup
    pub fn network_mut(&mut self) -> &mut Network {
        &mut self.network
    }

    /// Get access to network
    pub fn network(&self) -> &Network {
        &self.network
    }

    /// Get access to scheduler for scheduling events
    pub fn scheduler_mut(&mut self) -> &mut EventScheduler {
        &mut self.scheduler
    }

    /// Apply a scenario to the simulation
    pub fn apply_scenario(&mut self, scenario: &dyn Scenario) {
        scenario.setup(&mut self.network, &mut self.scheduler);
    }

    /// Run the simulation
    pub fn run(&mut self) -> SimulationResult {
        let start = Instant::now();
        let error = None;

        while self.network.tick < self.config.max_ticks {
            // Process scheduled events
            let due_events = self.scheduler.due_at(self.network.tick);
            for event in due_events {
                self.network.queue_event(event);
            }

            // Advance simulation
            self.network.tick();

            // Progress reporting
            if let Some(interval) = self.config.progress_interval {
                if self.network.tick % interval == 0 {
                    if let Some(ref callback) = self.progress_callback {
                        callback(self.network.tick, self.config.max_ticks);
                    }
                }
            }
        }

        let duration = start.elapsed();

        SimulationResult {
            summary: self.network.metrics.summary(),
            duration_ms: duration.as_millis() as u64,
            final_tick: self.network.tick,
            completed: error.is_none(),
            error,
        }
    }

    /// Run with a specific scenario
    pub fn run_scenario(config: SimulationConfig, scenario: &dyn Scenario) -> SimulationResult {
        let mut runner = SimulationRunner::new(config);
        runner.apply_scenario(scenario);
        runner.run()
    }
}

/// Quick-run helper for simple simulations
pub fn quick_run(node_count: usize, ticks: u64) -> SimulationResult {
    use crate::agents::HonestAgent;
    use symbiont_core::capability::common;

    let config = SimulationConfig::default()
        .with_ticks(ticks)
        .with_network(
            NetworkConfig::default()
                .with_nodes(node_count)
                .with_capability(common::analysis())
                .with_connection_prob(0.3),
        );

    let mut runner = SimulationRunner::new(config);

    // Add honest agents to all nodes
    let node_ids: Vec<_> = runner.network().nodes().keys().cloned().collect();
    for id in node_ids {
        runner.network_mut().set_agent(id, Box::new(HonestAgent::new(0.5)));
    }

    runner.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_run() {
        let result = quick_run(5, 100);

        assert!(result.completed);
        assert_eq!(result.final_tick, 100);
        assert!(result.duration_ms < 10000); // Should be fast
    }

    #[test]
    fn test_simulation_runner() {
        let config = SimulationConfig::default()
            .with_ticks(50)
            .with_network(NetworkConfig::default().with_nodes(3));

        let mut runner = SimulationRunner::new(config);
        let result = runner.run();

        assert!(result.completed);
        assert_eq!(result.final_tick, 50);
    }
}
