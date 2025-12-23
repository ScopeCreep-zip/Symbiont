# Symbiont: Mycorrhizal Trust Protocol v0.1
## Language-Agnostic Specification

**Version 0.1 — December 2025**

> This specification defines Symbiont in pure mathematics and abstract pseudocode.
> It can be implemented in ANY programming language: Rust, Go, TypeScript, Python, C++, Haskell, etc.

---

# SECTION 1: DATA STRUCTURES

## 1.1 Core Types

```
TYPE NodeId       = bytes[32]           // Cryptographic identifier
TYPE Timestamp    = uint64              // Unix timestamp ms
TYPE Score        = float64 ∈ [0, 1]    // Normalized score
TYPE SignedScore  = float64 ∈ [-1, 1]   // Signed normalized score
TYPE Weight       = float64 ∈ [0, 1]    // Connection weight
TYPE Hash         = bytes[32]           // Cryptographic hash
TYPE Signature    = bytes[64]           // Ed25519 signature
```

## 1.2 Connection Structure

```
STRUCTURE Connection {
    partner_id   : NodeId
    w            : Weight              // Connection strength
    r            : float64             // Reciprocity score (unbounded)
    q            : Score               // Quality score
    τ            : SignedScore         // Tone score
    π            : Score               // Priming level
    last_active  : Timestamp
    count        : uint32              // Interaction count
}
```

## 1.3 Node Structure

```
STRUCTURE Node {
    id              : NodeId
    status          : enum { PROBATIONARY, MEMBER, ESTABLISHED, HUB }
    trust           : Score
    trust_cap       : Score
    confidence      : Score
    priming         : Score
    connections     : Map<NodeId, Connection>
    capabilities    : Map<CapabilityId, CapabilityState>
    threat_beliefs  : Map<NodeId, ThreatBelief>
    quality_score   : Score
    flags           : Set<Flag>
}

STRUCTURE ThreatBelief {
    level       : Score
    threat_type : enum { CHEATING, SYBIL, COLLUSION, QUALITY_FRAUD, STRATEGIC }
    evidence    : List<Hash>
    updated     : Timestamp
}

STRUCTURE Capability {
    id           : CapabilityId
    name         : string
    category     : enum { ANALYSIS, GENERATION, TRANSFORMATION, VALIDATION }
    input_types  : List<DataType>
    output_types : List<DataType>
}

STRUCTURE CapabilityState {
    capability   : Capability
    quality      : Score              // Quality score FOR THIS CAPABILITY
    volume       : uint32             // Times used
    last_used    : Timestamp
    available    : boolean
    load         : Score              // Current load (0 = idle, 1 = maxed)
}
```

## 1.4 Interaction Structure

```
STRUCTURE Interaction {
    initiator    : NodeId
    responder    : NodeId
    task_volume  : float64             // Q - amount of work/value
    capability   : CapabilityId        // Which capability was used
    quality      : Score               // Measured quality
    tone         : SignedScore         // Measured tone
    exchange_in  : float64             // Value received
    exchange_out : float64             // Value given
    timestamp    : Timestamp
}

STRUCTURE Feedback {
    helpfulness  : int ∈ [1, 5]
    accuracy     : int ∈ [1, 5]
    relevance    : int ∈ [1, 5]
    timeliness   : int ∈ [1, 5]
    would_reuse  : boolean
}
```

## 1.5 Task and Workflow Structures

```
STRUCTURE Task {
    id              : TaskId
    type            : enum { SINGLE, SEQUENTIAL, PARALLEL, DAG }
    required_caps   : List<CapabilityId>
    input           : TaskInput
    constraints     : TaskConstraints
    context         : TaskContext
    origin          : NodeId
}

STRUCTURE TaskConstraints {
    timeout         : Duration
    priority        : enum { LOW, NORMAL, HIGH, CRITICAL }
    min_trust       : Score
    min_quality     : Score
    preferred_nodes : List<NodeId>
    excluded_nodes  : List<NodeId>
}

STRUCTURE Workflow {
    id          : WorkflowId
    type        : enum { SINGLE, SEQUENTIAL, PARALLEL, DAG }
    steps       : List<WorkflowStep>
    context     : WorkflowContext
    status      : enum { PENDING, RUNNING, COMPLETED, FAILED }
}

STRUCTURE WorkflowStep {
    step_id     : StepId
    task        : Task
    assigned_to : NodeId
    depends_on  : List<StepId>
    status      : StepStatus
    result      : Option<StepResult>
}

STRUCTURE Handoff {
    from_node   : NodeId
    to_node     : NodeId
    task        : Task
    context     : HandoffContext
    timestamp   : Timestamp
    signature   : Signature
}

STRUCTURE HandoffContext {
    workflow_id  : WorkflowId
    step_index   : uint32
    prior_results: List<StepResult>
    accumulated  : Map<string, any>
    lineage      : List<NodeId>
}
```

## 1.6 Defense Structures

```
STRUCTURE DefenseSignal {
    type         : enum { GENERAL_ALERT, SPECIFIC_THREAT, BROADCAST }
    sender       : NodeId
    origin       : NodeId
    threat       : NodeId
    threat_type  : ThreatType
    confidence   : Score
    evidence     : Hash
    hops         : uint8
    timestamp    : Timestamp
    signature    : Signature
}

STRUCTURE Affirmation {
    from         : NodeId
    to           : NodeId
    type         : enum { QUALITY, RELIABILITY, COLLABORATION, GROWTH }
    strength     : Score
    timestamp    : Timestamp
    signature    : Signature
}
```

---

# SECTION 2: CONSTANTS

```
// Connection dynamics
CONST γ (GAMMA)           = 0.1       // Reinforcement rate
CONST μ (MU)              = 0.5       // Flow exponent (sublinear)
CONST α (ALPHA)           = 0.01      // Decay rate
CONST β (BETA)            = 2.0       // Reciprocity sensitivity
CONST λ (LAMBDA)          = 0.9       // Memory factor (EMA)
CONST θ (THETA)           = 0.5       // Quality weight in reciprocity
CONST δ (DELTA)           = 0.2       // Defense dampening factor
CONST ε (EPSILON)         = 0.001     // Small constant for division safety

// Connection bounds
CONST W_MIN               = 0.01
CONST W_MAX               = 1.0
CONST W_INIT              = 0.3

// Cold start
CONST SWIFT_TRUST_BASE    = 0.4
CONST VOUCH_PENALTY       = 0.5
CONST PROBATION_COUNT     = 50
CONST PROBATION_THRESHOLD = 0.6

// Quality weights
CONST ω_HELP              = 0.4
CONST ω_ACC               = 0.3
CONST ω_REL               = 0.2
CONST ω_TIME              = 0.1
CONST REUSE_BOOST         = 1.2
CONST REUSE_PENALTY       = 0.8

// Defense signaling
CONST PROPAGATE_THRESHOLD = 0.6
CONST DECAY_PER_HOP       = 0.8
CONST MIN_SIGNAL          = 0.1
CONST MAX_HOPS            = 5
CONST PRIMING_SENSITIVITY = 0.1
CONST PRIMING_DECAY       = 0.99
CONST ACTION_THRESHOLD    = 0.7

// Confidence
CONST CONFIDENCE_MEMORY   = 0.95

// Detection
CONST COLLUSION_THRESHOLD = 0.85
CONST DIVERSITY_THRESHOLD = 0.3
CONST ADVERSARY_DROP      = 0.3
```

---

# SECTION 3: MATHEMATICAL FUNCTIONS

## 3.1 Sigmoid Functions

**Reciprocity Sigmoid** — Maps unbounded reciprocity to [-1, 1]:
```
σ(r) = (2 / (1 + e^(-β × r))) - 1
```

**Standard Sigmoid** — Maps to [0, 1]:
```
sigmoid(x) = 1 / (1 + e^(-x))
```

## 3.2 Multiplier Functions

**Quality Multiplier** — Maps quality [0,1] to [0.5, 1.5]:
```
ψ(q) = 0.5 + q
```

**Tone Multiplier** — Maps tone [-1,1] to [0.4, 1.0]:
```
φ(τ) = 0.7 + (0.3 × τ)
```

## 3.3 Core Dynamics Equation

**Connection Weight Update (Physarum Equation):**
```
dw/dt = Φ(Q, r, q, τ) - α×w - D

WHERE:
    Φ = γ × |Q|^μ × σ(r) × ψ(q) × φ(τ)
    D = δ × threat_level    (if defense signal active, else 0)
```

**Discrete Form:**
```
w_new = clamp(w + Δt × (Φ - α×w - D), W_MIN, W_MAX)
```

## 3.4 Reciprocity Update

```
r_new = λ × r + (1 - λ) × (log(ρ + ε) + θ × (q - 0.5))

WHERE:
    ρ = exchange_in / (exchange_out + ε)    // Exchange ratio
```

## 3.5 Quality Score

**From Feedback:**
```
Q_raw = (ω_HELP × helpfulness + ω_ACC × accuracy + 
         ω_REL × relevance + ω_TIME × timeliness) / 4

Q_multiplied = Q_raw × (REUSE_BOOST if would_reuse else REUSE_PENALTY)

Q_normalized = (Q_multiplied - 1) / 4    // Maps to [0, 1]
```

**Aggregated Quality:**
```
Q_agg(n) = Σ(Q_i × T_giver_i × decay(age_i)) / Σ(T_giver_i × decay(age_i))
```

## 3.6 Tone Score

```
τ = tanh(ω_e × E + ω_f × F + ω_c × C)

WHERE:
    E = 0.4 × latency_score + 0.4 × elaboration + 0.2 × questions
    F = 0.5 × (affirmative - 0.5) + 0.3 × (0.5 - hedging) + 0.2 × acknowledgment
    C = 0.5 × alternatives + 0.3 × build_on + 0.2 × credit_giving
```

## 3.7 Trust Computation

**Global Trust:**
```
T(n) = (w_Q × Q_agg + w_R × σ(R_agg) + w_S × S_social + w_D × D_diversity) / Σw

WHERE:
    R_agg = mean(r) across all connections
    D_diversity = unique_partners_last_100 / 100
```

**Diversity Cap:**
```
T_final = min(T(n), D_diversity + 0.3)
```

## 3.8 Self-Confidence

```
S_conf_new = α_conf × S_conf + (1 - α_conf) × A_mean

WHERE:
    A_mean = Σ(T_affirmer × strength) / Σ(T_affirmer)
    α_conf = CONFIDENCE_MEMORY (0.95)
```

## 3.9 Defense Signal Attenuation

```
confidence_at_hop_n = confidence_origin × (DECAY_PER_HOP ^ n) × w_connection
```

## 3.10 Threat Belief Update (Bayesian)

```
belief_new = belief_old + weight × (1 - belief_old)

WHERE:
    weight = T_sender × signal_confidence
```

## 3.11 Convergence Score

```
Conv(task) = 1 - (Var(positions) / Var_max)
```

---


# SECTION 4: THE MASTER ALGORITHM

## 4.1 System Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                  SYMBIONT NODE RUNTIME                          │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐          │
│   │    TASK     │   │   EVENT     │   │ BACKGROUND  │          │
│   │  PROCESSOR  │   │  HANDLERS   │   │    JOBS     │          │
│   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘          │
│          │                 │                 │                  │
│          └────────────────┬┴─────────────────┘                  │
│                           │                                     │
│                           ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐  │
│   │                      NODE STATE                          │  │
│   │  ┌────────────┐ ┌────────────┐ ┌────────────┐           │  │
│   │  │   Trust    │ │Connections │ │Capabilities│           │  │
│   │  │    T(n)    │ │ {w,r,q,τ}  │ │ {cap,q,load}│          │  │
│   │  └────────────┘ └────────────┘ └────────────┘           │  │
│   │  ┌────────────┐ ┌────────────┐ ┌────────────┐           │  │
│   │  │  Priming   │ │ Confidence │ │  Threats   │           │  │
│   │  │     π      │ │   S_conf   │ │  beliefs   │           │  │
│   │  └────────────┘ └────────────┘ └────────────┘           │  │
│   └─────────────────────────────────────────────────────────┘  │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘

Four concurrent processes:
1. TASK PROCESSOR    — Route tasks, execute, orchestrate workflows
2. INTERACTION LOOP  — Handle interactions, update connections
3. EVENT HANDLERS    — React to signals, affirmations, handoffs
4. BACKGROUND JOBS   — Periodic decay, scans, status checks
```

## 4.2 Node Initialization

```
FUNCTION initialize_node(node_id, role, capabilities, vouchers):
    
    // Compute initial trust (Swift Trust)
    T_swift    = SWIFT_TRUST_BASE                              // 0.4
    T_category = lookup_role_trust(role) × similarity_score()
    T_vouch    = Σ(voucher.trust × voucher.stake × time_decay(voucher.age))
    T_social   = sigmoid(interaction_quality_sum / network_average)
    
    T_init = 0.3 × T_swift + 0.2 × T_category + 0.3 × T_vouch + 0.2 × T_social
    
    // Initialize capability states
    cap_states = {}
    FOR EACH cap IN capabilities:
        cap_states[cap.id] = CapabilityState {
            capability = cap
            quality    = 0.5           // Start neutral
            volume     = 0
            available  = true
            load       = 0.0
        }
    
    RETURN Node {
        id           = node_id
        status       = PROBATIONARY
        trust        = T_init
        trust_cap    = 1.0
        confidence   = 0.5
        priming      = 0.0
        connections  = {}
        capabilities = cap_states
        // ... other fields initialized to defaults
    }
```

## 4.3 Task Processor (Orchestration Entry Point)

```
FUNCTION process_task(self, task, network):
    
    SWITCH task.type:
        CASE SINGLE:     RETURN process_single_task(self, task, network)
        CASE SEQUENTIAL: RETURN process_workflow(self, task.workflow, network)
        CASE PARALLEL:   RETURN process_parallel(self, task, network)
        CASE DAG:        RETURN process_dag_workflow(self, task.workflow, network)


FUNCTION process_single_task(self, task, network):
    
    required_cap = task.required_caps[0]
    
    // Can we handle it ourselves?
    IF self.has_capability(required_cap) AND self.load < 0.9:
        result = self.execute(task)
        RETURN TaskResult.LOCAL(result)
    
    // Route to best candidate
    candidates = find_candidates(task, network)
    IF candidates is empty:
        RETURN TaskResult.NO_CANDIDATES
    
    // Score each candidate
    scored = []
    FOR EACH node IN candidates:
        score = compute_routing_score(self, node, task)
        scored.append((node, score))
    
    scored.sort(BY score DESC)
    target = scored[0].node
    
    // Execute via handoff (triggers interaction tracking)
    handoff = perform_handoff(self, target, task, task.context)
    result = await_result(handoff)
    
    RETURN TaskResult.ROUTED(target, result)


FUNCTION compute_routing_score(from_node, candidate, task):
    
    required_cap = task.required_caps[0]
    
    // Base: Trust × Capability Quality × Availability
    score = T(candidate) × q_cap(candidate, required_cap) × (1 - candidate.load)
    
    // Connection factor
    IF candidate.id IN from_node.connections:
        w = from_node.connections[candidate.id].w
    ELSE:
        w = W_INIT
    score = score × w
    
    // Defense penalty
    IF candidate.id IN from_node.threat_beliefs:
        threat = from_node.threat_beliefs[candidate.id].level
        score = score × (1 - threat)
    
    // Preference bonus
    IF candidate.id IN task.constraints.preferred_nodes:
        score = score × 1.2
    
    RETURN score
```

## 4.4 Main Interaction Handler

```
FUNCTION handle_interaction(self, partner, interaction):
    
    // STEP 1: Get or create connection
    IF partner.id NOT IN self.connections:
        self.connections[partner.id] = new_connection(partner.id)
    conn = self.connections[partner.id]
    
    // STEP 2: Measure outcomes
    quality = compute_quality(interaction.feedback)
    tone    = compute_tone(interaction)
    ρ       = interaction.exchange_in / (interaction.exchange_out + ε)
    
    // STEP 3: Update reciprocity
    log_rho     = log(ρ + ε)
    quality_adj = θ × (quality - 0.5)
    conn.r      = λ × conn.r + (1 - λ) × (log_rho + quality_adj)
    
    // STEP 4: Update quality (global AND per-capability)
    conn.q = λ × conn.q + (1 - λ) × quality
    IF interaction.capability IS NOT NULL:
        cap_state = partner.capabilities[interaction.capability]
        cap_state.quality = λ × cap_state.quality + (1 - λ) × quality
        cap_state.volume += 1
    
    // STEP 5: Update tone
    conn.τ = λ × conn.τ + (1 - λ) × tone
    
    // STEP 6: Compute reinforcement (THE CORE EQUATION)
    Q   = interaction.task_volume
    σ_r = (2 / (1 + exp(-β × conn.r))) - 1
    ψ_q = 0.5 + conn.q
    φ_τ = 0.7 + 0.3 × conn.τ
    Φ   = γ × pow(Q, μ) × σ_r × ψ_q × φ_τ
    
    // STEP 7: Defense dampening
    D = 0
    IF partner.id IN self.threat_beliefs:
        D = δ × self.threat_beliefs[partner.id].level
    
    // STEP 8: Update connection weight
    Δw     = Φ - α × conn.w - D
    conn.w = clamp(conn.w + Δw, W_MIN, W_MAX)
    
    // STEP 9: Post-interaction actions
    IF quality > 0.8 AND tone > 0.5:
        send_affirmation(self, partner, quality)
    IF should_warn(partner, conn):
        emit_defense_signal(self, partner, conn)
    
    conn.last_active = now()
    conn.count += 1
    RETURN success
```

## 4.5 Workflow Executor

```
FUNCTION process_workflow(self, workflow, network):
    
    context = workflow.context
    current_node = self
    
    FOR EACH step IN workflow.steps:
        
        // Route this step from current node's perspective
        task = step.task
        scored = []
        FOR EACH candidate IN find_candidates(task, network):
            score = compute_routing_score(current_node, candidate, task)
            scored.append((candidate, score))
        
        IF scored is empty:
            workflow.status = FAILED
            RETURN WorkflowResult.FAILED(step, "No candidates")
        
        scored.sort(BY score DESC)
        target = scored[0].node
        step.assigned_to = target.id
        
        // Handoff to target
        handoff = Handoff {
            from_node = current_node.id
            to_node   = target.id
            task      = task
            context   = context
            timestamp = now()
            signature = current_node.sign()
        }
        
        // Target executes (triggers normal interaction tracking)
        result = target.execute_and_respond(handoff)
        
        IF result.failed:
            workflow.status = FAILED
            RETURN WorkflowResult.FAILED(step, result.error)
        
        step.status = COMPLETED
        step.result = result
        
        // Update context, continue chain from target's perspective
        context = merge_context(context, result.output)
        context.lineage.append(target.id)
        current_node = target
    
    workflow.status = COMPLETED
    RETURN WorkflowResult.SUCCESS(context)
```

## 4.6 Event Handlers

```
FUNCTION handle_event(self, event):
    
    SWITCH event.type:
        CASE DEFENSE_SIGNAL:  handle_defense_signal(self, event.signal)
        CASE AFFIRMATION:     handle_affirmation(self, event.affirmation)
        CASE HANDOFF:         handle_incoming_handoff(self, event.handoff)
        CASE WORKFLOW_UPDATE: handle_workflow_update(self, event.update)


FUNCTION handle_defense_signal(self, signal):
    
    // Update threat belief (Bayesian)
    current      = self.threat_beliefs[signal.threat] OR default_belief()
    sender_trust = get_trust(signal.sender)
    weight       = sender_trust × signal.confidence
    new_level    = current.level + weight × (1 - current.level)
    
    self.threat_beliefs[signal.threat] = {
        level       = new_level
        threat_type = signal.threat_type
        evidence    = current.evidence + [signal.evidence]
        updated     = now()
    }
    
    // Increase priming
    boost = signal.confidence × PRIMING_SENSITIVITY
    self.priming = min(1.0, self.priming + boost)
    
    // Maybe take action
    IF new_level > ACTION_THRESHOLD:
        take_defensive_action(self, signal.threat)
    
    // Maybe propagate
    IF signal.confidence > PROPAGATE_THRESHOLD AND signal.hops < MAX_HOPS:
        propagate_defense_signal(self, signal)


FUNCTION handle_incoming_handoff(self, handoff):
    
    // Validate
    IF NOT verify(handoff.signature, handoff.from_node):
        RETURN HandoffResult.INVALID
    
    // Check capability and availability
    required_cap = handoff.task.required_caps[0]
    IF NOT self.has_capability(required_cap):
        RETURN HandoffResult.MISSING_CAPABILITY
    IF self.capabilities[required_cap].load > 0.95:
        RETURN HandoffResult.OVERLOADED
    
    // Update load, execute, update load
    self.capabilities[required_cap].load += task_load_estimate(handoff.task)
    result = self.execute(handoff.task, handoff.context)
    self.capabilities[required_cap].load -= task_load_estimate(handoff.task)
    
    // Send result (triggers interaction tracking)
    send_result(handoff.from_node, result)
    RETURN HandoffResult.SUCCESS(result)
```

## 4.7 Background Jobs

```
FUNCTION background_tick(self):
    
    // JOB 1: Decay priming
    self.priming = self.priming × PRIMING_DECAY
    
    // JOB 2: Decay unused connections
    FOR EACH conn IN self.connections:
        IF (now() - conn.last_active) > IDLE_THRESHOLD:
            conn.w = conn.w × (1 - α)
            IF conn.w < W_MIN:
                remove_connection(conn)
    
    // JOB 3: Update capability availability
    FOR EACH cap_state IN self.capabilities:
        cap_state.load = cap_state.load × 0.95
        cap_state.available = (cap_state.load < 0.9)
    
    // JOB 4: Check diversity (periodic)
    IF tick_count MOD DIVERSITY_INTERVAL == 0:
        unique = count_unique_partners(last_100_interactions)
        diversity = unique / 100
        IF diversity < DIVERSITY_THRESHOLD:
            self.trust_cap = 0.7
            self.flags.add(LOW_DIVERSITY)
        ELSE:
            self.trust_cap = 1.0
            self.flags.remove(LOW_DIVERSITY)
    
    // JOB 5: Check status transitions (periodic)
    IF tick_count MOD STATUS_INTERVAL == 0:
        check_status_transition(self)
    
    // JOB 6: Scan for adversaries (periodic)
    IF tick_count MOD ADVERSARY_INTERVAL == 0:
        FOR EACH partner IN self.connections:
            IF detect_strategic_adversary(partner):
                emit_defense_signal(self, partner, STRATEGIC)
```

## 4.8 Trust Computation

```
FUNCTION compute_trust(node):
    
    // Aggregate reciprocity
    R_agg = mean([conn.r FOR conn IN node.connections]) IF connections ELSE 0
    
    // Aggregate capability quality (weighted by usage)
    Q_agg = weighted_mean(
        [cap.quality FOR cap IN node.capabilities],
        [cap.volume FOR cap IN node.capabilities]
    )
    
    // Diversity and social proof
    D = count_unique_partners(node.last_100_interactions) / 100
    S = compute_social_proof(node)
    
    // Weighted sum
    T = 0.4 × Q_agg + 0.2 × sigmoid(R_agg) + 0.2 × S + 0.2 × D
    
    // Apply caps
    T = min(T, D + 0.3)          // Diversity cap
    T = min(T, node.trust_cap)   // Trust cap
    
    RETURN T
```

## 4.9 Adversary Detection

```
FUNCTION detect_strategic_adversary(node):
    
    history = node.interaction_history[last 100]
    IF length(history) < 100:
        RETURN false
    
    early  = history[0:50]
    recent = history[50:100]
    
    early_quality  = mean([i.quality FOR i IN early])
    recent_quality = mean([i.quality FOR i IN recent])
    early_variance = variance([i.quality FOR i IN early])
    
    // Suspiciously perfect early behavior
    IF early_quality > 0.95 AND early_variance < 0.01:
        RETURN true
    
    // Quality drop after building trust
    IF node.trust > 0.7 AND (early_quality - recent_quality) > ADVERSARY_DROP:
        RETURN true
    
    RETURN false


FUNCTION detect_collusion(network):
    
    graph = build_interaction_graph(network)
    communities = find_communities(graph)
    suspicious = []
    
    FOR EACH community IN communities:
        IF size(community) < 3:
            CONTINUE
        
        internal_density = edges_within(community) / max_possible_edges(community)
        external_ratio   = edges_outside(community) / (size(community) × 0.5)
        mutual_rating    = mean_internal_rating(community)
        
        IF internal_density > 0.8 AND external_ratio < 1.0 AND mutual_rating > 0.9:
            suspicious.add(community)
    
    RETURN suspicious
```

## 4.10 Complete Flow Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           SYMBIONT COMPLETE FLOW                            │
└─────────────────────────────────────────────────────────────────────────────┘

                              TASK ARRIVES
                                   │
                                   ▼
                        ┌─────────────────────┐
                        │  Can I do this?     │
                        │  (check capability) │
                        └──────────┬──────────┘
                                   │
                    ┌──────────────┴──────────────┐
                    │ YES                         │ NO
                    ▼                             ▼
           ┌───────────────┐            ┌───────────────────┐
           │ Execute local │            │ ROUTE TO BEST     │
           └───────────────┘            │                   │
                                        │ Score = T × q_cap │
                                        │   × (1-load) × w  │
                                        │   × (1-threat)    │
                                        └─────────┬─────────┘
                                                  │
                                                  ▼
                                        ┌───────────────────┐
                                        │ HANDOFF           │
                                        │ (task + context)  │
                                        └─────────┬─────────┘
                                                  │
                                                  ▼
                                        ┌───────────────────┐
                                        │ TARGET EXECUTES   │
                                        └─────────┬─────────┘
                                                  │
                                                  ▼
                              ┌────────────────────────────────┐
                              │      INTERACTION RECORDED       │
                              │                                │
                              │  1. Measure quality, tone, ρ   │
                              │  2. Update r (reciprocity)     │
                              │  3. Update q (quality)         │
                              │  4. Update q_cap (per-cap)     │
                              │  5. Update τ (tone)            │
                              │  6. Compute Φ (reinforcement)  │
                              │  7. Compute D (defense damp)   │
                              │  8. Update w = w + (Φ - αw - D)│
                              │  9. Maybe affirmation          │
                              │ 10. Maybe defense signal       │
                              └────────────────┬───────────────┘
                                               │
                              ┌────────────────┴───────────────┐
                              │                                │
                              ▼                                ▼
                     ┌───────────────┐               ┌───────────────┐
                     │ If workflow:  │               │ Update global │
                     │ Next step     │               │ T(n) trust    │
                     │ routes from   │               │               │
                     │ target's POV  │               │ Route future  │
                     └───────────────┘               │ tasks based   │
                                                     │ on new scores │
                                                     └───────────────┘
```

---
# SECTION 5: STATE TRANSITIONS

## 5.1 Node Status State Machine

```
STATES: { PROBATIONARY, MEMBER, ESTABLISHED, HUB, EXPELLED }

TRANSITIONS:

    PROBATIONARY → MEMBER:
        CONDITION: interaction_count >= 50 AND mean_quality >= 0.6
        ACTION:    trust = min(trust × 1.5, 0.8)
    
    PROBATIONARY → PROBATIONARY (extend):
        CONDITION: interaction_count >= 50 AND mean_quality < 0.6
        ACTION:    trust = trust × 0.8; required_count += 25
    
    PROBATIONARY → EXPELLED:
        CONDITION: failed_probation_3_times OR detected_as_adversary
        ACTION:    notify_vouchers(); blacklist()
    
    MEMBER → ESTABLISHED:
        CONDITION: interaction_count >= 200 AND mean_quality >= 0.8
        ACTION:    (none)
    
    ESTABLISHED → HUB:
        CONDITION: connection_count >= 20 AND diversity >= 0.5 AND quality >= 0.85
        ACTION:    (none)
    
    ANY → EXPELLED:
        CONDITION: threat_belief_level >= 0.9 from multiple sources
        ACTION:    notify_network(); blacklist()
    
    {MEMBER, ESTABLISHED, HUB} → PROBATIONARY (demotion):
        CONDITION: quality_drop > 0.4 OR trust < 0.3
        ACTION:    trust = trust × 0.5; reset_interaction_count()
```

## 5.2 Defense State Machine (Parallel)

```
STATES: { NORMAL, PRIMED, DEFENDING }

TRANSITIONS:

    NORMAL → PRIMED:
        CONDITION: received_defense_signal
        ACTION:    priming += signal.confidence × PRIMING_SENSITIVITY
    
    PRIMED → DEFENDING:
        CONDITION: threat_belief > ACTION_THRESHOLD
        ACTION:    take_defensive_action()
    
    PRIMED → NORMAL:
        CONDITION: priming < 0.1 (decayed)
        ACTION:    (none)
    
    DEFENDING → PRIMED:
        CONDITION: threat_resolved
        ACTION:    priming = 0.3
    
    DEFENDING → NORMAL:
        CONDITION: threat_resolved AND priming < 0.1
        ACTION:    (none)
```

---

# SECTION 6: CONVERGENCE PROTOCOL

## 6.1 Convergence Tracking

```
FUNCTION track_convergence(task, positions):
    
    // Compute pairwise distances
    distances = []
    nodes = keys(positions)
    
    FOR i FROM 0 TO length(nodes) - 1:
        FOR j FROM i + 1 TO length(nodes) - 1:
            d = distance(positions[nodes[i]], positions[nodes[j]])
            distances.append(d)
    
    IF length(distances) == 0:
        RETURN 1.0  // Single participant = converged
    
    variance = var(distances)
    max_var  = compute_max_variance(task)
    
    convergence = 1 - (variance / max_var)
    
    RETURN convergence


FUNCTION get_convergence_state(score):
    IF score > 0.85: RETURN CONVERGED
    IF score > 0.60: RETURN CONVERGING
    IF score > 0.40: RETURN EXPLORING
    IF score > 0.20: RETURN STUCK
    RETURN POLARIZED
```

## 6.2 Agree-to-Disagree

```
FUNCTION maybe_agree_to_disagree(tracker):
    
    state = get_convergence_state(tracker.current_score)
    rounds = tracker.round_count
    
    // Conditions for invoking ATD
    should_invoke = (
        rounds >= 5 AND
        state IN {STUCK, POLARIZED} AND
        tracker.trend == STABLE AND
        tracker.task.criticality < 0.8
    )
    
    IF NOT should_invoke:
        RETURN null
    
    // Execute ATD
    positions = tracker.current_positions
    clusters  = cluster_positions(positions)
    
    // Choose winning path
    IF length(clusters) == 2 AND sizes_roughly_equal(clusters):
        winning = tiebreaker_by_trust_weight(clusters)
    ELSE:
        winning = largest_cluster(clusters)
    
    // Record dissent (not penalty)
    FOR EACH (node, position) IN positions:
        IF position NOT IN winning.positions:
            record_dissent(node, position, winning)
    
    RETURN ATDResult {
        chosen_path = winning
        dissenting  = nodes_not_in_winning
    }
```

---

# SECTION 7: IMPLEMENTATION NOTES

## 7.1 Language-Specific Considerations

| Aspect | Consideration |
|--------|---------------|
| **Floating point** | Use 64-bit floats; watch for NaN in division |
| **Concurrency** | Three processes can be threads, async tasks, or actors |
| **Map/Dictionary** | Any associative container works |
| **Cryptography** | Use Ed25519 for signatures, BLAKE3 or SHA-256 for hashes |
| **Networking** | Adapt to your P2P layer (Veilid, libp2p, custom) |
| **Serialization** | Protocol Buffers, MessagePack, or CBOR recommended |

## 7.2 Numerical Stability

```
// Safe sigmoid
FUNCTION safe_sigmoid(r, β):
    x = -β × r
    IF x > 700:    RETURN -1.0    // Prevent overflow
    IF x < -700:   RETURN 1.0
    RETURN (2 / (1 + exp(x))) - 1

// Safe log
FUNCTION safe_log(x):
    RETURN log(max(x, ε))

// Safe division
FUNCTION safe_div(a, b):
    RETURN a / (b + ε)
```

## 7.3 Minimum Viable Implementation

For a minimal Symbiont implementation, you need:

1. **Data structures**: Node, Connection, Interaction
2. **Core equation**: `w_new = w + (Φ - αw) where Φ = γ|Q|^μ × σ(r) × ψ(q)`
3. **Reciprocity update**: `r_new = λr + (1-λ)log(ρ)`
4. **Quality from feedback**: weighted sum of dimensions
5. **Trust computation**: weighted sum of quality, reciprocity, diversity

Everything else (tone, defense signals, convergence, etc.) can be added incrementally.

---

# SECTION 8: ORCHESTRATION

## 8.1 Core Principle

**Orchestration emerges from trust dynamics.** There is no separate "orchestrator" — routing, selection, and workflow decisions are computed from the same equations that govern trust.

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   Traditional: Orchestrator decides → Agents execute            │
│                                                                 │
│   Symbiont:    Trust dynamics determine → Best path emerges     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## 8.2 Capability Model

### 9.2.1 Capability Declaration

Each node declares its capabilities:

```
STRUCTURE Capability {
    id           : CapabilityId
    name         : string
    category     : enum { ANALYSIS, GENERATION, TRANSFORMATION, VALIDATION, ... }
    input_types  : List<DataType>
    output_types : List<DataType>
    constraints  : Map<string, any>    // e.g., max_size, timeout, etc.
}

STRUCTURE Node {
    // ... existing fields ...
    capabilities : Map<CapabilityId, CapabilityState>
}

STRUCTURE CapabilityState {
    capability   : Capability
    quality      : Score              // Quality score FOR THIS CAPABILITY
    volume       : uint32             // Times used
    last_used    : Timestamp
    available    : boolean            // Currently available?
    load         : Score              // Current load (0 = idle, 1 = maxed)
}
```

### 9.2.2 Per-Capability Quality Tracking

Quality scores are tracked **per capability**, not just globally:

```
q_capability(node, cap) = aggregated quality for node performing capability cap

Q_agg(node) = weighted average across all capabilities
```

This allows: "Node A is great at analysis but mediocre at generation."

## 8.3 Task Routing

### 9.3.1 Task Structure

```
STRUCTURE Task {
    id              : TaskId
    type            : enum { SINGLE, SEQUENTIAL, PARALLEL, DAG }
    required_caps   : List<CapabilityId>        // What capabilities needed
    input           : TaskInput
    constraints     : TaskConstraints           // Timeout, priority, etc.
    context         : TaskContext               // Carried through workflow
    origin          : NodeId                    // Who initiated
}

STRUCTURE TaskConstraints {
    timeout         : Duration
    priority        : enum { LOW, NORMAL, HIGH, CRITICAL }
    min_trust       : Score                     // Minimum trust required
    min_quality     : Score                     // Minimum quality required
    preferred_nodes : List<NodeId>              // Hints (not requirements)
    excluded_nodes  : List<NodeId>              // Hard exclusions
}
```

### 9.3.2 Routing Score

For a task requiring capability `cap`, the routing score for candidate node `n` is:

$$S_{route}(n, cap, task) = T(n) \cdot q_{cap}(n) \cdot (1 - load(n)) \cdot \phi_{conn} \cdot (1 - \theta_{threat})$$

Where:

| Component | Formula | Meaning |
|-----------|---------|---------|
| $T(n)$ | Global trust | Overall trustworthiness |
| $q_{cap}(n)$ | Capability quality | How good at THIS task type |
| $(1 - load(n))$ | Availability | Not overloaded |
| $\phi_{conn}$ | Connection factor | $w_{ij}$ if existing connection, else $W_{INIT}$ |
| $(1 - \theta_{threat})$ | Safety factor | Reduced if defense signals active |

### 9.3.3 Routing Algorithm

```
FUNCTION route_task(task, from_node, network):
    
    required_cap = task.required_caps[0]    // For single-cap tasks
    
    // 1. Find candidates with capability
    candidates = []
    FOR EACH node IN network.nodes:
        IF node.has_capability(required_cap):
            IF node.capabilities[required_cap].available:
                IF node.id NOT IN task.constraints.excluded_nodes:
                    candidates.append(node)
    
    // 2. Filter by constraints
    candidates = filter(candidates, WHERE:
        T(node) >= task.constraints.min_trust AND
        q_cap(node, required_cap) >= task.constraints.min_quality
    )
    
    // 3. Score candidates
    scored = []
    FOR EACH node IN candidates:
        
        // Base score
        score = T(node) * q_cap(node, required_cap) * (1 - node.load)
        
        // Connection bonus (prefer existing relationships)
        IF node.id IN from_node.connections:
            conn = from_node.connections[node.id]
            score = score * conn.w
        ELSE:
            score = score * W_INIT
        
        // Defense penalty
        IF node.id IN from_node.threat_beliefs:
            threat = from_node.threat_beliefs[node.id].level
            score = score * (1 - threat)
        
        // Preference bonus
        IF node.id IN task.constraints.preferred_nodes:
            score = score * 1.2
        
        scored.append((node, score))
    
    // 4. Select best (or top-k for redundancy)
    scored.sort(BY score DESC)
    
    IF scored is empty:
        RETURN RoutingResult.NO_CANDIDATES
    
    RETURN RoutingResult.SUCCESS(scored[0].node)
```

## 8.4 Workflow Execution

### 9.4.1 Workflow Types

```
ENUM WorkflowType {
    SINGLE      // One task, one agent
    SEQUENTIAL  // A → B → C (chain)
    PARALLEL    // A, B, C simultaneously, then merge
    DAG         // Directed acyclic graph of dependencies
}

STRUCTURE Workflow {
    id          : WorkflowId
    type        : WorkflowType
    steps       : List<WorkflowStep>
    context     : WorkflowContext       // Accumulated state
    status      : enum { PENDING, RUNNING, COMPLETED, FAILED }
}

STRUCTURE WorkflowStep {
    step_id     : StepId
    task        : Task
    assigned_to : NodeId                // Filled during execution
    depends_on  : List<StepId>          // For DAG workflows
    status      : StepStatus
    result      : Option<StepResult>
}
```

### 9.4.2 Sequential Workflow Execution

```
FUNCTION execute_sequential_workflow(workflow, initiator, network):
    
    context = workflow.context
    current_node = initiator
    
    FOR EACH step IN workflow.steps:
        
        // 1. Route to best candidate
        routing = route_task(step.task, current_node, network)
        
        IF routing.failed:
            workflow.status = FAILED
            RETURN WorkflowResult.FAILED(step, routing.reason)
        
        target = routing.node
        step.assigned_to = target.id
        
        // 2. Execute task (this triggers normal interaction)
        result = execute_and_track(current_node, target, step.task, context)
        
        // 3. Handle result
        IF result.failed:
            workflow.status = FAILED
            RETURN WorkflowResult.FAILED(step, result.error)
        
        step.status = COMPLETED
        step.result = result
        
        // 4. Update context with output
        context = merge_context(context, result.output)
        
        // 5. Handoff: next step starts from this node
        current_node = target
    
    workflow.status = COMPLETED
    RETURN WorkflowResult.SUCCESS(context)
```

### 9.4.3 Parallel Workflow Execution

```
FUNCTION execute_parallel_workflow(workflow, initiator, network):
    
    // 1. Route all parallel tasks
    assignments = []
    FOR EACH step IN workflow.steps WHERE step.depends_on is empty:
        routing = route_task(step.task, initiator, network)
        IF routing.failed:
            RETURN WorkflowResult.FAILED(step, routing.reason)
        assignments.append((step, routing.node))
    
    // 2. Execute in parallel
    results = parallel_execute(assignments)
    
    // 3. Aggregate results (e.g., ensemble voting, merge, etc.)
    aggregated = aggregate_results(results, workflow.aggregation_strategy)
    
    // 4. If there are downstream steps, continue
    IF workflow has merge_step:
        context = merge_context(workflow.context, aggregated)
        // Continue with dependent steps...
    
    RETURN WorkflowResult.SUCCESS(aggregated)
```

## 8.5 Handoff Protocol

When work passes from node A to node B:

### 9.5.1 Handoff Structure

```
STRUCTURE Handoff {
    from_node    : NodeId
    to_node      : NodeId
    task         : Task
    context      : HandoffContext
    timestamp    : Timestamp
    signature    : Signature           // Signed by from_node
}

STRUCTURE HandoffContext {
    workflow_id  : WorkflowId
    step_index   : uint32
    prior_results: List<StepResult>    // Results from prior steps
    accumulated  : Map<string, any>    // Key-value context
    lineage      : List<NodeId>        // Who touched this workflow
}
```

### 9.5.2 Handoff Execution

```
FUNCTION perform_handoff(from_node, to_node, task, context):
    
    // 1. Create handoff record
    handoff = Handoff {
        from_node = from_node.id
        to_node = to_node.id
        task = task
        context = context
        timestamp = now()
        signature = from_node.sign(hash(task, context))
    }
    
    // 2. Update connection (this IS an interaction)
    conn = from_node.connections[to_node.id]
    conn.last_active = now()
    
    // 3. Send to target
    send(to_node, handoff)
    
    // 4. Target validates and executes
    // (handled by to_node.receive_handoff)
    
    RETURN handoff


FUNCTION receive_handoff(self, handoff):
    
    // 1. Validate signature
    IF NOT verify(handoff.signature, handoff.from_node):
        RETURN HandoffResult.INVALID_SIGNATURE
    
    // 2. Check capability
    required_cap = handoff.task.required_caps[0]
    IF NOT self.has_capability(required_cap):
        RETURN HandoffResult.MISSING_CAPABILITY
    
    // 3. Check load
    IF self.load > 0.95:
        RETURN HandoffResult.OVERLOADED
    
    // 4. Execute task
    result = self.execute(handoff.task, handoff.context)
    
    // 5. After completion, interaction tracking happens normally
    // (quality feedback, reciprocity update, etc.)
    
    RETURN HandoffResult.SUCCESS(result)
```

## 8.6 Load Balancing via Reciprocity

**Key insight:** Load balancing is AUTOMATIC via reciprocity dynamics.

When a node is overloaded:
1. It's giving more than receiving → $\rho < 1$
2. Reciprocity score drops → $r$ goes negative
3. Connection reinforcement drops → $\sigma(r) < 0$
4. Routing score drops → tasks route elsewhere

```
// No separate load balancer needed — reciprocity IS load balancing

IF node.is_overloaded:
    // Incoming reciprocity ratios are low (giving a lot, receiving little)
    // This naturally reduces routing scores to this node
    
    // Additionally, node can signal:
    node.capabilities[cap].available = false  // Temporary unavailability
    node.load = current_task_count / max_capacity
```

## 8.7 Ensemble Decisions

When multiple agents work on the same task (for redundancy or consensus):

### 9.7.1 Ensemble Routing

```
FUNCTION route_ensemble(task, from_node, network, k):
    
    // Get top-k candidates instead of just best
    scored = score_all_candidates(task, from_node, network)
    scored.sort(BY score DESC)
    
    top_k = scored[0:k]
    
    // Ensure diversity (not all from same cluster)
    IF too_similar(top_k):
        top_k = diversify(scored, k)
    
    RETURN top_k
```

### 9.7.2 Ensemble Aggregation

```
FUNCTION aggregate_ensemble(results, nodes):
    
    strategy = determine_strategy(task.type)
    
    SWITCH strategy:
        
        CASE VOTING:
            // Weight votes by trust
            votes = {}
            FOR EACH (node, result) IN zip(nodes, results):
                weight = T(node) * q_cap(node, task.cap)
                votes[result.answer] += weight
            RETURN max_vote(votes)
        
        CASE AVERAGING:
            // Weighted average of numeric results
            weights = [T(n) * q_cap(n, task.cap) FOR n IN nodes]
            RETURN weighted_average(results, weights)
        
        CASE CONSENSUS:
            // Use convergence dynamics (Section 6)
            RETURN run_convergence(results, nodes)
        
        CASE BEST_EFFORT:
            // Take result from highest-scored node
            best = max(nodes, BY T(n) * q_cap(n, task.cap))
            RETURN results[index(best)]
```

## 8.8 Predefined Workflow Patterns

Common patterns that can be instantiated:

```
PATTERN chain(caps: List<Capability>):
    // Sequential: A → B → C
    RETURN Workflow {
        type = SEQUENTIAL
        steps = [WorkflowStep(cap) FOR cap IN caps]
    }

PATTERN fan_out_fan_in(parallel_cap: Capability, k: int, merge_cap: Capability):
    // Parallel execution then merge
    RETURN Workflow {
        type = DAG
        steps = [
            ...k WorkflowSteps for parallel_cap,
            WorkflowStep(merge_cap, depends_on=parallel_steps)
        ]
    }

PATTERN pipeline_with_validation(main_cap: Capability, validate_cap: Capability):
    // Do work, then validate
    RETURN Workflow {
        type = SEQUENTIAL
        steps = [
            WorkflowStep(main_cap),
            WorkflowStep(validate_cap)
        ]
    }

// Example instantiation for Athena:
WORKFLOW threat_discovery = chain([
    CAPABILITY.MALWARE_ANALYSIS,      // Doru
    CAPABILITY.THREAT_CORRELATION,    // Aegis  
    CAPABILITY.THREAT_MODELING,       // Weaver
    CAPABILITY.SECURITY_TESTING       // Owl
])
```

## 8.9 Orchestration Integration Summary

| Orchestration Function | Symbiont Component | Equation/Logic |
|------------------------|-------------------|----------------|
| **Who can do this?** | Capability registry | Node.capabilities |
| **Who's best?** | Per-capability quality | $q_{cap}(n)$ |
| **Who to route to?** | Routing score | $S_{route} = T \cdot q_{cap} \cdot (1-load) \cdot w \cdot (1-\theta)$ |
| **Who to avoid?** | Defense signals | $\theta_{threat}$ from threat beliefs |
| **Load balancing?** | Reciprocity dynamics | Overloaded → $r < 0$ → lower routing score |
| **Handoff trust?** | Connection weight | Use existing $w_{ij}$ |
| **Ensemble weighting?** | Trust × quality | $T(n) \cdot q_{cap}(n)$ |
| **Workflow paths?** | Connection graph | Follow strong connections |

---

# SECTION 9: QUICK REFERENCE

## 9.1 The Core Equation

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│    dw/dt = γ × |Q|^μ × σ(r) × ψ(q) × φ(τ) - α×w - D           │
│                                                                 │
│    WHERE:                                                       │
│        Q = interaction volume (flow)                            │
│        r = reciprocity score                                    │
│        q = quality score                                        │
│        τ = tone score                                           │
│        D = defense dampening                                    │
│                                                                 │
│        σ(r) = (2 / (1 + e^(-βr))) - 1     ∈ [-1, 1]           │
│        ψ(q) = 0.5 + q                      ∈ [0.5, 1.5]        │
│        φ(τ) = 0.7 + 0.3τ                   ∈ [0.4, 1.0]        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## 9.2 All Equations Summary

| # | Equation | Purpose |
|---|----------|---------|
| 1 | `w = w + (Φ - αw - D)` | Connection weight update |
| 2 | `Φ = γ\|Q\|^μ σ(r) ψ(q) φ(τ)` | Reinforcement |
| 3 | `σ(r) = 2/(1+e^-βr) - 1` | Reciprocity sigmoid |
| 4 | `ψ(q) = 0.5 + q` | Quality multiplier |
| 5 | `φ(τ) = 0.7 + 0.3τ` | Tone multiplier |
| 6 | `r = λr + (1-λ)(log(ρ) + θ(q-0.5))` | Reciprocity update |
| 7 | `T = Σ(wQ + wR·σ(R) + wS·S + wD·D) / Σw` | Trust level |
| 8 | `T_final = min(T, D + 0.3)` | Diversity cap |
| 9 | `S_conf = α·S_conf + (1-α)·A_mean` | Self-confidence |
| 10 | `conf_n = conf_0 × decay^n × w` | Signal attenuation |
| 11 | `belief = belief + w(1-belief)` | Threat belief update |
| 12 | `Conv = 1 - Var(pos)/Var_max` | Convergence score |
| 13 | `S_route = T × q_cap × (1-load) × w × (1-θ)` | **Routing score** |

## 9.3 Default Parameters

```
γ=0.1  μ=0.5  α=0.01  β=2.0  λ=0.9  θ=0.5  δ=0.2  ε=0.001
W_MIN=0.01  W_MAX=1.0  W_INIT=0.3
SWIFT_TRUST=0.4  PROBATION=50  THRESHOLD=0.6
DECAY_PER_HOP=0.8  MAX_HOPS=5  ACTION_THRESHOLD=0.7
```

---

**END OF SPECIFICATION**

*Symbiont v0.1 — Implement in any language, deploy on any network.*
