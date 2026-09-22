pub enum Tier {
    New,         // Propose, endorse
    Contributor, // Upvote
    Voter,       // Downvote
    Trusted,     // Auto-approval, full visibility
    Editor,      // Approve, reject
    Moderator,   // Veto, protect
    Admin,       // *
}

impl From<i32> for Tier {
    fn from(score: i32) -> Self {
        match score {
            0..=9 => Tier::New,
            10..=29 => Tier::Contributor,
            30..=99 => Tier::Voter,
            100..=199 => Tier::Trusted,
            200..=499 => Tier::Editor,
            _ => Tier::Moderator,
        }
    }
}

pub enum Permission {
    Propose,
    Upvote,
    Downvote,
    Approve,
    Reject,
    Protect,
    Suspend,
    Ban,
}

#[derive(Debug, Clone, Default)]
pub struct UserPermissions {
    propose: bool,
    upvote: bool,
    downvote: bool,
    approve: bool,
    reject: bool,
    protect: bool,
    suspend: bool,
    ban: bool,
}

impl From<Tier> for UserPermissions {
    fn from(tier: Tier) -> Self {
        match tier {
            Tier::New => UserPermissions {
                propose: true,
                ..Default::default()
            },
            Tier::Contributor => UserPermissions {
                propose: true,
                upvote: true,
                ..Default::default()
            },
            Tier::Voter => UserPermissions {
                propose: true,
                upvote: true,
                downvote: true,
                ..Default::default()
            },
            Tier::Trusted => UserPermissions {
                propose: true,
                upvote: true,
                downvote: true,
                approve: true,
                reject: true,
                ..Default::default()
            },
            Tier::Editor => UserPermissions {
                propose: true,
                upvote: true,
                downvote: true,
                approve: true,
                reject: true,
                protect: true,
                ..Default::default()
            },
            Tier::Moderator => UserPermissions {
                propose: true,
                upvote: true,
                downvote: true,
                approve: true,
                reject: true,
                protect: true,
                suspend: true,
                ban: true,
            },
            Tier::Admin => UserPermissions {
                propose: true,
                upvote: true,
                downvote: true,
                approve: true,
                reject: true,
                protect: true,
                suspend: true,
                ban: true,
            },
        }
    }
}
