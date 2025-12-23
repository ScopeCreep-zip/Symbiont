//! # Symbiont Simulation Harness
//!
//! Simulation framework for testing and validating the Symbiont protocol.
//!
//! ## Modules
//!
//! - [`network`] - Simulated network of nodes
//! - [`agents`] - Agent behavior models
//! - [`scenarios`] - Predefined simulation scenarios
//! - [`metrics`] - Metrics collection and export
//! - [`events`] - Discrete event system
//! - [`runner`] - Simulation executor

pub mod agents;
pub mod events;
pub mod metrics;
pub mod network;
pub mod runner;
pub mod scenarios;

pub use network::Network;
pub use runner::SimulationRunner;
