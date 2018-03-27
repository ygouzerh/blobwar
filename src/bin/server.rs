extern crate blobwar;
use blobwar::configuration::Configuration;
use blobwar::board::Board;
use blobwar::strategy::{IterativeDeepening, IterativeStrategy, NetworkPlayer};

use std::net::TcpListener;
use std::env::args;

fn main() {
    let map_name = args().nth(1).unwrap_or("standard".to_owned());
    // we need two players
    let listener = TcpListener::bind("0.0.0.0:12345").expect("bind failed");
    let mut players = listener
        .incoming()
        .map(|c| NetworkPlayer::new(c.expect("accept failed")));

    // create board and start game
    let board = Board::load(map_name).expect("failed loading map");
    let mut game = Configuration::new(&board);
    game.battle(
        IterativeDeepening::new(IterativeStrategy::AlphaBeta),
        players.next().unwrap(),
    );
}
