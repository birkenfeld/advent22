use advtools::input;
use advtools::prelude::{HashMap, HashSet, Itertools};

fn main() {
    let mut elves = HashSet::new();

    for (y, line) in input::lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let all_dirs = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let mut look_dirs = [
        (1, 0, 2),
        (6, 5, 7),
        (3, 0, 5),
        (4, 2, 7),
    ];

    let mut neighbors = [false; 8];


    for round in 1.. {
        let mut targets = HashMap::new();
        let has_elf = |(ex, ey), (dx, dy)| elves.contains(&(ex + dx, ey + dy));

        // Step 1: select new targets.
        for &elf in &elves {
            let neighbors = all_dirs.iter().enumerate().for_each(|(i, &d)| has_elf(elf, d)).collect_vec();
            // Not moving if no neighbors at all.
            if !neighbors.iter().any(|n| *n) {
                continue;
            }
            // Try to find a free direction.
            for &(main, side1, side2) in &look_dirs {
                if !(neighbors[main] || neighbors[side1] || neighbors[side2]) {
                    targets.entry((elf.0 + all_dirs[main].0,
                                   elf.1 + all_dirs[main].1)).or_insert(vec![]).push(elf);
                    break;
                }
            }
        }

        let mut moved = false;

        // Step 2: execute moves.
        for (target, want_elves) in targets {
            if let [pos] = want_elves[..] {
                elves.remove(&pos);
                elves.insert(target);
                moved = true;
            }
        }

        // Part 1: count the empty places in the elves' bounding rectangle.
        if round == 10 {
            let mut max_x = 0; let mut max_y = 0;
            let mut min_x = 999; let mut min_y = 999;
            for &(ex, ey) in &elves {
                max_x = max_x.max(ex);
                max_y = max_y.max(ey);
                min_x = min_x.min(ex);
                min_y = min_y.min(ey);
            }
            let free = (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32;
            advtools::verify("Free spaces after 10", free, 4172);
        }

        // Part 2: no moves, we're done.
        if !moved {
            advtools::verify("No elf moves in round", round, 942);
            return;
        }

        look_dirs.rotate_left(1);
    }
}
