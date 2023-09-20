use crate::InteractionCallbackData;
use crate::InteractionData;
use rand;

pub const SIZE: usize = 10;

fn rand_array(_: usize) -> [bool; SIZE] {
    rand::random()
}

fn rand_matrix() -> [[bool; SIZE]; SIZE] {
    core::array::from_fn(rand_array)
}

fn toroidal(grid: [[bool; SIZE]; SIZE], (x, y): (i8, i8)) -> bool {
    let sz = SIZE as i8;
    let (xmod, ymod) = (x.rem_euclid(sz) as usize, y.rem_euclid(sz) as usize);

    grid[xmod][ymod]
}

fn get_neighbor_count(grid: [[bool; SIZE]; SIZE], (x, y): (i8, i8)) -> u8 {
    let nghbhd: [(i8, i8); 9] = [
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1),
        (x, y),
    ];

    nghbhd
        .into_iter()
        .fold(0, |acc, b| acc + (toroidal(grid, b) as u8))
}

fn next_generation(grid: [[bool; SIZE]; SIZE]) -> [[bool; SIZE]; SIZE] {
    let mut next_gen = grid.clone();

    for i in 0..SIZE {
        for j in 0..SIZE {
            let nghbr_count: u8 = get_neighbor_count(next_gen, (i as i8, j as i8));

            match nghbr_count {
                3 => next_gen[i][j] = true,
                4 => continue,
                _ => next_gen[i][j] = false,
            }
        }
    }
    next_gen
}

fn bool_to_emote(value: bool) -> String {
    if value {
        return "🌝".to_string();
    }
    "🌚".to_string()
}

fn array_to_emotes(line: [bool; SIZE]) -> String {
    line.into_iter()
        .fold(String::new(), |acc, value| acc + &bool_to_emote(value))
}

fn grid_to_emotes(grid: [[bool; SIZE]; SIZE]) -> String {
    grid.into_iter().fold(String::new(), |acc, line| {
        acc + &array_to_emotes(line) + "\n"
    })
}

pub fn game_of_life(_: &InteractionData) -> InteractionCallbackData {
    let griddy = rand_matrix();
    let next_grid = next_generation(griddy);
    let output = grid_to_emotes(griddy) + "\n \n" + &grid_to_emotes(next_grid);

    InteractionCallbackData {
        content: Some(output.to_string()),
    }
}
