use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{COUNT, FUND};
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
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    COUNT.save(deps.storage, &msg.count)?;
    if info.funds.len() != 0 {
        let coin = info.funds[0].clone();
        FUND.save(deps.storage, &coin)?;
    } 
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
        ExecuteMsg::Donate {  } => donate(deps, env, info),
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

pub fn donate(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, StdError> {
    let fund = FUND.load(deps.storage)?;
    let mut funds = info.funds;
    
    // if the funds is the same as FUND, then add first element to FUND
    if funds.len() != 0 {
        if FUND.exists(deps.storage) {
            let fund = FUND.load(deps.storage)?;
            let new_fund: Vec<&Coin> = funds.iter().filter(
                | &x | x.denom == fund.denom
            ).collect();
            if new_fund.len() != 0 {
                let new_fund = new_fund[0].clone();
                let new_fund = Coin {
                    denom: new_fund.denom,
                    amount: new_fund.amount + fund.amount,
                };
                FUND.save(deps.storage, &new_fund)?;
            }

            let new_fund = FUND.load(deps.storage)?;
            return Ok(Response::new()
                .add_attribute("method", "donate")
                .add_attribute("ne", new_fund.to_string()));
        } else {
            FUND.save(deps.storage, &funds[0])?;
            return Ok(Response::new()
                .add_attribute("method", "donate")
                .add_attribute("ne", funds[0].to_string()));
        }
    } else {
        return Err(StdError::generic_err("Donate with no funds"));
    }
    
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
