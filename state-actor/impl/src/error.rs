use idea_vote_state_actor_codec::error::IdeaVoteActor;
use tea_sdk::{define_scope, tapp::Account};
use thiserror::Error;

define_scope! {
    Impl: IdeaVoteActor {
        HttpActionNotSupported => @IdeaVoteActor::HttpActionNotSupported;
        TxnErrors => @IdeaVoteActor::TxnErrors;
    }
}

#[derive(Debug, Error)]
#[error("Http method {0} is not supported")]
pub struct HttpActionNotSupported(pub String);

#[derive(Debug, Error)]
pub enum TxnErrors {
    #[error("Account {0:?} is not allowed to operate task")]
    InvalidAccount(Account),
}
