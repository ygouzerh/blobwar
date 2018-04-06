//! Implementation of the min max algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

fn minmax(state: &Configuration, depth: i8, is_the_player: bool) -> (Option<Movement>, i8) {
    // We stop at the end of the possible move, or at the terminal leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_movement: Option<Movement> = None;
        // For this game, the player want to minimize it
        if !is_the_player {
            let mut best_value: i8 = -127;
            // For each move, we take the move which minimize the value
            for mov in state.movements() {
                let config = state.play(&mov);
                let (_, v_read) = minmax(&config, depth - 1, is_the_player);
                let value = - v_read;
                if value > best_value {
                    best_value = value;
                    best_movement = Some(mov);
                }
            }
            return (best_movement, best_value);
        // The other player
        } else {
            let mut best_value: i8 = 127;
            // For each move, we take the move which maximize the value
            for mov in state.movements() {
                let config = state.play(&mov);
                let (_, v_read) = minmax(&config, depth - 1, is_the_player);
                let value = - v_read;
                if value < best_value {
                    best_value = value;
                    best_movement = Some(mov);
                }
            }
            return (best_movement, best_value);
        }
    } else {
        // TODO : Dirty solution, work only with depth = 3 and red player I think
        return (None, state.value());
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        println!("MINMAX me : current_player = {:?} , value = {:?}", state.current_player, state.value());
        for mov in state.movements() {
            let config = state.play(&mov);
            let value = config.value();
            println!("MINMAX other : current_player = {:?} , value = {:?}", state.current_player, value);
            for mov2 in config.movements() {
                let config2 = config.play(&mov2);
                let value2 = config2.value();
                println!("MINMAX again : current_player = {:?} , value = {:?}", config2.current_player, value2);
                break;
            }
            break;
        }
        let (best_movement, _) = minmax(state, 2, state.current_player);
        best_movement
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
