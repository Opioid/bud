use resource::Identifiable;

pub struct MaterialBase {
    pub stuff: u32,
}

pub trait Material {
    fn stuff(&self) -> u32 {
        4
    }
}
