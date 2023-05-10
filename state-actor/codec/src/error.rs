use tea_sdk::{actorx::error::ActorX, define_scope};

define_scope! {
    IdeaVoteActor: ActorX {
        HttpActionNotSupported;
        TxnErrors;
    }
}
