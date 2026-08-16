//! Normalized reactive signal data structures.

/// Normalized system metrics in range $[0.0, 1.0]$.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SystemSignals {
    /// Overall CPU utilization in $[0.0, 1.0]$.
    pub cpu_load: f32,
    /// System memory usage fraction in $[0.0, 1.0]$.
    pub memory_usage: f32,
    /// Battery capacity / charge level in $[0.0, 1.0]$ ($1.0$ for AC/desktop).
    pub battery_level: f32,
    /// Disk or I/O throughput activity metric in $[0.0, 1.0]$.
    pub io_activity: f32,
}

impl SystemSignals {
    /// Creates a new `SystemSignals` with values clamped to $[0.0, 1.0]$.
    pub fn new(cpu_load: f32, memory_usage: f32, battery_level: f32, io_activity: f32) -> Self {
        Self {
            cpu_load: cpu_load.clamp(0.0, 1.0),
            memory_usage: memory_usage.clamp(0.0, 1.0),
            battery_level: battery_level.clamp(0.0, 1.0),
            io_activity: io_activity.clamp(0.0, 1.0),
        }
    }
}

impl Default for SystemSignals {
    fn default() -> Self {
        Self {
            cpu_load: 0.10,
            memory_usage: 0.30,
            battery_level: 1.00,
            io_activity: 0.05,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_signals_clamping() {
        let signals = SystemSignals::new(1.5, -0.2, 2.0, -1.0);
        assert_eq!(signals.cpu_load, 1.0);
        assert_eq!(signals.memory_usage, 0.0);
        assert_eq!(signals.battery_level, 1.0);
        assert_eq!(signals.io_activity, 0.0);
    }

    #[test]
    fn test_system_signals_default() {
        let signals = SystemSignals::default();
        assert!(signals.cpu_load >= 0.0 && signals.cpu_load <= 1.0);
        assert!(signals.memory_usage >= 0.0 && signals.memory_usage <= 1.0);
        assert!(signals.battery_level >= 0.0 && signals.battery_level <= 1.0);
        assert!(signals.io_activity >= 0.0 && signals.io_activity <= 1.0);
    }
}
