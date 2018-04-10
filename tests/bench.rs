#![feature(test)]
//!
//! This script launch differents benchmarks to test
//! the algorithms of the differents strategies.
extern crate blobwar;
extern crate test;

use blobwar::configuration::Configuration;
use blobwar::strategy::Greedy;
use blobwar::strategy::AlphaBeta;
use blobwar::strategy::MinMax;
use blobwar::strategy::Strategy;

/// This function launch a battle
/// between greedy and greedy on a simple map
fn launch_battle_greedy_greedy(){
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle_silently(Greedy(), Greedy());
}

/// Function that launch AlphaBeta on a great depth with only itself
fn launch_solo_alpha_beta(){
    let board = Default::default();
    let game = Configuration::new(&board);
    AlphaBeta(3).compute_next_move(&game);
}

/// Function that launch MinMax // on a great depth with only itself
fn launch_solo_minmax_parallel(){
    let board = Default::default();
    let game = Configuration::new(&board);
    MinMax(5).compute_next_move(&game);
}

/// Function that launch MinMax sequential on a great depth with only itself
fn launch_solo_minmax_sequential(){
    let board = Default::default();
    let game = Configuration::new(&board);
    MinMax(5).compute_next_move_sequential(&game);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    //
    // #[bench]
    // fn greedy_versus_greedy(b: &mut Bencher) {
    //     b.iter(|| launch_battle_greedy_greedy());
    // }
    //
    // #[bench]
    // fn alphabeta_solo(b: &mut Bencher){
    //     b.iter(|| launch_solo_alpha_beta());
    // }

    #[bench]
    fn minmax_sequential_solo(b: &mut Bencher){
        b.iter(|| launch_solo_minmax_sequential());
    }

    #[bench]
    fn minmax_parallel_solo(b: &mut Bencher){
        b.iter(|| launch_solo_minmax_parallel());
    }

}
