use super::RateLimiter;
use std::net::IpAddr;
use std::time::Duration;

fn ip(last: u8) -> IpAddr {
    IpAddr::from([192, 0, 2, last])
}

#[test]
fn an_address_is_refused_after_its_allowance_and_others_are_not() {
    let limiter = RateLimiter::new(2, Duration::from_secs(60));
    assert!(limiter.check(ip(1)));
    assert!(limiter.check(ip(1)));
    assert!(!limiter.check(ip(1)), "the third request in the window is refused");
    assert!(limiter.check(ip(2)), "another address has its own allowance");
}

#[test]
fn the_allowance_returns_when_the_window_ends() {
    let limiter = RateLimiter::new(1, Duration::from_millis(30));
    assert!(limiter.check(ip(1)));
    assert!(!limiter.check(ip(1)));
    std::thread::sleep(Duration::from_millis(40));
    assert!(limiter.check(ip(1)), "a new window starts");
}

#[test]
fn clones_share_one_counter() {
    let limiter = RateLimiter::new(1, Duration::from_secs(60));
    let clone = limiter.clone();
    assert!(limiter.check(ip(1)));
    assert!(!clone.check(ip(1)), "REST and gRPC handles of one limiter count together");
}
