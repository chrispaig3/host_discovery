use std::env::consts::{ARCH, OS};
use wgpu::{Backends, Instance};

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

pub struct OSInfo {
    pub os: &'static str,
    pub win_edition: Option<String>,
    pub linux_distro: Option<String>,
    pub arch: &'static str,
}

pub struct OSProfile {
    pub os: &'static str,
    pub win_edition: Option<String>,
    pub linux_distro: Option<String>,
    pub arch: &'static str,
}

impl OSProfile {
    pub fn new() -> Self {
        Self {
            os: OS,
            win_edition: None,
            linux_distro: None,
            arch: ARCH,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn win_edition(mut self) -> Self {
        let sub_key = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion";
        let reg = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(sub_key).expect("Failed to open registry key");
        let edition: String = reg.get_value("EditionID").expect("Failed to get Windows edition from registry");
        self.win_edition = Some(edition);
        self
    }

    pub fn build(self) -> OSInfo {
        OSInfo {
            os: self.os,
            arch: self.arch,
            win_edition: self.win_edition,
            linux_distro: self.linux_distro,
        }
    }
}

pub fn gpu() -> Option<String> {
    let instance = Instance::default();
    
    for adapter in instance.enumerate_adapters(Backends::all()) {
        let name = adapter.get_info().name;
        return Some(name);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sys_profile() {
        let sys_profile = OSProfile::new().build();
        assert_eq!(sys_profile.os, OS);
        assert_eq!(sys_profile.arch, ARCH);

        println!("OS: {}", sys_profile.os);
        println!("Arch: {}", sys_profile.arch);
    }

    #[test]
    fn test_gpu() {
        let gpu = gpu();
        assert!(gpu.is_some());
        println!("GPU: {}", gpu.unwrap());
    }
}