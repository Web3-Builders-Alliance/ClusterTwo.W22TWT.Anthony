use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Deps, Addr};

use crate::state::{CONFIG, POOLS};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    
    let res = ConfigResponse {
        
        admin:  cfg.admin.into(),
        pool_code_id: cfg.pool_code_id,
        // pool_addr: cfg.pool_addr
    };
    Ok(res)
}

// settings for pagination
// const MAX_LIMIT: u32 = 30;
// const DEFAULT_LIMIT: u32 = 10;

#[cw_serde]
pub struct ConfigResponse {
    pub admin: String,
    pub pool_code_id: u64,
}

pub fn query_pool(deps: Deps,pool_id:u64) -> StdResult<PoolResponse> {
    let pool = POOLS.may_load(deps.storage, pool_id).unwrap();
    match pool {
        Some(pool) => {
            let res = PoolResponse {
                pool_id: pool_id,
                pool_addr: pool,
            };
            Ok(res)
        },
        None => {
            let res = PoolResponse {
                pool_id: pool_id,
                pool_addr: "".to_string(),
            };
            Ok(res)
        }
    }
}

#[cw_serde]
pub struct PoolResponse {
    pub pool_id: u64,
    pub pool_addr: String,
}