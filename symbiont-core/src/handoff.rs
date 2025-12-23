//! Handoff protocol for task transfer between nodes.

use crate::routing::Task;
use crate::types::{Hash, NodeId, Signature, Timestamp, WorkflowId};
use crate::workflow::WorkflowContext;
use serde::{Deserialize, Serialize};

/// A handoff of work from one node to another
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handoff {
    /// Node handing off the work
    pub from_node: NodeId,
    /// Node receiving the work
    pub to_node: NodeId,
    /// The task being handed off
    pub task: Task,
    /// Context for the handoff
    pub context: HandoffContext,
    /// Timestamp of handoff
    pub timestamp: Timestamp,
    /// Signature from the sending node
    pub signature: Signature,
}

/// Context passed with a handoff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffContext {
    /// Workflow this handoff is part of (if any)
    pub workflow_id: Option<WorkflowId>,
    /// Step index within the workflow
    pub step_index: u32,
    /// Results from prior steps
    pub prior_results: Vec<Vec<u8>>,
    /// Accumulated data
    pub accumulated: Vec<(String, Vec<u8>)>,
    /// Lineage of nodes that have touched this work
    pub lineage: Vec<NodeId>,
}

impl HandoffContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            workflow_id: None,
            step_index: 0,
            prior_results: Vec::new(),
            accumulated: Vec::new(),
            lineage: Vec::new(),
        }
    }

    /// Create context from a workflow context
    pub fn from_workflow(ctx: &WorkflowContext) -> Self {
        Self {
            workflow_id: Some(ctx.workflow_id),
            step_index: ctx.step_index,
            prior_results: ctx.prior_results.iter().map(|r| r.output.clone()).collect(),
            accumulated: ctx.data.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
            lineage: ctx.lineage.clone(),
        }
    }

    /// Add to lineage
    pub fn add_to_lineage(&mut self, node: NodeId) {
        if !self.lineage.contains(&node) {
            self.lineage.push(node);
        }
    }
}

impl Default for HandoffContext {
    fn default() -> Self {
        Self::new()
    }
}

impl Handoff {
    /// Create a new handoff
    pub fn new(from_node: NodeId, to_node: NodeId, task: Task, context: HandoffContext) -> Self {
        Self {
            from_node,
            to_node,
            task,
            context,
            timestamp: Timestamp::now(),
            signature: Signature::new([0u8; 64]), // Placeholder
        }
    }

    /// Compute the hash of this handoff (for signing)
    pub fn compute_hash(&self) -> Hash {
        // In production, serialize properly
        let data = format!(
            "{}:{}:{}:{}",
            self.from_node,
            self.to_node,
            self.task.id.0,
            self.timestamp.millis()
        );
        Hash::compute(data.as_bytes())
    }

    /// Check if handoff is still valid (not too old)
    pub fn is_valid(&self, max_age_ms: u64) -> bool {
        !self.timestamp.is_older_than(max_age_ms)
    }
}

/// Result of processing a handoff
#[derive(Debug, Clone)]
pub enum HandoffResult {
    /// Successfully accepted
    Success {
        /// Result data
        output: Vec<u8>,
    },
    /// Invalid signature
    InvalidSignature,
    /// Missing required capability
    MissingCapability,
    /// Node is overloaded
    Overloaded,
    /// Handoff expired
    Expired,
    /// Execution failed
    ExecutionFailed {
        reason: String,
    },
}

impl HandoffResult {
    /// Check if handoff was successful
    pub fn is_success(&self) -> bool {
        matches!(self, HandoffResult::Success { .. })
    }
}

/// Handler for processing incoming handoffs
pub struct HandoffHandler {
    /// Maximum age for handoffs (in ms)
    max_age_ms: u64,
}

impl HandoffHandler {
    /// Create a new handoff handler
    pub fn new(max_age_ms: u64) -> Self {
        Self { max_age_ms }
    }

    /// Validate a handoff
    pub fn validate(&self, handoff: &Handoff) -> Option<HandoffResult> {
        // Check expiry
        if !handoff.is_valid(self.max_age_ms) {
            return Some(HandoffResult::Expired);
        }

        // In production, verify signature here
        // if !verify_signature(&handoff.signature, &handoff.from_node) {
        //     return Some(HandoffResult::InvalidSignature);
        // }

        None // Valid
    }

    /// Process a handoff for a node
    pub fn process(
        &self,
        handoff: &Handoff,
        node: &crate::node::Node,
    ) -> HandoffResult {
        // Validate
        if let Some(result) = self.validate(handoff) {
            return result;
        }

        // Check capability
        if !handoff.task.required_caps.is_empty() {
            let cap = handoff.task.required_caps[0];
            if !node.has_capability(cap) {
                return HandoffResult::MissingCapability;
            }

            if !node.can_accept_capability_work(cap) {
                return HandoffResult::Overloaded;
            }
        }

        // In production, would actually execute the task here
        HandoffResult::Success {
            output: Vec::new(),
        }
    }
}

impl Default for HandoffHandler {
    fn default() -> Self {
        Self::new(60_000) // 1 minute default
    }
}

/// Create a handoff from one node to another
pub fn create_handoff(
    from: &crate::node::Node,
    to_node: NodeId,
    task: Task,
    workflow_ctx: Option<&WorkflowContext>,
) -> Handoff {
    let context = workflow_ctx
        .map(HandoffContext::from_workflow)
        .unwrap_or_default();

    Handoff::new(from.id, to_node, task, context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::common;
    use crate::routing::Task;
    use crate::types::TaskId;

    #[test]
    fn test_handoff_creation() {
        let from = NodeId::from_index(1);
        let to = NodeId::from_index(2);
        let task = Task::new(TaskId::random(), from, common::analysis().id);

        let handoff = Handoff::new(from, to, task, HandoffContext::new());

        assert_eq!(handoff.from_node, from);
        assert_eq!(handoff.to_node, to);
        assert!(handoff.is_valid(60_000));
    }

    #[test]
    fn test_handoff_context() {
        let mut ctx = HandoffContext::new();
        ctx.add_to_lineage(NodeId::from_index(1));
        ctx.add_to_lineage(NodeId::from_index(2));
        ctx.add_to_lineage(NodeId::from_index(1)); // Duplicate

        assert_eq!(ctx.lineage.len(), 2); // No duplicate
    }

    #[test]
    fn test_handoff_handler() {
        let handler = HandoffHandler::new(60_000);

        let from = NodeId::from_index(1);
        let to = NodeId::from_index(2);
        let task = Task::new(TaskId::random(), from, common::analysis().id);
        let handoff = Handoff::new(from, to, task, HandoffContext::new());

        // Should be valid
        assert!(handler.validate(&handoff).is_none());
    }

    #[test]
    fn test_expired_handoff() {
        let handler = HandoffHandler::new(0); // Immediate expiry

        let from = NodeId::from_index(1);
        let to = NodeId::from_index(2);
        let task = Task::new(TaskId::random(), from, common::analysis().id);
        let mut handoff = Handoff::new(from, to, task, HandoffContext::new());

        // Manually set timestamp to 100ms in the past to ensure expiry
        handoff.timestamp = Timestamp::new(Timestamp::now().millis().saturating_sub(100));

        // Should be expired
        let result = handler.validate(&handoff);
        assert!(matches!(result, Some(HandoffResult::Expired)));
    }
}
