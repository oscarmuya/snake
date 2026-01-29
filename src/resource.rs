use crate::game::random;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct GameWorld {
    pub grid: Grid,
    pub snake_body: Vec<[u32; 2]>,
    pub apple: [u32; 2],
    pub direction: Direction,
}

impl GameWorld {
    pub fn new(width: u32, height: u32, size: u32) -> Self {
        let grid = Grid::new(width, height, size);
        Self {
            grid,
            direction: Direction::Down,
            apple: [grid.grid_width / 2, grid.grid_height / 2],
            snake_body: vec![[0, 0], [0, 1]],
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn move_snake(&mut self) {
        let next = self.get_next();
        self.snake_body.insert(0, next);
        self.snake_body.pop();
    }

    pub fn check_for_apple(&mut self) {
        if self.snake_body.first().unwrap() == &[self.apple[0], self.apple[1]] {
            let next = self.get_next();
            self.snake_body.insert(0, next);
            self.apple = [random(self.grid.grid_width), random(self.grid.grid_height)];
        }
    }

    pub fn get_next(&mut self) -> [u32; 2] {
        match self.direction {
            Direction::Up => {
                let last = self.snake_body.first().unwrap();
                [last[0], last[1] - 1]
            }
            Direction::Left => {
                let last = self.snake_body.first().unwrap();
                [last[0] - 1, last[1]]
            }
            Direction::Down => {
                let last = self.snake_body.first().unwrap();
                [last[0], last[1] + 1]
            }
            Direction::Right => {
                let last = self.snake_body.first().unwrap();
                [last[0] + 1, last[1]]
            }
        }
    }

    pub fn add_square(&mut self, size: u32, x: u32, y: u32) -> Square {
        Square::new(size, x, y).on_grid(&self.grid)
    }

    pub fn add_circle(&mut self, radius: u32, x: u32, y: u32) -> Circle {
        Circle::new(radius, x, y).on_grid(&self.grid)
    }
}

#[derive(Clone, Debug)]
pub struct Square {
    pub size: u32,
    pub grid_x: u32,
    pub grid_y: u32,
    pub points: Vec<[u32; 2]>,
}

impl Square {
    pub fn new(size: u32, grid_x: u32, grid_y: u32) -> Self {
        Self {
            size,
            grid_x,
            grid_y,
            points: Vec::new(),
        }
    }

    pub fn on_grid(mut self, grid: &Grid) -> Self {
        self.points = grid.get_square_points_at_grid(self.grid_x, self.grid_y);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub radius: u32,
    pub grid_x: u32,
    pub grid_y: u32,
    pub points: Vec<[u32; 2]>,
}

impl Circle {
    pub fn new(radius: u32, grid_x: u32, grid_y: u32) -> Self {
        Self {
            radius,
            grid_x,
            grid_y,
            points: Vec::new(),
        }
    }

    pub fn on_grid(mut self, grid: &Grid) -> Self {
        self.points = grid.get_circle_points_at_grid(self.grid_x, self.grid_y, grid.size / 2);
        self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub size: u32,
    pub grid_width: u32,
    pub grid_height: u32,
}

impl Grid {
    pub fn new(width: u32, height: u32, size: u32) -> Self {
        let grid_height = height / size;
        let grid_width = width / size;

        Self {
            width,
            height,
            size,
            grid_width,
            grid_height,
        }
    }

    pub fn change_window(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn get_circle_points_at_grid(
        &self,
        grid_x: u32,
        grid_y: u32,
        radius: u32,
    ) -> Vec<[u32; 2]> {
        let mut points: Vec<[u32; 2]> = Vec::new();
        let start = (grid_x * self.size, grid_y * self.size);

        let cx = start.0 + self.size / 2;
        let cy = start.1 + self.size / 2;

        for x in start.0..start.0 + self.size {
            for y in start.1..start.1 + self.size {
                if point_in_circle(x as i32, y as i32, cx as i32, cy as i32, radius as i32) {
                    points.push([x, y]);
                }
            }
        }

        points
    }

    pub fn get_square_points_at_grid(self, grid_x: u32, grid_y: u32) -> Vec<[u32; 2]> {
        let mut points: Vec<[u32; 2]> = Vec::new();

        let start = (grid_x * self.size, grid_y * self.size);

        for x in start.0..start.0 + self.size {
            for y in start.1..start.1 + self.size {
                points.push([x, y]);
            }
        }

        points
    }
}

/// Checks if a point is inside a circle
fn point_in_circle(px: i32, py: i32, cx: i32, cy: i32, radius: i32) -> bool {
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy) <= (radius * radius)
}
