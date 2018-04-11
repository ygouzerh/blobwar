extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};





fn main() {
    //let board = Board::load("x").expect("failed loading board");

    println!("MinMax1 vs MinMax1");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(1), MinMax(1));
    }
    println!("MinMax2 vs MinMax2");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(2), MinMax(2));
    }
    println!("MinMax3 vs MinMax3");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(AlphaBeta(3), MinMax(3));
    }
    println!("MinMax4 vs MinMax4");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(4), MinMax(4));
    }
    println!("MinMax5 vs MinMax5");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(5), MinMax(5));
    }
    println!("MinMax6 vs MinMax6");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(6), MinMax(6));
    }
    println!("MinMax7 vs MinMax7");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(7), MinMax(7));
    }
    println!("MinMax8 vs MinMax8");
    for i in (1..6){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(MinMax(8), MinMax(8));
    }





}
