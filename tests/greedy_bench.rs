#![feature(test)]
//!
//! This script launch differents benchmarks to test
//! the algorithms of the differents strategies.
extern crate blobwar;
extern crate test;

use blobwar::configuration::Configuration;
use blobwar::strategy::Greedy;

/// This function launch a battle
/// between greedy and greedy on a simple map
fn launch_battle_greedy_greedy(){
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle_silently(Greedy(), Greedy());
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn greedy_versus_greedy(b: &mut Bencher) {
        b.iter(|| launch_battle_greedy_greedy());
    }
}
