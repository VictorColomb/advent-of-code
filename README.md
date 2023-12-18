# Advent of Code 🎄🫎

## ⏯️ Usage

```bash
cargo scaffold <day> [-d|--download]
cargo download <day>
cargo solve <day>
```

During an Advent of Code event, the `day` parameter can be omitted and will default to the current day.

The event year in read from the `AOC_YEAR` environment variable (see `.cargo/config.toml`). It can also be changed on the command line:

```bash
cargo run --release --quiet 2021 [scaffold|solve|...]
```

## 🍪 Session cookie

The [aoc-client](https://crates.io/crates/aoc-client) crate is used underneath to download input files. If found, it is fed with the contents of the `.adventofcode.session` file at the root of the repo.

See the [aoc-cli documentation](https://crates.io/crates/aoc-cli#user-content-session-cookie-) for other authentication methods.

## ⚙️ Development

### 1. Pre-commit hooks

```bash
pip install pre-commit
pre-commit install
```

### 2. Editor Config

For VS Code, install the [Editor Config extension](https://marketplace.visualstudio.com/items?itemName=EditorConfig.EditorConfig).

## ⚖️ License

This repository is licensed under the MIT license. Even though you should try to solve AoC puzzles on your own, feel free to use this as you please.

However, the puzzles and input files found and downloaded at adventofcode.com are under copyright. See <https://adventofcode.com/about#legal>.
