#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaucetRequest {
	pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateIdeaRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub id: String,
  pub title: String,
  pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoteIdeaRequest {
  pub uuid: String,
  pub address: String,
  pub tapp_id_b64: String,
  pub auth_b64: String,

  pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryIdeaRequest {
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAppTokenRequest {
  pub token_id: String, 
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitAppDBRequest {
  pub token_id: String, 
  pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryOpLogsRequest {
	pub uuid: String,
	pub target: Option<String>,
	pub year: Option<i32>,
	pub month: Option<u32>,
	pub day: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAllowanceRequest {
	pub uuid: String,
	pub tapp_id_b64: String,
	pub target_tapp_id_b64: String,
	pub address: String,
	pub auth_b64: String,
	pub amount: String,
}