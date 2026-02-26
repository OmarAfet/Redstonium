use gpui::{Rgba, rgb};

pub struct Colors;

impl Colors {
    // Background
    pub fn background() -> Rgba { rgb(0x1e1e2e) }

    // Foreground (primary text)
    pub fn foreground() -> Rgba { rgb(0xcdd6f4) }

    // Muted (subtle backgrounds)
    pub fn muted() -> Rgba { rgb(0x313244) }

    // Muted foreground (secondary text)
    pub fn muted_foreground() -> Rgba { rgb(0xa6adc8) }

    // Accent (hover states, highlights)
    pub fn accent() -> Rgba { rgb(0x45475a) }

    // Sidebar
    pub fn sidebar() -> Rgba { rgb(0x181825) }

    // Border
    pub fn border() -> Rgba { rgb(0x313244) }

    // Secondary (avatar backgrounds, misc)
    pub fn secondary() -> Rgba { rgb(0x585b70) }
}
