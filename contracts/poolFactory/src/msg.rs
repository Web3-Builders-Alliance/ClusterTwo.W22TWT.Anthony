use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{query::ConfigResponse};

#[cw_serde]
pub struct InitMsg {
    pub admin: String,
    pub pool_code_id: u64,
}

#[cw_serde]
pub struct InitPoolMsg {
    pub admin: String,
    pub title: String,
    // pub pool_id: u64,
}

#[cw_serde]
pub struct MigrateMsg {
    
}

#[cw_serde]
pub enum ExecuteMsg {
    CreatePool { title: String},
    RedirectFund { pool_id: u64},
}

#[cw_serde]
pub enum QueryMsg {
    
    PoolAddress { pool_id: u64 },
    Config {},
}

