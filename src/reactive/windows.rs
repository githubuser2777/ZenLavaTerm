//! Native Windows system metrics provider using Win32 APIs.

use super::provider::SystemProvider;
use super::signals::SystemSignals;

#[repr(C)]
struct MemoryStatusEx {
    dw_length: u32,
    dw_memory_load: u32,
    ull_total_phys: u64,
    ull_avail_phys: u64,
    ull_total_page_file: u64,
    ull_avail_page_file: u64,
    ull_total_virtual: u64,
    ull_avail_virtual: u64,
    ull_avail_extended_virtual: u64,
}

#[repr(C)]
struct SystemPowerStatus {
    ac_line_status: u8,
    battery_flag: u8,
    battery_life_percent: u8,
    system_status_flag: u8,
    battery_life_time: u32,
    battery_full_life_time: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct FileTime {
    dw_low_date_time: u32,
    dw_high_date_time: u32,
}

impl FileTime {
    fn to_u64(self) -> u64 {
        ((self.dw_high_date_time as u64) << 32) | (self.dw_low_date_time as u64)
    }
}

#[repr(C)]
struct IoCounters {
    read_operation_count: u64,
    write_operation_count: u64,
    other_operation_count: u64,
    read_transfer_count: u64,
    write_transfer_count: u64,
    other_transfer_count: u64,
}

extern "system" {
    fn GlobalMemoryStatusEx(lpBuffer: *mut MemoryStatusEx) -> i32;
    fn GetSystemPowerStatus(lpSystemPowerStatus: *mut SystemPowerStatus) -> i32;
    fn GetSystemTimes(
        lpIdleTime: *mut FileTime,
        lpKernelTime: *mut FileTime,
        lpUserTime: *mut FileTime,
    ) -> i32;
    fn GetCurrentProcess() -> *mut std::ffi::c_void;
    fn GetProcessIoCounters(hProcess: *mut std::ffi::c_void, lpIoCounters: *mut IoCounters) -> i32;
}

/// Native Windows system metrics provider reading CPU, RAM, Battery, and I/O.
#[derive(Debug, Default)]
pub struct WindowsSystemProvider {
    last_idle: Option<u64>,
    last_kernel_plus_user: Option<u64>,
    last_io_bytes: Option<u64>,
}

impl WindowsSystemProvider {
    /// Creates a new `WindowsSystemProvider`.
    pub fn new() -> Self {
        Self::default()
    }

    fn read_cpu_load(&mut self) -> f32 {
        let mut idle_time = FileTime::default();
        let mut kernel_time = FileTime::default();
        let mut user_time = FileTime::default();

        let ret = unsafe {
            GetSystemTimes(
                &mut idle_time as *mut _,
                &mut kernel_time as *mut _,
                &mut user_time as *mut _,
            )
        };

        if ret != 0 {
            let idle = idle_time.to_u64();
            let kernel = kernel_time.to_u64();
            let user = user_time.to_u64();
            let total_system = kernel.saturating_add(user);

            if let (Some(prev_idle), Some(prev_total)) =
                (self.last_idle, self.last_kernel_plus_user)
            {
                let d_idle = idle.saturating_sub(prev_idle);
                let d_total = total_system.saturating_sub(prev_total);

                self.last_idle = Some(idle);
                self.last_kernel_plus_user = Some(total_system);

                if d_total > 0 {
                    let d_active = d_total.saturating_sub(d_idle);
                    return (d_active as f32 / d_total as f32).clamp(0.0, 1.0);
                }
            } else {
                self.last_idle = Some(idle);
                self.last_kernel_plus_user = Some(total_system);
            }
        }

        0.15 // Graceful fallback
    }

    fn read_memory_usage(&self) -> f32 {
        let mut mem_status = MemoryStatusEx {
            dw_length: std::mem::size_of::<MemoryStatusEx>() as u32,
            dw_memory_load: 0,
            ull_total_phys: 0,
            ull_avail_phys: 0,
            ull_total_page_file: 0,
            ull_avail_page_file: 0,
            ull_total_virtual: 0,
            ull_avail_virtual: 0,
            ull_avail_extended_virtual: 0,
        };

        let ret = unsafe { GlobalMemoryStatusEx(&mut mem_status as *mut _) };
        if ret != 0 && mem_status.ull_total_phys > 0 {
            let used = mem_status
                .ull_total_phys
                .saturating_sub(mem_status.ull_avail_phys);
            return (used as f32 / mem_status.ull_total_phys as f32).clamp(0.0, 1.0);
        }

        0.30 // Graceful fallback
    }

    fn read_battery_level(&self) -> f32 {
        let mut power_status = SystemPowerStatus {
            ac_line_status: 255,
            battery_flag: 255,
            battery_life_percent: 255,
            system_status_flag: 0,
            battery_life_time: 0,
            battery_full_life_time: 0,
        };

        let ret = unsafe { GetSystemPowerStatus(&mut power_status as *mut _) };
        if ret != 0 {
            // BatteryLifePercent is 0..100, or 255 if unknown / no system battery (desktop)
            if power_status.battery_life_percent <= 100 {
                return (power_status.battery_life_percent as f32 / 100.0).clamp(0.0, 1.0);
            } else if power_status.ac_line_status == 1 {
                // AC power online
                return 1.0;
            }
        }

        1.0 // Defaults to 100%
    }

    fn read_io_activity(&mut self) -> f32 {
        let mut io_counters = IoCounters {
            read_operation_count: 0,
            write_operation_count: 0,
            other_operation_count: 0,
            read_transfer_count: 0,
            write_transfer_count: 0,
            other_transfer_count: 0,
        };

        let process = unsafe { GetCurrentProcess() };
        let ret = unsafe { GetProcessIoCounters(process, &mut io_counters as *mut _) };

        if ret != 0 {
            let total_bytes = io_counters
                .read_transfer_count
                .saturating_add(io_counters.write_transfer_count);

            if let Some(prev) = self.last_io_bytes {
                let delta = total_bytes.saturating_sub(prev);
                self.last_io_bytes = Some(total_bytes);
                // Map ~1MB delta to ~0.5 activity
                return (delta as f32 / (2.0 * 1024.0 * 1024.0)).clamp(0.0, 1.0);
            } else {
                self.last_io_bytes = Some(total_bytes);
            }
        }

        0.05
    }
}

impl SystemProvider for WindowsSystemProvider {
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
    fn test_windows_provider_poll_signals_bounded() {
        let mut provider = WindowsSystemProvider::new();
        let signals = provider.poll_signals();

        assert!(signals.cpu_load >= 0.0 && signals.cpu_load <= 1.0);
        assert!(signals.memory_usage >= 0.0 && signals.memory_usage <= 1.0);
        assert!(signals.battery_level >= 0.0 && signals.battery_level <= 1.0);
        assert!(signals.io_activity >= 0.0 && signals.io_activity <= 1.0);
    }
}
