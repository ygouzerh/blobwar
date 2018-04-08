//! Implementation of the min max algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

// It's more a negamax here than a min max algorithm, due to the inversion done by state.play()
fn minmax(state: &Configuration, depth: i8) -> (Option<Movement>, i8) {
    // Test if we have a movement to perform or if we aren't at the leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_value: i8 = 127;
        let mut best_movement: Option<Movement> = None;
        // For each possible movements we test he possible combinations
        for mov in state.movements() {
            let (_, v_read) = minmax(&state.play(&mov), depth - 1);
            // We invert the value at each turn because the second level will give the value
            // for himslef
            let value = -v_read;
            // Keep if it's a better value
            if value < best_value {
                best_value = value;
                best_movement = Some(mov);
            }
        }
        return (best_movement, best_value);
    } else {
        // The last level return the value of th egame
        return (None, state.value());
    }
}

impl Strategy for MinMax {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        println!(
            "MINMAX me : current_player = {:?} , value = {:?}",
            state.current_player,
            state.value()
        );
        let (best_movement, _) = minmax(state, 2);
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
