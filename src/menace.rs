use std::{collections::{HashMap, LinkedList}, fs::File, path::Path};
use rand::{prelude::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use crate::game::{symbol::Symbol, tictactoe::TicTacToe};
use serde_json;
use std::io::prelude::*;

pub const WIN_REWARD: i32 = 3;
pub const LOSE_REWARD: i32 = -1;
pub const DRAW_REWARD: i32 = 1;

const DEFAULT_BEAD_COUNT: usize = 3;
const MIN_BEAD_COUNT: i32 = 1;

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
struct Bead {
    next_state: String,
    count: usize
}

impl Bead {
    fn new(next_state: String, count: usize) -> Self {
        Self {
            next_state,
            count
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Menace {
    beads: HashMap<String, Vec<Bead>>,
    #[serde(skip_serializing)]
    episode: LinkedList<(String, Bead)>
}

impl Menace {
    pub fn new() -> Self {
        Self {
            beads: HashMap::new(),
            episode: LinkedList::new()
        }
    }

    /// Creates a new MENACE from the contents of a JSON file
    pub fn from_json(filename: &str) -> Result<Self, String> {
        let mut file = match File::open(filename) {
            Ok(file) => file,
            Err(err) => return Err(err.to_string())
        };
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .expect("Error reading file contents");

        match serde_json::from_str::<Self>(file_contents.as_str()) {
            Ok(result) => Ok(result),
            Err(err) => Err(err.to_string())
        }
    }

    /// Saves the current state of MENACE to a JSON file
    pub fn save_to_json(&self) -> Result<(), std::io::Error> {
        let mut file = File::create(&Path::new("menace.json")).unwrap();
        let menace_json = serde_json::to_string_pretty(self).unwrap();

        file.write_all(menace_json.as_bytes())
    }

    /// Picks a weighted random next state state for the game based on MENACE's
    /// bead counts for current state. If no beads exist for the current state,
    /// the game's next possible states are calulated and MENACE's bead counts
    /// for these states are set to the default values.
    /// 
    /// The next state chosen by MENACE is recorded in 'episode' to keep
    /// track of MENACE's moves from the last game played so it can be
    /// trained appropriately
    ///
    /// # Returns
    /// 
    /// 'None' if the game has no possible next state to go to,
    /// 'Some(next_state)' otherwise
    pub fn step(&mut self, game: &TicTacToe, piece: Symbol) -> Option<String> {
        if game.is_winner(Symbol::X) || game.is_winner(Symbol::O) || game.is_draw() {
            return None;
        }

        let state: String = game.flat();
        let mut rng = thread_rng();

        if !self.beads.contains_key(&state) {
            let next_states: Vec<String> = game.next_states(piece);

            self.beads.insert(state.clone(), next_states.iter()
                .map(|next_state| 
                    Bead::new(next_state.clone(), DEFAULT_BEAD_COUNT)
                ).collect()
            );
        }
        let bead: &Bead = self.beads[&state].choose_weighted(&mut rng, 
            |bead| bead.count
        ).unwrap();

        self.episode.push_front((state, bead.clone()));
        Some(bead.next_state.clone())
    }

    /// MENACE "learns" by updating the bead counts for the state transitions
    /// it chose during the most recent game (represented by 'episode')
    pub fn train(&mut self, delta: i32) {
        loop {
            let (state, bead) = match self.episode.pop_front() {
                Some(state_bead_pair) => state_bead_pair,
                None => break
            };
            let beads_for_state: &mut Vec<Bead> = self.beads.get_mut(&state).unwrap();
            let bead_index: usize = beads_for_state.iter().position(
                |el| *el == bead
            ).unwrap();

            let bead: &mut Bead = beads_for_state.get_mut(bead_index).unwrap();
            let mut prev_count = bead.count as i32;

            prev_count = (prev_count + delta).max(MIN_BEAD_COUNT);
            bead.count = prev_count as usize
        }
    }
}