//! Compatibility path for the raw upgrade codec.
//!
//! New code should use [`super::codec`]. This module is retained so callers
//! that imported the old module path can migrate without changing the raw
//! `UpgradeWireEntry` wire representation.
