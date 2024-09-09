#![cfg_attr(feature = "avx512", feature(stdarch_x86_avx512))]

pub mod atom_xml;
pub mod attributes;
pub mod common;
pub mod error;
pub mod serializer;
mod utils;
