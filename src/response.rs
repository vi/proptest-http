use super::*;

#[derive(Debug)]
pub struct ArbitraryResponse(pub http::response::Response<()>);
#[derive(Debug,Clone,Copy)]
pub struct ResponseStrategy;
#[derive(Debug,Clone)]
pub struct ResponseValueTree ( TupleValueTree <(
    StatusCodeValueTree,
    super::header::HeaderMapValueTree,
)>);



impl Arbitrary for ArbitraryResponse {
    type Strategy = ResponseStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        ResponseStrategy
    }
}
impl Strategy for ResponseStrategy {
    type Tree = ResponseValueTree;
    type Value = ArbitraryResponse;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(ResponseValueTree(
            <(
                ArbitraryStatusCode,
                ArbitraryHeaderMap,
            )>::arbitrary().new_tree(runner)?
        ))
    }
}
impl ValueTree for ResponseValueTree {
    type Value = ArbitraryResponse;

    fn current(&self) -> Self::Value {
        let mut b = http::response::Builder::default();
        let (
            ArbitraryStatusCode(s),
            ArbitraryHeaderMap(h),
        ) = self.0.current();
        b.status(s);
        *b.headers_mut().unwrap() = h;
        ArbitraryResponse(b.body(()).unwrap())
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
pub struct ArbitraryStatusCode(pub http::status::StatusCode);
#[derive(Debug,Clone,Copy)]
pub struct StatusCodeStrategy;
#[derive(Debug,Clone)]
pub struct StatusCodeValueTree(IndexValueTree);


const STATUS_CODES : [http::status::StatusCode; 14] = [
    http::StatusCode::OK,
    http::StatusCode::NOT_FOUND,
    http::StatusCode::FORBIDDEN,
    http::StatusCode::BAD_REQUEST,
    http::StatusCode::NO_CONTENT,
    http::StatusCode::FOUND,
    http::StatusCode::GONE,
    http::StatusCode::CONTINUE,
    http::StatusCode::UPGRADE_REQUIRED,
    http::StatusCode::INTERNAL_SERVER_ERROR,
    http::StatusCode::NOT_IMPLEMENTED,
    http::StatusCode::GATEWAY_TIMEOUT,
    http::StatusCode::LOOP_DETECTED,
    http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED ,
];

impl Arbitrary for ArbitraryStatusCode {
    type Strategy = StatusCodeStrategy;
    type Parameters = ();

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        StatusCodeStrategy
    }
}
impl Strategy for StatusCodeStrategy {
    type Tree = StatusCodeValueTree;
    type Value = ArbitraryStatusCode;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
       Ok(StatusCodeValueTree(
           Index::arbitrary().new_tree(runner)?
       ))
    }
}
impl ValueTree for StatusCodeValueTree {
    type Value = ArbitraryStatusCode;

    fn current(&self) -> Self::Value {
       ArbitraryStatusCode(self.0.current().get(&STATUS_CODES).clone())
    }

    fn simplify(&mut self) -> bool {
        self.0.simplify()
    }

    fn complicate(&mut self) -> bool {
        self.0.complicate()
    }
}
