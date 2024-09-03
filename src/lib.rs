use raw_cpuid::{CpuId, ProcessorBrandString};
use std::env::consts::{ARCH, OS};
#[cfg(target_os = "linux")]
use std::fs;
#[cfg(target_os = "linux")]
use std::path::Path;
use wgpu::{Backends, Instance};
#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

#[derive(Debug)]
pub struct OSProfile {
    pub os: &'static str,
    pub arch: &'static str,
    pub win_edition: Option<String>,
    pub is_wsl: Option<bool>,
    pub linux_distro: Option<String>,
}

#[derive(Debug)]
pub struct Processor {
    pub model: ProcessorBrandString,
    pub cores: u32,
}

#[derive(Debug)]
pub struct GPU {
    pub model: String,
    pub driver_version: String,
}

impl OSProfile {
    pub fn new() -> Self {
        Self {
            os: OS,
            arch: ARCH,
            win_edition: None,
            is_wsl: None,
            linux_distro: None,
        }
    }

    /// Returns the Windows edition if a Windows system is available
    #[cfg(target_os = "windows")]
    pub fn win_edition(mut self) -> Self {
        let sub_key = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";
        let reg = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey(sub_key)
            .expect("Failed to open registry key");
        let edition: String = reg
            .get_value("EditionID")
            .expect("Failed to get Windows edition from registry");

        self.win_edition = Some(edition);
        self
    }

    /// Returns the Linux distro if a Linux system is available
    #[cfg(target_os = "linux")]
    pub fn linux_distro(mut self) -> Self {
        let text = fs::read_to_string("/etc/os-release").expect("Failed to read /etc/os-release");
        let tokens = text.split("\n").collect::<Vec<&str>>();
        let pretty_name = tokens
            .iter()
            .filter(|line| line.contains("PRETTY_NAME"))
            .collect::<Vec<&&str>>();

        let distro = pretty_name[0].split("=").collect::<Vec<&str>>()[1].replace("\"", "");
        self.linux_distro = Some(distro);
        self
    }

    /// Returns true if the Linux host is running on WSL
    #[cfg(target_os = "linux")]
    pub fn is_wsl(mut self) -> Self {
        let path = Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists();
        self.is_wsl = Some(path);
        self
    }

    pub fn build(self) -> Self {
        Self {
            os: self.os,
            arch: self.arch,
            win_edition: self.win_edition,
            is_wsl: self.is_wsl,
            linux_distro: self.linux_distro,
        }
    }
}

/// Returns a `Processor` object containing the CPU model and logical core count  (x86 only)
pub fn cpu() -> Processor {
    let cpuid = CpuId::new();
    let brand = cpuid.get_processor_brand_string().expect("Unsupported CPU");
    let cores = cpuid
        .get_processor_capacity_feature_info()
        .expect("Failed to retrieve proc cap info");

    let cpu = Processor {
        model: brand,
        cores: cores.maximum_logical_processors() as u32,
    };
    cpu
}

/// Returns a `GPU` object containing the GPU model and driver version
pub fn gpu() -> Option<GPU> {
    let instance = Instance::default();

    for adapter in instance.enumerate_adapters(Backends::all()) {
        let info = adapter.get_info();
        let gpu = GPU {
            model: info.name,
            driver_version: info.driver_info,
        };
        return Some(gpu);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile() {
        let profile = OSProfile::new().build();
        assert_eq!(profile.os, OS);
        assert_eq!(profile.arch, ARCH);
    }

    #[test]
    fn test_distro() {
        let distro = OSProfile::new().linux_distro().build();
        assert!(distro.linux_distro.unwrap().starts_with("Fedora"));
    }

    #[test]
    fn test_wsl() {
        let wsl = OSProfile::new().is_wsl().build();
        assert_eq!(wsl.is_wsl, Some(false));
    }

    #[test]
    fn test_cpu() {
        let cpu = cpu();
        assert!(cpu.model.as_str().starts_with("AMD"));
        assert_eq!(cpu.cores, 16);
    }

    #[test]
    fn test_gpu() {
        let gpu = gpu();
        assert!(gpu.is_some());
        println!("{:?}", gpu.unwrap().driver_version);
    }
}
