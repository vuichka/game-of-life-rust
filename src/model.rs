const ALIVE_SYM: &str = "*";
const DEAD_SYM: &str = " ";

#[derive(Debug)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

pub fn new_world(width: usize, height: usize) -> World {
    let cells: Vec<Vec<bool>> = vec![vec![true; width]; height];

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
                *v = rand::random::<bool>();
            }
        }
        self
    }

    pub fn next_state(&mut self) {
        for row in 0..self.cells.len() {
            for i in 0..self.cells[row].len() {
                self.cells[row][i] = self.next(row, i)
            }
        }
    }

    pub fn print_world(&self) {
        for i in 0..self.cells.len() {
            for &row in self.cells[i].iter() {
                match row {
                    false => print!("\x1b[1;47m{}", DEAD_SYM),
                    true => print!("\x1b[1;42m{}", ALIVE_SYM),
                }
            }
            println!("\x1b[1;49m");
        }
        println!("\x1b[1;49m");
    }

    fn next(&self, x: usize, y: usize) -> bool {
        let n = self.neighbours(x, y);
        let alive = self.cells[x][y];

        if n < 4 && n > 2 && alive {
            return true;
        }

        if n == 3 && !alive {
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
                let (mut rx, mut ry): (i32, i32) = (x as i32 + i, y as i32 + k);
                if rx < 0 {
                    rx = self.height as i32 + rx;
                }
                if ry < 0 {
                    ry = self.width as i32 + ry;
                }
                if self.cells[(rx % self.height as i32) as usize][(ry % self.width as i32) as usize]
                    == true
                {
                    neighbours += 1;
                }
            }
        }

        neighbours
    }
}
