//! Cleanroom Rust port of upstream Go source files: `ansi_unix.go` and `ansi_windows.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <user-docs>
//! Platform-specific ANSI enabling. Unix is a no-op. The Windows entry point
//! currently preserves the public API as a safe no-op until the native console
//! adapter is ported; profile-aware rendering remains available independently.
//! </user-docs>
//!
//! Internal maintainer note: this crate denies unsafe Rust, so native Win32
//! console mode changes must not be reintroduced as handwritten FFI. Add a
//! reviewed safe adapter before changing the Windows behavior.

/// <upstream-comment>EnableLegacyWindowsANSI enables support for ANSI color sequences in the
/// Windows default console (cmd.exe and the PowerShell application). Note that
/// this only works with Windows 10 and greater. Also note that Windows Terminal
/// supports colors by default.
///
/// On Unix systems this is a no-op.</upstream-comment>
#[cfg(not(windows))]
pub fn enable_legacy_windows_ansi(_f: &std::fs::File) {}

/// Enables legacy Windows ANSI output when a native safe adapter is available.
///
/// The current Windows port is intentionally a no-op because this crate denies
/// unsafe Rust and the native Win32 console adapter is not yet available.
/// Callers can still use profile-aware rendering to select deterministic ANSI,
/// reduced-color, or no-color output.
#[cfg(windows)]
pub fn enable_legacy_windows_ansi(_f: &std::fs::File) {}

#[cfg(all(test, windows))]
mod tests {
    use super::enable_legacy_windows_ansi;

    #[test]
    fn windows_ansi_entrypoint_is_safe_until_native_adapter_exists() {
        let file = std::fs::File::open("NUL").expect("Windows NUL device is available");
        enable_legacy_windows_ansi(&file);
    }
}
