use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;
use cw_storage_plus::Item;

pub const COUNT: Item<Uint64> = Item::new("count");