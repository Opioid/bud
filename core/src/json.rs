use base::math::Transformation;
use base::math::{self, float3, float3x3, int2, Quaternion};
use serde_json::Value;

pub fn read_float(value: &Value) -> f32 {
    value.as_f64().unwrap() as f32
}

pub fn read_int2(value: &Value) -> int2 {
    match value {
        Value::Array(a) => int2::new(a[0].as_i64().unwrap() as i32, a[1].as_i64().unwrap() as i32),
        _ => int2::identity(),
    }
}

pub fn read_int2_from(value: &Value, name: &str, default: int2) -> int2 {
    if let Some(value) = value.get(name) {
        return read_int2(value);
    };

    default
}

pub fn read_float3(value: &Value) -> float3 {
    match value {
        Value::Array(a) => float3::new(
            a[0].as_f64().unwrap() as f32,
            a[1].as_f64().unwrap() as f32,
            a[2].as_f64().unwrap() as f32,
        ),
        _ => float3::identity(),
    }
}

pub fn read_string_from<'a>(value: &'a Value, name: &str, default: &'a str) -> &'a str {
    if let Some(value) = value.get(name) {
        return value.as_str().unwrap();
    }

    default
}

fn create_rotation_matrix(xyz: &float3) -> float3x3 {
    let rot_x = float3x3::rotation_x(math::degrees_to_radians(xyz.v[0]));
    let rot_y = float3x3::rotation_y(math::degrees_to_radians(xyz.v[1]));
    let rot_z = float3x3::rotation_z(math::degrees_to_radians(xyz.v[2]));

    rot_z * rot_x * rot_y
}

fn read_rotation_matrix(value: &Value) -> float3x3 {
    let rot = read_float3(value);

    create_rotation_matrix(&rot)
}

fn read_local_rotation(value: &Value) -> Quaternion {
    Quaternion::from_matrix(&read_rotation_matrix(value))
}

pub fn read_transformation(value: &Value, transformation: &mut Transformation) {
    match value {
        Value::Array(array) => {}
        Value::Object(map) => {
            for (name, value) in map.iter() {
                match name.as_ref() {
                    "position" => transformation.position = read_float3(value),
                    "scale" => transformation.scale = read_float3(value),
                    "rotation" => transformation.rotation = read_local_rotation(value),
                    _ => (),
                }
            }
        }
        _ => (),
    }
}
