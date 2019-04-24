use base::math::float3;
use base::math::Transformation;

pub fn read_float3(value: &serde_json::Value) -> float3 {
    let a = value.as_array().unwrap();

    float3::new(
        a[0].as_f64().unwrap() as f32,
        a[1].as_f64().unwrap() as f32,
        a[2].as_f64().unwrap() as f32,
    )
}

pub fn read_transformation(value: &serde_json::Value, transformation: &mut Transformation) {
    if value.is_array() {
    } else if value.is_object() {
        for (name, value) in value.as_object().unwrap().iter() {
            match name.as_ref() {
                "position" => transformation.position = read_float3(value),
                "scale" => transformation.scale = read_float3(value),
                _ => continue,
            }
        }
    }
}
