use cosmwasm_schema::{cw_serde, QueryResponses};


#[cw_serde]
pub struct InitMsg {
   pub admin: String,
   pub title: String,
}

#[cw_serde]
pub struct MigrateMsg {
    // pub default_gas_limit: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    WithdrawFunds{recipient: String},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config{}
}

#[cw_serde]
pub struct ConfigResponse {
   pub admin: String,
   pub title: String,
}
