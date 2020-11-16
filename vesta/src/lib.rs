#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the prime-order curve Vesta, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Pallas, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Pallas.
//!
//!
//! Curve information:
//! Vesta:
//! * Base field:
//!   F_{28948022309329048855892746252171976963363056481941647379679742748393362948097}
//! * Scalar field:
//!   F_{28948022309329048855892746252171976963363056481941560715954676764349967630337}
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(p - 1, 2) = 32
//! * Valuation(q - 1, 2) = 32

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;
