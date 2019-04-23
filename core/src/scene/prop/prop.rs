use scene::entity::Entity;

pub struct Prop {
    pub entity: Entity,
}

impl Prop {
    pub fn new() -> Prop {
        Prop {
            entity: Entity::new(),
        }
    }
}
