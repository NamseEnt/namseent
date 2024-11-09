//! # BPTree - B+ Tree implementation in Rust
//!
//! BPTree implements a B+ Tree with the backing storage as a file.
//! - File system based
//! - Strong consistency guarantees by using a write-ahead log(WAL)
//! - Memory cache for fast reads
//!

mod checksum;
pub mod id_set;

use checksum::*;
