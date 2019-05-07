use super::{Camera, CameraBase};
use base::math::{self, float2, float3, int2};
use json;
use rendering::sensor::Sensor;
use sampler::CameraSample;
use scene::{self, Ray};
use serde_json::Value;

pub struct Lens {
    radius: f32,
}

pub struct Focus {
    point: float3,
    distance: f32,
    use_point: bool,
}

pub struct Perspective {
    pub base: CameraBase,

    left_top: float3,
    d_x: float3,
    d_y: float3,

    fov: f32,

    lens_radius: f32,

    focus_distance: f32,
}

impl Perspective {
    pub fn new(resolution: int2, sensor: Box<dyn Sensor>) -> Perspective {
        let mut p = Perspective {
            base: CameraBase::new(resolution, sensor),
            left_top: float3::identity(),
            d_x: float3::identity(),
            d_y: float3::identity(),
            fov: math::degrees_to_radians(60.0),
            lens_radius: 0.0,
            focus_distance: 0.0,
        };

        let dimensions = p.sensor_dimensions();
        p.base.sensor.resize(dimensions);

        p
    }

    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }

    pub fn set_lens(&mut self, lens: Lens) {
        self.lens_radius = lens.radius;
    }

    pub fn set_focus(&mut self, focus: Focus) {
        self.focus_distance = focus.distance;
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
    }

    fn generate_ray(&self, sample: &CameraSample) -> Option<Ray> {
        let coords = float2::from(sample.pixel) + sample.pixel_uv;

        let mut direction = self.left_top + coords.v[0] * self.d_x + coords.v[1] * self.d_y;

        let origin;

        if self.lens_radius > 0.0 {
            let lens = math::sample_disk_concentric(sample.lens_uv);

            origin = float3::from_2(self.lens_radius * lens, 0.0);

            let t = self.focus_distance / direction.v[2];

            let focus = t * direction;

            direction = focus - origin;
        } else {
            origin = float3::identity();
        }

        let transformation = self.base.entity.transformation_at(0);

        let origin_w = transformation.object_to_world.transform_point(origin);

        let direction = direction.normalized();
        let direction_w = transformation.object_to_world.transform_vector(direction);

        Some(Ray::new(origin_w, direction_w, 0.0, scene::RAY_MAX_T, 0))
    }

    fn sensor_mut(&mut self) -> &mut dyn Sensor {
        &mut (*self.base.sensor)
    }

    fn sensor_dimensions(&self) -> int2 {
        self.base.resolution
    }

    fn set_parameter(&mut self, name: &str, value: &Value) {
        match name {
            "fov" => self.set_fov(math::degrees_to_radians(json::read_float(value))),
            "lens" => self.set_lens(load_lens(value)),
            "focus" => self.set_focus(load_focus(value)),
            _ => (),
        }
    }
}

fn load_lens(lens_value: &Value) -> Lens {
    let mut l = Lens { radius: 0.0 };

    let lens_value = match lens_value {
        Value::Object(lens_value) => lens_value,
        _ => return l,
    };

    for (name, value) in lens_value.iter() {
        match name.as_ref() {
            "radius" => l.radius = json::read_float(value),
            _ => (),
        }
    }

    l
}

fn load_focus(focus_value: &Value) -> Focus {
    let mut f = Focus {
        point: float3::new(0.5, 0.5, 0.0),
        distance: 0.0,
        use_point: false,
    };

    let focus_value = match focus_value {
        Value::Object(focus_value) => focus_value,
        _ => return f,
    };

    for (name, value) in focus_value.iter() {
        match name.as_ref() {
            "point" => {
                f.point = json::read_float3(value);
                f.use_point = true;
            }
            "distance" => f.distance = json::read_float(value),
            _ => (),
        }
    }

    f
}
