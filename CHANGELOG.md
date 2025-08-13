# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Tetris Synthwave
- Classic Tetris gameplay with 10x20 board
- 7 standard tetrominoes (I, O, T, S, Z, J, L)
- Super Rotation System (SRS) for piece rotation
- 7-bag randomizer for fair piece distribution
- Ghost piece preview
- Hold function for pieces
- Combo chain system (2.5s window, up to 5x multiplier)
- Power cells with Nova and SlowTime effects (~5% spawn rate)
- Gravity modes: Normal and Pulse (sine wave oscillation)
- Synthwave theming with light and dark variants
- Right sidebar with preview queue, controls, and statistics
- Persistent settings and high score storage
- Windows MSI installers for x64 and ARM64 architectures
- Comprehensive test suite
- GitHub Actions CI/CD pipeline

### Game Features
- **Board**: 10x20 grid with visible grid lines
- **Pieces**: Standard 7 tetrominoes with SRS rotation
- **Randomizer**: 7-bag system ensures fair distribution
- **Scoring**: Classic Tetris scoring system with level progression
- **Preview**: 3-piece preview queue
- **Hold**: Store one piece for later use
- **Ghost**: Toggle-able ghost piece preview
- **Combo**: Chain line clears for multiplier bonuses
- **Power Cells**: Special effects when cleared in lines
- **Gravity**: Normal or pulse oscillation modes

### UI/UX
- **Theme**: Synthwave aesthetics with neon accents
- **Variants**: Light and dark theme options  
- **Layout**: Game board left, controls sidebar right
- **Accessibility**: No rapid flashing, good contrast
- **Controls**: Customizable key bindings
- **Statistics**: Real-time score, level, lines, combo display

### Technical
- **Language**: Rust (stable toolchain)
- **UI Framework**: eframe/egui for lightweight, fast rendering
- **Performance**: 60 FPS target, optimized game loop
- **Persistence**: JSON settings storage in OS config directory
- **Testing**: Unit tests for all game logic components
- **Packaging**: MSI installers via WiX Toolset
- **CI/CD**: Automated building and testing via GitHub Actions

### Controls
- **Movement**: Arrow keys (left/right), down for soft drop
- **Rotation**: Z (counterclockwise), X (clockwise)
- **Actions**: Space (hard drop), C (hold)
- **Game**: P (pause), R (restart), Esc (quit)

## [0.1.0] - 2025-01-XX

### Added
- Initial project setup and core architecture
- Basic game loop and rendering system
- Core Tetris mechanics implementation
- Windows installer packaging
- Documentation and contributing guidelines

---

## Version History

- **v0.1.0**: Initial release with core Tetris gameplay, synthwave theming, power-ups, and Windows installers
