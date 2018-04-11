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
fn launch_battle_greedy_greedy() {
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle_silently(Greedy(), Greedy());
}

/// Function that launch AlphaBeta on a great depth with only itself
fn launch_solo_alpha_beta_simple() {
    let board = Default::default();
    let game = Configuration::new(&board);
    AlphaBeta(8).compute_next_move_simple(&game);
}

/// Function that launch AlphaBeta with memoization
fn launch_solo_alphabeta_memoization() {
    let board = Default::default();
    let game = Configuration::new(&board);
    AlphaBeta(8).compute_next_move_memoization(&game);
}

/// Function that launch AlphaBeta with memoization and sort
fn launch_solo_alphabeta_sorted(depth: u8) {
    let board = Default::default();
    let game = Configuration::new(&board);
    AlphaBeta(depth).compute_next_move_sorted(&game);
}

/// Function that launch AlphaBeta with negascout
fn launch_solo_negascout(depth: u8) {
    let board = Default::default();
    let game = Configuration::new(&board);
    AlphaBeta(depth).compute_next_move_negascout(&game);
}

/// Function that launch MinMax // on a great depth with only itself
fn launch_solo_minmax_parallel() {
    let board = Default::default();
    let game = Configuration::new(&board);
    MinMax(5).compute_next_move(&game);
}

/// Function that launch MinMax sequential on a great depth with only itself
fn launch_solo_minmax_sequential() {
    let board = Default::default();
    let game = Configuration::new(&board);
    MinMax(5).compute_next_move_sequential(&game);
}

fn launch_solo_minmax_parallel_parameter(depth: u8) {
    let board = Default::default();
    let game = Configuration::new(&board);
    MinMax(depth).compute_next_move(&game);
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
    // fn minmax_sequential_solo(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_sequential());
    // }
    //
    // #[bench]
    // fn minmax_parallel_solo(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_parallel());
    // }

    // #[bench]
    // fn minmax_parallel_1(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_parallel_parameter(1));
    // }
    //
    // #[bench]
    // fn minmax_parallel_2(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_parallel_parameter(2));
    // }
    //
    // #[bench]
    // fn minmax_parallel_3(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_parallel_parameter(3));
    // }
    //
    // #[bench]
    // fn minmax_parallel_4(b: &mut Bencher) {
    //     b.iter(|| launch_solo_minmax_parallel_parameter(4));
    // }
    //
    // #[bench]
    // fn alphabeta_memoization_sorted_1(b: &mut Bencher) {
    //     b.iter(|| launch_solo_alphabeta_sorted(1));
    // }
    //
    // #[bench]
    // fn alphabeta_memoization_sorted_2(b: &mut Bencher) {
    //     b.iter(|| launch_solo_alphabeta_sorted(2));
    // }
    //
    // #[bench]
    // fn alphabeta_memoization_sorted_3(b: &mut Bencher) {
    //     b.iter(|| launch_solo_alphabeta_sorted(3));
    // }

    // #[bench]
    // fn alphabeta_memoization_sorted_4(b: &mut Bencher) {
    //     b.iter(|| launch_solo_alphabeta_sorted(4));
    // }

    #[bench]
    fn alphabeta_solo(b: &mut Bencher) {
        b.iter(|| launch_solo_alpha_beta_simple());
    }

    #[bench]
    fn alphabeta_memoization_solo(b: &mut Bencher) {
        b.iter(|| launch_solo_alphabeta_memoization());
    }

    // #[bench]
    // fn negascout(b: &mut Bencher) {
    //     b.iter(|| launch_solo_negascout(10));
    // }

    #[bench]
    fn alphabeta_memoization_sorted_8(b: &mut Bencher) {
        b.iter(|| launch_solo_alphabeta_sorted(10));
    }

}
