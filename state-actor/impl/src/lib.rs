#![feature(min_specialization)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use error::{Result};
use idea_vote_state_actor_codec::{txn::Txns, NAME, *};
use log::{error, info};
use tea_sdk::{
    actors::{replica::ExecTxnCast, state_receiver::ActorTxnCheckMessage},
    actorx::{actor, hooks::Activate, ActorId, HandlerActor},
    deserialize,
    serde::handle::{Handle, Handles},
    utils::wasm_actor::{action::process_txn_error, logging::set_logging},
    Handle,
};

mod account;
pub mod error;
mod sql;
mod txn;
mod utils;

actor!(Actor);

#[derive(Default, Clone)]
pub struct Actor;

impl Handles for Actor {
    type List = Handle![
        Activate,
        IdeaQueryRequest,
        ExecTxnCast,
        ActorTxnCheckMessage
    ];
}

impl HandlerActor for Actor {
    fn id(&self) -> Option<ActorId> {
        Some(NAME.into())
    }

    async fn pre_handle<'a>(&'a self, req: &'a [u8]) -> Result<std::borrow::Cow<'a, [u8]>> {
        set_logging(false, false);
        Ok(std::borrow::Cow::Borrowed(req))
    }
}

impl Handle<Activate> for Actor {
    async fn handle(&self, _: Activate) -> Result<()> {
        info!("activate {:?} successfully.", NAME);
        Ok(())
    }
}

impl Handle<IdeaQueryRequest> for Actor {
    async fn handle(&self, req: IdeaQueryRequest) -> Result<IdeaQueryResponse> {
        if req.owner.is_none() {
            Ok(IdeaQueryResponse(sql::query_all_ideas().await?))
        } else {
            Ok(IdeaQueryResponse(
                sql::query_ideas_by_owner(req.owner.unwrap()).await?,
            ))
        }
    }
}

impl Handle<ExecTxnCast> for Actor {
    async fn handle(&self, ExecTxnCast(tsid, txn_bytes, _args): ExecTxnCast) -> Result<()> {
        let txn: Txns = deserialize(txn_bytes)?;
        if let Err(e) = txn::txn_exec(tsid, &txn).await {
            error!("exec txn error: {}", e);
            process_txn_error(tsid, e.into()).await?;
        }
        Ok(())
    }
}

impl Handle<ActorTxnCheckMessage> for Actor {
    async fn handle(&self, req: ActorTxnCheckMessage) -> Result<()> {
        let _txn: Txns = deserialize(req.txn_bytes.as_slice())?;

        Ok(())
    }
}
