use byteorder::{NativeEndian, ReadBytesExt};
use controls::{self, Key};
use error::{log_and_return, Error};
use js::{self, EventType};
use map;
use na;
use physics;
use render;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use webgl_test_common::MapData;

pub struct PlayerState {
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
            orient: na::Unit::new_unchecked(na::Vector3::new(0.0, 0.0, -1.0)),
        }
    }
}

impl Default for PlayerState {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub fn init_bg() -> i32 {
    log_and_return(init())
}

#[inline]
fn init() -> Result<(), Error> {
    render::init()
}

#[wasm_bindgen]
pub fn load_map_bg(map_data: &[u8]) -> i32 {
    log_and_return(load_map(map_data))
}

#[inline]
fn load_map(map_data: &[u8]) -> Result<(), Error> {
    let map_data = MapData::from_raw_data(map_data)?;

    let mut map_state = map::MAP.lock().unwrap();
    *map_state = map::Map::from_map_data(&map_data)?;

    let radius = map_state.get_radius();
    let (_, (x, y)) = map_state.get_hexes()[radius][radius];
    physics::init_world(&map_state, &na::Point3::new(x, 4.0, -y));

    Ok(())
}

#[wasm_bindgen]
pub fn main_loop_bg(time_stamp: f64, event_queue: &js::EventQueue) -> i32 {
    log_and_return(main_loop(time_stamp, event_queue))
}

fn main_loop(
    _time_stamp: f64,
    event_queue: &js::EventQueue,
) -> Result<(), Error> {
    // Handle events sent from JS "event queue"
    for i in 0..event_queue.len() {
        let event_data = event_queue.get(i);
        let event_opcode = event_data.opcode();

        if let Some(e) = EventType::from_u8(event_opcode) {
            match e {
                EventType::KeyDown =>
                    if let Some(k) = Key::from_u8(event_data.payload()[0]) {
                        controls::press(k);
                    },
                EventType::KeyUp =>
                    if let Some(k) = Key::from_u8(event_data.payload()[0]) {
                        controls::release(k);
                    },
                EventType::MouseMove => {
                    let payload = event_data.payload();
                    controls::handle_mouse_movement(
                        payload.as_ref().read_f32::<NativeEndian>()?,
                        payload[4..].as_ref().read_f32::<NativeEndian>()?,
                    );
                },
            }
        }
    }

    // Handle pressed keys
    if controls::is_pressed(Key::W) {
        let player_state = PLAYER_STATE.lock().unwrap();
        let lin_acc = player_state.orient.unwrap() * physics::CONTROL_FORCE;

        physics::update_control_acc(lin_acc);
    } else if controls::is_pressed(Key::S) {
        let player_state = PLAYER_STATE.lock().unwrap();
        let lin_acc = player_state.orient.unwrap() * -physics::CONTROL_FORCE;

        physics::update_control_acc(lin_acc);
    } else {
        physics::update_control_acc(na::Vector3::zeros());
    }

    // Run physics
    physics::step();

    // Render to screen
    render::render()?;

    Ok(())
}
