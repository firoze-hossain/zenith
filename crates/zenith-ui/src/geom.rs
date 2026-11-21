//! 2D geometry and ARGB color primitives for software rendering.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x,
            y,
            w: w.max(0),
            h: h.max(0),
        }
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.right() && py >= self.y && py < self.bottom()
    }

    pub fn inset(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.w - dx * 2, self.h - dy * 2)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Color {
    pub argb: u32,
}

impl Color {
    pub const BLACK: Color = Color { argb: 0xFF000000 };
    pub const WHITE: Color = Color { argb: 0xFFFFFFFF };
    pub const TRANSPARENT: Color = Color { argb: 0x00000000 };

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color {
            argb: 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color {
            argb: ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub fn a(&self) -> u8 {
        ((self.argb >> 24) & 0xFF) as u8
    }

    pub fn r(&self) -> u8 {
        ((self.argb >> 16) & 0xFF) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.argb >> 8) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        (self.argb & 0xFF) as u8
    }

    pub fn lighten(&self, amount: u8) -> Self {
        Self::rgb(
            self.r().saturating_add(amount),
            self.g().saturating_add(amount),
            self.b().saturating_add(amount),
        )
    }

    pub fn darken(&self, amount: u8) -> Self {
        Self::rgb(
            self.r().saturating_sub(amount),
            self.g().saturating_sub(amount),
            self.b().saturating_sub(amount),
        )
    }

    pub fn blend(&self, over: Color) -> Self {
        let alpha = over.a() as u32;
        if alpha == 255 {
            return over;
        }
        if alpha == 0 {
            return *self;
        }

        let inv_alpha = 255 - alpha;
        let r = ((over.r() as u32 * alpha + self.r() as u32 * inv_alpha) / 255) as u8;
        let g = ((over.g() as u32 * alpha + self.g() as u32 * inv_alpha) / 255) as u8;
        let b = ((over.b() as u32 * alpha + self.b() as u32 * inv_alpha) / 255) as u8;
        Self::rgb(r, g, b)
    }
}
