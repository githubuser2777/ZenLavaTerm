# Reactive System Observability

LavaTerm includes an ambient system observability engine that transforms operating system metrics into organic fluid motion without cluttering your workspace with charts or numeric gauges.

## Architecture

The reactive system follows a strictly decoupled provider-signal pattern:

```text
┌─────────────────────────────────────────────────────────────┐
│                 System Metric Providers                     │
│  - LinuxSystemProvider (/proc/stat, /proc/meminfo, /sys)    │
│  - WindowsSystemProvider (GetSystemTimes, GlobalMemoryStatus)│
│  - MacOSSystemProvider (host_statistics64, sysctl)          │
│  - MockSystemProvider (deterministic testing / fallback)    │
└──────────────────────────────┬──────────────────────────────┘
                               │ Polls OS metrics
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 SystemSignals [0.0, 1.0]                    │
│  - cpu_load: f32                                            │
│  - memory_usage: f32                                        │
│  - battery_level: f32                                       │
│  - io_activity: f32                                         │
└──────────────────────────────┬──────────────────────────────┘
                               │ Modulates physics
                               ▼
┌─────────────────────────────────────────────────────────────┐
│                 Simulation Core (Blobs)                     │
│  - CPU load     ──> Increases thermal turbulence & noise    │
│  - Memory usage ──> Dynamically scales blob radii           │
│  - Battery      ──> Adjusts convection speed and buoyancy   │
└─────────────────────────────────────────────────────────────┘
```

## Metric Mappings

| Metric | Source (Linux) | Source (Windows) | Source (macOS) | Range | Lava Physical Effect |
|---|---|---|---|:---:|---|
| **CPU Utilization** | `/proc/stat` delta ticks | `GetSystemTimes` (idle vs total) | `host_statistics64` (`HOST_CPU_LOAD_INFO`) | `[0.0, 1.0]` | Modulates Brownian thermal noise and fluid turbulence ($0.15 \times (1.0 + 2.5 \times \text{cpu})$). |
| **RAM Usage** | `/proc/meminfo` (`MemTotal` vs `MemAvailable`) | `GlobalMemoryStatusEx` (`ullTotalPhys` vs `ullAvailPhys`) | `host_statistics64` (`HOST_VM_INFO64`) & `sysctl` | `[0.0, 1.0]` | Modulates active blob radius and expansion ($0.85 + 0.40 \times \text{ram}$). |
| **Battery Level** | `/sys/class/power_supply/BAT*/capacity` | `GetSystemPowerStatus` (`BatteryLifePercent`) | Power Management / Default 1.0 | `[0.0, 1.0]` | Modulates thermal buoyancy and convection energy ($0.50 + 0.60 \times \text{bat}$). |
| **Disk/Storage I/O** | `/proc/diskstats` delta sectors | `GetProcessIoCounters` delta transfer bytes | Delta throughput baseline | `[0.0, 1.0]` | Modulates bubble perturbance frequency. |

## Usage

### Enabling via CLI Flag

To run LavaTerm in ambient system monitoring mode:

```bash
lavaterm --system
```

Combine with any renderer backend:

```bash
lavaterm --system --renderer braille
```

### Enabling via TOML Configuration

```toml
[reactive]
enabled = true
poll_interval_ms = 500
```

## Platform Support

- **Linux**: Full native zero-dependency metric telemetry via `/proc/stat` (CPU ticks), `/proc/meminfo` (active/available memory), `/proc/diskstats` (I/O sectors), and `/sys/class/power_supply` (battery capacity).
- **Windows**: Native zero-dependency Win32 telemetry via `kernel32` APIs (`GetSystemTimes` for CPU delta ticks, `GlobalMemoryStatusEx` for physical RAM utilization, `GetSystemPowerStatus` for battery and AC status, and `GetProcessIoCounters` for I/O activity).
- **macOS**: Native Darwin telemetry using Mach kernel subsystem APIs (`host_statistics64` with `HOST_CPU_LOAD_INFO` for CPU ticks and `HOST_VM_INFO64` for memory pages).
- **Graceful Fallback**: If hardware metrics or platform APIs are unavailable or permission-restricted, `MockSystemProvider` automatically provides normalized baseline signals (`SystemSignals::default()`), ensuring uninterrupted simulation across all platforms without runtime errors.
