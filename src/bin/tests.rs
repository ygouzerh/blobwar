extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};





fn main() {
    //let board = Board::load("x").expect("failed loading board");


    println!("Greedy vs Greedy");
    for i in (1..500){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(Greedy(), Greedy());
    }

}