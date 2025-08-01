~~#This repository is no longer maintained.~~ A full re-write is in the works!

This crate might be a better choice: [Whoami](https://github.com/ardaku/whoami)

### Basic Usage
```rust
use host_discovery::{gpu, x86_cpu, OSProfile};

fn main() {
    let profile = OSProfile::new().computer_name().win_edition().build();

    let os = profile.os;
    let arch = profile.arch;
    let win_edition = profile.win_edition.unwrap();
    let computer_name = profile.computer_name.unwrap();
    println!(
        "Profile: [OS: {}, arch: {}, edition: {}, computer name: {}]",
        os, arch, win_edition, computer_name
    );
    println!("GPU: {}", gpu().unwrap());
    println!("CPU: {}", x86_cpu());
}
```

```
Console Output:

Profile: [OS: windows, arch: x86_64, edition: Professional, computer name: WORK]
GPU: GraphicsCard { model: "NVIDIA GeForce RTX 3070 Ti", driver_version: "566.36" }
CPU: Processor { model: ProcessorBrandString { as_str: "AMD Ryzen 7 5700X 8-Core Processor" }, cores: 16 }
```

#### V3 Roadmap
- Migrate from raw_cpuid to custom solution to support more CPU architectures.

> Current Features

- OS
- Architecture
- Hostname
- Windows Edition
- WSL Detection
- Linux Distro
- GPU Enumeration
- CPU Detection (x86_64, aarch64 - macOS Only)
