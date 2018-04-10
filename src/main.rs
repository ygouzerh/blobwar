extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
<<<<<<< HEAD
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(Greedy(), Greedy());
=======

    for mov in (1..100) {
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle(AlphaBeta(2),AlphaBeta(5) );
    }
>>>>>>> 2a0846954b5b2f1cd3b8df6112e4a44be7d059b7
}
