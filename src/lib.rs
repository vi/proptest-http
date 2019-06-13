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
use proptest::tuple::TupleValueTree;
use proptest::collection::VecValueTree;

pub mod uri;
pub use uri::ArbitraryUri;

pub mod request;
pub use request::{ArbitraryRequest,ArbitraryMethod};

pub mod header;
pub use header::{ArbitraryHeaderName,ArbitraryHeaderValue,ArbitraryHeaderMap};

pub mod response;
pub use response::{ArbitraryResponse,ArbitraryStatusCode};
