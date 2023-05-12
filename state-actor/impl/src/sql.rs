use crate::{error::Result, utils::my_token_id};
use idea_vote_state_actor_codec::txn::{Idea};
use log::info;
use tea_sdk::vmh::utils::to_short_timestamp;
use tea_sdk::{
    actor_txns::Tsid,
    actors::tokenstate::{ExecGlueCmdRequest, InitGlueSqlRequest, NAME},
    actorx::ActorId,
    serialize,
    tapp::{Account, Balance},
    utils::wasm_actor::{
        actors::tokenstate::{
            query_first_row, query_select_rows, sql_query_first,
            sql_value_to_string, sql_value_to_u64,
        },
        prelude::Row,
    },
    vmh::message::{encode_protobuf, structs_proto::tokenstate},
    OptionExt,
};

pub(crate) async fn query_all_ideas() -> Result<Vec<Idea>> {
    let payload = sql_query_first(
        my_token_id().await?,
        "SELECT * FROM Ideas;".into(),
    )
    .await?;
    let rows = query_select_rows(&payload)?;
    rows.iter().map(|v| parse_idea(v)).collect()
}

pub(crate) async fn query_by_id(id: &str) -> Result<Idea> {
    let payload = sql_query_first(
        my_token_id().await?,
        format!("SELECT * FROM Ideas WHERE id = '{id}';"),
    )
    .await?;
    let r = query_first_row(&payload)?;
    parse_idea(r)
}

pub(crate) async fn query_ideas_by_owner(owner: Account) -> Result<Vec<Idea>> {
    let payload = sql_query_first(
        my_token_id().await?,
        format!("SELECT * FROM Ideas WHERE owner = '{owner:?}';"),
    )
    .await?;
    let rows = query_select_rows(&payload)?;
    rows.iter().map(|v| parse_idea(v)).collect()
}

pub(crate) async fn create_idea(
    tsid: Tsid,
    id: String,
    title: String,
    description: String,
    owner: Account,
    unit: Balance,
) -> Result<()> {
    let sql = format!(
        r#"
    INSERT INTO Ideas VALUES (
        '{id}', '{title}', '{description}', '{owner:?}', {create_at}, '{total_contribution}'
    );
        "#,
        create_at = to_short_timestamp(tsid.ts)?,
        total_contribution = unit.to_string(),
    );
    exec_sql(tsid, sql).await
}

pub(crate) async fn vote_idea(tsid: Tsid, id: String, _user: Account, price: Balance) -> Result<()> {
    let idea = query_by_id(&id).await?;
    let total_contribution = Balance::from_str_radix(&idea.total_contribution, 10)?;
    let new_total_contribution = total_contribution.checked_add(price).ok_or_err("add overflow")?;
    exec_sql(
        tsid,
        format!("UPDATE Ideas SET total_contribution = '{}' WHERE id = '{id}';", new_total_contribution.to_string()),
    )
    .await?;

    Ok(())
}

pub(crate) async fn sql_init(tsid: Tsid) -> Result<()> {
    let req = tokenstate::InitGlueSqlRequest {
        token_id: serialize(&my_token_id().await?)?,
        tsid: serialize(&tsid)?,
    };
    ActorId::Static(NAME)
        .call(InitGlueSqlRequest(encode_protobuf(req)?))
        .await?;

    let sql = String::from_utf8(include_bytes!("tables.sql").to_vec())?;
    exec_sql(tsid, sql).await
}

async fn exec_sql(tsid: Tsid, sql: String) -> Result<()> {
    info!("sql => {sql:?}");
    let req = tokenstate::ExecGlueSqlRequest {
        token_id: serialize(&my_token_id().await?)?,
        sql,
        tsid: serialize(&tsid)?,
    };
    ActorId::Static(NAME)
        .call(ExecGlueCmdRequest(encode_protobuf(req)?))
        .await?;
    info!("SQL executed successfully.");

    Ok(())
}

fn parse_idea(v: &Row) -> Result<Idea> {
    let idea = Idea {
        id: sql_value_to_string(v.get_value_by_index(0).ok_or_err("id")?)?.to_string(),
        title: sql_value_to_string(v.get_value_by_index(1).ok_or_err("title")?)?.to_string(),
        description: sql_value_to_string(v.get_value_by_index(2).ok_or_err("description")?)?
            .to_string(),
        owner: sql_value_to_string(v.get_value_by_index(3).ok_or_err("owner")?)?.parse()?,
        create_at: sql_value_to_u64(v.get_value_by_index(4).ok_or_err("0")?)?,
        total_contribution: sql_value_to_string(v.get_value_by_index(5).ok_or_err("total_contribution")?)?.to_string(),
    };
    Ok(idea)
}
