use std::{f32::consts::PI, fs::OpenOptions, io::Write, ops::Rem};

use rand::prelude::*;
use serde::Serialize;

pub enum GameCommand {
    Join { id: u8 },
    Strike { id: u8, strike: [u8; 3] },
    Leave { id: u8 },
    None,
}

#[derive(Serialize)]
pub struct Player {
    pub id: u8,
    pub ball_pos_x: f32,
    pub ball_pos_y: f32,
    pub ball_height: f32,
    pub ball_vel_dir: f32,
    pub ball_vel_amount: f32,
    pub ball_vel_height: f32,
    pub in_hole: bool,
}

impl Player {
    pub fn new(id: u8) -> Self {
        Player {
            id,
            ball_pos_x: 0.0,
            ball_pos_y: 0.0,
            ball_height: 0.0,
            ball_vel_dir: 0.0,
            ball_vel_amount: 0.0,
            ball_vel_height: 0.0,
            in_hole: false,
        }
    }
}

#[derive(Serialize)]
pub struct GameState {
    players: Vec<Player>,
    wind: Wind,
    //#[serde(skip_serializing)] // we do that manually
    map: Map,
}

impl GameState {
    /// the main game loop
    pub fn calc_next_state(&mut self, game_command: GameCommand, delta_time: f32) {
        const HIT_STRENGHT: f32 = 100.0;
        const GROUND_DRAG: f32 = 1.0;
        const GRAVITY: f32 = 55.0;
        const _SAND_DRAG: f32 = 0.5;
        const AIR_DRAG: f32 = 0.5;

        // use incomming Command
        match game_command {
            GameCommand::Join { id } => {
                self.players.push(Player::new(id));
            }
            GameCommand::Strike { id, strike } => {
                for player in self.players.iter_mut() {
                    if player.id == id {
                        player.ball_vel_height =
                            strike[0] as f32 * (strike[1] as f32 / 250.0) * 7.0;
                        player.ball_vel_amount = strike[1] as f32 / 255.0;
                        player.ball_vel_dir = (360.0 / 255.0) * strike[2] as f32;
                        break;
                    }
                }
            }
            GameCommand::Leave { id } => {
                let mut leaving_index: i16 = -1; //spooky sentinal value
                for (i, player) in self.players.iter_mut().enumerate() {
                    if id == player.id {
                        leaving_index = i as i16;
                        break;
                    }
                }
                if leaving_index == -1 {
                    println!("ERROR: no such player id to leave");
                } else {
                    self.players.swap_remove(leaving_index as usize);
                }
            }
            GameCommand::None => {}
        }

        // apply our amazing physics to every Ball not in the goal
        let mut every_player_in_hole = true;
        for player in self.players.iter_mut().filter(|p| !p.in_hole) {
            every_player_in_hole = false;

            player.ball_pos_x += player.ball_vel_dir.to_radians().cos()
                * player.ball_vel_amount
                * HIT_STRENGHT
                * delta_time;
            player.ball_pos_y += player.ball_vel_dir.to_radians().sin()
                * player.ball_vel_amount
                * HIT_STRENGHT
                * delta_time;

            player.ball_height += player.ball_vel_height * delta_time;
            player.ball_vel_height -= GRAVITY * delta_time;
            if player.ball_height < 0.0 {
                if player.ball_vel_height < -2.0 {
                    player.ball_vel_height = player.ball_vel_height.abs() / 2.0;
                }
                player.ball_height = 0.0;
            }

            if player.ball_vel_amount.abs() < GROUND_DRAG * delta_time {
                player.ball_vel_amount = 0.0;
            } else if player.ball_height < 0.0 {
                player.ball_vel_amount -= AIR_DRAG * delta_time;
            } else {
                player.ball_vel_amount -= GROUND_DRAG * delta_time;
            }
            player.ball_pos_x = player.ball_pos_x.rem_euclid(64.0);
            player.ball_pos_y = player.ball_pos_y.rem_euclid(64.0);

            // TODO: Wind physics

            // check if they are in the hole
            let hole_pos_x = (self.map.end_pos_x - 3.0)..(self.map.end_pos_x + 3.0);
            let hole_pos_y = (self.map.end_pos_y - 3.0)..(self.map.end_pos_y + 3.0);
            if hole_pos_x.contains(&player.ball_pos_x)
                && hole_pos_y.contains(&player.ball_pos_y)
                && player.ball_height < 0.1
            {
                player.in_hole = true;
            }
        }

        if every_player_in_hole {
            self.map = Map::new();
            self.wind.randomize();
            self.players.iter_mut().for_each(|p| {
                p.in_hole = false;
                p.ball_pos_x = self.map.start_pos_x;
                p.ball_pos_y = self.map.start_pos_y;
                p.ball_vel_amount = 0.0;
            });
        }
    }

    pub fn new() -> Self {
        GameState {
            players: vec![],
            wind: Wind::new(),
            map: Map::new(),
        }
    }
}

#[derive(Serialize)]
struct Wind {
    strenght: f32,
    direction: f32,
}

impl Wind {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let wind = Wind {
            strenght: rng.gen_range(0.8..1.0),
            direction: rng.gen_range(0.8..(2.0 * PI)),
        };
        return wind;
    }

    /// re-randomize strenght and direction
    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.strenght = rng.gen_range(0.8..1.0);
        self.direction = rng.gen_range(0.8..(2.0 * PI));
    }
}

// TODO: make a good map
#[derive(Serialize)]
struct Map {
    start_pos_x: f32,
    start_pos_y: f32,
    end_pos_x: f32,
    end_pos_y: f32,
}

impl Map {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let map = Map {
            start_pos_x: rng.gen_range(0.0..64.0),
            start_pos_y: rng.gen_range(0.0..64.0),
            end_pos_x: rng.gen_range(0.0..64.0),
            end_pos_y: rng.gen_range(0.0..64.0),
        };
        // write new map to file
        let mut map_file = OpenOptions::new()
            .truncate(true)
            .write(true)
            .open("map_info.json")
            .unwrap();
        _ = map_file.write(map.to_test_json().as_bytes());

        map
    }

    /// this really is only for producing some json for the panel - no need to read this
    pub fn to_test_json(&self) -> String {
        let mut result_arr: [[u8; 64]; 64] = [[0; 64]; 64];
        // everything green
        for (x, line) in result_arr.iter_mut().enumerate() {
            let mut should_be_hole_x = false;
            let mut should_be_hole_y = false;
            if ((self.end_pos_x - 3.0)..(self.end_pos_x + 3.0)).contains(&(x as f32)) {
                should_be_hole_x = true;
            }
            for (y, cell) in line.iter_mut().enumerate() {
                if ((self.end_pos_y - 3.0)..(self.end_pos_y + 3.0)).contains(&(y as f32)) {
                    should_be_hole_y = true;
                }
                if should_be_hole_x && should_be_hole_y {
                    *cell = 3;
                } else {
                    *cell = 0;
                }
            }
        }

        // testing other terrain
        result_arr[1][0] = 1;
        result_arr[2][0] = 2;

        // to json
        let mut result_json = String::new();
        result_json.push_str("{[");
        for (i, line) in result_arr.iter().enumerate() {
            result_json.push('[');
            for (j, cell) in line.iter().enumerate() {
                if j != 63 {
                    result_json.push_str(&format!("{cell},"));
                } else {
                    result_json.push_str(&format!("{cell}"));
                }
            }
            if i != 63 {
                result_json.push_str("],");
            } else {
                result_json.push_str("]");
            }
        }
        result_json.push_str("]}");
        return result_json;
    }
}

// ah hell naw
pub fn _step_towards_rad(start: f32, goal: f32, step: f32) -> f32 {
    let naive_dist = (goal - start).rem(2.0 * PI).abs();
    let actual_dist = (goal - start).rem(PI).abs();
    if actual_dist < step {
        return goal;
    }
    if naive_dist > PI {
        if start > goal {
            return (start + step).rem_euclid(2.0 * PI);
        }
        return (start - step).rem_euclid(2.0 * PI);
    }

    if start > goal {
        return (start - step).rem_euclid(2.0 * PI);
    }
    return (start + step).rem_euclid(2.0 * PI);
}
