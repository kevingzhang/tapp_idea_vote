use crate::{error::Result, utils::my_token_id};
use tea_sdk::{
    actor_txns::{context::TokenContext, Tsid},
    serialize,
    tapp::{Account, TokenId, DOLLARS, PUBLIC_RESERVED_ACCOUNT},
    utils::wasm_actor::actors::{env::tappstore_id, tokenstate::api_cross_move},
    ResultExt,
};

pub(crate) async fn deposit_for_idea(
    tsid: Tsid,
    base: Tsid,
    from: Account,
    ctx: Vec<u8>,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let tappstore_ctx = tappstore_ctx(tsid, base, Some(my_token_id().await?)).await?;
    api_cross_move(from, PUBLIC_RESERVED_ACCOUNT, DOLLARS, tappstore_ctx, ctx)
        .await
        .err_into()
}

pub(crate) async fn tappstore_ctx(
    tsid: Tsid,
    base: Tsid,
    allowance_tid: Option<TokenId>,
) -> Result<Vec<u8>> {
    if let Some(token_id) = allowance_tid {
        serialize(&TokenContext::new_cross_move(
            tsid,
            base,
            tappstore_id().await?,
            token_id,
        ))
        .err_into()
    } else {
        serialize(&TokenContext::new_slim(tsid, base, tappstore_id().await?)).err_into()
    }
}
