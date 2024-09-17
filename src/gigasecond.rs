use time::{Duration, OffsetDateTime, PrimitiveDateTime as DateTime};

fn main() {
    let now_utc = OffsetDateTime::now_utc();

    let now: DateTime = now_utc.date().with_time(now_utc.time());

    // Call the function to get the date and time after 10 seconds
    let end = after(now);
    println!("{}", end);
}

pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1000000000)
}
