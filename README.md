# A simple Rust crate for host discovery

> Make more decisions based on the characteristics of the environment you are working in.

- Basic Usage

```rust
use host_discovery::{Environment, WindowsSystem, CrossPlatform};

fn main() {
    let edition = Environment.get_edition();
    let os = Environment.get_os();
    let arch = Environment.get_arch();

    println!("OS: {}\n Edition: {}\nArch: {}", os, edition, arch);
}
```

- API Methods
  - ***fn*** `get_os`: Returns a variant of OperatingSystem
  - ***fn*** `get_arch`: Returns a variant of Architecture
  - ***fn*** `cpuinfo_cores`: Returns a u32 representing the CPU core count
  - ***fn*** `cpuinfo_model`: Returns a String containing the name of the CPU model
  - ***fn*** `get_distro`: Returns a String containing the name of the running Linux distribution
  - ***fn*** `get_version`: Returns a String containing the version id of the running Linux distribution
  - ***fn*** `is_subsystem_env`: Returns a boolean based on whether the Linux environment is a Windows subsystem
  - ***fn*** `get_edition` Returns a String containing the Windows edition via the registry

### Add to your project
```sh 
    cargo add host_discovery
```

- Planned Features
  - GPU enumeration & extended device detection (CPU, etc.).

