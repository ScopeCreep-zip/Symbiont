//! # Symbiont Core
//!
//! Implementation of the Symbiont Mycorrhizal Trust Protocol v1.0.
//!
//! Symbiont is a decentralized trust and orchestration protocol inspired by
//! biological mycorrhizal networks. Trust emerges organically through interactions,
//! and orchestration decisions flow naturally from trust dynamics.
//!
//! ## Core Concepts
//!
//! - **Nodes**: Agents in the network with trust scores, capabilities, and connections
//! - **Connections**: Weighted relationships between nodes, governed by the Physarum equation
//! - **Trust**: Computed from quality, reciprocity, social proof, and diversity
//! - **Routing**: Tasks flow to the best-suited nodes based on trust and capability
//! - **Defense**: Threat signals propagate through trusted connections
//!
//! ## The Physarum Equation
//!
//! Connection weights evolve according to:
//!
//! ```text
//! dw/dt = γ × |Q|^μ × σ(r) × ψ(q) × φ(τ) - α×w - D
//! ```
//!
//! Where:
//! - Q = interaction volume (flow)
//! - r = reciprocity score
//! - q = quality score
//! - τ = tone score
//! - D = defense dampening
//!
//! ## Modules
//!
//! - [`constants`] - Protocol parameters
//! - [`types`] - Core types (NodeId, Score, Weight, etc.)
//! - [`math`] - Mathematical functions (sigmoid, multipliers, etc.)
//! - [`node`] - Node structure and state
//! - [`connection`] - Connection dynamics
//! - [`interaction`] - Interaction handling
//! - [`capability`] - Capability model
//! - [`trust`] - Trust computation
//! - [`defense`] - Defense signals and threat beliefs
//! - [`routing`] - Task routing
//! - [`workflow`] - Workflow execution
//! - [`handoff`] - Handoff protocol
//! - [`convergence`] - Convergence tracking
//! - [`detection`] - Adversary detection

pub mod constants;
pub mod math;
pub mod types;

// Phase 2 modules (to be implemented)
pub mod capability;
pub mod connection;
pub mod interaction;
pub mod node;

// Phase 3 modules
pub mod trust;

// Phase 4 modules
pub mod defense;
pub mod detection;

// Phase 5 modules
pub mod handoff;
pub mod routing;
pub mod workflow;

// Phase 6 modules
pub mod convergence;

// Re-export commonly used types
pub use types::{
    CapabilityId, Hash, NodeId, Score, Signature, SignedScore, StepId, TaskId, Timestamp, Weight,
    WorkflowId,
};

/// Protocol version
pub const VERSION: &str = "1.0.0";
