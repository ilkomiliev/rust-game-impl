pub mod rules;

use rules::GameRule;

pub struct GameBoard {
    length: usize,
    cells: Vec<GameCell>,
}

pub struct Player {
    name: String,
    idx: usize,
    moves: usize,
}

impl Player {

    pub fn new(name: &str) -> Player {
        Player {
            name: name.to_string(),
            idx: 0,
            moves: 0
        }
    }

    pub fn new_with_idx(name: &str, idx: usize) -> Player {
        let mut player = Player::new(name);
        player.idx = idx;
        player
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_moves(&self) -> usize {
        self.moves
    }

    pub fn increment_moves(&mut self) {
        self.moves = self.moves + 1;
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

impl GameBoard {

    pub fn new(length: usize) -> GameBoard {
        let cells = (0..length).map(|_i| {
            GameCell {game_rule: None}
        }).collect();
        GameBoard {
            length: length,
            cells: cells
        }
    } 

    fn get_cell_at(&self, idx: usize) -> Option<&GameCell> {
        self.cells.get(idx)
    }

    fn get_cell_rule_at(&self, idx: usize) -> Option<&Box<dyn GameRule>> {
        match self.get_cell_at(idx) {
            Some(cell) => cell.game_rule.as_ref(),
            None => None
        }
    }

    pub fn get_length(&self) -> usize {
        self.length
    }

    pub fn set_cell_at(&mut self, idx: usize, cell: GameCell) {
        self.cells.remove(idx);
        self.cells.insert(idx, cell);
    }
}

pub struct GameCell {
    game_rule: Option<Box<dyn GameRule>>
}

impl GameCell {

    pub fn new(rule: Box<dyn GameRule>) -> GameCell {
        GameCell {
            game_rule: Some(rule)
        }
    }
}

/// the bool in the return signals, if this was the last turn or an interactive rule was hit
/// and the processing must continue after the interactive rule has been handled
fn game_move(player: &mut Player, moves: i32, game_board: &GameBoard, num_of_tries: i32) -> bool {
    let from = player.idx;
    player.increment_moves();
    let new_idx = from as i32 + moves;
    if new_idx < 0 {
        player.idx = 0;
        return true;
    }
    if new_idx as usize >= game_board.get_length() {
        return true;
    }
    let new_idx = new_idx as usize; // this is safe here
    let game_rule = game_board.get_cell_rule_at(new_idx);
    player.idx = new_idx;
    match (game_rule, num_of_tries) {
        (Some(_rule), 2) => {
                println!("Max attempts reached - exiting!");
                true
            },
        (Some(rule), _) => {
            if rule.is_interactive() {
                false
            }
            else {
                player.idx = rule.apply_rule(new_idx);
                game_move(player, 0, game_board, num_of_tries+1)
            }
        },
        (None, _) => true
    }   
    
}

pub fn play(game_board: &GameBoard, player: &mut Player, dice_roll: i32) -> bool {
    let last_move = game_move(player, dice_roll, game_board, 0);
    last_move && player.idx == game_board.length - 1
}

#[cfg(test)]
mod tests {

    use super::*;
    use rules::MoveRule;

    #[test]
    fn test_move_simple_cells_only_edge_cases() {
        let game = GameBoard::new(10);
        let mut player = Player::new("John");
        game_move(&mut player, -6, &game, 0);
        assert_eq!(player.idx, 0);
        let mut player = Player::new_with_idx("John", 8);
        game_move(&mut player, 2, &game, 0);
        assert_eq!(player.idx, 8);
        let mut player = Player::new_with_idx("John", 9);
        game_move(&mut player, 1, &game, 0);
        assert_eq!(player.idx, 9);
    }

    #[test]
    fn test_move_rule_target_is_simple_cell() {
        let mut game = GameBoard::new(10);
        let rule_cell = GameCell {
            game_rule: Some(Box::new(MoveRule::new(3)))
        };
        game.set_cell_at(2, rule_cell);
        let mut player = Player::new("John");
        game_move(&mut player, 2, &game, 0);
        assert_eq!(player.idx, 5);
    }

    #[test]
    fn test_move_rule_target_is_rule_cell() {
        let mut game = GameBoard::new(10);
        let rule_cell = GameCell {
            game_rule: Some(Box::new(MoveRule::new(3)))
        };
        game.set_cell_at(2, rule_cell);
        let rule_cell = GameCell {
            game_rule: Some(Box::new(MoveRule::new(-2)))
        };
        game.set_cell_at(5, rule_cell);
        let mut player = Player::new("John");
        game_move(&mut player, 2, &game, 0);
        assert_eq!(player.idx, 3);
    }

    #[test]
    fn test_move_dead_lock_with_rules_exits_after_three_attempts() {
        let mut game = GameBoard::new(10);
        let rule_cell = GameCell {
            game_rule: Some(Box::new(MoveRule::new(3)))
        };
        game.set_cell_at(2, rule_cell);
        let rule_cell = GameCell {
            game_rule: Some(Box::new(MoveRule::new(-3)))
        };
        game.set_cell_at(5, rule_cell);
        let mut player = Player::new("John");
        game_move(&mut player, 2, &game, 0);
        assert_eq!(player.idx, 2);
    }
}