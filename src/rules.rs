/// This is the main interface for handling different rules in our game.
pub trait GameRule {

    /// 
    /// Implementation of the rule.
    /// Takes the current position in the game and returns the 
    /// calculated new position.
    /// 
    fn apply_rule(&self, idx: usize) -> usize;

    ///
    /// This methods marks the rule as interactive (needs interaction from the player) 
    /// or non-interactive. The latter can be handled automatically, the former must stop
    /// the current move and continue after the interaction has been completed
    /// 
    fn is_interactive(&self) -> bool;
}

/// Simple move - if the moves parameter is negative it is a backwards move.
pub struct MoveRule {
    moves: i32, // negative means backwards
    is_interactive: bool,
}

impl MoveRule {

    /// 
    /// Creates a MoveRule initializing it with the number of moves the player
    /// must perform.
    /// 
    /// # Example
    /// 
    /// ```
    /// let rule = MoveRule::new(5);
    /// assert_eq!(rule.apply_rule(3), 8);
    /// ```
    /// 
    /// Or backwards:
    /// 
    /// ```
    /// let rule = MoveRule::new(-5);
    /// assert_eq!(rule.apply_rule(8), 3);
    /// ```
    /// 
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
}

impl SwapWithOtherPlayerRule {

    pub fn new() -> SwapWithOtherPlayerRule {
        SwapWithOtherPlayerRule {
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