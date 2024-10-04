#[cfg(not(feature = "library"))]
use crate::data::GameResponse; // Import the GameResponse struct used for returning game details in queries
use crate::msg::QueryKey; // Import the QueryKey struct for validating the addresses (host and opponent)
use crate::state::{Status, GAMES}; // Import game state and status types for filtering games
use cosmwasm_std::{Deps, Order, StdResult}; // Import CosmWasm dependencies and result types

/*
This function queries a specific game based on the host address, opponent address, and game ID.

    1. The addresses of both the host and opponent are validated using `addr_validate`.
    2. It looks up the game from the GAMES map, indexed by (host address, opponent address, game ID).
    3. If a game is found, it constructs a `GameResponse` with the game details.
    4. If no game is found, an empty response is returned.
*/

pub fn query_game(deps: Deps, key: QueryKey, game_id: u64) -> StdResult<Vec<GameResponse>> {
    let res: Vec<GameResponse>; // Response vector to store the queried game

    // Validate the host and opponent addresses using the provided `key`
    let host_address = deps.api.addr_validate(&key.host)?;
    let opponent_address = deps.api.addr_validate(&key.opponent)?;

    // Attempt to load the game from the GAMES map using the validated addresses and game ID
    let game_option = GAMES
        .may_load(deps.storage, (&host_address, &opponent_address, game_id))
        .unwrap(); // Use `may_load` to retrieve the game option

    // If the game exists, construct a `GameResponse`; otherwise, return an empty vector
    match game_option {
        Some(_game) => {
            res = vec![GameResponse {
                game: _game, // The actual game state
                host: host_address, // The validated host address
                opponent: opponent_address, // The validated opponent address
            }]
        }
        None => res = vec![], // If no game found, return an empty response
    }

    Ok(res) // Return the result (game or empty response)
}

/*
This function queries all games, optionally filtering them by their status.

    1. Retrieves all games from the GAMES map using the `range` function.
    2. Constructs a `GameResponse` for each game with its details.
    3. If a status filter is provided, filters the games by their status.
    4. Returns the resulting list of games.
*/

pub fn query_games(deps: Deps, status: Option<Status>) -> StdResult<Vec<GameResponse>> {
    // Load all games from the GAMES map in ascending order of keys (addresses and game IDs)
    let mut res: Vec<GameResponse> = GAMES
        .range(deps.storage, None, None, Order::Ascending)
        .map(|f| {
            let (addresses, game) = f.unwrap(); // Unwrap the result to get the game and its associated addresses

            // Construct a `GameResponse` for each game
            GameResponse {
                game: game,
                host: addresses.0, // Host address
                opponent: addresses.1, // Opponent address
            }
        })
        .collect();

    // If a status filter is provided, filter the games by their status
    match status {
        Some(status) => {
            res = res
                .into_iter() // Iterate over the games
                .filter(|res| res.game.status == status) // Only keep games with the matching status
                .collect()
        }
        None => {} // If no status filter is provided, return all games
    }

    Ok(res) // Return the filtered or unfiltered list of games
}
