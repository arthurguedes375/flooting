use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let time = SystemTime::now();
    let now_nano = time
                    .duration_since(UNIX_EPOCH)
                    .expect("Failed to get timestamp.")
                    .as_nanos();
    return now_nano;
}

pub const fn to_nano(milliseconds: u16) -> u128 {
    return milliseconds as u128 * 1_000_000;
}