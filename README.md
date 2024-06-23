> This library is a work in progress

# A simple Rust crate for host discovery

### Basic Usage
```rust
use host_discovery::{OSProfile, gpu};

fn main() {
    // linux example
    //let profile = OSProfile::new().linux_distro().build();
    
    // or
    //let profile = OSProfile::new().is_wsl().build();
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

> Current Features

- OS
- Architecture
- Windows Edition
- WSL Detection
- Linux Distro
- GPU Enumeration
