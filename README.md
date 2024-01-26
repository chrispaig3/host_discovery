# A simple Rust crate for host discovery

> Make more decisions based on the characteristics of the environment you are working in.

- Basic Usage

```rust
use host_discovery::{Environment, CrossPlatform};

fn main() {
    let env = Environment;
    let os = env.get_os();
    let arch = env.get_arch();
    println!("OS: {}, Arch: {}", os, arch);
}
```

- API Methods
> ***trait*** `CrossPlatform`
  
  - ***fn*** `get_os`: Returns a variant of OperatingSystem
  - ***fn*** `get_arch`: Returns a variant of Architecture
> ***trait*** `LinuxSystem`

  - ***fn*** `cpuinfo_cores`: Returns a u32 representing the CPU core count
  - ***fn*** `cpuinfo_model`: Returns a String containing the name of the CPU model
  - ***fn*** `get_distro`: Returns a String containing the name of the running Linux distribution
  - ***fn*** `is_subsystem_env`: Returns a boolean based on whether the Linux environment is a Windows subsystem
> ***trait*** `WindowsSystem`

  - ***fn*** `get_edition` Returns a String containing the Windows edition via the registry
    
### Add to your project
```sh 
    cargo add host_discovery
```

- Planned Features
  - GPU enumeration & extended device detection (CPU, etc.).

