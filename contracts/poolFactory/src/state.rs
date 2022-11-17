use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, StdResult, };
use cw_storage_plus::{Item, Map, };


pub const CONFIG: Item<Config> = Item::new("config");
pub const POOL_COUNT: Item<u64> = Item::new("pool_count");

pub const POOLS: Map<u64, Addr> = Map::new("pools");

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub pool_code_id: u64,
}

pub fn next_id(store: &mut dyn Storage) -> StdResult<u64> {
    let id: u64 = POOL_COUNT.may_load(store)?.unwrap_or_default() + 1;
    POOL_COUNT.save(store, &id)?;
    Ok(id)
}


