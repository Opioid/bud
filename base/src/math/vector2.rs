#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct int2 {
    pub x: i32,
    pub y: i32,
}

impl int2 {
    pub fn new(x: i32, y: i32) -> int2 {
        int2 { x, y }
    }
}
