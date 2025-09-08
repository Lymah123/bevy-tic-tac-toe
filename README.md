[![CI](https://github.com/Lymah123/bevy-tic-tac-toe/workflows/CI/badge.svg)](https://github.com/Lymah123/bevy-tic-tac-toe/actions)
[![Security Audit](https://github.com/Lymah123/bevy-tic-tac-toe/workflows/Security%20Audit/badge.svg)](https://github.com/Lymah123/bevy-tic-tac-toe/actions)
[![Latest Release](https://img.shields.io/github/v/release/Lymah123/bevy-tic-tac-toe)](https://github.com/Lymah123/bevy-tic-tac-toe/releases/latest)

## 🎮 **Download & Play**

[![Download for Windows](https://img.shields.io/badge/Download-Windows-blue?style=for-the-badge&logo=windows)](https://github.com/Lymah123/bevy-tic-tac-toe/releases/latest/download/bevy-tic-tac-toe-windows-x86_64.exe)
[![Download for macOS](https://img.shields.io/badge/Download-macOS-lightgrey?style=for-the-badge&logo=apple)](https://github.com/Lymah123/bevy-tic-tac-toe/releases/latest/download/bevy-tic-tac-toe-macos-x86_64)
[![Download for Linux](https://img.shields.io/badge/Download-Linux-orange?style=for-the-badge&logo=linux)](https://github.com/Lymah123/bevy-tic-tac-toe/releases/latest/download/bevy-tic-tac-toe-linux-x86_64)

*Download the executable for your platform - no installation required!*

---

# Bevy Tic-Tac-Toe

![Tic-Tac-Toe Gameplay](assets/Screenshot%20(2302).png)

Tic-Tac-Toe built in Rust with the Bevy game engine.
Features AI opponent using Minimax, responsive UI, and a modular ECS-based design.

## Table of Contents
- [Download & Play](#-download--play)
- [Features](#features)
- [Quick Start](#quick-start)
- [Development Setup](#development-setup)
- [Architecture](#architecture)
- [AI Logic](#ai-logic)
- [Contributing](#contributing)
- [Development Standards](#development-standards)
- [Performance](#performance)
- [Tech Stack](#tech-stack)
- [Future Plans](#future-plans)
- [License](#license)

## Features

- ✅ **Human vs AI gameplay** with intelligent opponent
- 🧠 **Minimax algorithm** with alpha-beta pruning for unbeatable AI
- 🎮 **Responsive mouse controls** - click any cell to play
- 🏗️ **Modern ECS architecture** using Bevy game engine
- 🔄 **Automatic turn management** and game flow
- 📱 **Cross-platform support** (Windows, macOS, Linux)
- 🧪 **Comprehensive unit tests** for AI logic
- ⚡ **High performance** - native Rust speed

## Quick Start

### Option 1: Download Executable (Recommended)
1. **[Download for your platform](#-download--play)** using the buttons above
2. **Run the executable** - double-click to play
3. **No installation needed** - runs immediately

### Option 2: Build from Source
```bash
git clone https://github.com/Lymah123/bevy-tic-tac-toe
cd bevy-tic-tac-toe
cargo run --release
```

## Development Setup

### Local Development

```bash
# Clone and setup
git clone https://github.com/Lymah123/bevy-tic-tac-toe
cd bevy-tic-tac-toe

# Install dependencies
cargo fetch

# Run in development mode (with debug info)
cargo run

# Run with optimizations
cargo run --release
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test ai_logic

# Run tests with output
cargo test -- --nocapture

# Test with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Code Quality

```bash
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

- **Download Size**: ~15MB executable
- **Startup Time**: < 1 second on modern hardware
- **Memory Usage**: ~50MB RAM typical
- **Frame Rate**: 60 FPS consistent
- **No Dependencies**: Self-contained executable

## Tech Stack

### Core Technologies
- **[Rust](https://www.rust-lang.org/)** - Systems programming language
- **[Bevy Engine](https://bevyengine.org/)** - Data-driven game engine

### Dependencies
```toml
[dependencies]
bevy = "0.11"           # Main game engine
```

### Architecture Patterns
- **ECS (Entity-Component-System)** - Bevy's core architecture
- **Event-Driven Design** - Decoupled system communication
- **Resource Management** - Global state handling
- **System Scheduling** - Ordered execution pipelines

## Acknowledgments

- **Bevy Engine**: [Outstanding ECS game engine](https://bevyengine.org/)
- **Rust Community**: [Excellent ecosystem and support](https://www.rust-lang.org/community)
- **Minimax Algorithm**: [Classic game theory foundation](https://en.wikipedia.org/wiki/Minimax)
- **Open Source Contributors**: Thanks to everyone who contributes!

## Usage

### How to Play

1. **Download and run** the executable for your platform
2. **Make your move** - Click any empty cell (you play as X)
3. **AI responds** - The AI automatically places O
4. **Win conditions** - Get 3 in a row (horizontal, vertical, or diagonal)

### System Requirements
- **Windows**: Windows 10+ (64-bit)
- **macOS**: macOS 10.12+ (64-bit)
- **Linux**: Most modern distributions (64-bit)
- **RAM**: 50MB minimum
- **Storage**: 15MB disk space

### Controls

| Action | Input |
|--------|-------|
| Place mark | Left mouse click on empty cell |
| Restart game | R key (if implemented) |
| Exit game | Alt + F4 / Cmd + Q |

## Future Plans

- [ ] Mode selector screen with in-game menu
- [ ] Sound effects and visual effects
- [ ] Mobile-friendly UI (larger touch zones)
- [ ] Undo/Redo move history
- [ ] Export match statistics to file

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### MIT License Summary
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Private use allowed
- ❌ No warranty provided

**Built with ❤️ in Rust**

*If you found this project useful, please consider giving it a ⭐!*
