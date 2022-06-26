# Usage

`os-release-rs` provide a Rust wrapper for `/etc/os-release` file.

It's easy to use:

```rust
use os_release_rs::OsRelease;

fn main() {
    let os_release = OsRelease::new().unwrap();
    println!("I use {} btw!", os_release.name);
}
```

OsRelease is a struct that contains all the informations from `/etc/os-release` file.

See the [documentation](https://docs.rs/os-release-rs) for a complete list of fields.

