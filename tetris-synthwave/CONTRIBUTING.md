# Contributing to Tetris Synthwave

Thank you for your interest in contributing to Tetris Synthwave! This document provides guidelines and information for contributors.

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Git](https://git-scm.com/)
- [WiX Toolset](https://wixtoolset.org/releases/) 3.14+ (for building Windows installers)

### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/tetris-synthwave.git
   cd tetris-synthwave
   ```
3. Create a new branch for your feature:
   ```bash
   git checkout -b feature/your-feature-name
   ```
4. Make your changes
5. Test your changes:
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```
6. Commit and push your changes
7. Create a Pull Request

## Code Style

### Formatting

- Use `cargo fmt` to format your code
- Follow standard Rust conventions
- Use 4 spaces for indentation
- Line length limit: 100 characters

### Naming Conventions

- Functions and variables: `snake_case`
- Types and traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Documentation

- Document all public functions with doc comments (`///`)
- Include examples in doc comments when helpful
- Keep comments concise and clear
- Update README.md if adding new features

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for all game logic functions
- Place tests in the same file as the code they test
- Use descriptive test names: `test_should_clear_line_when_full`
- Test both happy path and edge cases

### Test Coverage Areas

- **Board logic**: Line clearing, piece placement, collision detection
- **Piece operations**: Rotation, movement, SRS kicks
- **Randomizer**: 7-bag sequence generation
- **Scoring**: Points calculation, level progression
- **Power-ups**: Combo chains, power cell effects
- **Settings**: Save/load functionality

## Performance Guidelines

- Avoid allocations in the main game loop
- Use `&str` instead of `String` when possible  
- Prefer iterators over manual loops
- Profile code if adding complex features
- Target 60 FPS performance

## UI Guidelines

### egui Best Practices

- Keep UI logic separate from game logic
- Use consistent spacing and sizing
- Follow the synthwave theme colors
- Ensure accessibility (contrast, no flashing)

### Theme Consistency

- Use colors from `ui/theme.rs`
- Maintain consistent visual hierarchy
- Test both light and dark themes
- Follow minimalist design principles

## Git Workflow

### Commit Messages

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Types:
- `feat`: New features
- `fix`: Bug fixes
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Test additions/changes
- `chore`: Build/tool changes

Example:
```
feat(game): add power cell nova effect

Implement nova effect that clears adjacent blocks when power cells
are included in line clears.

Closes #123
```

### Branch Naming

- Features: `feature/description`
- Bug fixes: `fix/description`
- Documentation: `docs/description`
- Releases: `release/v1.0.0`

## Issues and Bug Reports

### Bug Reports

Include:
- OS and version
- Rust version (`rustc --version`)
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if UI-related

### Feature Requests

Include:
- Clear description of the feature
- Use case and motivation
- Proposed implementation approach
- Consider backward compatibility

## Pull Request Guidelines

### Before Submitting

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Linter passes (`cargo clippy -- -D warnings`)
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated for significant changes

### PR Description

Include:
- Clear description of changes
- Link to related issues
- Screenshots for UI changes
- Breaking changes noted
- Testing performed

## Release Process

Releases follow semantic versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Create git tag: `git tag v1.0.0`
- [ ] Push tag: `git push origin v1.0.0`
- [ ] GitHub Actions will build and create release

## Questions and Support

- Open an issue for bugs or feature requests
- Use GitHub Discussions for questions
- Check existing issues before creating new ones

## License

By contributing to Tetris Synthwave, you agree that your contributions will be licensed under the MIT License.
