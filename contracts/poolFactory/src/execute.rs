use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg, to_binary, ReplyOn, BankMsg, coins};

use crate::{ContractError, state::{CONFIG, next_id, POOLS, CONTRIB}, msg::InitPoolMsg, contract::{REDIRECT_FUNDS_ID, INSTANTIATE_REPLY_ID}};

// check if pool_id exists and send fund there
pub fn execute_redirect_funds(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    pool_id: u64,
) -> Result<Response, ContractError> {
    if _info.funds.is_empty() {
        return Err(ContractError::NoFunds {});
    }

    let pool = POOLS.may_load(_deps.storage, pool_id).unwrap();
    match pool {
        Some(pool) => {
            // storing sender for reply msg
            CONTRIB.save(_deps.storage, &_info.sender.to_string())?;
            Ok(Response::new().add_submessage(SubMsg {
                // Instantiate Pool
                msg: BankMsg::Send { to_address: pool.to_string(), amount: coins(_info.funds[0].amount.u128(), &_info.funds[0].denom )} .into(),
                gas_limit: None,
                id: REDIRECT_FUNDS_ID,
                reply_on: ReplyOn::Success,
            }))
        },
        None => {
            Err(ContractError::PoolNotFound { pool_id })
        }
         
        
    }

   
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

    Ok(Response::new().add_submessage(SubMsg {
        // Instantiate Pool
        msg: WasmMsg::Instantiate {
            admin: Some(_info.sender.to_string()),
            code_id: cfg.pool_code_id,
            msg: to_binary(&InitPoolMsg {
                admin: cfg.admin.to_string(),
                title: title.clone(),
                // pool_id: id,
                
            })?,
            funds: vec![],
            label: title,
        }
        .into(),
        gas_limit: None,
        id: INSTANTIATE_REPLY_ID,
        reply_on: ReplyOn::Success,
    }))

    
}


