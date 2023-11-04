mod consts;
mod game;
mod model;
mod raylib_ui;
mod rle;

use ::core::time;
use consts::*;
use game::*;
use raylib_ui::*;
use std::thread::sleep;

use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Game-Of-Life-Rust")
        .build();

    let pause_btn = rl
        .load_texture(&thread, "src/static/pause_btn.png")
        .unwrap();

    let mut game: Game = new_game(pause_btn);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        if d.is_key_down(KeyboardKey::KEY_R) {
            game.restart_game();
        }

        draw_grid(&mut d, &game);
        draw_pause_button(&mut d, &mut game);
        press_drawing(&mut d, &mut game);
        change_draw_state(&mut d, &mut game);

        if !game.pause {
            game.world.next_state();
            sleep(time::Duration::from_millis(100));
        }
    }
}
