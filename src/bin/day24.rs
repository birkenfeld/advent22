use advtools::input;
use advtools::prelude::Itertools;
use advtools::grid::{Dir, Grid};

const LEFT: u8 = 0;
const RIGHT: u8 = 1;
const UP: u8 = 2;
const DOWN: u8 = 3;

fn step(grid: &mut Grid<u8>) {
    // for p in grid.positions() {
        
    // }
}

fn main() {
    let mut grid = Grid::new(
        input::lines().skip(1).take_while(|l| !l.starts_with("##")).map(|line| {
            line.chars().skip(1).take_while(|&c| c != '#').map(|ch| match ch {
                '<' => 1 << LEFT,
                '>' => 1 << RIGHT,
                '^' => 1 << UP,
                'v' => 1 << DOWN,
                _ => 0
            }).collect_vec()
        })
    );

    println!("{:?}", grid[(149, 19)]);
}
