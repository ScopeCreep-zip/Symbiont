# Symbiont: Mycorrhizal Trust Protocol v0.1
## Complete Specification: Mathematics, Algorithms, and Implementation

**Version 0.1 — December 2025**

**Foundations:** Physarum Dynamics + Mycorrhizal Reciprocity + Human Trust Psychology + Plant Defense Biology

---

# TABLE OF CONTENTS

1. [Network Foundations](#part-i-network-foundations)
2. [Cold Start — Swift Trust](#part-ii-cold-start--swift-trust)
3. [Reciprocity Dynamics](#part-iii-reciprocity-dynamics)
4. [Quality Measurement](#part-iv-quality-measurement)
5. [Collaborative Tone](#part-v-collaborative-tone)
6. [Connection Dynamics (Physarum)](#part-vi-connection-dynamics-physarum)
7. [Resource Allocation](#part-vii-resource-allocation)
8. [Peer Review Consensus](#part-viii-peer-review-consensus)
9. [Self-Confidence via Peer Affirmation](#part-ix-self-confidence-via-peer-affirmation)
10. [Defense Signaling](#part-x-defense-signaling)
11. [Convergence Dynamics](#part-xi-convergence-dynamics)
12. [Global Trust Computation](#part-xii-global-trust-computation)
13. [Adversary Detection](#part-xiii-adversary-detection)
14. [Network Topology](#part-xiv-network-topology)
15. [Theoretical Properties](#part-xv-theoretical-properties)
16. [Node Lifecycle](#part-xvi-node-lifecycle)
17. [Complete Master Algorithm](#part-xvii-complete-master-algorithm)
18. [Parameter Reference](#part-xviii-parameter-reference)
19. [References](#part-xix-references)

---

# PART I: NETWORK FOUNDATIONS

## 1.1 Network Representation

$$G = (V, E, W, R, Q, T, S, \mathcal{T}, \Pi)$$

| Symbol | Type | Description |
|--------|------|-------------|
| $V$ | Set | Nodes (agents/participants) |
| $E \subseteq V \times V$ | Set | Directed edges (connections) |
| $W: E \rightarrow [0,1]$ | Function | Connection strength |
| $R: E \rightarrow \mathbb{R}$ | Function | Reciprocity score |
| $Q: E \rightarrow [0,1]$ | Function | Quality score |
| $T: V \rightarrow [0,1]$ | Function | Trust level |
| $S: V \rightarrow Status$ | Function | Node status |
| $\mathcal{T}: E \rightarrow [-1,1]$ | Function | Collaborative tone |
| $\Pi: V \rightarrow [0,1]$ | Function | Priming level (defense readiness) |

## 1.2 Connection State

$$C_{ij} = (w_{ij}, r_{ij}, q_{ij}, \tau_{ij}, \pi_{ij}, h_{ij}, t_{ij}, d_{ij})$$

| Component | Domain | Description |
|-----------|--------|-------------|
| $w_{ij}$ | $[0, 1]$ | Connection strength (conductivity) |
| $r_{ij}$ | $\mathbb{R}$ | Cumulative reciprocity score |
| $q_{ij}$ | $[0, 1]$ | Quality score from feedback |
| $\tau_{ij}$ | $[-1, 1]$ | Collaborative tone score |
| $\pi_{ij}$ | $[0, 1]$ | Priming level (defense readiness) |
| $h_{ij}$ | Buffer | Interaction history (bounded) |
| $t_{ij}$ | Timestamp | Last interaction time |
| $d_{ij}$ | $\mathbb{N}$ | Interaction count |

## 1.3 Node State

```python
@dataclass
class NodeState:
    # === IDENTITY ===
    id: VeilidNodeId                     # Cryptographic identity
    capabilities: Set[Capability]         # Skills/resources offered
    declared_role: RoleType              # Category for swift trust
    
    # === TRUST & STATUS ===
    status: NodeStatus                   # PROBATIONARY → MEMBER → ESTABLISHED → HUB
    trust: float                         # Global trust level T(n)
    trust_cap: float = 1.0               # Cap from diversity requirements
    self_confidence: float = 0.5         # Peer-affirmed confidence
    
    # === CONNECTIONS ===
    connections: Map[NodeId, Connection]
    mentor: Optional[NodeId]             # Assigned during probation
    
    # === VOUCHING ===
    vouchers: List[VouchRecord]          # Who vouched for this node
    vouched_for: List[VouchRecord]       # Who this node vouched for
    
    # === QUALITY ===
    quality_score: float                 # Aggregated Q_agg
    feedback_given: List[FeedbackRecord]
    feedback_received: List[FeedbackRecord]
    calibration: CalibrationState        # Rater calibration parameters
    
    # === AFFIRMATION ===
    affirmations_received: List[PeerAffirmation]
    affirmations_given: List[PeerAffirmation]
    
    # === DEFENSE STATE ===
    threat_beliefs: Map[NodeId, ThreatBelief]
    priming_level: float = 0.0           # Overall defense readiness
    defense_signals_received: List[DefenseSignal]
    
    # === CONVERGENCE ===
    current_positions: Map[TaskId, Position]
    dissent_history: List[DissentRecord]
    
    # === HISTORY ===
    interaction_history: List[InteractionRecord]  # Bounded buffer
    flags: Set[NodeFlag]                 # Warning flags
    join_timestamp: Timestamp
```

---

# PART II: COLD START — SWIFT TRUST

## 2.1 The Psychological Insight

From Meyerson, Weick & Kramer (1996):
> "A temporary team interacts **as if trust were present**, but then must verify that the team can manage expectations."

**Key principle:** Trust first, verify later. Don't start at zero.

## 2.2 Initial Trust Computation

When newcomer $n$ joins the network:

$$T_{init}(n) = \omega_s \cdot S_{swift} + \omega_c \cdot C_{category} + \omega_v \cdot V_{vouch} + \omega_p \cdot P_{social}$$

Where $\omega_s + \omega_c + \omega_v + \omega_p = 1$ (default: 0.3, 0.2, 0.3, 0.2)

### 2.2.1 Swift Trust Baseline

$$S_{swift} = 0.4$$

**Psychological basis:** Dunning et al. (2014) found people trust strangers even when expecting exploitation, because NOT trusting violates social norms. Societies with higher baseline trust have better outcomes.

### 2.2.2 Category-Based Trust

$$C_{category}(n) = \tau_{role}(n.role) \cdot \phi_{similarity}(n, \mathcal{T})$$

**Role Trust Scores:**

| Role | $\tau_{role}$ | Rationale |
|------|---------------|-----------|
| VERIFIED_INSTITUTION | 0.8 | Institutional backing |
| KNOWN_CAPABILITY | 0.6 | Demonstrated skill type |
| STANDARD_AGENT | 0.4 | Default category |
| UNKNOWN | 0.2 | No category information |

**Behavioral Similarity:**

$$\phi_{similarity}(n, \mathcal{T}) = \max_{t \in \mathcal{T}} \left( \frac{\vec{b}_n \cdot \vec{b}_t}{|\vec{b}_n||\vec{b}_t|} \right)$$

Where:
- $\mathcal{T}$ = set of trusted nodes (high reputation)
- $\vec{b}_n$ = behavioral feature vector of newcomer
- $\vec{b}_t$ = behavioral feature vector of trusted node

### 2.2.3 Vouching Trust Transfer

$$V_{vouch}(n) = \sum_{v \in Vouchers(n)} T(v) \cdot s_v \cdot \delta(age_v)$$

Where:
- $T(v)$ = voucher's current trust level
- $s_v$ = stake fraction (typically 0.1)
- $\delta(age)$ = time decay on vouch validity

**Voucher Accountability — If vouchee cheats:**

$$T_{voucher}^{new} = T_{voucher} - 0.5 \cdot s_v \cdot T_{voucher}$$

```python
@dataclass
class VouchRecord:
    voucher: NodeId
    newcomer: NodeId
    stake: float           # Fraction of voucher's reputation staked
    timestamp: Timestamp
    active: bool           # Still valid?
    voucher_sig: Signature
    
def vouch_for(voucher: Node, newcomer: Node, stake_fraction: float = 0.1) -> float:
    """
    Voucher transfers reputation to newcomer.
    CRITICAL: Voucher is accountable if newcomer cheats.
    """
    transferred = voucher.trust * stake_fraction
    
    record = VouchRecord(
        voucher=voucher.id,
        newcomer=newcomer.id,
        stake=transferred,
        timestamp=now(),
        active=True,
        voucher_sig=voucher.sign()
    )
    
    voucher.vouched_for.append(record)
    newcomer.vouchers.append(record)
    
    return transferred

def on_cheating_detected(cheater: Node):
    """
    When cheater is caught, punish their vouchers too.
    This makes vouching costly — not cheap talk.
    """
    VOUCHER_PENALTY = 0.5  # Vouchers lose 50% of staked amount
    
    for record in cheater.vouchers:
        if record.active:
            voucher = get_node(record.voucher)
            penalty = record.stake * VOUCHER_PENALTY
            voucher.trust -= penalty
            record.active = False
            log(f"Voucher {voucher.id} penalized {penalty}")
```

### 2.2.4 Social Proof

$$P_{social}(n) = \sigma\left(\frac{\sum_{i \in Interactors(n)} T(i) \cdot q_i}{\bar{I}_{network}}\right)$$

Where:
- $Interactors(n)$ = nodes that have interacted with $n$
- $q_i$ = quality of interaction with node $i$
- $\bar{I}_{network}$ = average interaction rate in network
- $\sigma$ = sigmoid normalization

## 2.3 Probationary Period

```python
class ProbationaryProtocol:
    """
    New nodes go through mentored probation before full membership.
    Based on organizational socialization research (Bauer et al. 2007).
    """
    
    PROBATION_DURATION = 50   # interactions
    PROBATION_THRESHOLD = 0.6 # minimum quality to pass
    MENTOR_REVIEW_RATE = 0.2  # fraction spot-checked
    
    def __init__(self, newcomer: Node, network: Network):
        self.newcomer = newcomer
        self.newcomer.status = NodeStatus.PROBATIONARY
        self.mentor = self.assign_mentor(network)
        self.performance_log = []
        
    def assign_mentor(self, network: Network) -> Node:
        """
        Assign established node as mentor.
        Prefer mentors with capability overlap.
        """
        candidates = [n for n in network.nodes 
                     if n.status in [NodeStatus.ESTABLISHED, NodeStatus.HUB]
                     and n.current_mentees < MAX_MENTEES]
        
        scored = [(c, capability_overlap(c, self.newcomer)) for c in candidates]
        scored.sort(key=lambda x: x[1], reverse=True)
        
        mentor = scored[0][0]
        mentor.current_mentees += 1
        return mentor
    
    def record_interaction(self, interaction: Interaction, feedback: Feedback):
        """During probation, all interactions logged and mentor-reviewed."""
        quality = calculate_quality_score(feedback)
        
        self.performance_log.append({
            'interaction': interaction,
            'feedback': feedback,
            'quality': quality,
            'timestamp': now()
        })
        
        # Mentor spot-checks
        if random() < self.MENTOR_REVIEW_RATE:
            mentor_review = self.mentor.review(interaction)
            self.performance_log[-1]['mentor_review'] = mentor_review
    
    def check_graduation(self) -> bool:
        """After sufficient interactions, evaluate for membership."""
        if len(self.performance_log) < self.PROBATION_DURATION:
            return False
        
        avg_quality = mean([p['quality'] for p in self.performance_log])
        
        if avg_quality >= self.PROBATION_THRESHOLD:
            self.graduate()
            return True
        else:
            self.extend_probation()
            return False
    
    def graduate(self):
        """Newcomer becomes full member."""
        self.newcomer.status = NodeStatus.MEMBER
        self.newcomer.trust = min(self.newcomer.trust * 1.5, 0.8)
        self.mentor.trust += MENTORSHIP_BONUS
        self.mentor.current_mentees -= 1
        log(f"Node {self.newcomer.id} graduated to MEMBER")
    
    def extend_probation(self):
        """Not ready yet — extend with reduced trust."""
        self.newcomer.trust *= 0.8
        self.PROBATION_DURATION += 25
        log(f"Probation extended for {self.newcomer.id}")
```

---

# PART III: RECIPROCITY DYNAMICS

## 3.1 Exchange Ratio Measurement

For each interaction between nodes $i$ and $j$:

$$\rho_{ij} = \frac{c_i^{in}}{c_i^{out} + \epsilon}$$

Where:
- $c_i^{in}$ = value received by $i$ from $j$
- $c_i^{out}$ = value given by $i$ to $j$
- $\epsilon = 0.001$ = small constant (prevents division by zero)

## 3.2 Cumulative Reciprocity Score

$$r_{ij}(t+1) = \lambda \cdot r_{ij}(t) + (1-\lambda) \cdot \left[\log(\rho_{ij}) + \theta \cdot (q_{ij} - 0.5)\right]$$

Where:
- $\lambda \approx 0.9$ = memory factor (exponential moving average)
- $\theta \approx 0.5$ = quality weight
- $q_{ij}$ = quality score from feedback

**Interpretation:**
- $r_{ij} > 0$: Node $j$ is generous to $i$
- $r_{ij} < 0$: Node $j$ is exploiting $i$
- $r_{ij} \approx 0$: Fair exchange

**Bidirectional Consistency:**
For healthy connections: $r_{ij} \approx -r_{ji}$

## 3.3 Reciprocity Sigmoid

$$\sigma(r) = \frac{2}{1 + e^{-\beta r}} - 1$$

Maps $r \in (-\infty, +\infty)$ to $\sigma \in (-1, 1)$

Where $\beta \approx 2.0$ controls sensitivity.

| $r$ | $\sigma(r)$ | Meaning |
|-----|-------------|---------|
| -2 | -0.96 | Severely exploitative |
| -1 | -0.76 | Exploitative |
| 0 | 0 | Fair |
| +1 | +0.76 | Generous |
| +2 | +0.96 | Very generous |

---

# PART IV: QUALITY MEASUREMENT

## 4.1 Feedback Structure

```python
@dataclass
class Feedback:
    """Multi-dimensional feedback from interaction recipient."""
    giver: NodeId
    receiver: NodeId
    interaction_id: InteractionId
    timestamp: Timestamp
    
    # Dimensional scores (1-5 scale)
    helpfulness: int      # Did it solve my problem?
    accuracy: int         # Was the information correct?
    relevance: int        # Was it on-topic?
    timeliness: int       # Was it fast enough?
    
    # Binary signal
    would_use_again: bool
    
    # Optional
    specific_feedback: Optional[str]
    
    # Cryptographic
    signature: Signature  # Signed by giver
```

## 4.2 Quality Score Computation

$$Q(f) = \frac{\omega_h \cdot f.helpfulness + \omega_a \cdot f.accuracy + \omega_r \cdot f.relevance + \omega_t \cdot f.timeliness}{4} \cdot \psi(f.reuse)$$

Default weights: $\omega_h=0.4, \omega_a=0.3, \omega_r=0.2, \omega_t=0.1$

**Reuse multiplier:**
- $\psi(true) = 1.2$ — would use again
- $\psi(false) = 0.8$ — would not use again

**Normalized to [0,1]:**

$$Q_{norm}(f) = \frac{Q(f) - 1}{4}$$

## 4.3 Aggregated Quality Score

For node $n$ with feedback history $\mathcal{F}_n$:

$$Q_{agg}(n) = \frac{\sum_{f \in \mathcal{F}_n} Q_{norm}(f) \cdot T(f.giver) \cdot \delta(age(f))}{\sum_{f \in \mathcal{F}_n} T(f.giver) \cdot \delta(age(f))}$$

Weighted by:
- Giver's trust level (trusted evaluators matter more)
- Recency via time decay $\delta(age) = e^{-\lambda_{age} \cdot age}$

## 4.4 Quality Multiplier in Dynamics

$$\psi(q) = 0.5 + q$$

Maps $q \in [0, 1]$ to multiplier $\in [0.5, 1.5]$

## 4.5 Calibrated Peer Assessment

**Problem:** Different raters have different standards. Alice's "4" might be Bob's "5".

**Solution:** Calibrate each rater against consensus.

```python
class RaterCalibration:
    """
    Adjust for individual rater bias and scale.
    Based on psychometric calibration research.
    """
    
    def __init__(self):
        self.rater_params = {}  # NodeId -> (bias, scale)
    
    def calibrate_rater(self, rater: Node, calibration_set: List[CalibratedItem]):
        """Use known-quality items to estimate rater's bias and scale."""
        rater_scores = [rater.evaluate(item) for item in calibration_set]
        true_scores = [item.known_quality for item in calibration_set]
        
        # Linear regression: rater_score = scale * true_score + bias
        bias = mean(rater_scores) - mean(true_scores)
        scale = std(rater_scores) / std(true_scores) if std(true_scores) > 0 else 1.0
        
        self.rater_params[rater.id] = (bias, scale)
    
    def calibrated_score(self, rater: Node, raw_score: float) -> float:
        """Adjust raw score based on rater's calibration."""
        if rater.id not in self.rater_params:
            return raw_score  # Uncalibrated
        
        bias, scale = self.rater_params[rater.id]
        return (raw_score - bias) / scale
    
    def aggregate_calibrated(self, scores: Dict[Node, float]) -> float:
        """Combine calibrated scores from multiple raters."""
        calibrated = [self.calibrated_score(rater, score) 
                      for rater, score in scores.items()]
        weights = [rater.trust for rater in scores.keys()]
        return weighted_mean(calibrated, weights)
```

## 4.6 Feedback Integrity — Preventing Gaming

```python
class FeedbackIntegrity:
    """Detect and prevent feedback manipulation."""
    
    COLLUSION_CORRELATION_THRESHOLD = 0.85
    OUTLIER_THRESHOLD = 2.5  # standard deviations
    FEEDBACK_STAKE = 0.01    # Stake per feedback given
    
    def validate_feedback(self, feedback: Feedback) -> FeedbackValidity:
        """Check if feedback is legitimate."""
        giver = get_node(feedback.giver)
        receiver = get_node(feedback.receiver)
        
        # 1. Collusion detection
        correlation = self.feedback_correlation(giver, receiver)
        if correlation > self.COLLUSION_CORRELATION_THRESHOLD:
            return FeedbackValidity.SUSPICIOUS_COLLUSION
        
        # 2. Outlier detection
        peer_ratings = self.get_peer_ratings(receiver)
        if len(peer_ratings) > 5:
            mean_rating = mean(peer_ratings)
            std_rating = std(peer_ratings)
            deviation = abs(feedback.quality - mean_rating) / (std_rating + 0.01)
            
            if deviation > self.OUTLIER_THRESHOLD:
                return FeedbackValidity.OUTLIER
        
        # 3. Stake-based accountability
        giver.feedback_stake += self.FEEDBACK_STAKE
        
        return FeedbackValidity.VALID
    
    def feedback_correlation(self, a: Node, b: Node) -> float:
        """How correlated are ratings between these two nodes?"""
        a_to_b = [f.quality for f in a.feedback_given if f.receiver == b.id]
        b_to_a = [f.quality for f in b.feedback_given if f.receiver == a.id]
        
        if len(a_to_b) < 3 or len(b_to_a) < 3:
            return 0.0  # Not enough data
        
        # Both consistently rate each other high?
        if mean(a_to_b) > 0.9 and mean(b_to_a) > 0.9:
            return 0.95  # Very suspicious
        
        return pearson_correlation(a_to_b, b_to_a)
    
    def consensus_validation(self, interaction: Interaction):
        """Validate feedback against peer consensus after time passes."""
        original = interaction.feedback
        peer_assessments = self.collect_peer_assessments(interaction)
        
        if len(peer_assessments) < 3:
            return  # Not enough peers
        
        consensus = mean(peer_assessments)
        accuracy = 1 - abs(consensus - original.quality) / 5
        
        giver = get_node(original.giver)
        
        if accuracy > 0.7:
            # Accurate feedback — reward
            giver.feedback_stake *= 1.1
            giver.trust += 0.001
        else:
            # Inaccurate feedback — penalize
            giver.feedback_stake *= 0.8
            giver.trust -= 0.005
            log(f"Feedback gaming detected: {giver.id}")
```

---

# PART V: COLLABORATIVE TONE

## 5.1 Psychological Foundation

From Edmondson's psychological safety research:
> "People communicate their emotions and perceptions through tone of voice, how fast or slow they speak, and word choice."

For AI agents, "tone" manifests in observable behavioral patterns.

## 5.2 Tone Indicators

```python
@dataclass
class ToneIndicators:
    """Observable signals of collaborative vs. adversarial tone."""
    
    # Engagement signals
    response_latency: float      # Fast = engaged, slow = reluctant
    elaboration_ratio: float     # Unprompted detail = invested
    question_asking: float       # Curiosity = collaborative
    
    # Framing signals
    affirmative_ratio: float     # "Yes, and..." vs "No, but..."
    hedge_frequency: float       # Uncertainty markers
    acknowledgment_rate: float   # "I see your point..." 
    
    # Constructive signals
    alternative_offering: float  # Suggests options, not just critiques
    build_on_ratio: float        # Extends others' ideas
    credit_giving: float         # Acknowledges contributions
```

## 5.3 Tone Score Computation

$$\tau_{ij} = \tanh\left(\omega_e \cdot E + \omega_f \cdot F + \omega_c \cdot C\right)$$

**Engagement ($E$):**
$$E = 0.4 \cdot latency\_score + 0.4 \cdot elaboration + 0.2 \cdot questions$$

**Framing ($F$):**
$$F = 0.5 \cdot (affirmative - 0.5) + 0.3 \cdot (0.5 - hedging) + 0.2 \cdot acknowledgment$$

**Constructive ($C$):**
$$C = 0.5 \cdot alternatives + 0.3 \cdot build\_on + 0.2 \cdot credit$$

The $\tanh$ bounds the result to $[-1, 1]$.

## 5.4 Tone Multiplier

$$\phi(\tau) = 0.7 + 0.3\tau$$

Maps $\tau \in [-1, 1]$ to multiplier $\in [0.4, 1.0]$

**Intuition:**
- Positive tone ($\tau > 0$) amplifies connection reinforcement
- Negative tone ($\tau < 0$) dampens it
- Captures: "I don't want to work with them even though they're competent"

## 5.5 Ambient Tone — The Mood of the Room

For a task group or subnetwork $G$:

$$T_{ambient}(G) = \frac{\sum_{(i,j) \in E_G} w_{ij} \cdot \tau_{ij}}{\sum_{(i,j) \in E_G} w_{ij}}$$

```python
def monitor_ambient_tone(task_group: TaskGroup) -> AmbientToneReport:
    """Monitor overall collaborative climate."""
    tones = [c.tau for c in task_group.active_connections]
    weights = [c.w for c in task_group.active_connections]
    
    ambient = weighted_mean(tones, weights)
    variance = weighted_variance(tones, weights)
    
    report = AmbientToneReport(
        ambient_tone=ambient,
        tone_variance=variance,
        trend=compute_trend(task_group.tone_history)
    )
    
    # Intervention triggers
    if ambient < -0.3:
        report.flags.add('HOSTILE_CLIMATE')
    if variance > 0.5:
        report.flags.add('POLARIZED_CLIMATE')
    if report.trend.slope < -0.05:
        report.flags.add('DETERIORATING_CLIMATE')
    
    return report
```

---

# PART VI: CONNECTION DYNAMICS (PHYSARUM)

## 6.1 Master Equation

$$\frac{dw_{ij}}{dt} = \Phi(Q_{ij}, r_{ij}, q_{ij}, \tau_{ij}) - \alpha w_{ij} + \eta_{ij}(t) - D_{ij}(t)$$

| Term | Description |
|------|-------------|
| $\Phi$ | Reinforcement function (positive feedback) |
| $-\alpha w_{ij}$ | Decay term (unused connections fade) |
| $\eta_{ij}(t)$ | Exploration noise (discover new paths) |
| $-D_{ij}(t)$ | Defense signal dampening |

## 6.2 Reinforcement Function

$$\Phi(Q, r, q, \tau) = \gamma \cdot |Q|^{\mu} \cdot \sigma(r) \cdot \psi(q) \cdot \phi(\tau)$$

| Component | Formula | Range | Effect |
|-----------|---------|-------|--------|
| Flow | $\|Q\|^\mu$ | $[0, \infty)$ | More interaction → stronger (sublinear) |
| Reciprocity | $\sigma(r)$ | $[-1, 1]$ | Fair exchange required |
| Quality | $\psi(q) = 0.5 + q$ | $[0.5, 1.5]$ | High quality = bonus |
| Tone | $\phi(\tau) = 0.7 + 0.3\tau$ | $[0.4, 1.0]$ | Collaborative = bonus |

**Note:** The sublinear exponent $\mu = 0.5$ prevents monopolies — doubling flow doesn't double reinforcement.

## 6.3 Discrete Update

$$w_{ij}(t+1) = w_{ij}(t) + \Delta t \cdot \left[\Phi - \alpha w_{ij} - D_{ij}\right]$$

Clamped: $w_{ij} \in [w_{min}, w_{max}]$ where $w_{min} = 0.01$, $w_{max} = 1.0$

## 6.4 Defense Signal Dampening

$$D_{ij}(t) = \delta \cdot threat\_level(j) \cdot \mathbf{1}[defense\_signal\_active]$$

Where $\delta \approx 0.2$

## 6.5 Exploration Noise

$$\eta_{ij}(t) \sim \mathcal{N}(0, \sigma_\eta^2) \cdot (1 - w_{ij})$$

Noise is higher for weak connections (explore) and lower for strong ones (exploit).

---

# PART VII: RESOURCE ALLOCATION

## 7.1 Allocation Formula

When node $i$ has resources to distribute:

$$A_{ij} = A_{total} \cdot \frac{w_{ij} \cdot f(r_{ij})}{\sum_{k \in N(i)} w_{ik} \cdot f(r_{ik})}$$

## 7.2 Reward Function

$$f(r) = \max(f_{min}, e^{\kappa r})$$

Where:
- $\kappa \approx 1.0$ = reward sensitivity
- $f_{min} \approx 0.1$ = minimum allocation (no complete cutoff)

**Effect:**
- Positive reciprocity ($r > 0$) → more resources
- Negative reciprocity ($r < 0$) → fewer resources (but not zero)

---

# PART VIII: PEER REVIEW CONSENSUS

## 8.1 Component Scores

**Approval Rate:**
$$A = \frac{\#(APPROVE)}{total\_reviews}$$

**Severity Score:**
$$S = 1 - \frac{weighted\_severity}{max\_severity}$$

**Freshness:**
$$P = \exp(-\lambda_{age} \cdot avg\_review\_age)$$

## 8.2 Consensus Score

$$K = \omega_A \cdot A + \omega_S \cdot S + \omega_P \cdot P$$

Default weights: $\omega_A = 0.5$, $\omega_S = 0.3$, $\omega_P = 0.2$

## 8.3 Consensus Levels

| Score | Level | Meaning | Action |
|-------|-------|---------|--------|
| $K \geq 0.85$ | STRONG | Ready to proceed | Accept |
| $K \geq 0.65$ | WORKING | Minor issues | Accept with notes |
| $K \geq 0.50$ | WEAK | Needs discussion | Deliberate |
| $K < 0.50$ | NONE | Major revision needed | Reject/revise |

```python
def calculate_weighted_consensus(work: Work, reviews: List[Review]) -> Consensus:
    """Consensus weighted by reviewer trust AND quality history."""
    if len(reviews) == 0:
        return Consensus(level='NONE', confidence=0)
    
    weighted_scores = []
    total_weight = 0
    
    for review in reviews:
        reviewer = get_node(review.reviewer_id)
        
        # Combined weight: trust × quality × connection strength
        weight = (reviewer.trust * 
                  reviewer.quality_score * 
                  get_connection_strength(work.author, reviewer))
        
        score = review_to_score(review)
        weighted_scores.append(score * weight)
        total_weight += weight
    
    if total_weight == 0:
        return Consensus(level='NONE', confidence=0)
    
    K = sum(weighted_scores) / total_weight
    
    # Check for blocks
    blocks = [r for r in reviews if r.judgment == 'BLOCK']
    block_weight = sum(get_node(r.reviewer_id).trust for r in blocks)
    
    # Determine level
    if K >= 0.85 and len(blocks) == 0:
        level = 'STRONG'
    elif K >= 0.65 and block_weight < 0.2:
        level = 'WORKING'
    elif K >= 0.50:
        level = 'WEAK'
    else:
        level = 'NONE'
    
    return Consensus(level=level, confidence=K)
```

---

# PART IX: SELF-CONFIDENCE VIA PEER AFFIRMATION

## 9.1 The Insight

Nodes should know they're performing well because **peers tell them**, not just via impersonal quality scores. This mirrors how humans gain confidence.

## 9.2 Peer Affirmation Protocol

```python
@dataclass
class PeerAffirmation:
    """Explicit positive feedback beyond quality scores."""
    affirmer: NodeId
    recipient: NodeId
    affirmation_type: AffirmationType  # QUALITY | RELIABILITY | COLLABORATION | GROWTH | INNOVATION
    strength: float  # 0-1
    specific_context: Optional[str]
    timestamp: Timestamp
    signature: Signature
```

## 9.3 Self-Confidence Score

$$S_{conf}(n) = \alpha \cdot S_{conf}^{prev}(n) + (1-\alpha) \cdot \bar{A}(n)$$

Where:
$$\bar{A}(n) = \frac{\sum_{a \in A_n^{recent}} T(a.affirmer) \cdot a.strength}{\sum_{a \in A_n^{recent}} T(a.affirmer)}$$

- $A_n^{recent}$ = recent affirmations received
- $T(a.affirmer)$ = trust level of the affirmer
- $\alpha \approx 0.95$ = stability factor (confidence is stable, not volatile)

## 9.4 Confidence Effects

| Effect | Formula |
|--------|---------|
| Task complexity threshold | $base \times (1 + 0.3(conf - 0.5))$ |
| Voice threshold | $base \times (1 - 0.3 \cdot conf)$ |
| Mentorship capacity | $base \times (1 + 0.5 \cdot conf)$ |
| Exploration rate | $base \times (1 + 0.2 \cdot conf)$ |

```python
def apply_confidence_effects(node: Node):
    """Self-confidence affects node behavior."""
    conf = node.self_confidence
    
    node.task_complexity_threshold *= (1 + 0.3 * (conf - 0.5))
    node.voice_threshold *= (1 - 0.3 * conf)
    node.mentorship_capacity *= (1 + 0.5 * conf)
    node.exploration_rate *= (1 + 0.2 * conf)
    
    # Overconfidence check
    if conf > 0.9 and node.recent_quality < 0.6:
        node.flags.add('OVERCONFIDENT')
```

## 9.5 Affirmation Validation

```python
def validate_affirmation(affirmation: PeerAffirmation) -> AffirmationValidity:
    """Prevent affirmation gaming."""
    affirmer = get_node(affirmation.affirmer)
    recipient = get_node(affirmation.recipient)
    
    # 1. Must have recent interaction
    if not has_recent_interaction(affirmer, recipient, days=30):
        return AffirmationValidity.NO_BASIS
    
    # 2. Rate limiting
    recent = affirmer.affirmations_given_last_30_days()
    if len(recent) > AFFIRMATION_RATE_LIMIT:  # 10/month
        return AffirmationValidity.RATE_LIMITED
    
    # 3. Mutual affirmation check
    mutual_count = count_mutual_affirmations(affirmer, recipient, days=90)
    if mutual_count > MUTUAL_THRESHOLD:  # 3/quarter
        return AffirmationValidity.SUSPICIOUS_MUTUAL
    
    # 4. Consistency check
    if affirmation.type == AffirmationType.QUALITY:
        actual_quality = recipient.quality_from_perspective_of(affirmer)
        if actual_quality < 0.4:
            return AffirmationValidity.INCONSISTENT
    
    return AffirmationValidity.VALID
```

---

# PART X: DEFENSE SIGNALING

## 10.1 Biological Foundation

From plant immunology research (Cell Host & Microbe, 2025):
> "Donor plants infected by pathogens **transfer jasmonic acid via CMNs**, which acts as a chemical signal in receiver plants, inducing defense responses."

**Key mechanisms:**
1. **Dual pathways** (SA for biotrophic, JA for necrotrophic threats)
2. **Defense priming** — receivers respond faster and stronger
3. **Signal attenuation** — strength decreases with distance
4. **Two-stage defense** — fast general alert, then specific response

## 10.2 Defense Signal Types

```python
@dataclass
class DefenseSignal:
    """Warning signal propagated through network."""
    signal_type: SignalType     # GENERAL_ALERT | SPECIFIC_THREAT | BROADCAST
    sender: NodeId
    original_sender: NodeId     # Who first detected the threat
    threat_source: NodeId       # The bad actor
    threat_type: ThreatType     # CHEATING | SYBIL | COLLUSION | QUALITY_FRAUD | STRATEGIC_DEFECT
    confidence: float           # Attenuates with propagation
    evidence_hash: Hash         # Cryptographic proof
    hop_count: int              # How many hops from original
    timestamp: Timestamp
    signature: Signature

class SignalType(Enum):
    GENERAL_ALERT = "general"      # "Something is wrong" (fast, broad)
    SPECIFIC_THREAT = "specific"   # "Here's what and why" (detailed)
    BROADCAST = "broadcast"        # Network-wide warning (hub-originated)
```

## 10.3 Signal Attenuation

$$confidence_{hop_n} = confidence_{original} \times decay^n \times w_{connection}$$

Where $decay \approx 0.8$

## 10.4 Signal Propagation

```python
def propagate_defense_signal(signal: DefenseSignal, sender: Node, network: Network):
    """
    Propagate warning through trusted connections.
    Models plant-to-plant communication via CMN.
    """
    for partner_id, conn in sender.connections.items():
        # Only propagate through strong connections
        if conn.w < PROPAGATION_THRESHOLD:  # 0.6
            continue
        
        partner = get_node(partner_id)
        
        # Don't send back to who sent it
        if partner_id == signal.sender:
            continue
        
        # Attenuate signal
        attenuation = conn.w * (DECAY_PER_HOP ** signal.hop_count)
        
        attenuated_signal = DefenseSignal(
            signal_type=signal.signal_type,
            sender=sender.id,
            original_sender=signal.original_sender,
            threat_source=signal.threat_source,
            threat_type=signal.threat_type,
            confidence=signal.confidence * attenuation,
            evidence_hash=signal.evidence_hash,
            hop_count=signal.hop_count + 1,
            timestamp=now(),
            signature=sender.sign()
        )
        
        # Stop if signal too weak or too many hops
        if attenuated_signal.confidence < MIN_SIGNAL_STRENGTH:  # 0.1
            continue
        if attenuated_signal.hop_count > MAX_HOPS:  # 5
            continue
        
        # Receiver processes signal
        partner.receive_defense_signal(attenuated_signal)
        
        # Maybe propagate further
        if attenuated_signal.confidence > RELAY_THRESHOLD:
            propagate_defense_signal(attenuated_signal, partner, network)
```

## 10.5 Signal Reception and Priming

```python
def receive_defense_signal(self, signal: DefenseSignal):
    """Process incoming defense signal. This is where priming happens."""
    self.defense_signals_received.append(signal)
    
    # Update threat belief about the threat source
    current = self.threat_beliefs.get(signal.threat_source, ThreatBelief())
    
    # Weighted update: trust in sender × signal confidence
    sender_trust = self.trust_in(signal.sender)
    weight = sender_trust * signal.confidence
    
    # Bayesian-like update
    new_level = current.threat_level + weight * (1 - current.threat_level)
    
    self.threat_beliefs[signal.threat_source] = ThreatBelief(
        threat_level=new_level,
        threat_type=signal.threat_type,
        evidence_hashes=current.evidence_hashes + [signal.evidence_hash],
        last_update=now()
    )
    
    # PRIMING: Increase defense readiness
    self.increase_priming(signal)
    
    # Take action if threshold crossed
    if new_level > DEFENSE_ACTION_THRESHOLD:  # 0.7
        self.take_defensive_action(signal.threat_source)

def increase_priming(self, signal: DefenseSignal):
    """
    Become primed — ready to respond faster to future threats.
    From plant biology: primed plants respond 50% faster, 50% stronger.
    """
    priming_boost = signal.confidence * PRIMING_SENSITIVITY  # 0.1
    self.priming_level = min(1.0, self.priming_level + priming_boost)
    
    # Specific priming for this threat type
    self.threat_type_priming[signal.threat_type] += priming_boost
    
    # Priming decays over time if no threats materialize
    schedule_priming_decay(self, delay=PRIMING_DECAY_DELAY)
```

## 10.6 Priming Update

$$\pi(t+1) = \min(1.0, \pi(t) + signal\_confidence \times sensitivity)$$

With decay when no threats: $\pi(t+1) = \pi(t) \times decay\_rate$

## 10.7 Threat Belief Update (Bayesian)

$$belief_{new} = belief_{old} + weight \times (1 - belief_{old})$$

Where $weight = trust_{sender} \times signal\_confidence$

## 10.8 Dual-Channel Reality

Nodes calibrate truth through TWO channels:

$$Reality(n, topic) = \omega_d \cdot D_{direct}(n) + \omega_p \cdot P_{peer}(n)$$

Where $\omega_d + \omega_p = 1$ (typically 0.6, 0.4)

```python
def calibrate_reality(self, topic: str) -> float:
    """Combine own experience with peer consensus."""
    # Direct experience
    own_observations = self.get_observations(topic)
    direct_belief = aggregate_observations(own_observations) if own_observations else None
    
    # Peer signals
    peer_beliefs = []
    peer_weights = []
    for partner_id, conn in self.connections.items():
        partner = get_node(partner_id)
        belief = partner.get_belief(topic)
        if belief is not None:
            peer_beliefs.append(belief)
            peer_weights.append(conn.w * partner.trust)
    
    if peer_beliefs:
        peer_consensus = weighted_mean(peer_beliefs, peer_weights)
        peer_confidence = 1 - weighted_variance(peer_beliefs, peer_weights)
    else:
        peer_consensus = None
        peer_confidence = 0
    
    # Combine
    if direct_belief is None and peer_consensus is None:
        return None
    elif direct_belief is None:
        return peer_consensus
    elif peer_consensus is None:
        return direct_belief
    else:
        own_confidence = self.confidence_in_observations(topic)
        omega_d = own_confidence / (own_confidence + peer_confidence)
        omega_p = 1 - omega_d
        return omega_d * direct_belief + omega_p * peer_consensus
```

## 10.9 Two-Stage Defense

```python
class TwoStageDefense:
    """
    Stage 1: General priming (fast, broad)
    Stage 2: Specific response (slower, targeted)
    """
    
    def receive_general_alert(self, signal: DefenseSignal):
        """Stage 1: Become primed. Fast, non-specific."""
        if signal.signal_type != SignalType.GENERAL_ALERT:
            return
        
        self.stage = DefenseStage.PRIMED
        self.node.priming_level += signal.confidence * 0.3
        
        # Primed effects
        self.node.scrutiny_level = 'HIGH'
        self.node.new_connection_threshold += 0.2
        self.node.interaction_logging = True
    
    def receive_specific_threat(self, signal: DefenseSignal):
        """Stage 2: Targeted response. Primed nodes respond faster/stronger."""
        if signal.signal_type != SignalType.SPECIFIC_THREAT:
            return
        
        if self.stage == DefenseStage.PRIMED:
            response_strength = signal.confidence * 1.5  # 50% boost
            response_delay = BASE_DELAY * 0.5  # 50% faster
        else:
            response_strength = signal.confidence
            response_delay = BASE_DELAY
        
        schedule_action(
            delay=response_delay,
            action=lambda: self.execute_defense(signal, response_strength)
        )
```

---

# PART XI: CONVERGENCE DYNAMICS

## 11.1 The Framework

From team research:
> "The path from constructive divergence to convergence is far stronger than fake uniformity."

**Pattern:**
1. **Divergent Phase** — Generate options, voice disagreements
2. **Emergence Phase** — Patterns visible, clusters form
3. **Convergent Phase** — Move toward decision, resolve or park disagreements

## 11.2 Convergence Score

For a task with participants and positions $\{p_i\}$:

$$Conv(task) = 1 - \frac{Var(positions)}{Var_{max}}$$

Normalized position variance. Higher = more agreement.

## 11.3 Convergence States

| Score | State | Meaning | Action |
|-------|-------|---------|--------|
| > 0.85 | CONVERGED | Strong agreement | Ready to decide |
| 0.60 - 0.85 | CONVERGING | Building consensus | Continue |
| 0.40 - 0.60 | EXPLORING | Healthy divergence | Normal |
| 0.20 - 0.40 | STUCK | No progress | Intervention |
| < 0.20 | POLARIZED | Deep disagreement | ATD or escalate |

```python
class ConvergenceTracker:
    """Track convergence/divergence over time for a task."""
    
    def __init__(self, task: Task, participants: List[Node]):
        self.task = task
        self.participants = participants
        self.history = []  # (timestamp, positions, convergence_score)
    
    def record_positions(self, positions: Dict[NodeId, Position]) -> float:
        """Record current positions and compute convergence."""
        distances = []
        nodes = list(positions.keys())
        for i in range(len(nodes)):
            for j in range(i+1, len(nodes)):
                d = position_distance(positions[nodes[i]], positions[nodes[j]])
                distances.append(d)
        
        if not distances:
            return 1.0  # Single participant = converged
        
        variance = np.var(distances)
        max_variance = self.max_possible_variance()
        convergence = 1 - (variance / max_variance)
        
        self.history.append((now(), positions.copy(), convergence))
        return convergence
    
    def get_state(self) -> ConvergenceState:
        """Current convergence state."""
        if not self.history:
            return ConvergenceState.EXPLORING
        
        current = self.history[-1][2]
        
        if current > 0.85: return ConvergenceState.CONVERGED
        elif current > 0.60: return ConvergenceState.CONVERGING
        elif current > 0.40: return ConvergenceState.EXPLORING
        elif current > 0.20: return ConvergenceState.STUCK
        else: return ConvergenceState.POLARIZED
    
    def get_trend(self) -> ConvergenceTrend:
        """Is the group converging or diverging?"""
        if len(self.history) < 3:
            return ConvergenceTrend.UNKNOWN
        
        recent = [h[2] for h in self.history[-5:]]
        slope = np.polyfit(range(len(recent)), recent, 1)[0]
        
        if slope > 0.03: return ConvergenceTrend.CONVERGING
        elif slope < -0.03: return ConvergenceTrend.DIVERGING
        else: return ConvergenceTrend.STABLE
```

## 11.4 Agree-to-Disagree Protocol

When convergence fails, don't force false consensus:

```python
class AgreeToDisagreeProtocol:
    """Structured way to acknowledge persistent disagreement."""
    
    MIN_ROUNDS = 5
    CRITICALITY_THRESHOLD = 0.8  # Don't ATD on critical decisions
    
    def should_invoke(self, tracker: ConvergenceTracker) -> bool:
        """When should we consider agree-to-disagree?"""
        state = tracker.get_state()
        trend = tracker.get_trend()
        rounds = len(tracker.history)
        
        stuck_long_enough = rounds >= self.MIN_ROUNDS
        not_converging = state in [ConvergenceState.STUCK, ConvergenceState.POLARIZED]
        no_progress = trend == ConvergenceTrend.STABLE
        not_critical = tracker.task.criticality < self.CRITICALITY_THRESHOLD
        
        return stuck_long_enough and not_converging and no_progress and not_critical
    
    def invoke(self, tracker: ConvergenceTracker) -> AgreeToDisagreeResult:
        """Execute agree-to-disagree protocol."""
        positions = tracker.history[-1][1]
        
        # 1. Cluster the positions
        clusters = cluster_positions(positions)
        
        # 2. Document the disagreement
        record = DisagreementRecord(
            task=tracker.task,
            clusters=clusters,
            final_convergence=tracker.history[-1][2],
            rounds_attempted=len(tracker.history),
            timestamp=now()
        )
        
        # 3. Choose path forward
        if len(clusters) == 2 and clusters_roughly_equal(clusters):
            path = self.tiebreaker(clusters)  # Need tiebreaker
        else:
            path = self.majority_path(clusters)  # Go with majority
        
        # 4. Record dissent (NOT penalty, just learning)
        for node_id, position in positions.items():
            if position not in path.accepted_positions:
                dissent = DissentRecord(
                    node=node_id,
                    position=position,
                    chosen_path=path,
                    task=tracker.task
                )
                get_node(node_id).dissent_history.append(dissent)
        
        return AgreeToDisagreeResult(
            task=tracker.task,
            chosen_path=path,
            disagreement_record=record,
            dissenting_nodes=[n for n, p in positions.items() 
                            if p not in path.accepted_positions]
        )
```

---

# PART XII: GLOBAL TRUST COMPUTATION

## 12.1 Trust Level

$$T(n) = \frac{w_q \cdot Q_{agg}(n) + w_r \cdot R_{agg}(n) + w_s \cdot S_{social}(n) + w_d \cdot D_{diversity}(n)}{\sum w}$$

Where:
- $Q_{agg}$ = Aggregated quality score
- $R_{agg}$ = Average reciprocity across connections
- $S_{social}$ = Social proof (how many trust this node)
- $D_{diversity}$ = Diversity of connections

## 12.2 Diversity Score

$$D_{diversity}(n) = \frac{|unique\_partners_{last\_100}|}{100}$$

**Diversity Cap (anti-collusion):**

$$T_{capped}(n) = \min(T(n), D_{diversity}(n) + 0.3)$$

```python
def enforce_diversity_requirements(node: Node):
    """Require nodes to interact with diverse others."""
    recent = node.interaction_history[-100:]
    unique_partners = len(set(i.partner for i in recent))
    
    diversity_score = unique_partners / len(recent) if recent else 0
    
    if diversity_score < DIVERSITY_THRESHOLD:  # 0.3
        node.trust_cap = 0.7
        node.flags.add('LOW_DIVERSITY')
    else:
        node.trust_cap = 1.0
        node.flags.discard('LOW_DIVERSITY')
```

---

# PART XIII: ADVERSARY DETECTION

## 13.1 Sycophancy Detection

$$sycophancy\_score = \frac{approval\_rate}{issue\_identification\_rate + \epsilon}$$

Flag if score $> threshold$

## 13.2 Toxicity Detection

$$toxicity\_score = block\_rate \times (1 - resolution\_rate)$$

Flag if score $> threshold$

## 13.3 Collusion Detection

$$correlation(A, B) = \frac{cov(ratings_A, ratings_B)}{\sigma_A \cdot \sigma_B}$$

Flag if $correlation > 0.85$ AND mutual high ratings

```python
def detect_collusion_rings(network: Network) -> List[Set[NodeId]]:
    """Detect groups that only interact with each other with high ratings."""
    G = nx.DiGraph()
    for node in network.nodes:
        for partner_id, conn in node.connections.items():
            if conn.interaction_count > MIN_INTERACTIONS:
                G.add_edge(node.id, partner_id, weight=conn.w)
    
    communities = nx.community.louvain_communities(G.to_undirected())
    suspicious = []
    
    for community in communities:
        if len(community) < 3:
            continue
        
        internal_density = compute_internal_density(G, community)
        external_connections = count_external_edges(G, community)
        mutual_rating = compute_mutual_rating_correlation(community)
        
        is_suspicious = (
            internal_density > 0.8 and
            external_connections < len(community) * 0.5 and
            mutual_rating > 0.9
        )
        
        if is_suspicious:
            suspicious.append(community)
            for node_id in community:
                get_node(node_id).trust *= COLLUSION_PENALTY
                get_node(node_id).flags.add('POTENTIAL_COLLUSION')
    
    return suspicious
```

## 13.4 Strategic Adversary Detection

Pattern: Build trust, then defect.

```python
def detect_strategic_adversary(node: Node, window: int = 100) -> SuspicionLevel:
    """Detect nodes that cooperate to build trust, then exploit."""
    history = node.interaction_history[-window:]
    
    if len(history) < window:
        return SuspicionLevel.NORMAL
    
    early = history[:window//2]
    recent = history[window//2:]
    
    early_quality = mean([i.quality for i in early])
    recent_quality = mean([i.quality for i in recent])
    early_variance = variance([i.quality for i in early])
    
    # Flag 1: Suspiciously perfect early behavior
    if early_quality > 0.95 and early_variance < 0.01:
        log(f"Suspiciously perfect early behavior: {node.id}")
        return SuspicionLevel.ELEVATED
    
    # Flag 2: Quality drop after trust threshold
    if node.trust > TRUST_THRESHOLD:
        quality_drop = early_quality - recent_quality
        if quality_drop > 0.3:
            return SuspicionLevel.HIGH
    
    # Flag 3: Behavioral trajectory analysis
    trajectory = compute_behavioral_trajectory(history)
    if trajectory.shows_strategic_pattern():
        return SuspicionLevel.HIGH
    
    return SuspicionLevel.NORMAL
```

---

# PART XIV: NETWORK TOPOLOGY

## 14.1 Small-World Coefficient

$$\sigma = \frac{C / C_{random}}{L / L_{random}}$$

Where:
- $C$ = clustering coefficient
- $L$ = average path length

Healthy network: $\sigma > 1$ (high clustering, short paths)

## 14.2 Scale-Free Properties

Degree distribution:

$$P(k) \propto k^{-\gamma}$$

Where $\gamma \in [2, 3]$ for healthy scale-free networks

**Properties:**
- Few hub nodes with many connections
- Many nodes with few connections
- Robust to random failure, vulnerable to targeted hub attack

---

# PART XV: THEORETICAL PROPERTIES

## 15.1 Cheater Isolation Theorem

**Theorem:** Exploitative nodes are eventually isolated.

**Proof:**
1. Exploitative node: $\rho < 1$ consistently
2. Therefore: $r = \lambda r + (1-\lambda)\log(\rho) < 0$ (accumulates negative)
3. Therefore: $\sigma(r) < 0$ (negative reciprocity signal)
4. Therefore: $\Phi < 0$ (negative reinforcement)
5. Therefore: $\frac{dw}{dt} < 0$ (connection decays)
6. Eventually: $w \to w_{min}$ then connection pruned ∎

## 15.2 Byzantine Fault Tolerance

**Requirement:** $n \geq 3f + 1$ where $f$ = max Byzantine nodes

With Sybil resistance (via vouching + probation + diversity): adversary cannot easily create $> f$ nodes

## 15.3 Swift Trust Optimality

**Theorem:** Under uncertain quality, starting with $T_{init} > 0$ yields higher expected utility than starting at zero.

**Proof sketch:**
- Let $p$ = probability partner is trustworthy
- $E[trust] = p \cdot U_{cooperate} + (1-p) \cdot U_{exploited}$
- $E[distrust] = 0$
- For typical $p > 0.5$ and $U_{cooperate} > |U_{exploited}|$, trusting has positive expected value ∎

## 15.4 Convergence to Equilibrium

For fair exchange ($\rho = 1$, high $q$, positive $\tau$):

At equilibrium: $\frac{dw}{dt} = 0$

$$\Phi = \alpha w^* \implies w^* = \frac{\gamma |Q|^{\mu} \cdot \sigma(0) \cdot \psi(q) \cdot \phi(\tau)}{\alpha}$$

For fair exchange $\sigma(0) = 0$, so need slight positive reciprocity for stable positive $w$.

---

# PART XVI: NODE LIFECYCLE

```
┌─────────────────────────────────────────────────────────────────────┐
│                         NODE LIFECYCLE                               │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  1. DISCOVERY                                                        │
│     • Node discovered via Veilid DHT                                │
│     • Cryptographic identity verified                               │
│     • Role declaration received                                     │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  2. SWIFT TRUST INITIALIZATION                                       │
│     • Compute T_init from:                                          │
│       - Swift trust baseline (0.4)                                  │
│       - Category-based trust (role + similarity)                    │
│       - Vouching transfer (if introduced)                           │
│       - Social proof (if prior interactions)                        │
│     • Assign mentor                                                 │
│     • Status = PROBATIONARY                                         │
│     • Initial connection weight w_init = 0.3                        │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  3. PROBATION (50 interactions)                                      │
│     • All interactions logged, mentor-reviewed                      │
│     • Quality feedback collected                                    │
│     • Performance tracked                                           │
│     • Defense signals monitored                                     │
│     • After N interactions: evaluate                                │
│       - Pass (quality > 0.6) → MEMBER (trust × 1.5)                │
│       - Fail → Extended probation (trust × 0.8)                    │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  4. MEMBERSHIP                                                       │
│     • Full participation rights                                     │
│     • Trust evolves via:                                            │
│       - Physarum dynamics (connection weights)                      │
│       - Quality feedback aggregation                                │
│       - Reciprocity tracking                                        │
│       - Tone assessment                                             │
│       - Peer affirmations                                           │
│     • Can vouch for newcomers (at risk)                             │
│     • Can send/receive defense signals                              │
│     • Sustained excellence → ESTABLISHED                            │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  5. ESTABLISHED / HUB                                                │
│     • Can mentor newcomers                                          │
│     • Higher weight in consensus voting                             │
│     • Hub status if: high connectivity + quality + diversity        │
│     • Hub nodes are critical network infrastructure                 │
│     • Can initiate BROADCAST defense signals                        │
└─────────────────────────────────────────────────────────────────────┘
```

---

# PART XVII: COMPLETE MASTER ALGORITHM

## 17.0 System Overview — The One Algorithm

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                    Symbiont v0.1 MASTER ORCHESTRATION                            │
│                                                                             │
│  "One algorithm to rule them all"                                          │
└─────────────────────────────────────────────────────────────────────────────┘

                              ┌─────────────┐
                              │   NODE      │
                              │  JOINS      │
                              └──────┬──────┘
                                     │
                                     ▼
                        ┌────────────────────────┐
                        │  SWIFT TRUST INIT      │
                        │  T_init = Σ(weights)   │
                        │  Status = PROBATIONARY │
                        │  Assign mentor         │
                        └───────────┬────────────┘
                                    │
                    ┌───────────────┼───────────────┐
                    │               │               │
                    ▼               ▼               ▼
           ┌──────────────┐ ┌─────────────┐ ┌──────────────┐
           │  INTERACTION │ │  RECEIVE    │ │  BACKGROUND  │
           │  LOOP        │ │  EVENTS     │ │  PROCESSES   │
           │  (per task)  │ │  (async)    │ │  (periodic)  │
           └──────┬───────┘ └──────┬──────┘ └──────┬───────┘
                  │                │               │
                  ▼                ▼               ▼
    ┌─────────────────────────────────────────────────────────┐
    │                    NODE STATE                            │
    │  ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐        │
    │  │ Trust   │ │Connections│ │ Priming │ │Confidence│       │
    │  │ T(n)    │ │ {w,r,q,τ}│ │   π     │ │  S_conf │        │
    │  └─────────┘ └─────────┘ └─────────┘ └─────────┘        │
    └─────────────────────────────────────────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │  STATUS CHECK   │
                    │  Graduation?    │
                    │  Demotion?      │
                    │  Hub promotion? │
                    └─────────────────┘
```

## 17.0.1 The Three Concurrent Processes

The Symbiont system runs THREE concurrent processes:

```python
class MCPNode:
    """
    Complete Symbiont v0.1 Node Implementation
    
    Three concurrent processes:
    1. INTERACTION LOOP — Handle tasks, update connections
    2. EVENT HANDLERS — React to incoming signals
    3. BACKGROUND JOBS — Periodic maintenance
    """
    
    async def run(self):
        """Main entry point — runs all three processes concurrently."""
        await asyncio.gather(
            self.interaction_loop(),
            self.event_handler_loop(),
            self.background_loop()
        )
    
    # ═══════════════════════════════════════════════════════════════
    # PROCESS 1: INTERACTION LOOP (Per-Task)
    # ═══════════════════════════════════════════════════════════════
    
    async def interaction_loop(self):
        """
        Main loop: receive tasks, execute, update state.
        This is where Physarum dynamics happen.
        """
        while self.running:
            task = await self.task_queue.get()
            
            if task.type == TaskType.RESPOND:
                # We are the responder
                result = await self.perform(task)
                await self.send_result(task.initiator, result)
                
            elif task.type == TaskType.INITIATE:
                # We are the initiator
                result = await self.handle_interaction(
                    initiator=self,
                    responder=task.target,
                    task=task
                )
                
            elif task.type == TaskType.GROUP:
                # Group task — convergence dynamics
                result = await self.handle_group_task(task)
    
    # ═══════════════════════════════════════════════════════════════
    # PROCESS 2: EVENT HANDLERS (Async, Reactive)
    # ═══════════════════════════════════════════════════════════════
    
    async def event_handler_loop(self):
        """
        React to incoming events from network.
        Defense signals, affirmations, vouch requests, etc.
        """
        while self.running:
            event = await self.event_queue.get()
            
            if event.type == EventType.DEFENSE_SIGNAL:
                await self.on_defense_signal(event.signal)
                
            elif event.type == EventType.AFFIRMATION:
                await self.on_affirmation_received(event.affirmation)
                
            elif event.type == EventType.VOUCH_REQUEST:
                await self.on_vouch_request(event.request)
                
            elif event.type == EventType.CONSENSUS_VOTE:
                await self.on_consensus_vote(event.vote)
                
            elif event.type == EventType.PEER_POSITION:
                await self.on_peer_position_update(event.position)
    
    async def on_defense_signal(self, signal: DefenseSignal):
        """Handle incoming defense signal."""
        # 1. Update threat belief
        self.update_threat_belief(signal)
        
        # 2. Increase priming (defense readiness)
        self.increase_priming(signal)
        
        # 3. Maybe take defensive action
        if self.threat_beliefs[signal.threat_source].level > ACTION_THRESHOLD:
            await self.take_defensive_action(signal.threat_source)
        
        # 4. Maybe propagate further
        if signal.confidence > RELAY_THRESHOLD:
            await self.propagate_defense_signal(signal.attenuate())
    
    async def on_affirmation_received(self, affirmation: PeerAffirmation):
        """Handle incoming affirmation."""
        if validate_affirmation(affirmation) == AffirmationValidity.VALID:
            self.affirmations_received.append(affirmation)
            self.update_self_confidence()
    
    # ═══════════════════════════════════════════════════════════════
    # PROCESS 3: BACKGROUND JOBS (Periodic)
    # ═══════════════════════════════════════════════════════════════
    
    async def background_loop(self):
        """
        Periodic maintenance tasks.
        Run at different intervals.
        """
        while self.running:
            await asyncio.sleep(BACKGROUND_INTERVAL)
            
            # Every tick
            self.decay_priming()
            self.decay_unused_connections()
            
            # Every N ticks
            if self.tick % DIVERSITY_CHECK_INTERVAL == 0:
                self.enforce_diversity_requirements()
            
            if self.tick % ADVERSARY_CHECK_INTERVAL == 0:
                await self.scan_for_adversaries()
            
            if self.tick % STATUS_CHECK_INTERVAL == 0:
                self.check_status_transitions()
            
            if self.tick % AMBIENT_TONE_INTERVAL == 0:
                self.monitor_ambient_tone()
            
            self.tick += 1
    
    def decay_unused_connections(self):
        """Apply α decay to all connections (Physarum dynamics)."""
        for conn in self.connections.values():
            if now() - conn.last_interaction > IDLE_THRESHOLD:
                conn.w *= (1 - ALPHA * DECAY_MULTIPLIER)
                if conn.w < W_MIN:
                    self.prune_connection(conn)
    
    def decay_priming(self):
        """Priming decays if no threats materialize."""
        self.priming_level *= PRIMING_DECAY
        for threat_type in self.threat_type_priming:
            self.threat_type_priming[threat_type] *= PRIMING_DECAY
    
    async def scan_for_adversaries(self):
        """Periodic scan for strategic adversaries and collusion."""
        for partner_id, conn in self.connections.items():
            partner = get_node(partner_id)
            
            # Check for strategic adversary pattern
            suspicion = detect_strategic_adversary(partner)
            if suspicion >= SuspicionLevel.HIGH:
                partner.flags.add('SUSPECTED_ADVERSARY')
                await self.emit_defense_signal(partner, ThreatType.STRATEGIC_DEFECT)
        
        # Check for collusion rings (network-level)
        if self.status == NodeStatus.HUB:
            rings = detect_collusion_rings(self.local_network_view)
            for ring in rings:
                await self.broadcast_collusion_warning(ring)
    
    def check_status_transitions(self):
        """Check if node should change status."""
        if self.status == NodeStatus.PROBATIONARY:
            self.probation_protocol.check_graduation()
        
        elif self.status == NodeStatus.MEMBER:
            if self.qualifies_for_established():
                self.status = NodeStatus.ESTABLISHED
        
        elif self.status == NodeStatus.ESTABLISHED:
            if self.qualifies_for_hub():
                self.status = NodeStatus.HUB
    
    # ═══════════════════════════════════════════════════════════════
    # THE MASTER UPDATE FUNCTION
    # ═══════════════════════════════════════════════════════════════
    
    def compute_connection_update(self, conn: Connection, interaction: Interaction) -> float:
        """
        THE CORE EQUATION
        
        This is the heart of Symbiont — the Physarum-inspired dynamics
        that determine how connections strengthen or weaken.
        
        dw/dt = Φ(Q,r,q,τ) - αw + η - D
        
        Returns: Δw (change in connection weight)
        """
        # ─────────────────────────────────────────────────────────
        # STEP 1: Compute reinforcement Φ
        # ─────────────────────────────────────────────────────────
        
        # Flow component: |Q|^μ (sublinear in volume)
        Q = interaction.volume
        flow = Q ** MU
        
        # Reciprocity component: σ(r) ∈ [-1, 1]
        sigma_r = 2 / (1 + exp(-BETA * conn.r)) - 1
        
        # Quality component: ψ(q) ∈ [0.5, 1.5]
        psi_q = 0.5 + conn.q
        
        # Tone component: φ(τ) ∈ [0.4, 1.0]
        phi_tau = 0.7 + 0.3 * conn.tau
        
        # Full reinforcement
        Phi = GAMMA * flow * sigma_r * psi_q * phi_tau
        
        # ─────────────────────────────────────────────────────────
        # STEP 2: Compute decay -αw
        # ─────────────────────────────────────────────────────────
        
        decay = ALPHA * conn.w
        
        # ─────────────────────────────────────────────────────────
        # STEP 3: Compute exploration noise η
        # ─────────────────────────────────────────────────────────
        
        # More noise for weak connections (explore)
        # Less noise for strong connections (exploit)
        noise = random.gauss(0, SIGMA_ETA) * (1 - conn.w)
        
        # ─────────────────────────────────────────────────────────
        # STEP 4: Compute defense dampening D
        # ─────────────────────────────────────────────────────────
        
        D = 0
        partner = get_node(conn.partner_id)
        if partner.id in self.threat_beliefs:
            D = DELTA * self.threat_beliefs[partner.id].level
        
        # ─────────────────────────────────────────────────────────
        # STEP 5: Compute total change
        # ─────────────────────────────────────────────────────────
        
        delta_w = (Phi - decay + noise - D) * DELTA_T
        
        return delta_w
```

## 17.0.2 The Complete State Machine

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                         Symbiont NODE STATE MACHINE                              │
└─────────────────────────────────────────────────────────────────────────────┘

    ┌──────────┐      verify        ┌──────────────┐
    │ UNKNOWN  │ ─────────────────► │ DISCOVERED   │
    └──────────┘                    └───────┬──────┘
                                            │
                                            │ swift trust init
                                            ▼
                                    ┌──────────────┐
                              ┌─────│ PROBATIONARY │◄────────────────┐
                              │     └───────┬──────┘                 │
                              │             │                        │
                         fail │             │ pass (q > 0.6)         │ demote
                         (×3) │             │ after 50 interactions  │
                              │             ▼                        │
                              │     ┌──────────────┐                 │
                              │     │    MEMBER    │─────────────────┤
                              │     └───────┬──────┘                 │
                              │             │                        │
                              │             │ sustained excellence   │
                              │             │ (quality > 0.8, 200+)  │
                              │             ▼                        │
                              │     ┌──────────────┐                 │
                              │     │ ESTABLISHED  │─────────────────┤
                              │     └───────┬──────┘                 │
                              │             │                        │
                              │             │ high connectivity +    │
                              │             │ diversity + quality    │
                              │             ▼                        │
                              │     ┌──────────────┐                 │
                              │     │     HUB      │─────────────────┘
                              │     └──────────────┘
                              │
                              ▼
                      ┌──────────────┐
                      │   EXPELLED   │
                      └──────────────┘


    ┌─────────────────────────────────────────────────────────────────────┐
    │  PARALLEL: DEFENSE STATES (can be in any status + defense state)   │
    └─────────────────────────────────────────────────────────────────────┘
    
         ┌──────────┐     signal      ┌──────────┐    threat     ┌──────────┐
         │  NORMAL  │ ──────────────► │  PRIMED  │ ─────────────►│ DEFENDING│
         └──────────┘                 └──────────┘               └──────────┘
              ▲                            │                          │
              │                            │ decay (no threats)       │ resolved
              │                            ▼                          │
              └────────────────────────────┴──────────────────────────┘
```

## 17.0.3 Data Flow Summary

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Symbiont DATA FLOW                                     │
└─────────────────────────────────────────────────────────────────────────────┘

INPUTS                          PROCESSING                        OUTPUTS
──────                          ──────────                        ───────

Task Request ──────────┐
                       │
Feedback ─────────────┐│
                      ││
Exchange Ratio ──────┐││        ┌───────────────────┐
                     │││        │                   │
                     │││   ┌────┤ UPDATE RECIPROCITY├───► r_ij
                     │││   │    │ r = λr + (1-λ)... │
                     ▼▼▼   │    └───────────────────┘
                  ┌──────┐ │
                  │MEASURE│ │    ┌───────────────────┐
                  └──┬───┘ │    │                   │
                     │     ├────┤ UPDATE QUALITY    ├───► q_ij
                     │     │    │ q = λq + (1-λ)q'  │
                     │     │    └───────────────────┘
                     │     │
Tone Indicators ─────┼─────┤    ┌───────────────────┐
                     │     │    │                   │
                     │     ├────┤ UPDATE TONE       ├───► τ_ij
                     │     │    │ τ = λτ + (1-λ)τ'  │
                     │     │    └───────────────────┘
                     │     │
                     ▼     ▼
              ┌─────────────────┐
              │  COMPUTE Φ      │
              │                 │
              │ Φ = γ|Q|^μ ×    │
              │     σ(r) ×      │
              │     ψ(q) ×      │
              │     φ(τ)        │
              └────────┬────────┘
                       │
Defense Signal ────────┼───────► D = δ × threat_level
                       │
                       ▼
              ┌─────────────────┐
              │  UPDATE w       │
              │                 │
              │ Δw = (Φ-αw-D)Δt │───────────────────────► w_ij (connection)
              │ w = clamp(w+Δw) │
              └─────────────────┘
                       │
                       ▼
              ┌─────────────────┐
              │ AGGREGATE       │
              │                 │
              │ T(n) = Σ(w×Q×R× │───────────────────────► T(n) (trust)
              │        S×D)/Σw  │
              └─────────────────┘
                       │
                       ├──────────────────────────────────► Affirmation?
                       │
                       ├──────────────────────────────────► Defense Signal?
                       │
                       └──────────────────────────────────► Status Change?
```

## 17.1 Main Interaction Loop

```python
async def handle_interaction(
    initiator: Node, 
    responder: Node, 
    task: Task
) -> InteractionResult:
    """
    COMPLETE MASTER ALGORITHM
    
    Handles one interaction between two nodes, updating all state:
    - Connection dynamics (Physarum)
    - Reciprocity tracking
    - Quality measurement
    - Tone assessment
    - Defense signaling
    - Affirmation system
    - Convergence tracking
    - Trust computation
    """
    
    # =========================================================
    # PHASE 0: PRE-INTERACTION CHECKS
    # =========================================================
    
    # Check if initiator is primed (defense readiness)
    if initiator.priming_level > 0.5:
        initiator.scrutiny_level = 'HIGH'
        log(f"Initiator {initiator.id} operating in PRIMED state")
    
    # Get or create connection
    conn = initiator.connections.get(responder.id)
    if conn is None:
        # New connection — apply swift trust
        conn = create_connection_with_swift_trust(initiator, responder)
        initiator.connections[responder.id] = conn
    
    # =========================================================
    # PHASE 1: TASK EXECUTION
    # =========================================================
    
    result = await responder.perform(task)
    
    # =========================================================
    # PHASE 2: FEEDBACK & MEASUREMENT
    # =========================================================
    
    # 2.1 Initiator evaluates result
    feedback = initiator.evaluate(result)
    
    # 2.2 Compute quality score
    quality = compute_quality_score(feedback)
    
    # 2.3 Validate feedback integrity
    validity = FeedbackIntegrity().validate_feedback(feedback)
    if validity != FeedbackValidity.VALID:
        quality *= 0.5  # Discount suspicious feedback
        log(f"Suspicious feedback: {validity}")
    
    # 2.4 Extract tone indicators
    tone = extract_tone_indicators(result, feedback)
    
    # 2.5 Compute exchange ratio
    rho = compute_exchange_ratio(initiator, responder, task, result)
    
    # =========================================================
    # PHASE 3: UPDATE RECIPROCITY (Equation 3.2)
    # =========================================================
    
    log_rho = log(rho + EPSILON)
    quality_adj = THETA * (quality - 0.5)
    
    conn.r = LAMBDA * conn.r + (1 - LAMBDA) * (log_rho + quality_adj)
    
    # =========================================================
    # PHASE 4: UPDATE QUALITY SCORE
    # =========================================================
    
    conn.q = LAMBDA * conn.q + (1 - LAMBDA) * quality
    
    # =========================================================
    # PHASE 5: UPDATE TONE SCORE
    # =========================================================
    
    conn.tau = LAMBDA * conn.tau + (1 - LAMBDA) * tone
    
    # =========================================================
    # PHASE 6: COMPUTE REINFORCEMENT (Equation 6.2)
    # =========================================================
    
    # Reciprocity sigmoid
    sigma_r = 2 / (1 + exp(-BETA * conn.r)) - 1
    
    # Quality multiplier
    psi_q = 0.5 + conn.q
    
    # Tone multiplier
    phi_tau = 0.7 + 0.3 * conn.tau
    
    # Flow (task volume)
    Q = task.volume
    
    # Full reinforcement
    Phi = GAMMA * (Q ** MU) * sigma_r * psi_q * phi_tau
    
    # =========================================================
    # PHASE 7: DEFENSE SIGNAL DAMPENING
    # =========================================================
    
    D = 0
    if responder.id in initiator.threat_beliefs:
        threat = initiator.threat_beliefs[responder.id]
        D = DELTA * threat.threat_level
    
    # =========================================================
    # PHASE 8: UPDATE CONNECTION WEIGHT (Equation 6.1)
    # =========================================================
    
    delta_w = (Phi - ALPHA * conn.w - D) * DELTA_T
    conn.w = clamp(conn.w + delta_w, W_MIN, W_MAX)
    
    # =========================================================
    # PHASE 9: PRIMING DECAY
    # =========================================================
    
    initiator.priming_level *= PRIMING_DECAY
    
    # =========================================================
    # PHASE 10: MAYBE GENERATE AFFIRMATION
    # =========================================================
    
    if quality > 0.8 and tone > 0.5:
        affirmation = PeerAffirmation(
            affirmer=initiator.id,
            recipient=responder.id,
            affirmation_type=AffirmationType.QUALITY,
            strength=quality,
            timestamp=now(),
            signature=initiator.sign()
        )
        
        if validate_affirmation(affirmation) == AffirmationValidity.VALID:
            responder.affirmations_received.append(affirmation)
            initiator.affirmations_given.append(affirmation)
            update_self_confidence(responder)
    
    # =========================================================
    # PHASE 11: CHECK FOR THREATS & PROPAGATE DEFENSE SIGNALS
    # =========================================================
    
    if should_warn_about(responder, conn):
        threat_type = determine_threat_type(responder, conn)
        signal = DefenseSignal(
            signal_type=SignalType.SPECIFIC_THREAT,
            sender=initiator.id,
            original_sender=initiator.id,
            threat_source=responder.id,
            threat_type=threat_type,
            confidence=compute_threat_confidence(responder, conn),
            evidence_hash=hash_evidence(conn, feedback),
            hop_count=0,
            timestamp=now(),
            signature=initiator.sign()
        )
        propagate_defense_signal(signal, initiator, network)
    
    # =========================================================
    # PHASE 12: UPDATE CONVERGENCE TRACKING (if group task)
    # =========================================================
    
    if task.is_group_task:
        tracker = get_convergence_tracker(task)
        tracker.record_position(initiator.id, initiator.get_position(task))
        
        state = tracker.get_state()
        if state == ConvergenceState.STUCK:
            if AgreeToDisagreeProtocol().should_invoke(tracker):
                atd_result = AgreeToDisagreeProtocol().invoke(tracker)
                notify_participants(atd_result)
    
    # =========================================================
    # PHASE 13: UPDATE GLOBAL TRUST LEVELS
    # =========================================================
    
    responder.quality_score = update_quality_aggregation(responder, feedback)
    responder.trust = compute_trust_level(responder)
    responder.trust = min(responder.trust, responder.trust_cap)  # Diversity cap
    
    # =========================================================
    # PHASE 14: ADVERSARY DETECTION
    # =========================================================
    
    suspicion = detect_strategic_adversary(responder)
    if suspicion >= SuspicionLevel.HIGH:
        responder.flags.add('SUSPECTED_ADVERSARY')
        responder.trust *= ADVERSARY_PENALTY
    
    # =========================================================
    # PHASE 15: RECORD INTERACTION
    # =========================================================
    
    record = InteractionRecord(
        initiator=initiator.id,
        responder=responder.id,
        task=task,
        result=result,
        feedback=feedback,
        quality=quality,
        tone=tone,
        timestamp=now(),
        signatures=(initiator.sign(), responder.sign())
    )
    
    # Bounded history (keep last N)
    initiator.interaction_history.append(record)
    responder.interaction_history.append(record)
    conn.history.append(record)
    conn.interaction_count += 1
    conn.last_interaction = now()
    
    # =========================================================
    # PHASE 16: PROBATION CHECK (if applicable)
    # =========================================================
    
    if responder.status == NodeStatus.PROBATIONARY:
        responder.probation_protocol.record_interaction(record, feedback)
        responder.probation_protocol.check_graduation()
    
    # =========================================================
    # PHASE 17: DIVERSITY CHECK
    # =========================================================
    
    enforce_diversity_requirements(initiator)
    enforce_diversity_requirements(responder)
    
    return InteractionResult(record=record, success=True)
```

## 17.2 Helper Functions

```python
def create_connection_with_swift_trust(initiator: Node, responder: Node) -> Connection:
    """Create new connection using swift trust initialization."""
    # Compute initial trust
    T_init = (
        OMEGA_S * S_SWIFT +                           # Swift baseline
        OMEGA_C * category_trust(responder) +         # Category
        OMEGA_V * vouching_trust(responder) +         # Vouching
        OMEGA_P * social_proof(responder)             # Social proof
    )
    
    return Connection(
        w=0.3,              # Initial connection strength
        r=0.0,              # Neutral reciprocity
        q=0.5,              # Neutral quality
        tau=0.0,            # Neutral tone
        pi=0.0,             # No priming
        history=[],
        last_interaction=now(),
        interaction_count=0
    )

def compute_trust_level(node: Node) -> float:
    """Compute global trust level (Equation 12.1)."""
    Q_agg = node.quality_score
    R_agg = mean([c.r for c in node.connections.values()]) if node.connections else 0
    S_social = compute_social_proof(node)
    D_div = compute_diversity(node)
    
    T = (W_Q * Q_agg + W_R * sigmoid(R_agg) + W_S * S_social + W_D * D_div) / (W_Q + W_R + W_S + W_D)
    
    # Diversity cap
    T_capped = min(T, D_div + 0.3)
    
    return T_capped

def update_self_confidence(node: Node):
    """Update self-confidence from affirmations (Equation 9.1)."""
    recent = node.affirmations_received[-20:]  # Last 20
    
    if not recent:
        return
    
    weighted_sum = sum(get_node(a.affirmer).trust * a.strength for a in recent)
    weight_total = sum(get_node(a.affirmer).trust for a in recent)
    
    A_bar = weighted_sum / weight_total if weight_total > 0 else 0.5
    
    node.self_confidence = ALPHA_CONF * node.self_confidence + (1 - ALPHA_CONF) * A_bar

def should_warn_about(node: Node, conn: Connection) -> bool:
    """Should we propagate a defense signal about this node?"""
    # Significant quality drop
    if conn.q < 0.3 and conn.interaction_count > 10:
        return True
    
    # Severe reciprocity imbalance
    if conn.r < -1.5:
        return True
    
    # Hostile tone
    if conn.tau < -0.5:
        return True
    
    # Node already flagged
    if 'SUSPECTED_ADVERSARY' in node.flags:
        return True
    
    return False
```

---

# PART XVIII: PARAMETER REFERENCE

## 18.1 Core Dynamics

| Parameter | Symbol | Default | Description |
|-----------|--------|---------|-------------|
| Reinforcement rate | $\gamma$ | 0.1 | Strength gain rate |
| Flow exponent | $\mu$ | 0.5 | Sublinear scaling |
| Decay rate | $\alpha$ | 0.01 | Connection decay |
| Reciprocity sensitivity | $\beta$ | 2.0 | Sigmoid steepness |
| Memory factor | $\lambda$ | 0.9 | EMA weight |
| Quality weight | $\theta$ | 0.5 | Quality in reciprocity |
| Reward sensitivity | $\kappa$ | 1.0 | Resource allocation |
| Min allocation | $f_{min}$ | 0.1 | Floor for allocation |
| Defense dampening | $\delta$ | 0.2 | Signal impact |

## 18.2 Connection Bounds

| Parameter | Default | Description |
|-----------|---------|-------------|
| $w_{min}$ | 0.01 | Minimum strength |
| $w_{max}$ | 1.0 | Maximum strength |
| $w_{init}$ | 0.3 | Initial strength |

## 18.3 Cold Start

| Parameter | Default | Description |
|-----------|---------|-------------|
| Swift trust baseline | 0.4 | $S_{swift}$ |
| Swift weight $\omega_s$ | 0.3 | |
| Category weight $\omega_c$ | 0.2 | |
| Vouching weight $\omega_v$ | 0.3 | |
| Social weight $\omega_p$ | 0.2 | |
| Voucher penalty | 0.5 | Fraction lost if vouchee cheats |
| Probation duration | 50 | Interactions |
| Probation threshold | 0.6 | Quality to graduate |

## 18.4 Quality Measurement

| Parameter | Default | Description |
|-----------|---------|-------------|
| Helpfulness weight | 0.4 | |
| Accuracy weight | 0.3 | |
| Relevance weight | 0.2 | |
| Timeliness weight | 0.1 | |
| Reuse boost | 1.2 | Would use again |
| Reuse penalty | 0.8 | Would not use |

## 18.5 Tone

| Parameter | Default | Description |
|-----------|---------|-------------|
| Tone base | 0.7 | In $\phi(\tau)$ |
| Tone swing | 0.3 | In $\phi(\tau)$ |
| Hostile threshold | -0.3 | Ambient tone warning |
| Polarized variance | 0.5 | Climate warning |

## 18.6 Defense Signaling

| Parameter | Default | Description |
|-----------|---------|-------------|
| Propagation threshold | 0.6 | Min $w$ to propagate |
| Decay per hop | 0.8 | Signal attenuation |
| Min signal strength | 0.1 | Stop below |
| Max hops | 5 | Prevent loops |
| Priming sensitivity | 0.1 | Boost per signal |
| Priming decay | 0.99 | Decay rate |
| Action threshold | 0.7 | Take action above |

## 18.7 Confidence

| Parameter | Default | Description |
|-----------|---------|-------------|
| Confidence memory $\alpha$ | 0.95 | Stability |
| Affirmation rate limit | 10/month | Per node |
| Mutual limit | 3/quarter | Suspicious above |

## 18.8 Convergence

| Parameter | Default | Description |
|-----------|---------|-------------|
| Min rounds for ATD | 5 | Before agree-to-disagree |
| Criticality threshold | 0.8 | Don't ATD if critical |
| Converged | 0.85 | Score threshold |
| Stuck | 0.40 | Score threshold |

## 18.9 Detection

| Parameter | Default | Description |
|-----------|---------|-------------|
| Collusion correlation | 0.85 | Flag above |
| Diversity threshold | 0.3 | Minimum diversity |
| Quality drop trigger | 0.3 | Adversary detection |
| Adversary penalty | 0.5 | Trust multiplier |

---

# PART XIX: REFERENCES

## Psychological Foundations
1. Meyerson, D., Weick, K.E., & Kramer, R.M. (1996). "Swift trust and temporary groups." *Trust in Organizations*.
2. Dunning, D., et al. (2014). "Trust at zero acquaintance." *JPSP*.
3. FeldmanHall, O., & Phelps, E. (2018). "Trust in strangers." *PNAS*.
4. Edmondson, A. (1999). "Psychological safety and learning behavior." *Administrative Science Quarterly*.
5. Bauer, T.N., et al. (2007). "Newcomer adjustment during organizational socialization." *JAP*.

## Biological Foundations
6. Song et al. (2025). "Common mycorrhizal networks facilitate plant disease resistance." *Cell Host & Microbe*.
7. Kiers, E.T., et al. (2011). "Reciprocal rewards stabilize cooperation in mycorrhizal symbiosis." *Science*.
8. Tero, A., et al. (2010). "Rules for biologically inspired adaptive network design." *Science*.
9. Simard, S.W. (2018). "Mycorrhizal Networks Facilitate Tree Communication." *Signaling and Communication in Plants*.
10. Conrath, U. (2011). "Molecular aspects of defence priming." *Trends in Plant Science*.

## Game Theory & Network Science
11. Axelrod, R. (1984). *The Evolution of Cooperation*.
12. Noë, R., & Hammerstein, P. (1994). "Biological markets." *Behavioral Ecology*.
13. Watts, D.J., & Strogatz, S.H. (1998). "Small-world networks." *Nature*.

---

**END OF SPECIFICATION**

*Symbiont: Mycorrhizal Trust Protocol v0.1 — The network's truth emerges from the chorus of its members.*

---

**Document Statistics:**
- 17 major sections
- 30+ mathematical equations
- 15+ algorithms and implementations
- 60+ parameters
- Complete from cold start to adversary detection
