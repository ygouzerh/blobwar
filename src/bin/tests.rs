extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{AlphaBeta, Greedy, Human, IterativeDeepening, IterativeStrategy, MinMax};





fn main() {
	//let board = Board::load("x").expect("failed loading board");

    if false{
    println!("Greedy vs Greedy");
    for i in (0..500){
        let board = Default::default();
        let mut game = Configuration::new(&board);
        game.battle_silently(Greedy(), Greedy());
        }
    }

	//Greedy vs MinMax
	if false{
		for j in (1..4){
			println!("Greedy vs MinMax{}",j);
			for i in (0..100){
				let board = Default::default();
				let mut game = Configuration::new(&board);
				game.battle_silently(Greedy(), MinMax(j));
				}
			}
	}

	//MinMax vs MinMax
	if false{
		for h in (1..5){
			for j in (0..5){
				println!("MinMax{} vs MinMax{}",j,h);
				for i in (1..25){
					let board = Default::default();
					let mut game = Configuration::new(&board);
					game.battle_silently(MinMax(h), MinMax(j));
					}
			     }
	       }
    }
	//Alphabeta vs Greedy
	if true{
		for j in (1..11){
		println!("AlpB{} vs Greedy",j);
		for i in (0..100){
			let board = Default::default();
			let mut game = Configuration::new(&board);
			game.battle_silently(AlphaBeta(j), Greedy());
			}
		}
	}
	 //Alphabeta vs MinMax1
	if false{
		for j in (1..8){
		println!("Alphabeta{} vs MinMax1",j);
		for i in (0..25){
			let board = Default::default();
			let mut game = Configuration::new(&board);
			game.battle_silently(AlphaBeta(j), MinMax(1));
			}
		}
	}

	//Alphabeta vs MinMax2
	if false{
		for j in (1..8){
		println!("Alphabeta{} vs MinMax2",j);
		for i in (0..25){
			let board = Default::default();
			let mut game = Configuration::new(&board);
			game.battle_silently(AlphaBeta(j), MinMax(2));
			}
		}
	}

	//Alphabeta vs MinMax3
	if false{
		for j in (1..8){
		println!("Alphabeta{} vs MinMax3",j);
		for i in (0..25){
			let board = Default::default();
			let mut game = Configuration::new(&board);
			game.battle_silently(AlphaBeta(j), MinMax(3));
			}
		}
	}

	//Alphabeta vs MinMax4
	if false{
		for j in (1..8){
		println!("Alphabeta{} vs MinMax4",j);
		for i in (0..25){
			let board = Default::default();
			let mut game = Configuration::new(&board);
			game.battle_silently(AlphaBeta(j), MinMax(4));
			}
		}
	}


}
