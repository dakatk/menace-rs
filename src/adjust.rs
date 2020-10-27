/// Adjustment factors for the number of beads 
/// to add or remove from each of MENACE's 
/// matchbox upon game end
#[derive(Debug, Copy, Clone)]
#[repr(i64)]
pub enum Adjust {
    WIN = 3,
    LOSE = -1,
    DRAW = 1
}