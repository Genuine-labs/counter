use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct InstantiateMsg {
    pub count: Uint64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Increment{}
}

#[cw_serde]
pub enum QueryMsg {
    GetCount{}
}


