mod framebuffer;
mod maze;
mod player;
mod caster;

use framebuffer::Framebuffer;
use maze::{load_maze, find_player, dims, is_wall};
use player::{Player, read_input};
use caster::{render_3d, render_minimap};

use raylib::prelude::*;
use std::{thread, time::Duration};

const WIN_W: i32 = 800;
const WIN_H: i32 = 600;

fn try_move(pos: &mut Vector2, dx: f32, dy: f32, mz: &maze::Maze) {
    let (mw, mh) = dims(mz);
    let nx = (pos.x + dx).clamp(0.0, (mw as f32) - 0.0001);
    let ny = (pos.y + dy).clamp(0.0, (mh as f32) - 0.0001);

    let cx = nx.floor() as usize;
    let cy = ny.floor() as usize;

    if !is_wall(mz[cy][cx]) {
        pos.x = nx;
        pos.y = ny;
    } else {
        let cx_x = (pos.x + dx).floor() as usize;
        let cy_y = (pos.y + dy).floor() as usize;

        if !is_wall(mz[cy][(pos.x).floor() as usize]) { pos.y = ny; }
        if !is_wall(mz[(pos.y).floor() as usize][cx]) { pos.x = nx; }
        let _ = (cx_x, cy_y);
    }
}

fn main() {
    let (mut rl, th) = raylib::init()
        .size(WIN_W, WIN_H)
        .title("UVG - Raycasting (Juan Cruz)")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let fb_w: u32 = 320;
    let fb_h: u32 = 200;
    let mut fb = Framebuffer::new(fb_w, fb_h);
    fb.set_background_color(Color::new(30, 28, 40, 255));

    let mz = load_maze("maze.txt");
    let start = find_player(&mz);
    let mut player = Player::new(Vector2::new(start.0, start.1));

    let mut show_minimap = true;
    let frame_ms: u64 = 16;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_M) { show_minimap = !show_minimap; }
        let inp = read_input(&rl);

        let move_speed = 3.0 / 60.0;
        let rot_speed  = 2.4 / 60.0;

        if inp.turn_left  { player.ang -= rot_speed; }
        if inp.turn_right { player.ang += rot_speed; }

        let dir_x = player.ang.cos();
        let dir_y = player.ang.sin();
        let right_x =  dir_y;
        let right_y = -dir_x;

        let mut dx = 0.0;
        let mut dy = 0.0;

        if inp.move_forward { dx += dir_x * move_speed; dy += dir_y * move_speed; }
        if inp.move_back    { dx -= dir_x * move_speed; dy -= dir_y * move_speed; }
        if inp.strafe_left  { dx -= right_x * move_speed; dy -= right_y * move_speed; }
        if inp.strafe_right { dx += right_x * move_speed; dy += right_y * move_speed; }

        try_move(&mut player.pos, dx, dy, &mz);

        fb.clear();
        render_3d(&mut fb, &mz, player.pos, player.ang, player.fov);
        if show_minimap { render_minimap(&mut fb, &mz, player.pos, player.ang); }
        fb.swap_buffers(&mut rl, &th);

        thread::sleep(Duration::from_millis(frame_ms));
    }
}