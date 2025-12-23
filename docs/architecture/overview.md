# Architecture Overview

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

### 1.1 Purpose

This document provides a comprehensive overview of the Symbiont system architecture. It describes the major components, their relationships, and how they work together to achieve decentralized trust and orchestration.

### 1.2 Scope

This document covers:
- High-level system structure
- Component responsibilities
- Inter-component communication
- Deployment considerations

---

## 2. System Context

### 2.1 External View

Symbiont operates as a layer that enables trust-based coordination between autonomous agents:

```mermaid
graph TB
    subgraph "External Systems"
        U1[Users/Applications]
        U2[AI Models/Services]
        U3[External APIs]
    end

    subgraph "Symbiont Network"
        N1((Node 1))
        N2((Node 2))
        N3((Node 3))
        N4((Node 4))

        N1 --- N2
        N2 --- N3
        N3 --- N4
        N1 --- N3
        N2 --- N4
    end

    U1 --> N1
    U2 --> N2
    U3 --> N3

    style N1 fill:#87CEEB
    style N2 fill:#87CEEB
    style N3 fill:#87CEEB
    style N4 fill:#87CEEB
```

### 2.2 What Symbiont Provides

| Capability | Description |
|------------|-------------|
| **Trust Computation** | Determines how trustworthy each node is |
| **Task Routing** | Directs tasks to capable, trusted nodes |
| **Defense Signaling** | Propagates threat warnings through the network |
| **Workflow Orchestration** | Coordinates multi-step task sequences |
| **Adversary Detection** | Identifies malicious behavior patterns |

### 2.3 What Symbiont Does NOT Provide

| Not Included | Reason |
|--------------|--------|
| Transport Layer | Use existing P2P (libp2p, Veilid) or custom |
| Identity Management | Use existing PKI or identity systems |
| Task Execution | Nodes implement their own capabilities |
| Consensus | Trust emerges; no global consensus needed |

---

## 3. Architectural Layers

### 3.1 Layer Diagram

```mermaid
graph TB
    subgraph "Application Layer"
        APP[Applications & Agents]
    end

    subgraph "Orchestration Layer"
        WF[Workflow Engine]
        RT[Task Router]
        HO[Handoff Protocol]
    end

    subgraph "Trust Layer"
        TC[Trust Computation]
        CD[Connection Dynamics]
        RR[Reciprocity Tracking]
    end

    subgraph "Defense Layer"
        DS[Defense Signaling]
        AD[Adversary Detection]
        TB[Threat Beliefs]
    end

    subgraph "Foundation Layer"
        CR[Cryptography]
        ST[State Management]
        NT[Network Transport]
    end

    APP --> WF
    APP --> RT
    WF --> HO
    RT --> TC
    HO --> TC
    TC --> CD
    TC --> RR
    CD --> DS
    AD --> DS
    DS --> TB
    TC --> CR
    CD --> ST
    DS --> NT

    style APP fill:#FFD700
    style WF fill:#87CEEB
    style RT fill:#87CEEB
    style HO fill:#87CEEB
    style TC fill:#98FB98
    style CD fill:#98FB98
    style RR fill:#98FB98
    style DS fill:#FFB6C1
    style AD fill:#FFB6C1
    style TB fill:#FFB6C1
```

### 3.2 Layer Responsibilities

| Layer | Responsibility | Key Components |
|-------|----------------|----------------|
| **Application** | User-facing functionality | Agents, UIs, APIs |
| **Orchestration** | Task routing and workflow | Router, Workflow Engine, Handoff |
| **Trust** | Trust computation | Dynamics, Reciprocity, Aggregation |
| **Defense** | Threat response | Signals, Detection, Beliefs |
| **Foundation** | Core infrastructure | Crypto, State, Network |

---

## 4. Component Architecture

### 4.1 Node Components

Each Symbiont node contains the following internal components:

```mermaid
graph TB
    subgraph "Symbiont Node"
        subgraph "Processors"
            TP[Task Processor]
            EH[Event Handler]
            BG[Background Jobs]
        end

        subgraph "State"
            NS[Node State]
            CM[Connection Map]
            CP[Capabilities]
            TB[Threat Beliefs]
        end

        subgraph "Engines"
            TE[Trust Engine]
            DE[Defense Engine]
            WE[Workflow Engine]
        end

        TP --> NS
        TP --> TE
        TP --> WE
        EH --> DE
        EH --> NS
        BG --> NS
        BG --> CM
        TE --> CM
        DE --> TB
        WE --> TE
    end

    style TP fill:#FFD700
    style EH fill:#FFD700
    style BG fill:#FFD700
    style NS fill:#98FB98
    style CM fill:#98FB98
    style CP fill:#98FB98
    style TB fill:#98FB98
```

### 4.2 Component Descriptions

#### 4.2.1 Task Processor
**Purpose:** Routes incoming tasks and orchestrates workflows.

**Responsibilities:**
- Evaluate whether to handle locally or delegate
- Score candidates for routing
- Execute local tasks
- Manage workflow state

#### 4.2.2 Event Handler
**Purpose:** Reacts to asynchronous events from the network.

**Handled Events:**
- Defense signals from other nodes
- Affirmations received
- Handoff requests
- Workflow updates

#### 4.2.3 Background Jobs
**Purpose:** Performs periodic maintenance tasks.

**Jobs:**
- Decay idle connections
- Decay priming level
- Update capability availability
- Check diversity scores
- Scan for adversaries

#### 4.2.4 Trust Engine
**Purpose:** Computes trust scores and manages connection dynamics.

**Operations:**
- Apply Physarum equation to connections
- Aggregate trust scores
- Apply diversity caps
- Track quality per capability

#### 4.2.5 Defense Engine
**Purpose:** Manages threat detection and response.

**Operations:**
- Process incoming defense signals
- Update threat beliefs
- Emit signals when threats detected
- Manage priming state

#### 4.2.6 Workflow Engine
**Purpose:** Coordinates multi-step task execution.

**Operations:**
- Parse workflow definitions
- Track step completion
- Manage handoffs between nodes
- Handle failures and retries

---

## 5. State Management

### 5.1 State Categories

```mermaid
graph LR
    subgraph "Persistent State"
        PS1[Node Identity]
        PS2[Connection History]
        PS3[Capability Registry]
    end

    subgraph "Session State"
        SS1[Active Workflows]
        SS2[Current Load]
        SS3[Priming Level]
    end

    subgraph "Derived State"
        DS1[Trust Scores]
        DS2[Routing Scores]
        DS3[Threat Beliefs]
    end

    PS1 --> DS1
    PS2 --> DS1
    SS2 --> DS2
    DS1 --> DS2
    PS2 --> DS3
```

### 5.2 State Persistence

| State | Persistence | Recovery |
|-------|-------------|----------|
| Node Identity | Permanent | From key storage |
| Connections | Durable | From database |
| Active Workflows | Session | Lost on restart |
| Trust Scores | Derived | Recomputed |
| Priming | Session | Resets to 0 |

---

## 6. Communication Patterns

### 6.1 Message Types

```mermaid
graph LR
    subgraph "Request/Response"
        REQ[Task Request] --> RES[Task Response]
    end

    subgraph "One-Way"
        AFF[Affirmation] --> |no response| X1[" "]
        DEF[Defense Signal] --> |no response| X2[" "]
    end

    subgraph "Handoff"
        HO1[Handoff Request] --> HO2[Handoff Accept/Reject]
        HO2 --> HO3[Result]
    end

    style X1 fill:none,stroke:none
    style X2 fill:none,stroke:none
```

### 6.2 Message Flow Example

```mermaid
sequenceDiagram
    participant A as Node A
    participant B as Node B
    participant C as Node C

    Note over A: Task arrives
    A->>A: Score candidates

    A->>B: Task Request
    B->>B: Execute task
    B->>A: Task Response

    A->>A: Measure quality
    A->>A: Update connection
    A->>A: Compute trust

    opt High quality
        A->>B: Affirmation
    end

    opt Problem detected
        A->>B: Defense Signal
        A->>C: Defense Signal
    end
```

---

## 7. Concurrency Model

### 7.1 Concurrent Processes

A Symbiont node runs four concurrent processes:

```mermaid
graph TB
    subgraph "Process 1: Task Processing"
        TP1[Receive Task]
        TP2[Route/Execute]
        TP3[Return Result]
        TP1 --> TP2 --> TP3
    end

    subgraph "Process 2: Interaction Loop"
        IL1[Complete Interaction]
        IL2[Measure Outcomes]
        IL3[Update Connection]
        IL1 --> IL2 --> IL3
    end

    subgraph "Process 3: Event Handling"
        EH1[Receive Event]
        EH2[Update State]
        EH3[Maybe Propagate]
        EH1 --> EH2 --> EH3
    end

    subgraph "Process 4: Background"
        BG1[Decay Connections]
        BG2[Update Availability]
        BG3[Scan for Adversaries]
        BG1 --> BG2 --> BG3 --> BG1
    end
```

### 7.2 Synchronization Points

| Shared Resource | Access Pattern | Synchronization |
|-----------------|----------------|-----------------|
| Connection Map | Read-heavy, occasional write | RwLock |
| Trust Scores | Write after each interaction | Mutex |
| Threat Beliefs | Write on signal, read on route | RwLock |
| Workflow State | Exclusive per workflow | Per-workflow lock |

---

## 8. Deployment Architecture

### 8.1 Single Node Deployment

The simplest deployment is a single node:

```mermaid
graph TB
    subgraph "Host Machine"
        subgraph "Symbiont Node"
            CORE[symbiont-core]
            CLI[symbiont-cli]
        end
        APP[Application]
    end

    APP --> CLI
    CLI --> CORE
```

### 8.2 Multi-Node Network

Production deployments involve multiple nodes:

```mermaid
graph TB
    subgraph "Network"
        subgraph "Region A"
            N1[Node 1]
            N2[Node 2]
        end

        subgraph "Region B"
            N3[Node 3]
            N4[Node 4]
        end

        N1 --- N2
        N1 --- N3
        N2 --- N4
        N3 --- N4
    end

    style N1 fill:#87CEEB
    style N2 fill:#87CEEB
    style N3 fill:#98FB98
    style N4 fill:#98FB98
```

### 8.3 Crate Structure

The Symbiont implementation is organized into crates:

```
symbiont/
├── symbiont-core/     # Core protocol library
│   ├── types.rs       # Core types
│   ├── node.rs        # Node implementation
│   ├── connection.rs  # Connection dynamics
│   ├── trust.rs       # Trust computation
│   ├── defense.rs     # Defense signaling
│   ├── routing.rs     # Task routing
│   ├── workflow.rs    # Workflow execution
│   ├── handoff.rs     # Handoff protocol
│   ├── detection.rs   # Adversary detection
│   ├── convergence.rs # Convergence protocol
│   ├── math.rs        # Mathematical functions
│   └── constants.rs   # Protocol constants
│
├── symbiont-sim/      # Simulation harness
│   ├── network.rs     # Network simulation
│   ├── agents.rs      # Agent behaviors
│   ├── scenarios/     # Test scenarios
│   └── metrics.rs     # Metrics collection
│
└── symbiont-cli/      # Command-line interface
    └── main.rs        # CLI entry point
```

---

## 9. Integration Points

### 9.1 Transport Integration

Symbiont is transport-agnostic. Integrate with:

| Transport | Use Case |
|-----------|----------|
| libp2p | General-purpose P2P |
| Veilid | Privacy-focused P2P |
| gRPC | Service mesh integration |
| WebSocket | Browser-based clients |
| Custom | Specialized requirements |

### 9.2 Identity Integration

Node identity can integrate with:

| System | Integration |
|--------|-------------|
| Ed25519 keys | Native support |
| X.509 certificates | Extract public key |
| DID documents | Use verification method |
| Hardware security | HSM key storage |

### 9.3 Application Integration

Applications integrate via:

```mermaid
graph LR
    APP[Application] --> |Rust API| CORE[symbiont-core]
    APP --> |CLI| CLI[symbiont-cli]
    APP --> |Future: gRPC| GRPC[symbiont-server]
```

---

## 10. Scalability Considerations

### 10.1 Connection Limits

Each node maintains connections to a subset of the network:

| Metric | Guideline |
|--------|-----------|
| Active connections | 10-100 typical |
| Maximum connections | Implementation-dependent |
| Connection storage | O(connections) per node |

### 10.2 Network Size

Symbiont scales horizontally:

| Network Size | Considerations |
|--------------|----------------|
| Small (< 100) | Full connectivity possible |
| Medium (100-1000) | Partial connectivity |
| Large (> 1000) | Hub nodes emerge naturally |

### 10.3 Message Complexity

| Operation | Messages | Complexity |
|-----------|----------|------------|
| Single task | 2 | O(1) |
| Routing decision | 0-k (local) | O(connections) |
| Defense signal | ≤ 5 hops | O(connections × hops) |
| Workflow | 2 × steps | O(steps) |

---

## 11. Summary

The Symbiont architecture provides:

- **Modular design** — Clear separation of concerns
- **Concurrent execution** — Multiple processes work independently
- **Transport agnosticism** — Integrate with any network layer
- **Horizontal scalability** — Add nodes without coordination
- **Resilience** — No single points of failure

```mermaid
graph TB
    subgraph "Architecture Summary"
        L1[Application Layer<br/>Agents & UIs]
        L2[Orchestration Layer<br/>Routing & Workflows]
        L3[Trust Layer<br/>Dynamics & Computation]
        L4[Defense Layer<br/>Signals & Detection]
        L5[Foundation Layer<br/>Crypto & Network]
    end

    L1 --> L2 --> L3 --> L4 --> L5

    style L1 fill:#FFD700
    style L2 fill:#87CEEB
    style L3 fill:#98FB98
    style L4 fill:#FFB6C1
    style L5 fill:#E6E6FA
```

---

*Previous: [Glossary](../concepts/glossary.md) | Next: [Node Architecture](./node.md)*
