#[cfg(target_arch = "x86")]
mod x86;
#[cfg(target_arch = "x86_64")] 
mod x64;
#[cfg(target_arch = "arm")]
mod arm;
#[cfg(target_arch = "aarch64")]
mod arm64;

#[cfg(target_arch = "x86")]
pub use x86::*;
#[cfg(target_arch = "x86_64")] 
pub use x64::*;
#[cfg(target_arch = "arm")]
pub use arm::*;
#[cfg(target_arch = "aarch64")]
pub use arm64::*;