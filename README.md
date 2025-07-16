# Bevy Tic-Tac-Toe

![Tic-Tac-Toe Gameplay](assets/demo.gif)

Tic-Tac-Toe built in Rust with the Bevy game engine.
Features AI opponent using Minimax, responsive UI, and a modular ECS-based design.

## Table of Contents
- [Features](#features)
- [Setup](#setup)
- [Architecture](#architecture)
- [AI Logic](#ai-logic)
- [Future Plans](#future-plans)
- [License](#license)
- [Usage](#usage)

## Features
- Multiple game modes (Human vs Human, Human vs AI)
- AI with three difficulty levels (Easy, Medium, Hard)
- Minimax algorithm with alpha-beta pruning
- Game statistics tracking
- Modern graphics with smooth animations
- Responsive mouse interaction
- Clear win/draw messages
- Modular ECS architecture
- Comprehensive documentation and testing

## Architecture

This game follows the ECS (Entity-Component-System) architecture using Bevy.

| Layer        | Purpose                                           |
|--------------|---------------------------------------------------|
| Components   | Describe game entities (e.g., Cell, Marker)       |
| Systems      | Logic handlers (input, AI, rendering, gameplay)   |
| Resources    | Global state (game mode, board, statistics)       |
| Events       | Decoupled messaging (player move, game over)      |
| AI Logic     | Pure minimax functions (no Bevy dependency)       |



## AI Logic

The AI supports three difficulty levels:

- 🟢 Easy: Random valid moves
- 🟡 Medium: Minimax with depth limit
- 🔴 Hard: Full-depth Minimax with alpha-beta pruning

The AI is implemented in a pure Rust module (`ai_logic.rs`) so it can be unit tested independently of Bevy.

## Setup
```bash
git clone https://github.com/your_username/bevy_tic_tac_toe
cd bevy_tic_tac_toe
cargo run
```
## Usage

Click on an empty cell to make a move. The game ends when...


## Future Plans

- [ ] Mode selector screen with in-game menu
- [ ] Sound effects and visual effects
- [ ] Mobile-friendly UI (larger touch zones)
- [ ] Undo/Redo move history
- [ ] Export match statistics to file
