pub mod util;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::util::maze::Maze;

fn main() {
    let device_state = DeviceState::new();

    let mut maze: Maze = Maze::build_custom(6, 6);

    maze.make_maze();

    maze.print_maze();

    maze.solve_maze();

    maze.print_maze();

    // loop {
    //     let keys: Vec<Keycode> = device_state.get_keys();
    //     for key in keys.iter() {
    //         match key {
    //             Keycode::Left => {
    //                 print!("Left\n");
    //             },
    //             _ => (),
    //         }
    //     }
    // }
}
