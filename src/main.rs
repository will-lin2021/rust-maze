pub mod util;

use crate::util::maze::Maze;

fn main() {
    let mut maze: Maze = Maze::build_custom(16, 16);

    maze.make_maze();

    maze.print_maze();

    maze.solve_maze();

    maze.print_maze();
}
