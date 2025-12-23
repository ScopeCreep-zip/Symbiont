# Network Topology

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

### 1.1 Purpose

This document describes the network topology of Symbiont systems—how nodes connect, how the network evolves, and what emergent properties arise from local connection dynamics.

### 1.2 Key Concepts

- Networks are **self-organizing** — topology emerges from interactions
- Connections are **weighted** — not all links are equal
- The network is **dynamic** — connections strengthen, weaken, appear, and disappear
- There are **no privileged nodes** — the network is fully decentralized

---

## 2. Network Model

### 2.1 Graph Representation

A Symbiont network is a **weighted, directed graph** G = (V, E, W) where:

- **V** = Set of nodes (agents)
- **E** = Set of edges (connections)
- **W** = Edge weights ∈ [0, 1]

```mermaid
graph LR
    A((A)) ---|w=0.85| B((B))
    A ---|w=0.42| C((C))
    B ---|w=0.91| C
    B ---|w=0.33| D((D))
    C ---|w=0.67| D
    C ---|w=0.55| E((E))
    D ---|w=0.78| E

    style A fill:#98FB98
    style B fill:#87CEEB
    style C fill:#FFD700
    style D fill:#DDA0DD
    style E fill:#FFB6C1
```

### 2.2 Connection Directionality

While the graph is logically undirected (if A connects to B, B connects to A), each node maintains its **own view** of the connection:

```mermaid
graph LR
    subgraph "Node A's View"
        A1[A] -->|"w=0.75<br/>r=0.3"| B1[B]
    end

    subgraph "Node B's View"
        B2[B] -->|"w=0.68<br/>r=-0.2"| A2[A]
    end
```

**Key Point:** Node A's weight to B may differ from B's weight to A. This asymmetry captures:
- Different perceived quality of interactions
- Different reciprocity experiences
- Information asymmetry

### 2.3 Network Metrics

| Metric | Definition | Significance |
|--------|------------|--------------|
| **Degree** | Number of connections | Connectivity |
| **Weighted Degree** | Sum of connection weights | Connection strength |
| **Clustering Coefficient** | Triangle density | Local cohesion |
| **Path Length** | Hops between nodes | Information propagation |
| **Betweenness Centrality** | Bridge importance | Routing significance |

---

## 3. Network Evolution

### 3.1 Connection Formation

Connections form when nodes interact:

```mermaid
sequenceDiagram
    participant A as Node A
    participant B as Node B

    Note over A,B: No prior connection
    A->>B: First task request
    B->>A: Response

    Note over A,B: Connection created
    Note over A: Creates connection to B<br/>w = W_INIT (0.3)
    Note over B: Creates connection to A<br/>w = W_INIT (0.3)

    A->>B: Second task (good outcome)
    Note over A,B: Weights increase
```

### 3.2 Connection Dynamics

Connections evolve according to the Physarum equation:

$$\frac{dw}{dt} = \gamma \cdot |Q|^\mu \cdot \sigma(r) \cdot \psi(q) \cdot \phi(\tau) - \alpha \cdot w - D$$

```mermaid
graph TB
    subgraph "Weight Evolution"
        direction LR
        START[w = 0.3<br/>New connection]
        GOOD[w increases<br/>Good interactions]
        STABLE[w stable<br/>Balanced exchange]
        DECAY[w decreases<br/>Inactivity or problems]
        REMOVE[w < W_MIN<br/>Connection removed]
    end

    START --> GOOD
    START --> DECAY
    GOOD --> STABLE
    GOOD --> DECAY
    STABLE --> DECAY
    DECAY --> REMOVE
    DECAY --> GOOD
```

### 3.3 Emergent Topology

Over time, the network develops structure:

```mermaid
graph TB
    subgraph "Initial State"
        direction LR
        I1((1))
        I2((2))
        I3((3))
        I4((4))
        I5((5))
        I1 -.- I2
        I2 -.- I3
        I3 -.- I4
        I4 -.- I5
    end

    subgraph "Evolved State"
        direction LR
        E1((1))
        E2((2))
        E3((3<br/>HUB))
        E4((4))
        E5((5))
        E1 ---|strong| E3
        E2 ---|strong| E3
        E3 ===|very strong| E4
        E3 ---|medium| E5
        E1 -.-|weak| E2
        E4 ---|strong| E5
    end

    style E3 fill:#FFD700
```

**Emergent Properties:**
- **Hubs** emerge naturally from high-quality, well-connected nodes
- **Clusters** form around nodes with similar capabilities
- **Weak links** decay and disappear
- **Strong links** persist and strengthen

---

## 4. Network Properties

### 4.1 Small-World Property

Symbiont networks tend to exhibit small-world characteristics:

| Property | Description | Benefit |
|----------|-------------|---------|
| High clustering | Local neighborhoods well-connected | Redundancy |
| Short path lengths | Few hops between any two nodes | Fast signal propagation |
| Hub emergence | Some nodes become highly connected | Efficient routing |

```mermaid
graph TB
    subgraph "Small-World Network"
        H1((Hub 1))
        H2((Hub 2))

        N1((N1))
        N2((N2))
        N3((N3))
        N4((N4))
        N5((N5))
        N6((N6))

        H1 --- N1
        H1 --- N2
        H1 --- N3
        H1 === H2
        H2 --- N4
        H2 --- N5
        H2 --- N6

        N1 --- N2
        N2 --- N3
        N4 --- N5
        N5 --- N6
    end

    style H1 fill:#FFD700
    style H2 fill:#FFD700
```

### 4.2 Scale-Free Tendency

Connection degree distribution tends toward power-law:
- Most nodes have few connections
- Few nodes (hubs) have many connections

This emerges from **preferential attachment**: high-trust nodes receive more task requests, forming more connections.

### 4.3 Resilience

The network is resilient to:

| Failure Type | Network Response |
|--------------|------------------|
| Random node failure | Routes around; minimal impact |
| Hub failure | Degraded but functional; other hubs compensate |
| Adversary injection | Defense signals propagate; adversary isolated |
| Network partition | Each partition continues functioning |

---

## 5. Network Operations

### 5.1 Node Discovery

New nodes discover the network through:

```mermaid
graph LR
    subgraph "Discovery Methods"
        B[Bootstrap Nodes<br/>Well-known entry points]
        M[Multicast/Broadcast<br/>Local network discovery]
        R[Referral<br/>Introduced by existing node]
        D[DHT<br/>Distributed hash table]
    end

    NEW[New Node] --> B
    NEW --> M
    NEW --> R
    NEW --> D
```

### 5.2 Connection Management

Each node manages connections locally:

```
FUNCTION manage_connections(self):

    // Prune dead connections
    FOR EACH conn IN self.connections:
        IF conn.w < W_MIN:
            self.connections.remove(conn.partner_id)
        ELSE IF (now() - conn.last_active) > MAX_IDLE:
            // Probe to check if still alive
            IF NOT probe(conn.partner_id):
                self.connections.remove(conn.partner_id)

    // Consider new connections (e.g., from recommendations)
    FOR EACH candidate IN get_recommended_connections():
        IF should_connect(candidate):
            self.connections[candidate.id] = new_connection(candidate.id)
```

### 5.3 Network Partitioning

If the network partitions:

```mermaid
graph TB
    subgraph "Partition A"
        A1((A1))
        A2((A2))
        A3((A3))
        A1 --- A2
        A2 --- A3
        A1 --- A3
    end

    subgraph "Partition B"
        B1((B1))
        B2((B2))
        B3((B3))
        B1 --- B2
        B2 --- B3
        B1 --- B3
    end

    A3 -.-|broken| B1
```

**Behavior:**
- Each partition operates independently
- Trust scores remain valid within partition
- Cross-partition connections decay
- Upon reconnection, connections re-establish gradually

---

## 6. Defense Signal Propagation

### 6.1 Signal Flow Through Network

Defense signals propagate through trusted connections:

```mermaid
graph TB
    subgraph "Signal Propagation"
        O((Origin<br/>conf=1.0))
        H1((Hop 1<br/>conf=0.8))
        H2A((Hop 2a<br/>conf=0.64))
        H2B((Hop 2b<br/>conf=0.64))
        H3((Hop 3<br/>conf=0.51))

        O -->|w=1.0| H1
        H1 -->|w=1.0| H2A
        H1 -->|w=1.0| H2B
        H2A -->|w=1.0| H3
    end

    style O fill:#FF6B6B
```

### 6.2 Propagation Rules

```
confidence_at_hop = confidence_previous × DECAY_PER_HOP × w_connection

WHERE:
    DECAY_PER_HOP = 0.8
    MAX_HOPS = 5
    MIN_SIGNAL = 0.1
```

### 6.3 Multi-Path Aggregation

When signals arrive from multiple paths:

```mermaid
graph TB
    THREAT[Threat Node]

    A((A<br/>Detector))
    B((B))
    C((C))
    TARGET((Target))

    THREAT -.->|detected by| A
    A -->|signal| B
    A -->|signal| C
    B -->|signal| TARGET
    C -->|signal| TARGET

    style THREAT fill:#FF6B6B
    style TARGET fill:#FFD700
```

**Target's Belief Update:**
- Receives signal from B (confidence c₁)
- Receives signal from C (confidence c₂)
- Final belief: Bayesian aggregation of both

---

## 7. Routing in the Network

### 7.1 Local Routing Decisions

Each node makes routing decisions based on local information:

```mermaid
graph TB
    subgraph "Routing Decision at Node A"
        A[Node A<br/>Has task]

        B[Node B<br/>T=0.8, q_cap=0.9<br/>w=0.7, load=0.3]
        C[Node C<br/>T=0.7, q_cap=0.95<br/>w=0.5, load=0.1]
        D[Node D<br/>T=0.9, q_cap=0.85<br/>w=0.4, load=0.5]

        A --> B
        A --> C
        A --> D
    end

    subgraph "Scores"
        SB[B: 0.8 × 0.9 × 0.7 × 0.7 = 0.353]
        SC[C: 0.7 × 0.95 × 0.9 × 0.5 = 0.299]
        SD[D: 0.9 × 0.85 × 0.5 × 0.4 = 0.153]
    end

    B --> SB
    C --> SC
    D --> SD

    style SB fill:#98FB98
```

**Result:** Task routes to Node B (highest score).

### 7.2 Multi-Hop Routing

For workflows, routing decisions cascade:

```mermaid
sequenceDiagram
    participant O as Origin
    participant A as Node A
    participant B as Node B
    participant C as Node C

    O->>A: Task Step 1
    Note over A: Execute, route Step 2
    A->>B: Task Step 2
    Note over B: Execute, route Step 3
    B->>C: Task Step 3
    Note over C: Execute, return
    C->>B: Result 3
    B->>A: Result 2+3
    A->>O: Final Result
```

**Key Point:** Each node routes from its own perspective, using its own connection weights and trust assessments.

---

## 8. Network Health Metrics

### 8.1 Health Indicators

| Metric | Healthy Range | Warning Signs |
|--------|---------------|---------------|
| Mean connection weight | 0.5 - 0.8 | < 0.3 or > 0.95 |
| Network diameter | 3 - 6 hops | > 10 hops |
| Hub count | 5-15% of nodes | < 2% or > 30% |
| Mean diversity | > 0.4 | < 0.2 |
| Active defense signals | < 10% of nodes | > 30% |

### 8.2 Network Visualization

```mermaid
graph TB
    subgraph "Healthy Network"
        H1((Hub))
        N1((N1))
        N2((N2))
        N3((N3))
        N4((N4))
        N5((N5))

        H1 ===|0.9| N1
        H1 ===|0.85| N2
        H1 ---|0.7| N3
        N1 ---|0.6| N2
        N2 ---|0.65| N3
        N3 ---|0.55| N4
        N4 ---|0.7| N5
        N1 ---|0.5| N4
    end

    style H1 fill:#FFD700
```

Characteristics:
- Clear hub structure
- Mix of strong and medium connections
- Good path redundancy

---

## 9. Network Scenarios

### 9.1 New Node Joining

```mermaid
sequenceDiagram
    participant NEW as New Node
    participant BOOT as Bootstrap
    participant N1 as Node 1
    participant N2 as Node 2

    NEW->>BOOT: Request peers
    BOOT->>NEW: Peer list [N1, N2, ...]

    NEW->>N1: Hello + capabilities
    N1->>NEW: Welcome + capabilities
    Note over NEW,N1: Connection established<br/>w = 0.3

    NEW->>N2: Hello + capabilities
    N2->>NEW: Welcome + capabilities
    Note over NEW,N2: Connection established<br/>w = 0.3

    Note over NEW: Status: PROBATIONARY<br/>Trust: ~0.4 (swift trust)
```

### 9.2 Hub Emergence

```mermaid
graph TB
    subgraph "Time T1"
        direction LR
        T1A((A))
        T1B((B))
        T1C((C))
        T1A --- T1B
        T1B --- T1C
    end

    subgraph "Time T2"
        direction LR
        T2A((A))
        T2B((B))
        T2C((C))
        T2D((D))
        T2A --- T2B
        T2B === T2C
        T2C --- T2D
        T2A --- T2C
    end

    subgraph "Time T3"
        direction LR
        T3A((A))
        T3B((B))
        T3C((C<br/>HUB))
        T3D((D))
        T3E((E))
        T3A === T3C
        T3B === T3C
        T3C === T3D
        T3C --- T3E
        T3D --- T3E
    end

    style T3C fill:#FFD700
```

Node C becomes a hub through:
1. High-quality interactions
2. Good reciprocity
3. Diverse connections
4. Reliable availability

### 9.3 Adversary Isolation

```mermaid
graph TB
    subgraph "Before Detection"
        B1((Normal))
        B2((Normal))
        BAD1((Adversary))
        B3((Normal))

        B1 ---|0.6| BAD1
        BAD1 ---|0.5| B2
        B2 ---|0.7| B3
        B1 ---|0.4| B2
    end

    subgraph "After Defense Signals"
        A1((Normal))
        A2((Normal))
        BAD2((Adversary))
        A3((Normal))

        A1 -.-|0.1| BAD2
        BAD2 -.-|0.1| A2
        A2 ===|0.8| A3
        A1 ===|0.75| A2
    end

    style BAD1 fill:#FF6B6B
    style BAD2 fill:#FF6B6B
```

---

## 10. Summary

Symbiont networks are:

| Property | Description |
|----------|-------------|
| **Self-organizing** | Topology emerges from interaction patterns |
| **Weighted** | Connection strength varies based on history |
| **Dynamic** | Constantly evolving through Physarum dynamics |
| **Resilient** | Tolerates failures and attacks |
| **Efficient** | Small-world properties enable fast routing |

The network structure is not designed—it **emerges** from the collective behavior of nodes following simple local rules.

---

*Previous: [Node Architecture](./node.md) | Next: [Data Flow](./data-flow.md)*
