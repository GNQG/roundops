#![cfg_attr(feature = "hwrm", feature(cfg_target_feature))]

extern crate core;
extern crate float_traits;
extern crate num_traits;
#[cfg(test)]
extern crate rand;

mod roundops;
pub mod methods;
pub mod utils;

pub use roundops::*;
