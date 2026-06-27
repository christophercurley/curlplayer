use macroquad::prelude::*;

/// Owns the render context (font + DPI scale) and converts
/// logical coordinates -> physical pixels at the boundary.
/// All screen code works in logical units and never sees `scale`.
pub struct Painter {
    font: Font,
    scale: f32,
}

impl Painter {
    pub fn new(font: Font, scale: f32) -> Self {
        Self { font, scale }
    }

    /// logical font size -> physical px for crisp rasterization
    fn px(&self, logical: f32) -> f32 {
        logical * self.scale
    }

    /// Draw text positioned in LOGICAL coords at a LOGICAL font size.
    pub fn text(&self, s: &str, x: f32, y: f32, size: f32, color: Color) {
        draw_text_ex(
            s,
            (x * self.scale).floor(),
            (y * self.scale).floor(),
            TextParams {
                font: Some(&self.font),
                font_size: self.px(size) as u16, // rasterize at physical size = sharp
                color,
                ..Default::default()
            },
        );
    }

    /// Measure text in LOGICAL units (so layout math stays logical).
    pub fn measure(&self, s: &str, size: f32) -> TextDimensions {
        let d = measure_text(s, Some(&self.font), self.px(size) as u16, 1.0);
        TextDimensions {
            width: d.width / self.scale,
            height: d.height / self.scale,
            offset_y: d.offset_y / self.scale,
        }
    }

    /// Filled rect in LOGICAL coords.
    pub fn rect(&self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        draw_rectangle(
            x * self.scale,
            y * self.scale,
            w * self.scale,
            h * self.scale,
            color,
        );
    }

    /// Rect outline in LOGICAL coords.
    pub fn rect_lines(&self, x: f32, y: f32, w: f32, h: f32, thick: f32, color: Color) {
        draw_rectangle_lines(
            x * self.scale,
            y * self.scale,
            w * self.scale,
            h * self.scale,
            thick * self.scale,
            color,
        );
    }

    /// Logical screen dimensions (physical / scale).
    pub fn width(&self) -> f32 {
        screen_width() / self.scale
    }
    pub fn height(&self) -> f32 {
        screen_height() / self.scale
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}
