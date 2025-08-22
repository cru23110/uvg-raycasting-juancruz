use raylib::prelude::*;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vector2,   
    pub ang: f32,       
    pub fov: f32,       
}

impl Player {
    pub fn new(pos: Vector2) -> Self {
        Self { pos, ang: PI/3.0, fov: PI/3.0 }
    }
}

pub struct InputState {
    pub move_forward: bool,
    pub move_back: bool,
    pub strafe_left: bool,
    pub strafe_right: bool,
    pub turn_left: bool,
    pub turn_right: bool,
}

pub fn read_input(rl: &RaylibHandle) -> InputState {
    InputState {
        move_forward: rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP),
        move_back:    rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN),
        strafe_left:  rl.is_key_down(KeyboardKey::KEY_A),
        strafe_right: rl.is_key_down(KeyboardKey::KEY_D),
        turn_left:    rl.is_key_down(KeyboardKey::KEY_LEFT),
        turn_right:   rl.is_key_down(KeyboardKey::KEY_RIGHT),
    }
}