use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Debug)]
enum VisitStatus {
    NotVisited,
    Visited,
}

#[derive(Debug)]
enum RightBoundary {
    Open,
    Closed,
}

#[derive(Debug)]
enum BottomBoundary {
    Open,
    Closed,
}

#[derive(Debug)]
enum PossibleDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Cell {
    visit_status: VisitStatus,
    right_side: RightBoundary,
    bottom_side: BottomBoundary,
}

impl Cell {
    pub fn build() -> Cell {
        Cell {
            visit_status: VisitStatus::NotVisited,
            right_side: RightBoundary::Closed,
            bottom_side: BottomBoundary::Closed,
        }
    }
}

#[derive(Debug)]
pub struct Maze {
    start_row: usize,
    end_row: usize,
    num_rows: usize,
    num_columns: usize,
    maze: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn build() -> Maze {
        let mut rng = thread_rng();

        let mut new_maze: Maze = Maze {
            start_row: rng.gen_range(0..5),
            end_row: rng.gen_range(0..5),
            num_rows: 5,
            num_columns: 5,
            maze: Vec::new(),
        };

        new_maze.maze = Vec::with_capacity(new_maze.num_rows);
        for _ in 0..new_maze.num_rows {
            new_maze.maze.push(Vec::with_capacity(new_maze.num_columns));
        }

        for row in 0..new_maze.num_rows {
            for _ in 0..new_maze.num_columns {
                new_maze.maze[row].push(Cell::build());
            }
        }

        new_maze
    }

    pub fn build_custom(num_rows: usize, num_columns: usize) -> Maze {
        let mut rng = thread_rng();

        let mut new_maze: Maze = Maze {
            start_row: rng.gen_range(0..num_rows),
            end_row: rng.gen_range(0..num_rows),
            num_rows,
            num_columns,
            maze: Vec::new(),
        };

        new_maze.maze = Vec::with_capacity(new_maze.num_rows);
        for _ in 0..new_maze.num_rows {
            new_maze.maze.push(Vec::with_capacity(new_maze.num_columns));
        }

        for row in 0..num_rows {
            for col in 0..num_columns {
                new_maze.maze[row].push(Cell::build());

                if row == num_rows - 1 {
                    new_maze.maze[row][col].bottom_side = BottomBoundary::Closed;
                }
                if col == num_columns - 1 {
                    new_maze.maze[row][col].right_side = RightBoundary::Closed;
                }
            }
        }

        new_maze
    }

    pub fn make_maze(&mut self) -> () {
        let mut stack: Vec<(usize, usize)> = Vec::new();

        stack.push((self.start_row, 0));

        while !stack.is_empty() {
            let (row, col) = stack.last().copied().unwrap();

            let mut poss_dir: Vec<PossibleDirection> = Vec::with_capacity(4);

            if row != 0 {
                match self.maze[row - 1][col].visit_status {
                    VisitStatus::NotVisited => poss_dir.push(PossibleDirection::Up),
                    _ => (),
                }
            }
            if row != self.num_rows - 1 {
                match self.maze[row + 1][col].visit_status {
                    VisitStatus::NotVisited => poss_dir.push(PossibleDirection::Down),
                    _ => (),
                }
            }
            if col != 0 {
                match self.maze[row][col - 1].visit_status {
                    VisitStatus::NotVisited => poss_dir.push(PossibleDirection::Left),
                    _ => (),
                }
            }
            if col != self.num_columns - 1 {
                match self.maze[row][col + 1].visit_status {
                    VisitStatus::NotVisited => poss_dir.push(PossibleDirection::Right),
                    _ => (),
                }
            }

            if poss_dir.len() == 0 {
                stack.pop();
                continue;
            }

            match poss_dir.choose(&mut rand::thread_rng()).unwrap() {
                PossibleDirection::Up => {
                    stack.push((row - 1, col));
                    self.maze[row - 1][col].visit_status = VisitStatus::Visited;
                    self.maze[row - 1][col].bottom_side = BottomBoundary::Open;
                }
                PossibleDirection::Down => {
                    stack.push((row + 1, col));
                    self.maze[row + 1][col].visit_status = VisitStatus::Visited;
                    self.maze[row][col].bottom_side = BottomBoundary::Open;
                }
                PossibleDirection::Left => {
                    stack.push((row, col - 1));
                    self.maze[row][col - 1].visit_status = VisitStatus::Visited;
                    self.maze[row][col - 1].right_side = RightBoundary::Open;
                }
                PossibleDirection::Right => {
                    stack.push((row, col + 1));
                    self.maze[row][col + 1].visit_status = VisitStatus::Visited;
                    self.maze[row][col].right_side = RightBoundary::Open;
                }
            }
        }

        for row in 0..self.num_rows {
            for col in 0..self.num_columns {
                self.maze[row][col].visit_status = VisitStatus::NotVisited
            }
        }
    }

    pub fn solve_maze(&mut self) -> () {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

        queue.push_back((self.start_row, 0));

        while !queue.is_empty() {
            let (row, col) = queue.pop_front().unwrap();

            self.maze[row][col].visit_status = VisitStatus::Visited;

            if row == self.end_row && col == self.num_columns - 1 {
                return;
            }

            let mut poss_dir: Vec<PossibleDirection> = Vec::new();

            if row != 0 {
                if let BottomBoundary::Open = self.maze[row - 1][col].bottom_side {
                    if let VisitStatus::NotVisited = self.maze[row - 1][col].visit_status {
                        poss_dir.push(PossibleDirection::Up);
                    }
                }
            }
            if row != self.num_rows - 1 {
                if let BottomBoundary::Open = self.maze[row][col].bottom_side {
                    if let VisitStatus::NotVisited = self.maze[row + 1][col].visit_status {
                        poss_dir.push(PossibleDirection::Down);
                    }
                }
            }
            if col != 0 {
                if let RightBoundary::Open = self.maze[row][col - 1].right_side {
                    if let VisitStatus::NotVisited = self.maze[row][col - 1].visit_status {
                        poss_dir.push(PossibleDirection::Left);
                    }
                }
            }
            if col != self.num_columns - 1 {
                if let RightBoundary::Open = self.maze[row][col].right_side {
                    if let VisitStatus::NotVisited = self.maze[row][col + 1].visit_status {
                        poss_dir.push(PossibleDirection::Right);
                    }
                }
            }

            for i in poss_dir.iter() {
                match i {
                    PossibleDirection::Up => queue.push_back((row - 1, col)),
                    PossibleDirection::Down => queue.push_back((row + 1, col)),
                    PossibleDirection::Left => queue.push_back((row, col - 1)),
                    PossibleDirection::Right => queue.push_back((row, col + 1)),
                }
            }
        }
    }

    pub fn reset_maze(&mut self) -> () {
        for row in 0..self.num_rows {
            for col in 0..self.num_columns {
                self.maze[row][col] = Cell::build()
            }
        }
    }

    pub fn print_maze(&self) -> () {
        // Top Side
        print!("⬛");
        for _ in 0..self.num_columns {
            print!("⬛⬛");
        }
        print!("\n");

        for row in 0..self.num_rows {
            let current_row: &Vec<Cell> = &self.maze[row];

            // Left Side
            if row == self.start_row {
                print!("🟩");
            } else {
                print!("⬛");
            }

            // Maze Stuff
            for col in 0..self.num_columns {
                let current_cell: &Cell = &current_row[col];

                match (&current_cell.visit_status, &current_cell.right_side) {
                    (VisitStatus::NotVisited, RightBoundary::Closed) => {
                        if col == self.num_columns - 1 && row == self.end_row {
                            print!("⬜🟥");
                        } else {
                            print!("⬜⬛");
                        }
                    }
                    (VisitStatus::NotVisited, RightBoundary::Open) => print!("⬜⬜"),
                    (VisitStatus::Visited, RightBoundary::Closed) => {
                        if col == self.num_columns - 1 && row == self.end_row {
                            print!("🟦🟥");
                        } else {
                            print!("🟦⬛");
                        }
                    }
                    (VisitStatus::Visited, RightBoundary::Open) => {
                        if let VisitStatus::Visited = self.maze[row][col + 1].visit_status {
                            print!("🟦🟦");
                        } else {
                            print!("🟦⬜");
                        }
                    }
                }
            }
            print!("\n");

            // Maze Bottom Separator
            print!("⬛");
            for col in 0..self.num_columns {
                match (&current_row[col].visit_status, &current_row[col].bottom_side) {
                    (VisitStatus::Visited, BottomBoundary::Open) => {
                        if let VisitStatus::Visited = self.maze[row+1][col].visit_status {
                            print!("🟦⬛");
                        } else {
                            print!("⬜⬛");
                        }
                    }
                    (VisitStatus::NotVisited, BottomBoundary::Open) => print!("⬜⬛"),
                    (_, BottomBoundary::Closed) => print!("⬛⬛"),
                }
            }
            print!("\n");
        }
    }
}
