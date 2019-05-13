//! Sha256 based proof of work over a typed piece of data.
//!
//! Any type that implementes serde::Deserialize can be tagged with a proof of work.
//!
//! # Examples
//!
//! Prove we did work targeting a phrase.
//!
//! ```rust
//! use pow::Pow;
//!
//! // very easy mode
//! let difficulty = u128::max_value() - u128::max_value() / 2;
//!
//! let phrase = b"Phrase to tag.".to_vec();
//! let pw = Pow::prove_work(&phrase, difficulty).unwrap();
//! assert!(pw.score(&phrase).unwrap() >= difficulty);
//! ```
//!
//! Prove more difficult work. This time targeting a time.
//!
//! ```rust
//! # fn get_unix_time_seconds() -> u64 {
//! #     use std::time::{Duration, SystemTime};
//! #     SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
//! # }
//! # use pow::Pow;
//!
//! // more diffcult, takes around 100_000 hashes to generate proof
//! let difficulty = u128::max_value() - u128::max_value() / 100_000;
//!
//! let now: u64 = get_unix_time_seconds();
//! let pw = Pow::prove_work(&now, difficulty).unwrap();
//! assert!(pw.score(&now).unwrap() >= difficulty);
//! ```

//! # Score scheme
//!
//! To score a proof of work for a given (target, Pow) pair:
//! Sha256 is calculated over the concatenation SALT + target + Pow.
//! The first 16 bytes of the hash are interpreted as a 128 bit unsigned integer.
//! That integer is the score.
//! A constant, SALT, is used as prefix to prevent pow reuse from other systems such as proof
//! of work blockchains.
//!
//! In other words:
//!
//! ```rust
//! # use serde::Serialize;
//! # use pow::Pow;
//! # use core::any::Any;
//! # const SALT: &'static str = "not the actual salt used";
//! # fn serialize<T: Serialize>(_: &T) -> u8 { 0 } // not the actual serialize function
//! # fn deserialize(_: &[u8]) -> u128 { 0 } // not the actual deserialize function
//! # fn sha256(_: &u8) -> [u8; 32] { [0; 32] } // not the actual sha256 function
//! fn score<T: Serialize>(target: &T, pow_tag: &Pow<T>) -> u128 {
//!     let bytes = serialize(&SALT) + serialize(target) + serialize(pow_tag);
//!     let hash = sha256(&bytes);
//!     deserialize(&hash[..16])
//! }
//! ```
//!
//! # Serialization encoding.
//!
//! It shouldn't matter to users of this library, but the bincode crate is used for cheap
//! deterministic serialization. All values are serialized using network byte order.
//!
//! # Threshold scheme
//!
//! Given a minimum score m. A Pow p satisfies the minimum score for target t iff score(t, p) >= m.
//!
//! # Choosing a difficulty setting.
//!
//! Difficulty settings are usually best adjusted dynamically a la bitcoin.
//!
//! To manually select a difficulty, choose the average number of hashes required.
//!
//! ```rust
//! fn difficulty(average: u128) -> u128 {
//!     debug_assert_ne!(average, 0, "It is impossible to prove work in zero attempts.");
//!     let m = u128::max_value();
//!     m - m / average
//! }
//! ```
//!
//! Conversely, to calculate probable number of hashes required to satisfy a given minimum
//! difficulty.
//!
//! ```rust
//! fn average(difficulty: u128) -> u128 {
//!     let m = u128::max_value();
//!     if difficulty == m {
//!         return m;
//!     } 
//!     m / (m - difficulty)
//! }
//! ```

mod proof_of_work;

pub use proof_of_work::Pow;
