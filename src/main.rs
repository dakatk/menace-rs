mod game;
mod menace;
mod adjust;

use crate::game::symbol::Symbol;
use crate::game::tictactoe::TicTacToe;
use crate::menace::Menace;
use crate::adjust::Adjust;

use std::io::{stdin, stdout, Write};

#[doc(hidden)]
fn main() -> Result<(), String> {
    let mut game = TicTacToe::new();
    let mut menace = match Menace::load("menace.json") {
        Ok(menace) => menace,
        Err(message) => {
            println!("{}\n", message);
            Menace::new()
        }
    };

    println!("{}", menace);

    game_loop(&mut game, &mut menace);

    menace.save("menace.json")
}

/// The main game loop. Doesn't end until the player chooses to
/// 
/// # Arguments
/// 
/// * `game` - Instance of TicTacToe game logic
/// * `menace` - Instance of the MENACE AI
fn game_loop(game: &mut TicTacToe, menace: &mut Menace) {
    println!("\n{}", game);

    loop {
        let player_move: (usize, usize) = get_player_move(game);
        game.set(player_move, Symbol::O);

        println!("\n{}", game);

        if check_game_over(game, menace, false) {
            println!("\nPlayer wins!\n");

            if !prompt_continue() {
                break;
            }
        }

        let menace_move: (usize, usize) = menace.choose(game);
        game.set(menace_move, Symbol::X);

        println!("\n{}", game);

        if check_game_over(game, menace, true) {
            println!("\nMENACE wins!\n");

            if !prompt_continue() {
                break;
            }
        }
    }
}

/// Ask for the player's input for their next move
/// 
/// # Arguments
/// 
/// * `game` - Instance of TicTacToe game logic
/// 
/// # Returns 
/// 
/// A tuple in the form (row, col) with the player's
/// chosen cell for their next move
fn get_player_move(game: &TicTacToe) -> (usize, usize) {
    loop {
        let user_input = get_user_input("Player's move (row,col): ");

        match str_to_tuple(user_input) {
            Some(player_move) => {
                if game.legal_move(player_move) {
                    return player_move;
                }
            }
            None => println!("Invalid input"),
        };
    }
}

/// Prompt the user whether to play another game or not
/// 
/// # Returns
/// 
/// True if the player answered "yes" or "y", false otherwise
fn prompt_continue() -> bool {
    let user_input = get_user_input("Play again? (Y/N): ");

    match user_input.to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}

/// Generic user input prompt
/// 
/// # Arguments
/// 
/// * `prompt` - Prompt shown to the user before accepting input
/// 
/// # Returns
/// 
/// The user input, trimmed of trailing and leading whitespace
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().expect("Error: unable to flush stdout");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Error: unable to read user input");
    input.trim().to_string()
}

/// Convert comma separated string of length 2 
/// to a tuple of the same length
/// 
/// # Arguments
/// 
/// * `value` - comma separated string to be converted
/// 
/// # Returns
/// 
/// The Some((row, col)) tuple represented by the input
/// string if the the input string was parsed correctly,
/// None otherwise
fn str_to_tuple(value: String) -> Option<(usize, usize)> {
    let split: Vec<&str> = value.split(",").collect();
    if split.len() != 2 {
        return None;
    }

    let row: usize = match split[0].trim().parse::<usize>() {
        Ok(row) => row,
        Err(_) => return None,
    };

    let col: usize = match split[1].trim().parse::<usize>() {
        Ok(col) => col,
        Err(_) => return None,
    };

    Some((row, col))
}

/// Check if the game has been won by either player, 
/// or if the game is a draw. Adjusts MENACE accordingly
/// 
/// # Arguments
/// 
/// * `game` - Instance of TicTacToe game logic
/// * `menace` - Instance of the MENACE AI
/// * `menace_turn` - Whether it was the AI's turn 
/// or not, signifying the winner
/// 
/// # Returns
/// 
/// True if a winner was found or the game was 
/// determined to be a draw, false otherwise
fn check_game_over(game: &TicTacToe, menace: &mut Menace, menace_turn: bool) -> bool {
    if game.check_win() {
        menace.adjust(if menace_turn { Adjust::WIN } else { Adjust::LOSE });
        true
    } else if game.board_full() {
        menace.adjust(Adjust::DRAW);
        true
    } else {
        false
    }
}
