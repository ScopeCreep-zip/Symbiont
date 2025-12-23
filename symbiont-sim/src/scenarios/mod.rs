//! Predefined simulation scenarios.

pub mod adversary;
mod trust_emergence;
pub mod workflow;

pub use adversary::{AdversaryScenario, AdversaryType};
pub use trust_emergence::TrustEmergenceScenario;
pub use workflow::{WorkflowScenario, WorkflowType};

use crate::events::EventScheduler;
use crate::network::Network;

/// Trait for simulation scenarios
pub trait Scenario: Send + Sync {
    /// Set up the scenario (add agents, schedule events)
    fn setup(&self, network: &mut Network, scheduler: &mut EventScheduler);

    /// Name of the scenario
    fn name(&self) -> &'static str;

    /// Description of the scenario
    fn description(&self) -> &'static str;
}
