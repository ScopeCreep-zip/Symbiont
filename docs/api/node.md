# Node API

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

This document specifies the Node API—the primary interface for interacting with Symbiont nodes. It covers initialization, task handling, trust queries, and administrative operations.

---

## 2. Node Creation

### 2.1 Constructor

```rust
impl Node {
    /// Create a new node with generated keypair
    pub fn new() -> Self;

    /// Create a node from existing keypair
    pub fn from_keypair(keypair: Keypair) -> Self;

    /// Create a node with configuration
    pub fn with_config(config: NodeConfig) -> Self;
}
```

### 2.2 NodeConfig

```rust
pub struct NodeConfig {
    /// Optional existing keypair
    pub keypair: Option<Keypair>,

    /// Initial capabilities
    pub capabilities: Vec<Capability>,

    /// Role category (affects swift trust)
    pub role: Option<RoleCategory>,

    /// Custom constants override
    pub constants: Option<ProtocolConstants>,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            keypair: None,
            capabilities: vec![],
            role: None,
            constants: None,
        }
    }
}
```

### 2.3 Example

```rust
// Simple creation
let node = Node::new();

// With capabilities
let config = NodeConfig {
    capabilities: vec![
        Capability::analysis("text-analysis"),
        Capability::generation("summarization"),
    ],
    role: Some(RoleCategory::Analyst),
    ..Default::default()
};
let node = Node::with_config(config);
```

---

## 3. Identity

### 3.1 Identity Methods

```rust
impl Node {
    /// Get node's unique identifier
    pub fn id(&self) -> NodeId;

    /// Get node's public key
    pub fn public_key(&self) -> PublicKey;

    /// Sign data with node's private key
    pub fn sign(&self, data: &[u8]) -> Signature;

    /// Verify a signature from another node
    pub fn verify(
        &self,
        node_id: &NodeId,
        data: &[u8],
        signature: &Signature,
    ) -> bool;
}
```

---

## 4. Status and Trust

### 4.1 Status Queries

```rust
impl Node {
    /// Get current node status
    pub fn status(&self) -> NodeStatus;

    /// Get current trust score
    pub fn trust(&self) -> Score;

    /// Get trust cap (may be reduced by flags)
    pub fn trust_cap(&self) -> Score;

    /// Get self-confidence score
    pub fn confidence(&self) -> Score;

    /// Get priming level
    pub fn priming(&self) -> Score;

    /// Get active flags
    pub fn flags(&self) -> &HashSet<Flag>;

    /// Get interaction count
    pub fn interaction_count(&self) -> u64;
}
```

### 4.2 Trust Computation

```rust
impl Node {
    /// Compute trust for another node (from this node's perspective)
    pub fn compute_trust_for(&self, node_id: &NodeId) -> Option<Score>;

    /// Get routing score for a candidate
    pub fn routing_score(
        &self,
        candidate: &NodeId,
        capability: &CapabilityId,
    ) -> Option<Score>;

    /// Get aggregated quality score
    pub fn quality_score(&self) -> Score;

    /// Get diversity score
    pub fn diversity_score(&self) -> Score;
}
```

---

## 5. Connections

### 5.1 Connection Management

```rust
impl Node {
    /// Get all connections
    pub fn connections(&self) -> &HashMap<NodeId, Connection>;

    /// Get specific connection
    pub fn connection(&self, partner: &NodeId) -> Option<&Connection>;

    /// Get connection count
    pub fn connection_count(&self) -> usize;

    /// Check if connected to a node
    pub fn is_connected(&self, partner: &NodeId) -> bool;

    /// Get all connection weights
    pub fn connection_weights(&self) -> Vec<(NodeId, Weight)>;
}
```

### 5.2 Connection Details

```rust
impl Connection {
    /// Get partner node ID
    pub fn partner_id(&self) -> NodeId;

    /// Get connection weight
    pub fn weight(&self) -> Weight;

    /// Get reciprocity score
    pub fn reciprocity(&self) -> f64;

    /// Get quality score
    pub fn quality(&self) -> Score;

    /// Get tone score
    pub fn tone(&self) -> SignedScore;

    /// Get last activity timestamp
    pub fn last_active(&self) -> Timestamp;

    /// Get interaction count
    pub fn interaction_count(&self) -> u32;
}
```

---

## 6. Capabilities

### 6.1 Capability Management

```rust
impl Node {
    /// Get all capabilities
    pub fn capabilities(&self) -> &HashMap<CapabilityId, CapabilityState>;

    /// Check if node has capability
    pub fn has_capability(&self, cap_id: &CapabilityId) -> bool;

    /// Get capability state
    pub fn capability(&self, cap_id: &CapabilityId) -> Option<&CapabilityState>;

    /// Add a capability
    pub fn add_capability(&mut self, capability: Capability);

    /// Remove a capability
    pub fn remove_capability(&mut self, cap_id: &CapabilityId);

    /// Set capability availability
    pub fn set_capability_available(
        &mut self,
        cap_id: &CapabilityId,
        available: bool,
    );

    /// Get current load
    pub fn load(&self) -> Score;

    /// Is the node overloaded?
    pub fn is_overloaded(&self) -> bool;
}
```

### 6.2 Capability State

```rust
impl CapabilityState {
    /// Get capability definition
    pub fn capability(&self) -> &Capability;

    /// Get quality score for this capability
    pub fn quality(&self) -> Score;

    /// Get usage volume
    pub fn volume(&self) -> u32;

    /// Get last used timestamp
    pub fn last_used(&self) -> Timestamp;

    /// Is currently available?
    pub fn is_available(&self) -> bool;

    /// Get current load
    pub fn load(&self) -> Score;
}
```

---

## 7. Task Processing

### 7.1 Task Handling

```rust
impl Node {
    /// Process a task (route or execute locally)
    pub async fn process_task(
        &mut self,
        task: Task,
        network: &Network,
    ) -> TaskResult;

    /// Execute a task locally
    pub async fn execute_task(&mut self, task: Task) -> TaskResult;

    /// Route a task to the best candidate
    pub async fn route_task(
        &self,
        task: &Task,
        network: &Network,
    ) -> RoutingResult;

    /// Find candidates for a task
    pub fn find_candidates(
        &self,
        task: &Task,
        network: &Network,
    ) -> Vec<NodeId>;

    /// Score a candidate for routing
    pub fn score_candidate(
        &self,
        candidate: &NodeId,
        task: &Task,
    ) -> Score;
}
```

### 7.2 Task Execution

```rust
/// Result of task processing
pub enum TaskResult {
    /// Executed locally
    Local(TaskOutput),

    /// Routed to another node
    Routed {
        target: NodeId,
        result: TaskOutput,
    },

    /// No suitable candidates found
    NoCandidates,

    /// Task failed
    Failed(TaskError),
}
```

---

## 8. Workflow Execution

### 8.1 Workflow Methods

```rust
impl Node {
    /// Execute a workflow
    pub async fn execute_workflow(
        &mut self,
        workflow: Workflow,
        context: WorkflowContext,
    ) -> WorkflowResult;

    /// Get workflow status
    pub fn workflow_status(&self, id: &WorkflowId) -> Option<WorkflowStatus>;

    /// Cancel a running workflow
    pub fn cancel_workflow(&mut self, id: &WorkflowId) -> bool;
}
```

### 8.2 Handoff Operations

```rust
impl Node {
    /// Perform handoff to another node
    pub async fn handoff(
        &mut self,
        target: &NodeId,
        task: Task,
        context: HandoffContext,
    ) -> HandoffResult;

    /// Receive and process a handoff
    pub async fn receive_handoff(
        &mut self,
        handoff: Handoff,
    ) -> HandoffResult;
}
```

---

## 9. Defense Operations

### 9.1 Threat Management

```rust
impl Node {
    /// Get threat beliefs
    pub fn threat_beliefs(&self) -> &HashMap<NodeId, ThreatBelief>;

    /// Get threat belief for specific node
    pub fn threat_belief(&self, node_id: &NodeId) -> Option<&ThreatBelief>;

    /// Emit a defense signal
    pub fn emit_defense_signal(
        &mut self,
        threat: NodeId,
        threat_type: ThreatType,
        evidence: Hash,
        confidence: Score,
    );

    /// Handle an incoming defense signal
    pub fn handle_defense_signal(&mut self, signal: DefenseSignal);

    /// Take defensive action
    pub fn take_defensive_action(&mut self, threat: &NodeId);

    /// Get defense state
    pub fn defense_state(&self) -> DefenseState;
}
```

### 9.2 Signal Propagation

```rust
impl Node {
    /// Propagate a signal to connections
    fn propagate_signal(&mut self, signal: DefenseSignal);

    /// Should this signal be propagated?
    fn should_propagate(&self, signal: &DefenseSignal) -> bool;
}
```

---

## 10. Affirmations

### 10.1 Affirmation Methods

```rust
impl Node {
    /// Send an affirmation to another node
    pub fn send_affirmation(
        &mut self,
        to: NodeId,
        affirmation_type: AffirmationType,
        strength: Score,
    );

    /// Handle a received affirmation
    pub fn receive_affirmation(&mut self, affirmation: Affirmation);

    /// Get received affirmations
    pub fn affirmations_received(&self) -> &[Affirmation];
}
```

---

## 11. Interaction Recording

### 11.1 Recording Interactions

```rust
impl Node {
    /// Record an interaction and update connection
    pub fn record_interaction(
        &mut self,
        partner: NodeId,
        interaction: Interaction,
    );

    /// Get interaction history
    pub fn interaction_history(&self) -> &[Interaction];

    /// Get recent interactions (last N)
    pub fn recent_interactions(&self, count: usize) -> &[Interaction];
}
```

---

## 12. Background Operations

### 12.1 Tick Processing

```rust
impl Node {
    /// Process background operations (call periodically)
    pub fn tick(&mut self);

    /// Decay idle connections
    fn decay_connections(&mut self);

    /// Decay priming level
    fn decay_priming(&mut self);

    /// Update capability loads
    fn update_capability_loads(&mut self);

    /// Check status transitions
    fn check_status_transition(&mut self);

    /// Scan for adversaries
    fn scan_for_adversaries(&mut self) -> Vec<NodeId>;
}
```

---

## 13. Persistence

### 13.1 Serialization

```rust
impl Node {
    /// Serialize node state for persistence
    pub fn serialize(&self) -> Vec<u8>;

    /// Deserialize node state
    pub fn deserialize(data: &[u8]) -> Result<Self, Error>;

    /// Export state as JSON (for debugging)
    pub fn to_json(&self) -> String;
}
```

### 13.2 Selective Persistence

```rust
impl Node {
    /// Get persistable state (excludes session state)
    pub fn persistable_state(&self) -> PersistedNodeState;

    /// Restore from persisted state
    pub fn restore(state: PersistedNodeState, keypair: Keypair) -> Self;
}
```

---

## 14. Callbacks

### 14.1 Event Callbacks

```rust
impl Node {
    /// Set callback for interactions
    pub fn on_interaction<F>(&mut self, callback: F)
    where
        F: Fn(&Interaction) + 'static;

    /// Set callback for defense signals
    pub fn on_defense_signal<F>(&mut self, callback: F)
    where
        F: Fn(&DefenseSignal) + 'static;

    /// Set callback for status changes
    pub fn on_status_change<F>(&mut self, callback: F)
    where
        F: Fn(NodeStatus, NodeStatus) + 'static;

    /// Set callback for trust changes
    pub fn on_trust_change<F>(&mut self, callback: F)
    where
        F: Fn(Score, Score) + 'static;
}
```

---

## 15. Error Types

### 15.1 Node Errors

```rust
pub enum NodeError {
    /// Invalid signature
    InvalidSignature,

    /// Missing capability
    MissingCapability(CapabilityId),

    /// Node overloaded
    Overloaded,

    /// Task timeout
    Timeout,

    /// Network error
    NetworkError(String),

    /// Serialization error
    SerializationError(String),

    /// Invalid state transition
    InvalidTransition(NodeStatus, NodeStatus),
}
```

---

## 16. Complete Example

```rust
use symbiont_core::*;

async fn example() -> Result<(), NodeError> {
    // Create a node with capabilities
    let mut node = Node::with_config(NodeConfig {
        capabilities: vec![
            Capability::analysis("text-analysis"),
        ],
        ..Default::default()
    });

    println!("Node ID: {:?}", node.id());
    println!("Status: {:?}", node.status());
    println!("Trust: {}", node.trust());

    // Process a task
    let task = Task::new(CapabilityId::from("text-analysis"))
        .with_input(b"Analyze this text".to_vec())
        .with_timeout(Duration::from_secs(30));

    let network = get_network(); // Assume network is available
    let result = node.process_task(task, &network).await;

    match result {
        TaskResult::Local(output) => {
            println!("Executed locally: {:?}", output);
        }
        TaskResult::Routed { target, result } => {
            println!("Routed to {:?}: {:?}", target, result);
        }
        TaskResult::NoCandidates => {
            println!("No candidates available");
        }
        TaskResult::Failed(e) => {
            return Err(e);
        }
    }

    // Check connections
    for (partner, conn) in node.connections() {
        println!(
            "Connection to {:?}: w={:.2}, r={:.2}",
            partner,
            conn.weight(),
            conn.reciprocity(),
        );
    }

    // Run background tick
    node.tick();

    Ok(())
}
```

---

## 17. Summary

The Node API provides:

| Category | Methods |
|----------|---------|
| Identity | id, sign, verify |
| Status | status, trust, flags |
| Connections | connections, connection_weights |
| Capabilities | has_capability, add_capability |
| Tasks | process_task, route_task |
| Workflows | execute_workflow |
| Defense | emit_defense_signal, threat_beliefs |
| Affirmations | send_affirmation |
| Background | tick, decay_connections |
| Persistence | serialize, deserialize |

---

*Previous: [Mathematical Functions](./math.md) | Back to [Documentation Index](../index.md)*
