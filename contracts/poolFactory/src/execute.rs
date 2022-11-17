use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg, to_binary, ReplyOn, BankMsg, coins};

use crate::{ContractError, state::{CONFIG, next_id, POOLS}, msg::InitPoolMsg};

const INSTANTIATE_REPLY_ID:u64=0;

pub fn execute_redirect_funds(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    pool_id: u64,
) -> Result<Response, ContractError> {
    if _info.funds.is_empty() {
        return Err(ContractError::NoFunds {});
    }
    // check if pool_id exists and send fund there

    let pool = POOLS.load(_deps.storage, pool_id)?;

    Ok(Response::new().add_submessage(SubMsg {
        // Instantiate Pool
        msg: BankMsg::Send { to_address: pool.to_string(), amount: coins(123, "ujunox") } .into(),
        gas_limit: None,
        id: INSTANTIATE_REPLY_ID,
        reply_on: ReplyOn::Success,
    }))
}


pub fn execute_create_pool(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    title:String
) -> Result<Response, ContractError> {
    // check if sender is admin
let cfg =CONFIG.load(_deps.storage)?;

let id = next_id(_deps.storage)?;
// to be done in the reply
    // POOLS.save(_deps.storage, id, )?;

    Ok(Response::new().add_submessage(SubMsg {
        // Instantiate Pool
        msg: WasmMsg::Instantiate {
            admin: Some(_info.sender.to_string()),
            code_id: cfg.pool_code_id,
            msg: to_binary(&InitPoolMsg {
                admin: cfg.admin.to_string(),
                title: title,
                pool_id: id,
                
            })?,
            funds: vec![],
            label: "Pool Funds".to_string(),
        }
        .into(),
        gas_limit: None,
        id: INSTANTIATE_REPLY_ID,
        reply_on: ReplyOn::Success,
    }))

    
}


