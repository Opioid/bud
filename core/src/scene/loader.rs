use base::math::float3;
use base::math::Transformation;
use error::Error;
use json;
use resource;
use scene::prop::Prop;
use scene::Scene;
use serde_json::Value;

pub struct Loader {
    resource_manager: resource::Manager,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            resource_manager: resource::Manager::new(),
        }
    }

    pub fn resource_manager(&mut self) -> &mut resource::Manager {
        &mut self.resource_manager
    }

    pub fn load(&mut self, filename: &str) -> Result<Scene, Error> {
        let stream = self.resource_manager.file_system().read_stream(filename)?;

        let root: Value = serde_json::from_reader(stream)?;
        if !root.is_object() {
            return Err(Error::new("No suitable root object."));
        }

        let root = root.as_object().unwrap();

        let mut scene = Scene::new();

        for (name, value) in root.iter() {
            match name.as_ref() {
                "entities" => self.load_entities(value, &mut scene),
                _ => continue,
            }
        }

        Ok(scene)
    }

    fn load_entities(&mut self, entities_value: &serde_json::Value, scene: &mut Scene) {
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

            println!("{}", type_name);

            let mut entity = None;

            match type_name.as_ref() {
                "Prop" => entity = Some(&mut self.load_prop(e, scene).entity),
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

            println!("{:?}", transformation.scale);
        }
    }

    fn load_prop<'a>(
        &mut self,
        prop_value: &serde_json::Map<String, serde_json::Value>,
        scene: &'a mut Scene,
    ) -> &'a mut Prop {
        scene.create_prop()
    }
}
