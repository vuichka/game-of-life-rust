use raylib::texture::Texture2D;

use crate::consts::*;
use crate::model::*;

pub struct Game {
    pub world: World,
    pub draw_state: bool,
    pub pause: bool,
    pub pause_btn_txt: Texture2D,
}

pub fn new_game(pause_btn_txt: Texture2D) -> Game {
    Game {
        world: new_world(COLS as usize, ROWS as usize, true), //.with_random(),
        draw_state: true,
        pause: true,
        pause_btn_txt: pause_btn_txt,
    }
}

impl Game {
    pub fn restart_game(&mut self, alive: bool) {
        if alive {
            self.world = new_world(COLS as usize, ROWS as usize, false).with_random();
        } else {
            self.world = new_world(COLS as usize, ROWS as usize, alive);
        }
    }
}
