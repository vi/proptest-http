use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArbitraryHeaderMap(pub http::header::HeaderMap);
#[derive(Debug, Clone, Copy)]
pub struct HeaderMapStrategy;
#[derive(Debug, Clone)]
pub struct HeaderMapValueTree(
    VecValueTree<TupleValueTree<(HeaderNameValueTree, HeaderValueValueTree)>>,
);

impl Arbitrary for ArbitraryHeaderMap {
    type Strategy = HeaderMapStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        HeaderMapStrategy
    }
}
impl Strategy for HeaderMapStrategy {
    type Tree = HeaderMapValueTree;
    type Value = ArbitraryHeaderMap;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(HeaderMapValueTree(
            Vec::<(ArbitraryHeaderName, ArbitraryHeaderValue)>::arbitrary().new_tree(runner)?,
        ))
    }
}
impl ValueTree for HeaderMapValueTree {
    type Value = ArbitraryHeaderMap;

    fn current(&self) -> Self::Value {
        let q = self.0.current();
        let mut hm = http::header::HeaderMap::with_capacity(q.len());
        for (n, v) in q {
            hm.insert(n.0, v.0);
        }
        ArbitraryHeaderMap(hm)
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}

// ----------------------------

static HEADER_NAMES: [http::header::HeaderName; 15] = [
    http::header::HOST,
    http::header::CONTENT_LENGTH,
    http::header::CONTENT_TYPE,
    http::header::AUTHORIZATION,
    http::header::USER_AGENT,
    http::header::ACCEPT,
    http::header::ACCEPT_LANGUAGE,
    http::header::ACCEPT_ENCODING,
    http::header::UPGRADE,
    http::header::CONNECTION,
    http::header::DNT,
    http::header::CACHE_CONTROL,
    http::header::DATE,
    http::header::EXPIRES,
    http::header::SERVER,
];

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArbitraryHeaderName(pub http::header::HeaderName);
#[derive(Debug, Clone, Copy)]
pub struct HeaderNameStrategy;
#[derive(Debug, Clone)]
pub struct HeaderNameValueTree(IndexValueTree);

impl Arbitrary for ArbitraryHeaderName {
    type Strategy = HeaderNameStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        HeaderNameStrategy
    }
}
impl Strategy for HeaderNameStrategy {
    type Tree = HeaderNameValueTree;
    type Value = ArbitraryHeaderName;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(HeaderNameValueTree(Index::arbitrary().new_tree(runner)?))
    }
}
impl ValueTree for HeaderNameValueTree {
    type Value = ArbitraryHeaderName;

    fn current(&self) -> Self::Value {
        ArbitraryHeaderName(self.0.current().get(&HEADER_NAMES).clone())
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}

// -------------------

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArbitraryHeaderValue(pub http::header::HeaderValue);
#[derive(Debug, Clone, Copy)]
pub struct HeaderValueStrategy;
#[derive(Debug, Clone)]
pub struct HeaderValueValueTree(IndexValueTree);

const HEADER_VALUES: [&str; 30] = [
    "",
    "0",
    "1",
    ":",
    "\t",
    "%",
    "\"\"",
    "%D1%89",
    "100000",
    "\"%D1%89\"",
    "\\r\\n",
    "deflate",
    "localhost",
    "close",
    "en-GB",
    "text/html",
    "keep-alive",
    "websocket",
    "example.com",
    "example.com:1234",
    "max-age=604800",
    "en-GB,en;q=0.5",
    "Accept-Encoding",
    "EOS (vny006/0453)",
    "Thu, 20 Jun 2019 21:06:20 GMT",
    "text/html; charset=UTF-8",
    "999999999999999999999999999999999999999999999999999999",
    "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
    "_xsrf=2|8bea5404|5ef47a59a0516e67bbd5f86849e28a1c|1553532280",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:52.1) Gecko/20100101 Firefox/52.1",
];

impl Arbitrary for ArbitraryHeaderValue {
    type Strategy = HeaderValueStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        HeaderValueStrategy
    }
}
impl Strategy for HeaderValueStrategy {
    type Tree = HeaderValueValueTree;
    type Value = ArbitraryHeaderValue;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(HeaderValueValueTree(Index::arbitrary().new_tree(runner)?))
    }
}
impl ValueTree for HeaderValueValueTree {
    type Value = ArbitraryHeaderValue;

    fn current(&self) -> Self::Value {
        ArbitraryHeaderValue(http::header::HeaderValue::from_static(
            &self.0.current().get(&HEADER_VALUES),
        ))
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}
