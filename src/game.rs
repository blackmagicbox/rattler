use std::collections::VecDeque;
use macroquad::prelude::draw_line;

// --------------------- Movement and Direction Logic ---------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Vector2d {
    pub x: i32,
    pub y: i32,
}

impl std::ops::Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(dead_code)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // Converts the direction into a displacement vector (delta) that can be added to a position to move in that direction.
    // Attention: + The y-axis is inverted in the terminal, so Up is negative and Down is positive.
    pub fn delta(self) -> Vector2d {
        match self {
            Direction::Up => Vector2d { x: 0, y: -1 },
            Direction::Down => Vector2d { x: 0, y: 1 },
            Direction::Left => Vector2d { x: -1, y: 0 },
            Direction::Right => Vector2d { x: 1, y: 0 },
        }
    }

    #[expect(dead_code)]
    pub fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// --------------------- Drawing Logic ---------------------

pub struct Layout {
    pub cell_size: f32,
    pub x0: f32,
    pub y0: f32,
}

impl Layout {
    pub fn from_screen(cols: u32, rows: u32, margin: f32, sw: f32, sh: f32) -> Layout {
        let cell_w = sw / cols as f32;
        let cell_h = sh / rows as f32;
        let cell_size = cell_w.min(cell_h) * margin;

        let grid_w = cell_size * cols as f32;
        let grid_h = cell_size * rows as f32;

        Layout {
            cell_size,
            x0: (sw - grid_w) / 2.0,
            y0: (sh - grid_h) / 2.0,
        }
    }

    // Converts a cell coordinate (col, row) to screen coordinates (x, y) based on the layout's cell size and origin.
    fn cell_to_screen(&self, cell: Vector2d) -> (f32, f32) {
        (
            self.x0 + cell.x as f32 * self.cell_size,
            self.y0 + cell.y as f32 * self.cell_size,
        )
    }

    // Draws a rectangle representing a cell at the given cell coordinate with the specified color.
    pub fn draw_cell(&self, cell: Vector2d, color: macroquad::color::Color) {
        let (x, y) = self.cell_to_screen(cell);
        macroquad::shapes::draw_rectangle(x, y, self.cell_size, self.cell_size, color);
    }

    pub fn draw_grid(&self, cols: u32, rows: u32, line_color: macroquad::color::Color) {
        for col in 0..=cols {
            let x = self.x0 + self.cell_size * col as f32;
            draw_line(
                x,
                self.y0,
                x,
                self.y0 + self.cell_size * rows as f32,
                1.0,
                line_color,
            );
        }

        for row in 0..=rows {
            let y = self.y0 + self.cell_size * row as f32;
            draw_line(
                self.x0,
                y,
                self.x0 + self.cell_size * cols as f32,
                y,
                1.0,
                line_color,
            );
        }
    }
}

//-----------------------------------  Actors --------------------------------

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snake{
    direction: Direction,
    body: VecDeque<Vector2d>,
}


impl Snake {
    pub fn new(head: Vector2d, length:usize, direction: Direction) -> Snake {
        // Assert to test the snake have at least a head.
        assert!(length >= 1, "Snake must have at least the head");
        // Set the direction
        let behind  = direction.opposite().delta();
        // Create a new body `with_capacity' that's why the length is for
        let mut body:VecDeque<Vector2d> = VecDeque::with_capacity(length);
        // Create the head
        let mut cell = head;
        // Populate the body
        for _ in 0..length {
            body.push_back(cell);
            cell = cell + behind;
        }
        // Return the new snake
        Snake {
            direction,
            body,
        }
    }

    pub fn head(&self) -> Vector2d {
        *self.body.front().expect("snake body is never empty")
    }

    pub fn step(&mut self, grow: bool) {
        // Set the new Head in front of the current head (direction.delta)
        let new_head = self.head() + self.direction.delta();
        // Push the new head to the body
        self.body.push_front(new_head);
        // check if the snake should grow (ate a fruit)
        // If not just pop the back
        if !grow {
            self.body.pop_back();
        }
    }

    pub fn get_body(&self) -> &VecDeque<Vector2d> {
        &self.body
    }

    pub fn set_direction(&mut self, dir: Direction) {
        if dir != self.direction.opposite(){
            self.direction = dir;
        }
    }

}




