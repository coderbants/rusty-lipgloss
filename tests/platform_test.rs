#![cfg(windows)]

use rusty_lipgloss::platform::enable_legacy_windows_ansi;

#[test]
fn windows_public_ansi_entrypoint_is_safe_noop() {
    let file = std::fs::File::open("NUL").expect("Windows NUL device is available");
    enable_legacy_windows_ansi(&file);
}
