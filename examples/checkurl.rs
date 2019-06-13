extern crate http;

fn main() {
    let arg = std::env::args().nth(1).unwrap();
    let url : http::uri::Uri = arg.parse().unwrap();
    println!("{:?}", url.into_parts());
}
