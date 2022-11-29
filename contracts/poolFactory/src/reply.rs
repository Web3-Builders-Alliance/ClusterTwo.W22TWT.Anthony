use std::{fmt::Debug, default};

use cosmwasm_std::{DepsMut, Reply, StdResult, Response, StdError, from_binary, Binary, SubMsgExecutionResponse, SubMsgResponse};
use cw_utils::{parse_reply_instantiate_data, MsgExecuteContractResponse, parse_reply_execute_data};

use crate::{state::{POOLS, POOL_COUNT, CONTRIB}, ContractError, helpers::event_contains_attr};

// Handle the msg data and save the contract address
// See: https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs
pub fn handle_instantiate_reply(_deps: DepsMut,msg: Reply) -> StdResult<Response> {

    let res = parse_reply_instantiate_data(msg).unwrap();

    let count = POOL_COUNT.load(_deps.storage)?;

    POOLS.save(_deps.storage, count, &res.contract_address)?;

    Ok(Response::new().add_attribute("action","instantiated by factory").add_attribute("pool_addr", res.contract_address))
}



pub fn handle_transfer_reply(deps: DepsMut, _msg: SubMsgResponse) -> StdResult<Response> {

    Ok(Response::new().add_attribute("action","redirected").add_attribute("contributor",CONTRIB.load(deps.storage)?))
    
    // clean up CONTRIB
    
    // we could send some 721 to sender here

}