use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::user::models::user::UserId;

pub mod events;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionState {
    Suggested {
        votes: HashMap<UserId, i32>,
        user_id: UserId,
    },
    Active,
    Rejected,
}

pub trait Suggestable {
    fn get_state_mut(&mut self) -> &mut SuggestionState;

    fn add_vote(&mut self, user_id: UserId, power: i32) {
        match self.get_state_mut() {
            SuggestionState::Suggested { votes, .. } => {
                votes.insert(user_id, power);
            }
            _ => {
                // Do nothing, votes can only be added to suggested festivals
            }
        }
    }

    fn remove_vote(&mut self, user_id: UserId) {
        match self.get_state_mut() {
            SuggestionState::Suggested { votes, .. } => {
                votes.remove(&user_id);
            }
            _ => {
                // Do nothing, votes can only be removed from suggested festivals
            }
        }
    }

    fn accept(&mut self) {
        let state = self.get_state_mut();
        match state {
            SuggestionState::Suggested { .. } => {
                *state = SuggestionState::Active;
            }
            _ => {
                // Do nothing, only suggested festivals can be accepted
            }
        }
    }

    fn reject(&mut self) {
        let state = self.get_state_mut();
        match state {
            SuggestionState::Suggested { .. } => {
                *state = SuggestionState::Rejected;
            }
            _ => {
                // Do nothing, only suggested festivals can be rejected
            }
        }
    }
}
