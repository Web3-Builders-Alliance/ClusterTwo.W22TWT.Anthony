use cosmwasm_schema::write_api;

use poolFactory::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InitMsg,
        execute: ExecuteMsg,
        // query: QueryMsg,
        // cause :
        // no variant or associated item named `response_schemas` found for enum `QueryMsg` in the current scope
        // variant or associated item not found in `QueryMsg`
    }
}
