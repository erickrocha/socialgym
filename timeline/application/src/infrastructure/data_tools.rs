use chrono::NaiveDateTime;
use mongodb::bson::DateTime;

pub fn opt_bson_datetime_to_naive(dt: DateTime) -> Option<NaiveDateTime> {
    let millis = dt.timestamp_millis();
    let secs = millis / 1000;
    let nsecs = ((millis % 1000) * 1_000_000) as u32;
    let datetime = chrono::DateTime::from_timestamp(secs, nsecs);
    Some(datetime.unwrap().naive_utc())
}

pub fn opt_naive_to_bson_datetime(dt: NaiveDateTime) -> Option<DateTime> {
    let millis = dt.and_utc().timestamp_millis();
    Some(DateTime::from_millis(millis))
}

pub fn naive_to_bson_datetime(dt: NaiveDateTime) -> DateTime {
    let millis = dt.and_utc().timestamp_millis();
    DateTime::from_millis(millis)
}

pub fn bson_datetime_to_naive(dt: DateTime) -> NaiveDateTime {
    let millis = dt.timestamp_millis();
    let secs = millis / 1000;
    let nano_secs = ((millis % 1000) * 1_000_000) as u32;
    let datetime = chrono::DateTime::from_timestamp(secs, nano_secs);
    datetime.unwrap().naive_utc()
}