use crate::errors::ContractError;
use crate::execution::{try_accept, try_invite, try_play, try_reject}; // Importing execution functions for state-changing operations
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg}; // Importing message types for instantiation, execution, and query operations
use crate::query::{query_game, query_games}; // Importing query functions for read-only operations
use crate::state::GAMES_COUNT; // Importing the state variable for keeping track of the games count
use cosmwasm_std::to_binary;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::{get_contract_version, set_contract_version}; // Importing utilities for managing contract versions

// Constants for contract name and version, used in migration and instantiation
const CONTRACT_NAME: &str = "tic-tac-toe";
const CONTRACT_VERSION: &str = "0.1.0";

/* 
This function is called when the contract is first deployed:
    1. Sets the contract version in the storage using `set_contract_version`.
    2. Initializes the `GAMES_COUNT` to 0 to track the number of games played.
    3. Returns a response with an "action" attribute to confirm successful initialization.
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Set contract version in the storage
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    // Initialize games count to 0
    GAMES_COUNT.save(deps.storage, &0)?;
    
    // Return response with an "action" attribute to confirm successful instantiation
    Ok(Response::new().add_attribute("action", "tic-tac-toe"))
}

/*
This function handles all state-changing operations (execution phase):
    1. Uses pattern matching to route different `ExecuteMsg` messages to the appropriate handler functions.
    2. Supports actions like inviting a player, rejecting/accepting invitations, and making moves during the game.
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // Invite an opponent to a game with specified coordinates
        ExecuteMsg::Invite { coord, opponent } => try_invite(deps, info, coord, opponent),
        
        // Reject an invitation based on the role (host or opponent) and game ID
        ExecuteMsg::Reject {
            as_host,
            opponent,
            game_id,
        } => try_reject(deps, info, as_host, opponent, game_id),
        
        // Accept an invitation from the host with specified coordinates and game ID
        ExecuteMsg::Accept {
            coord,
            host,
            game_id,
        } => try_accept(deps, info, coord, host, game_id),
        
        // Make a move in the game based on the role (host or opponent), coordinates, and game ID
        ExecuteMsg::Play {
            as_host,
            coord,
            opponent,
            game_id,
        } => try_play(deps, info, as_host, coord, opponent, game_id),
    }
}

/*
This function handles read-only (query) operations:
    1. Uses pattern matching to route different `QueryMsg` messages to the corresponding query handlers.
    2. Supports queries for contract version, specific game information, and multiple games based on their status.
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Query the contract version information
        QueryMsg::GetContractVersion {} => to_binary(&get_contract_version(deps.storage)?),
        
        // Query specific game information based on a key and game ID
        QueryMsg::Game { key, game_id } => to_binary(&query_game(deps, key, game_id)?),
        
        // Query multiple games based on their current status
        QueryMsg::Games { status } => to_binary(&query_games(deps, status)?),
    }
}
