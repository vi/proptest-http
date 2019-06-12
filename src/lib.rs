pub extern crate http;
pub extern crate proptest;

use proptest::arbitrary::Arbitrary;
use proptest::strategy::{Strategy,NewTree,ValueTree};
use proptest::test_runner::TestRunner;

#[derive(Debug,PartialEq,Eq,Clone)]
pub struct ArbitraryUri(pub http::Uri);

#[derive(Debug)]
pub struct UriStrategy;

impl Arbitrary for ArbitraryUri {
    type Parameters = ();
    type Strategy = UriStrategy;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        UriStrategy
    }
}

impl Strategy for UriStrategy {
    type Value = ArbitraryUri;
    type Tree = ArbitraryUri;

    fn new_tree(&self, _runner: &mut TestRunner) -> NewTree<Self> {
        let uri : http::uri::Uri = "/test".parse().unwrap();
        Ok(ArbitraryUri(uri))
    }
}

impl ValueTree for ArbitraryUri {
    type Value = ArbitraryUri;

    fn current(&self) -> Self::Value {
        self.clone()
    }

    fn simplify(&mut self) -> bool {
        false
    }

    fn complicate(&mut self) -> bool {
        false
    }
}
