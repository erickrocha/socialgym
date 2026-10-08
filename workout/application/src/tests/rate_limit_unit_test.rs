use super::{auth_limiter, legal_limiter, refresh_limiter};
use std::net::IpAddr;

fn allowed(limiter: &business::commons::rate_limit::RateLimiter, ip: IpAddr) -> u32 {
    (0..200).take_while(|_| limiter.check(ip)).count() as u32
}

#[test]
fn credentials_and_refresh_allow_twenty_a_minute_and_legal_documents_sixty() {
    assert_eq!(allowed(&auth_limiter(), IpAddr::from([198, 51, 100, 1])), 20);
    assert_eq!(allowed(&refresh_limiter(), IpAddr::from([198, 51, 100, 2])), 20);
    assert_eq!(allowed(&legal_limiter(), IpAddr::from([198, 51, 100, 3])), 60);
}

#[test]
fn the_limiters_count_apart() {
    let ip = IpAddr::from([198, 51, 100, 4]);
    assert_eq!(allowed(&refresh_limiter(), ip), 20);
    assert!(auth_limiter().check(ip), "using up refresh does not use up sign-in");
}
