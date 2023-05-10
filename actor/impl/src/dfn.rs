use crate::error::Result;
use crate::api;
pub fn name_list() -> Vec<&'static str> {
	vec![
		"ping",
		"faucet",
		"create_idea",
		"query_idea_list",
		"vote_idea",
		"init_db",
		"init_token",
		"queryOpLogs",
		"setAllowance",
	]
}

pub async fn map_handler(action: &str, arg: Vec<u8>, from_actor: String) -> Result<Vec<u8>> {
	let res = match action {
		"ping" => serde_json::to_vec("pong").unwrap(),
		"faucet" => api::txn_faucet(arg, from_actor).await?,
		"create_idea" => api::create_idea(arg, from_actor).await?,
		"query_idea_list" => api::query_idea_list(arg, from_actor).await?,
		"vote_idea" => api::vote_idea(arg, from_actor).await?,
		"init_db" => api::init_db(arg, from_actor).await?,
		"init_token" => api::init_token(arg, from_actor).await?,
		"queryOpLogs" => api::query_op_logs(arg, from_actor).await?,
		"setAllowance" => api::set_allowance(arg, from_actor).await?,

		_ => vec![],
	};
	Ok(res)
}
