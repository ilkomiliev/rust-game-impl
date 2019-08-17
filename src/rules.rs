pub trait GameRule {

    fn apply_rule(&self, idx: usize) -> usize;

    fn is_interactive(&self) -> bool;
}

/// Simple move - if the moves is negative it is a backwards move
pub struct MoveRule {
    moves: i32, // negative means backwards
    is_interactive: bool,
}

impl MoveRule {

    pub fn new(moves: i32) -> MoveRule {
        MoveRule {
            moves: moves,
            is_interactive: false
        }
    }
}

impl GameRule for MoveRule {

    fn apply_rule(&self, idx: usize) -> usize {
        let new_idx = (idx as i32 + self.moves) as usize;
        println!("Hit MoveRule moving from {} to {}", idx, new_idx);
        new_idx
    }

    fn is_interactive(&self) -> bool {
        self.is_interactive
    }
}

/// Special case of the MoveRule - always returns to the start field
pub struct MoveToStartRule {
    is_interactive: bool,
}

impl MoveToStartRule {

    pub fn new() -> MoveToStartRule {
        MoveToStartRule {
            is_interactive: false
        }
    }

}

impl GameRule for MoveToStartRule {

    fn apply_rule(&self, idx: usize) -> usize {
        println!("Hit MoveToStartRule from {}", idx);
        0
    }

    fn is_interactive(&self) -> bool {
        self.is_interactive
    }
}

pub struct SwapWithOtherPlayerRule {
    is_interactive: bool,
    my_idx: usize
}

impl SwapWithOtherPlayerRule {

    pub fn new(idx: usize) -> SwapWithOtherPlayerRule {
        SwapWithOtherPlayerRule {
            my_idx: idx,
            is_interactive: true
        }
    } 
}

impl GameRule for SwapWithOtherPlayerRule {

    fn apply_rule(&self, idx_other_player: usize) -> usize {
        idx_other_player
    }

    fn is_interactive(&self) -> bool {
        self.is_interactive
    }

}