use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint64};
use cw_storage_plus::Item;

pub const COUNT: Item<Uint64> = Item::new("count");
pub const FUND: Item<Coin> = Item::new("fund");