use super::*;

#[derive(Debug)]
pub struct ArbitraryRequest(pub http::request::Request<()>);
#[derive(Debug,Clone,Copy)]
pub struct RequestStrategy;
#[derive(Debug,Clone)]
pub struct RequestValueTree ( TupleValueTree <(
    super::uri::UriValueTree,
    MethodValueTree,
    super::header::HeaderMapValueTree,
)>);



impl Arbitrary for ArbitraryRequest {
    type Strategy = RequestStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        RequestStrategy
    }
}
impl Strategy for RequestStrategy {
    type Tree = RequestValueTree;
    type Value = ArbitraryRequest;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(RequestValueTree(
            <(
                ArbitraryUri,
                ArbitraryMethod,
                ArbitraryHeaderMap,
            )>::arbitrary().new_tree(runner)?
        ))
    }
}
impl ValueTree for RequestValueTree {
    type Value = ArbitraryRequest;

    fn current(&self) -> Self::Value {
        let mut b = http::request::Builder::default();
        let (
            ArbitraryUri(u),
            ArbitraryMethod(m),
            ArbitraryHeaderMap(h),
        ) = self.0.current();
        b.uri(u);
        b.method(m);
        *b.headers_mut().unwrap() = h;
        ArbitraryRequest(b.body(()).unwrap())
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}

//-----------------------------

#[derive(Debug,Eq,PartialEq)]
pub struct ArbitraryMethod(pub http::method::Method);
#[derive(Debug,Clone,Copy)]
pub struct MethodStrategy;
#[derive(Debug,Clone)]
pub struct MethodValueTree(IndexValueTree);


const METHODS : [http::Method; 5] = [
    http::method::Method::GET,
    http::method::Method::POST,
    http::method::Method::PUT,
    http::method::Method::DELETE,
    http::method::Method::TRACE,
];

impl Arbitrary for ArbitraryMethod {
    type Strategy = MethodStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        MethodStrategy
    }
}
impl Strategy for MethodStrategy {
    type Tree = MethodValueTree;
    type Value = ArbitraryMethod;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
       Ok(MethodValueTree(
           Index::arbitrary().new_tree(runner)?
       ))
    }
}
impl ValueTree for MethodValueTree {
    type Value = ArbitraryMethod;

    fn current(&self) -> Self::Value {
       ArbitraryMethod(self.0.current().get(&METHODS).clone())
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}
