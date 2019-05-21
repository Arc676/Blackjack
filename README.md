# Blackjack

A Rust backend crate and command line frontend for the popular card game, Blackjack. This crate is designed to be compilable for linking with C.

# Crate

This crate is available on [crates.io](https://crates.io/crates/blackjack).

The library provides data types for representing decks (specifically, multiple decks combined into a single deck), players, hands, and individual cards. These structs provide the necessary methods for implementing a simple game of Blackjack.

The Rust command line frontend provides a sample implementation as well as functional gameplay experience.

# Linking with C

The repository includes a C header file indicating the exposed methods of the library. A command line frontend to the library written in C is also available. This can be compiled using the included `Makefile`. If the Rust library output is stored at a path other than `project_root/target/debug`, the references will have to be updated.

# Licensing

Project available under GPLv3. See `LICENSE` for full license text.
