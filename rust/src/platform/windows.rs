//! Windows-specific functionality and platform details.

pub fn get_platform_name() -> &'static str {
    "Windows"
}

/// Formats CSV line ending for Windows (CRLF)
#[allow(dead_code)]
pub fn csv_line_ending() -> &'static str {
    "\r\n"
}
