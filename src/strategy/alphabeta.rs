//! Alpha - Beta algorithm.
use std::fmt;

use super::Strategy;
use configuration::{Configuration, Movement};
use shmem::AtomicMove;
use std::collections::HashMap;

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

impl AlphaBeta {
    /// Compute all the move but store at each node the value of the node
    pub fn compute_next_move_memoization(&mut self, state: &Configuration) -> Option<Movement> {
        let mut memoizer: HashMap<String, (Movement, i8)> = HashMap::new();
        let (best_movement, _value) =
            alphabeta_memoization(state, self.0, -127, 127, &mut memoizer);
        best_movement
    }

    /// Compute all the move and memoize, but sort the game by value fo each move
    pub fn compute_next_move_sorted(&mut self, state: &Configuration) -> Option<Movement> {
        let mut memoizer: HashMap<String, (Movement, i8)> = HashMap::new();
        let (best_movement, _value) = alphabeta_sorted(state, self.0, -127, 127, &mut memoizer);
        best_movement
    }

    /// Compute all the moves but use the simple algorithm
    pub fn compute_next_move_simple(&mut self, state: &Configuration) -> Option<Movement> {
        // self.compute_next_move_memoization(state)
        let (best_movement, _value) = alphabeta(state, self.0, -127, 127);
        best_movement
    }

    /// Compute all the moves but use negascout
    pub fn compute_next_move_negascout(&mut self, state: &Configuration) -> Option<Movement> {
        let mut memoizer: HashMap<String, (Movement, i8)> = HashMap::new();
        let (best_movement, _value) = negascout(state, self.0, -127, 127, &mut memoizer);
        best_movement
    }
}

impl fmt::Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Alpha - Beta (max level: {})", self.0)
    }
}

// It's like the minmax algorithm (negamax), but we keep a range of value
// so we are able to follow only interesting paths
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

// It's like the minmax algorithm (negamax), but we keep a range of value
// so we are able to follow only interesting paths
fn alphabeta_memoization(
    state: &Configuration,
    depth: u8,
    alpha: i8,
    mut beta: i8,
    memoizer: &mut HashMap<String, (Movement, i8)>,
) -> (Option<Movement>, i8) {
    // Test if we have a movement to perform or if we aren't at the leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_value = 127;
        let mut best_movement: Option<Movement> = None;
        for mov in state.movements() {
            let value: i8;
            let serializing_state: String = state.serialize();
            if memoizer.contains_key(&serializing_state) {
                let result = memoizer.get(&serializing_state);
                let v_read = match result {
                    Some(y) => y.1,
                    None => 127,
                };
                value = v_read;
            } else {
                // We keep give at the opponent the contrary of our game
                // So we invert the alpha beta
                let (_, v_read) =
                    alphabeta_memoization(&state.play(&mov), depth - 1, -beta, -alpha, memoizer);
                // And he give us his version of the score, so we need to inverst it
                value = -v_read;
                memoizer.insert(state.serialize(), (mov, value));
            }
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

// It's like alphabeta_memoization but it's sort the value of each move
// so we will have more chance to find the best quickly
fn alphabeta_sorted(
    state: &Configuration,
    depth: u8,
    alpha: i8,
    mut beta: i8,
    memoizer: &mut HashMap<String, (Movement, i8)>,
) -> (Option<Movement>, i8) {
    // Test if we have a movement to perform or if we aren't at the leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_value = 127;
        let mut best_movement: Option<Movement> = None;
        // We sort the movements by the value of the game.
        // So we have a best chance to have the best moves before
        // We want the minimum value, so we took the minimum first
        // So we sort by taking the minimum first
        let mut moves_sorted: Vec<_> = state.movements().collect::<Vec<_>>();
        moves_sorted.sort_by_key(|m| -state.play(m).value());
        for mov in moves_sorted.into_iter() {
            let value: i8;
            let serializing_state: String = state.serialize();
            if memoizer.contains_key(&serializing_state) {
                let result = memoizer.get(&serializing_state);
                let v_read = match result {
                    Some(y) => y.1,
                    None => 127,
                };
                value = v_read;
            } else {
                // We keep give at the opponent the contrary of our game
                // So we invert the alpha beta
                let (_, v_read) =
                    alphabeta_sorted(&state.play(&mov), depth - 1, -beta, -alpha, memoizer);
                // And he give us his version of the score, so we need to inverst it
                value = -v_read;
                memoizer.insert(state.serialize(), (mov, value));
            }
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

//Negascout algorithm with memoization and sort
fn negascout(
    state: &Configuration,
    depth: u8,
    alpha: i8,
    mut beta: i8,
    memoizer: &mut HashMap<String, (Movement, i8)>,
) -> (Option<Movement>, i8) {
    // Test if we have a movement to perform or if we aren't at the leaves
    if state.movements().next().is_some() && depth > 0 {
        let mut best_value = 127;
        let mut best_movement: Option<Movement> = None;
        // We sort the movements by the value of the game.
        // So we have a best chance to have the best moves before
        // We want the minimum value, so we took the minimum first
        // So we sort by taking the minimum first
        let mut moves_sorted: Vec<_> = state.movements().collect::<Vec<_>>();
        moves_sorted.sort_by_key(|m| -state.play(m).value());
        for (i, mov) in moves_sorted.into_iter().enumerate() {
            let mut value: i8;
            let serializing_state: String = state.serialize();
            if memoizer.contains_key(&serializing_state) {
                let result = memoizer.get(&serializing_state);
                let v_read = match result {
                    Some(y) => y.1,
                    None => 127,
                };
                value = v_read;
            } else {
                if i == 0 {
                    let (_, v_read) =
                        negascout(&state.play(&mov), depth - 1, -beta, -alpha, memoizer);
                    // And he give us his version of the score, so we need to inverst it
                    value = -v_read;
                // // TODO
                } else {
                    // We keep give at the opponent the contrary of our game
                    // So we invert the alpha beta
                    let (_, v_read) =
                        negascout(&state.play(&mov), depth - 1, -alpha, -alpha - 1, memoizer);
                    // And he give us his version of the score, so we need to inverst it
                    value = -v_read;
                    if alpha < value && value < beta {
                        let (_, v_read_again) =
                            negascout(&state.play(&mov), depth - 1, -value, -alpha, memoizer);
                        // And he give us his version of the score, so we need to inverst it
                        value = -v_read_again;
                    } else {
                        println!("It's good ");
                    }
                }
                memoizer.insert(state.serialize(), (mov, value));
            }
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
        // self.compute_next_move_memoization(state)
        self.compute_next_move_memoization(state)
    }
}
