# `os-release-rs`

Rust wrapper for `/etc/os-release` file.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
os-release-rs = "0.1.0"
```

## Usage

```rust
use os_release_rs::OsRelease;

fn main() {
    let os_release = OsRelease::new().unwrap();
    println!("I use {} btw!", os_release.name);
}
```

## License

This crate is under the GNU General Public License v3.0.

See [LICENSE.md](LICENSE.md) for more information.

## Contributing

Open an issue or pull request to add or improve a feature.

