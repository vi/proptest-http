#![allow(unused)]
#![warn(unused_must_use)]
pub extern crate http;
pub extern crate proptest;

use proptest::arbitrary::Arbitrary;
use proptest::strategy::{Strategy,NewTree,ValueTree};
use proptest::test_runner::TestRunner;
use proptest::prelude::RngCore;
use proptest::sample::{Index,IndexValueTree};
use proptest::bool::{weighted,BoolValueTree};

pub mod uri;
pub use uri::ArbitraryUri;
