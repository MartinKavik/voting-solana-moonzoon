# Voting example (MoonZoon app)

> [MoonZoon](http://moonzoon.rs/) is a Rust Fullstack Framework.

## Run on a local machine

1. Follow instructions in [/program/README.md](../program/README.md).

1. Go to the `/app` directory.

1. Install `mzoon` to `cargo_install_root`:
    ```bash
    cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --rev a1ed81c --root cargo_install_root --locked
    ```
    - _Note:_ There will be faster and simpler ways with pre-compiled binaries.

1. Move `cargo_install_root/bin/mzoon` to the project root.
    ```bash
    mv cargo_install_root/bin/mzoon mzoon
    # or
    move cargo_install_root/bin/mzoon mzoon
    ```
    - _Note:_: You can delete the `cargo_install_root` directory now.

1. Build and run:
    ```bash
    ./mzoon start -o
    # or
    mzoon start -o
    ```
    - _Note_: `-o / --open` opens a new tab in your browser.
    - _Note_: The app is much faster when built in the release mode (`-r`).
