extern crate proptest;
extern crate proptest_http;

use proptest::arbitrary::Arbitrary;
use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();

    loop {
        let mut u = proptest_http::ArbitraryRequest::arbitrary()
            .new_tree(&mut r)
            .unwrap();
        for _ in 0..10 {
            for _ in 0..10 {
                u.simplify();
            }
            u.current();
        }

        let mut u = proptest_http::ArbitraryResponse::arbitrary()
            .new_tree(&mut r)
            .unwrap();
        for _ in 0..10 {
            for _ in 0..10 {
                u.simplify();
            }
            u.current();
        }
    }
}
