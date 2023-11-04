mod consts;
mod game;
mod model;
mod raylib_ui;
mod rle;

use ::core::time;
use consts::*;
use copypasta::{ClipboardContext, ClipboardProvider};
use game::*;
use raylib_ui::*;
use std::thread::sleep;

use raylib::prelude::*;

fn main() {
    // context for clipboard
    let mut ctx = ClipboardContext::new().unwrap();

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game-Of-Life-Rust")
        .build();

    let pause_btn = rl
        .load_texture(&thread, "src/static/pause_btn.png")
        .unwrap();

    let erase_area = 4.;

    let mut game: Game = new_game(pause_btn);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        draw_grid(&mut d, &game);
        change_draw_state(&mut d, &mut game);
        draw_pause_button(&mut d, &mut game);

        // controls handlers
        press_drawing(&mut d, &mut game, erase_area);
        if d.is_key_down(KeyboardKey::KEY_R) {
            game.restart_game(true);
        }
        if d.is_key_down(KeyboardKey::KEY_D) {
            game.restart_game(false);
        }
        if d.is_key_down(KeyboardKey::KEY_P) || d.is_key_down(KeyboardKey::KEY_SPACE) {
            game.pause = !game.pause;
            sleep(time::Duration::from_millis(100));
        }
        // controls handlers end

        // set figures
        if d.is_key_down(KeyboardKey::KEY_H) {
            let (x, y) = (
                (d.get_mouse_x() / CELL_WIDTH) as usize,
                (d.get_mouse_y() / CELL_HEIGHT) as usize,
            );
            let buffer = ctx.get_contents().unwrap();
            game.world.spawn_from_rle(buffer.as_str(), x, y);
        }

        if d.is_key_down(KeyboardKey::KEY_ONE) {
            let (x, y) = (
                (d.get_mouse_x() / CELL_WIDTH) as usize,
                (d.get_mouse_y() / CELL_HEIGHT) as usize,
            );
            game.world.spawn_glider(x, y);
        }

        if d.is_key_down(KeyboardKey::KEY_TWO) {
            let (x, y) = (
                (d.get_mouse_x() / CELL_WIDTH) as usize,
                (d.get_mouse_y() / CELL_HEIGHT) as usize,
            );
            game.world.spawn_gosper_glider(x, y);
        }
        // set figures end

        if !game.pause {
            game.world.next_state();
            sleep(time::Duration::from_millis(REFRESH_MILLIS));
        }
    }
}
