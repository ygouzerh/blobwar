extern crate blobwar;
extern crate rayon;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, Human, MinMax};

fn main() {
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    // TODO : This number must be used
    game.battle(MinMax(3), Greedy());
}
