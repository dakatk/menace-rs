mod menace;
mod game;

use std::io::{Write, stdin, stdout};
use game::{symbol::Symbol, tictactoe::TicTacToe};
use menace::Menace;

fn main() -> Result<(), std::io::Error> {
    let mut game = TicTacToe::new();
    let mut menace = match Menace::from_json("menace.json") {
        Ok(menace) => menace,
        Err(_) => Menace::new()
    };

    println!("\n{}\n", game);
    loop {
        if menace_turn(&mut game, &mut menace) {
            game.reset();
            continue;
        }
        let (done, quit) = player_turn(&mut game, &mut menace);
        if done {
            game.reset();
        }
        else if quit {
            break;
        }
    }
    menace.save_to_json()
}

fn menace_turn(game: &mut TicTacToe, menace: &mut Menace) -> bool {
    println!("AI's turn");
    game.from_state(&menace.step(&game, Symbol::O).unwrap());
    println!("\n{}\n", game);

    if game.is_winner(Symbol::O) {
        println!("O wins!\n");
        menace.train(menace::WIN_REWARD);
        return true
    }
    else if game.is_draw() {
        println!("Draw!\n");
        menace.train(menace::DRAW_REWARD);
        return true
    }
    false
}

fn player_turn(game: &mut TicTacToe, menace: &mut Menace) -> (bool, bool) {
    print!("Player's turn: ");
        stdout().flush().unwrap();

        let mut line = String::with_capacity(5);
        stdin().read_line(&mut line).unwrap();

        if line.to_lowercase().trim() == "quit" {
            return (false, true);
        }

        let player_action: u8 = {
            let values: Vec<&str> = line.trim().split(',').collect();
            let row: u8 = values[0].to_string().parse().unwrap();
            let col: u8 = values[1].to_string().parse().unwrap();
            row * 3 + col
        };

        game.place_piece(Symbol::X, player_action).unwrap();
        println!("\n{}\n", game);

        if game.is_winner(Symbol::X) {
            println!("X wins!\n");
            menace.train(menace::LOSE_REWARD);
            return (true, false);
        }
        (false, false)
}
