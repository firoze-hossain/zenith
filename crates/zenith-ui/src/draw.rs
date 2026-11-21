//! 2D drawing routines for 32-bit linear surfaces.

use crate::geom::{Color, Rect};

pub trait Surface {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn put_pixel(&mut self, x: i32, y: i32, color: Color);
    fn get_pixel(&self, x: i32, y: i32) -> Color;

    fn fill_rect(&mut self, rect: Rect, color: Color) {
        let x0 = rect.x.max(0);
        let y0 = rect.y.max(0);
        let x1 = rect.right().min(self.width());
        let y1 = rect.bottom().min(self.height());

        for y in y0..y1 {
            for x in x0..x1 {
                if color.a() == 255 {
                    self.put_pixel(x, y, color);
                } else {
                    let cur = self.get_pixel(x, y);
                    self.put_pixel(x, y, cur.blend(color));
                }
            }
        }
    }
}

pub struct BufferSurface32 {
    pub width: i32,
    pub height: i32,
    pub buffer: Vec<u32>,
}

impl BufferSurface32 {
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width.max(0) as usize) * (height.max(0) as usize);
        Self {
            width,
            height,
            buffer: vec![0xFF000000; size],
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.buffer.fill(color.argb);
    }
}

impl Surface for BufferSurface32 {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            self.buffer[idx] = color.argb;
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> Color {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            let idx = (y as usize) * (self.width as usize) + (x as usize);
            Color { argb: self.buffer[idx] }
        } else {
            Color::BLACK
        }
    }
}

pub fn draw_line<S: Surface + ?Sized>(s: &mut S, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    let mut cx = x0;
    let mut cy = y0;
    loop {
        s.put_pixel(cx, cy, color);
        if cx == x1 && cy == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            cx += sx;
        }
        if e2 <= dx {
            err += dx;
            cy += sy;
        }
    }
}

pub fn draw_rect_outline<S: Surface + ?Sized>(s: &mut S, r: Rect, thickness: i32, color: Color) {
    if r.w <= 0 || r.h <= 0 || thickness <= 0 {
        return;
    }
    // Top & Bottom
    s.fill_rect(Rect::new(r.x, r.y, r.w, thickness), color);
    s.fill_rect(Rect::new(r.x, r.y + r.h - thickness, r.w, thickness), color);
    // Left & Right
    s.fill_rect(Rect::new(r.x, r.y, thickness, r.h), color);
    s.fill_rect(Rect::new(r.x + r.w - thickness, r.y, thickness, r.h), color);
}

pub fn draw_rounded_rect<S: Surface + ?Sized>(s: &mut S, r: Rect, radius: i32, color: Color) {
    if r.w <= 0 || r.h <= 0 {
        return;
    }
    let rad = radius.min(r.w / 2).min(r.h / 2).max(0);
    if rad == 0 {
        s.fill_rect(r, color);
        return;
    }

    // Center cross fill
    s.fill_rect(Rect::new(r.x + rad, r.y, r.w - rad * 2, r.h), color);
    s.fill_rect(Rect::new(r.x, r.y + rad, rad, r.h - rad * 2), color);
    s.fill_rect(Rect::new(r.right() - rad, r.y + rad, rad, r.h - rad * 2), color);

    // Rounded corner caps
    for dy in 0..rad {
        for dx in 0..rad {
            let dist_sq = (rad - 1 - dx) * (rad - 1 - dx) + (rad - 1 - dy) * (rad - 1 - dy);
            if dist_sq <= rad * rad {
                // Top-left
                s.put_pixel(r.x + dx, r.y + dy, color);
                // Top-right
                s.put_pixel(r.right() - 1 - dx, r.y + dy, color);
                // Bottom-left
                s.put_pixel(r.x + dx, r.bottom() - 1 - dy, color);
                // Bottom-right
                s.put_pixel(r.right() - 1 - dx, r.bottom() - 1 - dy, color);
            }
        }
    }
}
