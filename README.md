# hello-world-rust-cli-homebrew

Hello world CLI. Releases are automated with release-plz and published to GitHub Releases (macOS arm64/x86_64, Linux x86_64/arm64) and the [PeterKneale/homebrew-tap](https://github.com/PeterKneale/homebrew-tap) Homebrew tap.

## Install

```sh
brew install PeterKneale/tap/hello-world-rust-cli-homebrew
```

or

```sh
brew tap PeterKneale/tap
brew install hello-world-rust-cli-homebrew
```

## Usage

```sh
hello-world-rust-cli-homebrew          # Hello, world!
hello-world-rust-cli-homebrew Peter    # Hello, Peter!
```

## Development

```sh
cargo test --all-targets   # unit + integration tests
cargo run -- Peter
```

Use conventional commits (`feat:`, `fix:`) so release-plz infers version bumps.
