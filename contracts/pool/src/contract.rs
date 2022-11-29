#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{ to_binary,  Binary, Deps, DepsMut, Env,  MessageInfo, Response,  StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::execute::{execute_withdraw_funds,};
use crate::msg::{  ExecuteMsg,  MigrateMsg,  QueryMsg, InitMsg, };

use crate::query::query_config;
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw20-ics20";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> Result<Response, ContractError> {
    set_contract_version(_deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    let _cfg = Config {admin:_deps.api.addr_validate(msg.admin.as_str())?, title: msg.title };
    
    CONFIG.save(_deps.storage, &_cfg)?;
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
        ExecuteMsg::WithdrawFunds {recipient}=> execute_withdraw_funds(deps, env, info, recipient),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{}=>to_binary(&query_config(deps)?),
        
 }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(mut _deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::new())
}



