use time::{Duration, PlainDateTime as DateTime};

fn giga() -> u32 {
    let thousand: u32 = 1000;
    let mil = thousand * thousand;
    let giga = thousand * mil;
    giga
}
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga = giga();

    let final_date_time = start + Duration::new(giga as i64, 0);

    final_date_time
}
