extern crate proptest;
extern crate proptest_http;

use proptest::arbitrary::Arbitrary;
use proptest::strategy::{Strategy, ValueTree};

fn main() {
    let mut r = proptest::test_runner::TestRunner::default();

    for _ in 0..10 {
        let mut u = proptest_http::ArbitraryUri::arbitrary()
            .new_tree(&mut r)
            .unwrap();
        for _ in 0..10 {
            println!("{} ", u.current().0);
            u.simplify();
            //if u.current().0.into_parts().scheme.is_none() {
            //    u.complicate();
            //}
        }
        println!()
    }
}
