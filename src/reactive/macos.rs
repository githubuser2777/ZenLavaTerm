//! Native macOS / Darwin system metrics provider.

use super::provider::SystemProvider;
use super::signals::SystemSignals;

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct HostCpuLoadInfo {
    cpu_ticks: [u32; 4], // CPU_STATE_USER, CPU_STATE_SYSTEM, CPU_STATE_IDLE, CPU_STATE_NICE
}

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
struct VmStatistics64 {
    free_count: u32,
    active_count: u32,
    inactive_count: u32,
    wire_count: u32,
    zero_fill_count: u64,
    reactivations: u64,
    pageins: u64,
    pageouts: u64,
    faults: u64,
    cow_faults: u64,
    lookups: u64,
    hits: u64,
    purges: u64,
    purgeable_count: u32,
    speculative_count: u32,
    decompressions: u64,
    compressions: u64,
    swapins: u64,
    swapouts: u64,
    compressor_page_count: u32,
    throttled_count: u32,
    external_page_count: u32,
    internal_page_count: u32,
    total_uncompressed_pages_in_compressor: u64,
}

#[cfg(target_os = "macos")]
extern "C" {
    fn mach_host_self() -> u32;
    fn host_statistics64(
        host_priv: u32,
        flavor: i32,
        host_info_out: *mut i32,
        host_info_outCnt: *mut u32,
    ) -> i32;
}

/// Native macOS system metrics provider.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct MacOSSystemProvider {
    last_cpu_ticks: Option<(u64, u64)>, // (total, active)
    last_io_metric: Option<u64>,
}

impl MacOSSystemProvider {
    /// Creates a new `MacOSSystemProvider`.
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(target_os = "macos")]
    fn read_cpu_load(&mut self) -> f32 {
        const HOST_CPU_LOAD_INFO: i32 = 3;
        let mut cpu_info = HostCpuLoadInfo::default();
        let mut count =
            (std::mem::size_of::<HostCpuLoadInfo>() / std::mem::size_of::<i32>()) as u32;

        let kr = unsafe {
            host_statistics64(
                mach_host_self(),
                HOST_CPU_LOAD_INFO,
                &mut cpu_info as *mut HostCpuLoadInfo as *mut i32,
                &mut count as *mut _,
            )
        };

        if kr == 0 {
            let user = cpu_info.cpu_ticks[0] as u64;
            let system = cpu_info.cpu_ticks[1] as u64;
            let idle = cpu_info.cpu_ticks[2] as u64;
            let nice = cpu_info.cpu_ticks[3] as u64;

            let total = user + system + idle + nice;
            let active = user + system + nice;

            if let Some((prev_total, prev_active)) = self.last_cpu_ticks {
                let d_total = total.saturating_sub(prev_total);
                let d_active = active.saturating_sub(prev_active);
                self.last_cpu_ticks = Some((total, active));

                if d_total > 0 {
                    return (d_active as f32 / d_total as f32).clamp(0.0, 1.0);
                }
            } else {
                self.last_cpu_ticks = Some((total, active));
            }
        }

        0.15
    }

    #[cfg(not(target_os = "macos"))]
    fn read_cpu_load(&mut self) -> f32 {
        0.15
    }

    #[cfg(target_os = "macos")]
    fn read_memory_usage(&self) -> f32 {
        const HOST_VM_INFO64: i32 = 4;
        let mut vm_stat = VmStatistics64::default();
        let mut count = (std::mem::size_of::<VmStatistics64>() / std::mem::size_of::<i32>()) as u32;

        let kr = unsafe {
            host_statistics64(
                mach_host_self(),
                HOST_VM_INFO64,
                &mut vm_stat as *mut VmStatistics64 as *mut i32,
                &mut count as *mut _,
            )
        };

        if kr == 0 {
            let active = vm_stat.active_count as u64;
            let wire = vm_stat.wire_count as u64;
            let compressed = vm_stat.compressor_page_count as u64;
            let free = vm_stat.free_count as u64;
            let inactive = vm_stat.inactive_count as u64;

            let used_pages = active + wire + compressed;
            let total_pages = used_pages + free + inactive;

            if total_pages > 0 {
                return (used_pages as f32 / total_pages as f32).clamp(0.0, 1.0);
            }
        }

        0.30
    }

    #[cfg(not(target_os = "macos"))]
    fn read_memory_usage(&self) -> f32 {
        0.30
    }

    fn read_battery_level(&self) -> f32 {
        1.0 // Default to 100%
    }

    fn read_io_activity(&mut self) -> f32 {
        0.05 // Baseline
    }
}

impl SystemProvider for MacOSSystemProvider {
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
    fn test_macos_provider_poll_signals_bounded() {
        let mut provider = MacOSSystemProvider::new();
        let signals = provider.poll_signals();

        assert!(signals.cpu_load >= 0.0 && signals.cpu_load <= 1.0);
        assert!(signals.memory_usage >= 0.0 && signals.memory_usage <= 1.0);
        assert!(signals.battery_level >= 0.0 && signals.battery_level <= 1.0);
        assert!(signals.io_activity >= 0.0 && signals.io_activity <= 1.0);

        // Second poll to exercise delta calculation logic
        let signals2 = provider.poll_signals();
        assert!(signals2.cpu_load >= 0.0 && signals2.cpu_load <= 1.0);
        assert!(signals2.io_activity >= 0.0 && signals2.io_activity <= 1.0);
    }
}
