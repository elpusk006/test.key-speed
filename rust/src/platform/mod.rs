//! Cross-platform module supporting both Windows and Linux targets.

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn get_platform_name() -> &'static str {
    "Other OS"
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn csv_line_ending() -> &'static str {
    "\n"
}
