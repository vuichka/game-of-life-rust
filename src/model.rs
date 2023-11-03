const ALIVE_SYM: &str = "#";
const DEAD_SYM: &str = ".";

#[derive(Debug)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

pub fn new_world(width: usize, height: usize, state: bool) -> World {
    let cells: Vec<Vec<bool>> = vec![vec![state; width]; height];

    World {
        width: width,
        height: height,
        cells: cells,
    }
}

impl World {
    pub fn with_random(mut self) -> World {
        for row in self.cells.iter_mut() {
            for v in row {
                let r = rand::random::<u8>();
                if r < 7 {
                    *v = true;
                } else {
                    *v = false;
                }
            }
        }
        self
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        self.cells[x][y] = alive;
    }

    pub fn spawn_glider(&mut self, x: usize, y: usize) {
        let glider: Vec<Vec<bool>> = vec![
            vec![false, false, true],
            vec![true, false, true],
            vec![false, true, true],
        ];

        for i in -1..=1 as i32 {
            for k in -1..=1 as i32 {
                let (mut rx, mut ry): (i32, i32) = (x as i32 + i, y as i32 + k);
                if rx < 0 {
                    rx = self.height as i32 + rx;
                }
                if ry < 0 {
                    ry = self.width as i32 + ry;
                }
                let rx: usize = (rx % self.height as i32) as usize;
                let ry: usize = (ry % self.width as i32) as usize;
                self.cells[rx][ry] = glider[(i + 1) as usize][(k + 1) as usize];
            }
        }
    }

    pub fn spawn_block(&mut self, x: usize, y: usize) {
        for r in 0..=1 {
            for i in 0..=1 {
                self.cells[x + r][y + i] = true;
            }
        }
    }

    pub fn spawn_diamond(&mut self, x: usize, y: usize) {
        for r in 0..=1 {
            for i in 0..=1 {
                if r == 1 && i == 1 {
                    self.cells[x + r][y + i] = false;
                }
                self.cells[x + r][y + i] = true;
            }
        }
        for r in 0..=1 {
            for i in 0..=1 {
                if r == 0 && i == 0 {
                    self.cells[x + 2 + r][y + 2 + i] = false;
                }
                self.cells[x + 2 + r][y + 2 + i] = true;
            }
        }
    }

    pub fn print_world(&self) {
        for row in 0..self.height {
            for i in 0..self.width {
                match self.cells[row][i] {
                    false => print!("\x1b[1;47m{}", DEAD_SYM),
                    true => print!("\x1b[97;102m{}", ALIVE_SYM),
                }
            }
            println!("\x1b[1;49m");
        }
        println!("\x1b[1;49m");
    }

    pub fn next_state(&mut self) {
        let mut new = new_world(self.width, self.height, false);
        for x in 0..self.height {
            for y in 0..self.width {
                new.cells[x][y] = self.next(x, y);
            }
        }
        self.cells = new.cells;
    }

    pub fn next(&self, x: usize, y: usize) -> bool {
        let nb = self.neighbours(x, y);
        let alive = self.cells[x][y];

        if alive && nb > 1 && nb < 4 {
            return true;
        }
        if !alive && nb == 3 {
            return true;
        }

        false
    }

    pub fn neighbours(&self, x: usize, y: usize) -> i32 {
        let mut neighbours = 0;

        for i in -1..=1 as i32 {
            for k in -1..=1 as i32 {
                if i == 0 && k == 0 {
                    continue;
                }
                let (h, w): (i32, i32) = (self.height as i32, self.width as i32);
                let mut x: i32 = x as i32 + i;
                let mut y: i32 = y as i32 + k;
                if x < 0 || x >= self.height as i32 {
                    x = (x + h) % h;
                }
                if y < 0 || y >= self.width as i32 {
                    y = (y + w) % w;
                }
                if self.cells[x as usize][y as usize] == true {
                    neighbours += 1;
                }
            }
        }
        neighbours
    }
}
