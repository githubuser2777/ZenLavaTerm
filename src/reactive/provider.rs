//! Provider interfaces and mock fixtures for system metric acquisition.

use super::signals::SystemSignals;

/// Trait implemented by system metric providers (Linux procfs, Windows, mock, etc.).
pub trait SystemProvider: Send {
    /// Polls the latest system signals. Returns normalized `SystemSignals`.
    fn poll_signals(&mut self) -> SystemSignals;
}

/// Deterministic mock system provider for testing.
#[derive(Debug, Clone)]
pub struct MockSystemProvider {
    pub signals: SystemSignals,
}

impl MockSystemProvider {
    /// Creates a new `MockSystemProvider` with given fixed signals.
    pub fn new(signals: SystemSignals) -> Self {
        Self { signals }
    }
}

impl SystemProvider for MockSystemProvider {
    fn poll_signals(&mut self) -> SystemSignals {
        self.signals
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_provider_returns_expected_signals() {
        let expected = SystemSignals::new(0.75, 0.50, 0.90, 0.20);
        let mut provider = MockSystemProvider::new(expected);
        let polled = provider.poll_signals();
        assert_eq!(polled, expected);
    }
}
