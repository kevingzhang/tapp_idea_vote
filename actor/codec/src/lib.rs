#![feature(min_specialization)]

use serde::{Deserialize, Serialize};
use tea_sdk::serde::TypeId;

pub mod error;

pub const NAME: &[u8] = b"com.developer.idea-vote-actor";

