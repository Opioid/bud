use resource::Identifiable;

pub trait Image : Identifiable {
    type Typed;

    fn typed(&self) -> &Self::Typed;
}
