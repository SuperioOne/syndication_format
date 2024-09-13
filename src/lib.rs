#![cfg_attr(feature = "avx512", feature(stdarch_x86_avx512))]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod atom;
pub mod common;
pub mod error;
pub mod escape;
pub mod serializer;
mod utils;
