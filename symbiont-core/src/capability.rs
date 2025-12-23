//! Capability model for Symbiont nodes.
//!
//! Each node declares capabilities it can perform, with per-capability quality tracking.

use crate::types::{CapabilityId, Score, Timestamp};
use serde::{Deserialize, Serialize};

/// Category of capability
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CapabilityCategory {
    /// Analysis tasks (reasoning, classification, etc.)
    Analysis,
    /// Generation tasks (content creation, synthesis)
    Generation,
    /// Transformation tasks (format conversion, translation)
    Transformation,
    /// Validation tasks (verification, checking)
    Validation,
}

/// A capability that a node can perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    /// Unique identifier
    pub id: CapabilityId,
    /// Human-readable name
    pub name: String,
    /// Category of this capability
    pub category: CapabilityCategory,
    /// Description of what this capability does
    pub description: String,
}

impl Capability {
    /// Create a new capability
    pub fn new(id: CapabilityId, name: impl Into<String>, category: CapabilityCategory) -> Self {
        Self {
            id,
            name: name.into(),
            category,
            description: String::new(),
        }
    }

    /// Create a capability with automatic ID from name
    pub fn from_name(name: impl Into<String>, category: CapabilityCategory) -> Self {
        let name = name.into();
        let id = CapabilityId::from_name(&name);
        Self {
            id,
            name,
            category,
            description: String::new(),
        }
    }

    /// Add a description
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}

/// State of a capability for a specific node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityState {
    /// The capability definition
    pub capability: Capability,
    /// Quality score for this capability (tracked separately from global quality)
    pub quality: Score,
    /// Number of times this capability has been used
    pub volume: u32,
    /// Last time this capability was used
    pub last_used: Timestamp,
    /// Whether this capability is currently available
    pub available: bool,
    /// Current load for this capability (0 = idle, 1 = maxed out)
    pub load: Score,
}

impl CapabilityState {
    /// Create a new capability state with default values
    pub fn new(capability: Capability) -> Self {
        Self {
            capability,
            quality: Score::HALF, // Start neutral
            volume: 0,
            last_used: Timestamp::now(),
            available: true,
            load: Score::ZERO,
        }
    }

    /// Check if this capability can accept more work
    pub fn can_accept_work(&self) -> bool {
        self.available && self.load.value() < 0.95
    }

    /// Update quality with new observation using EMA
    pub fn update_quality(&mut self, observed: Score, lambda: f64) {
        let new_quality = lambda * self.quality.value() + (1.0 - lambda) * observed.value();
        self.quality = Score::new(new_quality);
    }

    /// Record usage of this capability
    pub fn record_usage(&mut self, quality: Score, lambda: f64) {
        self.update_quality(quality, lambda);
        self.volume += 1;
        self.last_used = Timestamp::now();
    }

    /// Apply load decay (called periodically)
    pub fn decay_load(&mut self, decay_factor: f64) {
        self.load = Score::new(self.load.value() * decay_factor);
        self.available = self.load.value() < 0.9;
    }
}

/// Common capabilities used in simulations
pub mod common {
    use super::*;

    pub fn analysis() -> Capability {
        Capability::from_name("analysis", CapabilityCategory::Analysis)
            .with_description("General analysis and reasoning")
    }

    pub fn generation() -> Capability {
        Capability::from_name("generation", CapabilityCategory::Generation)
            .with_description("Content generation and synthesis")
    }

    pub fn transformation() -> Capability {
        Capability::from_name("transformation", CapabilityCategory::Transformation)
            .with_description("Data transformation and conversion")
    }

    pub fn validation() -> Capability {
        Capability::from_name("validation", CapabilityCategory::Validation)
            .with_description("Verification and validation")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capability_creation() {
        let cap = Capability::from_name("test_cap", CapabilityCategory::Analysis);
        assert_eq!(cap.name, "test_cap");
        assert_eq!(cap.category, CapabilityCategory::Analysis);
    }

    #[test]
    fn test_capability_state() {
        let cap = common::analysis();
        let mut state = CapabilityState::new(cap);

        assert!(state.can_accept_work());
        assert_eq!(state.volume, 0);

        // Record usage
        state.record_usage(Score::new(0.8), 0.9);
        assert_eq!(state.volume, 1);
        assert!(state.quality.value() > 0.5);
    }

    #[test]
    fn test_load_management() {
        let cap = common::generation();
        let mut state = CapabilityState::new(cap);

        state.load = Score::new(0.96);
        assert!(!state.can_accept_work());

        state.decay_load(0.9);
        assert!(state.load.value() < 0.9);
        assert!(state.can_accept_work());
    }
}
