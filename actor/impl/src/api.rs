use std::str::FromStr;

use tea_sdk::actors::tappstore::{txns::TappstoreTxn};
use tea_sdk::utils::wasm_actor::actors::{
	env::{tappstore_id},
	tappstore::{SimpleDate, get_statements_async},
	replica::IntelliSendMode,
};
use serde_json::json;
use primitive_types::H160;
use tea_sdk::tapp::{DOLLARS, Account, Balance, TokenId};
use crate::types::*;
use tea_sdk::utils::client_wasm_actor::{help, check_auth, request, Result};
use idea_vote_state_actor_codec::{
	NAME,
	IdeaQueryRequest,
	txn::{Idea, Txns}
};

const DAO_RESERVED_ACCOUNT: Account = H160([254u8; 20]);
const TARGET_ACTOR: &'static [u8] = NAME;

pub async fn txn_faucet(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: FaucetRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Start faucet action...");

	let txn = TappstoreTxn::TransferTea {
    token_id: tappstore_id().await?,
    from: DAO_RESERVED_ACCOUNT,
		to: req.address.parse()?,
		amount: DOLLARS * 1000,
		auth_b64: req.auth_b64.to_string(),
	};

	request::send_tappstore_txn(
		&from_actor,
		"faucet_txn",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}

pub async fn create_idea(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: CreateIdeaRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Create idea action...");

	let txn = Txns::CreateIdea { 
		id: req.id.to_string(),
		title: req.title.to_string(), 
		description: req.description.to_string(), 
		owner: req.address.parse()?, 
		auth_b64: req.auth_b64.to_string(),
		unit: Balance::from_str_radix(&req.unit, 10)?,
	};

	request::send_custom_txn(
		&from_actor,
		"create_idea",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn vote_idea(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: VoteIdeaRequest = serde_json::from_slice(&payload)?;
  check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("Vote idea action...");

	let txn = Txns::VoteIdea {
		id: req.id.to_string(),
		user: req.address.parse()?, 
		auth_b64: req.auth_b64.to_string(),
		price: Balance::from_str_radix(&req.price, 10)?,
	};

	request::send_custom_txn(
		&from_actor,
		"vote_idea",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn query_idea_list(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: QueryIdeaRequest = serde_json::from_slice(&payload)?;
	info!("Start query_idea_list...");

	let uuid: String = req.uuid.to_string();

	let res = request::send_custom_query(
		&from_actor,
		IdeaQueryRequest {
			owner: None,
		},
		TARGET_ACTOR,
	)
	.await?;

	let r: Vec<Idea>  = res.0;
	let x = serde_json::json!({
		"list": r,
	});
	info!("query_idea_list => {:?}", x);
	help::cache_json_with_uuid(&uuid, x).await?;

	help::result_ok()
}

pub async fn init_db(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: InitAppDBRequest = serde_json::from_slice(&payload)?;
	info!("Init DB action...");

	let txn = Txns::Init {};

	request::send_custom_txn(
		&from_actor,
		"init_db",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		tea_sdk::serialize(&txn)?,
		vec![],
		TARGET_ACTOR,
	)
	.await?;

	help::result_ok()
}

pub async fn init_token(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: InitAppTokenRequest = serde_json::from_slice(&payload)?;
	info!("Init token action...");

	let txn = TappstoreTxn::GenAesKey {
    token_id: TokenId::from_hex(&req.token_id)?,
	};

	request::send_tappstore_txn(
		&from_actor,
		"faucet_txn",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonStatement {
	pub account: String,
	pub gross_amount: String,
	pub statement_type: String,
	pub token_id: String,
	pub state_type: String,
	pub memo: String,
	pub time: String,
}
pub async fn query_op_logs(payload: Vec<u8>, _from_actor: String) -> Result<Vec<u8>> {
	let req: QueryOpLogsRequest = serde_json::from_slice(&payload)?;
	info!("query_op_logs... => {:?}", req);
	let acct: Option<Account> = match &req.target {
		Some(acct) => Some(acct.parse()?),
		None => None,
	};

	let uuid = req.uuid.clone();

	let date: Option<SimpleDate> = req
		.year
		.as_ref()
		.map(|year| SimpleDate::new(*year, req.month.unwrap_or(1_u32), req.day.unwrap_or(1_u32)));

	let (statements, read_to_end) = get_statements_async(
		acct,
		date,
		IntelliSendMode::RemoteOnly,
	)
	.await?;

	info!(
		"read to end {}, get statements result: {:?}",
		read_to_end, statements
	);

	let mut rows: Vec<JsonStatement> = Vec::new();
	for item in statements {
		let s = item.0;
		let tmp = JsonStatement {
			account: format!("{:?}", s.statement.account),
			gross_amount: s.statement.gross_amount.to_string(),
			statement_type: s.statement.statement_type.to_string(),
			token_id: s.statement.token_id.to_hex(),
			state_type: s.state_type.to_string(),
			memo: item.2,
			time: item.1,
		};
		rows.push(tmp);
	}
	info!("log rows => {:?}", rows);
	let x = json!({ "logs": rows });
	help::cache_json_with_uuid(&uuid, x).await?;

	help::result_ok()
}

pub async fn set_allowance(payload: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let req: SetAllowanceRequest = serde_json::from_slice(&payload)?;
	check_auth(&req.tapp_id_b64, &req.address, &req.auth_b64).await?;
	info!("set allowance action... {:?}", req);

	let txn = TappstoreTxn::SetAllowance {
		address: req.address.parse()?,
		token_id: TokenId::from_hex(&req.target_tapp_id_b64)?,
		amount: Balance::from(u128::from_str(&req.amount)?),
	};

	request::send_tappstore_txn(
		&from_actor,
		"set_allowance",
		&req.uuid,
		tea_sdk::serialize(&req)?,
		txn,
		vec![],
	)
	.await?;
	help::result_ok()
}