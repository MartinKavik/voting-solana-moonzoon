It's my first project with Solana / blockchain so I have several questions for more experienced Solana users. Thank you very much for answers - create a new issue or you are very welcome to join the [MoonZoon Discord](https://discord.gg/eGduTxK2Es).

1. Off-chain `system_instruction::create_account_with_seed` vs `Pubkey::find_program_address` + `system_instruction::create_account` on chain. What are ideal use cases for them? Are there any related best practices?

`system_instruction::create_account_with_seed` allows using the same keypair to manage multiple on-chain accounts. It was more commonly used before the introduction of program derived addresses. It's not super useful now besides creating accounts for programs which don't support PDA's like the stake and vote program.

`Pubkey::find_program_address` is used to derive a "program derived address". You can think of these addresses as being namespaced to a particular on-chain program. It allows programs to "sign" for accounts when they invoke other programs since obviously an on-chain program isn't able to produce a real signature on-chain. Using PDA's is the recommended approach for all programs now. A common pattern is for a program to create PDA accounts for users by invoking the system program create account instruction with the user's address included in the PDA seed list. That way each user has a deterministic address based on their personal wallet address and they don't need to manage an additional keypair in their wallet for that account.

`system_instruction::create_account` requires the created account to "sign" and so if you invoke this in a transaction, you must have access to the keypair. It's probably best for program developers to not require users to create their own accounts, and instead create accounts on a user's behalf which are PDA's.

2. How to use `RpcClient::send_and_confirm_transaction_with_spinner`?

Can you explain what your confusion is here?

3. What are the best practices for setting the `commitment`? Should I write a retry somewhere? The default value `finalized` makes the requests toooo slow and it's a bit hard to find the problem. A related snippet from the repo:
    ```rust
    RpcClient::new_with_commitment(
        "http://localhost:8899".to_owned(),
        CommitmentConfig::confirmed(),
    )
    ```

You should definitely use the `confirmed` commitment. It assumes that < 5% of staked validators are malicious vs `finalized` which assumes < 33% are malicious (and also waits for full lockout by a supermajority of validators (this means > 2/3 of validators cannot vote on a different fork which doesn't include your transaction). There is some more info here: https://docs.solana.com/developing/clients/jsonrpc-api#configuring-state-commitment. We do plan to allow users to set their own X% safety threshold if you think 5% is too low.

4. Is this repo the only Solana example with a Wasm frontend? I wasn't able to make it work with the stable Solana version. The `Cargo.toml` looks like this now:
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

Yes, thank you! (I did write some prototypes like 2 years with wasm, and the explorer had some wasm for decoding stake account state in the past)

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

Yes, it's ok. When a transaction is serialized, account keys are grouped by signer / writable properties. But instructions can use those accounts in any order (and can use the same account multiple times).

6. How to properly create a `Vec<Party>` on chain? (The current implementation creates party accounts with PDAs as pubkeys. One seed used to generate these PDAs is set to the next index value. The current index is set in a "shared" account and incremented in the create account transaction. It's a bit ugly and there may be conflicts because the current index value is read outside of the transaction.)

If your program essentially has a global shared list of ordered accounts, then you're going to have contention for the next account. You could lessen the impact though by having each transaction include the next X party accounts and iterate through to find one that isn't initialized yet? But either way, your program can get DOS'd by someone who tries to front run whenever a new account is made. Another approach is a reservation system where you reserve the next index for a requesting user and they can fulfill that reservation async. You would have some holes in the list though.

7. How to properly define the default program functions and variables? See the current `lib.rs` code:
    ```rust
    // ..
    // @TODO_QUESTION Is possible to read id from "../keypairs/program-pubkey"?
    //
    // @TODO_QUESTION Should I use declare_program! instead? Does it work with the `no-entrypoint` feature?
    // (Do I really need the `no-entrypoint` feature?)
    declare_id!("4dKeVRjqyVNA3n48d1RGf3k2f8fEo1fGsUMPSmsHW4LG");
    ```
    
Your program id is determined by whatever keypair you use locally. When you use `solana program deploy`, it will deploy your program at the address associated with your program keypair. Typically devs grind out a vanity address with `solana-keygen grind`. Including that program id in your lib.rs is mostly just useful when creating a rust sdk for clients that interact with your program. Having "no-entrypoint" is important because otherwise a Rust crate that depends on multiple programs without that feature will try to include conflicting entrypoints. And you just don't need that entrypoint for non-bpf builds.

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

It should be sufficient to check the first byte is non-zero depending on how you setup your encoding. I would also recommend using anchor https://github.com/project-serum/anchor

9. How to store strings and avoid problems with fixed data length? Something like [ArrayString](https://docs.rs/arrayvec/0.7.1/arrayvec/struct.ArrayString.html) with a max length XX bytes?

Typically people just do "text".as_bytes() and store the length inside a `[u8; MAX_LEN]` in an account. ArrayString looks great though. In a few months we'll have reallocatable accounts so you would have more flexibility then. Borsh also supports String.

10. How to store (long) texts with very variable length (imagine a blog article)? Chunk into accounts?

Best solution is to not store large chunks of data. You should instead store large amounts of data on something like IPFS or Arweave and just store a uri to that off-chain data in a solana account. Typically anything you don't need to check in on-chain program logic should be stored off-chain if possible.

11. Is it possible to get the size of a serialized struct by Borsh representing account data more efficiently? Use something like [binary-layout](https://crates.io/crates/binary-layout), but with `LEN` / `size()` instead of Borsh?
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

I'm not very familiar with Borsh encoding, sorry.

12. I understand the Program API is designed for performance and to be as universal as possible, but are there plans to create less error-prone Rust abstraction / interface? Maybe something remotely similar to [Anchor](https://project-serum.github.io/anchor). Do you think [Move Language](https://docs.solana.com/proposals/embedding-move) will help to mitigate this problem?

yeah, the answer right now is Anchor

13. Is the following situation possible: The program fails in runtime because Cross-Program Invocations consume all available computation units because one of the called programs has been upgraded and has become too expensive to run?

Yes, that's possible. We have a new instruction though which when included in a transaction, will allow users to pay for additional compute units. It's not on mainnet yet.
