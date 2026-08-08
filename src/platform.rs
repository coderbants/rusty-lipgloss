//! Cleanroom Rust port of upstream Go source files: `ansi_unix.go` and `ansi_windows.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Platform-specific ANSI enabling. On Windows, the legacy console must be
//! switched into virtual-terminal-processing mode; on Unix this is a no-op.
//! </public-docs>

/// <upstream-comment>EnableLegacyWindowsANSI enables support for ANSI color sequences in the
/// Windows default console (cmd.exe and the PowerShell application). Note that
/// this only works with Windows 10 and greater. Also note that Windows Terminal
/// supports colors by default.
///
/// On Unix systems this is a no-op.</upstream-comment>
#[cfg(not(windows))]
pub fn enable_legacy_windows_ansi(_f: &std::fs::File) {}

/// <upstream-comment>EnableLegacyWindowsANSI enables support for ANSI color sequences in the
/// Windows default console (cmd.exe and the PowerShell application).</upstream-comment>
#[cfg(windows)]
pub fn enable_legacy_windows_ansi(f: &std::fs::File) {
    // Use the standard library's handle to enable virtual terminal processing.
    // We perform the call via the console API; when it fails the function is a
    // no-op, matching upstream behavior.
    use std::os::windows::io::AsRawHandle;

    #[repr(u32)]
    #[allow(non_camel_case_types)]
    enum EnableVirtualTerminalProcessing {
        value = 0x0004,
    }

    extern "system" {
        fn GetConsoleMode(hConsoleHandle: usize, lpMode: *mut u32) -> i32;
        fn SetConsoleMode(hConsoleHandle: usize, dwMode: u32) -> i32;
    }

    let handle = f.as_raw_handle() as usize;
    let mut mode: u32 = 0;
    unsafe {
        if GetConsoleMode(handle, &mut mode) == 0 {
            return;
        }
        if mode & EnableVirtualTerminalProcessing::value as u32
            != EnableVirtualTerminalProcessing::value as u32
        {
            let vtpmode = mode | EnableVirtualTerminalProcessing::value as u32;
            if SetConsoleMode(handle, vtpmode) == 0 {
                return;
            }
        }
    }
}
