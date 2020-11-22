# HanaBot
A Rust bot to play Hanabi on Discord

# Architecture :

- `main.rs` : Entry point, where all commands are defined
- `hanabi.rs` : A game of Hanabi, with the different phases
    - `hanabi/player.rs` : A player of Hanabi
    - `hanabi/hand.rs` : The hands of the players
    - `hanabi/playground.rs` : The table where players play. With the color stacks
    - `hanabi/card.rs` : A card, with color and information
