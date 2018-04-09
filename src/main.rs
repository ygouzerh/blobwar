extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, MinMax};

fn main() {
    //let board = Board::load("x").expect("failed loading board");

    for mov in (1..100) {
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle(AlphaBeta(2),AlphaBeta(5) );
    }
}
