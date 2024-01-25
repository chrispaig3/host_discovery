use std::env::consts::{ARCH, OS};

mod base;
use base::{Architecture, OperatingSystem, Parser};
pub use base::{LinuxSystem, System};

#[cfg(target_os = "windows")]
pub use base::WindowsSystem;

static SYS_META: [&str; 2] = [OS, ARCH];
// Index Constants
const OS_: u8 = 0;
const ARCH_: u8 = 1;

/// Returns the name of the host operating system; matches against `std::env::consts::OS`
pub fn detect_os() -> OperatingSystem {
    let os = match SYS_META[OS_ as usize] {
        "linux" => OperatingSystem::Linux,
        "android" => OperatingSystem::Android,
        "freebsd" => OperatingSystem::FreeBSD,
        "dragonfly" => OperatingSystem::DragonFlyBSD,
        "netbsd" => OperatingSystem::NetBSD,
        "openbsd" => OperatingSystem::OpenBSD,
        "solaris" => OperatingSystem::Solaris,
        "macos" => OperatingSystem::MacOS,
        "windows" => OperatingSystem::Windows,
        _ => OperatingSystem::Unknown,
    };
    os
}

/// Returns the architecture of the host operating system; matches against `std::env::consts::ARCH`
pub fn detect_arch() -> Architecture {
    let arch = match SYS_META[ARCH_ as usize] {
        "x86" => Architecture::X86,
        "x86_64" => Architecture::X86_64,
        "arm" => Architecture::Arm,
        "aarch64" => Architecture::Aarch64,
        "loongarch64" => Architecture::Loongarch64,
        "m68k" => Architecture::M68k,
        "csky" => Architecture::Csky,
        "mips" => Architecture::Mips,
        "mips64" => Architecture::Mips64,
        "powerpc" => Architecture::Powerpc,
        "powerpc64" => Architecture::Powerpc64,
        "riscv64" => Architecture::Riscv64,
        "s390x" => Architecture::S390x,
        "sparc64" => Architecture::Sparc64,
        _ => Architecture::Unknown,
    };
    arch
}

/// Returns the CPU core count via `/proc/cpuinfo`
pub fn cpuinfo_cores() -> u32 {
    String::select("/proc/cpuinfo", "cpu cores", ':')
        .trim()
        .parse::<u32>()
        .expect("Failed to parse String to unsigned int")
}

/// Returns the CPU model name via `/proc/cpuinfo`
pub fn cpuinfo_model() -> String {
    String::select("/proc/cpuinfo", "model name", ':')
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    // Note(may not be immediately obvious): Test depends on my local environment for 100% pass.
    // Why? Testing for the correct behavior, not necessarily the value...
    // i.e., if the test fails on your machine, it is not inherently a bug.
    // To prevent unnecessary Tickets, read the error message beforehand.
    use super::*;

    // Generic test function, keeps tests DRY
    fn test_fn<T: PartialEq + std::fmt::Debug + std::fmt::Display>(f: fn() -> T, expected: T) {
        assert_eq!(f(), expected);
        println!("Result: {}", f());
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cpuinfo_cores() {
        test_fn(cpuinfo_cores, 8);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cpuinfo_model() {
        test_fn(cpu_model, "AMD Ryzen 7 5700X 8-Core Processor".to_string());
    }

    #[test]
    fn test_detect_os() {
        test_fn(detect_os, OperatingSystem::Windows);
    }

    #[test]
    fn test_detect_arch() {
        test_fn(detect_arch, Architecture::X86_64);
    }   
}