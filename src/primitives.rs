use macroquad::{
    color::{Color, GREEN},
    shapes::{draw_poly, draw_rectangle},
};

pub fn draw_rectangle_re(x: f32, y: f32, w: f32, h: f32, color: Color, radius: f32) {
    // big boi
    draw_rectangle(x + radius, y, w, h, color);

    // top left, then go clockwise
    draw_poly(x + radius, y + radius, 64, radius, 0.0, color);
    draw_poly(x + w + radius, y + radius, 64, radius, 0.0, color);
    draw_poly(x + w + radius, y + h - radius, 64, radius, 0.0, color);
    draw_poly(x + radius, y + h - radius, 64, radius, 0.0, color);

    // two small rects
    draw_rectangle(x, y + radius, radius, h - (2.0 * radius), color);
    draw_rectangle(
        x + w + radius,
        y + radius,
        radius,
        h - (2.0 * radius),
        color,
    )
}
