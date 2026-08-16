//! Reactive system signal acquisition and provider adapters.

pub mod linux;
pub mod provider;
pub mod signals;

pub use linux::LinuxSystemProvider;
pub use provider::{MockSystemProvider, SystemProvider};
pub use signals::SystemSignals;

/// Creates the platform-appropriate system provider (or fallback mock provider).
pub fn default_system_provider() -> Box<dyn SystemProvider> {
    #[cfg(target_os = "linux")]
    {
        Box::new(LinuxSystemProvider::default())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Box::new(MockSystemProvider::new(SystemSignals::default()))
    }
}
