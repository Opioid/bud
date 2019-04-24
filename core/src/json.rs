use base::math::Transformation;
use base::math::{float3, int2};
use serde_json::Value;

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

pub fn read_transformation(value: &Value, transformation: &mut Transformation) {
    match value {
        Value::Array(array) => {}
        Value::Object(map) => {
            for (name, value) in map.iter() {
                match name.as_ref() {
                    "position" => transformation.position = read_float3(value),
                    "scale" => transformation.scale = read_float3(value),
                    _ => continue,
                }
            }
        }
        _ => {}
    }
}
