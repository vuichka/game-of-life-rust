use crate::consts::*;

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
                if r < 30 {
                    *v = true;
                } else {
                    *v = false;
                }
            }
        }
        self
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool, area: i32) {
        if area == 1 {
            self.cells[x][y] = alive;
        } else {
            for row in -area / 2..area / 2 {
                for cell in -area / 2..area / 2 {
                    if x as i32 + row >= area / 2
                        && x as i32 + row < self.cells.len() as i32 - area / 2
                        && y as i32 + cell >= area / 2
                        && y as i32 + cell < self.cells[0].len() as i32 - area / 2
                    {
                        self.cells[x + row as usize][y + cell as usize] = alive;
                    }
                }
            }
        }
    }

    // used for shell printing
    #[allow(dead_code)]
    pub fn print_world(&self) {
        for row in 0..self.height {
            for e in 0..self.width {
                match self.cells[row][e] {
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

    fn next(&self, x: usize, y: usize) -> bool {
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

    fn neighbours(&self, x: usize, y: usize) -> i32 {
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
