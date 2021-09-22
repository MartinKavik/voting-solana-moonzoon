use solana_program::declare_id;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod instruction;
mod error;
mod processor;
pub mod state;

// @TODO Is possible to read it from "../keypairs/program-pubkey"?
//
// @TODO Should I use declare_program! instead? Does it work with the `no-entrypoint` feature? 
// (Do I really need the `no-entrypoint` feature?)
declare_id!("4dKeVRjqyVNA3n48d1RGf3k2f8fEo1fGsUMPSmsHW4LG");
