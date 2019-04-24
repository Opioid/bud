use base::math::float3;
use base::math::Transformation;
use error::Error;
use json;
use resource;
use scene::prop::Prop;
use scene::shape::{self, Shape};
use scene::Scene;
use serde_json::Value;

pub struct Loader {
    resource_manager: resource::Manager,

    plane: shape::Plane,
    sphere: shape::Sphere,
}

impl Loader {
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

    pub fn load<'a, 'b>(
        &'a mut self,
        filename: &str,
        scene: &'b mut Scene<'a>,
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

    fn load_entities<'a, 'b>(
        &'a self,
        entities_value: &serde_json::Value,
        scene: &'b mut Scene<'a>,
    ) {
        if !entities_value.is_array() {
            return;
        }

        for e in entities_value.as_array().unwrap().iter() {
            if !e.is_object() {
                continue;
            }

            let e = e.as_object().unwrap();

            let type_node = &e.get("type");
            if type_node.is_none() {
                continue;
            }

            let type_name = type_node.unwrap().as_str().unwrap();

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

            let mut transformation = Transformation {
                position: float3::from_scalar(0.0),
                scale: float3::from_scalar(1.0),
            };

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

    fn load_prop<'a, 'b>(
        &'a self,
        prop_value: &serde_json::Map<String, serde_json::Value>,
        scene: &'b mut Scene<'a>,
    ) -> Option<&'b mut Prop> {
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

    fn load_shape(&self, shape_value: &serde_json::Value) -> Option<&dyn Shape> {
        if !shape_value.is_object() {
            return None;
        }

        let shape_value = shape_value.as_object().unwrap();

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
