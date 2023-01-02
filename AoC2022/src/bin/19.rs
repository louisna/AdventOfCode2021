use core::num;
use std::collections::HashMap;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

#[derive(Debug, Clone)]
struct Blueprint {
    ore: i32,             // Ore
    clay: i32,            // Ore
    obsidian: (i32, i32), // (ore, clay)
    geode: (i32, i32),    // (ore, obsidian)
    max_value: i32,
}

impl Blueprint {
    fn dfs(&mut self, state: &State, value: i32, memo: &mut HashMap<State, i32>, can_build: u8) -> i32 {
        if state.time_left <= 0 {
            return value;
        }

        if state.time_left + state.robot_geo < self.max_value {
            return 0; // Prune.
        }

        if value + state.robot_geo * state.time_left + (state.time_left * state.time_left + 1) / 2 <= self.max_value {
            println!("Prune");
            return 0;
        }

        if memo.contains_key(state) {
            return *memo.get(state).unwrap();
        }

        let mut can_build_robot = false;

        // Construct a geode robot.
        let construct_geo = if state.ore >= self.geode.0 && state.obsidian >= self.geode.1 && can_build & 0b1 > 0 {
            can_build_robot = true;
            // println!("Time to construct (time={})", state.time_left);
            let new_state = State {
                ore: state.ore - self.geode.0 + state.robot_ore,
                clay: state.clay + state.robot_clay,
                obsidian: state.obsidian - self.geode.1 + state.robot_obs,
                geode: state.geode + state.robot_geo,
                robot_ore: state.robot_ore,
                robot_clay: state.robot_clay,
                robot_obs: state.robot_obs,
                robot_geo: state.robot_geo + 1,
                time_left: state.time_left - 1,
            };
            self.dfs(&new_state, value + state.robot_geo, memo, 0b1111)
        } else {
            0
        };

        // Construct an obsidian robot.
        let construct_obs = if construct_geo == 0 && state.ore >= self.obsidian.0 && state.clay >= self.obsidian.1 && can_build & 0b10 > 0 {
            // If I have enough robots.
            can_build_robot = true;
            let max_resource = self.geode.1;
            if max_resource <= state.robot_obs {
                0
            } else {
                let new_state = State {
                    ore: state.ore - self.obsidian.0 + state.robot_ore,
                    clay: state.clay - self.obsidian.1 + state.robot_clay,
                    obsidian: state.obsidian + state.robot_obs,
                    geode: state.geode + state.robot_geo,
                    robot_ore: state.robot_ore,
                    robot_clay: state.robot_clay,
                    robot_obs: state.robot_obs + 1,
                    robot_geo: state.robot_geo,
                    time_left: state.time_left - 1,
                };
                self.dfs(&new_state, value + state.robot_geo, memo, 0b1111)
            }

        } else {
            0
        };

        // Construct a clay robot.
        let construct_clay = if construct_geo == 0 && state.ore >= self.clay && can_build & 0b100 > 0 {
            can_build_robot = true;
            
            let max_resource = self.obsidian.1;
            if max_resource <= state.robot_clay {
                0
            } else {
                let new_state = State {
                    ore: state.ore + state.robot_ore - self.clay,
                    clay: state.clay + state.robot_clay,
                    obsidian: state.obsidian + state.robot_obs,
                    geode: state.geode + state.robot_geo,
                    robot_ore: state.robot_ore,
                    robot_clay: state.robot_clay + 1,
                    robot_obs: state.robot_obs,
                    robot_geo: state.robot_geo,
                    time_left: state.time_left - 1,
                };
                self.dfs(&new_state, value + state.robot_geo, memo, 0b1111)
            }
        } else {
            0
        };

        // Construct an ore robot.
        let construct_ore = if construct_geo == 0 && state.ore >= self.ore && can_build & 0b1000 > 0 {
            can_build_robot = true;

            let max_resource = *[self.ore, self.clay, self.obsidian.0, self.geode.0].iter().max().unwrap();
            if max_resource <= state.robot_ore {
                0
            } else {
                let new_state = State {
                    ore: state.ore - self.ore + state.robot_ore,
                    clay: state.clay + state.robot_clay,
                    obsidian: state.obsidian + state.robot_obs,
                    geode: state.geode + state.robot_geo,
                    robot_ore: state.robot_ore + 1,
                    robot_clay: state.robot_clay,
                    robot_obs: state.robot_obs,
                    robot_geo: state.robot_geo,
                    time_left: state.time_left - 1,
                };
                self.dfs(&new_state, value + state.robot_geo, memo, 0b1111)
            }
        } else {
            0
        };

        // Do nothing.
        // = Wait for the next robot to build.
        // Wait for ore.
        let wait_ore = if construct_ore == 0 {
            if state.robot_ore == 0 {
                0
            } else {
                let number_wait = (((self.ore - state.ore) as f64) / (state.robot_ore as f64)).ceil() as i32;
                if number_wait <= 0 || state.time_left < number_wait {
                    0
                } else {
                   number_wait
                }
            }
        } else {
            0
        };

        // Wait for clay.
        let wait_clay = if construct_clay == 0 {
            if state.robot_ore == 0 {
                0
            } else {
                let number_wait = (((self.clay - state.ore) as f64) / (state.robot_ore as f64)).ceil() as i32;
                if number_wait <= 0 || state.time_left < number_wait {
                    0
                } else {
                    number_wait
                }
            }
        } else {
            0
        };

        // Wait for obsidian.
        let wait_obsidian = if construct_obs == 0 {
            if state.robot_ore == 0 || state.robot_clay == 0 {
                0
            } else {
                let number_wait = ((((self.obsidian.0 - state.ore) as f64) / (state.robot_ore as f64)).ceil() as i32).max((((self.obsidian.1 - state.clay) as f64) / (state.robot_clay as f64)).ceil() as i32);
                if number_wait <= 0 || state.time_left < number_wait {
                    0
                } else {
                    number_wait
                }
            }
        } else {
            0
        };

        // Wait for geode.
        let wait_geode = if construct_geo == 0 {
            if state.robot_ore == 0 || state.robot_obs == 0 {
                0
            } else {
                let number_wait = ((((self.geode.0 - state.ore) as f64) / (state.robot_ore as f64)).ceil() as i32).max((((self.geode.1 - state.obsidian) as f64) / (state.robot_obs as f64)).ceil() as i32);
                if number_wait <= 0 || state.time_left < number_wait {
                    0
                } else {
                    number_wait
                }
            }
        } else {
            0
        };

        let tmp = [wait_clay, wait_geode, wait_obsidian, wait_ore];
        let number_wait = tmp.iter().filter(|&&i| i > 0).min();
        let res_wait = match number_wait {
            Some(number_wait) if !can_build_robot => {
                let state = State {
                    ore: state.ore + state.robot_ore * number_wait,
                    clay: state.clay + state.robot_clay * number_wait,
                    obsidian: state.obsidian + state.robot_obs * number_wait,
                    geode: state.geode + state.robot_geo * number_wait,
                    robot_ore: state.robot_ore,
                    robot_clay: state.robot_clay,
                    robot_obs: state.robot_obs,
                    robot_geo: state.robot_geo,
                    time_left: state.time_left - number_wait,
                };
                self.dfs(&state, value + state.robot_geo * number_wait, memo, 0b1111)
            },
            _ => 0,
        };


        let max = *[
            construct_ore,
            construct_clay,
            construct_obs,
            construct_geo,
            res_wait,
        ]
        .iter()
        .max()
        .unwrap();

        memo.insert(state.to_owned(), max);

        self.max_value = self.max_value.max(state.robot_geo);

        max
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    robot_ore: i32,
    robot_clay: i32,
    robot_obs: i32,
    robot_geo: i32,
    time_left: i32,
}

fn main() {
    let content = include_str!("../../inputs/19.txt");

    let mut blueprints: Vec<_> = content
        .split('\n')
        .map(|line| {
            let split: Vec<_> = line.split(" ").collect();
            let ore = split[6].parse().unwrap();
            let clay = split[12].parse().unwrap();
            let obs_ore = split[18].parse().unwrap();
            let obs_clay = split[21].parse().unwrap();
            let geo_ore = split[27].parse().unwrap();
            let geo_obs = split[30].parse().unwrap();

            Blueprint {
                ore,
                clay,
                obsidian: (obs_ore, obs_clay),
                geode: (geo_ore, geo_obs),
                max_value: 0,
            }
        })
        .collect();

    println!("Blueprints: {:?}", blueprints);

    let pool = ThreadPool::new(4);
    let mut output = Vec::with_capacity(blueprints.len());
    let (tx, rx) = channel();
    for (i, blueprint) in blueprints.iter().enumerate() {
        if i >= 3 {
            break; // Only first three
        }
        let tx = tx.clone();
        let mut blueprint = blueprint.clone();
        pool.execute(move || {
            let mut memo = HashMap::new();
            let init_state = State {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                robot_ore: 1,
                robot_clay: 0,
                robot_obs: 0,
                robot_geo: 0,
                time_left: 32,
            };
            let v = blueprint.dfs(&init_state, 0, &mut memo, 0b1111);
            tx.send((i, v)).unwrap();
        });
    }

    for _ in 0..3 {
        let v = rx.recv().unwrap();
        output.push(v);
    }
    println!("Output: {:?}", output);

    let res: i32 = output.iter().map(|(_, v)| v).product();
    println!("Res 1: {}", res);

    // let bests: Vec<_> = [&mut blueprints[0]]
    //     .iter_mut()
    //     .map(|blueprint| {
    //         let mut memo = HashMap::new();
    //         let init_state = State {
    //             ore: 0,
    //             clay: 0,
    //             obsidian: 0,
    //             geode: 0,
    //             robot_ore: 1,
    //             robot_clay: 0,
    //             robot_obs: 0,
    //             robot_geo: 0,
    //             time_left: 32,
    //         };
    //         let v = blueprint.dfs(&init_state, 0, &mut memo, 0b1111);
    //         v
    //     })
    //     .collect();

    // println!("Bests: {:?}", bests);

    // let res: i32 = bests
    //     .iter()
    //     .enumerate()
    //     .map(|(i, v)| (i as i32 + 1) * v)
    //     .sum();
}
