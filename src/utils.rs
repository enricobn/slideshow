use std::time::Duration;
use separator::Separatable;

pub fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs == 0 {
        format!("{} millis", duration.subsec_millis().separated_string())
    } else if secs == 1 {
        format!("1 second {} millis", duration.subsec_millis().separated_string())
    } else {
        format!("{} seconds {} millis", secs.separated_string(), duration.subsec_millis().separated_string())
    }
}