//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;

/// Anytime alpha beta algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn alpha_beta_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        let chosen_movement = AlphaBeta(depth).compute_next_move(state);
        movement.store(chosen_movement);
    }
}

/// Alpha - Beta algorithm with given maximum number of recursions.
pub struct AlphaBeta(pub u8);

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

// It's like the minmax algorithm (negamax), but we keep a range of value
// so we are able to follow only interesting paths
// TODO : Write it in rust and parallelize it
fn alphabeta(state: &Configuration, depth: u8, alpha: i8, mut beta: i8) -> (Option<Movement>, i8) {
    // Test if we have a movement to perform or if we aren't at the leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_value = 127;
        let mut best_movement: Option<Movement> = None;
        for mov in state.movements() {
            // We keep give at the opponent the contrary of our game
            // So we invert the alpha beta
            let (_, v_read) = alphabeta(&state.play(&mov), depth - 1, -beta, -alpha);
            // And he give us his version of the score, so we need to inverst it
            let value = -v_read;
            if value < best_value {
                best_value = value;
                best_movement = Some(mov);
                // If the value is better for us, we keep it
                if best_value < beta {
                    beta = best_value;
                }
                // In the case where the better value is worse than the minimum
                // allowed, we cut this node and don't explore it
                if value <= alpha {
                    break;
                }
            }
        }
        return (best_movement, best_value);
    } else {
        // The last level return the value of th egame
        return (None, state.value());
    }
}

impl Strategy for AlphaBeta {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        let (best_movement, _value) = alphabeta(state, self.0, -127, 127);
        best_movement
    }
}
