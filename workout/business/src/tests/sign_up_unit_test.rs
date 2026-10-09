use super::{acceptance_ip, is_at_least_eighteen};
use chrono::NaiveDate;

#[test]
fn rejects_the_day_before_the_eighteenth_birthday() {
    assert!(!is_at_least_eighteen(
        NaiveDate::from_ymd_opt(2008, 8, 28).unwrap(),
        NaiveDate::from_ymd_opt(2026, 8, 27).unwrap(),
    ));
}

#[test]
fn accepts_on_the_eighteenth_birthday() {
    assert!(is_at_least_eighteen(
        NaiveDate::from_ymd_opt(2008, 8, 27).unwrap(),
        NaiveDate::from_ymd_opt(2026, 8, 27).unwrap(),
    ));
}

#[test]
fn handles_leap_day_without_year_subtraction_errors() {
    assert!(is_at_least_eighteen(
        NaiveDate::from_ymd_opt(2008, 2, 29).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 28).unwrap(),
    ));
}

#[test]
fn the_consent_record_takes_the_gateway_address_first() {
    assert_eq!(
        acceptance_ip(Some("203.0.113.9"), Some("198.51.100.1")),
        "203.0.113.9"
    );
    assert_eq!(
        acceptance_ip(None, Some("198.51.100.1, 10.0.0.1")),
        "198.51.100.1",
        "the first forwarded entry"
    );
    assert_eq!(
        acceptance_ip(Some(" 203.0.113.9 "), None),
        "203.0.113.9",
        "trimmed"
    );
}

#[test]
fn a_missing_empty_or_oversized_address_is_unknown() {
    assert_eq!(acceptance_ip(None, None), "unknown");
    assert_eq!(acceptance_ip(Some("  "), None), "unknown");
    assert_eq!(acceptance_ip(Some(&"1".repeat(46)), None), "unknown");
    assert_eq!(
        acceptance_ip(Some(&"1".repeat(45)), None).len(),
        45,
        "45 characters is the longest IPv6 text"
    );
}
