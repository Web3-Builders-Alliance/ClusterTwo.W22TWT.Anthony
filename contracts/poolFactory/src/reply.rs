use std::{fmt::Debug, default};

use cosmwasm_std::{DepsMut, Reply, StdResult, Response, StdError, from_binary, Binary, SubMsgExecutionResponse, SubMsgResponse};
use cw_utils::{parse_reply_instantiate_data, MsgExecuteContractResponse, parse_reply_execute_data};

use crate::{state::{POOLS, POOL_COUNT, CONTRIB}, ContractError, helpers::event_contains_attr};



pub fn handle_instantiate_reply(_deps: DepsMut,msg: Reply) -> StdResult<Response> {
    // Handle the msg data and save the contract address
    // See: https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs

    let res = parse_reply_instantiate_data(msg).unwrap();

   


    // TODO: Save the contract address in the state in POOLS
    // get pool_id

    let count = POOL_COUNT.load(_deps.storage)?;

    POOLS.save(_deps.storage, count, &res.contract_address)?;

    // let data = msg.result.unwrap().data.unwrap();
    // let res: MsgInstantiateContractResponse =
    //     Message::parse_from_bytes(data.as_slice()).map_err(|_| {
    //         StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
    //     })?;

    // config.pair_info.liquidity_token =
    //     addr_validate_to_lower(deps.api, res.get_contract_address())?;

    // CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action","instantiated by factory").add_attribute("pool_addr", res.contract_address))
}



pub fn handle_transfer_reply(deps: DepsMut, _msg: SubMsgResponse) -> StdResult<Response> {

    Ok(Response::new().add_attribute("action","redirected").add_attribute("contributor",CONTRIB.load(deps.storage)?))
    

    // we could send some 721 to sender here

}