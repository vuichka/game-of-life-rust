pub mod model;

use model::*;

fn main() {
    let mut game = new_world(130, 40, false);

    game.spawn_glider(5, 5);
    game.spawn_diamond(15, 15);
    loop {
        game.print_world();
        game.next_state();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
