use mains;
use na;
use std::{f32::consts::PI, intrinsics, sync::Mutex};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    W = 0x01,
    A = 0x02,
    S = 0x03,
    D = 0x04,
}

pub struct ControlState {
    pressed: u8,
}

pub struct PressedKeysIter {
    pressed: u8,
    i:       u8,
}

const MOUSE_SENSITIVITY: f32 = 1.0 / 128.0;
const PITCH_MARGIN: f32 = PI / 128.0;

lazy_static! {
    static ref CONTROL_STATE: Mutex<ControlState> =
        Mutex::new(ControlState::new());
}

#[inline]
pub fn is_pressed(key: Key) -> bool {
    CONTROL_STATE.lock().unwrap().is_pressed(key)
}

#[inline]
pub fn press(key: Key) {
    CONTROL_STATE.lock().unwrap().press(key);
}

#[inline]
pub fn release(key: Key) {
    CONTROL_STATE.lock().unwrap().release(key);
}

#[inline]
pub fn pressed_iter() -> PressedKeysIter {
    CONTROL_STATE.lock().unwrap().pressed_iter()
}

pub fn handle_mouse_movement(mouse_x: f32, mouse_y: f32) {
    let mut player_state = mains::PLAYER_STATE.lock().unwrap();
    let orient = player_state.orient.unwrap();

    let curr_pitch = na::Vector3::y().dot(&orient).acos();
    let mut delta_pitch =
        unsafe { intrinsics::fmul_fast(MOUSE_SENSITIVITY, mouse_y) };
    let new_pitch = unsafe { intrinsics::fadd_fast(curr_pitch, delta_pitch) };
    if new_pitch < PITCH_MARGIN {
        delta_pitch =
            unsafe { intrinsics::fsub_fast(curr_pitch, PITCH_MARGIN) };
    } else if new_pitch > PI - PITCH_MARGIN {
        delta_pitch =
            unsafe { intrinsics::fsub_fast(curr_pitch, PI - PITCH_MARGIN) };
    }

    player_state.orient = na::Unit::new_normalize(
        na::Rotation3::new(
            na::Vector3::y().cross(&orient).normalize() * delta_pitch,
        ) * na::Rotation3::new(na::Vector3::new(
            0.0,
            unsafe { intrinsics::fmul_fast(-MOUSE_SENSITIVITY, mouse_x) },
            0.0,
        )) * orient,
    );
}

impl Key {
    #[inline]
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            n if n == Key::W as u8 => Some(Key::W),
            n if n == Key::A as u8 => Some(Key::A),
            n if n == Key::S as u8 => Some(Key::S),
            n if n == Key::D as u8 => Some(Key::D),
            _ => None,
        }
    }
}

impl ControlState {
    #[inline]
    pub fn new() -> Self {
        Self { pressed: 0 }
    }

    #[inline]
    pub fn is_pressed(&self, key: Key) -> bool {
        self.pressed & (1 << key as u8) != 0
    }

    #[inline]
    pub fn press(&mut self, key: Key) {
        self.pressed |= 1 << key as u8;
    }

    #[inline]
    pub fn release(&mut self, key: Key) {
        self.pressed &= !(1 << key as u8);
    }

    #[inline]
    pub fn pressed_iter(&self) -> PressedKeysIter {
        PressedKeysIter {
            pressed: self.pressed,
            i:       0,
        }
    }
}

impl Iterator for PressedKeysIter {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 8 && self.pressed & (1 << self.i) == 0 {
            self.i += 1;
        }
        if self.i == 8 {
            return None;
        }

        let ret = Key::from_u8(self.i);
        self.i += 1;

        ret
    }
}
