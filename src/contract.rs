use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse,
    InitResponse, Querier, StdError, StdResult, Storage
};

use crate::msg::{UsersResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        users: vec![],
        owner: deps.api.canonical_address(&env.message.sender)?,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::AddUser { user } => add_user(deps, env, user),
        HandleMsg::RemoveUser { user } => remove_user(deps, env, user),
    }
}

pub fn add_user<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    user: String,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    config(&mut deps.storage).update(|mut state| {
        if api.canonical_address(&env.message.sender)? != state.owner {
            return Err(StdError::unauthorized());
        }
        state.users.push(user);
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

pub fn remove_user<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    user: String,
) -> StdResult<HandleResponse> {
    let api = &deps.api;
    config(&mut deps.storage).update(|mut state| {
        if api.canonical_address(&env.message.sender)? != state.owner {
            return Err(StdError::unauthorized());
        }
        let index = state.users.iter().position(|x| *x == user).unwrap();
        state.users.remove(index);
        Ok(state)
    })?;
    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetUsers {} => to_binary(&query_users(deps)?),
    }
}

fn query_users<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<UsersResponse> {
    let state = config_read(&deps.storage).load()?;
    Ok(UsersResponse {
        users: state.users,
    })
}
