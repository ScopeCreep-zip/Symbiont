//! Discrete event system for simulation.

use symbiont_core::defense::DefenseSignal;
use symbiont_core::node::Node;
use symbiont_core::types::{CapabilityId, NodeId, Score, SignedScore};

/// Agent behavior type for delayed agent assignment
#[derive(Debug, Clone)]
pub enum AgentType {
    /// Honest agent with interaction rate
    Honest { interaction_rate: f64, base_quality: f64 },
    /// Strategic adversary with defection tick
    Strategic { defection_tick: u64 },
    /// Free rider
    FreeRider { interaction_rate: f64 },
    /// Sybil cluster member
    Sybil { cluster_members: Vec<NodeId> },
    /// Passive agent
    Passive,
}

/// Events that can occur in the simulation
#[derive(Debug, Clone)]
pub enum Event {
    /// An interaction between two nodes
    Interaction {
        from: NodeId,
        to: NodeId,
        volume: f64,
        quality: Score,
        tone: SignedScore,
        capability: Option<CapabilityId>,
    },

    /// A defense signal was emitted
    DefenseSignal {
        signal: DefenseSignal,
    },

    /// A new node joins the network with optional agent behavior
    NodeJoin {
        node: Node,
        agent_type: Option<AgentType>,
    },

    /// A node leaves the network
    NodeLeave {
        node_id: NodeId,
    },
}

impl Event {
    /// Create an interaction event
    pub fn interaction(
        from: NodeId,
        to: NodeId,
        quality: Score,
    ) -> Self {
        Self::Interaction {
            from,
            to,
            volume: 1.0,
            quality,
            tone: SignedScore::ZERO,
            capability: None,
        }
    }

    /// Create a node join event without agent
    pub fn node_join(node: Node) -> Self {
        Self::NodeJoin { node, agent_type: None }
    }

    /// Create a node join event with agent behavior
    pub fn node_join_with_agent(node: Node, agent_type: AgentType) -> Self {
        Self::NodeJoin { node, agent_type: Some(agent_type) }
    }

    /// Create a node leave event
    pub fn node_leave(node_id: NodeId) -> Self {
        Self::NodeLeave { node_id }
    }
}

/// A scheduled event with timing
#[derive(Debug, Clone)]
pub struct ScheduledEvent {
    /// When to execute (tick number)
    pub at_tick: u64,
    /// The event to execute
    pub event: Event,
}

impl ScheduledEvent {
    /// Create a new scheduled event
    pub fn new(at_tick: u64, event: Event) -> Self {
        Self { at_tick, event }
    }
}

/// Event scheduler for time-based events
#[derive(Debug, Default)]
pub struct EventScheduler {
    /// Scheduled events (sorted by tick)
    events: Vec<ScheduledEvent>,
}

impl EventScheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self::default()
    }

    /// Schedule an event
    pub fn schedule(&mut self, at_tick: u64, event: Event) {
        self.events.push(ScheduledEvent::new(at_tick, event));
        self.events.sort_by_key(|e| e.at_tick);
    }

    /// Get events due at the given tick
    pub fn due_at(&mut self, tick: u64) -> Vec<Event> {
        let mut due = Vec::new();
        while let Some(scheduled) = self.events.first() {
            if scheduled.at_tick <= tick {
                due.push(self.events.remove(0).event);
            } else {
                break;
            }
        }
        due
    }

    /// Check if there are pending events
    pub fn has_pending(&self) -> bool {
        !self.events.is_empty()
    }

    /// Get the next scheduled tick
    pub fn next_tick(&self) -> Option<u64> {
        self.events.first().map(|e| e.at_tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_scheduler() {
        let mut scheduler = EventScheduler::new();

        scheduler.schedule(10, Event::node_leave(NodeId::from_index(1)));
        scheduler.schedule(5, Event::node_leave(NodeId::from_index(2)));
        scheduler.schedule(15, Event::node_leave(NodeId::from_index(3)));

        // Events should be sorted
        assert_eq!(scheduler.next_tick(), Some(5));

        // Get events at tick 5
        let events = scheduler.due_at(5);
        assert_eq!(events.len(), 1);

        // Get events at tick 12 (should include tick 10)
        let events = scheduler.due_at(12);
        assert_eq!(events.len(), 1);

        // Tick 15 event still pending
        assert!(scheduler.has_pending());
    }
}
