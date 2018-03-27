//! Dumb greedy algorithm.
use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        if state.movements().next().is_some() {
            println!("current_player is : {:?}", state.current_player);
            let mut optimal_value: i8;
            if state.current_player {
                // Blue need to maximaxe the value
                optimal_value = -128;
            } else {
                // Red need to minimize it
                optimal_value = 127;
            }
            let mut optimal_mouvement = state.movements().next();
            for mouv in state.movements() {
                let new_value = state.play(&mouv).value();
                println!("Value : {}, Mouvement : {:?}", new_value, mouv);
                if state.current_player {
                    if new_value > optimal_value {
                        optimal_mouvement = Some(mouv);
                        optimal_value = new_value;
                    }
                } else {
                    if new_value > optimal_value {
                        optimal_mouvement = Some(mouv);
                        optimal_value = new_value;
                    }
                }
            }
            println!("We have taken : {:?}", optimal_mouvement);
            return optimal_mouvement;
        } else {
            return None;
        }
    }
}
