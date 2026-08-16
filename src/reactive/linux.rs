//! Native Linux system metrics provider reading `/proc` and `/sys/class/power_supply`.

use super::provider::SystemProvider;
use super::signals::SystemSignals;
use std::fs;
use std::path::{Path, PathBuf};

/// System metrics collector reading Linux virtual filesystems without external dependencies.
#[derive(Debug)]
pub struct LinuxSystemProvider {
    stat_path: PathBuf,
    meminfo_path: PathBuf,
    battery_dir: PathBuf,
    diskstats_path: PathBuf,
    last_cpu: Option<(u64, u64)>, // (total, active)
    last_io_sectors: Option<u64>,
}

impl Default for LinuxSystemProvider {
    fn default() -> Self {
        Self {
            stat_path: PathBuf::from("/proc/stat"),
            meminfo_path: PathBuf::from("/proc/meminfo"),
            battery_dir: PathBuf::from("/sys/class/power_supply"),
            diskstats_path: PathBuf::from("/proc/diskstats"),
            last_cpu: None,
            last_io_sectors: None,
        }
    }
}

impl LinuxSystemProvider {
    /// Creates a new `LinuxSystemProvider` configured with custom paths (useful for testing).
    pub fn new_with_paths(
        stat_path: impl AsRef<Path>,
        meminfo_path: impl AsRef<Path>,
        battery_dir: impl AsRef<Path>,
        diskstats_path: impl AsRef<Path>,
    ) -> Self {
        Self {
            stat_path: stat_path.as_ref().to_path_buf(),
            meminfo_path: meminfo_path.as_ref().to_path_buf(),
            battery_dir: battery_dir.as_ref().to_path_buf(),
            diskstats_path: diskstats_path.as_ref().to_path_buf(),
            last_cpu: None,
            last_io_sectors: None,
        }
    }

    /// Parses total and active CPU ticks from `/proc/stat` content.
    pub fn parse_cpu_stat(content: &str) -> Option<(u64, u64)> {
        for line in content.lines() {
            if line.starts_with("cpu ") {
                let parts: Vec<u64> = line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|s| s.parse::<u64>().ok())
                    .collect();

                if parts.len() >= 4 {
                    let user = parts[0];
                    let nice = parts[1];
                    let system = parts[2];
                    let idle = parts[3];
                    let iowait = parts.get(4).copied().unwrap_or(0);
                    let irq = parts.get(5).copied().unwrap_or(0);
                    let softirq = parts.get(6).copied().unwrap_or(0);
                    let steal = parts.get(7).copied().unwrap_or(0);

                    let total = user + nice + system + idle + iowait + irq + softirq + steal;
                    let active = user + nice + system + irq + softirq + steal;
                    return Some((total, active));
                }
            }
        }
        None
    }

    /// Parses total and available memory in kB from `/proc/meminfo`.
    pub fn parse_meminfo(content: &str) -> Option<(u64, u64)> {
        let mut total_kb = None;
        let mut avail_kb = None;

        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<u64>().ok());
            } else if line.starts_with("MemAvailable:") {
                avail_kb = line
                    .split_whitespace()
                    .nth(1)
                    .and_then(|s| s.parse::<u64>().ok());
            }
            if total_kb.is_some() && avail_kb.is_some() {
                break;
            }
        }

        match (total_kb, avail_kb) {
            (Some(tot), Some(avail)) if tot > 0 => Some((tot, avail)),
            _ => None,
        }
    }

    /// Parses battery capacity from capacity file string.
    pub fn parse_battery(content: &str) -> Option<f32> {
        content
            .trim()
            .parse::<f32>()
            .ok()
            .map(|cap| (cap / 100.0).clamp(0.0, 1.0))
    }

    /// Parses total disk sectors read & written from `/proc/diskstats`.
    pub fn parse_diskstats(content: &str) -> u64 {
        let mut total_sectors: u64 = 0;
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            // Typical diskstats: major minor dev_name reads_completed reads_merged sectors_read ms_reading writes_completed writes_merged sectors_written ...
            if parts.len() >= 10 {
                let dev_name = parts[2];
                // Filter partition devices (keep main block devices like sda, nvme0n1, vda)
                if !dev_name.starts_with("loop") && !dev_name.starts_with("ram") {
                    let reads_sec = parts[5].parse::<u64>().unwrap_or(0);
                    let writes_sec = parts[9].parse::<u64>().unwrap_or(0);
                    total_sectors = total_sectors
                        .saturating_add(reads_sec)
                        .saturating_add(writes_sec);
                }
            }
        }
        total_sectors
    }

    fn read_cpu_load(&mut self) -> f32 {
        if let Ok(content) = fs::read_to_string(&self.stat_path) {
            if let Some((total, active)) = Self::parse_cpu_stat(&content) {
                if let Some((prev_total, prev_active)) = self.last_cpu {
                    let d_total = total.saturating_sub(prev_total);
                    let d_active = active.saturating_sub(prev_active);
                    self.last_cpu = Some((total, active));
                    if d_total > 0 {
                        return (d_active as f32 / d_total as f32).clamp(0.0, 1.0);
                    }
                } else {
                    self.last_cpu = Some((total, active));
                }
            }
        }
        0.15 // Graceful fallback
    }

    fn read_memory_usage(&self) -> f32 {
        if let Ok(content) = fs::read_to_string(&self.meminfo_path) {
            if let Some((total, avail)) = Self::parse_meminfo(&content) {
                let used = total.saturating_sub(avail);
                return (used as f32 / total as f32).clamp(0.0, 1.0);
            }
        }
        0.30 // Graceful fallback
    }

    fn read_battery_level(&self) -> f32 {
        if self.battery_dir.exists() {
            // Check BAT0, BAT1, or any battery device
            if let Ok(entries) = fs::read_dir(&self.battery_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name.starts_with("BAT") {
                        let cap_file = path.join("capacity");
                        if let Ok(content) = fs::read_to_string(cap_file) {
                            if let Some(level) = Self::parse_battery(&content) {
                                return level;
                            }
                        }
                    }
                }
            }
        }
        1.0 // Defaults to 100% (e.g. desktop on AC power)
    }

    fn read_io_activity(&mut self) -> f32 {
        if let Ok(content) = fs::read_to_string(&self.diskstats_path) {
            let total_sectors = Self::parse_diskstats(&content);
            if let Some(prev) = self.last_io_sectors {
                let delta = total_sectors.saturating_sub(prev);
                self.last_io_sectors = Some(total_sectors);
                // Normalize: 1000 sectors (~500KB) delta mapped to ~0.5 activity
                let normalized = (delta as f32 / 2000.0).clamp(0.0, 1.0);
                return normalized;
            } else {
                self.last_io_sectors = Some(total_sectors);
            }
        }
        0.05
    }
}

impl SystemProvider for LinuxSystemProvider {
    fn poll_signals(&mut self) -> SystemSignals {
        let cpu = self.read_cpu_load();
        let mem = self.read_memory_usage();
        let bat = self.read_battery_level();
        let io = self.read_io_activity();

        SystemSignals::new(cpu, mem, bat, io)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cpu_stat_valid() {
        let stat_sample =
            "cpu  10132153 290696 3084719 46828483 16683 415309 234562 0 0 0\ncpu0 1234 5678 ...\n";
        let parsed = LinuxSystemProvider::parse_cpu_stat(stat_sample);
        assert!(parsed.is_some());
        let (total, active) = parsed.unwrap();
        assert!(total > active);
        assert!(active > 0);
    }

    #[test]
    fn test_parse_meminfo_valid() {
        let meminfo_sample = "MemTotal:       16301248 kB\nMemFree:         4123456 kB\nMemAvailable:    8150624 kB\nBuffers:          512000 kB\n";
        let parsed = LinuxSystemProvider::parse_meminfo(meminfo_sample);
        assert_eq!(parsed, Some((16301248, 8150624)));
    }

    #[test]
    fn test_parse_battery_valid() {
        let bat_sample = "87\n";
        let parsed = LinuxSystemProvider::parse_battery(bat_sample);
        assert_eq!(parsed, Some(0.87));

        let full_sample = "100\n";
        assert_eq!(LinuxSystemProvider::parse_battery(full_sample), Some(1.00));
    }

    #[test]
    fn test_parse_diskstats_valid() {
        let diskstats_sample = "   8       0 sda 100 0 2000 50 300 0 4000 60 0 0 0\n   7       0 loop0 10 0 20 5 0 0 0 0 0 0 0\n";
        let total = LinuxSystemProvider::parse_diskstats(diskstats_sample);
        assert_eq!(total, 6000);
    }

    #[test]
    fn test_linux_provider_missing_files_does_not_panic() {
        let mut provider = LinuxSystemProvider::new_with_paths(
            "/nonexistent/stat",
            "/nonexistent/meminfo",
            "/nonexistent/battery",
            "/nonexistent/diskstats",
        );
        let signals = provider.poll_signals();
        assert!(signals.cpu_load >= 0.0 && signals.cpu_load <= 1.0);
        assert!(signals.memory_usage >= 0.0 && signals.memory_usage <= 1.0);
        assert_eq!(signals.battery_level, 1.0);
    }
}
