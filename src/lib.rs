use core::fmt::{Display, Formatter, Result};
use std::env::consts::{ARCH, OS};
use std::fs;
use std::path::Path;

use num_cpus;
use wgpu::{Backends, Instance};

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

mod impl_display;

pub struct Environment;

#[derive(Debug, PartialEq)]
pub enum OperatingSystem {
    Linux,
    Android,
    FreeBSD,
    DragonFlyBSD,
    NetBSD,
    OpenBSD,
    Solaris,
    MacOS,
    Windows,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum Architecture {
    X86,
    X86_64,
    Arm,
    Aarch64,
    Loongarch64,
    M68k,
    Csky,
    Mips,
    Mips64,
    Powerpc,
    Powerpc64,
    Riscv64,
    S390x,
    Sparc64,
    Unknown,
}

pub trait LinuxSystem {
    fn is_subsystem_env(&self) -> bool;
    fn get_distro(&self) -> String;
    fn get_platform_id(&self) -> String;
    fn get_cpe_name(&self) -> String;
    fn get_hostname(&self) -> String;
    fn cpuinfo_model(&self) -> String;
}

#[cfg(target_os = "windows")]
pub trait WindowsSystem {
    fn get_edition(&self) -> String;
    fn get_cpu_model(&self) -> String;
}

pub trait CrossPlatform {
    fn get_os(&self) -> OperatingSystem;
    fn get_arch(&self) -> Architecture;
    fn get_cpu_cores(&self) -> u32;
    fn get_public_ip(&self) -> String;
    fn get_gpu_model(&self) -> Option<String>;
}

trait Parser {
    fn select(path: &'static str, text: &'static str, elem: char) -> String;
}

impl Parser for String {
    // select: takes a path, some text to match against, and a character (element) to split the line by
    fn select(path: &'static str, text: &'static str, elem: char) -> String {
        let contents = fs::read_to_string(path).expect("Failed to read file");

        let capture = contents
            .lines()
            .find(|line| line.starts_with(text))
            .expect("Failed to find the specified text")
            .split(elem)
            .nth(1)
            .expect("Failed to parse environment variable")
            .trim_matches('"')
            .to_string();
        capture
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {}
    }
}

impl LinuxSystem for Environment {
    /// is_subsystem_env: Returns true if the environment is a Windows Subsystem for Linux
    fn is_subsystem_env(&self) -> bool {
        Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
    }

    /// get_distro: Returns the name of the Linux distribution
    fn get_distro(&self) -> String {
        String::select("/etc/os-release", "NAME", '=')
    }

    /// get_platform_id: Returns the platform id
    fn get_platform_id(&self) -> String {
        String::select("/etc/os-release", "PLATFORM_ID", '=')
    }

    /// get_cpe_name: Returns the Common Platform Enum Name
    fn get_cpe_name(&self) -> String {
        String::select("/etc/os-release", "CPE_NAME", '=')
    }

    /// get_hostname: Returns the hostname of the system
    fn get_hostname(&self) -> String {
        let hostname = fs::read_to_string("/etc/hostname").expect("Failed to retrieve hostname");

        hostname.trim().to_string()
    }

    /// cpuinfo_model: Returns the model of the CPU
    fn cpuinfo_model(&self) -> String {
        String::select("/proc/cpuinfo", "model name", ':')
            .trim()
            .to_string()
    }
}

#[cfg(target_os = "windows")]
impl WindowsSystem for Environment {
    /// get_edition: Returns the edition of Windows
    fn get_edition(&self) -> String {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .expect("Failed to open subkey");

        let edition = subkey
            .get_value::<String, _>("EditionID")
            .expect("Failed to get value");
        edition
    }

    /// get_cpu_model: Returns the model of the CPU
    fn get_cpu_model(&self) -> String {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm
            .open_subkey("HARDWARE\\DESCRIPTION\\System\\CentralProcessor\\0")
            .expect("Failed to open subkey");

        let model = subkey
            .get_value::<String, _>("ProcessorNameString")
            .expect("Failed to get value");
        model.trim().to_string()
    }

}

impl CrossPlatform for Environment {
    /// get_os: Returns the Operating System
    fn get_os(&self) -> OperatingSystem {
        match OS {
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
        }
    }
    
    /// get_arch: Returns the CPU Architecture
    fn get_arch(&self) -> Architecture {
        match ARCH {
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
        }
    }

    /// get_cpu_cores: Returns the number of cores on the CPU
    fn get_cpu_cores(&self) -> u32 {
        let cores = num_cpus::get_physical();
        cores as u32
    }
    
    /// get_gpu_model: Returns the model name of the GPU
    fn get_gpu_model(&self) -> Option<String> {
        let instance = Instance::default();
        let adapters = |all| instance.enumerate_adapters(all);
        
        for adapter in adapters(Backends::all()) {
            let name = adapter.get_info().name;
            return Some(name);
        }
        None
    }
    
    /// get_public_ip: Returns the public IP address of the host machine
    fn get_public_ip(&self) -> String {
        let ip = reqwest::blocking::get("https://api.ipify.org")
        .expect("Failed to get public IP")
        .text()
        .expect("Failed to parse response");
        ip
    }
}

impl_display!(OperatingSystem);
impl_display!(Architecture);

#[cfg(test)]
mod tests {
    // Note(may not be immediately obvious): Test depends on my local environment for 100% pass.
    // Why? Testing for the correct behavior, not necessarily the value...
    // i.e., if the test fails on your machine, it is not inherently a bug.
    // To prevent unnecessary Tickets, read the error message beforehand.
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_distro() {
        let distro = Environment.get_distro();
        assert_eq!(distro, "Fedora Linux");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_cpe_name() {
        let cpe_name = Environment.get_cpe_name();
        assert_eq!(cpe_name, "cpe:/o:fedoraproject:fedora:39")
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_hostname() {
        let hostname = Environment.get_hostname();
        assert_eq!(hostname, "TheLab");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_get_platform_id() {
        let platform_id = Environment.get_platform_id();
        assert_eq!(platform_id, "platform:f39");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_is_subsystem_env() {
        let is_subsystem = Environment.is_subsystem_env();
        assert_eq!(is_subsystem, false);
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_cpuinfo_model() {
        let model = Environment.cpuinfo_model();
        assert_eq!(model, "AMD Ryzen 7 5700X 8-Core Processor");
    }

    #[test]
    fn test_get_gpu_model() {
        let gpu_model = Environment.get_gpu_model();
        assert_eq!(Some("NVIDIA GeForce RTX 3070 Ti".to_string()), gpu_model);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_edition() {
        let edition = Environment.get_edition();
        assert_eq!(edition, "Professional");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_get_cpu_model() {
        let model = Environment.get_cpu_model();
        assert_eq!(model, "AMD Ryzen 7 5700X 8-Core Processor");
    }

    #[test]
    fn test_get_cpu_cores() {
        let cores = Environment.get_cpu_cores();
        assert_eq!(cores, 8);
    }

    #[test]
    fn test_get_os() {
        let os = Environment.get_os();
        assert_eq!(os, OperatingSystem::Linux);
    }

    #[test]
    fn test_get_arch() {
        let arch = Environment.get_arch();
        assert_eq!(arch, Architecture::X86_64);
    }

    #[test]
    fn test_get_public_ip() {
        let ip = Environment.get_public_ip();
        assert_eq!(ip, "");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_select() {
        let version_id = String::select("/etc/os-release", "VERSION_ID", '=');
        assert_eq!(version_id, "39");
    }
}
