# Voting example (Solana program)

> [Solana](https://solana.com/) is a decentralized blockchain.

## Run on a local machine

1. Check you've installed [Rust](https://www.rust-lang.org/):
    ```bash
    rustc -V # rustc 1.55.0 (c8dfcfe04 2021-09-06)
    ```

1. Check you've installed [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools):
    ```bash
    solana -V # solana-cli 1.7.12 (src:ca83167c; feat:2013646575)
    ```
    - Note: The example has been developed on Kubuntu 21.04.
    - Note: Use WSL on Windows.

1. Go to the `/program` directory.

1. Start Solana localnet:
    ```bash
    solana-test-validator -C ./config.yml
    ```
    - Tip: `-r, --reset:  Reset the ledger to genesis if it exists.`

1. Open a new terminal tab and go to `/program` again.

1. Airdrop SOLs
    ```bash
    solana airdrop -C ./config.yml 1 ./keypairs/voting-owner-keypair.json

    solana airdrop -C ./config.yml 1 ./keypairs/voter-keypair.json
    ```

1. Build the program:
    ```bash
    cargo build-bpf
    ```

1. Deploy the program:
    ```bash
    solana program deploy -C ./config.yml --program-id ./keypairs/program-keypair.json ./target/deploy/voting_program.so
    ```

1. Continue in [/app/README.md](../app/README.md).
