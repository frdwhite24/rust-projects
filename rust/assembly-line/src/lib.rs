pub fn production_rate_per_hour(speed: u8) -> f64 {
    const COUNT_PER_SPEED_UNIT: u8 = 221;

    let success_rate = if speed >= 1 && speed < 5 {
        1.0
    } else if speed >= 5 && speed < 9 {
        0.9
    } else if speed >= 9 && speed < 11 {
        0.77
    } else {
        0 as f64
    };

    speed as f64 * COUNT_PER_SPEED_UNIT as f64 * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60 as f64) as u32
}
