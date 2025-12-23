# Mathematical Functions

**Document Version:** 1.0
**Last Updated:** December 2025
**Status:** Normative

---

## 1. Introduction

This document specifies all mathematical functions used in Symbiont. Each function includes its mathematical definition, domain, range, and reference implementation.

---

## 2. Sigmoid Functions

### 2.1 Reciprocity Sigmoid

**Purpose:** Maps unbounded reciprocity score to [-1, 1].

**Definition:**
$$\sigma(r) = \frac{2}{1 + e^{-\beta r}} - 1$$

**Parameters:**

- β (beta) = 2.0 (reciprocity sensitivity)

**Domain:** r ∈ (-∞, +∞)

**Range:** σ(r) ∈ (-1, +1)

**Properties:**

- σ(0) = 0
- σ(r) → 1 as r → +∞
- σ(r) → -1 as r → -∞
- Antisymmetric: σ(-r) = -σ(r)

**Implementation:**

```rust
pub fn reciprocity_sigmoid(r: f64) -> f64 {
    const BETA: f64 = 2.0;
    let x = -BETA * r;

    // Overflow protection
    if x > 700.0 { return -1.0; }
    if x < -700.0 { return 1.0; }

    (2.0 / (1.0 + x.exp())) - 1.0
}
```

**Test Values:**

| Input r | Output σ(r) |
|---------|-------------|
| -2.0 | -0.964 |
| -1.0 | -0.762 |
| -0.5 | -0.462 |
| 0.0 | 0.000 |
| 0.5 | 0.462 |
| 1.0 | 0.762 |
| 2.0 | 0.964 |

---

### 2.2 Standard Sigmoid

**Purpose:** Maps any value to (0, 1).

**Definition:**
$$sigmoid(x) = \frac{1}{1 + e^{-x}}$$

**Domain:** x ∈ (-∞, +∞)

**Range:** sigmoid(x) ∈ (0, 1)

**Implementation:**

```rust
pub fn sigmoid(x: f64) -> f64 {
    if x > 700.0 { return 1.0; }
    if x < -700.0 { return 0.0; }

    1.0 / (1.0 + (-x).exp())
}
```

---

## 3. Multiplier Functions

### 3.1 Quality Multiplier ψ(q)

**Purpose:** Scales reinforcement based on interaction quality.

**Definition:**
$$\psi(q) = 0.5 + q$$

**Domain:** q ∈ [0, 1]

**Range:** ψ(q) ∈ [0.5, 1.5]

**Implementation:**

```rust
pub fn quality_multiplier(q: f64) -> f64 {
    0.5 + q.clamp(0.0, 1.0)
}
```

**Interpretation:**

| Quality | ψ(q) | Effect |
|---------|------|--------|
| 0.0 (poor) | 0.5 | Halves reinforcement |
| 0.5 (neutral) | 1.0 | No modification |
| 1.0 (excellent) | 1.5 | 50% boost |

---

### 3.2 Tone Multiplier φ(τ)

**Purpose:** Adjusts reinforcement based on interaction tone.

**Definition:**
$$\phi(\tau) = 0.7 + 0.3 \cdot \tau$$

**Domain:** τ ∈ [-1, 1]

**Range:** φ(τ) ∈ [0.4, 1.0]

**Implementation:**

```rust
pub fn tone_multiplier(tau: f64) -> f64 {
    0.7 + 0.3 * tau.clamp(-1.0, 1.0)
}
```

**Interpretation:**

| Tone | φ(τ) | Effect |
|------|------|--------|
| -1.0 (hostile) | 0.4 | 60% reduction |
| 0.0 (neutral) | 0.7 | 30% reduction |
| 1.0 (collaborative) | 1.0 | Full reinforcement |

---

## 4. Reinforcement Function

### 4.1 Physarum Reinforcement Φ

**Purpose:** Computes the reinforcement term for connection weight update.

**Definition:**
$$\Phi(Q, r, q, \tau) = \gamma \cdot |Q|^\mu \cdot \sigma(r) \cdot \psi(q) \cdot \phi(\tau)$$

**Parameters:**

- γ (gamma) = 0.1 (reinforcement rate)
- μ (mu) = 0.5 (flow exponent)

**Implementation:**

```rust
pub fn compute_reinforcement(
    volume: f64,
    reciprocity: f64,
    quality: f64,
    tone: f64,
) -> f64 {
    const GAMMA: f64 = 0.1;
    const MU: f64 = 0.5;

    let flow = GAMMA * volume.abs().powf(MU);
    let sigma_r = reciprocity_sigmoid(reciprocity);
    let psi_q = quality_multiplier(quality);
    let phi_tau = tone_multiplier(tone);

    flow * sigma_r * psi_q * phi_tau
}
```

---

## 5. Weight Update

### 5.1 Complete Weight Update

**Definition:**
$$w_{new} = clamp(w + \Phi - \alpha \cdot w - D, W_{MIN}, W_{MAX})$$

Where:

- Φ = reinforcement term
- α = 0.01 (decay rate)
- D = δ × θ_threat (defense dampening)
- W_MIN = 0.01, W_MAX = 1.0

**Implementation:**

```rust
pub fn update_weight(
    current_weight: f64,
    reinforcement: f64,
    threat_level: f64,
) -> f64 {
    const ALPHA: f64 = 0.01;
    const DELTA: f64 = 0.2;
    const W_MIN: f64 = 0.01;
    const W_MAX: f64 = 1.0;

    let decay = ALPHA * current_weight;
    let defense = DELTA * threat_level;

    let delta_w = reinforcement - decay - defense;
    let new_weight = current_weight + delta_w;

    new_weight.clamp(W_MIN, W_MAX)
}
```

---

## 6. Reciprocity Update

### 6.1 Reciprocity Score Update

**Definition:**
$$r_{new} = \lambda \cdot r_{old} + (1 - \lambda) \cdot \left(\log\left(\frac{in}{out + \epsilon}\right) + \theta \cdot (q - 0.5)\right)$$

**Parameters:**

- λ (lambda) = 0.9 (memory factor)
- θ (theta) = 0.5 (quality weight)
- ε (epsilon) = 0.001 (safety constant)

**Implementation:**

```rust
pub fn update_reciprocity(
    current_r: f64,
    exchange_in: f64,
    exchange_out: f64,
    quality: f64,
) -> f64 {
    const LAMBDA: f64 = 0.9;
    const THETA: f64 = 0.5;
    const EPSILON: f64 = 0.001;

    let rho = exchange_in / (exchange_out + EPSILON);
    let log_rho = (rho + EPSILON).ln();
    let quality_adj = THETA * (quality - 0.5);

    let signal = log_rho + quality_adj;

    LAMBDA * current_r + (1.0 - LAMBDA) * signal
}
```

---

## 7. Trust Computation

### 7.1 Trust Aggregation

**Definition:**
$$T(n) = \frac{w_Q \cdot Q_{agg} + w_R \cdot R_{trust} + w_S \cdot S_{social} + w_D \cdot D}{\sum w}$$

**Weights:** w_Q = 0.4, w_R = 0.2, w_S = 0.2, w_D = 0.2

**Implementation:**

```rust
pub fn compute_trust(
    q_agg: f64,
    r_agg: f64,
    s_social: f64,
    diversity: f64,
) -> f64 {
    const W_Q: f64 = 0.4;
    const W_R: f64 = 0.2;
    const W_S: f64 = 0.2;
    const W_D: f64 = 0.2;

    // Convert reciprocity to [0, 1]
    let r_trust = (reciprocity_sigmoid(r_agg) + 1.0) / 2.0;

    let trust = W_Q * q_agg + W_R * r_trust + W_S * s_social + W_D * diversity;

    // Apply diversity cap
    let capped = trust.min(diversity + 0.3);

    capped.clamp(0.0, 1.0)
}
```

---

## 8. Signal Attenuation

### 8.1 Defense Signal Decay

**Definition:**
$$confidence_{hop_n} = confidence_{origin} \times (DECAY\_PER\_HOP)^n \times w_{connection}$$

**Parameters:**

- DECAY_PER_HOP = 0.8
- n = hop count

**Implementation:**

```rust
pub fn attenuate_signal(
    origin_confidence: f64,
    hops: u8,
    connection_weight: f64,
) -> f64 {
    const DECAY_PER_HOP: f64 = 0.8;

    origin_confidence * DECAY_PER_HOP.powi(hops as i32) * connection_weight
}
```

---

## 9. Bayesian Belief Update

### 9.1 Threat Belief Update

**Definition:**
$$belief_{new} = belief_{old} + weight \times (1 - belief_{old})$$

Where weight = T_sender × signal_confidence

**Properties:**

- Approaches but never exceeds 1.0
- Multiple signals compound with diminishing returns
- Trust-weighted by sender

**Implementation:**

```rust
pub fn bayesian_belief_update(
    current_belief: f64,
    sender_trust: f64,
    signal_confidence: f64,
) -> f64 {
    let weight = sender_trust * signal_confidence;
    let new_belief = current_belief + weight * (1.0 - current_belief);
    new_belief.clamp(0.0, 1.0)
}
```

---

## 10. Convergence Score

### 10.1 Convergence Computation

**Definition:**
$$Conv(task) = 1 - \frac{Var(positions)}{Var_{max}}$$

**Implementation:**

```rust
pub fn compute_convergence(positions: &[f64], max_variance: f64) -> f64 {
    if positions.len() <= 1 {
        return 1.0;  // Single position = converged
    }

    let variance = compute_variance(positions);
    let convergence = 1.0 - (variance / max_variance);

    convergence.clamp(0.0, 1.0)
}

pub fn compute_variance(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    if n == 0.0 { return 0.0; }

    let mean = values.iter().sum::<f64>() / n;
    let sum_sq_diff: f64 = values.iter()
        .map(|x| (x - mean).powi(2))
        .sum();

    sum_sq_diff / n
}
```

---

## 11. Routing Score

### 11.1 Candidate Scoring

**Definition:**
$$S_{route}(n) = T(n) \cdot q_{cap}(n) \cdot (1 - load) \cdot w_{conn} \cdot (1 - \theta_{threat})$$

**Implementation:**

```rust
pub fn compute_routing_score(
    trust: f64,
    capability_quality: f64,
    load: f64,
    connection_weight: f64,
    threat_level: f64,
) -> f64 {
    trust * capability_quality * (1.0 - load) * connection_weight * (1.0 - threat_level)
}
```

---

## 12. Helper Functions

### 12.1 Clamp

```rust
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min { min }
    else if value > max { max }
    else { value }
}
```

### 12.2 Safe Division

```rust
pub fn safe_div(a: f64, b: f64) -> f64 {
    const EPSILON: f64 = 0.001;
    a / (b + EPSILON)
}
```

### 12.3 Safe Logarithm

```rust
pub fn safe_log(x: f64) -> f64 {
    const EPSILON: f64 = 0.001;
    (x.max(EPSILON)).ln()
}
```

### 12.4 Exponential Moving Average

```rust
pub fn ema(old_value: f64, new_value: f64, alpha: f64) -> f64 {
    alpha * old_value + (1.0 - alpha) * new_value
}
```

---

## 13. Numerical Stability

### 13.1 Overflow Prevention

All exponential functions should guard against overflow:

```rust
// Safe sigmoid
if x > 700.0 { return 1.0; }
if x < -700.0 { return 0.0; }
```

### 13.2 Division Safety

Always add ε to denominators:

```rust
let ratio = numerator / (denominator + EPSILON);
```

### 13.3 Bounds Checking

Always clamp final results:

```rust
result.clamp(MIN, MAX)
```

---

## 14. Summary

| Function | Formula | Range |
|----------|---------|-------|
| reciprocity_sigmoid | (2/(1+e^(-βr)))-1 | [-1, 1] |
| quality_multiplier | 0.5 + q | [0.5, 1.5] |
| tone_multiplier | 0.7 + 0.3τ | [0.4, 1.0] |
| reinforcement | γ\|Q\|^μ σ(r) ψ(q) φ(τ) | unbounded |
| weight_update | w + Φ - αw - D | [W_MIN, W_MAX] |
| trust | weighted sum with caps | [0, 1] |
| routing_score | T × q × (1-load) × w × (1-θ) | [0, 1] |

---

*Previous: [Constants](./constants.md) | Next: [Node API](./node.md)*
