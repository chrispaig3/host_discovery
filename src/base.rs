use core::fmt::{Display, Formatter, Result};
use std::path::Path;
use std::fs;

#[cfg(target_os = "windows")]
use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

pub struct System;

pub trait LinuxSystem {
    fn get_distro(self) -> String;
    fn get_linux_version(self) -> String;
    fn is_subsystem_env(self) -> bool;
}

pub trait WindowsSystem {
    fn get_edition(self) -> String;
}

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

pub trait Parser {
    fn select(path: &'static str, env_var: &'static str, elem: char) -> String;
}

impl Parser for String {
    fn select(path: &'static str, env_var: &'static str, elem: char) -> String {
        let contents = fs::read_to_string(path).expect("Failed to read file");

        let capture = contents
            .lines()
            .find(|line| line.starts_with(env_var))
            .expect("Failed to find the specified environment variable")
            .split(elem)
            .nth(1)
            .expect("Failed to parse environment variable")
            .trim_matches('"')
            .to_string();
        capture
    }
}

impl LinuxSystem for System {
    fn get_distro(self) -> String {
        String::select("/etc/os-release", "NAME", '=')
    }

    fn get_linux_version(self) -> String {
        String::select("/etc/os-release", "VERSION_ID", '=')
    }
    
    fn is_subsystem_env(self) -> bool {
        Path::new("/proc/sys/fs/binfmt_misc/WSLInterop").exists()
    }
}

impl WindowsSystem for System {
    fn get_edition(self) -> String {
        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .expect("Failed to open subkey");
        
        let edition = subkey
            .get_value::<String, _>("EditionID")
            .expect("Failed to get value");
        edition
    }
}

// impl_display: Implements the Display trait for OperatingSystem and Architecture
macro_rules! impl_display {
    ($type:ident) => {
        impl Display for $type {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

impl_display!(OperatingSystem);
impl_display!(Architecture);