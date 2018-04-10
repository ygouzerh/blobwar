//! Dumb greedy algorithm.

use std::fmt;
use super::Strategy;
use configuration::{Configuration, Movement};


extern crate rand;

use self::rand::{Rng, thread_rng};
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
        let mut rng = thread_rng();

        if state.movements().next().is_some() {
            // Each player want to minimize the value
            let mut optimal_value: i8 = 127;
            let mut optimal_mouvement = state.movements().next();
            let mut history_mouvement = Vec::new();


            // For each mouv we play it and we observe the value of the game
            for mov in state.movements() {
                // Play a game will inverse the players, so skip_play is like a second inversion
                let new_value = state.play(&mov).skip_play().value();
                // If we find a better mouve, we update it
                if new_value < optimal_value {
                    optimal_mouvement = Some(mov);
                    history_mouvement.push(optimal_mouvement);
                    optimal_value = new_value;
                }
            }
<<<<<<< HEAD

            optimal_mouvement=history_mouvement[rng.gen_range(0,history_mouvement.len())];
=======
            //for i in history_mouvement.iter(){
                //println!(">>{:?}", i );
        //    }

            optimal_mouvement=history_mouvement[rng.gen_range(0,history_mouvement.len())];
            //println!("Choise : {:?}", optimal_mouvement);
>>>>>>> 2a0846954b5b2f1cd3b8df6112e4a44be7d059b7

            // We return the best movement
            // return optimal_mouvement;
            return optimal_mouvement;
        } else {
            return None;
        }
    }
}
