//! # howick-rs
//!
//! Rust library for generating and parsing Howick FRAMA machine CSV files
//! for light gauge steel (LGS) framing.
//!
//! ## Overview
//!
//! Howick FRAMA roll-forming machines consume CSV files that describe each
//! steel member to be produced — its length, and the positions of every
//! punch, dimple, lip cut, swage, and web hole along it.
//!
//! This crate provides:
//! - A type model for Howick framesets and components ([`types`])
//! - A CSV parser ([`csv::parse`]) to read existing Howick CSV files
//! - A CSV serialiser ([`csv::serialize`]) to generate new files

pub mod csv;
pub mod error;
pub mod types;
