# pose3 [![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/pose3.svg
[crates.io]: https://crates.io/crates/pose3

## What is it?

Rust data types for (de)serializing pose settings from pose3.json files

## How do I obtain this majestic tool?

Run the following Cargo command in your project directory (assuming you have [cargo-edit](https://github.com/killercup/cargo-edit) installed):

```fish
cargo add pose3
```

Or add the following line to your `Cargo.toml` (in the `[dependencies]` array):

```toml
pose3 = "^ 0.2"
```

## How do I use it?

```rust
use pose3::Pose3;

fn main() {
  let json = std::fs::read_to_string("./path/to/some.pose3.json").unwrap();
  let pose3: Pose3 = serde_json::from_str(&json).unwrap();
  println!("{pose3:#?}");
}
```

## How was this made?

Using the discovery process for undocumented JSON formats described [here](https://gist.github.com/colstrom/44b30fdddc8b0a9bfb44b09972a68676).

## License

`pose3` is available under the MIT License. See `LICENSE.txt` for the full text.

While the license is short, it's still written in fancy lawyer-speak. If you
prefer more down-to-earth language, consider the following:

- tl;drLegal has a simple visual summary available [here](https://www.tldrlegal.com/license/mit-license).
- FOSSA has a more in-depth overview available [here](https://fossa.com/blog/open-source-licenses-101-mit-license/).
