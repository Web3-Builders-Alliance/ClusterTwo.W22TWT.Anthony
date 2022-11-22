

use cosmwasm_std::{Reply, StdResult, SubMsgResponse,Event,StdError};


pub fn unwrap_reply(reply: Reply) -> StdResult<SubMsgResponse> {
    reply.result.into_result().map_err(StdError::generic_err)
}

pub fn event_contains_attr(event: &Event, key: &str) -> bool {
    event
        .attributes
        .iter()
        .any(|attr| attr.key == key)
}