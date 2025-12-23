//! Mathematical functions for the Symbiont protocol.
//!
//! Implements sigmoid functions, multipliers, and safe arithmetic.

use crate::constants::{BETA, EPSILON};
use crate::types::{Score, SignedScore};

// =============================================================================
// SIGMOID FUNCTIONS
// =============================================================================

/// Reciprocity sigmoid: maps unbounded reciprocity to [-1, 1]
///
/// σ(r) = (2 / (1 + e^(-β × r))) - 1
///
/// This is a scaled sigmoid that outputs:
/// - Negative values for negative reciprocity (giving more than receiving)
/// - Positive values for positive reciprocity (receiving more than giving)
/// - Zero at r = 0
pub fn reciprocity_sigmoid(r: f64) -> SignedScore {
    safe_reciprocity_sigmoid(r, BETA)
}

/// Safe reciprocity sigmoid with configurable β
/// Handles overflow for extreme values
pub fn safe_reciprocity_sigmoid(r: f64, beta: f64) -> SignedScore {
    let x = -beta * r;

    // Prevent overflow
    if x > 700.0 {
        return SignedScore::NEG_ONE;
    }
    if x < -700.0 {
        return SignedScore::ONE;
    }

    let result = (2.0 / (1.0 + x.exp())) - 1.0;
    SignedScore::new(result)
}

/// Standard sigmoid: maps to [0, 1]
///
/// sigmoid(x) = 1 / (1 + e^(-x))
pub fn sigmoid(x: f64) -> Score {
    if x > 700.0 {
        return Score::ONE;
    }
    if x < -700.0 {
        return Score::ZERO;
    }

    Score::new(1.0 / (1.0 + (-x).exp()))
}

// =============================================================================
// MULTIPLIER FUNCTIONS
// =============================================================================

/// Quality multiplier: maps quality [0,1] to [0.5, 1.5]
///
/// ψ(q) = 0.5 + q
///
/// - Quality 0 → multiplier 0.5 (halves reinforcement)
/// - Quality 0.5 → multiplier 1.0 (neutral)
/// - Quality 1 → multiplier 1.5 (50% boost)
pub fn quality_multiplier(q: Score) -> f64 {
    0.5 + q.value()
}

/// Tone multiplier: maps tone [-1,1] to [0.4, 1.0]
///
/// φ(τ) = 0.7 + (0.3 × τ)
///
/// - Tone -1 → multiplier 0.4 (60% reduction)
/// - Tone 0 → multiplier 0.7 (30% reduction)
/// - Tone +1 → multiplier 1.0 (no reduction)
pub fn tone_multiplier(tau: SignedScore) -> f64 {
    0.7 + (0.3 * tau.value())
}

// =============================================================================
// SAFE ARITHMETIC
// =============================================================================

/// Safe logarithm with epsilon protection
///
/// Returns log(max(x, ε))
pub fn safe_log(x: f64) -> f64 {
    (x.max(EPSILON)).ln()
}

/// Safe division with epsilon protection
///
/// Returns a / (b + ε)
pub fn safe_div(a: f64, b: f64) -> f64 {
    a / (b + EPSILON)
}

/// Safe ratio for reciprocity calculation
///
/// Returns log(exchange_in / (exchange_out + ε) + ε)
pub fn exchange_ratio_log(exchange_in: f64, exchange_out: f64) -> f64 {
    let ratio = exchange_in / (exchange_out + EPSILON);
    safe_log(ratio)
}

// =============================================================================
// DECAY FUNCTIONS
// =============================================================================

/// Exponential decay
///
/// Returns base × decay_rate^elapsed
pub fn exponential_decay(base: f64, decay_rate: f64, elapsed: f64) -> f64 {
    base * decay_rate.powf(elapsed)
}

/// Time-based decay factor (for aging evidence, etc.)
///
/// Returns a value in [0, 1] that decreases with age
pub fn time_decay(age_ms: u64, half_life_ms: u64) -> f64 {
    if half_life_ms == 0 {
        return 0.0;
    }
    let lambda = (0.5_f64).ln() / (half_life_ms as f64);
    (lambda * age_ms as f64).exp()
}

// =============================================================================
// AGGREGATION FUNCTIONS
// =============================================================================

/// Weighted mean of values
///
/// Returns Σ(value × weight) / Σ(weight)
pub fn weighted_mean(values: &[f64], weights: &[f64]) -> f64 {
    debug_assert_eq!(values.len(), weights.len());

    let (sum, weight_sum) = values
        .iter()
        .zip(weights.iter())
        .fold((0.0, 0.0), |(sum, w_sum), (&v, &w)| {
            (sum + v * w, w_sum + w)
        });

    if weight_sum < EPSILON {
        return 0.0;
    }

    sum / weight_sum
}

/// Variance of a slice of values
pub fn variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let sum_sq_diff: f64 = values.iter().map(|&x| (x - mean).powi(2)).sum();
    sum_sq_diff / values.len() as f64
}

/// Standard deviation
pub fn std_dev(values: &[f64]) -> f64 {
    variance(values).sqrt()
}

// =============================================================================
// REINFORCEMENT CALCULATION
// =============================================================================

/// Compute the reinforcement term Φ
///
/// Φ = γ × |Q|^μ × σ(r) × ψ(q) × φ(τ)
///
/// Where:
/// - Q = interaction volume
/// - r = reciprocity score
/// - q = quality score
/// - τ = tone score
pub fn compute_reinforcement(
    gamma: f64,
    mu: f64,
    volume: f64,
    reciprocity: f64,
    quality: Score,
    tone: SignedScore,
) -> f64 {
    let flow = volume.abs().powf(mu);
    let sigma_r = reciprocity_sigmoid(reciprocity);
    let psi_q = quality_multiplier(quality);
    let phi_tau = tone_multiplier(tone);

    gamma * flow * sigma_r.value() * psi_q * phi_tau
}

// =============================================================================
// TRUST AGGREGATION
// =============================================================================

/// Compute aggregated trust score
///
/// T = (w_Q × Q + w_R × σ(R) + w_S × S + w_D × D) / Σw
#[allow(clippy::too_many_arguments)]
pub fn compute_trust(
    quality: Score,
    reciprocity: f64,
    social: Score,
    diversity: Score,
    w_q: f64,
    w_r: f64,
    w_s: f64,
    w_d: f64,
) -> Score {
    let sigma_r = sigmoid(reciprocity);
    let total_weight = w_q + w_r + w_s + w_d;

    let trust = (w_q * quality.value()
        + w_r * sigma_r.value()
        + w_s * social.value()
        + w_d * diversity.value())
        / total_weight;

    Score::new(trust)
}

/// Apply diversity cap to trust
///
/// T_final = min(T, D + 0.3)
pub fn apply_diversity_cap(trust: Score, diversity: Score) -> Score {
    let cap = diversity.value() + 0.3;
    Score::new(trust.value().min(cap))
}

// =============================================================================
// CONVERGENCE
// =============================================================================

/// Compute convergence score
///
/// Conv = 1 - (Var(positions) / Var_max)
pub fn convergence_score(position_variance: f64, max_variance: f64) -> Score {
    if max_variance < EPSILON {
        return Score::ONE;
    }
    Score::new(1.0 - (position_variance / max_variance))
}

// =============================================================================
// BAYESIAN UPDATES
// =============================================================================

/// Bayesian belief update
///
/// belief_new = belief_old + weight × (1 - belief_old)
///
/// This approaches 1 asymptotically as evidence accumulates
pub fn bayesian_update(belief: Score, weight: f64) -> Score {
    let new_belief = belief.value() + weight * (1.0 - belief.value());
    Score::new(new_belief)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reciprocity_sigmoid() {
        // r = 0 → σ(r) = 0
        let s0 = reciprocity_sigmoid(0.0);
        assert!((s0.value() - 0.0).abs() < 0.01);

        // Positive r → positive σ(r)
        let sp = reciprocity_sigmoid(1.0);
        assert!(sp.value() > 0.0);

        // Negative r → negative σ(r)
        let sn = reciprocity_sigmoid(-1.0);
        assert!(sn.value() < 0.0);

        // Large values approach bounds
        let large_pos = reciprocity_sigmoid(100.0);
        assert!((large_pos.value() - 1.0).abs() < 0.01);

        let large_neg = reciprocity_sigmoid(-100.0);
        assert!((large_neg.value() - (-1.0)).abs() < 0.01);
    }

    #[test]
    fn test_sigmoid() {
        // x = 0 → sigmoid = 0.5
        let s0 = sigmoid(0.0);
        assert!((s0.value() - 0.5).abs() < 0.01);

        // Large positive → 1
        let sp = sigmoid(100.0);
        assert!((sp.value() - 1.0).abs() < 0.01);

        // Large negative → 0
        let sn = sigmoid(-100.0);
        assert!(sn.value() < 0.01);
    }

    #[test]
    fn test_quality_multiplier() {
        assert!((quality_multiplier(Score::ZERO) - 0.5).abs() < 0.001);
        assert!((quality_multiplier(Score::HALF) - 1.0).abs() < 0.001);
        assert!((quality_multiplier(Score::ONE) - 1.5).abs() < 0.001);
    }

    #[test]
    fn test_tone_multiplier() {
        assert!((tone_multiplier(SignedScore::NEG_ONE) - 0.4).abs() < 0.001);
        assert!((tone_multiplier(SignedScore::ZERO) - 0.7).abs() < 0.001);
        assert!((tone_multiplier(SignedScore::ONE) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_safe_log() {
        // Normal case
        assert!((safe_log(1.0) - 0.0).abs() < 0.001);
        assert!(safe_log(2.0) > 0.0);

        // Edge case: zero or negative should use epsilon
        let log_zero = safe_log(0.0);
        let log_neg = safe_log(-1.0);
        assert!(log_zero.is_finite());
        assert_eq!(log_zero, log_neg); // Both clamped to EPSILON
    }

    #[test]
    fn test_weighted_mean() {
        let values = vec![1.0, 2.0, 3.0];
        let equal_weights = vec![1.0, 1.0, 1.0];
        assert!((weighted_mean(&values, &equal_weights) - 2.0).abs() < 0.001);

        let skewed_weights = vec![1.0, 0.0, 0.0];
        assert!((weighted_mean(&values, &skewed_weights) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_variance() {
        let values = vec![1.0, 2.0, 3.0];
        let var = variance(&values);
        // Variance of [1,2,3] = ((1-2)² + (2-2)² + (3-2)²) / 3 = 2/3 ≈ 0.667
        assert!((var - 0.6667).abs() < 0.01);

        // All same values → variance = 0
        let same = vec![5.0, 5.0, 5.0];
        assert!(variance(&same) < 0.001);
    }

    #[test]
    fn test_bayesian_update() {
        let belief = Score::ZERO;
        let updated = bayesian_update(belief, 0.5);
        assert!((updated.value() - 0.5).abs() < 0.001);

        // Multiple updates approach 1
        let mut b = Score::ZERO;
        for _ in 0..10 {
            b = bayesian_update(b, 0.3);
        }
        assert!(b.value() > 0.95);
    }

    #[test]
    fn test_time_decay() {
        // At t=0, decay = 1
        let d0 = time_decay(0, 1000);
        assert!((d0 - 1.0).abs() < 0.001);

        // At t=half_life, decay ≈ 0.5
        let d_half = time_decay(1000, 1000);
        assert!((d_half - 0.5).abs() < 0.01);

        // At t=2×half_life, decay ≈ 0.25
        let d_double = time_decay(2000, 1000);
        assert!((d_double - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_reinforcement() {
        // Neutral case
        let phi = compute_reinforcement(
            0.1,      // gamma
            0.5,      // mu
            1.0,      // volume
            0.0,      // reciprocity (neutral)
            Score::HALF,
            SignedScore::ZERO,
        );
        // With r=0, σ(r)=0, so Φ=0
        assert!(phi.abs() < 0.01);

        // Positive reciprocity should give positive reinforcement
        let phi_pos = compute_reinforcement(
            0.1,
            0.5,
            1.0,
            1.0, // positive reciprocity
            Score::ONE,
            SignedScore::ONE,
        );
        assert!(phi_pos > 0.0);

        // Negative reciprocity should give negative reinforcement
        let phi_neg = compute_reinforcement(
            0.1,
            0.5,
            1.0,
            -1.0, // negative reciprocity
            Score::ONE,
            SignedScore::ONE,
        );
        assert!(phi_neg < 0.0);
    }
}
