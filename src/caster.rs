use crate::framebuffer::Framebuffer;
use crate::maze::{Maze, is_wall, dims};

use raylib::prelude::*;
// use std::f32::consts::PI;

pub fn wall_color(c: char) -> Color {
    match c {
        '#' => Color::new(40, 40, 160, 255),
        '1' => Color::new(180, 60, 60, 255),
        '2' => Color::new(60, 170, 80, 255),
        '3' => Color::new(200, 160, 40, 255),
        '4' => Color::new(140, 80, 200, 255),
        _   => Color::GRAY,
    }
}

pub struct Hit {
    pub dist: f32,
    pub cell: char,
    pub side: u8,
}

pub fn cast_ray(maze: &Maze, origin: Vector2, angle: f32) -> Hit {
    let (mw, mh) = dims(maze);

    let dx = angle.cos();
    let dy = angle.sin();

    let mut map_x = origin.x.floor() as i32;
    let mut map_y = origin.y.floor() as i32;

    let delta_dist_x = if dx.abs() < 1e-6 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy.abs() < 1e-6 { 1e30 } else { (1.0 / dy).abs() };

    let (step_x, mut side_dist_x) = if dx < 0.0 {
        (-1, (origin.x - map_x as f32) * delta_dist_x)
    } else {
        ( 1, ((map_x + 1) as f32 - origin.x) * delta_dist_x)
    };
    let (step_y, mut side_dist_y) = if dy < 0.0 {
        (-1, (origin.y - map_y as f32) * delta_dist_y)
    } else {
        ( 1, ((map_y + 1) as f32 - origin.y) * delta_dist_y)
    };

    let (hit_side, hit_cell);

    loop {
        let stepped_side =
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                0u8
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                1u8
            };

        if map_x < 0 || map_y < 0 || map_x >= mw as i32 || map_y >= mh as i32 {
            return Hit { dist: 1.0e6, cell: '#', side: stepped_side };
        }

        let c = maze[map_y as usize][map_x as usize];
        if is_wall(c) {
            hit_side = stepped_side;
            hit_cell = c;
            break;
        }
    }

    let dist = if hit_side == 0 {
        (map_x as f32 - origin.x + (1 - step_x) as f32 * 0.5) / dx.max(1e-6)
    } else {
        (map_y as f32 - origin.y + (1 - step_y) as f32 * 0.5) / dy.max(1e-6)
    }.abs();

    Hit { dist, cell: hit_cell, side: hit_side }
}

fn vline(fb: &mut Framebuffer, x: i32, y0: i32, y1: i32) {
    let y0 = y0.max(0);
    let y1 = y1.min(fb.height as i32 - 1);
    for y in y0..=y1 { fb.set_pixel(x, y); }
}

pub fn render_3d(fb: &mut Framebuffer, maze: &Maze, player_pos: Vector2, player_ang: f32, fov: f32) {
    let w = fb.width as i32;
    let h = fb.height as i32;

    fb.set_current_color(Color::new(180, 160, 200, 255));
    for y in 0..h/2 { for x in 0..w { fb.set_pixel(x, y); } }
    fb.set_current_color(Color::new(60, 50, 70, 255));
    for y in h/2..h { for x in 0..w { fb.set_pixel(x, y); } }

    for x in 0..w {
        let cam_x = 2.0 * x as f32 / (w as f32) - 1.0;
        let ray_ang = player_ang + cam_x * fov * 0.5;

        let hit = cast_ray(maze, player_pos, ray_ang);

        let dist = hit.dist.max(0.0001);
        let line_h = ((h as f32) / dist) as i32;

        let draw_start = -line_h/2 + h/2;
        let draw_end   =  line_h/2 + h/2;

        let mut col = wall_color(hit.cell);
        if hit.side == 1 {col = Color::new(col.r / 2, col.g / 2, col.b / 2, 255);}

        fb.set_current_color(col);
        vline(fb, x, draw_start, draw_end);
    }
}

pub fn render_minimap(fb: &mut Framebuffer, maze: &Maze, player_pos: Vector2, player_ang: f32) {
    let scale = 4i32; 
    let margin = 8i32;
    let (mw, mh) = (maze[0].len() as i32, maze.len() as i32);

    fb.set_current_color(Color::new(230, 220, 230, 255));
    for y in 0..mh*scale {
        for x in 0..mw*scale { fb.set_pixel(margin + x, margin + y); }
    }

    for my in 0..mh {
        for mx in 0..mw {
            let c = maze[my as usize][mx as usize];
            if super::maze::is_wall(c) {
                fb.set_current_color(Color::new(80, 80, 120, 255));
                for yy in 0..scale {
                    for xx in 0..scale {
                        fb.set_pixel(margin + mx*scale + xx, margin + my*scale + yy);
                    }
                }
            }
        }
    }

    let px = (player_pos.x * scale as f32) as i32 + margin;
    let py = (player_pos.y * scale as f32) as i32 + margin;
    fb.set_current_color(Color::RED);
    fb.set_pixel(px, py);
    fb.set_pixel(px+1, py);
    fb.set_pixel(px, py+1);

    let dx = (player_ang.cos() * 6.0) as i32;
    let dy = (player_ang.sin() * 6.0) as i32;
    fb.set_pixel(px+dx, py+dy);
}