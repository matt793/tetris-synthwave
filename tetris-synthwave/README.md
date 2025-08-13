# Tetris Synthwave

A modern, minimal, synthwave-themed desktop Tetris game built in Rust with eframe/egui.

![Tetris Synthwave Screenshot](assets/screenshot.png)

## Features

- **Classic Tetris gameplay**: 10x20 board, 7 standard tetrominoes, SRS rotation system
- **Modern mechanics**: 7-bag randomizer, ghost pieces, hold function
- **Unique power-ups**:
  - **Combo chains**: Consecutive line clears within 2.5s build multipliers up to x5
  - **Power Cells**: ~5% of pieces contain power cells that trigger special effects
  - **Gravity modes**: Normal or Pulse (oscillating gravity)
- **Synthwave aesthetics**: Neon accents, dark gradients, retro styling
- **Dual themes**: Light and dark variants
- **Accessibility**: No rapid flashing, good color contrast, customizable settings

## Controls

| Key | Action |
|-----|--------|
| ←/→ | Move piece left/right |
| ↓ | Soft drop |
| Space | Hard drop |
| Z | Rotate counterclockwise |
| X | Rotate clockwise |
| C | Hold piece |
| P | Pause/Resume |
| R | Restart game |
| Esc | Quit |

## Installation

### Windows

Download the latest MSI installer for your architecture:

- **x64**: `tetris-synthwave-x86_64.msi` 
- **ARM64**: `tetris-synthwave-aarch64.msi`

Double-click the MSI file to install. The game will be available in your Start Menu and Desktop.

### Building from Source

#### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [WiX Toolset](https://wixtoolset.org/releases/) 3.14+ (for building installers)

#### Quick Start

```bash
# Clone the repository
git clone https://github.com/your-username/tetris-synthwave.git
cd tetris-synthwave

# Run the game
cargo run

# Run in release mode for better performance
cargo run --release
```

#### Building Installers

```bash
# Install cargo-wix
cargo install cargo-wix

# Build x64 installer
cargo build --release --target x86_64-pc-windows-msvc
cargo wix --target x86_64-pc-windows-msvc --no-build

# Build ARM64 installer (requires aarch64 target)
rustup target add aarch64-pc-windows-msvc
cargo build --release --target aarch64-pc-windows-msvc
# Switch to ARM64 WiX config
cp wix/main-aarch64.wxs wix/main.wxs
cargo wix --target aarch64-pc-windows-msvc --no-build
```

## Development

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings
```

### Project Structure

```
src/
├── main.rs          # Entry point
├── app.rs           # Main application logic
├── game/            # Game logic modules
│   ├── mod.rs       # Game state and main loop
│   ├── board.rs     # Board representation and logic
│   ├── piece.rs     # Tetromino definitions and operations
│   ├── srs.rs       # Super Rotation System
│   ├── random.rs    # 7-bag randomizer
│   ├── scoring.rs   # Scoring and level progression
│   └── modifiers.rs # Power cells, combos, gravity modes
└── ui/              # User interface modules
    ├── mod.rs       # UI coordination
    ├── theme.rs     # Synthwave theme definitions
    ├── panel.rs     # Right sidebar panel
    └── draw.rs      # Game board rendering
```

## Power-Ups Guide

### Combo Chains
Clear lines within 2.5 seconds of each other to build combo multipliers:
- 2+ consecutive clears: 2x multiplier
- 3+ consecutive clears: 3x multiplier  
- 4+ consecutive clears: 4x multiplier
- 5+ consecutive clears: 5x multiplier (max)

### Power Cells
Special glowing cells (~5% spawn rate) that trigger effects when cleared:
- **Nova**: Clears all adjacent pieces in a 1-block radius
- **Slow Time**: Reduces gravity by 50% for 5 seconds

### Gravity Modes
- **Normal**: Standard constant gravity
- **Pulse**: Gravity oscillates in a sine wave (~8 second period)

## Configuration

Settings are automatically saved to:
- **Windows**: `%APPDATA%/tetris-synthwave/settings.json`

Configurable options:
- Theme (Light/Dark)
- Ghost pieces (On/Off)
- Gravity mode (Normal/Pulse)
- Key bindings
- High scores

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute to this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history and changes.
