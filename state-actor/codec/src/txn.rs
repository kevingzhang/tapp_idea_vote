use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString};
use tea_sdk::tapp::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    pub id: String,
    pub title: String,
    pub description: String,
    pub owner: Account,
    pub create_at: u64,
    pub vote_num: u64,
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, AsRefStr, EnumString, Display,
)]
pub enum Status {
    New,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsRefStr, Display)]
pub enum Txns {
    Init {},
    CreateIdea {
        id: String,
        title: String,
        description: String,
        owner: Account,
        auth_b64: String,
    },
    VoteIdea {
        id: String,
        user: Account,
        auth_b64: String,
    },
}
