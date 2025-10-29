# Mola Render

## Build Instruction

### macOS

```bash
brew install direnv
brew install openssl@3
```

### General

```bash
# Install cargo-binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# Install dioxus-cli
cargo binstall dioxus-cli@0.7.0-alpha.1
# Install Dependencies
cargo install --path .
```
