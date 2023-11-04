use std::thread::sleep;
use std::time;

use raylib::prelude::*;

use crate::consts::*;
use crate::game::Game;

pub fn draw_grid(d: &mut RaylibDrawHandle, game: &Game) {
    for row in 0..ROWS {
        for cell in 0..COLS {
            let cell_color = match game.world.cells[row as usize][cell as usize] {
                true => Color::from_hex(ALIVE_COLOR).unwrap(),
                false => Color::from_hex(DEAD_COLOR).unwrap(),
            };
            d.draw_rectangle(
                cell * CELL_WIDTH,
                row * CELL_HEIGHT,
                CELL_WIDTH,
                CELL_HEIGHT,
                cell_color,
            );
            d.draw_rectangle_lines(
                cell * CELL_WIDTH,
                row * CELL_HEIGHT,
                CELL_WIDTH,
                CELL_HEIGHT,
                Color::from_hex(GRID_COLOR).unwrap(),
            );
        }
    }
}

pub fn draw_pause_button(d: &mut RaylibDrawHandle, game: &mut Game) {
    // pause button
    let (mut btn_x, mut btn_y) = (15f32, 8f32);
    let (mut btn_w, mut btn_h) = (20f32, 8f32);

    (btn_x, btn_y) = (btn_x * CELL_WIDTH as f32, btn_y * CELL_HEIGHT as f32);
    (btn_w, btn_h) = (btn_w * CELL_WIDTH as f32, btn_h * CELL_HEIGHT as f32);
    let mut btn_dest = prelude::Rectangle::new(btn_x, btn_y, btn_w, btn_h);
    let pause_btn_origin = prelude::Vector2::new(btn_w / 2., btn_h / 2.);

    d.draw_texture_pro(
        &game.pause_btn_txt,
        prelude::Rectangle::new(
            0.,
            0.,
            game.pause_btn_txt.width as f32,
            game.pause_btn_txt.height as f32,
        ),
        btn_dest,
        pause_btn_origin,
        -5.,
        Color::GREEN,
    );

    // check for collision
    btn_dest = prelude::Rectangle::new(btn_x - (btn_w / 2.), btn_y - (btn_h / 2.), btn_w, btn_h);
    if btn_dest.check_collision_point_rec(d.get_mouse_position())
        && d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON)
    {
        game.pause = !game.pause;
    }
}

pub fn press_drawing(d: &mut RaylibDrawHandle, game: &mut Game, erase_area: f32) {
    let (mut x, mut y) = (d.get_mouse_x(), d.get_mouse_y());

    x = x / CELL_WIDTH;
    y = y / CELL_HEIGHT;
    if !game.draw_state {
        d.draw_rectangle_lines_ex(
            Rectangle::new(
                x as f32 * CELL_WIDTH as f32 - (erase_area / 2_f32 * CELL_WIDTH as f32),
                y as f32 * CELL_HEIGHT as f32 - (erase_area / 2_f32 * CELL_HEIGHT as f32),
                erase_area * CELL_WIDTH as f32,
                erase_area * CELL_HEIGHT as f32,
            ),
            2,
            Color::WHITE,
        );
    }

    if d.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        if game.draw_state == true {
            game.world.set(y as usize, x as usize, game.draw_state, 1);
        } else {
            game.world
                .set(y as usize, x as usize, game.draw_state, erase_area as i32)
        }
    }
}

pub fn change_draw_state(d: &mut RaylibDrawHandle, game: &mut Game) {
    if d.is_key_pressed(KeyboardKey::KEY_M) {
        game.draw_state = !game.draw_state;
        sleep(time::Duration::from_millis(100));
    }
}
