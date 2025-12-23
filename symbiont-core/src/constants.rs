//! Protocol constants for Symbiont v1.0
//!
//! All constants are defined here to allow easy tuning and experimentation.

// =============================================================================
// CONNECTION DYNAMICS
// =============================================================================

/// γ (GAMMA) - Reinforcement rate
/// Controls how strongly positive interactions strengthen connections
pub const GAMMA: f64 = 0.1;

/// μ (MU) - Flow exponent (sublinear)
/// Makes large interactions have diminishing returns: |Q|^μ
pub const MU: f64 = 0.5;

/// α (ALPHA) - Decay rate
/// Passive decay of connection weights over time
pub const ALPHA: f64 = 0.01;

/// β (BETA) - Reciprocity sensitivity
/// Controls steepness of reciprocity sigmoid
pub const BETA: f64 = 2.0;

/// λ (LAMBDA) - Memory factor for exponential moving average
/// Higher = more weight on historical values
pub const LAMBDA: f64 = 0.9;

/// θ (THETA) - Quality weight in reciprocity calculation
/// Adjusts reciprocity based on interaction quality
pub const THETA: f64 = 0.5;

/// δ (DELTA) - Defense dampening factor
/// How much threat signals reduce connection reinforcement
pub const DELTA: f64 = 0.2;

/// ε (EPSILON) - Small constant for division safety
/// Prevents division by zero in various calculations
pub const EPSILON: f64 = 0.001;

// =============================================================================
// CONNECTION BOUNDS
// =============================================================================

/// Minimum connection weight (prevents full disconnection)
pub const W_MIN: f64 = 0.01;

/// Maximum connection weight
pub const W_MAX: f64 = 1.0;

/// Initial connection weight for new connections
pub const W_INIT: f64 = 0.3;

// =============================================================================
// COLD START / SWIFT TRUST
// =============================================================================

/// Base trust level for new nodes (Swift Trust)
pub const SWIFT_TRUST_BASE: f64 = 0.4;

/// Penalty multiplier applied to voucher's trust when vouched node fails
pub const VOUCH_PENALTY: f64 = 0.5;

/// Number of interactions required during probation period
pub const PROBATION_COUNT: u32 = 50;

/// Minimum quality threshold to pass probation
pub const PROBATION_THRESHOLD: f64 = 0.6;

// =============================================================================
// QUALITY WEIGHTS
// =============================================================================

/// Weight for helpfulness in quality score
pub const OMEGA_HELP: f64 = 0.4;

/// Weight for accuracy in quality score
pub const OMEGA_ACC: f64 = 0.3;

/// Weight for relevance in quality score
pub const OMEGA_REL: f64 = 0.2;

/// Weight for timeliness in quality score
pub const OMEGA_TIME: f64 = 0.1;

/// Multiplier when user indicates they would reuse
pub const REUSE_BOOST: f64 = 1.2;

/// Multiplier when user indicates they would NOT reuse
pub const REUSE_PENALTY: f64 = 0.8;

// =============================================================================
// DEFENSE SIGNALING
// =============================================================================

/// Minimum confidence to propagate a defense signal
pub const PROPAGATE_THRESHOLD: f64 = 0.6;

/// Confidence decay per hop in signal propagation
pub const DECAY_PER_HOP: f64 = 0.8;

/// Minimum signal strength to process
pub const MIN_SIGNAL: f64 = 0.1;

/// Maximum hops for defense signal propagation
pub const MAX_HOPS: u8 = 5;

/// How much defense signals increase priming
pub const PRIMING_SENSITIVITY: f64 = 0.1;

/// Per-tick decay of priming level
pub const PRIMING_DECAY: f64 = 0.99;

/// Threat belief level that triggers defensive action
pub const ACTION_THRESHOLD: f64 = 0.7;

// =============================================================================
// CONFIDENCE
// =============================================================================

/// Memory factor for self-confidence EMA
pub const CONFIDENCE_MEMORY: f64 = 0.95;

// =============================================================================
// DETECTION
// =============================================================================

/// Threshold for detecting collusion (internal density)
pub const COLLUSION_THRESHOLD: f64 = 0.85;

/// Minimum diversity score to avoid trust cap
pub const DIVERSITY_THRESHOLD: f64 = 0.3;

/// Quality drop threshold to flag strategic adversary
pub const ADVERSARY_DROP: f64 = 0.3;

// =============================================================================
// TRUST COMPUTATION WEIGHTS
// =============================================================================

/// Weight for quality in trust computation
pub const TRUST_WEIGHT_QUALITY: f64 = 0.4;

/// Weight for reciprocity in trust computation
pub const TRUST_WEIGHT_RECIPROCITY: f64 = 0.2;

/// Weight for social proof in trust computation
pub const TRUST_WEIGHT_SOCIAL: f64 = 0.2;

/// Weight for diversity in trust computation
pub const TRUST_WEIGHT_DIVERSITY: f64 = 0.2;

// =============================================================================
// TONE COMPUTATION WEIGHTS
// =============================================================================

/// Weight for engagement signals in tone
pub const TONE_WEIGHT_ENGAGEMENT: f64 = 0.4;

/// Weight for friendliness signals in tone
pub const TONE_WEIGHT_FRIENDLINESS: f64 = 0.3;

/// Weight for collaboration signals in tone
pub const TONE_WEIGHT_COLLABORATION: f64 = 0.3;

// =============================================================================
// TIMING
// =============================================================================

/// Threshold (in ticks/ms) for considering a connection idle
pub const IDLE_THRESHOLD: u64 = 100_000;

/// Interval for diversity checks (in ticks)
pub const DIVERSITY_INTERVAL: u64 = 100;

/// Interval for status transition checks (in ticks)
pub const STATUS_INTERVAL: u64 = 50;

/// Interval for adversary scanning (in ticks)
pub const ADVERSARY_INTERVAL: u64 = 100;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_constants_validity() {
        // Bounds checks
        assert!(W_MIN > 0.0 && W_MIN < W_MAX);
        assert!(W_INIT >= W_MIN && W_INIT <= W_MAX);

        // Probability/score bounds
        assert!(SWIFT_TRUST_BASE >= 0.0 && SWIFT_TRUST_BASE <= 1.0);
        assert!(PROBATION_THRESHOLD >= 0.0 && PROBATION_THRESHOLD <= 1.0);

        // Weight sums (should approximately sum to 1)
        let quality_sum = OMEGA_HELP + OMEGA_ACC + OMEGA_REL + OMEGA_TIME;
        assert!((quality_sum - 1.0).abs() < 0.001);

        let trust_sum = TRUST_WEIGHT_QUALITY + TRUST_WEIGHT_RECIPROCITY
                      + TRUST_WEIGHT_SOCIAL + TRUST_WEIGHT_DIVERSITY;
        assert!((trust_sum - 1.0).abs() < 0.001);

        let tone_sum = TONE_WEIGHT_ENGAGEMENT + TONE_WEIGHT_FRIENDLINESS
                     + TONE_WEIGHT_COLLABORATION;
        assert!((tone_sum - 1.0).abs() < 0.001);

        // Decay factors should be in (0, 1)
        assert!(LAMBDA > 0.0 && LAMBDA < 1.0);
        assert!(PRIMING_DECAY > 0.0 && PRIMING_DECAY < 1.0);
        assert!(DECAY_PER_HOP > 0.0 && DECAY_PER_HOP < 1.0);
        assert!(CONFIDENCE_MEMORY > 0.0 && CONFIDENCE_MEMORY < 1.0);

        // Epsilon should be small but positive
        assert!(EPSILON > 0.0 && EPSILON < 0.01);
    }
}
