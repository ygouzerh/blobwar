extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(
        IterativeDeepening::new(IterativeStrategy::AlphaBeta),
        IterativeDeepening::new(IterativeStrategy::AlphaBeta),
    );
    // game.battle(AlphaBeta(8), Greedy());
    // game.battle_silently(AlphaBeta(8), Greedy());
}
