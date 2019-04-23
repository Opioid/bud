use super::prop::Prop;

pub struct Scene {
    props: Vec<Box<Prop>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene { props: Vec::new() }
    }

    pub fn create_prop(&mut self) -> &mut Prop {
        self.props.push(Box::new(Prop::new()));

        &mut *self.props.last_mut().unwrap()
    }
}
