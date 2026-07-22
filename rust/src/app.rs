use std::time::Instant;
use chrono::Local;
use eframe::egui::{self, Color32, FontId, RichText, ScrollArea};
use crate::platform;

// Modern Dark Theme Colors matching Python & JS apps
const BG_DARK: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x1e);       // Main editor background
const BG_DARKER: Color32 = Color32::from_rgb(0x18, 0x18, 0x18);     // Log panel background
const BG_HEADER: Color32 = Color32::from_rgb(0x25, 0x25, 0x26);     // Header & stats background
const FG_LIGHT: Color32 = Color32::from_rgb(0xe3, 0xe3, 0xe3);      // Main text
const FG_MUTED: Color32 = Color32::from_rgb(0x85, 0x85, 0x85);      // Secondary labels

const ACCENT_BLUE: Color32 = Color32::from_rgb(0x4f, 0xc3, 0xf7);    // Timestamps / Stats (Teal Blue)
const ACCENT_ORANGE: Color32 = Color32::from_rgb(0xff, 0xb7, 0x4d);  // Key names (Orange)
const ACCENT_GREEN: Color32 = Color32::from_rgb(0x81, 0xc7, 0x84);   // Fast latency <250ms
const ACCENT_YELLOW: Color32 = Color32::from_rgb(0xd4, 0xe1, 0x57);  // Medium latency 250-600ms
const ACCENT_RED: Color32 = Color32::from_rgb(0xef, 0x53, 0x50);     // Slow latency >600ms

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub key: String,
    pub latency_ms: Option<u64>,
    pub latency_str: String,
}

pub struct KeySpeedApp {
    // Editor State
    editor_text: String,
    
    // Keystroke Timing & Log State
    last_keypress_time: Option<Instant>,
    logs: Vec<LogEntry>,
    total_keys: usize,
    total_latency_ms: f64,
    latency_count: usize,
    
    // UI state
    scroll_to_bottom: bool,
    request_focus_editor: bool,
    status_message: Option<(String, Instant)>,
}

impl Default for KeySpeedApp {
    fn default() -> Self {
        Self {
            editor_text: String::new(),
            last_keypress_time: None,
            logs: Vec::new(),
            total_keys: 0,
            total_latency_ms: 0.0,
            latency_count: 0,
            scroll_to_bottom: false,
            request_focus_editor: true,
            status_message: None,
        }
    }
}

impl KeySpeedApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Calculates current stats
    pub fn avg_latency(&self) -> f64 {
        if self.latency_count == 0 {
            0.0
        } else {
            self.total_latency_ms / (self.latency_count as f64)
        }
    }

    pub fn char_count(&self) -> usize {
        self.editor_text.chars().count()
    }

    pub fn word_count(&self) -> usize {
        self.editor_text.split_whitespace().count()
    }

    /// Reset log and statistics
    pub fn clear_log(&mut self) {
        self.logs.clear();
        self.total_keys = 0;
        self.total_latency_ms = 0.0;
        self.latency_count = 0;
        self.last_keypress_time = None;
    }

    /// Clear both the editor text and the keystroke log
    pub fn clear_all(&mut self) {
        self.editor_text.clear();
        self.clear_log();
        self.request_focus_editor = true;
    }

    /// Records a key press with millisecond latency calculations
    pub fn record_keypress(&mut self, key_display: String) {
        let now_instant = Instant::now();
        let timestamp = Local::now().format("%H:%M:%S.%3f").to_string();

        let (latency_ms, latency_str) = match self.last_keypress_time {
            Some(last) => {
                let diff_ms = now_instant.duration_since(last).as_secs_f64() * 1000.0;
                if diff_ms >= 1500.0 {
                    // Reset log if gap >= 1.5 seconds (matching Python/JS logic)
                    self.clear_log();
                    (None, "First".to_string())
                } else {
                    let ms_u64 = diff_ms as u64;
                    self.total_latency_ms += diff_ms;
                    self.latency_count += 1;
                    (Some(ms_u64), format!("+{}ms", ms_u64))
                }
            }
            None => (None, "First".to_string()),
        };

        self.last_keypress_time = Some(now_instant);
        self.total_keys += 1;

        self.logs.push(LogEntry {
            timestamp,
            key: key_display,
            latency_ms,
            latency_str,
        });

        self.scroll_to_bottom = true;
    }

    /// Export logged keystrokes to CSV
    pub fn export_csv(&mut self) {
        if self.logs.is_empty() {
            self.status_message = Some(("Log is empty. Nothing to export.".to_string(), Instant::now()));
            return;
        }

        let file_dialog = rfd::FileDialog::new()
            .set_title("Export Keystroke Log to CSV")
            .add_filter("CSV File", &["csv"])
            .set_file_name("keystroke_log.csv");

        if let Some(path) = file_dialog.save_file() {
            let mut wtr = match csv::Writer::from_path(&path) {
                Ok(w) => w,
                Err(err) => {
                    self.status_message = Some((format!("Failed to export CSV: {}", err), Instant::now()));
                    return;
                }
            };

            // Write header
            if wtr.write_record(&["timestamp", "key", "latency_ms"]).is_ok() {
                for entry in &self.logs {
                    let lat_str = entry.latency_ms.map_or("".to_string(), |v| v.to_string());
                    let _ = wtr.write_record(&[&entry.timestamp, &entry.key, &lat_str]);
                }
                let _ = wtr.flush();
                self.status_message = Some((format!("Exported to {}", path.display()), Instant::now()));
            }
        }
    }
}

impl eframe::App for KeySpeedApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Dark theme customization
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = BG_DARK;
        visuals.window_fill = BG_DARK;
        ctx.set_visuals(visuals);

        // Process raw input events to capture precise keystroke events
        let mut key_events_to_record = Vec::new();
        ctx.input(|i| {
            for event in &i.raw.events {
                match event {
                    egui::Event::Key { key, pressed: true, .. } => {
                        let key_str = match key {
                            egui::Key::Space => Some("Space".to_string()),
                            egui::Key::Enter => Some("Enter".to_string()),
                            egui::Key::Backspace => Some("Backspace".to_string()),
                            egui::Key::Tab => Some("Tab".to_string()),
                            egui::Key::Escape => Some("<Escape>".to_string()),
                            egui::Key::Delete => Some("<Delete>".to_string()),
                            egui::Key::ArrowUp => Some("<Up>".to_string()),
                            egui::Key::ArrowDown => Some("<Down>".to_string()),
                            egui::Key::ArrowLeft => Some("<Left>".to_string()),
                            egui::Key::ArrowRight => Some("<Right>".to_string()),
                            _ => None, // Alphabetical / symbol keys are handled by Event::Text
                        };
                        if let Some(name) = key_str {
                            key_events_to_record.push(name);
                        }
                    }
                    egui::Event::Text(text) => {
                        for ch in text.chars() {
                            if ch == ' ' {
                                // Handled in Key::Space above if triggered
                                continue;
                            }
                            if ch.is_ascii_graphic() || !ch.is_control() {
                                key_events_to_record.push(format!("'{}'", ch));
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        for key_name in key_events_to_record {
            self.record_keypress(key_name);
        }

        // Top Header Banner (showing platform OS target)
        egui::TopBottomPanel::top("header_panel")
            .exact_height(40.0)
            .frame(egui::Frame::none().fill(BG_HEADER))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(12.0);
                    ui.label(
                        RichText::new("Key-Speed Notepad Logger")
                            .font(FontId::proportional(15.0))
                            .strong()
                            .color(FG_LIGHT),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(12.0);
                        let plat_info = format!("Target: {}", platform::get_platform_name());
                        ui.label(
                            RichText::new(plat_info)
                                .font(FontId::monospace(11.0))
                                .color(ACCENT_BLUE),
                        );
                    });
                });
            });

        // Split Main Pane: Left (Editor), Right (Logs & Stats)
        egui::SidePanel::right("right_panel")
            .min_width(360.0)
            .default_width(420.0)
            .frame(egui::Frame::none().fill(BG_DARKER))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Stats Header Area
                    egui::Frame::none()
                        .fill(BG_HEADER)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.columns(2, |cols| {
                                // Stat 1: Keys Typed
                                cols[0].vertical(|ui| {
                                    ui.label(
                                        RichText::new("KEYS TYPED")
                                            .font(FontId::proportional(10.0))
                                            .strong()
                                            .color(FG_MUTED),
                                    );
                                    ui.label(
                                        RichText::new(format!("{}", self.total_keys))
                                            .font(FontId::proportional(20.0))
                                            .strong()
                                            .color(ACCENT_BLUE),
                                    );
                                });

                                // Stat 2: Avg Latency
                                cols[1].vertical(|ui| {
                                    ui.label(
                                        RichText::new("AVG LATENCY")
                                            .font(FontId::proportional(10.0))
                                            .strong()
                                            .color(FG_MUTED),
                                    );
                                    let avg = self.avg_latency();
                                    let lat_color = if avg < 250.0 {
                                        ACCENT_GREEN
                                    } else if avg < 600.0 {
                                        ACCENT_YELLOW
                                    } else {
                                        ACCENT_RED
                                    };
                                    ui.label(
                                        RichText::new(format!("{:.0} ms", avg))
                                            .font(FontId::proportional(20.0))
                                            .strong()
                                            .color(lat_color),
                                    );
                                });
                            });
                        });

                    ui.add_space(6.0);

                    // Log Section Header
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("Keystroke Log (Millisecond Precision)")
                                .font(FontId::proportional(12.0))
                                .strong()
                                .color(FG_LIGHT),
                        );
                    });

                    ui.add_space(4.0);

                    // Keystroke Log Scroll Area
                    let scroll_height = ui.available_height() - 60.0;
                    ScrollArea::vertical()
                        .max_height(scroll_height)
                        .stick_to_bottom(true)
                        .show(ui, |ui| {
                            ui.add_space(4.0);
                            for entry in &self.logs {
                                ui.horizontal(|ui| {
                                    ui.add_space(8.0);
                                    // Timestamp
                                    ui.label(
                                        RichText::new(format!("[{}]", entry.timestamp))
                                            .font(FontId::monospace(11.0))
                                            .color(ACCENT_BLUE),
                                    );
                                    // Key label & name
                                    ui.label(
                                        RichText::new("Key:")
                                            .font(FontId::monospace(11.0))
                                            .color(FG_MUTED),
                                    );
                                    ui.label(
                                        RichText::new(format!("{:10}", entry.key))
                                            .font(FontId::monospace(11.0))
                                            .strong()
                                            .color(ACCENT_ORANGE),
                                    );
                                    // Interval label & latency
                                    ui.label(
                                        RichText::new("Interval:")
                                            .font(FontId::monospace(11.0))
                                            .color(FG_MUTED),
                                    );
                                    let lat_color = match entry.latency_ms {
                                        Some(ms) if ms < 250 => ACCENT_GREEN,
                                        Some(ms) if ms < 600 => ACCENT_YELLOW,
                                        Some(_) => ACCENT_RED,
                                        None => FG_MUTED,
                                    };
                                    ui.label(
                                        RichText::new(&entry.latency_str)
                                            .font(FontId::monospace(11.0))
                                            .strong()
                                            .color(lat_color),
                                    );
                                });
                            }

                            if self.scroll_to_bottom {
                                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                                self.scroll_to_bottom = false;
                            }
                        });

                    // Log Controls Footer
                    egui::Frame::none()
                        .fill(BG_HEADER)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                if ui.add(egui::Button::new(
                                    RichText::new("CLEAR LOG").strong().color(FG_LIGHT),
                                )).clicked() {
                                    self.clear_all();
                                }

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(8.0);
                                    if ui.add(egui::Button::new(
                                        RichText::new("EXPORT CSV").strong().color(FG_LIGHT),
                                    )).clicked() {
                                        self.export_csv();
                                    }
                                });
                            });
                        });
                });
            });

        // Central Panel (Editor)
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(BG_DARK))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Editor Status Bar at bottom
                    egui::TopBottomPanel::bottom("editor_status")
                        .exact_height(28.0)
                        .frame(egui::Frame::none().fill(BG_HEADER))
                        .show_inside(ui, |ui| {
                            ui.horizontal_centered(|ui| {
                                ui.add_space(10.0);
                                if let Some((msg, created)) = &self.status_message {
                                    if created.elapsed().as_secs() < 4 {
                                        ui.label(
                                            RichText::new(msg)
                                                .font(FontId::proportional(11.0))
                                                .color(ACCENT_YELLOW),
                                        );
                                    }
                                }
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(12.0);
                                    let status_text = format!(
                                        "Chars: {} | Words: {}",
                                        self.char_count(),
                                        self.word_count()
                                    );
                                    ui.label(
                                        RichText::new(status_text)
                                            .font(FontId::proportional(11.0))
                                            .color(FG_LIGHT),
                                    );
                                });
                            });
                        });

                    // Multi-line Text Editor
                    let editor_margin = 10.0;
                    ui.add_space(editor_margin);
                    ui.horizontal(|ui| {
                        ui.add_space(editor_margin);
                        let available_size = ui.available_size() - egui::vec2(editor_margin, 0.0);
                        let editor_response = ui.add_sized(
                            available_size,
                            egui::TextEdit::multiline(&mut self.editor_text)
                                .font(FontId::monospace(14.0))
                                .text_color(FG_LIGHT)
                                .desired_width(f32::INFINITY)
                                .hint_text("Start typing here... Keystroke speeds will be recorded in real-time."),
                        );

                        if self.request_focus_editor {
                            editor_response.request_focus();
                            self.request_focus_editor = false;
                        }
                    });
                });
            });
    }
}
