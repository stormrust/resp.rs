//! A RESP (REdis Serialization Protocol) parser for Rust.
pub use self::data::Data;
pub use self::encode::*;
pub use self::decode::*;

mod data;
mod encode;
mod decode;
