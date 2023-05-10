#![feature(min_specialization)]

use serde::{Deserialize, Serialize};
use tea_sdk::{serde::TypeId, tapp::Account};

pub mod error;
pub mod txn;

pub const NAME: &[u8] = b"com.developer.idea-vote-state-actor-executor";

#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct IdeaQueryRequest {
    pub owner: Option<Account>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypeId)]
pub struct IdeaQueryResponse(pub Vec<txn::Idea>);
