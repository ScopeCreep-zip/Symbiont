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
```

## 1.4 Interaction Structure

```
STRUCTURE Interaction {
    initiator    : NodeId
    responder    : NodeId
    task_volume  : float64             // Q - amount of work/value
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
┌─────────────────────────────────────────────────────────────┐
│                  Symbiont NODE RUNTIME                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│   ┌─────────────┐   ┌─────────────┐   ┌─────────────┐      │
│   │ INTERACTION │   │   EVENT     │   │ BACKGROUND  │      │
│   │    LOOP     │   │  HANDLERS   │   │    JOBS     │      │
│   └──────┬──────┘   └──────┬──────┘   └──────┬──────┘      │
│          │                 │                 │              │
│          └────────────────┬┴─────────────────┘              │
│                           │                                 │
│                           ▼                                 │
│                    ┌─────────────┐                          │
│                    │ NODE STATE  │                          │
│                    └─────────────┘                          │
│                                                             │
└─────────────────────────────────────────────────────────────┘

Three concurrent processes:
1. INTERACTION LOOP  — Handles tasks, updates connections
2. EVENT HANDLERS    — Reacts to signals, affirmations, votes
3. BACKGROUND JOBS   — Periodic decay, scans, status checks
```

## 4.2 Node Initialization

```
FUNCTION initialize_node(node_id, role, vouchers):
    
    // Compute initial trust (Swift Trust)
    T_swift    = SWIFT_TRUST_BASE                              // 0.4
    T_category = lookup_role_trust(role) × similarity_score()
    T_vouch    = Σ(voucher.trust × voucher.stake × time_decay(voucher.age))
    T_social   = sigmoid(interaction_quality_sum / network_average)
    
    T_init = 0.3 × T_swift + 0.2 × T_category + 0.3 × T_vouch + 0.2 × T_social
    
    RETURN Node {
        id          = node_id
        status      = PROBATIONARY
        trust       = T_init
        trust_cap   = 1.0
        confidence  = 0.5
        priming     = 0.0
        connections = {}
        // ... other fields initialized to defaults
    }
```

## 4.3 Main Interaction Handler

```
FUNCTION handle_interaction(self, partner, interaction):
    
    // ═══════════════════════════════════════════════════════════
    // STEP 1: Get or create connection
    // ═══════════════════════════════════════════════════════════
    
    IF partner.id NOT IN self.connections:
        self.connections[partner.id] = new_connection(partner.id)
    
    conn = self.connections[partner.id]
    
    // ═══════════════════════════════════════════════════════════
    // STEP 2: Measure interaction outcomes
    // ═══════════════════════════════════════════════════════════
    
    quality = compute_quality(interaction.feedback)
    tone    = compute_tone(interaction)
    ρ       = interaction.exchange_in / (interaction.exchange_out + ε)
    
    // ═══════════════════════════════════════════════════════════
    // STEP 3: Update reciprocity score
    // ═══════════════════════════════════════════════════════════
    
    log_rho     = log(ρ + ε)
    quality_adj = θ × (quality - 0.5)
    conn.r      = λ × conn.r + (1 - λ) × (log_rho + quality_adj)
    
    // ═══════════════════════════════════════════════════════════
    // STEP 4: Update quality and tone scores
    // ═══════════════════════════════════════════════════════════
    
    conn.q = λ × conn.q + (1 - λ) × quality
    conn.τ = λ × conn.τ + (1 - λ) × tone
    
    // ═══════════════════════════════════════════════════════════
    // STEP 5: Compute reinforcement Φ
    // ═══════════════════════════════════════════════════════════
    
    Q       = interaction.task_volume
    σ_r     = (2 / (1 + exp(-β × conn.r))) - 1
    ψ_q     = 0.5 + conn.q
    φ_τ     = 0.7 + 0.3 × conn.τ
    
    Φ       = γ × pow(Q, μ) × σ_r × ψ_q × φ_τ
    
    // ═══════════════════════════════════════════════════════════
    // STEP 6: Compute defense dampening
    // ═══════════════════════════════════════════════════════════
    
    D = 0
    IF partner.id IN self.threat_beliefs:
        D = δ × self.threat_beliefs[partner.id].level
    
    // ═══════════════════════════════════════════════════════════
    // STEP 7: Update connection weight (THE CORE EQUATION)
    // ═══════════════════════════════════════════════════════════
    
    Δw     = Φ - α × conn.w - D
    conn.w = clamp(conn.w + Δw, W_MIN, W_MAX)
    
    // ═══════════════════════════════════════════════════════════
    // STEP 8: Post-interaction actions
    // ═══════════════════════════════════════════════════════════
    
    // Maybe send affirmation
    IF quality > 0.8 AND tone > 0.5:
        send_affirmation(self, partner, quality)
    
    // Maybe emit defense signal
    IF should_warn(partner, conn):
        emit_defense_signal(self, partner, conn)
    
    // Update metadata
    conn.last_active = now()
    conn.count += 1
    
    RETURN success
```

## 4.4 Defense Signal Handler

```
FUNCTION handle_defense_signal(self, signal):
    
    // ═══════════════════════════════════════════════════════════
    // STEP 1: Update threat belief (Bayesian)
    // ═══════════════════════════════════════════════════════════
    
    current = self.threat_beliefs[signal.threat] OR default_belief()
    
    sender_trust = get_trust(signal.sender)
    weight       = sender_trust × signal.confidence
    
    new_level    = current.level + weight × (1 - current.level)
    
    self.threat_beliefs[signal.threat] = {
        level       = new_level
        threat_type = signal.threat_type
        evidence    = current.evidence + [signal.evidence]
        updated     = now()
    }
    
    // ═══════════════════════════════════════════════════════════
    // STEP 2: Increase priming (defense readiness)
    // ═══════════════════════════════════════════════════════════
    
    boost = signal.confidence × PRIMING_SENSITIVITY
    self.priming = min(1.0, self.priming + boost)
    
    // ═══════════════════════════════════════════════════════════
    // STEP 3: Maybe take defensive action
    // ═══════════════════════════════════════════════════════════
    
    IF new_level > ACTION_THRESHOLD:
        take_defensive_action(self, signal.threat, signal.threat_type)
    
    // ═══════════════════════════════════════════════════════════
    // STEP 4: Maybe propagate further
    // ═══════════════════════════════════════════════════════════
    
    IF signal.confidence > PROPAGATE_THRESHOLD AND signal.hops < MAX_HOPS:
        FOR EACH (partner_id, conn) IN self.connections:
            IF conn.w >= PROPAGATE_THRESHOLD AND partner_id != signal.sender:
                
                attenuated = signal.copy()
                attenuated.confidence = signal.confidence × DECAY_PER_HOP × conn.w
                attenuated.hops = signal.hops + 1
                attenuated.sender = self.id
                
                IF attenuated.confidence >= MIN_SIGNAL:
                    send_to(partner_id, attenuated)
```

## 4.5 Background Jobs

```
FUNCTION background_tick(self):
    
    // ═══════════════════════════════════════════════════════════
    // JOB 1: Decay priming (every tick)
    // ═══════════════════════════════════════════════════════════
    
    self.priming = self.priming × PRIMING_DECAY
    
    // ═══════════════════════════════════════════════════════════
    // JOB 2: Decay unused connections (every tick)
    // ═══════════════════════════════════════════════════════════
    
    FOR EACH conn IN self.connections:
        IF (now() - conn.last_active) > IDLE_THRESHOLD:
            conn.w = conn.w × (1 - α)
            IF conn.w < W_MIN:
                remove_connection(conn)
    
    // ═══════════════════════════════════════════════════════════
    // JOB 3: Check diversity (periodic)
    // ═══════════════════════════════════════════════════════════
    
    IF tick_count MOD DIVERSITY_INTERVAL == 0:
        unique = count_unique_partners(last_100_interactions)
        diversity = unique / 100
        
        IF diversity < DIVERSITY_THRESHOLD:
            self.trust_cap = 0.7
            self.flags.add(LOW_DIVERSITY)
        ELSE:
            self.trust_cap = 1.0
            self.flags.remove(LOW_DIVERSITY)
    
    // ═══════════════════════════════════════════════════════════
    // JOB 4: Check status transitions (periodic)
    // ═══════════════════════════════════════════════════════════
    
    IF tick_count MOD STATUS_INTERVAL == 0:
        check_status_transition(self)
    
    // ═══════════════════════════════════════════════════════════
    // JOB 5: Scan for adversaries (periodic)
    // ═══════════════════════════════════════════════════════════
    
    IF tick_count MOD ADVERSARY_INTERVAL == 0:
        FOR EACH partner IN self.connections:
            IF detect_strategic_adversary(partner):
                emit_defense_signal(self, partner, STRATEGIC)
```

## 4.6 Trust Computation

```
FUNCTION compute_trust(node):
    
    // Aggregate reciprocity
    IF node.connections is empty:
        R_agg = 0
    ELSE:
        R_agg = mean([conn.r FOR conn IN node.connections])
    
    // Diversity score
    unique = count_unique_partners(node.last_100_interactions)
    D = unique / 100
    
    // Social proof
    S = compute_social_proof(node)
    
    // Weighted sum
    T = (0.4 × node.quality_score + 
         0.2 × sigmoid(R_agg) + 
         0.2 × S + 
         0.2 × D)
    
    // Apply diversity cap
    T = min(T, D + 0.3)
    
    // Apply trust cap (from diversity requirements)
    T = min(T, node.trust_cap)
    
    RETURN T
```

## 4.7 Adversary Detection

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
    
    // Flag 1: Suspiciously perfect early behavior
    IF early_quality > 0.95 AND early_variance < 0.01:
        RETURN true
    
    // Flag 2: Quality drop after building trust
    IF node.trust > 0.7:
        IF (early_quality - recent_quality) > ADVERSARY_DROP:
            RETURN true
    
    RETURN false


FUNCTION detect_collusion(network):
    
    // Build interaction graph
    graph = build_interaction_graph(network)
    
    // Find dense communities
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

# SECTION 8: QUICK REFERENCE

## 8.1 The Core Equation

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

## 8.2 All Equations Summary

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

## 8.3 Default Parameters

```
γ=0.1  μ=0.5  α=0.01  β=2.0  λ=0.9  θ=0.5  δ=0.2  ε=0.001
W_MIN=0.01  W_MAX=1.0  W_INIT=0.3
SWIFT_TRUST=0.4  PROBATION=50  THRESHOLD=0.6
DECAY_PER_HOP=0.8  MAX_HOPS=5  ACTION_THRESHOLD=0.7
```

---

**END OF SPECIFICATION**

*Symbiont v0.1 — Implement in any language, deploy on any network.*
