#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary,  Binary, Deps, DepsMut, Env,  MessageInfo, Response, StdError, StdResult, SubMsgResponse, Reply,};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{ execute_redirect_funds, execute_create_pool};
use crate::helpers::unwrap_reply;
use crate::msg::{  ExecuteMsg,  MigrateMsg,  QueryMsg, InitMsg, };
use crate::query::{query_config, query_pool};
use crate::reply::{handle_instantiate_reply, handle_transfer_reply};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-ics20";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const INSTANTIATE_REPLY_ID:u64=0;
pub const REDIRECT_FUNDS_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let valid_admin = deps.api.addr_validate(msg.admin.as_str())?;
    // let valid_pool_addr = deps.api.addr_validate(msg.pool_addr.as_str())?;


    let cfg = Config {
        admin: valid_admin,
        pool_code_id: msg.pool_code_id
    };
    
    CONFIG.save(deps.storage, &cfg)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // shoudlbe in contract_A
        ExecuteMsg::CreatePool {title  } => execute_create_pool(deps, env, info, title),
        ExecuteMsg::RedirectFund { pool_id } => execute_redirect_funds(deps, env, info,pool_id),        
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(mut _deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {

  

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PoolAddress { pool_id } => to_binary(&query_pool(deps, pool_id)?),
        QueryMsg::Config{}=>to_binary(&query_config(deps)?),
        
 }
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    match reply.id {
        INSTANTIATE_REPLY_ID => handle_instantiate_reply(deps,  reply),
        REDIRECT_FUNDS_ID => handle_transfer_reply(deps, unwrap_reply(reply).unwrap()),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id))),
    }
}

