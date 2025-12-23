# Protocol Constants

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

This document defines all protocol constants used in Symbiont. Constants are organized by functional category. Default values are provided; implementations may allow configuration within specified ranges.

---

## 2. Connection Dynamics

### 2.1 Physarum Equation Constants

| Constant | Symbol | Default | Range | Description |
|----------|--------|---------|-------|-------------|
| GAMMA | γ | 0.1 | [0.01, 0.5] | Reinforcement rate |
| MU | μ | 0.5 | [0.3, 0.8] | Flow exponent (sublinear) |
| ALPHA | α | 0.01 | [0.001, 0.1] | Decay rate |
| BETA | β | 2.0 | [1.0, 4.0] | Reciprocity sensitivity |
| LAMBDA | λ | 0.9 | [0.8, 0.99] | Memory factor (EMA) |
| THETA | θ | 0.5 | [0.3, 0.7] | Quality weight in reciprocity |
| DELTA | δ | 0.2 | [0.1, 0.5] | Defense dampening factor |
| EPSILON | ε | 0.001 | [0.0001, 0.01] | Division safety constant |

### 2.2 Connection Bounds

| Constant | Value | Description |
|----------|-------|-------------|
| W_MIN | 0.01 | Minimum connection weight |
| W_MAX | 1.0 | Maximum connection weight |
| W_INIT | 0.3 | Initial connection weight |

### 2.3 Usage

```rust
// Physarum equation implementation
let flow = GAMMA * volume.abs().powf(MU);
let sigma_r = reciprocity_sigmoid(r, BETA);
let psi_q = 0.5 + quality;
let phi_tau = 0.7 + 0.3 * tone;

let phi = flow * sigma_r * psi_q * phi_tau;
let delta_w = phi - ALPHA * current_weight - defense_dampening;

new_weight = (current_weight + delta_w).clamp(W_MIN, W_MAX);
```

---

## 3. Cold Start

### 3.1 Swift Trust Constants

| Constant | Value | Description |
|----------|-------|-------------|
| SWIFT_TRUST_BASE | 0.4 | Base trust for new nodes |
| VOUCH_PENALTY | 0.5 | Trust reduction for voucher if vouched node misbehaves |

### 3.2 Probation Constants

| Constant | Value | Description |
|----------|-------|-------------|
| PROBATION_COUNT | 50 | Interactions to complete probation |
| PROBATION_THRESHOLD | 0.6 | Quality needed to pass probation |
| PROBATION_EXTENSION | 25 | Additional interactions if failed |
| MAX_PROBATION_FAILURES | 3 | Failures before expulsion |

### 3.3 Swift Trust Computation

```
T_init = 0.3 × SWIFT_TRUST_BASE +
         0.2 × T_category +
         0.3 × T_vouch +
         0.2 × T_social
```

---

## 4. Quality Assessment

### 4.1 Quality Dimension Weights

| Constant | Symbol | Value | Description |
|----------|--------|-------|-------------|
| WEIGHT_HELPFULNESS | ω_help | 0.4 | Weight for helpfulness |
| WEIGHT_ACCURACY | ω_acc | 0.3 | Weight for accuracy |
| WEIGHT_RELEVANCE | ω_rel | 0.2 | Weight for relevance |
| WEIGHT_TIMELINESS | ω_time | 0.1 | Weight for timeliness |

### 4.2 Quality Modifiers

| Constant | Value | Description |
|----------|-------|-------------|
| REUSE_BOOST | 1.2 | Multiplier when would_reuse = true |
| REUSE_PENALTY | 0.8 | Multiplier when would_reuse = false |

### 4.3 Quality Calculation

```
Q_raw = (ω_help × helpfulness +
         ω_acc × accuracy +
         ω_rel × relevance +
         ω_time × timeliness) / 4

IF would_reuse:
    Q_raw *= REUSE_BOOST
ELSE:
    Q_raw *= REUSE_PENALTY

Q = (Q_raw - 1) / 4  // Normalize to [0, 1]
```

---

## 5. Trust Computation

### 5.1 Trust Component Weights

| Constant | Symbol | Value | Description |
|----------|--------|-------|-------------|
| WEIGHT_QUALITY | w_Q | 0.4 | Quality component weight |
| WEIGHT_RECIPROCITY | w_R | 0.2 | Reciprocity component weight |
| WEIGHT_SOCIAL | w_S | 0.2 | Social proof component weight |
| WEIGHT_DIVERSITY | w_D | 0.2 | Diversity component weight |

### 5.2 Diversity Constants

| Constant | Value | Description |
|----------|-------|-------------|
| DIVERSITY_THRESHOLD | 0.3 | Minimum diversity for full trust |
| DIVERSITY_WINDOW | 100 | Interactions to consider for diversity |
| DIVERSITY_CAP_OFFSET | 0.3 | Added to diversity for trust cap |

### 5.3 Trust Computation

```
T = (w_Q × Q_agg + w_R × R_trust + w_S × S_social + w_D × D) / Σw

// Apply diversity cap
T = min(T, D + DIVERSITY_CAP_OFFSET)

// Apply trust cap from flags
T = min(T, trust_cap)
```

---

## 6. Defense Signaling

### 6.1 Signal Propagation

| Constant | Value | Description |
|----------|-------|-------------|
| PROPAGATE_THRESHOLD | 0.6 | Min confidence to forward signal |
| DECAY_PER_HOP | 0.8 | Confidence decay per network hop |
| MIN_SIGNAL | 0.1 | Minimum signal strength |
| MAX_HOPS | 5 | Maximum propagation depth |

### 6.2 Priming

| Constant | Value | Description |
|----------|-------|-------------|
| PRIMING_SENSITIVITY | 0.1 | Priming increase per signal |
| PRIMING_DECAY | 0.99 | Priming decay per tick |

### 6.3 Action Thresholds

| Constant | Value | Description |
|----------|-------|-------------|
| ACTION_THRESHOLD | 0.7 | Threat level triggering action |
| SEVERE_THRESHOLD | 0.9 | Threat level for blocking |

### 6.4 Signal Attenuation Formula

```
confidence_hop_n = confidence_origin × (DECAY_PER_HOP ^ n) × w_connection

IF confidence_hop_n < MIN_SIGNAL:
    STOP propagation

IF confidence_hop_n < PROPAGATE_THRESHOLD:
    STOP propagation
```

---

## 7. Detection

### 7.1 Adversary Detection

| Constant | Value | Description |
|----------|-------|-------------|
| ADVERSARY_DROP | 0.3 | Quality drop indicating defection |
| ADVERSARY_VARIANCE_THRESHOLD | 0.01 | Suspiciously low variance |
| ADVERSARY_HISTORY_WINDOW | 100 | Interactions to analyze |

### 7.2 Collusion Detection

| Constant | Value | Description |
|----------|-------|-------------|
| COLLUSION_THRESHOLD | 0.85 | Internal density for suspicion |
| COLLUSION_EXTERNAL_RATIO | 1.0 | Max external ratio for suspicion |
| COLLUSION_MUTUAL_RATING | 0.9 | Suspiciously high mutual ratings |
| MIN_COLLUSION_SIZE | 3 | Minimum cluster size to consider |

---

## 8. Self-Confidence

### 8.1 Confidence Constants

| Constant | Value | Description |
|----------|-------|-------------|
| CONFIDENCE_MEMORY | 0.95 | EMA factor for confidence |
| CONFIDENCE_INIT | 0.5 | Initial self-confidence |

### 8.2 Confidence Update

```
S_conf_new = CONFIDENCE_MEMORY × S_conf_old +
             (1 - CONFIDENCE_MEMORY) × A_mean
```

---

## 9. Status Transitions

### 9.1 Promotion Requirements

| Transition | Requirements |
|------------|--------------|
| PROBATIONARY → MEMBER | count ≥ 50, quality ≥ 0.6 |
| MEMBER → ESTABLISHED | count ≥ 200, quality ≥ 0.8 |
| ESTABLISHED → HUB | connections ≥ 20, diversity ≥ 0.5, quality ≥ 0.85 |

### 9.2 Demotion Triggers

| Constant | Value | Description |
|----------|-------|-------------|
| QUALITY_DROP_THRESHOLD | 0.4 | Quality drop triggering demotion |
| TRUST_DEMOTION_THRESHOLD | 0.3 | Trust level triggering demotion |
| DEMOTION_TRUST_MULTIPLIER | 0.5 | Trust multiplier on demotion |

---

## 10. Convergence

### 10.1 Convergence States

| Constant | Value | Description |
|----------|-------|-------------|
| CONVERGED_THRESHOLD | 0.85 | Score for CONVERGED state |
| CONVERGING_THRESHOLD | 0.60 | Score for CONVERGING state |
| EXPLORING_THRESHOLD | 0.40 | Score for EXPLORING state |
| STUCK_THRESHOLD | 0.20 | Score for STUCK state |

### 10.2 Agree-to-Disagree

| Constant | Value | Description |
|----------|-------|-------------|
| ATD_MIN_ROUNDS | 5 | Minimum rounds before ATD |
| ATD_CRITICALITY_THRESHOLD | 0.8 | Criticality above which ATD disabled |

---

## 11. Timing

### 11.1 Timeouts

| Constant | Value | Description |
|----------|-------|-------------|
| DEFAULT_TASK_TIMEOUT | 30000 | Default task timeout (ms) |
| HANDOFF_TIMEOUT | 10000 | Handoff acceptance timeout (ms) |
| SIGNAL_MAX_AGE | 3600000 | Maximum signal age (1 hour, ms) |

### 11.2 Background Jobs

| Constant | Value | Description |
|----------|-------|-------------|
| IDLE_THRESHOLD | 86400000 | Connection idle threshold (24h, ms) |
| DIVERSITY_CHECK_INTERVAL | 100 | Ticks between diversity checks |
| STATUS_CHECK_INTERVAL | 50 | Ticks between status checks |
| ADVERSARY_SCAN_INTERVAL | 200 | Ticks between adversary scans |

---

## 12. Limits

### 12.1 Size Limits

| Constant | Value | Description |
|----------|-------|-------------|
| MAX_CONNECTIONS | 1000 | Maximum connections per node |
| MAX_CAPABILITIES | 100 | Maximum capabilities per node |
| MAX_WORKFLOW_STEPS | 50 | Maximum steps per workflow |
| MAX_RETRY_COUNT | 3 | Maximum retries per step |

### 12.2 Message Limits

| Constant | Value | Description |
|----------|-------|-------------|
| MAX_EVIDENCE_SIZE | 4096 | Maximum evidence bytes |
| MAX_CONTEXT_SIZE | 1048576 | Maximum context size (1MB) |

---

## 13. Implementation Notes

### 13.1 Rust Constants Module

```rust
pub mod constants {
    // Connection dynamics
    pub const GAMMA: f64 = 0.1;
    pub const MU: f64 = 0.5;
    pub const ALPHA: f64 = 0.01;
    pub const BETA: f64 = 2.0;
    pub const LAMBDA: f64 = 0.9;
    pub const THETA: f64 = 0.5;
    pub const DELTA: f64 = 0.2;
    pub const EPSILON: f64 = 0.001;

    // Connection bounds
    pub const W_MIN: f64 = 0.01;
    pub const W_MAX: f64 = 1.0;
    pub const W_INIT: f64 = 0.3;

    // Cold start
    pub const SWIFT_TRUST_BASE: f64 = 0.4;
    pub const PROBATION_COUNT: u32 = 50;
    pub const PROBATION_THRESHOLD: f64 = 0.6;

    // ... etc
}
```

---

## 14. Summary

Constants are organized into categories:

| Category | Key Constants |
|----------|---------------|
| Dynamics | γ, μ, α, β, λ, θ, δ, ε |
| Bounds | W_MIN, W_MAX, W_INIT |
| Cold Start | SWIFT_TRUST_BASE, PROBATION_* |
| Quality | ω_help, ω_acc, ω_rel, ω_time |
| Trust | w_Q, w_R, w_S, w_D |
| Defense | DECAY_PER_HOP, MAX_HOPS, ACTION_THRESHOLD |
| Timing | Timeouts, intervals |
| Limits | MAX_* constraints |

---

*Previous: [Core Types](./types.md) | Next: [Mathematical Functions](./math.md)*
