pub mod model;
mod rle;

use model::*;

fn main() {
    let mut game = new_world(130, 38, false);
    // game.spawn_glider(0, 0);
    // game.spawn_gosper_glider(0, 0);
    game.spawn_from_rle("", 0, 0);
    loop {
        game.print_world();
        game.next_state();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
