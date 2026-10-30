// A poly-fill for `lazy_cell`
// Replace with std::sync::LazyLock when https://github.com/rust-lang/rust/issues/109736 is stabilized.

// This isn't used on every platform, which can come up as dead code warnings.
#![allow(dead_code)]

use std::ops::Deref;
use std::sync::OnceLock;
use std::time::Duration;
#[cfg(not(web_platform))]
use std::time::Instant;

#[cfg(web_platform)]
use web_time::Instant;

pub(crate) struct Lazy<T> {
    cell: OnceLock<T>,
    init: fn() -> T,
}

impl<T> Lazy<T> {
    pub const fn new(f: fn() -> T) -> Self {
        Self { cell: OnceLock::new(), init: f }
    }
}

impl<T> Deref for Lazy<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &'_ T {
        self.cell.get_or_init(self.init)
    }
}

/// An input older than this by the platform's clock is not trusted: its clock does not share a base
/// with ours (X11 and Wayland leave the base unspecified), so it counts as happening now.
const MAX_INPUT_AGE_SECS: f64 = 10.0;

/// The instant an input happened, given how many seconds before now the platform's own clock says it
/// did.
pub(crate) fn input_time(age_secs: f64) -> Instant {
    let now = Instant::now();
    if (0.0..=MAX_INPUT_AGE_SECS).contains(&age_secs) {
        now.checked_sub(Duration::from_secs_f64(age_secs)).unwrap_or(now)
    } else {
        now
    }
}

/// The instant an input happened, from two readings of one wrapping millisecond clock: the input's
/// and the current time's (Windows message time, X11 and Wayland event time).
pub(crate) fn input_time_from_millis(event_ms: u32, now_ms: u32) -> Instant {
    input_time(f64::from(now_ms.wrapping_sub(event_ms)) / 1000.0)
}
