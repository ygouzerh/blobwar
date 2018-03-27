//! rust alpha - beta implementation for the blobwar game.
#![feature(conservative_impl_trait)]
#![feature(range_contains)]
#![deny(missing_docs)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate itertools;
extern crate libc;
extern crate nix;
extern crate term;

pub(crate) mod positions;
pub(crate) mod shmem;
pub mod configuration;
pub mod strategy;
pub mod board;
