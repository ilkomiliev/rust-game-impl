extern crate rand;

use rand::Rng;

use rust_game_impl::*;
use rust_game_impl::rules::*;

fn create_game() -> GameBoard {
    let mut game_board = GameBoard::new(15);
    let rule: Box<dyn GameRule> = Box::new(MoveRule::new(3));
    game_board.set_cell_at(3, GameCell::new(rule));
    let rule: Box<dyn GameRule> = Box::new(MoveRule::new(-5));
    game_board.set_cell_at(7, GameCell::new(rule));
    // the move -5 rule should bring us here
    let rule: Box<dyn GameRule> = Box::new(MoveRule::new(2));
    game_board.set_cell_at(2, GameCell::new(rule));
    // oops, almost done but back to start :-(
    let rule: Box<dyn GameRule> = Box::new(MoveToStartRule::new());
    game_board.set_cell_at(13, GameCell::new(rule));
    game_board
}

fn main() {
    let game_board = create_game();
    let mut player1 = Player::new("John");
    let mut player2 = Player::new("Peter");
    let mut player3 = Player::new("George");
    let mut won = false;
    let mut rng = rand::thread_rng();
    let mut counter = 0;
    while !won {
        let dice_roll = rng.gen_range(1, 7);
        let player = match counter % 3 {
            0 => &mut player1,
            1 => &mut player2,
            2 => &mut player3,
            _ => panic!("Help")
        };
        println!("=============================");
        println!("Player {}, from {} rolling {}", player.get_name(), player.get_idx(), dice_roll);
        won = play(&game_board, player, dice_roll);
        println!("New position is at {}", player.get_idx());
        if won {
            println!("Player {} is the winner!!! Took total {} moves.", player.get_name(), player.get_moves());
        }
        println!("=============================");
        counter = counter + 1;
    }
}