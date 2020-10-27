use crate::game::tictactoe::TicTacToe;
use crate::adjust::Adjust;

use serde::{Deserialize, Serialize};

use rand::distributions::WeightedIndex;
use rand::prelude::*;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::File;

/// Representation of a set of same-state beads
#[derive(Debug, Serialize, Deserialize)]
struct Bead {
    /// Action that drawing from this set of beads leads to
    action: (usize, usize),
    /// Number of beads in this set
    count: u32
}

impl Bead {
    /// The initial count of beads in each set
    const INITIAL_COUNT: u32 = 2;

    /// Creates a new Bead instance to represent 
    /// specific action
    /// 
    /// # Arguments
    /// 
    /// * `action` - The cell that the AI would play to 
    /// if any beads from this set are chosen
    /// 
    /// # Returns
    /// 
    /// New Bead instance with the initial count
    fn new(action: (usize, usize)) -> Self {
        Self {
            action,
            count: Self::INITIAL_COUNT
        }
    }
}

impl Display for Bead {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{ action: ({}, {}), count: {} }}", self.action.0, self.action.1, self.count)
    }
}

/// Represenation of the MENACE AI
#[derive(Debug)]
pub struct Menace {
    /// Each matchbox is represented by a keyed entry 
    /// in the HashMap, the value stored at each key 
    /// is the sum of all Bead sets stored in the 
    /// given matchbox
    matchbox: HashMap<String, Vec<Bead>>,
    /// A record of all the moves that have been chosen 
    /// during gameplay, so that the matchbox values 
    /// can be adjusted accordingly
    record: Vec<(String, usize)>,
}

impl Menace {
    /// Creates a new Menace instance
    /// 
    /// # Returns
    /// 
    /// New Menace instance with empty matchboxes 
    /// \and empty record
    pub fn new() -> Self {
        Self {
            matchbox: HashMap::new(),
            record: Vec::new(),
        }
    }

    /// Loads a new Menace instance by deserializing from 
    /// a JSON file
    /// 
    /// # Arguments
    /// 
    /// * `filename` - JSON file path
    /// 
    /// # Returns
    /// 
    /// A new Menace instace wrapped in an Ok() Result if 
    /// the file was found and properly deserialized,
    /// error message wrapped in Err() Result otherwise
    pub fn load(filename: &'static str) -> Result<Self, String> {
        let file_contents: String = match fs::read_to_string(filename) {
            Ok(file_contents) => file_contents,
            Err(_) => return Err(format!("Unable to read file: {}", filename)),
        };

        let from_file: serde_json::Result<HashMap<String, Vec<Bead>>> =
            serde_json::from_str(file_contents.as_str());

        let loaded = match from_file {
            Ok(from_file) => from_file,
            Err(_) => return Err(format!("Failed to load JSON file: {}", filename)),
        };

        Ok(Self {
            matchbox: loaded,
            record: Vec::new(),
        })
    }

    /// Serializes and saves the current state of 
    /// Menace to a JSON file
    /// 
    /// # Arguments
    /// 
    /// * `filename` - File path of the JSON 
    /// file to save to
    /// 
    /// # Returns
    /// 
    /// Ok() result if the file was successfully written to,
    /// error message wrapped in Err() Result otherwise
    pub fn save(&self, filename: &'static str) -> Result<(), String> {
        let file: File = match File::create(filename) {
            Ok(file) => file,
            Err(_) => return Err(format!("Failed to create file: {}", filename)),
        };

        match serde_json::to_writer_pretty(&file, &self.matchbox) {
            Ok(_) => Ok(()),
            Err(_) => return Err(format!("Failed to write to file: {}", filename)),
        }
    }

    /// Chooses MENACE's next move, based on a weighted
    /// random choice from the number of beads for each
    /// state the current matchbox can branch to
    /// 
    /// # Arguments
    /// 
    /// * `game` - Instance of TicTacToe game logic
    /// 
    /// # Returns
    /// 
    /// The cell chosen for MENACE to play to
    pub fn choose(&mut self, game: &TicTacToe) -> (usize, usize) {
        let game_state: String = game.flatten();

        if !self.matchbox.contains_key(&game_state) {
            self.populate(game_state.clone(), game.possible_moves());
        }

        let moves: &Vec<Bead> = self.matchbox.get(&game_state).unwrap();
        let weighted_dist = WeightedIndex::new(moves.iter().map(|el| el.count)).unwrap();

        let index: usize = {
            let mut rng = thread_rng();
            weighted_dist.sample(&mut rng)
        };
        let bead: &Bead = moves.get(index).unwrap();

        self.record.push((game_state, index));

        if !game.legal_move(bead.action) {
            panic!("MENACE attempted to make an illegal move. Aborting");
        }
        bead.action
    }

    /// Creates a new matchbox state and populates it with
    /// sets of Beads for each possible move that the new
    /// matchbox can generate
    /// 
    /// # Arguments
    /// 
    /// * `game_state` - The current board state as 
    /// a flattened string
    /// * `moves` - A list of all possible moves given
    /// the current board state
    fn populate(&mut self, game_state: String, moves: Vec<(usize, usize)>) {
        let mut beads: Vec<Bead> = Vec::new();

        for action in moves.iter() {
            beads.push(Bead::new(*action))
        }
        self.matchbox.insert(game_state, beads);
    }

    /// Adjusts all matchboxes from the most recently 
    /// finished game
    /// 
    /// # Arguments
    /// 
    /// * `delta` - Adjustment factor for win, loss, or draw
    pub fn adjust(&mut self, delta: Adjust) {
        for (key, index) in self.record.iter() {
            let bead: &mut Bead = {
                let moves: &mut Vec<Bead> = self.matchbox.get_mut(key).unwrap();
                moves.get_mut(*index).unwrap()
            };
            let new_bead_count = (bead.count as i64) + delta as i64;

            bead.count = if new_bead_count >= 0 {
                new_bead_count as u32
            } else {
                0
            };
        }
        self.record.clear();
    }
}

impl Display for Menace {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for key in self.matchbox.keys() {
            let beads: &Vec<Bead> = self.matchbox.get(key).unwrap();
            writeln!(f, "\"{}\": [", key)?;

            for bead in beads {
                writeln!(f, "    {},", bead)?;
            }
            writeln!(f, "],")?;
        }
        Ok(())
    }
}