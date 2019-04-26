use super::{Camera, CameraBase};
use base::math::{self, int2, float2, float3};
use sampler::CameraSample;
use scene::Ray;
use rendering::sensor::Sensor;
use json;

pub struct Perspective {
    base: CameraBase,
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
    fn update(&mut self) {
        self.base.update();

        let fr = float2::from(self.base.resolution);

        let  ratio = fr.x / fr.y;  

        let z = ratio * math::PI / self.fov * 0.5;

        //	float3 left_top   (-ratio,  1.f, z); 
        //	float3 right_top  ( ratio,  1.f, z); 
        //	float3 left_bottom(-ratio, -1.f, z);

        let left_top = float3::new(-ratio, 1.0, z);
        let right_top = float3::new(ratio, 1.0, z);
        let left_bottom = float3::new(ratio, -1.0, z);

        self.left_top = left_top; 
        self.d_x      = (right_top - left_top) / fr.x; 
        self.d_y      = (left_bottom - left_top) / fr.y;
        
        println!("{}", self.fov);
    }
    
    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray> {
        Some(Ray::new())
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
            _ => ()
        }
    }
}
