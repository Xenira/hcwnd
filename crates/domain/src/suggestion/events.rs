use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuggestionVoted {
    pub user_id: Uuid,
    pub power: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuggestionVoteRemoved {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuggestionAccepted {
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuggestionRejected {
    pub user_id: Uuid,
}

macro_rules! impl_suggestion_events {
    ($t:ty, $f:ident) => {
        impl crate::suggestion::Suggestable for $t {
            fn get_state_mut(&mut self) -> &mut crate::suggestion::SuggestionState {
                self.$f()
            }
        }

        impl Apply<crate::suggestion::events::SuggestionVoted> for $t {
            fn apply(&mut self, event: crate::suggestion::events::SuggestionVoted) {
                crate::suggestion::Suggestable::add_vote(
                    self,
                    UserId::new(event.user_id),
                    event.power,
                );
            }
        }

        impl Apply<crate::suggestion::events::SuggestionVoteRemoved> for $t {
            fn apply(&mut self, event: crate::suggestion::events::SuggestionVoteRemoved) {
                crate::suggestion::Suggestable::remove_vote(self, UserId::new(event.user_id));
            }
        }

        impl Apply<crate::suggestion::events::SuggestionAccepted> for $t {
            fn apply(&mut self, _event: crate::suggestion::events::SuggestionAccepted) {
                crate::suggestion::Suggestable::accept(self);
            }
        }

        impl Apply<crate::suggestion::events::SuggestionRejected> for $t {
            fn apply(&mut self, _event: crate::suggestion::events::SuggestionRejected) {
                crate::suggestion::Suggestable::reject(self);
            }
        }
    };
}
pub(crate) use impl_suggestion_events;
