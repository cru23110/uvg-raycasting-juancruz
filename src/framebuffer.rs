use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        Self {
            width, height, color_buffer,
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    #[inline] pub fn set_background_color(&mut self, c: Color) { self.background_color = c; }
    #[inline] pub fn set_current_color(&mut self, c: Color) { self.current_color = c; }

    #[inline]
    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, self.current_color);
        }
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, rl: &RaylibThread) {
        if let Ok(tex) = window.load_texture_from_image(rl, &self.color_buffer) {
            let win_w = window.get_screen_width() as f32;
            let win_h = window.get_screen_height() as f32;
            let src = Rectangle::new(0.0, 0.0, tex.width() as f32, tex.height() as f32);
            let dst = Rectangle::new(0.0, 0.0, win_w, win_h);
            let mut d = window.begin_drawing(rl);
            d.clear_background(Color::BLACK);
            d.draw_texture_pro(&tex, src, dst, Vector2::zero(), 0.0, Color::WHITE);
        }
    }
}