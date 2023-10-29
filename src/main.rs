pub mod model;

use model::*;

fn main() {
    let mut game = new_world(170, 50).with_random();

    loop {
        game.print_world();
        game.next_state();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }
}
