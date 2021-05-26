pub type Size = u8;
pub type Id = u8;

#[derive(Copy, Clone, Debug)]
pub struct Pacman {
    pub x: Size,
    pub y: Size,
    pub w: Size,
    pub h: Size
}

impl Pacman {
    pub fn new(x: Size, y: Size, w: Size, h: Size) -> Self {
        Pacman { 
            x: add_pacman(0, x, w),
            y: add_pacman(0, y, h),
            w, h
        }
    }

    pub fn equals(&self, other: Self) -> bool {
        sub_pacman(0, other.x, self.w) == sub_pacman(0, self.x, self.w)
        && sub_pacman(0, other.y, self.h) == sub_pacman(0, self.y, self.h)
    }

    pub fn sub(&mut self, x: Size, y: Size) {
        self.x = sub_pacman(self.x, x, self.w);
        self.y = sub_pacman(self.y, y, self.h);
    }

    pub fn add(&mut self, x: Size, y: Size) {
        self.x = add_pacman(self.x, x, self.w);
        self.y = add_pacman(self.y, y, self.h);
    }
}

fn sub_pacman(a: Size, b: Size, max: Size) -> Size {
    if a > b {
        a - b
    } else if a == b {
        a
    } else {
        max - (b - a)
    }
}

fn add_pacman(a: Size, b: Size, max: Size) -> Size {
    if a > Size::MAX-b {
        (a % max) + (b % max)
    } else {
        (a + b) % max
    }
}