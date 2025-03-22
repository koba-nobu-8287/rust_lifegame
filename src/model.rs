/// model.rs
///  - Model of the life-game.
/// 

/// Cell
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    x: i32,
    y: i32,
    alive: bool,
}

impl Cell {
    /// Create a new cell.
    pub fn new(x: i32, y: i32, alive: bool) -> Cell {
        Cell { x, y, alive }
    }
    /// Check if the cell is alive.
    pub fn is_alive(&self) -> bool {
        self.alive
    }
    /// Set the cell alive or dead.
    pub fn set_alive(&mut self, alive: bool) {
        self.alive = alive;
    }
    /// Get the position of the cell.
    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

/// LifeGame model
#[derive(Debug)]
pub struct LifeGame {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    generation: i32,
}

impl LifeGame {
    /// Create a new life-game model.
    pub fn new(width: usize, height: usize) -> LifeGame {
        let mut cells = Vec::new();
        for i in 0 .. width * height {
            let x = (i % width) as i32;
            let y = (i / width) as i32;
            cells.push(Cell::new(x, y, false));
        }
        LifeGame {
            width,
            height,
            cells,
            generation: 0,
        }
    }
    /// Get the width of the life-game.
    pub fn get_width(&self) -> usize {
        self.width
    }
    /// Get the height of the life-game.
    pub fn get_heght(&self) -> usize {
        self.height
    }
    /// Get the generation of the life-game.
    pub fn get_generation(&self) -> i32 {
        self.generation
    }

    /// Calculate the position.
    fn add_position(pos: i32, delta: i32, max: i32) -> i32 {
        (pos + delta).rem_euclid(max)
    }
    /// Calculatet the vector index from the position.
    fn get_index(&self, x: i32, y: i32) -> usize {
        let x = LifeGame::add_position(x, 0, self.width as i32);
        let y = LifeGame::add_position(y, 0, self.height as i32);
        (y * self.width as i32 + x) as usize
    }
    /// Get the cell at the position.
    pub fn get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        self.cells.get(self.get_index(x, y))
    }
    /// Get the cell at the position.(mutable)
    pub fn get_cell_mut(&mut self, x: i32, y: i32) -> Option<&mut Cell> {
        let index = self.get_index(x, y);
        self.cells.get_mut(index)
    }
    /// Update the state of the game to the next generation.
    pub fn next_generation(&mut self) {
        let mut new_cells = self.cells.clone();
        for cell in &self.cells {
            let (x, y) = cell.get_position();
            let alive_neighbors = self.count_alive_neighbors(x, y);
            let new_state = match (cell.is_alive(), alive_neighbors) {
                (true, 2) | (true, 3) => true,
                (true, _) => false,
                (false, 3) => true,
                (false, _) => false,
            };
            let index = self.get_index(x, y);
            if let Some(new_cell) = new_cells.get_mut(index) {
                new_cell.set_alive(new_state);
            }
        }
        self.cells = new_cells;
        self.generation += 1;
    }

    /// Count the number of alive neighbors of a cell at a given position.
    fn count_alive_neighbors(&self, x: i32, y: i32) -> usize {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 {
                    if let Some(neighbor) = self.get_cell(x + dx, y + dy) {
                        if neighbor.is_alive() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}
