use idea_vote_actor_codec::error::IdeaVoteActor;
use tea_sdk::define_scope;
use thiserror::Error;

define_scope! {
    Impl: IdeaVoteActor {
        HttpActionNotSupported => @IdeaVoteActor::HttpActionNotSupported;
    }
}

#[derive(Debug, Error)]
#[error("Http method {0} is not supported")]
pub struct HttpActionNotSupported(pub String);

#[derive(Debug, Error)]
#[error("Greeting name is empty")]
pub struct GreetingNameEmpty;
