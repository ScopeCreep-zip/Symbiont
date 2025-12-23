//! Workflow execution for Symbiont.
//!
//! Supports sequential, parallel, and DAG-based workflows.

use crate::routing::{route_task, RoutingResult, Task};
use crate::node::Node;
use crate::types::{NodeId, Score, StepId, TaskId, Timestamp, WorkflowId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowType {
    /// Single task, single agent
    Single,
    /// Sequential: A → B → C
    Sequential,
    /// Parallel: A, B, C simultaneously
    Parallel,
    /// Directed acyclic graph of dependencies
    Dag,
}

/// Status of a workflow
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkflowStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Status of a workflow step
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    Ready,
    Running,
    Completed,
    Failed,
    Skipped,
}

/// Result of a workflow step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    /// Step that produced this result
    pub step_id: StepId,
    /// Whether the step succeeded
    pub success: bool,
    /// Output data (opaque bytes)
    pub output: Vec<u8>,
    /// Quality of the result
    pub quality: Score,
    /// Node that executed the step
    pub executor: NodeId,
    /// Execution time in ms
    pub duration_ms: u64,
}

/// A step in a workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Unique step identifier
    pub id: StepId,
    /// Task to execute
    pub task: Task,
    /// Node assigned to execute (filled during execution)
    pub assigned_to: Option<NodeId>,
    /// Dependencies (steps that must complete first)
    pub depends_on: Vec<StepId>,
    /// Current status
    pub status: StepStatus,
    /// Result (if completed)
    pub result: Option<StepResult>,
}

impl WorkflowStep {
    /// Create a new workflow step
    pub fn new(id: StepId, task: Task) -> Self {
        Self {
            id,
            task,
            assigned_to: None,
            depends_on: Vec::new(),
            status: StepStatus::Pending,
            result: None,
        }
    }

    /// Add a dependency
    pub fn depends_on(mut self, step_id: StepId) -> Self {
        self.depends_on.push(step_id);
        self
    }

    /// Check if all dependencies are satisfied
    pub fn dependencies_satisfied(&self, completed_steps: &[StepId]) -> bool {
        self.depends_on.iter().all(|dep| completed_steps.contains(dep))
    }
}

/// Context accumulated during workflow execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Workflow identifier
    pub workflow_id: WorkflowId,
    /// Current step index (for sequential)
    pub step_index: u32,
    /// Results from prior steps
    pub prior_results: Vec<StepResult>,
    /// Accumulated key-value data
    pub data: HashMap<String, Vec<u8>>,
    /// Nodes that have touched this workflow
    pub lineage: Vec<NodeId>,
}

impl WorkflowContext {
    /// Create a new context
    pub fn new(workflow_id: WorkflowId) -> Self {
        Self {
            workflow_id,
            step_index: 0,
            prior_results: Vec::new(),
            data: HashMap::new(),
            lineage: Vec::new(),
        }
    }

    /// Add a step result
    pub fn add_result(&mut self, result: StepResult) {
        if !self.lineage.contains(&result.executor) {
            self.lineage.push(result.executor);
        }
        self.prior_results.push(result);
        self.step_index += 1;
    }

    /// Set data
    pub fn set_data(&mut self, key: impl Into<String>, value: Vec<u8>) {
        self.data.insert(key.into(), value);
    }

    /// Get data
    pub fn get_data(&self, key: &str) -> Option<&Vec<u8>> {
        self.data.get(key)
    }
}

/// A workflow definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// Unique identifier
    pub id: WorkflowId,
    /// Type of workflow
    pub workflow_type: WorkflowType,
    /// Steps in the workflow
    pub steps: Vec<WorkflowStep>,
    /// Execution context
    pub context: WorkflowContext,
    /// Current status
    pub status: WorkflowStatus,
    /// When the workflow was created
    pub created: Timestamp,
    /// When the workflow started executing
    pub started: Option<Timestamp>,
    /// When the workflow completed
    pub completed: Option<Timestamp>,
}

impl Workflow {
    /// Create a new workflow
    pub fn new(id: WorkflowId, workflow_type: WorkflowType) -> Self {
        Self {
            id,
            workflow_type,
            steps: Vec::new(),
            context: WorkflowContext::new(id),
            status: WorkflowStatus::Pending,
            created: Timestamp::now(),
            started: None,
            completed: None,
        }
    }

    /// Add a step
    pub fn add_step(&mut self, step: WorkflowStep) {
        self.steps.push(step);
    }

    /// Get steps that are ready to execute
    pub fn ready_steps(&self) -> Vec<&WorkflowStep> {
        let completed: Vec<_> = self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .map(|s| s.id)
            .collect();

        self.steps
            .iter()
            .filter(|s| s.status == StepStatus::Pending && s.dependencies_satisfied(&completed))
            .collect()
    }

    /// Mark a step as started
    pub fn start_step(&mut self, step_id: StepId, executor: NodeId) {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == step_id) {
            step.status = StepStatus::Running;
            step.assigned_to = Some(executor);
        }

        if self.status == WorkflowStatus::Pending {
            self.status = WorkflowStatus::Running;
            self.started = Some(Timestamp::now());
        }
    }

    /// Complete a step
    pub fn complete_step(&mut self, step_id: StepId, result: StepResult) {
        if let Some(step) = self.steps.iter_mut().find(|s| s.id == step_id) {
            step.status = if result.success {
                StepStatus::Completed
            } else {
                StepStatus::Failed
            };
            step.result = Some(result.clone());
        }

        self.context.add_result(result);
        self.check_completion();
    }

    /// Check if workflow is complete
    fn check_completion(&mut self) {
        let all_done = self.steps.iter().all(|s| {
            matches!(s.status, StepStatus::Completed | StepStatus::Failed | StepStatus::Skipped)
        });

        if all_done {
            let any_failed = self.steps.iter().any(|s| s.status == StepStatus::Failed);
            self.status = if any_failed {
                WorkflowStatus::Failed
            } else {
                WorkflowStatus::Completed
            };
            self.completed = Some(Timestamp::now());
        }
    }

    /// Calculate overall quality
    pub fn overall_quality(&self) -> Score {
        let completed: Vec<_> = self.steps
            .iter()
            .filter_map(|s| s.result.as_ref())
            .filter(|r| r.success)
            .collect();

        if completed.is_empty() {
            return Score::ZERO;
        }

        let sum: f64 = completed.iter().map(|r| r.quality.value()).sum();
        Score::new(sum / completed.len() as f64)
    }
}

/// Result of workflow execution
#[derive(Debug, Clone)]
pub enum WorkflowResult {
    /// Workflow completed successfully
    Success(WorkflowContext),
    /// Workflow failed at a specific step
    Failed {
        step_id: StepId,
        reason: String,
    },
    /// Workflow is still running
    InProgress,
}

/// Execute a sequential workflow
pub fn execute_sequential_workflow(
    workflow: &mut Workflow,
    current_node: &Node,
    network: &HashMap<NodeId, Node>,
) -> WorkflowResult {
    for step in workflow.steps.iter_mut() {
        if step.status != StepStatus::Pending {
            continue;
        }

        // Route to best candidate
        let routing = route_task(current_node, &step.task, network);

        match routing {
            RoutingResult::Success(candidate) => {
                step.assigned_to = Some(candidate.node_id);
                step.status = StepStatus::Ready;
                // In real implementation, would execute here
            }
            RoutingResult::NoCandidates | RoutingResult::ConstraintsNotMet => {
                step.status = StepStatus::Failed;
                workflow.status = WorkflowStatus::Failed;
                return WorkflowResult::Failed {
                    step_id: step.id,
                    reason: String::from("No suitable candidates"),
                };
            }
        }
    }

    WorkflowResult::InProgress
}

/// Common workflow patterns
pub mod patterns {
    use super::*;
    use crate::types::CapabilityId;

    /// Create a chain workflow: cap1 → cap2 → cap3
    pub fn chain(origin: NodeId, capabilities: Vec<CapabilityId>) -> Workflow {
        let id = WorkflowId::random();
        let mut workflow = Workflow::new(id, WorkflowType::Sequential);

        let mut prev_step: Option<StepId> = None;

        for (i, cap) in capabilities.into_iter().enumerate() {
            let step_id = StepId::new(i as u64);
            let task = Task::new(TaskId::random(), origin, cap);
            let mut step = WorkflowStep::new(step_id, task);

            if let Some(prev) = prev_step {
                step = step.depends_on(prev);
            }

            workflow.add_step(step);
            prev_step = Some(step_id);
        }

        workflow
    }

    /// Create a fan-out/fan-in workflow
    pub fn fan_out_fan_in(
        origin: NodeId,
        parallel_cap: CapabilityId,
        parallelism: usize,
        merge_cap: CapabilityId,
    ) -> Workflow {
        let id = WorkflowId::random();
        let mut workflow = Workflow::new(id, WorkflowType::Dag);

        let mut parallel_step_ids = Vec::new();

        // Parallel steps
        for i in 0..parallelism {
            let step_id = StepId::new(i as u64);
            let task = Task::new(TaskId::random(), origin, parallel_cap);
            let step = WorkflowStep::new(step_id, task);
            workflow.add_step(step);
            parallel_step_ids.push(step_id);
        }

        // Merge step depends on all parallel steps
        let merge_step_id = StepId::new(parallelism as u64);
        let merge_task = Task::new(TaskId::random(), origin, merge_cap);
        let mut merge_step = WorkflowStep::new(merge_step_id, merge_task);

        for step_id in parallel_step_ids {
            merge_step = merge_step.depends_on(step_id);
        }

        workflow.add_step(merge_step);
        workflow
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::capability::common;

    #[test]
    fn test_workflow_creation() {
        let id = WorkflowId::random();
        let workflow = Workflow::new(id, WorkflowType::Sequential);

        assert_eq!(workflow.status, WorkflowStatus::Pending);
        assert!(workflow.steps.is_empty());
    }

    #[test]
    fn test_step_dependencies() {
        let step1 = StepId::new(1);
        let step2 = StepId::new(2);

        let task = Task::new(TaskId::random(), NodeId::from_index(0), common::analysis().id);
        let step = WorkflowStep::new(step2, task).depends_on(step1);

        // Not satisfied if step1 not completed
        assert!(!step.dependencies_satisfied(&[]));

        // Satisfied if step1 completed
        assert!(step.dependencies_satisfied(&[step1]));
    }

    #[test]
    fn test_chain_pattern() {
        let origin = NodeId::from_index(0);
        let caps = vec![
            common::analysis().id,
            common::transformation().id,
            common::validation().id,
        ];

        let workflow = patterns::chain(origin, caps);

        assert_eq!(workflow.workflow_type, WorkflowType::Sequential);
        assert_eq!(workflow.steps.len(), 3);

        // First step has no dependencies
        assert!(workflow.steps[0].depends_on.is_empty());

        // Second step depends on first
        assert_eq!(workflow.steps[1].depends_on.len(), 1);
        assert!(workflow.steps[1].depends_on.contains(&StepId::new(0)));

        // Third step depends on second
        assert!(workflow.steps[2].depends_on.contains(&StepId::new(1)));
    }

    #[test]
    fn test_fan_out_fan_in() {
        let origin = NodeId::from_index(0);
        let workflow = patterns::fan_out_fan_in(
            origin,
            common::analysis().id,
            3, // 3 parallel workers
            common::transformation().id,
        );

        assert_eq!(workflow.workflow_type, WorkflowType::Dag);
        assert_eq!(workflow.steps.len(), 4); // 3 parallel + 1 merge

        // Merge step should depend on all 3 parallel steps
        let merge_step = &workflow.steps[3];
        assert_eq!(merge_step.depends_on.len(), 3);
    }

    #[test]
    fn test_workflow_context() {
        let mut context = WorkflowContext::new(WorkflowId::random());

        context.set_data("key1", vec![1, 2, 3]);

        let result = StepResult {
            step_id: StepId::new(0),
            success: true,
            output: vec![4, 5, 6],
            quality: Score::new(0.9),
            executor: NodeId::from_index(1),
            duration_ms: 100,
        };

        context.add_result(result);

        assert_eq!(context.step_index, 1);
        assert_eq!(context.prior_results.len(), 1);
        assert!(context.lineage.contains(&NodeId::from_index(1)));
        assert_eq!(context.get_data("key1"), Some(&vec![1, 2, 3]));
    }

    #[test]
    fn test_ready_steps() {
        let id = WorkflowId::random();
        let mut workflow = Workflow::new(id, WorkflowType::Dag);

        let task = Task::new(TaskId::random(), NodeId::from_index(0), common::analysis().id);

        let step1 = WorkflowStep::new(StepId::new(0), task.clone());
        let step2 = WorkflowStep::new(StepId::new(1), task.clone()).depends_on(StepId::new(0));
        let step3 = WorkflowStep::new(StepId::new(2), task); // No deps

        workflow.add_step(step1);
        workflow.add_step(step2);
        workflow.add_step(step3);

        // Steps without deps should be ready
        let ready = workflow.ready_steps();
        assert_eq!(ready.len(), 2); // step1 and step3

        // Complete step1
        workflow.steps[0].status = StepStatus::Completed;

        // Now step2 should also be ready
        let ready = workflow.ready_steps();
        assert_eq!(ready.len(), 2); // step2 and step3
    }
}
