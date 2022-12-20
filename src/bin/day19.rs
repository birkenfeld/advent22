use advtools::input;
use advtools::prelude::HashMap;

const RX: &str = r".* (\d+) ore.* (\d+) ore.* (\d+) ore.* (\d+) clay.* (\d+) ore.* (\d+)";

fn main() {
    input::set("
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
");
    let mut blueprints = vec![];
    for bp in input::rx_lines::<(i32, i32, i32, i32, i32, i32)>(RX) {
        blueprints.push(bp);
    }

    for (ore_r_cost, cly_r_cost, obs_r_cost_ore, obs_r_cost_cly,
         geo_r_cost_ore, geo_r_cost_obs) in blueprints
    {
        let mut best = HashMap::new();
        let mut queue = Vec::new();
        queue.push((0, 0, 0, 0, // Materials
                    1, 0, 0, 0)); // Robots

        for steps in 0..24 {
            println!("{}", queue.len());
            for (ore, cly, obs, geo, ore_r, cly_r, obs_r, geo_r) in std::mem::take(&mut queue) {
                

                let new_ore = ore + ore_r;
                let new_cly = cly + cly_r;
                let new_obs = obs + obs_r;
                let new_geo = geo + geo_r;

                best.insert(new_geo, steps + 1);

                // Build one of the different robot kinds.
                if ore >= ore_r_cost {
                    queue.push((new_ore - ore_r_cost, new_cly, new_obs, new_geo,
                                ore_r + 1, cly_r, obs_r, geo_r));
                }
                if ore >= cly_r_cost {
                    queue.push((new_ore - cly_r_cost, new_cly, new_obs, new_geo,
                                ore_r, cly_r + 1, obs_r, geo_r));
                }
                if ore >= obs_r_cost_ore && cly >= obs_r_cost_cly {
                    queue.push((new_ore - obs_r_cost_ore, new_cly - obs_r_cost_cly, new_obs, new_geo,
                                ore_r, cly_r, obs_r + 1, geo_r));
                }
                if ore >= geo_r_cost_ore && obs >= geo_r_cost_obs {
                    queue.push((new_ore - geo_r_cost_ore, new_cly, new_obs - geo_r_cost_obs, new_geo,
                                ore_r, cly_r, obs_r, geo_r + 1));
                }
                // Do nothing.
                queue.push((new_ore, new_cly, new_obs, new_geo,
                            ore_r, cly_r, obs_r, geo_r));
            }
        }

        let max = queue.into_iter().map(|v| v.3).max().unwrap();
        println!(">>> {}", max);
    }

    // advtools::verify("Outer surface area", surface, 2062);
}
