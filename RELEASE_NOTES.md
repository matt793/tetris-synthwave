# Release Information

## Version 0.1.1 (Bug Fix Release)

### Tag
`v0.1.1`

### Date
January 12, 2025

### Changes
- **Fixed**: Piece randomization now uses a truly random seed instead of a fixed seed, ensuring different piece sequences each game
- **Fixed**: Console window no longer appears when launching the game from the MSI installer on Windows
- **Updated**: WiX configuration to use correct build output path

### Technical Details
- Modified `SevenBag::new()` in `src/game/random.rs` to generate random seeds using `thread_rng()`
- Added `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` attribute to hide console in release builds
- Corrected WiX source path from `target/x86_64-pc-windows-msvc/release/` to `target/release/`

---

## Version 0.1.0 (Initial Release)

### Tag
`v0.1.0`

## Title
Tetris Synthwave v0.1.0 - Initial Release 🎮✨

## Description

### 🚀 Introducing Tetris Synthwave

We're excited to announce the first official release of **Tetris Synthwave** - a modern reimagining of the classic block-stacking puzzle game with stunning synthwave aesthetics and innovative gameplay mechanics!

Built with **Rust** for blazing-fast performance and featuring a beautiful neon-soaked visual design, this desktop game brings fresh energy to the timeless Tetris formula.

---

### ✨ Key Features

#### 🎮 **Classic Gameplay, Modern Polish**
- Standard 10x20 board with all 7 tetrominoes
- Super Rotation System (SRS) for smooth, predictable piece control
- 7-bag randomizer ensuring fair piece distribution
- Ghost pieces, hold function, and 3-piece preview queue
- Soft drop and hard drop controls

#### ⚡ **Innovative Mechanics**
- **Combo Chain System**: Build multipliers up to 5x by clearing lines consecutively
- **Power Cells**: Special glowing blocks (~5% spawn rate) that grant abilities:
  - 🌟 **Nova**: Clears adjacent blocks instantly
  - ⏰ **Slow Time**: Reduces gravity by 50% for 5 seconds
- **Gravity Modes**: Choose between normal or pulse (oscillating) gravity

#### 🎨 **Synthwave Aesthetics**
- Beautiful dark theme with neon magenta/cyan accents
- Light theme option with soft pastels
- Clean, modern UI with the Inter font family
- Smooth 60 FPS rendering
- No rapid flashing - safe for photosensitive users

#### 🛠️ **Technical Excellence**
- Written in pure Rust for performance and memory safety
- Lightweight ~5MB installer with zero runtime dependencies
- Uses eframe/egui for responsive, immediate-mode UI
- Persistent high scores and settings
- Cross-architecture support (x64 and ARM64)

---

### 📦 Installation

Download the appropriate MSI installer for your system:

| Architecture | File | Size | SHA256 |
|-------------|------|------|--------|
| **x64** (Most PCs) | `tetris-synthwave-0.1.0-x86_64.msi` | 5.1 MB | `8f3a2b1c...` |
| **ARM64** (Surface Pro X) | `tetris-synthwave-0.1.0-aarch64.msi` | 4.8 MB | `7d4e9f2a...` |

Simply run the installer and follow the prompts. The game will be installed to your Program Files directory and a shortcut will be added to your Start Menu.

---

### 🎮 Controls

| Key | Action |
|-----|--------|
| **←/→** | Move piece |
| **↓** | Soft drop |
| **Space** | Hard drop |
| **Z** | Rotate counterclockwise |
| **X** | Rotate clockwise |
| **C** | Hold piece |
| **G** | Toggle ghost |
| **M** | Switch gravity mode |
| **T** | Toggle theme |
| **P** | Pause |
| **R** | Restart |
| **Esc** | Quit |

---

### 📋 System Requirements

**Minimum:**
- Windows 10 version 1809 or later
- 1 GHz dual-core processor
- 2 GB RAM
- DirectX 11 compatible graphics
- 50 MB storage space

**Recommended:**
- Windows 11
- 2 GHz quad-core processor
- 4 GB RAM
- DirectX 12 compatible graphics

---

### 🙏 Acknowledgments

This project wouldn't be possible without:
- The amazing Rust community and ecosystem
- The egui team for their fantastic immediate-mode GUI library
- Alexey Pajitnov, creator of the original Tetris
- The synthwave aesthetic movement for visual inspiration

---

### 🐛 Known Issues

This is our first release, and while we've tested extensively, you may encounter minor issues. Please report any bugs via our [GitHub Issues](https://github.com/matt793/tetris-synthwave/issues) page.

---

### 🚀 What's Next?

We're already working on v0.2.0, which will include:
- 🎵 Original synthwave soundtrack
- 🏆 Achievement system
- 🌐 Online leaderboards
- 🎯 New challenge modes

---

### 📝 Contributing

We welcome contributions! Check out our [Contributing Guide](https://github.com/matt793/tetris-synthwave/blob/main/CONTRIBUTING.md) to get started.

---

**Thank you for playing Tetris Synthwave!** 🎮✨

If you enjoy the game, please consider:
- ⭐ Starring the repository
- 🐛 Reporting any issues you find
- 💡 Suggesting new features
- 🤝 Contributing to the project

---

*Made with ❤️ and 🦀 Rust*
