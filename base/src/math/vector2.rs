#[derive(Copy, Clone, Eq, Debug)]
#[allow(non_camel_case_types)]
pub struct int2 {
    pub x: i32,
    pub y: i32,
}

impl int2 {
    pub fn identity() -> int2 {
        int2 { x: 0, y: 0 }
    }

    pub fn new(x: i32, y: i32) -> int2 {
        int2 { x, y }
    }
}

impl PartialEq for int2 {
    fn eq(&self, other: &int2) -> bool {
        self.x == other.x && self.y == other.y
    }
}
