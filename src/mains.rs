use js;
use map;
use na;
use render;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

pub struct PlayerState {
    pub pos:    na::Point3<f32>,
    pub orient: na::Unit<na::Vector3<f32>>,
}

lazy_static! {
    pub static ref PLAYER_STATE: Mutex<PlayerState> =
        Mutex::new(PlayerState::new());
}

impl PlayerState {
    #[inline]
    pub fn new() -> Self {
        PlayerState {
            pos:    na::Point3::origin(),
            orient: na::Unit::new_unchecked(na::Vector3::new(0.0, 0.0, -1.0)),
        }
    }
}

#[wasm_bindgen]
pub fn init() {
    render::init();
}

#[wasm_bindgen]
pub fn load_map(map_data: &[u8]) -> i32 {
    if let Some(map_data) = map::MapData::from_raw_data(map_data) {
        let mut map_state = map::MAP.lock().unwrap();
        *map_state = map::Map::from_map_data(map_data);

        let radius = map_state.get_radius();
        let (_, (x, y)) = map_state.get_hexes()[radius][radius];
        PLAYER_STATE.lock().unwrap().pos = na::Point3::new(x, 2.0, -y);

        0
    } else {
        1
    }
}

#[wasm_bindgen]
pub fn main_loop(_time_stamp: f64, event_queue: &js::Uint16Vec) {
    for i in 0..event_queue.len() {
        let event_data = event_queue.get(i);
        match (event_data >> 8) as u8 {
            js::KEY_DOWN => match (event_data & 0x00FF) as u8 {
                js::key::W => {
                    move_player(0.5, true);
                },
                js::key::A => {
                    turn_player(0.0, 0.1, 0.0);
                },
                js::key::S => {
                    move_player(0.5, false);
                },
                js::key::D => {
                    turn_player(0.0, -0.1, 0.0);
                },
                _ => (),
            },
            js::KEY_UP => {},
            _ => (),
        }
    }

    render::render();
}

#[inline]
fn move_player(d: f32, forward: bool) {
    let mut player_state = PLAYER_STATE.lock().unwrap();
    player_state.pos +=
        if forward { d } else { -d } * player_state.orient.unwrap();
}

#[inline]
fn turn_player(a: f32, b: f32, c: f32) {
    let mut player_state = PLAYER_STATE.lock().unwrap();
    player_state.orient = na::Unit::new_normalize(
        na::Rotation3::from_euler_angles(a, b, c)
            * player_state.orient.unwrap(),
    );
}
