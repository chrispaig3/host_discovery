# A simple Rust crate for host discovery

> Make more decisions based on the characteristics of the environment you are working in.

> Basic Usage

- API
  - ***fn*** `detect_os`: Returns a variant of OperatingSystem
  - ***fn*** `detect_arch`: Returns a variant of Architecture
  - ***fn*** `cpu_cores`: Returns a u32 representing the CPU core count
  - ***fn*** `cpu_model`: Returns a String containing the name of the CPU model
  - ***fn*** `detect_distro`: Returns a String containing the name of the running Linux distribution
  - ***fn*** `detect_distro_version`: Returns a String containing the version id of the running Linux distribution
  - ***fn*** `is_subsystem_env`: Returns a boolean based on whether the Linux environment is a Windows subsystem

- Add to your project
```sh 
    cargo add host_discovery
```

- Planned Features:
  - Detect Windows Edition
  - Detect Windows Version
      - Implements trait PartialProfile

