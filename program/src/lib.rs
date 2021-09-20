use solana_program::declare_id;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

mod instruction;
mod error;
mod processor;
mod state;

// @TODO Read the pubkey from file.
// Note: Only `Pubkey::new_from_array` is `const`, 
// but it doesn't accept a base58-encoded pubkey read by `include_str!/bytes`.
// And the `declare_id` macro doesn't accept `&str`. Other libs decode only base64/hex/32.
// 
// = "../keypairs/program-pubkey"
declare_id!("4dKeVRjqyVNA3n48d1RGf3k2f8fEo1fGsUMPSmsHW4LG");
