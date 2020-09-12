# HanaBot
A Rust bot to play Hanabi on Discord

# Architecture :

- `main.rs` : Entry point, where all commands are defined
- `table.rs` : The creation/deletion/list of active tables, allowing to join a game
- `hanabi.rs` : A game of Hanabi
    - `hanabi/player.rs` : A player of Hanabi
    - `hanabi/hand.rs` : The hands of the players
    - `hanabi/stack.rs` : The color stacks on the table
    - `hanabi/card.rs` : A card, with color and information
