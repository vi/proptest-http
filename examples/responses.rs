extern crate proptest;
extern crate proptest_http;

use proptest::strategy::{Strategy, ValueTree};
use proptest::arbitrary::Arbitrary;

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();
    
    for _ in 0..10 {
        let mut u = proptest_http::ArbitraryResponse::arbitrary().new_tree(&mut r).unwrap();
        for _ in 0..3 {
            println!("{:#?}", u.current().0);
            for _ in 0..10 { u.simplify(); }
        }
        println!()
    }
}