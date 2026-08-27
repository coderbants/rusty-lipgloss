//! Cleanroom Rust port of upstream Go source files: `query.go` and `terminal.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <user-docs>
//! Terminal background color detection. Like upstream, the OSC 11 query runs
//! with the input in raw mode (to avoid echoing the response), and
//! `has_dark_background` defaults to `true` (dark) whenever detection fails or
//! the input/output is not a terminal.
//! </user-docs>

use std::io::{IsTerminal, Read, Write};
use std::time::Duration;

#[cfg(unix)]
use nix::poll::{poll, PollFd, PollFlags, PollTimeout};
#[cfg(unix)]
use nix::sys::termios::{cfmakeraw, tcgetattr, tcsetattr, SetArg, Termios};
#[cfg(unix)]
use std::os::fd::AsFd;

use crate::color::{is_dark_color, Color};

const DEFAULT_QUERY_TIMEOUT: Duration = Duration::from_secs(2);

/// Queries the terminal for its background color by writing an OSC 11 request
/// and reading the response. Returns `None` if the terminal does not respond or
/// is not a terminal.
///
/// The input is placed in raw mode for the duration of the query so the
/// terminal does not echo the response.
pub fn background_color(_in: &mut dyn Read, out: &mut dyn Write) -> Option<Color> {
    #[cfg(unix)]
    {
        background_color_with_terminal(_in, out, std::io::stdin(), DEFAULT_QUERY_TIMEOUT)
    }
    #[cfg(not(unix))]
    {
        // Raw-mode querying is not supported on this platform; fall back to
        // environment-based detection.
        let _ = (_in, out);
        None
    }
}

#[cfg(unix)]
fn background_color_with_terminal<F: AsFd>(
    input: &mut dyn Read,
    out: &mut dyn Write,
    terminal: F,
    timeout: Duration,
) -> Option<Color> {
    // Write the OSC 11 background color query plus a device attributes
    // request, then read the response with raw mode enabled on the terminal.
    let mut raw = RawMode::enter(terminal)?;
    let query = "\x1b]11;?\x07\x1b[c";
    if out.write_all(query.as_bytes()).is_err() {
        return None;
    }
    let _ = out.flush();

    let mut buf = [0u8; 256];
    let deadline = std::time::Instant::now() + timeout;
    let mut acc = String::new();
    loop {
        let remaining = deadline.saturating_duration_since(std::time::Instant::now());
        if remaining.is_zero() {
            return None;
        }
        if !raw.poll_read(remaining.min(Duration::from_millis(200))) {
            continue;
        }
        match input.read(&mut buf) {
            Ok(0) | Err(_) => return None,
            Ok(n) => {
                acc.push_str(&String::from_utf8_lossy(&buf[..n]));
                if acc.contains('\x07') {
                    return parse_os11_response(&acc);
                }
            }
        }
    }
}

/// <upstream-comment>HasDarkBackground detects whether the terminal has a light or dark
/// background.
///
/// By default, this function will return true if it encounters an error.</upstream-comment>
pub fn has_dark_background() -> bool {
    // Best-effort environment-based detection first: some terminals expose
    // COLORFGBG which encodes the default background color.
    if let Ok(colorfgbg) = std::env::var("COLORFGBG") {
        // COLORFGBG is "fg;bg" with color IDs 0-7 (dark) or 8-15 (light).
        let parts: Vec<&str> = colorfgbg.split(';').collect();
        if let Some(bg) = parts.last() {
            if let Ok(id) = bg.parse::<u8>() {
                return id < 8;
            }
        }
    }

    // Only attempt a terminal query when both input and output are TTYs.
    if !stdin_is_tty() || !stdout_is_tty() {
        return true;
    }

    // Fall back to a terminal query; on any failure, default to dark (true),
    // matching upstream behavior.
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut r = stdin.lock();
    let mut w = stdout.lock();
    match background_color(&mut r, &mut w) {
        Some(bg) => is_dark_color(&bg),
        None => true,
    }
}

fn stdin_is_tty() -> bool {
    std::io::stdin().is_terminal()
}

fn stdout_is_tty() -> bool {
    std::io::stdout().is_terminal()
}

/// A raw-mode guard for the terminal input used during queries.
#[cfg(unix)]
struct RawMode<F: AsFd> {
    terminal: F,
    original: Termios,
    restored: bool,
}

#[cfg(unix)]
impl<F: AsFd> RawMode<F> {
    fn enter(terminal: F) -> Option<RawMode<F>> {
        let original = tcgetattr(&terminal).ok()?;
        let mut raw = original.clone();
        cfmakeraw(&mut raw);
        tcsetattr(&terminal, SetArg::TCSANOW, &raw).ok()?;
        Some(RawMode {
            terminal,
            original,
            restored: false,
        })
    }

    fn restore(&mut self) -> bool {
        if self.restored {
            return true;
        }
        self.restored = tcsetattr(&self.terminal, SetArg::TCSANOW, &self.original).is_ok();
        self.restored
    }

    /// Polls the input for readability until `timeout` elapses.
    fn poll_read(&mut self, timeout: Duration) -> bool {
        let mut fds = [PollFd::new(self.terminal.as_fd(), PollFlags::POLLIN)];
        let Ok(timeout) = PollTimeout::try_from(timeout) else {
            return false;
        };
        poll(&mut fds, timeout).is_ok_and(|ready| {
            ready > 0
                && fds[0]
                    .revents()
                    .is_some_and(|events| events.contains(PollFlags::POLLIN))
        })
    }
}

#[cfg(unix)]
impl<F: AsFd> Drop for RawMode<F> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

/// Parses an OSC 11 response of the form `\x1b]11;rgb:0000/ffff/0000\x07`.
fn parse_os11_response(s: &str) -> Option<Color> {
    for line in s.split('\x1b') {
        if !line.starts_with("]11;") {
            continue;
        }
        let payload = line.trim_start_matches("]11;");
        let payload = payload.split('\x07').next().unwrap_or(payload);
        let parts: Vec<&str> = payload.split(';').collect();
        if parts.is_empty() {
            continue;
        }
        let col = parts[parts.len() - 1];
        // Accept either `rgb:RRRR/GGGG/BBBB` or `#RRGGBB` forms.
        if let Some(rgb) = col.strip_prefix("rgb:") {
            let channels: Vec<&str> = rgb.split('/').collect();
            if channels.len() != 3 {
                continue;
            }
            let parse = |c: &str| -> Option<u8> {
                let hex = &c[..2.min(c.len())];
                u8::from_str_radix(hex, 16).ok()
            };
            let r = parse(channels[0])?;
            let g = parse(channels[1])?;
            let b = parse(channels[2])?;
            return Some(Color::TrueColor { r, g, b });
        } else if let Some(hex) = col.strip_prefix('#') {
            let mut hex = hex.to_string();
            while hex.len() < 6 {
                hex.push('0');
            }
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some(Color::TrueColor { r, g, b });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    use nix::pty::openpty;
    #[cfg(unix)]
    use nix::sys::termios::LocalFlags;
    #[cfg(unix)]
    use nix::unistd::{dup, write};
    #[cfg(unix)]
    use std::fs::File;

    #[cfg(unix)]
    struct FailingReader;

    #[cfg(unix)]
    impl Read for FailingReader {
        fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("controlled read failure"))
        }
    }

    #[cfg(unix)]
    struct FailingWriter;

    #[cfg(unix)]
    impl Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("controlled write failure"))
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[cfg(unix)]
    fn assert_terminal_restored<F: AsFd>(terminal: &F, expected: &Termios) {
        let actual = tcgetattr(terminal).expect("restored termios must be readable");
        let durable_local_flags = |mut flags: LocalFlags| {
            flags.remove(LocalFlags::PENDIN);
            flags
        };
        assert_eq!(actual.input_flags, expected.input_flags);
        assert_eq!(actual.output_flags, expected.output_flags);
        assert_eq!(actual.control_flags, expected.control_flags);
        assert_eq!(
            durable_local_flags(actual.local_flags),
            durable_local_flags(expected.local_flags)
        );
        assert_eq!(actual.control_chars, expected.control_chars);
    }

    #[test]
    fn test_terminal_detection_uses_safe_standard_io_contracts() {
        let _stdin = stdin_is_tty();
        let _stdout = stdout_is_tty();
    }

    #[cfg(unix)]
    #[test]
    fn test_raw_mode_restores_explicitly_and_on_drop() {
        let explicit_pty = openpty(None, None).expect("PTY must open for explicit restoration");
        let explicit_inspection =
            dup(&explicit_pty.slave).expect("slave descriptor must duplicate");
        let explicit_original = tcgetattr(&explicit_inspection).expect("termios must be readable");
        let mut explicit_raw = RawMode::enter(explicit_pty.slave).expect("raw mode must start");
        assert_ne!(
            tcgetattr(&explicit_inspection).expect("raw termios must be readable"),
            explicit_original
        );
        assert!(explicit_raw.restore());
        assert_terminal_restored(&explicit_inspection, &explicit_original);

        let drop_pty = openpty(None, None).expect("PTY must open for drop restoration");
        let drop_inspection = dup(&drop_pty.slave).expect("slave descriptor must duplicate");
        let drop_original = tcgetattr(&drop_inspection).expect("termios must be readable");
        let drop_raw = RawMode::enter(drop_pty.slave).expect("raw mode must start");
        assert_ne!(
            tcgetattr(&drop_inspection).expect("raw termios must be readable"),
            drop_original
        );
        drop(drop_raw);
        assert_terminal_restored(&drop_inspection, &drop_original);
    }

    #[cfg(unix)]
    #[test]
    fn test_poll_read_covers_readable_timeout_and_invalid_timeout() {
        let readable_pty = openpty(None, None).expect("PTY must open for readable polling");
        let mut readable_raw = RawMode::enter(readable_pty.slave).expect("raw mode must start");
        write(&readable_pty.master, b"x").expect("PTY master must accept input");
        assert!(readable_raw.poll_read(Duration::from_millis(50)));

        let timeout_pty = openpty(None, None).expect("PTY must open for timeout polling");
        let mut timeout_raw = RawMode::enter(timeout_pty.slave).expect("raw mode must start");
        assert!(!timeout_raw.poll_read(Duration::from_millis(1)));
        assert!(!timeout_raw.poll_read(Duration::MAX));
    }

    #[cfg(unix)]
    #[test]
    fn test_background_query_restores_after_success_and_failures() {
        let success_pty = openpty(None, None).expect("PTY must open for successful query");
        let success_inspection = dup(&success_pty.slave).expect("slave descriptor must duplicate");
        let success_reader = dup(&success_pty.slave).expect("reader descriptor must duplicate");
        let success_original = tcgetattr(&success_inspection).expect("termios must be readable");
        write(&success_pty.master, b"\x1b]11;rgb:ffff/0000/0000\x07\n")
            .expect("PTY master must accept a terminal response");
        let mut reader = File::from(success_reader);
        let mut output = Vec::new();
        assert_eq!(
            background_color_with_terminal(
                &mut reader,
                &mut output,
                success_pty.slave,
                Duration::from_millis(50),
            ),
            Some(Color::TrueColor { r: 255, g: 0, b: 0 })
        );
        assert_terminal_restored(&success_inspection, &success_original);

        let write_pty = openpty(None, None).expect("PTY must open for write failure");
        let write_inspection = dup(&write_pty.slave).expect("slave descriptor must duplicate");
        let write_original = tcgetattr(&write_inspection).expect("termios must be readable");
        assert_eq!(
            background_color_with_terminal(
                &mut std::io::empty(),
                &mut FailingWriter,
                write_pty.slave,
                Duration::from_millis(10),
            ),
            None
        );
        assert_terminal_restored(&write_inspection, &write_original);

        let read_pty = openpty(None, None).expect("PTY must open for read failure");
        let read_inspection = dup(&read_pty.slave).expect("slave descriptor must duplicate");
        let read_original = tcgetattr(&read_inspection).expect("termios must be readable");
        write(&read_pty.master, b"x").expect("PTY master must accept input");
        assert_eq!(
            background_color_with_terminal(
                &mut FailingReader,
                &mut Vec::new(),
                read_pty.slave,
                Duration::from_millis(50),
            ),
            None
        );
        assert_terminal_restored(&read_inspection, &read_original);

        let timeout_pty = openpty(None, None).expect("PTY must open for timeout");
        let timeout_inspection = dup(&timeout_pty.slave).expect("slave descriptor must duplicate");
        let timeout_original = tcgetattr(&timeout_inspection).expect("termios must be readable");
        assert_eq!(
            background_color_with_terminal(
                &mut std::io::empty(),
                &mut Vec::new(),
                timeout_pty.slave,
                Duration::from_millis(2),
            ),
            None
        );
        assert_terminal_restored(&timeout_inspection, &timeout_original);
    }

    #[test]
    fn test_parse_os11_response() {
        let s = "\x1b]11;rgb:ffff/0000/0000\x07\x1b[c";
        let c = parse_os11_response(s);
        assert_eq!(c, Some(Color::TrueColor { r: 255, g: 0, b: 0 }));
    }

    #[test]
    fn test_parse_hex_response() {
        let s = "\x1b]11;#00ff00\x07";
        let c = parse_os11_response(s);
        assert_eq!(c, Some(Color::TrueColor { r: 0, g: 255, b: 0 }));
    }
}
