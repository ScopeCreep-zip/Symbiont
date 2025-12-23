# Data Flow

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

### 1.1 Purpose

This document describes how data flows through the Symbiont system—from task initiation through execution, trust updates, and signal propagation. Understanding data flow is essential for implementing and debugging Symbiont systems.

### 1.2 Overview

Data flows through Symbiont in several patterns:

```mermaid
graph TB
    subgraph "Data Flow Patterns"
        P1[Task Flow<br/>Request → Execute → Response]
        P2[Trust Flow<br/>Interaction → Measurement → Update]
        P3[Signal Flow<br/>Detection → Propagation → Action]
        P4[Workflow Flow<br/>Step → Handoff → Step → ...]
    end
```

---

## 2. Task Flow

### 2.1 Single Task Flow

The simplest flow: one task, one executor.

```mermaid
sequenceDiagram
    participant I as Initiator
    participant R as Router
    participant E as Executor

    Note over I: Task arrives
    I->>R: Route(task)

    Note over R: Score candidates
    R->>R: For each candidate:<br/>score = T × q_cap × (1-load) × w × (1-threat)

    R->>I: Best candidate: E

    I->>E: TaskRequest(task, context)

    Note over E: Execute task
    E->>E: Validate capability
    E->>E: Execute
    E->>E: Prepare result

    E->>I: TaskResponse(result)

    Note over I: Record interaction
    I->>I: Measure quality, reciprocity, tone
    I->>I: Update connection to E
    I->>I: Maybe send affirmation/signal
```

### 2.2 Task Data Structure

```
TaskRequest {
    task_id      : TaskId
    capability   : CapabilityId
    input        : TaskInput
    constraints  : TaskConstraints
    context      : TaskContext
    timestamp    : Timestamp
    signature    : Signature
}

TaskResponse {
    task_id      : TaskId
    status       : { SUCCESS, FAILURE, PARTIAL }
    output       : TaskOutput
    metrics      : ExecutionMetrics
    timestamp    : Timestamp
    signature    : Signature
}
```

### 2.3 Routing Decision Flow

```mermaid
flowchart TD
    START[Task Arrives] --> CHECK{Can handle<br/>locally?}

    CHECK -->|Yes, available| LOCAL[Execute Locally]
    CHECK -->|No capability| ROUTE[Route to Network]
    CHECK -->|Overloaded| ROUTE

    ROUTE --> FIND[Find Candidates]
    FIND --> SCORE[Score Each Candidate]
    SCORE --> SELECT[Select Best]
    SELECT --> HANDOFF[Handoff Task]

    LOCAL --> RESULT[Return Result]
    HANDOFF --> WAIT[Wait for Result]
    WAIT --> RESULT

    SCORE --> FORMULA["score = T(n) × q_cap(n)<br/>× (1-load) × w<br/>× (1-threat)"]

    style FORMULA fill:#c9c9a8
```

---

## 3. Interaction Flow

### 3.1 Complete Interaction Cycle

After every interaction, the following updates occur:

```mermaid
flowchart TD
    INT[Interaction Complete] --> MEAS[Measure Outcomes]

    MEAS --> Q[Quality Score<br/>q ∈ [0,1]]
    MEAS --> RHO[Exchange Ratio<br/>ρ = in/out]
    MEAS --> TAU[Tone Score<br/>τ ∈ [-1,1]]

    Q --> UPD[Update Connection]
    RHO --> UPD
    TAU --> UPD

    UPD --> RECIP[Update Reciprocity<br/>r = λr + (1-λ)(log(ρ) + θ(q-0.5))]

    UPD --> QUAL[Update Quality<br/>conn.q = λ(conn.q) + (1-λ)q]

    UPD --> TONE[Update Tone<br/>conn.τ = λ(conn.τ) + (1-λ)τ]

    RECIP --> PHY[Apply Physarum Equation]
    QUAL --> PHY
    TONE --> PHY

    PHY --> NEWW["w_new = w + Δt(Φ - αw - D)"]

    NEWW --> POST[Post-Interaction]
    POST --> AFF{Quality > 0.8<br/>Tone > 0.5?}
    POST --> WARN{Problems<br/>detected?}

    AFF -->|Yes| SEND_AFF[Send Affirmation]
    WARN -->|Yes| SEND_DEF[Emit Defense Signal]

    style NEWW fill:#b8956b
```

### 3.2 Measurement Details

#### Quality Measurement

```
FUNCTION compute_quality(feedback):

    Q_raw = (ω_HELP × feedback.helpfulness +    // 0.4
             ω_ACC  × feedback.accuracy +        // 0.3
             ω_REL  × feedback.relevance +       // 0.2
             ω_TIME × feedback.timeliness) / 4   // 0.1

    // Apply reuse modifier
    IF feedback.would_reuse:
        Q_raw = Q_raw × REUSE_BOOST     // 1.2
    ELSE:
        Q_raw = Q_raw × REUSE_PENALTY   // 0.8

    // Normalize to [0, 1]
    Q_normalized = (Q_raw - 1) / 4

    RETURN Q_normalized
```

#### Reciprocity Update

```mermaid
graph LR
    subgraph "Inputs"
        IN[exchange_in]
        OUT[exchange_out]
        Q[quality]
    end

    subgraph "Computation"
        RATIO["ρ = in / (out + ε)"]
        LOG["log(ρ + ε)"]
        ADJ["θ × (q - 0.5)"]
        SUM["log(ρ) + adjustment"]
    end

    subgraph "Update"
        EMA["r_new = λ×r + (1-λ)×sum"]
    end

    IN --> RATIO
    OUT --> RATIO
    RATIO --> LOG
    Q --> ADJ
    LOG --> SUM
    ADJ --> SUM
    SUM --> EMA
```

### 3.3 Connection Update Pipeline

```
FUNCTION update_connection(self, partner, interaction):

    conn = self.connections[partner.id]

    // Step 1: Compute measurements
    q   = compute_quality(interaction.feedback)
    ρ   = interaction.exchange_in / (interaction.exchange_out + ε)
    τ   = compute_tone(interaction)

    // Step 2: Update reciprocity (EMA)
    log_rho = log(ρ + ε)
    quality_adj = θ × (q - 0.5)
    conn.r = λ × conn.r + (1 - λ) × (log_rho + quality_adj)

    // Step 3: Update quality (EMA)
    conn.q = λ × conn.q + (1 - λ) × q

    // Step 4: Update tone (EMA)
    conn.τ = λ × conn.τ + (1 - λ) × τ

    // Step 5: Compute reinforcement (Physarum)
    Q   = interaction.task_volume
    σ_r = sigmoid(conn.r)       // (2 / (1 + e^(-βr))) - 1
    ψ_q = 0.5 + conn.q          // [0.5, 1.5]
    φ_τ = 0.7 + 0.3 × conn.τ    // [0.4, 1.0]

    Φ = γ × pow(|Q|, μ) × σ_r × ψ_q × φ_τ

    // Step 6: Compute defense dampening
    D = 0
    IF partner.id IN self.threat_beliefs:
        D = δ × self.threat_beliefs[partner.id].level

    // Step 7: Update weight
    Δw = Φ - α × conn.w - D
    conn.w = clamp(conn.w + Δw, W_MIN, W_MAX)

    // Step 8: Bookkeeping
    conn.last_active = now()
    conn.count += 1
```

---

## 4. Defense Signal Flow

### 4.1 Signal Generation

```mermaid
flowchart TD
    DETECT[Threat Detected] --> CREATE[Create Signal]

    CREATE --> SIG["DefenseSignal {
        type: SPECIFIC_THREAT
        threat: adversary_id
        threat_type: detected_type
        confidence: computed_conf
        evidence: hash(evidence)
        hops: 0
    }"]

    SIG --> SIGN[Sign with Private Key]
    SIGN --> SEND[Send to Connections]

    SEND --> FILTER{w > threshold?}
    FILTER -->|Yes| DELIVER[Deliver Signal]
    FILTER -->|No| SKIP[Skip Connection]
```

### 4.2 Signal Propagation

```mermaid
sequenceDiagram
    participant A as Node A<br/>(Detector)
    participant B as Node B
    participant C as Node C
    participant D as Node D

    Note over A: Detects threat X

    A->>B: Signal(X, conf=0.9, hops=0)
    A->>C: Signal(X, conf=0.9, hops=0)

    Note over B: Process signal
    B->>B: Update threat_belief[X]
    B->>B: Increase priming

    Note over B: Check propagation
    B->>B: conf × decay = 0.9 × 0.8 = 0.72
    B->>B: 0.72 > PROPAGATE_THRESHOLD (0.6)

    B->>D: Signal(X, conf=0.72, hops=1)

    Note over C: Process signal
    C->>C: Update threat_belief[X]
    C->>C: Increase priming

    Note over D: Process signal
    D->>D: Update threat_belief[X]
    D->>D: conf=0.72 × 0.8 = 0.58 < 0.6
    Note over D: Don't propagate further
```

### 4.3 Belief Update Flow

```mermaid
flowchart TD
    RECV[Receive Signal] --> VALID{Valid<br/>signature?}

    VALID -->|No| DROP[Drop Signal]
    VALID -->|Yes| TRUST[Get Sender Trust]

    TRUST --> WEIGHT["weight = T(sender) × signal.conf"]

    WEIGHT --> UPDATE["belief_new = belief_old + weight × (1 - belief_old)"]

    UPDATE --> PRIME["priming += conf × SENSITIVITY"]

    PRIME --> ACTION{belief ><br/>ACTION_THRESHOLD?}

    ACTION -->|Yes| DEFEND[Take Defensive Action]
    ACTION -->|No| PROP{conf × decay ><br/>PROPAGATE_THRESHOLD?}

    PROP -->|Yes| FORWARD[Forward Signal]
    PROP -->|No| DONE[Done]
```

---

## 5. Workflow Flow

### 5.1 Sequential Workflow

```mermaid
sequenceDiagram
    participant O as Origin
    participant A as Node A
    participant B as Node B
    participant C as Node C

    Note over O: Workflow: Step1 → Step2 → Step3

    O->>O: Route Step1
    O->>A: Handoff(Step1, context)
    A->>A: Execute Step1
    A->>O: Result1

    Note over O: Update context
    O->>O: context += Result1
    O->>O: Route Step2

    O->>B: Handoff(Step2, context)
    B->>B: Execute Step2
    B->>O: Result2

    Note over O: Update context
    O->>O: context += Result2
    O->>O: Route Step3

    O->>C: Handoff(Step3, context)
    C->>C: Execute Step3
    C->>O: Result3

    Note over O: Workflow complete
```

### 5.2 Handoff Data Structure

```
Handoff {
    from_node    : NodeId
    to_node      : NodeId
    task         : Task
    context      : HandoffContext
    timestamp    : Timestamp
    signature    : Signature
}

HandoffContext {
    workflow_id   : WorkflowId
    step_index    : uint32
    prior_results : List<StepResult>
    accumulated   : Map<string, any>
    lineage       : List<NodeId>
}
```

### 5.3 Parallel Workflow

```mermaid
flowchart TD
    START[Workflow Start] --> FAN[Fan Out]

    FAN --> P1[Route to Node A]
    FAN --> P2[Route to Node B]
    FAN --> P3[Route to Node C]

    P1 --> E1[Execute]
    P2 --> E2[Execute]
    P3 --> E3[Execute]

    E1 --> WAIT[Wait for All]
    E2 --> WAIT
    E3 --> WAIT

    WAIT --> AGG[Aggregate Results]
    AGG --> MERGE[Merge Step]
    MERGE --> DONE[Complete]
```

### 5.4 DAG Workflow

```mermaid
graph LR
    subgraph "DAG Workflow"
        S1[Step 1] --> S2[Step 2]
        S1 --> S3[Step 3]
        S2 --> S4[Step 4]
        S3 --> S4
        S4 --> S5[Step 5]
    end

    style S4 fill:#b8956b
```

Execution order:
1. S1 executes
2. S2 and S3 execute in parallel (both depend only on S1)
3. S4 executes (waits for S2 AND S3)
4. S5 executes

---

## 6. Trust Computation Flow

### 6.1 Trust Aggregation

```mermaid
flowchart TD
    subgraph "Per-Connection Data"
        C1[Connection 1<br/>q=0.8, r=0.3]
        C2[Connection 2<br/>q=0.7, r=-0.1]
        C3[Connection 3<br/>q=0.9, r=0.5]
    end

    subgraph "Aggregation"
        Q_AGG["Q_agg = weighted_mean(q)"]
        R_AGG["R_agg = mean(r)"]
    end

    subgraph "Social Proof"
        S["S = Σ(affirmations × T_affirmer)"]
    end

    subgraph "Diversity"
        D["D = unique_partners / 100"]
    end

    C1 --> Q_AGG
    C2 --> Q_AGG
    C3 --> Q_AGG
    C1 --> R_AGG
    C2 --> R_AGG
    C3 --> R_AGG

    Q_AGG --> T
    R_AGG --> T
    S --> T
    D --> T

    T["T = (w_Q × Q + w_R × σ(R) + w_S × S + w_D × D) / Σw"]

    T --> CAP1{T > D + 0.3?}
    CAP1 -->|Yes| DIV_CAP["T = D + 0.3"]
    CAP1 -->|No| CAP2{T > trust_cap?}
    DIV_CAP --> CAP2

    CAP2 -->|Yes| TRUST_CAP["T = trust_cap"]
    CAP2 -->|No| FINAL[Final Trust Score]
    TRUST_CAP --> FINAL
```

### 6.2 Per-Capability Quality

```mermaid
graph TB
    subgraph "Interactions"
        I1[Interaction 1<br/>cap=A, q=0.9]
        I2[Interaction 2<br/>cap=B, q=0.7]
        I3[Interaction 3<br/>cap=A, q=0.85]
        I4[Interaction 4<br/>cap=A, q=0.92]
    end

    subgraph "Per-Cap Aggregation"
        QA["q_cap(A) = EMA(0.9, 0.85, 0.92)"]
        QB["q_cap(B) = EMA(0.7)"]
    end

    I1 --> QA
    I3 --> QA
    I4 --> QA
    I2 --> QB

    subgraph "Global"
        GLOBAL["Q_global = weighted_avg(q_cap, volume)"]
    end

    QA --> GLOBAL
    QB --> GLOBAL
```

---

## 7. Background Flow

### 7.1 Periodic Tasks

```mermaid
flowchart TD
    TICK[Background Tick] --> DECAY_PRIME[Decay Priming<br/>π = π × 0.99]

    DECAY_PRIME --> DECAY_CONN[Decay Idle Connections]

    DECAY_CONN --> FOREACH{For each<br/>connection}
    FOREACH --> CHECK{Idle too<br/>long?}
    CHECK -->|Yes| DECAY_W["w = w × (1-α)"]
    CHECK -->|No| NEXT[Next]
    DECAY_W --> REMOVE{w < W_MIN?}
    REMOVE -->|Yes| DELETE[Remove Connection]
    REMOVE -->|No| NEXT

    NEXT --> FOREACH
    DELETE --> FOREACH

    FOREACH -->|Done| CAPS[Update Capability Load]

    CAPS --> DIV{Diversity<br/>check due?}
    DIV -->|Yes| CHECK_DIV[Compute Diversity]
    CHECK_DIV --> UPDATE_CAP[Update Trust Cap if Low]

    DIV -->|No| STATUS{Status<br/>check due?}
    UPDATE_CAP --> STATUS

    STATUS -->|Yes| TRANS[Check Status Transition]
    STATUS -->|No| SCAN{Adversary<br/>scan due?}
    TRANS --> SCAN

    SCAN -->|Yes| DETECT[Scan for Adversaries]
    SCAN -->|No| DONE[Tick Complete]
    DETECT --> DONE
```

### 7.2 Decay Dynamics

| What Decays | Rate | Formula |
|-------------|------|---------|
| Priming | Per tick | π × 0.99 |
| Idle connections | Per tick | w × (1 - α) |
| Capability load | Per tick | load × 0.95 |
| Threat beliefs | Per tick | belief × decay_rate |

---

## 8. Data Flow Summary

```mermaid
graph TB
    subgraph "Input Events"
        TASK[Task Request]
        SIGNAL[Defense Signal]
        AFF[Affirmation]
        HANDOFF[Handoff Request]
    end

    subgraph "Processing"
        ROUTE[Routing Engine]
        INTERACT[Interaction Handler]
        DEFENSE[Defense Engine]
        WORKFLOW[Workflow Engine]
    end

    subgraph "State Updates"
        CONN[Connection State]
        TRUST[Trust Scores]
        BELIEF[Threat Beliefs]
        PRIME[Priming Level]
    end

    subgraph "Output Events"
        RESP[Task Response]
        OUT_SIG[Defense Signal Out]
        OUT_AFF[Affirmation Out]
        RESULT[Workflow Result]
    end

    TASK --> ROUTE
    ROUTE --> INTERACT
    INTERACT --> CONN
    CONN --> TRUST
    INTERACT --> RESP
    INTERACT --> OUT_AFF

    SIGNAL --> DEFENSE
    DEFENSE --> BELIEF
    DEFENSE --> PRIME
    DEFENSE --> OUT_SIG

    AFF --> INTERACT

    HANDOFF --> WORKFLOW
    WORKFLOW --> INTERACT
    WORKFLOW --> RESULT

    style TRUST fill:#7d9f85
    style BELIEF fill:#a67676
```

---

## 9. Key Takeaways

| Flow Type | Trigger | Updates | Outputs |
|-----------|---------|---------|---------|
| **Task** | Task request | Connection w, q, r, τ | Response, maybe signal/affirmation |
| **Defense** | Threat detection | Threat beliefs, priming | Forwarded signals |
| **Workflow** | Multi-step task | Same as task per step | Final result |
| **Background** | Timer | Decay connections, priming | Status transitions |

All flows ultimately contribute to the trust computation, which in turn affects future routing decisions—creating a feedback loop that naturally optimizes network behavior.

---

*Previous: [Network Topology](./network.md) | Next: [The Physarum Equation](../protocol/physarum-equation.md)*
