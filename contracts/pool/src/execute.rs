use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, BankMsg};

use crate::{ContractError, state::CONFIG};


pub fn execute_withdraw_funds(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    recipient: String,
) -> Result<Response, ContractError> {
    
    let cfg = CONFIG.load(_deps.storage)?;

    if _info.sender != cfg.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    let valid_addr = _deps.api.addr_validate(recipient.as_str())?;


    let funds = _deps.querier.query_all_balances(_env.contract.address)?;
    if funds.is_empty() {

        return Err(ContractError::EmptyBalance {});
    }
    let balance =_deps.querier.query_all_balances(_env.contract.address)?;
    
    // send funds to recipient
    let bank_message = BankMsg::Send {
        to_address: valid_addr.to_string(),
        amount: funds,
    };
    
    Ok(Response::new().add_message(bank_message).add_attributes(vec![
        ("action", "withdraw_funds"),
        ("recipient", &valid_addr.to_string()),

        // ("amount", &balance[0].into()),
        // we want to know how much was withdrawn   
    ]))
    // send funds to recipient
    
    
}
