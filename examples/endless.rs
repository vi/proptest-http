extern crate proptest;
extern crate proptest_http;

use proptest::strategy::{Strategy, ValueTree};
use proptest::arbitrary::Arbitrary;

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();
    
    loop {
        let mut u = proptest_http::ArbitraryRequest::arbitrary().new_tree(&mut r).unwrap();
        for _ in 0..10 {
            for _ in 0..10 { u.simplify(); }
            u.current();
        }
    }
}
