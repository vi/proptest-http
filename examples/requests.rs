extern crate proptest;
extern crate proptest_http;

use proptest::arbitrary::Arbitrary;
use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();

    for _ in 0..10 {
        let mut u = proptest_http::ArbitraryRequest::arbitrary()
            .new_tree(&mut r)
            .unwrap();
        for _ in 0..3 {
            println!("{:#?}", u.current().0);
            for _ in 0..10 {
                u.simplify();
            }
        }
        println!()
    }
}
