#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::try_err,
    clippy::must_use_candidate,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::semicolon_if_nothing_returned,
    clippy::shadow_unrelated,
    clippy::missing_errors_doc,
    clippy::similar_names
)]

use solana_program::declare_id;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

mod error;
pub mod instruction;
mod processor;
pub mod state;

// @TODO_QUESTION Is possible to read it from "../keypairs/program-pubkey"?
//
// @TODO_QUESTION Should I use declare_program! instead? Does it work with the `no-entrypoint` feature?
// (Do I really need the `no-entrypoint` feature?)
declare_id!("4dKeVRjqyVNA3n48d1RGf3k2f8fEo1fGsUMPSmsHW4LG");
