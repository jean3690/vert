/// Backend abstraction — detects available conversion backends (native Rust,
/// external CLI tools) and reports availability so the frontend can filter
/// out unavailable conversions.
///
/// # Current backends
///
/// | Backend | Kind | Detection |
/// |---------|------|-----------|
/// | Native (built-in) | Built-in | Always available |
/// | Pandoc | External CLI | `which pandoc` |
/// | FFmpeg | External CLI | `which ffmpeg` |
///
/// The `Native` backend is always available. External backends are detected
/// once on first access and cached.

use std::sync::{Mutex, OnceLock};

static DETECTION_CACHE: OnceLock<Mutex<std::collections::HashMap<&'static str, bool>>> =
    OnceLock::new();

/// A conversion backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Built-in Rust converters — always available.
    Native,
    /// Pandoc document converter (http://pandoc.org).
    Pandoc,
    /// FFmpeg multimedia framework (http://ffmpeg.org).
    Ffmpeg,
}

impl Backend {
    /// All known backends in priority order.
    pub const ALL: &[Backend] = &[Backend::Native, Backend::Pandoc, Backend::Ffmpeg];

    /// Human-readable display name.
    pub fn display_name(&self) -> &'static str {
        match self {
            Backend::Native => "Built-in",
            Backend::Pandoc => "Pandoc",
            Backend::Ffmpeg => "FFmpeg",
        }
    }

    /// Install hint shown when the tool is missing.
    pub fn install_hint(&self) -> Option<&'static str> {
        match self {
            Backend::Native => None,
            Backend::Pandoc => Some("Install Pandoc: https://pandoc.org/installing.html"),
            Backend::Ffmpeg => Some("Install FFmpeg: https://ffmpeg.org/download.html"),
        }
    }

    /// Returns `true` if this backend is available on the current system.
    ///
    /// `Native` is always available; external backends are lazily detected
    /// via `which` and cached.
    pub fn available(&self) -> bool {
        match self {
            Backend::Native => true,
            Backend::Pandoc => Self::lazy_detect("pandoc"),
            Backend::Ffmpeg => Self::lazy_detect("ffmpeg"),
        }
    }

    /// List all currently available backends.
    pub fn available_backends() -> Vec<Backend> {
        Self::ALL.iter().filter(|b| b.available()).copied().collect()
    }

    // ── lazy detection ──

    fn lazy_detect(executable: &'static str) -> bool {
        let cache = DETECTION_CACHE
            .get_or_init(|| Mutex::new(std::collections::HashMap::new()));
        let mut map = cache.lock().expect("detection cache lock");
        *map.entry(executable).or_insert_with(|| which::which(executable).is_ok())
    }

    /// Reset the detection cache (useful for tests).
    #[cfg(test)]
    pub fn reset_cache() {
        if let Some(cache) = DETECTION_CACHE.get() {
            cache.lock().expect("detection cache lock").clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_always_available() {
        assert!(Backend::Native.available());
    }

    #[test]
    fn available_backends_includes_native() {
        let backends = Backend::available_backends();
        assert!(backends.contains(&Backend::Native));
    }

    #[test]
    fn detection_caches_result() {
        Backend::reset_cache();
        // First access populates, second reads cache — just check no panic.
        let _ = Backend::Pandoc.available();
        let _ = Backend::Pandoc.available();
    }
}
