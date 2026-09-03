# Reactive Telemetry Subsystem

ZenLavaTerm includes a zero-dependency system telemetry subsystem (`src/reactive/`) that converts native OS resource utilization into real-time fluid perturbations.

---

## 1. Normalized Signal Abstraction (`src/reactive/signals.rs`)

All platform telemetry is normalized into continuous floating-point metrics strictly bounded in `[0.0, 1.0]`:

```rust
pub struct SystemSignals {
    pub cpu_load: f32,       // Total CPU utilization across all cores [0.0, 1.0]
    pub memory_usage: f32,   // Active RAM utilization percentage [0.0, 1.0]
    pub battery_level: f32,  // Battery charge percentage [0.0, 1.0]
    pub io_activity: f32,    // Disk / IO throughput activity index [0.0, 1.0]
}
```

---

## 2. Platform Provider Implementations

Each supported operating system implements the `SystemProvider` trait:

### 2.1 Linux (`src/reactive/linux.rs`)
- **CPU Load**: Parses `/proc/stat` delta between user, nice, system, and idle jiffies.
- **Memory**: Parses `MemTotal` and `MemAvailable` from `/proc/meminfo`.
- **Battery**: Reads `/sys/class/power_supply/*/capacity` and `/sys/class/power_supply/*/status`.
- **I/O**: Evaluates sector read/write deltas from `/proc/diskstats`.

### 2.2 Windows (`src/reactive/windows.rs`)
- **CPU Load**: Invokes Win32 `GetSystemTimes` to compute idle vs kernel/user time deltas.
- **Memory**: Queries `GlobalMemoryStatusEx` for physical RAM utilization.
- **Battery**: Queries `GetSystemPowerStatus` for AC line status and battery percentage.
- **I/O**: Reads `GetProcessIoCounters` for byte transfer rates.

### 2.3 macOS (`src/reactive/macos.rs`)
- **CPU Load**: Interrogates Mach kernel subsystem via `host_statistics64` with `HOST_CPU_LOAD_INFO`.
- **Memory**: Reads `HOST_VM_INFO64` to aggregate active, wired, and compressed page counts against physical memory.

### 2.4 Mock Provider (`src/reactive/provider.rs`)
- `MockSystemProvider`: Provides deterministic, controllable telemetry signals for unit tests and headless CI environments.

---

## 3. Graceful Degradation Invariant

If an OS file is missing (e.g. running in a restricted container without `/proc` access) or a system call fails, providers do not panic:
- They log a warning once and return default baseline signals (`0.0`).
- The simulation continues running smoothly without interruption.
