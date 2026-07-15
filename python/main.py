import tkinter as tk
from tkinter import ttk, filedialog, messagebox
import datetime
import time
import csv

# Modern Dark Theme Palette
BG_DARKER = "#181818"      # Right panel log background
BG_DARK = "#1e1e1e"        # Editor background
BG_HEADER = "#252526"      # Top stats and button area background
FG_LIGHT = "#e3e3e3"       # General text
FG_MUTED = "#858585"       # Subtitles/labels
ACCENT_BLUE = "#4fc3f7"    # Timestamps (Teal/Blue)
ACCENT_ORANGE = "#ffb74d"  # Key names (Orange)
ACCENT_GREEN = "#81c784"   # Latency under 250ms (Green)
ACCENT_YELLOW = "#d4e157"  # Latency 250ms - 600ms (Lime/Yellow)
ACCENT_RED = "#ef5350"     # Latency above 600ms (Red)

class NotepadApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Key-Speed Notepad Logger")
        self.root.geometry("1100x700")
        self.root.minsize(800, 500)
        
        # Application state
        self.last_keypress_time = None
        self.logs = []
        self.total_keys = 0
        self.total_latency = 0.0
        self.latency_count = 0
        
        # Configure overall window background
        self.root.configure(bg=BG_DARK)
        
        # Configure styles
        self.style = ttk.Style()
        self.style.theme_use('clam')
        
        # Custom styles for dark theme components
        self.style.configure('.', background=BG_DARK, foreground=FG_LIGHT)
        self.style.configure('TFrame', background=BG_DARK)
        self.style.configure('Header.TFrame', background=BG_HEADER)
        
        self.style.configure('TLabel', background=BG_DARK, foreground=FG_LIGHT, font=("Segoe UI", 10))
        self.style.configure('Header.TLabel', background=BG_HEADER, foreground=FG_LIGHT, font=("Segoe UI", 10))
        self.style.configure('Title.TLabel', background=BG_DARK, foreground=FG_LIGHT, font=("Segoe UI", 13, "bold"))
        self.style.configure('StatVal.TLabel', background=BG_HEADER, foreground=ACCENT_BLUE, font=("Segoe UI", 14, "bold"))
        
        # Button styles
        self.style.configure('Action.TButton', background="#3e3e42", foreground=FG_LIGHT, borderwidth=0, font=("Segoe UI", 9, "bold"))
        self.style.map('Action.TButton',
            background=[('active', '#505054'), ('pressed', '#2d2d30')],
            foreground=[('active', '#ffffff')]
        )
        
        # Create Layout
        self.create_widgets()
        
        # Bind events
        self.editor.bind("<KeyPress>", self.on_key_press)
        self.editor.bind("<KeyRelease>", self.update_counts)
        self.editor.focus_set()

    def create_widgets(self):
        # Main split container
        self.main_pane = ttk.PanedWindow(self.root, orient=tk.HORIZONTAL)
        self.main_pane.pack(fill=tk.BOTH, expand=True)
        
        # --- LEFT PANEL (Editor) ---
        self.left_frame = ttk.Frame(self.main_pane)
        self.main_pane.add(self.left_frame, weight=3)
        
        # Editor Header
        self.left_header = ttk.Frame(self.left_frame, style='Header.TFrame', height=50)
        self.left_header.pack(fill=tk.X, side=tk.TOP)
        self.left_header.pack_propagate(False)
        
        self.editor_title = ttk.Label(self.left_header, text="  Notepad Editor", style='Header.TLabel', font=("Segoe UI", 11, "bold"))
        self.editor_title.pack(side=tk.LEFT, padx=10, pady=10)
        
        # Editor Text Area
        self.editor = tk.Text(
            self.left_frame,
            bg=BG_DARK,
            fg=FG_LIGHT,
            insertbackground="#ffffff",          # Caret color
            selectbackground="#264f78",          # Selection color
            selectforeground="#ffffff",
            font=("Consolas", 12),
            padx=15,
            pady=15,
            bd=0,
            highlightthickness=1,
            highlightbackground="#2d2d2d",
            highlightcolor="#3e3e42",
            undo=True
        )
        self.editor.pack(fill=tk.BOTH, expand=True, side=tk.TOP)
        
        # Editor Status Bar (Counts)
        self.editor_status = ttk.Frame(self.left_frame, style='Header.TFrame', height=30)
        self.editor_status.pack(fill=tk.X, side=tk.BOTTOM)
        self.editor_status.pack_propagate(False)
        
        self.char_count_lbl = ttk.Label(self.editor_status, text="Chars: 0 | Words: 0", style='Header.TLabel', font=("Segoe UI", 9))
        self.char_count_lbl.pack(side=tk.RIGHT, padx=15, pady=5)
        
        # --- RIGHT PANEL (Log & Stats) ---
        self.right_frame = ttk.Frame(self.main_pane)
        self.main_pane.add(self.right_frame, weight=2)
        
        # Stats Top Dashboard
        self.stats_frame = ttk.Frame(self.right_frame, style='Header.TFrame', height=90)
        self.stats_frame.pack(fill=tk.X, side=tk.TOP)
        self.stats_frame.pack_propagate(False)
        
        # Stat 1: Total Keys
        self.stat_keys_frame = ttk.Frame(self.stats_frame, style='Header.TFrame')
        self.stat_keys_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=5, pady=10)
        
        self.lbl_keys_title = ttk.Label(self.stat_keys_frame, text="KEYS TYPED", style='Header.TLabel', font=("Segoe UI", 8, "bold"), foreground=FG_MUTED)
        self.lbl_keys_title.pack(anchor="w", padx=10)
        self.lbl_keys_val = ttk.Label(self.stat_keys_frame, text="0", style='StatVal.TLabel')
        self.lbl_keys_val.pack(anchor="w", padx=10, pady=2)
        
        # Stat 2: Avg Latency
        self.stat_latency_frame = ttk.Frame(self.stats_frame, style='Header.TFrame')
        self.stat_latency_frame.pack(side=tk.LEFT, fill=tk.BOTH, expand=True, padx=5, pady=10)
        
        self.lbl_lat_title = ttk.Label(self.stat_latency_frame, text="AVG LATENCY", style='Header.TLabel', font=("Segoe UI", 8, "bold"), foreground=FG_MUTED)
        self.lbl_lat_title.pack(anchor="w", padx=10)
        self.lbl_lat_val = ttk.Label(self.stat_latency_frame, text="0 ms", style='StatVal.TLabel', foreground=ACCENT_GREEN)
        self.lbl_lat_val.pack(anchor="w", padx=10, pady=2)
        
        # Log Pane Header
        self.log_header = ttk.Frame(self.right_frame, style='Header.TFrame', height=40)
        self.log_header.pack(fill=tk.X, side=tk.TOP, pady=(2, 0))
        self.log_header.pack_propagate(False)
        
        self.log_title = ttk.Label(self.log_header, text="  Keystroke Log (Millisecond Precision)", style='Header.TLabel', font=("Segoe UI", 10, "bold"))
        self.log_title.pack(side=tk.LEFT, padx=10, pady=8)
        
        # Log Text Area (Read-Only)
        self.log_display = tk.Text(
            self.right_frame,
            bg=BG_DARKER,
            fg=FG_LIGHT,
            font=("Consolas", 10),
            padx=12,
            pady=12,
            bd=0,
            highlightthickness=0,
            state=tk.DISABLED,
            wrap=tk.WORD
        )
        self.log_display.pack(fill=tk.BOTH, expand=True, side=tk.TOP)
        
        # Define syntax styling tags for the log output
        self.log_display.tag_config("time", foreground=ACCENT_BLUE)
        self.log_display.tag_config("key", foreground=ACCENT_ORANGE, font=("Consolas", 10, "bold"))
        self.log_display.tag_config("lat_green", foreground=ACCENT_GREEN)
        self.log_display.tag_config("lat_yellow", foreground=ACCENT_YELLOW)
        self.log_display.tag_config("lat_red", foreground=ACCENT_RED)
        self.log_display.tag_config("label", foreground=FG_MUTED)
        
        # Log Footer Controls
        self.log_controls = ttk.Frame(self.right_frame, style='Header.TFrame', height=50)
        self.log_controls.pack(fill=tk.X, side=tk.BOTTOM)
        self.log_controls.pack_propagate(False)
        
        self.btn_clear = ttk.Button(self.log_controls, text="CLEAR LOG", style='Action.TButton', command=self.clear_log)
        self.btn_clear.pack(side=tk.LEFT, padx=15, pady=10)
        
        self.btn_export = ttk.Button(self.log_controls, text="EXPORT CSV", style='Action.TButton', command=self.export_csv)
        self.btn_export.pack(side=tk.RIGHT, padx=15, pady=10)

    def on_key_press(self, event):
        # Calculate current time down to milliseconds
        now = datetime.datetime.now()
        timestamp = now.strftime("%H:%M:%S") + f".{now.microsecond // 1000:03d}"
        
        # Calculate precise interval in ms from last keypress
        current_perf = time.perf_counter()
        if self.last_keypress_time is not None:
            latency_ms = (current_perf - self.last_keypress_time) * 1000
            
            # If latency is 1.5 seconds (1500 ms) or more, clear the log first
            if latency_ms >= 1500:
                self.clear_log()
                latency_ms = None
                latency_str = "First"
            else:
                latency_str = f"+{int(latency_ms)}ms"
                # Update stats
                self.total_latency += latency_ms
                self.latency_count += 1
                avg_latency = self.total_latency / self.latency_count
                self.lbl_lat_val.config(text=f"{int(avg_latency)} ms")
                
                # Change color of average latency indicator based on speed
                if avg_latency < 250:
                    self.lbl_lat_val.config(foreground=ACCENT_GREEN)
                elif avg_latency < 600:
                    self.lbl_lat_val.config(foreground=ACCENT_YELLOW)
                else:
                    self.lbl_lat_val.config(foreground=ACCENT_RED)
        else:
            latency_ms = None
            latency_str = "First"
        
        self.last_keypress_time = current_perf
        
        # Resolve user-friendly key representation
        key_symbol = event.keysym
        char_val = event.char
        
        # Custom key display format
        key_display = key_symbol
        if key_symbol == 'space':
            key_display = 'Space'
        elif key_symbol == 'Return':
            key_display = 'Enter'
        elif key_symbol == 'BackSpace':
            key_display = 'Backspace'
        elif key_symbol == 'Tab':
            key_display = 'Tab'
        elif char_val and char_val.isprintable() and len(key_symbol) == 1:
            key_display = f"'{char_val}'"
        else:
            key_display = f"<{key_symbol}>"
            
        # Store log
        self.logs.append({
            "timestamp": timestamp,
            "key": key_display,
            "latency": int(latency_ms) if latency_ms is not None else ""
        })
        
        # Update keys typed stat
        self.total_keys += 1
        self.lbl_keys_val.config(text=str(self.total_keys))
        
        # Write event to log panel
        self.append_log(timestamp, key_display, latency_ms, latency_str)

    def append_log(self, timestamp, key_display, latency_ms, latency_str):
        self.log_display.config(state=tk.NORMAL)
        
        # Format elements
        self.log_display.insert(tk.END, f"[{timestamp}]  ", "time")
        self.log_display.insert(tk.END, "Key: ", "label")
        self.log_display.insert(tk.END, f"{key_display:<12}", "key")
        
        # Add latency with dynamic speed coloring
        self.log_display.insert(tk.END, "Interval: ", "label")
        
        if latency_ms is None:
            self.log_display.insert(tk.END, f"{latency_str}\n", "label")
        else:
            tag = "lat_green"
            if latency_ms > 600:
                tag = "lat_red"
            elif latency_ms > 250:
                tag = "lat_yellow"
            
            self.log_display.insert(tk.END, f"{latency_str}\n", tag)
            
        # Auto scroll to bottom
        self.log_display.see(tk.END)
        self.log_display.config(state=tk.DISABLED)

    def update_counts(self, event=None):
        content = self.editor.get("1.0", tk.END)
        # Remove trailing newline automatically appended by Tkinter Text
        if content.endswith('\n'):
            content = content[:-1]
        chars = len(content)
        words = len(content.split())
        self.char_count_lbl.config(text=f"Chars: {chars} | Words: {words}")

    def clear_log(self):
        self.log_display.config(state=tk.NORMAL)
        self.log_display.delete("1.0", tk.END)
        self.log_display.config(state=tk.DISABLED)
        
        # Reset relative states
        self.logs.clear()
        self.last_keypress_time = None
        self.total_keys = 0
        self.total_latency = 0.0
        self.latency_count = 0
        
        self.lbl_keys_val.config(text="0")
        self.lbl_lat_val.config(text="0 ms", foreground=ACCENT_GREEN)
        
        # Set focus back to the notepad editor automatically
        self.editor.focus_set()


    def export_csv(self):
        if not self.logs:
            messagebox.showwarning("Empty Log", "There is no keystroke log data to export.")
            return
            
        file_path = filedialog.asksaveasfilename(
            defaultextension=".csv",
            filetypes=[("CSV Files", "*.csv"), ("Text Files", "*.txt"), ("All Files", "*.*")],
            title="Save Keystroke Log"
        )
        
        if file_path:
            try:
                with open(file_path, 'w', newline='', encoding='utf-8') as f:
                    writer = csv.writer(f)
                    writer.writerow(["Timestamp", "Key Pressed", "Interval (ms)"])
                    for row in self.logs:
                        writer.writerow([row["timestamp"], row["key"], row["latency"]])
                messagebox.showinfo("Export Successful", f"Keystroke log exported to:\n{file_path}")
            except Exception as e:
                messagebox.showerror("Export Error", f"Failed to export log:\n{str(e)}")

if __name__ == "__main__":
    root = tk.Tk()
    app = NotepadApp(root)
    root.mainloop()
