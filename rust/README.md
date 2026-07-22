# Key-Speed Notepad Logger (Rust)

A high-performance, cross-platform **Key-Speed Notepad Logger** written in Rust using `eframe`/`egui`. It provides real-time keystroke latency measurement down to millisecond precision, statistics tracking, and dark mode UI parity with Python Tkinter and Web JS implementations.

## Features

- ⚡ **Millisecond Precision Latency**: Measures exact interval elapsed between consecutive keystrokes using Rust `std::time::Instant`.
- 📊 **Real-time Statistics**:
  - **Keys Typed**: Total keypress counter.
  - **Avg Latency**: Color-coded keypress interval indicator:
    - 🟢 `< 250 ms`: Fast
    - 🟡 `250ms - 600ms`: Medium
    - 🔴 `> 600ms`: Slow
- ⏱️ **Auto Reset**: Automatically clears current log and starts a new session if typing pauses for 1.5 seconds (1500 ms) or longer.
- 📝 **Live Scroll Keystroke Log**: Displays timestamps `[HH:MM:SS.mmm]`, key representation (`'a'`, `Space`, `Enter`, `Backspace`, `Tab`, `<KeyName>`), and latency (`+Xms` or `First`).
- 📁 **Export CSV**: Export logged keystrokes to a `.csv` file via native file dialog (`rfd`).
- 🧹 **Clear Log**: One-click log reset.
- 🪟 **Cross-Platform**: Runs natively on both **Windows** (`x86_64-pc-windows-msvc`) and **Linux** (`x86_64-unknown-linux-gnu`).

## Building & Running

### Requirements
- Rust toolchain (`rustc` & `cargo` 1.75+)

### Running Locally
```bash
cd rust
cargo run --release
```

### Building for Windows
```bash
# Native Windows build
cargo build --release --target x86_64-pc-windows-msvc
```
Executable location: `target/x86_64-pc-windows-msvc/release/key_speed_logger.exe`

### Building for Linux
```bash
# Native Linux build or Cross-compilation target
cargo build --release --target x86_64-unknown-linux-gnu
```
Executable location: `target/x86_64-unknown-linux-gnu/release/key_speed_logger`

## Project Structure
- `src/main.rs`: Application entry point and window setup.
- `src/app.rs`: GUI rendering, keystroke event handling, latency calculations, and CSV export.
- `src/platform/mod.rs`: Platform abstraction layer.
- `src/platform/windows.rs`: Windows-specific platform module.
- `src/platform/linux.rs`: Linux-specific platform module.
