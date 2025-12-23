//! Core types for the Symbiont protocol.
//!
//! Uses newtypes for type safety to prevent mixing different score types.

use crate::constants::{W_MAX, W_MIN};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, Mul, Sub};

// =============================================================================
// NODE IDENTIFIER
// =============================================================================

/// Cryptographic node identifier (32 bytes)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NodeId(pub [u8; 32]);

impl NodeId {
    /// Create a new NodeId from bytes
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Create a random NodeId (for testing/simulation)
    pub fn random() -> Self {
        let mut bytes = [0u8; 32];
        rand::Rng::fill(&mut rand::thread_rng(), &mut bytes);
        Self(bytes)
    }

    /// Create a NodeId from a simple index (for simulation)
    pub fn from_index(index: u64) -> Self {
        let mut bytes = [0u8; 32];
        bytes[..8].copy_from_slice(&index.to_le_bytes());
        Self(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeId({})", hex::encode(&self.0[..4]))
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0[..8]))
    }
}

// =============================================================================
// SCORE (0 to 1)
// =============================================================================

/// A normalized score in the range [0, 1]
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Score(f64);

impl Score {
    /// Create a new Score, clamping to [0, 1]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    /// Create Score from raw value without clamping (unsafe)
    pub fn from_raw(value: f64) -> Self {
        debug_assert!((0.0..=1.0).contains(&value), "Score out of bounds: {value}");
        Self(value)
    }

    /// Get the inner value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Zero score
    pub const ZERO: Score = Score(0.0);

    /// Maximum score
    pub const ONE: Score = Score(1.0);

    /// Middle score (0.5)
    pub const HALF: Score = Score(0.5);
}

impl Default for Score {
    fn default() -> Self {
        Self::HALF
    }
}

impl fmt::Debug for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Score({:.3})", self.0)
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

impl Add for Score {
    type Output = Score;
    fn add(self, rhs: Self) -> Self::Output {
        Score::new(self.0 + rhs.0)
    }
}

impl Sub for Score {
    type Output = f64; // Difference can be negative
    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Mul<f64> for Score {
    type Output = Score;
    fn mul(self, rhs: f64) -> Self::Output {
        Score::new(self.0 * rhs)
    }
}

impl From<f64> for Score {
    fn from(value: f64) -> Self {
        Score::new(value)
    }
}

// =============================================================================
// SIGNED SCORE (-1 to 1)
// =============================================================================

/// A signed normalized score in the range [-1, 1]
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SignedScore(f64);

impl SignedScore {
    /// Create a new SignedScore, clamping to [-1, 1]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(-1.0, 1.0))
    }

    /// Create from raw value without clamping
    pub fn from_raw(value: f64) -> Self {
        debug_assert!((-1.0..=1.0).contains(&value), "SignedScore out of bounds: {value}");
        Self(value)
    }

    /// Get the inner value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Zero (neutral)
    pub const ZERO: SignedScore = SignedScore(0.0);

    /// Maximum positive
    pub const ONE: SignedScore = SignedScore(1.0);

    /// Maximum negative
    pub const NEG_ONE: SignedScore = SignedScore(-1.0);
}

impl Default for SignedScore {
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Debug for SignedScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SignedScore({:.3})", self.0)
    }
}

impl fmt::Display for SignedScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:+.3}", self.0)
    }
}

impl Mul<f64> for SignedScore {
    type Output = SignedScore;
    fn mul(self, rhs: f64) -> Self::Output {
        SignedScore::new(self.0 * rhs)
    }
}

impl From<f64> for SignedScore {
    fn from(value: f64) -> Self {
        SignedScore::new(value)
    }
}

// =============================================================================
// WEIGHT (W_MIN to W_MAX)
// =============================================================================

/// Connection weight in the range [W_MIN, W_MAX]
#[derive(Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Weight(f64);

impl Weight {
    /// Create a new Weight, clamping to [W_MIN, W_MAX]
    pub fn new(value: f64) -> Self {
        Self(value.clamp(W_MIN, W_MAX))
    }

    /// Create from raw value without clamping
    pub fn from_raw(value: f64) -> Self {
        debug_assert!((W_MIN..=W_MAX).contains(&value), "Weight out of bounds: {value}");
        Self(value)
    }

    /// Get the inner value
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Minimum weight
    pub const MIN: Weight = Weight(W_MIN);

    /// Maximum weight
    pub const MAX: Weight = Weight(W_MAX);

    /// Initial weight for new connections
    pub const INIT: Weight = Weight(crate::constants::W_INIT);
}

impl Default for Weight {
    fn default() -> Self {
        Self::INIT
    }
}

impl fmt::Debug for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Weight({:.3})", self.0)
    }
}

impl fmt::Display for Weight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

impl Mul<f64> for Weight {
    type Output = Weight;
    fn mul(self, rhs: f64) -> Self::Output {
        Weight::new(self.0 * rhs)
    }
}

impl Add<f64> for Weight {
    type Output = Weight;
    fn add(self, rhs: f64) -> Self::Output {
        Weight::new(self.0 + rhs)
    }
}

// =============================================================================
// HASH AND SIGNATURE
// =============================================================================

/// Cryptographic hash (32 bytes, Blake3)
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    /// Compute hash of data
    pub fn compute(data: &[u8]) -> Self {
        Self(*blake3::hash(data).as_bytes())
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Hash({})", hex::encode(&self.0[..8]))
    }
}

/// Ed25519 signature (64 bytes)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl Signature {
    /// Create from bytes
    pub fn new(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Get the underlying bytes
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }
}

impl fmt::Debug for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Signature({}...)", hex::encode(&self.0[..8]))
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SignatureVisitor;

        impl<'de> serde::de::Visitor<'de> for SignatureVisitor {
            type Value = Signature;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("64 bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 64 {
                    return Err(E::custom(format!("expected 64 bytes, got {}", v.len())));
                }
                let mut bytes = [0u8; 64];
                bytes.copy_from_slice(v);
                Ok(Signature(bytes))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 64];
                for (i, byte) in bytes.iter_mut().enumerate() {
                    *byte = seq
                        .next_element()?
                        .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                }
                Ok(Signature(bytes))
            }
        }

        deserializer.deserialize_bytes(SignatureVisitor)
    }
}

// =============================================================================
// TIMESTAMP
// =============================================================================

/// Unix timestamp in milliseconds
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Create a new timestamp
    pub fn new(millis: u64) -> Self {
        Self(millis)
    }

    /// Current time
    pub fn now() -> Self {
        Self(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64)
    }

    /// Get the inner value
    pub fn millis(&self) -> u64 {
        self.0
    }

    /// Check if timestamp is older than given duration (in ms)
    pub fn is_older_than(&self, duration_ms: u64) -> bool {
        Timestamp::now().0.saturating_sub(self.0) > duration_ms
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Timestamp({})", self.0)
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

// =============================================================================
// CAPABILITY IDENTIFIER
// =============================================================================

/// Unique identifier for a capability
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CapabilityId(pub u64);

impl CapabilityId {
    /// Create a new capability ID
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    /// Create from a string (hashes the string)
    pub fn from_name(name: &str) -> Self {
        let hash = blake3::hash(name.as_bytes());
        let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
        Self(u64::from_le_bytes(bytes))
    }
}

impl fmt::Debug for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CapabilityId({})", self.0)
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// TASK AND WORKFLOW IDENTIFIERS
// =============================================================================

/// Unique identifier for a task
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub u64);

impl TaskId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn random() -> Self {
        Self(rand::random())
    }
}

impl fmt::Debug for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TaskId({})", self.0)
    }
}

/// Unique identifier for a workflow
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct WorkflowId(pub u64);

impl WorkflowId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn random() -> Self {
        Self(rand::random())
    }
}

impl fmt::Debug for WorkflowId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorkflowId({})", self.0)
    }
}

/// Unique identifier for a workflow step
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StepId(pub u64);

impl StepId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Debug for StepId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StepId({})", self.0)
    }
}

// =============================================================================
// HELPER MODULE FOR HEX ENCODING
// =============================================================================

mod hex {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

    pub fn encode(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            s.push(HEX_CHARS[(b >> 4) as usize] as char);
            s.push(HEX_CHARS[(b & 0x0f) as usize] as char);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_score_clamping() {
        assert_eq!(Score::new(-0.5).value(), 0.0);
        assert_eq!(Score::new(0.5).value(), 0.5);
        assert_eq!(Score::new(1.5).value(), 1.0);
    }

    #[test]
    fn test_signed_score_clamping() {
        assert_eq!(SignedScore::new(-1.5).value(), -1.0);
        assert_eq!(SignedScore::new(0.5).value(), 0.5);
        assert_eq!(SignedScore::new(1.5).value(), 1.0);
    }

    #[test]
    fn test_weight_clamping() {
        assert_eq!(Weight::new(0.0).value(), W_MIN);
        assert_eq!(Weight::new(0.5).value(), 0.5);
        assert_eq!(Weight::new(2.0).value(), W_MAX);
    }

    #[test]
    fn test_node_id_from_index() {
        let id1 = NodeId::from_index(1);
        let id2 = NodeId::from_index(2);
        assert_ne!(id1, id2);

        let id1_again = NodeId::from_index(1);
        assert_eq!(id1, id1_again);
    }

    #[test]
    fn test_hash_computation() {
        let h1 = Hash::compute(b"hello");
        let h2 = Hash::compute(b"hello");
        let h3 = Hash::compute(b"world");

        assert_eq!(h1, h2);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_capability_id_from_name() {
        let cap1 = CapabilityId::from_name("analysis");
        let cap2 = CapabilityId::from_name("analysis");
        let cap3 = CapabilityId::from_name("generation");

        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);
    }
}
