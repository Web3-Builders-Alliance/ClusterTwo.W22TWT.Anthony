use cosmwasm_std::{DepsMut, Reply, StdResult, Response, StdError};
use cw_utils::{parse_reply_instantiate_data};



pub fn handle_instantiate_reply(_deps: DepsMut, msg: Reply) -> StdResult<Response> {
    // Handle the msg data and save the contract address
    // See: https://github.com/CosmWasm/cw-plus/blob/main/packages/utils/src/parse_reply.rs
    let res = parse_reply_instantiate_data(msg);
    println!("res: {:?}", res);

    // Save res.contract_address
    // Ok(Response::new())

   

    // let data = msg.result.unwrap().data.unwrap();
    // let res: MsgInstantiateContractResponse =
    //     Message::parse_from_bytes(data.as_slice()).map_err(|_| {
    //         StdError::parse_err("MsgInstantiateContractResponse", "failed to parse data")
    //     })?;

    // config.pair_info.liquidity_token =
    //     addr_validate_to_lower(deps.api, res.get_contract_address())?;

    // CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("whitelist addr", "config.pair_info.liquidity_token"))
}

pub fn handle_transfer_reply(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let data = msg.result.into_result().map_err(StdError::generic_err);
    println!("res: {:?}", data);
    // Search for the transfer event
    // // If there are multiple transfers, you will need to find the right event to handle
    // let transfer_event = msg
    //     .events
    //     .iter()
    //     .find(|e| {
    //         e.attributes
    //             .iter()
    //             .any(|attr| attr.key == "action" && attr.value == "transfer")
    //     })
    //     .ok_or_else(|| StdError::generic_err(format!("unable to find transfer action"))?;

    // Do whatever you want with the attributes in the transfer event
    // Reference to the full event: https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L239-L244
    Ok(Response::new())
}