# Reactive System Observability

LavaTerm includes an ambient system observability engine that transforms operating system metrics into organic fluid motion without cluttering your workspace with charts or numeric gauges.

## Architecture

The reactive system follows a strictly decoupled provider-signal pattern:

```text
┌─────────────────────────────────────────────────────────────┐
│                 System Metric Providers                     │
│  - LinuxSystemProvider (/proc/stat, /proc/meminfo, /sys)    │
│  - MockSystemProvider (deterministic testing)               │
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

| Metric | Source (Linux) | Range | Lava Physical Effect |
|---|---|:---:|---|
| **CPU Utilization** | `/proc/stat` delta ticks | `[0.0, 1.0]` | Modulates Brownian thermal noise and fluid turbulence ($0.15 \times (1.0 + 2.5 \times \text{cpu})$). |
| **RAM Usage** | `/proc/meminfo` (`MemTotal` vs `MemAvailable`) | `[0.0, 1.0]` | Modulates active blob radius and expansion ($0.85 + 0.40 \times \text{ram}$). |
| **Battery Level** | `/sys/class/power_supply/BAT*/capacity` | `[0.0, 1.0]` | Modulates thermal buoyancy and convection energy ($0.50 + 0.60 \times \text{bat}$). |
| **Disk/Storage I/O** | `/proc/diskstats` delta sectors | `[0.0, 1.0]` | Modulates bubble perturbance frequency. |

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
