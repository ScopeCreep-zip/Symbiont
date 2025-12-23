# Core Principles

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

### 1.1 Purpose

This document defines the foundational principles that guide Symbiont's design and implementation. These principles are non-negotiable constraints that ensure the protocol achieves its goals of decentralized, emergent trust.

### 1.2 Scope

These principles apply to:
- All implementations of the Symbiont protocol
- Extensions and modifications to the core protocol
- Systems integrating with Symbiont networks

---

## 2. The Seven Core Principles

```mermaid
graph TB
    subgraph "Core Principles"
        P1[1. Trust Through<br/>Interaction]
        P2[2. No Central<br/>Authority]
        P3[3. Behavioral<br/>Adaptation]
        P4[4. Reciprocity<br/>Balance]
        P5[5. Collective<br/>Defense]
        P6[6. Graceful<br/>Degradation]
        P7[7. Verifiable<br/>State]
    end

    P1 --> GOAL[Robust<br/>Decentralized<br/>Trust]
    P2 --> GOAL
    P3 --> GOAL
    P4 --> GOAL
    P5 --> GOAL
    P6 --> GOAL
    P7 --> GOAL

    style GOAL fill:#b8956b
```

---

## 3. Principle 1: Trust Through Interaction

### 3.1 Statement

> **Trust is not assigned—it emerges from patterns of interaction.**

### 3.2 Rationale

Traditional trust systems rely on external authorities (certificate authorities, reputation servers, blockchain oracles) to assign trust scores. This creates dependencies and single points of failure.

In Symbiont, trust emerges naturally from how agents behave:
- Good behavior → Trust increases
- Bad behavior → Trust decreases
- No behavior → Trust decays

### 3.3 Implications

```mermaid
flowchart TD
    subgraph "What This Means"
        I1[Every interaction<br/>is measured]
        I2[Trust scores are<br/>continuously updated]
        I3[No external<br/>trust assignments]
        I4[History matters<br/>but decays]
    end

    subgraph "What This Forbids"
        F1[Static trust<br/>assignments]
        F2[Trust without<br/>interaction evidence]
        F3[External reputation<br/>imports]
    end

    style I1 fill:#7d9f85
    style I2 fill:#7d9f85
    style I3 fill:#7d9f85
    style I4 fill:#7d9f85
    style F1 fill:#a67676
    style F2 fill:#a67676
    style F3 fill:#a67676
```

### 3.4 Mathematical Expression

Trust is computed from aggregated interaction metrics:

$$T(n) = \frac{w_Q \cdot Q_{agg} + w_R \cdot \sigma(R_{agg}) + w_S \cdot S_{social} + w_D \cdot D_{diversity}}{\sum w}$$

Where all components derive from interaction history.

---

## 4. Principle 2: No Central Authority

### 4.1 Statement

> **There is no master node, oracle, or single point of control.**

### 4.2 Rationale

Central authorities introduce:
- **Single points of failure** — If the authority fails, the system fails
- **Trust bottlenecks** — All trust flows through one entity
- **Attack targets** — Adversaries can focus on one point
- **Power concentration** — Central authority can abuse position

### 4.3 Implications

| Allowed | Forbidden |
|---------|-----------|
| Peer-to-peer communication | Master-slave architectures |
| Local trust computation | Centralized reputation servers |
| Distributed defense signaling | Central threat databases |
| Emergent coordination | Appointed coordinators |

### 4.4 Decentralization Verification

A valid Symbiont implementation must pass these tests:

```mermaid
graph TD
    T1{Can any single<br/>node be removed<br/>without system failure?}
    T2{Do nodes compute<br/>trust independently?}
    T3{Can nodes disagree<br/>on trust assessments?}
    T4{Is there any<br/>privileged node class?}

    T1 -->|Yes| P1[Pass]
    T1 -->|No| F1[Fail]

    T2 -->|Yes| P2[Pass]
    T2 -->|No| F2[Fail]

    T3 -->|Yes| P3[Pass]
    T3 -->|No| F3[Fail]

    T4 -->|No| P4[Pass]
    T4 -->|Yes| F4[Fail]

    style P1 fill:#7d9f85
    style P2 fill:#7d9f85
    style P3 fill:#7d9f85
    style P4 fill:#7d9f85
    style F1 fill:#a67676
    style F2 fill:#a67676
    style F3 fill:#a67676
    style F4 fill:#a67676
```

---

## 5. Principle 3: Behavioral Adaptation

### 5.1 Statement

> **Trust must adapt to changing behavior over time.**

### 5.2 Rationale

Agents can change. A previously trustworthy agent may become adversarial, and vice versa. The trust system must:
- Respond to behavior changes
- Not be permanently locked by historical reputation
- Detect behavioral pattern shifts

### 5.3 Adaptation Mechanisms

```mermaid
graph LR
    subgraph "Positive Adaptation"
        PA1[Consistent good<br/>quality] --> PA2[Trust increases]
        PA2 --> PA3[Better routing<br/>scores]
    end

    subgraph "Negative Adaptation"
        NA1[Quality drops] --> NA2[Trust decreases]
        NA2 --> NA3[Lower routing<br/>priority]
    end

    subgraph "Adversary Detection"
        AD1[Pattern analysis] --> AD2[Strategic adversary<br/>detected]
        AD2 --> AD3[Defense signals<br/>emitted]
    end
```

### 5.4 Memory and Decay

The exponential moving average (EMA) with factor λ balances history and recency:

$$value_{new} = \lambda \cdot value_{old} + (1-\lambda) \cdot measurement$$

With λ = 0.9:
- Recent interactions have ~10% weight
- Older interactions decay exponentially
- Complete "memory reset" takes ~50 interactions

```mermaid
graph LR
    subgraph "Weight Distribution Over Time"
        W1[Interaction 1<br/>w=0.1]
        W2[Interaction 2<br/>w=0.09]
        W3[Interaction 3<br/>w=0.081]
        W4[...]
        W5[Interaction n<br/>w≈0]
    end
```

---

## 6. Principle 4: Reciprocity Balance

### 6.1 Statement

> **Sustainable relationships require balanced exchange.**

### 6.2 Rationale

Inspired by mycorrhizal networks, Symbiont requires that value flows bidirectionally over time. One-sided relationships are unsustainable and indicate potential exploitation.

### 6.3 Reciprocity Tracking

Each connection tracks the balance of exchange:

$$r_{new} = \lambda \cdot r_{old} + (1-\lambda) \cdot \left(\log\left(\frac{in}{out + \epsilon}\right) + \theta \cdot (q - 0.5)\right)$$

| Scenario | ρ = in/out | log(ρ) | Effect on r |
|----------|------------|--------|-------------|
| Balanced | 1.0 | 0 | Neutral |
| Giving more | 0.5 | -0.69 | Decreases |
| Receiving more | 2.0 | 0.69 | Increases |

### 6.4 Reciprocity in Connection Dynamics

The reciprocity score directly affects connection reinforcement:

```mermaid
graph TB
    subgraph "Reciprocity Effect"
        R1[r > 0<br/>Receiving more] --> S1["σ(r) > 0<br/>Positive reinforcement"]
        R2[r ≈ 0<br/>Balanced] --> S2["σ(r) ≈ 0<br/>Neutral"]
        R3[r < 0<br/>Giving more] --> S3["σ(r) < 0<br/>Negative reinforcement"]
    end

    S1 --> W1[Connection<br/>strengthens]
    S2 --> W2[Connection<br/>stable]
    S3 --> W3[Connection<br/>weakens]

    style W1 fill:#7d9f85
    style W2 fill:#c9c9a8
    style W3 fill:#a67676
```

---

## 7. Principle 5: Collective Defense

### 7.1 Statement

> **The network defends itself through coordinated signal propagation.**

### 7.2 Rationale

Individual nodes have limited visibility. By sharing threat information through trusted connections, the network can:
- Detect threats faster
- Respond before direct harm
- Isolate adversaries collectively

### 7.3 Defense Signal Flow

```mermaid
sequenceDiagram
    participant A as Node A<br/>(Detector)
    participant B as Node B<br/>(Connected)
    participant C as Node C<br/>(2nd degree)
    participant X as Node X<br/>(Adversary)

    Note over A,X: A detects threat from X
    A->>A: Update threat belief for X
    A->>B: Defense Signal (conf=0.8)
    A->>C: Defense Signal (conf=0.8)

    Note over B: Receives signal, updates belief
    B->>B: threat_belief[X] += 0.8 × T(A)

    Note over B: If conf > threshold, forward
    B->>C: Forward Signal (conf=0.64)

    Note over C: Aggregates from multiple sources
    C->>C: threat_belief[X] updated
```

### 7.4 Signal Attenuation

Signals decay with network distance to prevent:
- Unbounded propagation
- Amplification attacks
- False positive cascades

$$confidence_{hop_n} = confidence_{origin} \times (DECAY\_PER\_HOP)^n \times w_{connection}$$

With DECAY_PER_HOP = 0.8 and MAX_HOPS = 5:

| Hop | Confidence (starting at 1.0) |
|-----|------------------------------|
| 0 | 1.00 |
| 1 | 0.80 |
| 2 | 0.64 |
| 3 | 0.51 |
| 4 | 0.41 |
| 5 | 0.33 (min) |

---

## 8. Principle 6: Graceful Degradation

### 8.1 Statement

> **The system continues functioning even when components fail or misbehave.**

### 8.2 Rationale

In decentralized systems, failures are inevitable. The protocol must:
- Continue operating with missing nodes
- Maintain service with adversarial nodes present
- Recover from partial network partitions

### 8.3 Degradation Scenarios

```mermaid
graph TB
    subgraph "Failure Scenarios"
        F1[Node Failure]
        F2[Connection Loss]
        F3[Adversary Present]
        F4[Network Partition]
    end

    subgraph "System Response"
        R1[Route around<br/>failed node]
        R2[Use alternate<br/>connections]
        R3[Reduce trust,<br/>emit signals]
        R4[Continue in<br/>partition]
    end

    F1 --> R1
    F2 --> R2
    F3 --> R3
    F4 --> R4

    subgraph "Service Level"
        S1[Degraded but<br/>functional]
    end

    R1 --> S1
    R2 --> S1
    R3 --> S1
    R4 --> S1

    style S1 fill:#c9c9a8
```

### 8.4 No Single Points of Failure

By Principle 2 (No Central Authority), every function must have redundancy:

| Function | Redundancy Mechanism |
|----------|----------------------|
| Task routing | Multiple candidates scored |
| Trust computation | Local computation per node |
| Defense signaling | Multi-path propagation |
| Workflow execution | Re-routing on failure |

---

## 9. Principle 7: Verifiable State

### 9.1 Statement

> **All state changes must be traceable to signed, verifiable events.**

### 9.2 Rationale

For trust to be meaningful, nodes must be able to:
- Verify that interactions occurred
- Confirm the identity of interaction partners
- Audit the history leading to trust scores

### 9.3 Cryptographic Requirements

```mermaid
graph LR
    subgraph "Identity"
        ID[NodeId] --> |Ed25519| PUB[Public Key]
        PUB --> |Hash| NID[32-byte ID]
    end

    subgraph "Verification"
        MSG[Message] --> |Sign| SIG[Signature]
        SIG --> |Verify| VALID{Valid?}
    end

    subgraph "Integrity"
        DATA[Data] --> |BLAKE3/SHA-256| HASH[Hash]
    end
```

### 9.4 What Must Be Signed

| Event Type | Signed By | Contains |
|------------|-----------|----------|
| Interaction | Both parties | Task, outcome, timestamp |
| Defense Signal | Originator | Threat, evidence hash, confidence |
| Affirmation | Sender | Type, strength, timestamp |
| Handoff | Sender | Task, context, signature chain |

---

## 10. Principle Interactions

The seven principles interact and reinforce each other:

```mermaid
graph TB
    P1[Trust Through<br/>Interaction]
    P2[No Central<br/>Authority]
    P3[Behavioral<br/>Adaptation]
    P4[Reciprocity<br/>Balance]
    P5[Collective<br/>Defense]
    P6[Graceful<br/>Degradation]
    P7[Verifiable<br/>State]

    P1 -->|requires| P7
    P2 -->|enables| P6
    P3 -->|uses| P1
    P4 -->|informs| P1
    P5 -->|requires| P2
    P5 -->|requires| P7
    P6 -->|requires| P2

    style P1 fill:#b8956b
    style P2 fill:#6b8fa3
    style P3 fill:#7d9f85
    style P4 fill:#8a7a9a
    style P5 fill:#a67676
    style P6 fill:#a8a070
    style P7 fill:#9a9ab0
```

---

## 11. Compliance Checklist

Implementations must satisfy all principles:

| # | Principle | Requirement | Verification |
|---|-----------|-------------|--------------|
| 1 | Trust Through Interaction | Trust derived from interaction metrics only | Audit trust computation |
| 2 | No Central Authority | No privileged nodes; peer-to-peer | Architecture review |
| 3 | Behavioral Adaptation | Trust changes with behavior; memory decays | Test with behavior changes |
| 4 | Reciprocity Balance | Exchange ratio tracked; affects dynamics | Verify reciprocity in equations |
| 5 | Collective Defense | Signals propagate through network | Test defense scenarios |
| 6 | Graceful Degradation | Service continues with failures | Fault injection testing |
| 7 | Verifiable State | All events signed and verifiable | Cryptographic audit |

---

## 12. Summary

The seven core principles ensure that Symbiont achieves its goal of decentralized, emergent trust:

1. **Trust Through Interaction** — No external trust sources
2. **No Central Authority** — Fully decentralized
3. **Behavioral Adaptation** — Responds to change
4. **Reciprocity Balance** — Sustainable relationships
5. **Collective Defense** — Network-wide security
6. **Graceful Degradation** — Fault tolerant
7. **Verifiable State** — Cryptographically auditable

These principles are not optional guidelines—they are constraints that define what Symbiont is and is not.

---

*Previous: [Biological Foundations](./biological-foundations.md) | Next: [Glossary](./glossary.md)*
