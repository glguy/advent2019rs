use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pos {
    pub x: i64,
    pub y: i64,
}

impl Pos {
    pub const ORIGIN: Pos = Pos { x: 0, y: 0 };
    pub fn advance(mut self, dir: Dir, n: i64) -> Pos {
        match dir {
            Dir::N => { self.y -= n }
            Dir::S => { self.y += n }
            Dir::W => { self.x -= n }
            Dir::E => { self.x += n }
        }
        self
    }

    pub fn norm1(self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn turn_clockwise(self) -> Self {
        Pos {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn turn_counterclockwise(self) -> Self {
        Pos {
            x: self.y,
            y: -self.x,
        }
    }

    pub fn turn_around(self) -> Self {
        Pos {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    pub const ELEMS: [Dir; 4] = [Dir::N, Dir::S, Dir::E, Dir::W];

    pub fn turn_clockwise(self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }

    pub fn turn_counterclockwise(self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::S => Dir::E,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
        }
    }

    pub fn turn_around(self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub for Pos {
    type Output = Pos;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}
