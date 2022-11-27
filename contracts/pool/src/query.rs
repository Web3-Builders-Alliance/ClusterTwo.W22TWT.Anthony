
use cosmwasm_std::{Deps, StdResult};

use crate::{state::CONFIG, msg::ConfigResponse};



// settings for pagination
// const MAX_LIMIT: u32 = 30;
// const DEFAULT_LIMIT: u32 = 10;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let cfg = CONFIG.load(deps.storage)?;
    
    let res = ConfigResponse {
        admin: cfg.admin.to_string(),   
        title: cfg.title,
        
    };
    Ok(res)
}