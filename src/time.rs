use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let time = SystemTime::now();
    let now_nano = time
                    .duration_since(UNIX_EPOCH)
                    .expect("Failed to get timestamp.")
                    .as_nanos();
    return now_nano;
}