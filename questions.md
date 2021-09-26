It's my first project with Solana / blockchain so I have several questions for more experienced Solana users. Thank you very much for answers - create a new issue or you are very welcome to join the [MoonZoon Discord](https://discord.gg/eGduTxK2Es).

1. Off-chain `system_instruction::create_account_with_seed` vs `Pubkey::find_program_address` + `system_instruction::create_account` on chain. What are ideal use cases for them? Are there any related best practices?

2. How to use `RpcClient::send_and_confirm_transaction_with_spinner`?

3. What are the best practices for setting the `commitment`? Should I write a retry somewhere? The default value `finalized` makes the requests toooo slow and it's a bit hard to find the problem. A related snippet from the repo:
    ```rust
    RpcClient::new_with_commitment(
        "http://localhost:8899".to_owned(),
        CommitmentConfig::confirmed(),
    )
    ```

4. Is this example the only Solana example with a Wasm frontend? I wasn't able to make it work with the stable Solana version. The `Cargo.toml` looks like this now:
    ```toml
    # @TODO Replace with stable solana-* versions. 
    # We need newer memmap2 (it's a transitive depedency), 
    # because older versions can't be compiled on Wasm (https://github.com/RazrFalcon/memmap2-rs/pull/10/files).
    #
    # Commit 92e72d99 seems to work with Solana 1.7.12.
    #
    # (No luck with disabling the `solana-sdk` feature `memmap2`.)
    #
    # solana-sdk = { version = "=1.7.12", default-features = false }
    solana-sdk = { git = "https://github.com/solana-labs/solana", rev="92e72d99" }
    ```

5.  Is ok to mix writable and read-only accounts in the account order? See this code snippet:
    ```rust
    /// ...
    /// Accounts expected:
    ///
    /// 0. `[writable, signer]` The voting owner account.
    /// 1. `[]` The voting state account.
    /// 2. `[writable]` The voter votes account.
    /// 3. `[]` The system program.
    AddVoter {
        voter_pubkey: Pubkey,
        voter_votes_bump_seed: u8,
    },
    ```

6. How to properly create a `Vec<Party>` on chain? (The current implementation creates party accounts with PDAs as pubkeys. One seed used to generate these PDAs is set to the next index value. The current index is set in a "shared" account and incremented in the create account transaction. It's a bit ugly and there may be conflicts because the current index value is read outside of the transaction.)

7. How to properly set a program "metadata"? See the current `lib.rs` code:
    ```rust
    // ..
    // @TODO_QUESTION Is possible to read it from "../keypairs/program-pubkey"?
    //
    // @TODO_QUESTION Should I use declare_program! instead? Does it work with the `no-entrypoint` feature?
    // (Do I really need the `no-entrypoint` feature?)
    declare_id!("4dKeVRjqyVNA3n48d1RGf3k2f8fEo1fGsUMPSmsHW4LG");
    ```

8. How to efficiently check if the account data have been initialized? Is it even possible when the data are encoded by Borsh? Or I have to use manual encoding or something like [binary-layout](https://crates.io/crates/binary-layout)? The current ugly solution: 
    ```rust
    if !voting_state_account
        .try_borrow_data()?
        .iter()
        .all(|byte| *byte == 0)
    {
        Err(ProgramError::AccountAlreadyInitialized)?
    }
    ```

9. How to store strings and avoid problems with fixed data length? Something like [ArrayString](https://docs.rs/arrayvec/0.7.1/arrayvec/struct.ArrayString.html) with a max length XX bytes?

10. How to store (long) texts with very variable length (imagine a blog article)? Chunk into accounts?

11. Is possible to get the size of a serialized struct by Borsh representing account data more efficiently? Use something like [binary-layout](https://crates.io/crates/binary-layout), but with `LEN` / `size()` instead of Borsh?
    ```rust
    impl VotingState {
        pub fn serialized_size() -> usize {
            // @TODO_QUESTION compute once? Use something like https://crates.io/crates/binary-layout
            // but with LEN/size()?
            Self::default()
                .try_to_vec()
                .expect("failed to serialize default VotingState")
                .len()
        }
    }
    ```

12. I understand the Program API is designed for performance and to be as universal as possible, but are there plans to create less error-prone Rust abstraction/interface? Maybe something remotely similar to [Anchor](https://project-serum.github.io/anchor). Do you think [Move Language](https://docs.solana.com/proposals/embedding-move) will help with this problem?

13. Is possible the situation where the program fails in runtime because Cross-Program Invocations consume all available computation units because one of the called programs has been upgraded and has become too expensive to run?
