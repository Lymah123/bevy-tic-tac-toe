# Bevy Tic-Tac-Toe

![Tic-Tac-Toe Gameplay](assets/Screenshot%20(2279).png)

Tic-Tac-Toe built in Rust with the Bevy game engine.
Features AI opponent using Minimax, responsive UI, and a modular ECS-based design.

## Table of Contents
- [Features](#features)
- [Development Setup](#development-setup)
- [Architecture](#architecture)
- [AI Logic](#ai-logic)
- [Contributing](#contributing)
- [Development Standards](#development-standards)
- [Performance](#performance)
- [Acknowledgements](#acknowledgments)
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

## Deveopment Setup

```
1. Local Development

# Clone and setup
git clone https://github.com/your_username/bevy-tic-tac-toe
cd bevy-tic-tac-toe

# Install dependencies
cargo fetch

# Run in development mode (with debug info)
cargo run

# Run with optimizations
cargo run --release

2. Testing

# Run all tests
cargo test

# Run specific test module
cargo test ai_logic

# Run tests with output
cargo test -- --nocapture

# Test with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html

3. Code Quality

# Format code
cargo fmt

# Lint and suggestions
cargo clippy

# Check for improvements
cargo clippy -- -W clippy::pedantic

# Audit dependencies
cargo audit

```

## Architecture

This game follows the ECS (Entity-Component-System) architecture using Bevy.

| Layer        | Purpose                                           |
|--------------|---------------------------------------------------|
| Components   | Describe game entities (e.g., Cell, Marker)       |
| Systems      | Logic handlers (input, AI, rendering, gameplay)   |
| Resources    | Global state (game mode, board, statistics)       |
| Events       | Decoupled messaging (player move, game over)      |
| AI Logic     | Pure minimax functions (no Bevy dependency)       |


```
src/
├── main.rs              # App setup and system registration
├── components.rs        # ECS components (BoardPosition, CellMark)
├── resources.rs         # Global state (BoardState, GameMode)
├── events.rs           # Event definitions (PlayerMove, GameOver)
├── types.rs            # Game enums and data structures
├── config.rs           # Game constants and configuration
├── ai_logic.rs         # Pure AI algorithms (testable)
└── systems/
    ├── setup.rs        # Game initialization
    ├── input.rs        # Mouse input handling
    ├── gameplay.rs     # Core game logic
    ├── ai.rs          # AI integration
    └── ui.rs          # User interface

```

## AI Logic

The AI supports three difficulty levels:

- 🟢 Easy: Random valid moves
- 🟡 Medium: Minimax with depth limit
- 🔴 Hard: Full-depth Minimax with alpha-beta pruning

The AI is implemented in a pure Rust module (`ai_logic.rs`) so it can be unit tested independently of Bevy.

## Contributing

Contributions are welcome! Please feel free to submit issues, fork the repository, and create pull requests.

### Contribution Guidelines

1. Fork the repository
2. Create a feature branch `(git checkout -b feature/amazing-feature)`
3. Commit your changes `(git commit -m 'Add amazing feature')`
4. Push to the branch `(git push origin feature/amazing-feature)`
5. Open a Pull Request

## Development Standards

- Follow Rust conventions and use `cargo fmt`
- Add unit tests for new functionality
- Update documentation for API changes
- Ensure no clippy warnings in your code

## Performance

- **Startup Time**: < 2 seconds on modern hardware
- **Memory Usage**: ~50MB RAM typical
- **Frame Rate**: 60 FPS consistent on integrated graphics
- **Binary Size**: ~15MB release build

## Acknowledgments

- **Bevy Engine**: [Outstanding ECS game engine](https://bevyengine.org/)
- **Rust Community**: [Excellent ecosystem and support](https://www.rust-lang.org/community)
- **Minimax Algorithm**: [Classic game theory foundation](https://en.wikipedia.org/wiki/Minimax)
- **Open Source Contributors**: Thanks to everyone who contributes!

## Usage

Click on an empty cell to make a move. The game ends when...


## Future Plans

- [ ] Mode selector screen with in-game menu
- [ ] Sound effects and visual effects
- [ ] Mobile-friendly UI (larger touch zones)
- [ ] Undo/Redo move history
- [ ] Export match statistics to file

**Built with ❤️ in Rust**

*If you found this project useful, please consider giving it a ⭐!*
