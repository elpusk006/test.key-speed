// DOM Elements
const editor = document.getElementById("editor");
const charWordCount = document.getElementById("char-word-count");
const keysCountVal = document.getElementById("keys-count-val");
const avgLatencyVal = document.getElementById("avg-latency-val");
const logDisplay = document.getElementById("log-display");
const btnClear = document.getElementById("btn-clear");
const btnExport = document.getElementById("btn-export");

// Application State
let lastKeypressTime = null;
let logs = [];
let totalKeys = 0;
let totalLatency = 0.0;
let latencyCount = 0;

// Resolve user-friendly key representation to match Python Tkinter
function getKeyDisplay(event) {
  const key = event.key;
  
  // Custom key display format
  if (key === " ") {
    return "Space";
  } else if (key === "Enter") {
    return "Enter";
  } else if (key === "Backspace") {
    return "Backspace";
  } else if (key === "Tab") {
    return "Tab";
  } else if (key.length === 1) {
    // Printable characters
    return `'${key}'`;
  } else {
    // Other keys (Control, Shift, ArrowKeys, etc.)
    return `<${key}>`;
  }
}

// Format timestamp down to millisecond precision: HH:MM:SS.mmm
function getTimestamp() {
  const now = new Date();
  const hours = String(now.getHours()).padStart(2, "0");
  const minutes = String(now.getMinutes()).padStart(2, "0");
  const seconds = String(now.getSeconds()).padStart(2, "0");
  const ms = String(now.getMilliseconds()).padStart(3, "0");
  return `${hours}:${minutes}:${seconds}.${ms}`;
}

// Append keystroke log entry to the scrollable console panel
function appendLog(timestamp, keyDisplay, latencyMs, latencyStr) {
  const row = document.createElement("div");
  row.className = "log-row";
  
  // 1. Timestamp
  const timeSpan = document.createElement("span");
  timeSpan.className = "log-time";
  timeSpan.textContent = `[${timestamp}]  `;
  row.appendChild(timeSpan);
  
  // 2. Key Label & Value
  const labelKeySpan = document.createElement("span");
  labelKeySpan.className = "log-label";
  labelKeySpan.textContent = "Key: ";
  row.appendChild(labelKeySpan);
  
  const keySpan = document.createElement("span");
  keySpan.className = "log-key";
  // Pad the key display name to 12 chars to maintain neat column alignment
  keySpan.textContent = keyDisplay.padEnd(12, " ");
  row.appendChild(keySpan);
  
  // 3. Interval Label & Value
  const labelIntervalSpan = document.createElement("span");
  labelIntervalSpan.className = "log-label";
  labelIntervalSpan.textContent = "Interval: ";
  row.appendChild(labelIntervalSpan);
  
  const intervalSpan = document.createElement("span");
  intervalSpan.className = "log-interval";
  
  if (latencyMs === null) {
    intervalSpan.classList.add("log-label"); // "First" gets muted styling
  } else {
    // Dynamic color coding based on keypress interval speed
    if (latencyMs < 250) {
      intervalSpan.classList.add("log-lat-green");
    } else if (latencyMs < 600) {
      intervalSpan.classList.add("log-lat-yellow");
    } else {
      intervalSpan.classList.add("log-lat-red");
    }
  }
  intervalSpan.textContent = latencyStr;
  row.appendChild(intervalSpan);
  
  // Append & Auto-scroll log
  logDisplay.appendChild(row);
  logDisplay.scrollTop = logDisplay.scrollHeight;
}

// Track keypress events
function onKeyPress(event) {
  const timestamp = getTimestamp();
  const currentPerf = performance.now();
  
  let latencyMs = null;
  let latencyStr = "First";
  
  if (lastKeypressTime !== null) {
    latencyMs = currentPerf - lastKeypressTime;
    
    // If latency is 1.5 seconds (1500 ms) or more, clear the log first
    if (latencyMs >= 1500) {
      clearLog();
      latencyMs = null;
      latencyStr = "First";
    } else {
      latencyStr = `+${Math.round(latencyMs)}ms`;
      
      // Update statistics
      totalLatency += latencyMs;
      latencyCount++;
      const avgLatency = totalLatency / latencyCount;
      avgLatencyVal.textContent = `${Math.round(avgLatency)} ms`;
      
      // Update average latency styling
      avgLatencyVal.className = "stat-value";
      if (avgLatency < 250) {
        avgLatencyVal.classList.add("speed-green");
      } else if (avgLatency < 600) {
        avgLatencyVal.classList.add("speed-yellow");
      } else {
        avgLatencyVal.classList.add("speed-red");
      }
    }
  } else {
    latencyMs = null;
    latencyStr = "First";
  }
  
  lastKeypressTime = currentPerf;
  const keyDisplay = getKeyDisplay(event);
  
  // Store log item for export
  logs.push({
    timestamp: timestamp,
    key: keyDisplay,
    latency: latencyMs !== null ? Math.round(latencyMs) : ""
  });
  
  // Update total key typed counter
  totalKeys++;
  keysCountVal.textContent = totalKeys;
  
  // Output line to terminal log
  appendLog(timestamp, keyDisplay, latencyMs, latencyStr);
}

// Update word & character statistics
function updateCounts() {
  const text = editor.value;
  const chars = text.length;
  // Regex to match non-whitespace words
  const words = text.trim() === "" ? 0 : text.trim().split(/\s+/).length;
  charWordCount.textContent = `Chars: ${chars} | Words: ${words}`;
}

// Reset logs & stats to clean state
function clearLog() {
  logDisplay.innerHTML = "";
  logs = [];
  lastKeypressTime = null;
  totalKeys = 0;
  totalLatency = 0.0;
  latencyCount = 0;
  
  keysCountVal.textContent = "0";
  avgLatencyVal.textContent = "0 ms";
  avgLatencyVal.className = "stat-value speed-green";
  
  editor.focus();
}

// Export statistics log to a CSV spreadsheet file
function exportCSV() {
  if (logs.length === 0) {
    alert("There is no keystroke log data to export.");
    return;
  }
  
  const csvRows = [["Timestamp", "Key Pressed", "Interval (ms)"]];
  logs.forEach(row => {
    csvRows.push([row.timestamp, row.key, row.latency]);
  });
  
  const csvContent = csvRows
    .map(e => e.map(val => `"${String(val).replace(/"/g, '""')}"`).join(","))
    .join("\n");
  
  const blob = new Blob([csvContent], { type: "text/csv;charset=utf-8;" });
  const url = URL.createObjectURL(blob);
  
  const link = document.createElement("a");
  link.setAttribute("href", url);
  link.setAttribute("download", `keystroke_log_${Date.now()}.csv`);
  link.style.visibility = "hidden";
  
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

// Bind event listeners
editor.addEventListener("keydown", onKeyPress);
editor.addEventListener("keyup", updateCounts);
btnClear.addEventListener("click", clearLog);
btnExport.addEventListener("click", exportCSV);

// Autofocus on load
editor.focus();
