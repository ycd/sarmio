//! Distributed unique ID generator inspired by Twitter's snowflake.
//! https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake.html
//!
//! How to use?
//!
//! First of all, add this as a dependency to your cargo.toml
//! ```toml
//! [dependencies]
//! sarmio = "0.1"
//! ```
//!
//!
//! Example
//!
//! fn main() {
//!     // Create new Sarmio instance with a machine-id of 255.
//!     let mut s = sarmio::Sarmio::new(255);
//!     
//!     // Sarmio implements Iterator
//!     // Which means you can iterate over it to create new IDs.
//!     let v = match s.next() {
//!         Some(s) => s,
//!         None => 0,
//!     };
//!     println!("{}", v);
//!     
//!     // Decompose it, get the values like
//!     // Unix time in that moment, machine id
//!     // and the Unique ID.
//!     let p = sarmio::decompose(v);  
//!
//!     println!("{:?}", p);    // {"id": 18190711796065697536, "time": 1610671519, "machine-id": 255}
//! }

mod sarmio;

pub use crate::sarmio::*;
