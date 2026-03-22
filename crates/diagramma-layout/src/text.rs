/// Font metrics for text measurement.
#[derive(Debug, Clone, Copy)]
pub struct FontMetrics {
    /// Character width at 14px font size (pixels).
    pub char_width_14px: f64,
    /// Character width at 12px font size (pixels).
    pub char_width_12px: f64,
    /// Horizontal padding inside box (pixels).
    pub h_padding: f64,
    /// Vertical padding inside box (pixels).
    pub v_padding: f64,
}

impl FontMetrics {
    /// Default monospace metrics: ~8px per char at 14px, ~7px at 12px.
    #[must_use]
    pub fn default_monospace() -> Self {
        Self {
            char_width_14px: 8.0,
            char_width_12px: 7.0,
            h_padding: 12.0,
            v_padding: 8.0,
        }
    }

    /// Estimate width for a string at 14px font.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn estimate_width_14px(&self, text: &str) -> f64 {
        text.len() as f64 * self.char_width_14px
    }

    /// Estimate width for a string at 12px font.
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn estimate_width_12px(&self, text: &str) -> f64 {
        text.len() as f64 * self.char_width_12px
    }
}

/// Calculate box dimensions from title and optional subtitle.
///
/// # Arguments
/// * `title` - Primary text (14px font)
/// * `subtitle` - Optional secondary text (12px font)
/// * `metrics` - Font metrics for measurement
///
/// # Returns
/// `(width, height)` in pixels.
#[must_use]
pub fn box_size(title: &str, subtitle: Option<&str>, metrics: &FontMetrics) -> (f64, f64) {
    let title_width = metrics.estimate_width_14px(title) + 2.0 * metrics.h_padding;
    let subtitle_width = subtitle.map_or(0.0, |s| {
        metrics.estimate_width_12px(s) + 2.0 * metrics.h_padding
    });

    let width = title_width.max(subtitle_width);
    let height = if subtitle.is_some() {
        2.0 * metrics.v_padding + 14.0 + 4.0 + 12.0
    } else {
        2.0 * metrics.v_padding + 14.0
    };

    (width, height)
}
