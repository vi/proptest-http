extern crate proptest;
extern crate proptest_http;

use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();

    for _ in 0..10 {
        let u = proptest_http::UriStrategy.new_tree(&mut r).unwrap();
        println!("{}", u.current().0);
    }
}
