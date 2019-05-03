use super::{Camera, CameraBase};
use base::math::{self, float2, float3, int2};
use json;
use rendering::sensor::Sensor;
use sampler::CameraSample;
use scene::Ray;

pub struct Perspective {
    pub base: CameraBase,
    fov: f32,

    left_top: float3,
    d_x: float3,
    d_y: float3,
}

impl Perspective {
    pub fn new(resolution: int2, sensor: Box<dyn Sensor>) -> Perspective {
        let mut p = Perspective {
            base: CameraBase::new(resolution, sensor),
            fov: math::degrees_to_radians(60.0),
            left_top: float3::identity(),
            d_x: float3::identity(),
            d_y: float3::identity(),
        };

        let dimensions = p.sensor_dimensions();
        p.base.sensor.resize(dimensions);

        p
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
}

impl Camera for Perspective {
    fn on_update(&mut self) {
        let fr = float2::from(self.base.resolution);

        let ratio = fr.v[0] / fr.v[1];

        let z = ratio * math::PI / self.fov * 0.5;

        //	float3 left_top   (-ratio,  1.f, z);
        //	float3 right_top  ( ratio,  1.f, z);
        //	float3 left_bottom(-ratio, -1.f, z);

        let left_top = float3::new(-ratio, 1.0, z);
        let right_top = float3::new(ratio, 1.0, z);
        let left_bottom = float3::new(-ratio, -1.0, z);

        self.left_top = left_top;
        self.d_x = (right_top - left_top) / fr.v[0];
        self.d_y = (left_bottom - left_top) / fr.v[1];

        println!("fov {}", self.fov);
    }

    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray> {
        let coords = float2::from(sample.pixel) + sample.pixel_uv;

        let direction = self.left_top + coords.v[0] * self.d_x + coords.v[1] * self.d_y;

        let origin = float3::identity();

        let transformation = self.base.entity.transformation_at(0);

        let origin_w = transformation.object_to_world.transform_point(&origin);

        let direction = direction.normalized();
        let direction_w = transformation.object_to_world.transform_vector(&direction);

        Some(Ray::new(origin_w, direction_w, 0.0, 1000.0, 0))
    }

    fn sensor_mut(&mut self) -> &mut dyn Sensor {
        &mut (*self.base.sensor)
    }

    fn sensor_dimensions(&self) -> int2 {
        self.base.resolution
    }

    fn set_parameter(&mut self, name: &str, value: &serde_json::Value) {
        match name {
            "fov" => self.set_fov(math::degrees_to_radians(json::read_float(value))),
            _ => (),
        }
    }
}
