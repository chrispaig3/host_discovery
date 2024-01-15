use core::fmt::{Display, Formatter, Result};
use std::fs;

pub struct Data;

pub struct LinuxSystem {
    pub distro: String,
    pub version_id: String,
}

pub struct WindowsSystem {
    pub edition: String,
    pub version: String,
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
    fn parse(path: &'static str, env_var: &'static str, elem: char) -> String;
}

impl Parser for Data {
    fn parse(path: &'static str, env_var: &'static str, elem: char) -> String {
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

pub trait PartialProfile {
    fn partial(self) -> Self;
    fn get_os_variant(self) -> String;
    fn get_version(self) -> String;
}

impl PartialProfile for LinuxSystem {
    fn partial(self) -> Self {
        Self {
            distro: self.distro,
            version_id: self.version_id,
        }
    }

    fn get_os_variant(self) -> String {
        self.distro
    }

    fn get_version(self) -> String {
        self.version_id
    }
}

impl PartialProfile for WindowsSystem {
    fn partial(self) -> Self {
        Self {
            edition: self.edition,
            version: self.version,
        }
    }

    fn get_os_variant(self) -> String { self.edition }

    fn get_version(self) -> String { self.version }
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
