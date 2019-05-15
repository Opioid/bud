use base::math::float3;
use base::math::Transformation;
use error::Error;
use json;
use resource;
use scene::material::Material;
use scene::prop::Prop;
use scene::shape::{self, Shape};
use scene::Scene;
use serde_json::{Map, Value};
use std::rc::Rc;

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
        let entities_value = match entities_value {
            Value::Array(entities_value) => entities_value,
            _ => return,
        };

        for e in entities_value.iter() {
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
                "Light" => {
                    if let Some(prop) = self.load_prop(e, scene) {
                        entity = Some(&mut prop.entity)
                    }
                }
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

        let mut materials_value = None;

        for (name, value) in prop_value.iter() {
            match name.as_ref() {
                "shape" => shape = self.load_shape(value),
                "materials" => materials_value = Some(value),
                _ => continue,
            }
        }

        if let None = shape {
            return None;
        }

        let mut materials = Vec::with_capacity(1);

        if let Some(materials_value) = materials_value {
            self.load_materials(materials_value, &mut materials);
        }

        Some(scene.create_prop(shape.unwrap(), materials))
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

    fn load_materials(&self, materials_value: &Value, materials: &mut Vec<Rc<dyn Material>>) {
        let materials_value = match materials_value {
            Value::Array(materials_value) => materials_value,
            _ => return,
        };

        for m in materials_value.iter() {
            let name = m.as_str().unwrap();
            match self.resource_manager.load_material(name) {
                Ok(material) => materials.push(material),
                Err(error) => {
                    println!("Using fallback for material \"{}\": {}", name, error.message())
                }
            }

            if materials.len() == materials.capacity() {
                return;
            }
        }
    }
}
