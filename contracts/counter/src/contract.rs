use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::COUNT;
#[cfg(not(feature = "imported"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::StdError;
use cosmwasm_std::{
    to_binary, AllBalanceResponse, BankQuery, Binary, Coin, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdResult, Uint64,
};

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    COUNT.save(deps.storage, &msg.count)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::Increment {} => increment(deps),
    }
}

pub fn increment(deps: DepsMut) -> Result<Response, StdError> {
    let count = COUNT.load(deps.storage)?;
    let new_count = count.checked_add(Uint64::one())?;
    COUNT.save(deps.storage, &new_count)?;
    Ok(Response::new()
        .add_attribute("method", "increment")
        .add_attribute("count", new_count))
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&get_count(deps)?),
        QueryMsg::GetTotalFunds {} => to_binary(&get_total_funds(deps, env)?),
    }
}

pub fn get_count(deps: Deps) -> Result<Uint64, StdError> {
    let count = COUNT.load(deps.storage)?;
    Ok(count)
}

pub fn get_total_funds(deps: Deps, env: Env) -> Result<Vec<Coin>, StdError> {
    let all_balances: AllBalanceResponse =
        deps.querier
            .query(&QueryRequest::Bank(BankQuery::AllBalances {
                address: env.contract.address.into_string(),
            }))?;
    Ok(all_balances.amount)
}
