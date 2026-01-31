//! State machine service for noun state transitions

use crate::error::{LevelError, Result};
use crate::models::NounState;
use std::collections::HashMap;

/// State transition rules
pub struct StateMachine;

impl StateMachine {
    /// Get valid transitions map
    fn transitions() -> HashMap<NounState, Vec<NounState>> {
        let mut map = HashMap::new();

        map.insert(
            NounState::Normal,
            vec![NounState::Escalated, NounState::Completed, NounState::Incompleted],
        );
        map.insert(
            NounState::Escalated,
            vec![NounState::Normal, NounState::Completed, NounState::Incompleted],
        );
        map.insert(NounState::Completed, vec![NounState::Closed]);
        map.insert(NounState::Incompleted, vec![NounState::Closed]);
        map.insert(NounState::Closed, vec![NounState::Archived]);
        map.insert(NounState::Archived, vec![]);

        map
    }

    /// Check if transition is valid
    pub fn is_valid_transition(from: NounState, to: NounState) -> bool {
        Self::transitions()
            .get(&from)
            .map(|valid| valid.contains(&to))
            .unwrap_or(false)
    }

    /// Validate and return error if invalid
    pub fn validate_transition(from: NounState, to: NounState) -> Result<()> {
        if Self::is_valid_transition(from, to) {
            Ok(())
        } else {
            Err(LevelError::InvalidStateTransition {
                from: from.to_string(),
                to: to.to_string(),
            })
        }
    }

    /// Get valid next states from current state
    pub fn valid_next_states(from: NounState) -> Vec<NounState> {
        Self::transitions()
            .get(&from)
            .cloned()
            .unwrap_or_default()
    }

    /// Check if Complete verb can be applied
    pub fn can_complete(state: NounState) -> bool {
        matches!(state, NounState::Normal | NounState::Escalated)
    }

    /// Check if Incomplete verb can be applied
    pub fn can_incomplete(state: NounState) -> bool {
        matches!(state, NounState::Normal | NounState::Escalated)
    }

    /// Check if Escalate verb can be applied
    pub fn can_escalate(state: NounState) -> bool {
        matches!(state, NounState::Normal)
    }

    /// Check if Normal (de-escalate) verb can be applied
    pub fn can_normalize(state: NounState) -> bool {
        matches!(state, NounState::Escalated)
    }

    /// Check if Close verb can be applied
    pub fn can_close(state: NounState) -> bool {
        matches!(state, NounState::Completed | NounState::Incompleted)
    }

    /// Check if state is terminal
    pub fn is_terminal(state: NounState) -> bool {
        matches!(state, NounState::Archived)
    }

    /// Check if most verbs can be applied (Normal or Escalated)
    pub fn can_modify(state: NounState) -> bool {
        matches!(state, NounState::Normal | NounState::Escalated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions_from_normal() {
        assert!(StateMachine::is_valid_transition(NounState::Normal, NounState::Escalated));
        assert!(StateMachine::is_valid_transition(NounState::Normal, NounState::Completed));
        assert!(StateMachine::is_valid_transition(NounState::Normal, NounState::Incompleted));
        assert!(!StateMachine::is_valid_transition(NounState::Normal, NounState::Closed));
        assert!(!StateMachine::is_valid_transition(NounState::Normal, NounState::Archived));
    }

    #[test]
    fn test_valid_transitions_from_escalated() {
        assert!(StateMachine::is_valid_transition(NounState::Escalated, NounState::Normal));
        assert!(StateMachine::is_valid_transition(NounState::Escalated, NounState::Completed));
        assert!(StateMachine::is_valid_transition(NounState::Escalated, NounState::Incompleted));
    }

    #[test]
    fn test_valid_transitions_from_completed() {
        assert!(StateMachine::is_valid_transition(NounState::Completed, NounState::Closed));
        assert!(!StateMachine::is_valid_transition(NounState::Completed, NounState::Normal));
    }

    #[test]
    fn test_archived_is_terminal() {
        assert!(StateMachine::is_terminal(NounState::Archived));
        assert!(!StateMachine::is_valid_transition(NounState::Archived, NounState::Normal));
    }

    #[test]
    fn test_can_complete() {
        assert!(StateMachine::can_complete(NounState::Normal));
        assert!(StateMachine::can_complete(NounState::Escalated));
        assert!(!StateMachine::can_complete(NounState::Completed));
        assert!(!StateMachine::can_complete(NounState::Closed));
    }

    #[test]
    fn test_can_close() {
        assert!(StateMachine::can_close(NounState::Completed));
        assert!(StateMachine::can_close(NounState::Incompleted));
        assert!(!StateMachine::can_close(NounState::Normal));
    }
}
