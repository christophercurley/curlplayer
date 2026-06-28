use macroquad::{miniquad::window::dpi_scale, prelude::*};

use crate::{
    Component, ComponentStore, Group, Shape,
    app::{self, AppState},
    primitives::draw_rectangle_re,
};

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

fn compute_menu_bar_geo(
    x_offset: f32,
    y_offset: f32,
    height: f32,
    menu_bar: &CurlMenuBar,
    font: &Font,
    font_size: u16,
    component_store: &mut ComponentStore,
    app_state: &AppState,
) {
    let mut mx_offset: f32 = x_offset;
    let my_offset: f32 = y_offset;
    let mut menu_width: f32;
    let menu_pad: f32 = 20.0;

    for menu in &menu_bar.menus {
        let text_dims = measure_text(menu.name, Some(font), font_size, 1.0);
        menu_width = text_dims.width + menu_pad;

        let mbc_group = Group::MenuBar;
        let mbc_shape = Shape::Rect {
            x: mx_offset + (height * 0.1),
            y: my_offset + (height * 0.1),
            w: menu_width,
            h: height * 0.8,
        };

        let menu_bar_component = Component::new(menu.name, mbc_group, mbc_shape);
        component_store.add(menu_bar_component);

        // compute panel dimensions if open
        if app_state.open_menu == Some(menu.name) {
            // mbpc = menu bar panel component
            let mbpc_name = menu.name;
            let mbpc_group = Group::MenuBarPanel;
            let mbpc_shape = Shape::RectRE {
                x: mx_offset,
                y: height,
                w: 125.0,
                h: 250.0,
                color: LIGHTGRAY,
                radius: 10.0,
            };

            let menu_bar_panel_component = Component::new(mbpc_name, mbpc_group, mbpc_shape);
            component_store.add(menu_bar_panel_component);
        }

        mx_offset = mx_offset + (height * 0.1) + menu_width;
    }
}

fn draw_menu_bar(
    x_offset: f32,
    y_offset: f32,
    height: f32,
    font: &Font,
    font_size: u16,
    component_store: &ComponentStore,
) {
    let menu_pad: f32 = 20.0;
    let text_height_offset: f32 = 0.65;

    draw_rectangle(x_offset, y_offset, screen_width(), height, WHITE);

    for component in component_store
        .components
        .iter()
        .filter(|c| c.group == Group::MenuBar)
    {
        if let Shape::Rect { x, y, w, h } = &component.shape {
            draw_rectangle(*x, *y, *w, *h, LIGHTGRAY);

            draw_text_ex(
                &component.name,
                *x + (menu_pad / 2.0),
                *y + (height * text_height_offset),
                TextParams {
                    font: Some(font),
                    font_size,
                    font_scale: 1.0,
                    font_scale_aspect: 1.0,
                    rotation: 0.0,
                    color: BLACK,
                },
            );
        }
    }

    for component in component_store
        .components
        .iter()
        .filter(|c| c.group == Group::MenuBarPanel)
    {
        if let Shape::RectRE {
            x,
            y,
            w,
            h,
            color,
            radius,
        } = &component.shape
        {
            draw_rectangle_re(*x, *y, *w, *h, *color, *radius);

            // draw_text_ex(
            //     &component.name,
            //     *x + (menu_pad / 2.0),
            //     *y + (height * text_height_offset),
            //     TextParams {
            //         font: Some(font),
            //         font_size,
            //         font_scale: 1.0,
            //         font_scale_aspect: 1.0,
            //         rotation: 0.0,
            //         color: BLACK,
            //     },
            // );
        }
    }

    // draw mouse coords
    let (mouse_x, mouse_y) = mouse_position();

    let mouse_coords = format!("[ {}, {} ]", mouse_x, mouse_y);

    draw_text_ex(
        &mouse_coords,
        screen_width() - 120.0,
        y_offset + (height * text_height_offset),
        TextParams {
            font: Some(font),
            font_size,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: BLACK,
        },
    );
}

// ----------------------------------------------
// Debug Rectangle
// ----------------------------------------------

fn compute_debug_geo(component_store: &mut ComponentStore) {
    let rx = screen_width() / 2.0;
    let ry = screen_height() / 2.0;
    let rw = 200.0;
    let rh = 75.0;

    // centered rect
    let drc_name = "Debug Rect";
    let drc_group = Group::Debug;
    let drc_shape = Shape::Rect {
        x: rx - rw / 2.0,
        y: ry - rh / 2.0,
        w: rw,
        h: rh,
    };
    let debug_rect_component = Component::new(drc_name, drc_group, drc_shape);

    component_store.add(debug_rect_component);

    // // offset rect
    // let drc2_name = "Debug Rect 2";
    // let drc2_group = Group::Debug;
    // let drc2_shape = Shape::Rect {
    //     x: rx - rw / 2.0 - 150.0,
    //     y: ry - rh / 2.0 - 150.0,
    //     w: rw,
    //     h: rh,
    // };
    // let debug_rect_component2 = Component::new(drc2_name, drc2_group, drc2_shape);

    // component_store.add(debug_rect_component2);
}

fn draw_debug(component_store: &ComponentStore) {
    for component in component_store
        .components
        .iter()
        .filter(|c| c.group == Group::Debug)
    {
        if let Shape::Rect { x, y, w, h } = &component.shape {
            draw_rectangle(*x, *y, *w, *h, GREEN);
        }
    }
}

// ----------------------------------------------
// UI Render Engine
// ----------------------------------------------

fn handle_input(component_store: &ComponentStore, app_state: &mut AppState) {
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mouse_x, mouse_y) = mouse_position();
        // component_clicked: &Component;

        //println!("MOUSE PRESS at [ {}, {} ]!!!!", mouse_x, mouse_y);

        for component in component_store.components.iter() {
            if let Shape::Rect { x, y, w, h } = &component.shape {
                if mouse_x >= *x && mouse_x <= *x + *w && mouse_y >= *y && mouse_y <= *y + *h {
                    let mut component_clicked = &component;
                    println! {"{} clicked!", component_clicked.name};

                    if app_state.open_menu == Some(component.name) {
                        app_state.open_menu = None;
                    } else {
                        app_state.open_menu = Some(component.name);
                    }

                    println!("open_menu = {:?}", app_state.open_menu);
                }
            }

            if let Shape::RectRE {
                x,
                y,
                w,
                h,
                color,
                radius,
            } = &component.shape
            {
                if mouse_x >= *x && mouse_x <= *x + *w && mouse_y >= *y && mouse_y <= *y + *h {
                    let mut component_clicked = &component;
                    println! {"{}{:?} clicked!", component_clicked.name, component.group};
                }
            }
        }
    }
}

// ----------------------------------------------
// UI Render Engine
// ----------------------------------------------

pub fn render_ui(
    menu_bar: &CurlMenuBar,
    menu_font: &Font,
    menu_font_size: u16,
    component_store: &mut ComponentStore,
    app_state: &mut AppState,
) {
    let rxo = screen_width() / 2.0;
    let ryo = screen_height() / 2.0;

    component_store.clear();

    clear_background(WHITE);

    // USELESS BULLSHIT
    compute_debug_geo(component_store);
    draw_debug(component_store);

    draw_rectangle_re(200.0, 200.0, 70.0, 300.0, BLUE, 7.5);

    draw_circle(950.0, 150.0, 120.0, BEIGE);
    draw_poly(950.0, 500.0, 64, 120.0, 0.0, ORANGE);

    // draw crosshair
    draw_rectangle(rxo, 0.0, 1.0, screen_height(), RED); // vertical half
    draw_rectangle(0.0, ryo, screen_width(), 1.0, RED); // horiz half

    // END USELESS BULLSHIT

    compute_menu_bar_geo(
        0.0,
        0.0,
        25.0,
        menu_bar,
        menu_font,
        menu_font_size,
        component_store,
        app_state,
    );

    draw_menu_bar(0.0, 0.0, 25.0, menu_font, menu_font_size, component_store);

    handle_input(component_store, app_state);
}
