use std::{
    env::consts::{ARCH, OS},
    path::Path,
};

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod base;
use base::{
    Architecture,
    Data,
    Parser,
    LinuxSystem,
    WindowsSystem,
    OperatingSystem,
    PartialProfile
};

static SYS_META: [&str; 2] = [OS, ARCH];
static ENV_META: [&str; 2] = ["NAME=", "VERSION_ID="];

// Index Constants
const OS_: u8 = 0;
const ARCH_: u8 = 1;
const NAME: u8 = 0;
const VERSION_ID: u8 = 1;

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

/// Returns the CPU core count via `proc/cpuinfo`
pub fn cpu_cores() -> u32 {
    Data::parse("/proc/cpuinfo", "cpu cores", ':')
        .trim()
        .to_string()
        .parse()
        .expect("Failed to parse String to unsigned int")
}

/// Returns the CPU model name via `/proc/cpuinfo`
pub fn cpu_model() -> String {
    Data::parse("/proc/cpuinfo", "model name", ':')
        .trim()
        .to_string()
}

/// Returns the NAME of the Linux distribution via `/etc/os-release`
pub fn detect_distro() -> String {
    LinuxSystem::get_os_variant(linux_profile())
}

/// Returns the VERSION_ID of the Linux distribution via `/etc/os-release`
pub fn detect_distro_version() -> String {
    LinuxSystem::get_version(linux_profile())
}

/// Checks if the linux environment is a Windows subsystem
pub fn is_subsystem_env() -> bool {
    Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
}

/// lookup_windows_edition returns the EditionID via the registry
pub fn lookup_windows_edition() -> String {
    WindowsSystem::get_os_variant(windows_profile())
}

/// lookup_product_name returns the ProductName via the registry
pub fn lookup_product_name() -> String {
    WindowsSystem::get_version(windows_profile())
}

// linux_profile acts as a constructor for LinuxSystem
fn linux_profile() -> LinuxSystem {
    let profile = LinuxSystem {
        distro: Data::parse("/etc/os-release", ENV_META[NAME as usize], '='),
        version_id: Data::parse("/etc/os-release", ENV_META[VERSION_ID as usize], '='),
    };

    LinuxSystem::partial(profile)
}

fn windows_profile() -> WindowsSystem {
    let profile = WindowsSystem {
        edition:  RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .expect("Failed to open key")
            .get_value("EditionID")
            .expect("Failed to retrieve Windows edition"),
        version:  RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .expect("Failed to open key")
            .get_value("ProductName")
            .expect("Failed to retrieve Windows version"),
    };

    WindowsSystem::partial(profile)
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

    #[test]
    fn test_cpu_cores() {
        test_fn(cpu_cores, 8);
    }

    #[test]
    fn test_cpu_model() {
        test_fn(cpu_model, "AMD Ryzen 7 5700X 8-Core Processor".to_string());
    }

    #[test]
    fn test_detect_os() {
        test_fn(detect_os, OperatingSystem::Linux);
    }

    #[test]
    fn test_detect_arch() {
        test_fn(detect_arch, Architecture::X86_64);
    }

    #[test]
    fn test_detect_distro() {
        test_fn(detect_distro, "Fedora Linux".to_string());
    }

    #[test]
    fn test_detect_distro_version() {
        test_fn(detect_distro_version, "39".to_string());
    }

    #[test]
    fn test_is_subsystem_env() {
        test_fn(is_subsystem_env, false);
    }

    #[test]
    fn test_lookup_windows_edition() {
        test_fn(lookup_windows_edition, "Professional".to_string());
    }

    #[test]
    fn test_lookup_product_name() {
        test_fn(lookup_product_name, "Windows 10 Pro".to_string());
    }
}
