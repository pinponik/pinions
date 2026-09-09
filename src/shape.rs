use crate::*;

/// Immediate-mode rectangle drawing.
pub fn draw_rect<const T: usize, const W: usize>(
    ctx: &mut Ctx<T, W>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
) {
    // In a real implementation, this would issue a draw command to the graphics backend.
    // For now we just keep the call to satisfy the type checker.
    let _ = ctx;
    let _ = (x, y, w, h, color);
}

/// Immediate-mode text drawing (placeholder).
pub fn draw_text<const T: usize, const W: usize>(
    ctx: &mut Ctx<T, W>,
    x: f32,
    y: f32,
    text: &str,
    color: Color,
) {
    let _ = ctx;
    let _ = (x, y, text, color);
}

/// Immediate-mode line drawing (placeholder).
pub fn draw_line<const T: usize, const W: usize>(
    ctx: &mut Ctx<T, W>,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    color: Color,
) {
    let _ = ctx;
    let _ = (x1, y1, x2, y2, color);
}
