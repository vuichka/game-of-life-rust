use ca_formats::rle::Rle;

use crate::model::World;

const GLIDER: &str = "#N Glider
#O Richard K. Guy
#C The smallest, most common, and first discovered spaceship. Diagonal, has period 4 and speed c/4.
#C www.conwaylife.com/wiki/index.php?title=Glider
x = 3, y = 3, rule = B3/S23
bob$2bo$3o!";

const GOSPER_GLIDER_GUN: &str = "#N Gosper glider gun
#O Bill Gosper
#C A true period 30 glider gun.
#C The first known gun and the first known finite pattern with unbounded growth.
#C www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
x = 36, y = 9, rule = B3/S23
24bo11b$22bobo11b$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o14b$2o8b
o3bob2o4bobo11b$10bo5bo7bo11b$11bo3bo20b$12b2o!";

impl World {
    pub fn spawn_from_rle(&mut self, rle: &str, x: usize, y: usize) {
        let rle = rle
            .trim()
            .split(" ")
            .filter(|el| !el.is_empty())
            .collect::<Vec<_>>()
            .join("");
        let mut figure = Rle::new(rle.as_str()).unwrap();
        match self.set_figure(&mut figure, x, y) {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e)
            }
        };
    }

    pub fn spawn_glider(&mut self, x: usize, y: usize) {
        let mut fig = Rle::new(GLIDER).unwrap();
        self.set_figure(&mut fig, x, y).unwrap();
    }

    pub fn spawn_gosper_glider(&mut self, x: usize, y: usize) {
        let mut fig = Rle::new(GOSPER_GLIDER_GUN).unwrap();
        self.set_figure(&mut fig, x, y).unwrap();
    }

    pub fn set_figure(&mut self, fig: &mut Rle<&str>, x: usize, y: usize) -> Result<(), &str> {
        let h = match fig.header_data() {
            Some(v) => v.to_owned(),
            None => return Err("failed to fetch header rle data"),
        };
        if self.height < h.y as usize || self.width < h.x as usize {
            return Err("game field is small for that figure");
        }

        let live_cells = fig.map(|cell| cell.unwrap().position).collect::<Vec<_>>();

        for col in x..=(x + h.x as usize) {
            for el in y..=(y + h.y as usize) {
                let mut sx = (x + col) as i32;
                let mut sy = (y + el) as i32;

                if sx < 0 || sx >= self.width as i32 {
                    sx = (sx as i32 + self.height as i32) % self.width as i32;
                }

                if sy < 0 || sy >= self.height as i32 {
                    sy = (y as i32 + self.height as i32) % self.height as i32;
                }

                self.cells[sy as usize][sx as usize] = false;
            }
        }

        for el in live_cells {
            let mut sx = (x as i64 + el.0) as i32;
            let mut sy = (y as i64 + el.1) as i32;

            if sx < 0 || sx >= self.width as i32 {
                sx = (sx as i32 + self.width as i32) % self.width as i32;
            }

            if sy < 0 || sy >= self.height as i32 {
                sy = (y as i32 + self.height as i32) % self.height as i32;
            }
            // println!("{x}, {y}| {sx}, {sy}| {} {}", h.x, h.y);
            self.cells[sy as usize][sx as usize] = true;
        }

        Ok(())
    }
}
