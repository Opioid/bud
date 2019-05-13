use base::math::float3;
use base::math::Transformation;
use error::Error;
use json;
use resource;
use scene::prop::Prop;
use scene::shape::{self, Shape};
use scene::Scene;
use serde_json::{Map, Value};

pub struct Loader {
    resource_manager: resource::Manager,

    plane: shape::Plane,
    sphere: shape::Sphere,
}

impl<'a> Loader {
    pub fn new() -> Loader {
        Loader {
            resource_manager: resource::Manager::new(),
            plane: shape::Plane {},
            sphere: shape::Sphere {},
        }
    }

    pub fn resource_manager(&mut self) -> &mut resource::Manager {
        &mut self.resource_manager
    }

    pub fn load<'b, 'c>(
        &'b mut self,
        filename: &str,
        scene: &'c mut Scene<'b>,
    ) -> Result<(), Error> {
        let stream = self.resource_manager.file_system().read_stream(filename)?;

        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new("No suitable root object."));
        }

        let root = root.as_object().unwrap();

        for (name, value) in root.iter() {
            match name.as_ref() {
                "entities" => self.load_entities(value, scene),
                _ => continue,
            }
        }

        Ok(())
    }

    fn load_entities<'b, 'c>(&'b self, entities_value: &Value, scene: &'c mut Scene<'b>) {
        if !entities_value.is_array() {
            return;
        }

        for e in entities_value.as_array().unwrap().iter() {
            let e = match e {
                Value::Object(e) => e,
                _ => continue,
            };

            let type_node = match e.get("type") {
                None => continue,
                Some(type_node) => type_node,
            };

            let type_name = type_node.as_str().unwrap();

            let mut entity = None;

            match type_name.as_ref() {
                "Prop" => {
                    if let Some(prop) = self.load_prop(e, scene) {
                        entity = Some(&mut prop.entity)
                    }
                }
                _ => continue,
            }

            if entity.is_none() {
                continue;
            }

            let mut transformation = Transformation::identity();

            for (name, value) in e.iter() {
                match name.as_ref() {
                    "transformation" => json::read_transformation(value, &mut transformation),
                    _ => continue,
                }
            }

            let mut entity = entity.unwrap();

            entity.set_transformation(&transformation);
        }
    }

    fn load_prop<'b, 'c>(
        &'b self,
        prop_value: &Map<String, Value>,
        scene: &'c mut Scene<'b>,
    ) -> Option<&'c mut Prop> {
        let mut shape = None;

        for (name, value) in prop_value.iter() {
            match name.as_ref() {
                "shape" => shape = self.load_shape(value),
                _ => continue,
            }
        }

        if let None = shape {
            return None;
        }

        Some(scene.create_prop(shape.unwrap()))
    }

    fn load_shape(&self, shape_value: &Value) -> Option<&dyn Shape> {
        let shape_value = match shape_value {
            Value::Object(shape_value) => shape_value,
            _ => return None,
        };

        if let Some(type_name) = shape_value.get("type") {
            match type_name.as_str().unwrap() {
                "Plane" => return Some(&self.plane),
                "Sphere" => return Some(&self.sphere),
                _ => return None,
            }
        }

        None
    }
}
