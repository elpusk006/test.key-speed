//! Linux-specific functionality and platform details.

pub fn get_platform_name() -> &'static str {
    "Linux"
}

/// Formats CSV line ending for Linux (LF)
#[allow(dead_code)]
pub fn csv_line_ending() -> &'static str {
    "\n"
}
