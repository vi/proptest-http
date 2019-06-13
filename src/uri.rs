use super::*;

/// Wrapper that generates random-ish `Uri` instances.
/// Not that URI generation is rather basic. There probably
/// a lot of tricky cases that are not considered
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct ArbitraryUri(pub http::Uri);

#[derive(Debug)]
pub struct UriStrategy;

const SCHEMAS : [&str;2] = [
    "http://",
    "https://",
];

const AUTHS : [&str;16] = [
    "",
    "a@",
    "a:a@",
    "a:@",
    ":a@",
    ":@",
    "%C3%BC:a@",
    "%20:%20@",
    "0:4@",
    "1:%20@",
    "2:%C5%81@",
    "3:%F3%A0%80%A0@",
    "4:%20a%20@",
    "5:%20%09%F3%A0%80%A0%D1%89%F3%A0%80%A0%09%20@",
    "6:%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20@",
    "7:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@",
];

const HOSTS : [&str;10] = [
    "localhost",
    "127.0.0.1",
    "localhost:8080",
    "127.0.0.1:8080",
    "example.com",
    "example.com:8080",
    "[::1]",
    "[::1]:8080",
    "[0000:0000:0000:0000:0000:0000:0000:0001]:8080",
    "longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong.example:8080",
];

const PATHS : [&str;9] = [
    "/",
    "/foo",
    "/foo/",
    "/foo/bar/",
    "/%20",
    "/%D0%B9%D1%86%D1%83%D0%BA%D0%B5%D0%BD%D0%B3%D1%88%D1%89",
    "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z",
    "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/",
    "/%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20?%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20#%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20",
];

const QUERIES : [&str; 9] = [
    "",
    "?",
    "?q",
    "?q=w",
    "?q=%20bar%20",
    "?q=w&",
    "?q=%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20?%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20#%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20&",
    "?q=http%3A%2F%2F%5B%3A%3A1%5D%3A123%2F%3Fqw%3D3%26q%3D1%231v&",
    "?q=http%253A%252F%252F%255B%253A%253A1%255D%253A123%252F%253Fqw%253D3%2526q%253D1%25231v&",
];

pub struct UriValueTree {
    pub with_schema_and_host: BoolValueTree,
    pub schema: IndexValueTree,
    pub authority: IndexValueTree,
    pub host: IndexValueTree,
    pub path: IndexValueTree,
    pub query: IndexValueTree,

    pub whose_turn: usize,
}


impl Arbitrary for ArbitraryUri {
    type Parameters = ();
    type Strategy = UriStrategy;

    fn arbitrary_with((): Self::Parameters) -> Self::Strategy {
        UriStrategy
    }
}

impl Strategy for UriStrategy {
    type Value = ArbitraryUri;
    type Tree = UriValueTree;

    fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
        Ok(UriValueTree {
            with_schema_and_host : weighted(0.95).new_tree(runner)?,
            schema:    Index::arbitrary().new_tree(runner)?,
            authority: Index::arbitrary().new_tree(runner)?,
            host:      Index::arbitrary().new_tree(runner)?,
            path:      Index::arbitrary().new_tree(runner)?,
            query:     Index::arbitrary().new_tree(runner)?,
            whose_turn : 1,
        })
    }
}

impl ValueTree for UriValueTree {
    type Value = ArbitraryUri;

    fn current(&self) -> Self::Value {
        //let uri : http::uri::Uri = "/test".parse().unwrap();
        let mut s = String::with_capacity(50);

        if self.with_schema_and_host.current() {
            s += *self.schema.current().get(&SCHEMAS[..]);
            s += *self.authority.current().get(&AUTHS[..]);
            s += *self.host.current().get(&HOSTS[..]);
        }
        s += *self.path.current().get(&PATHS[..]);
        s += *self.query.current().get(&QUERIES[..]);

        //eprintln!("Q: {}", s);
        let uri : http::uri::Uri = s.parse().unwrap();
        ArbitraryUri(uri)
    }

    fn simplify(&mut self) -> bool {
        let mut ctr = 0;
        loop {
            if self.simplify1() {
                self.advance_turn();
                return true
            } else {
                self.advance_turn();
                ctr += 1;
            }
            if ctr == 5 {
                return false;
            }
        }
    }

    fn complicate(&mut self) -> bool {
        self.unadvance_turn();
        let ret = self.complicate1();
        self.advance_turn(); // to prevent looping on one field
        ret
    }
}

impl UriValueTree {
    fn simplify1(&mut self) -> bool {
        match self.whose_turn {
            0 => self.with_schema_and_host.simplify(),
            1 => self.schema.simplify(),
            2 => self.authority.simplify(),
            3 => self.path.simplify(),
            4 => self.query.simplify(),
            _ => unreachable!(),
        }
    }
    fn complicate1(&mut self) -> bool {
        match self.whose_turn {
            0 => self.with_schema_and_host.complicate(),
            1 => self.schema.complicate(),
            2 => self.authority.complicate(),
            3 => self.path.complicate(),
            4 => self.query.complicate(),
            _ => unreachable!(),
        }
    }

    fn advance_turn(&mut self) {
        self.whose_turn += 1;
        if self.whose_turn > 4 {
            self.whose_turn = 0;
        }
    }

    fn unadvance_turn(&mut self) {
        if self.whose_turn == 0 {
            self.whose_turn = 5;
        }
        self.whose_turn -= 1;
    }
}
