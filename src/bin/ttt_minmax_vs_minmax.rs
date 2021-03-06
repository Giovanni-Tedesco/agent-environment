use games::abstractions::play;
use games::abstractions::Environment;

use games::agents::minmax_agent;

use games::tictactoe::AgentId;
use games::tictactoe::Board;

use games::tree_search::depth_first;
use minmax_agent::MinmaxAgent;

/// Plays tic tac toe with a minmax player vs minmax player
fn main() {
    let mut board = Board::initial_state();

    let mut player_x = MinmaxAgent::new(AgentId::X, &depth_first, 10);
    let mut player_o = MinmaxAgent::new(AgentId::O, &depth_first, 10);

    let log = play(&mut board, &mut player_x, &mut player_o);

    for (agent, mv) in log {
        println!("Player: {}, moved {}", agent, mv);
    }

    println!("{}", board.to_string());

    let winner = board.winner();

    match winner {
        Some(x) => println!("Player {} wins.", x),
        None => println!("The game ended in a draw"),
    }
}
