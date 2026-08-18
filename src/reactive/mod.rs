//! Reactive system signal acquisition and provider adapters.

pub mod linux;
pub mod macos;
pub mod provider;
pub mod signals;
pub mod windows;

pub use linux::LinuxSystemProvider;
pub use macos::MacOSSystemProvider;
pub use provider::{MockSystemProvider, SystemProvider};
pub use signals::SystemSignals;
pub use windows::WindowsSystemProvider;

/// Creates the platform-appropriate system provider (or fallback mock provider).
pub fn default_system_provider() -> Box<dyn SystemProvider> {
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxSystemProvider::default())
    }
    #[cfg(target_os = "windows")]
    {
        Box::new(WindowsSystemProvider::default())
    }
    #[cfg(target_os = "macos")]
    {
        Box::new(MacOSSystemProvider::default())
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        Box::new(MockSystemProvider::new(SystemSignals::default()))
    }
}
