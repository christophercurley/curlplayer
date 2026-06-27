use macroquad::prelude::*;

use crate::{ComponentStore, painter::Painter};

// ----------------------------------------------
// Curl Menu Bar
// ----------------------------------------------
pub struct CurlMenuBar {
    menus: Vec<CurlMenu>,
    color: Color,
}

#[derive(Debug)]
pub struct CurlMenu {
    name: &'static str,
    options: Vec<String>,
}

impl CurlMenu {
    pub fn new(name: &'static str, options: Vec<String>) -> Self {
        Self { name, options }
    }
}

impl CurlMenuBar {
    pub fn new(menus: Vec<CurlMenu>, color: Color) -> Self {
        Self { menus, color }
    }
}

pub fn build_menu_bar() -> CurlMenuBar {
    let file_menu_options = vec!["New".to_string(), "Load".to_string(), "Quit".to_string()];
    let edit_menu_options = vec!["Undo".to_string(), "Redo".to_string()];
    let view_menu_options = vec!["Debug Console".to_string()];
    let help_menu_options = vec!["Welcome".to_string(), "About".to_string()];

    let file = CurlMenu::new("File", file_menu_options);
    let edit = CurlMenu::new("Edit", edit_menu_options);
    let view = CurlMenu::new("View", view_menu_options);
    let help = CurlMenu::new("Help", help_menu_options);

    let menu_bar = CurlMenuBar::new(vec![file, edit, view, help], Color::new(0.0, 0.0, 0.0, 1.0));

    menu_bar
}

fn compute_menu_bar_geo(menu_bar: &CurlMenuBar, font_size: f32) {}

fn draw_menu_bar(
    x_offset: f32,
    y_offset: f32,
    height: f32,
    menu_bar: &CurlMenuBar,
    font_size: f32,
    painter: &Painter,
) {
    painter.rect(
        x_offset,
        y_offset,
        screen_width() / painter.scale(),
        height,
        LIGHTGRAY,
    );

    let mut mx_offset: f32 = x_offset;
    let my_offset: f32 = y_offset;
    let mut menu_width: f32;
    let menu_pad: f32 = 20.0;
    let text_height_offset: f32 = 0.75;

    for menu in &menu_bar.menus {
        let text_dims = painter.measure(menu.name, font_size);
        menu_width = text_dims.width + menu_pad;

        painter.rect(
            mx_offset + (height * 0.1),
            my_offset + (height * 0.1),
            menu_width,
            height * 0.8,
            WHITE,
        );

        painter.text(
            menu.name,
            mx_offset + (height * 0.1) + (menu_pad / 2.0),
            my_offset + (height * text_height_offset),
            font_size,
            BLACK,
        );

        mx_offset = mx_offset + (height * 0.1) + menu_width;
    }

    // draw mouse coords
    let (mouse_x, mouse_y) = mouse_position();

    let mouse_coords = format!("[ {}, {} ]", mouse_x, mouse_y);

    painter.text(
        &mouse_coords,
        screen_width() / painter.scale() - 100.0,
        my_offset + (height * text_height_offset),
        font_size,
        BLACK,
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        println!("MOUSE PRESS at [ {}, {} ]!!!!", mouse_x, mouse_y);
    }
}

// ----------------------------------------------
// UI Render Engine
// ----------------------------------------------

pub fn render_ui(
    menu_bar: &CurlMenuBar,
    menu_font_size: f32,
    painter: &Painter,
    component_store: &mut ComponentStore,
) {
    let rxo = screen_width() / 2.0;
    let ryo = screen_height() / 2.0;
    let rw = 200.0;
    let rh = 75.0;

    component_store.clear();

    clear_background(WHITE);

    draw_rectangle(rxo - rw / 2.0, ryo - rh / 2.0, rw, rh, GREEN);

    compute_menu_bar_geo(&menu_bar, menu_font_size);
    draw_menu_bar(0.0, 0.0, 25.0, &menu_bar, menu_font_size, &painter);

    // draw crosshair
    draw_rectangle(rxo, 0.0, 1.0, screen_height(), RED); // vertical half
    draw_rectangle(0.0, ryo, screen_width(), 1.0, RED); // horiz half
}
