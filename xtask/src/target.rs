use std::str::FromStr;

use anyhow::anyhow;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
	X86_64,
	X86_64Uefi,
	AArch64,
}

impl Target {
	pub fn name(&self) -> &'static str {
		match self {
			Self::X86_64 => "x86_64",
			Self::X86_64Uefi => "x86_64-uefi",
			Self::AArch64 => "aarch64",
		}
	}

	pub fn triple(&self) -> &'static str {
		match self {
			Self::X86_64 => "x86_64-unknown-none",
			Self::X86_64Uefi => "x86_64-unknown-uefi",
			Self::AArch64 => "aarch64-unknown-none-softfloat",
		}
	}

	pub fn cargo_args(&self) -> &'static [&'static str] {
		match self {
			Self::X86_64 => &["--target=x86_64-unknown-none"],
			Self::X86_64Uefi => &[
				"--target=x86_64-unknown-uefi",
				"-Zbuild-std=core,alloc,compiler_builtins",
				"-Zbuild-std-features=compiler-builtins-mem",
			],
			Self::AArch64 => &["--target=aarch64-unknown-none-softfloat"],
		}
	}

	pub fn rustflags(&self) -> &'static [&'static str] {
		match self {
			Self::X86_64 => &[
				"-Clink-arg=-Tsrc/arch/x86_64/link.ld",
				"-Crelocation-model=static",
			],
			Self::X86_64Uefi => &[],
			Self::AArch64 => &["-Clink-arg=-Tsrc/arch/aarch64/link.ld"],
		}
	}

	pub fn build_name(&self) -> &'static str {
		match self {
			Self::X86_64Uefi => "rusty-loader.efi",
			_ => "rusty-loader",
		}
	}

	pub fn dist_name(&self) -> &'static str {
		match self {
			Self::X86_64Uefi => "BootX64.efi",
			_ => "rusty-loader",
		}
	}
}

impl FromStr for Target {
	type Err = anyhow::Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"x86_64" => Ok(Self::X86_64),
			"x86_64-uefi" => Ok(Self::X86_64Uefi),
			"aarch64" => Ok(Self::AArch64),
			s => Err(anyhow!("Unsupported target: {s}")),
		}
	}
}
