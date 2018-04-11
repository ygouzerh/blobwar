extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};





fn main() {
    //let board = Board::load("x").expect("failed loading board");
    for j in (1..11){
    println!("compute_next_move_memoization{}",j);
    for i in (1..25){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(AlphaBeta(j), Greedy());
        }
    }

}
