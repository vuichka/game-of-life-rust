# Game Of Life Rust learning project

I managed to build a game while learning several programming languages, one of them is rust. You can check what a got after a few weeks here.

## Usage 

1. If you don't have Rust Programming Language install, please go [here](https://www.rust-lang.org/tools/install) and install it, or use this if you’re running **macOS**, **Linux**, or another **Unix-like OS**:
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the repository to your machine in your place.
```shell
git clone https://github.com/vuichka/game-of-life-rust.git
```

3. Run
```shell
cargo run --release
```

or in developer mode
```shell
cargo run
```

## Controls
The games stops when you press on Pause logo. When paused you can draw and erase any lives you want.

- `R` - random field restart
- `D` - dead field restart
- `P` - Pause/Unpause the game
- `M` - switch drawer to brush/eraser

> If you have any other keyboard layout than qwerty, please press qwerty keys. It's hardcoded.