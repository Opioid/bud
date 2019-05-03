use std::time::Duration;

pub fn duration_to_seconds(duration: Duration) -> f32 {
    let millis = duration.as_millis();
    millis as f32 / 1000.0
}
