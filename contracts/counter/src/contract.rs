#[cfg(not(feature = "imported"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64};
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg};
use crate::state::COUNT;
use cosmwasm_std::StdError;

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
        ExecuteMsg::Increment {  } => increment(deps)
    }
}

pub fn increment(deps: DepsMut) -> Result<Response, StdError> { 
    let count = COUNT.load(deps.storage)?;
    let new_count = count.checked_add(Uint64::one())?;
    COUNT.save(deps.storage, &new_count)?;
    Ok(Response::new().add_attribute("method", "increment").add_attribute("count", new_count))
}

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> { 
    match msg { 
        QueryMsg::GetCount {  } => to_binary(&get_count(deps)?)
    }
}

pub fn get_count(deps: Deps) -> Result<Uint64, StdError> { 
    let count = COUNT.load(deps.storage)?;
    Ok(count)
}