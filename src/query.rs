//! Cleanroom Rust port of upstream Go source files: `query.go` and `terminal.go`
//! Upstream Target Tag / Version: `v2.0.5`
//!
//! <public-docs>
//! Terminal background color detection. Like upstream, the OSC 11 query runs
//! with the input in raw mode (to avoid echoing the response), and
//! `has_dark_background` defaults to `true` (dark) whenever detection fails or
//! the input/output is not a terminal.
//! </public-docs>

// Raw-mode terminal I/O (termios) is inherently unsafe; this module isolates
// the unsafe FFI calls required to query the terminal without echoing.
#![allow(unsafe_code)]

use std::io::{Read, Write};
use std::time::Duration;

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
        // Write the OSC 11 background color query plus a device attributes
        // request, then read the response with raw mode enabled on stdin.
        let fd = std::io::stdin().as_raw_fd_checked()?;
        let mut raw = RawMode::enter(fd)?;
        let query = "\x1b]11;?\x07\x1b[c";
        if out.write_all(query.as_bytes()).is_err() {
            let _ = raw.restore();
            return None;
        }
        let _ = out.flush();

        let mut buf = [0u8; 256];
        let deadline = std::time::Instant::now() + DEFAULT_QUERY_TIMEOUT;
        let mut acc = String::new();
        let mut result = None;
        loop {
            let remaining = deadline.saturating_duration_since(std::time::Instant::now());
            if remaining.is_zero() {
                break;
            }
            let ms = remaining.as_millis().min(200) as i32;
            if !raw.poll_read(ms) {
                continue;
            }
            match _in.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    acc.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if acc.contains('\x07') {
                        result = parse_os11_response(&acc);
                        break;
                    }
                }
            }
        }
        let _ = raw.restore();
        result
    }
    #[cfg(not(unix))]
    {
        // Raw-mode querying is not supported on this platform; fall back to
        // environment-based detection.
        let _ = (_in, out);
        None
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

#[cfg(unix)]
fn stdin_is_tty() -> bool {
    use std::os::unix::io::AsRawFd;
    unsafe { libc::isatty(std::io::stdin().as_raw_fd()) == 1 }
}

#[cfg(unix)]
fn stdout_is_tty() -> bool {
    use std::os::unix::io::AsRawFd;
    unsafe { libc::isatty(std::io::stdout().as_raw_fd()) == 1 }
}

#[cfg(not(unix))]
fn stdin_is_tty() -> bool {
    false
}

#[cfg(not(unix))]
fn stdout_is_tty() -> bool {
    false
}

#[cfg(unix)]
trait RawFdExt {
    fn as_raw_fd_checked(&self) -> Option<i32>;
}

#[cfg(unix)]
impl RawFdExt for std::io::Stdin {
    fn as_raw_fd_checked(&self) -> Option<i32> {
        use std::os::unix::io::AsRawFd;
        Some(self.as_raw_fd())
    }
}

/// A raw-mode guard for the terminal input used during queries.
#[cfg(unix)]
struct RawMode {
    fd: i32,
    original: libc::termios,
}

#[cfg(unix)]
impl RawMode {
    fn enter(fd: i32) -> Option<RawMode> {
        unsafe {
            let mut original: libc::termios = std::mem::zeroed();
            if libc::tcgetattr(fd, &mut original) != 0 {
                return None;
            }
            let mut raw = original;
            libc::cfmakeraw(&mut raw);
            if libc::tcsetattr(fd, libc::TCSANOW, &raw) != 0 {
                return None;
            }
            Some(RawMode { fd, original })
        }
    }

    fn restore(&mut self) -> i32 {
        unsafe { libc::tcsetattr(self.fd, libc::TCSANOW, &self.original) }
    }

    /// Polls the input fd for readability, returning after at most `ms`
    /// milliseconds. Returns true if data is available.
    fn poll_read(&mut self, ms: i32) -> bool {
        let mut fds = [libc::pollfd {
            fd: self.fd,
            events: libc::POLLIN,
            revents: 0,
        }];
        let ret = unsafe { libc::poll(fds.as_mut_ptr(), 1, ms) };
        ret > 0 && fds[0].revents & libc::POLLIN != 0
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
