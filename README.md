# A simple Rust crate for host discovery

### Basic Usage
```rust
use host_discovery::{OSProfile, gpu};

fn main() {
    let profile = OSProfile::new().win_edition().build();
    let arch = profile.arch;
    let os = profile.os;
    let edition = profile.win_edition;
    let gpu = gpu().expect("Unreachable");

    println!(
        " Architecture: {}\n OS: {}\n Edition: {}\n GPU: {}", 
        arch,
        os,
        edition.unwrap(),
        gpu,
    )
}
```