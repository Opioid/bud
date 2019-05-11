use super::Image;
use resource::Identifiable;
use base::math::int2;

pub struct TypedImage<T> {
    pub dimensions: int2,
    data: Vec<T>,
}

impl<T> TypedImage<T>
where
    T: Copy + Clone + Default,
{
    pub fn new(dimensions: int2) -> TypedImage<T> {
        TypedImage {
            dimensions,
            data: vec![T::default(); (dimensions.v[0] * dimensions.v[1]) as usize],
        }
    }

    pub fn get_by_index(&self, i: i32) -> T {
        self.data[i as usize]
    }

    pub fn set_by_index(&mut self, i: i32, v: T) {
        unsafe {
            *self.data.get_unchecked_mut(i as usize) = v;
        }
    }

    pub fn set(&mut self, x: i32, y: i32, v: T) {
        let i = y * self.dimensions.v[0] + x;
        self.data[i as usize] = v;
    }
}

impl<T> Image for TypedImage<T> {
    type Typed = TypedImage<T>;
    
    fn typed(&self) -> &TypedImage<T> {
        &self
    }
}

impl<T> Identifiable for TypedImage<T> {
    fn name() -> &'static str {
        "Image"
    }
}
