# proptest-http

`impl proptest::Arbitrary for http::{Request,Response,Uri};`

This crate contains the code to generate random-ish `http` objects:
urls, requests, headers, responses

It is rather simple and straightforward:
most things are just chosen from a static list

To be useful for your project, you may want to fork it and
modify the arrays.

Example URL simplification sequence:

```norust
* `https://6:%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20@example.com:8080/foo/bar/?q=http%3A%2F%2F%5B%3A%3A1%5D%3A123%2F%3Fqw%3D3%26q%3D1%231v&`
* `http://6:%5B%5D%3F%2F%3C%7E%23%6D%21%40%24%25%5E%26%2A%28%29%2B%3D%7D%7C%3A%22%3B%27%2C%3E%7B%20@example.com:8080/foo/bar/?q=http%3A%2F%2F%5B%3A%3A1%5D%3A123%2F%3Fqw%3D3%26q%3D1%231v&`
* `http://%20:%20@example.com:8080/foo/bar/?q=http%3A%2F%2F%5B%3A%3A1%5D%3A123%2F%3Fqw%3D3%26q%3D1%231v&`
* `http://%20:%20@example.com:8080/foo?q=http%3A%2F%2F%5B%3A%3A1%5D%3A123%2F%3Fqw%3D3%26q%3D1%231v&`
* `http://%20:%20@example.com:8080/foo?q=w`
* `/foo?q=w`
* `/?q=w `
* `/?`
```

Example of a request (I know that header names and values are not congruent):

```norust
Request {
    method: DELETE,
    uri: /,
    version: HTTP/1.1,
    headers: {
        "dnt": "keep-alive",
        "host": "999999999999999999999999999999999999999999999999999999",
        "date": "websocket",
        "authorization": "close",
        "upgrade": "%",
        "connection": "deflate",
        "content-type": "Thu, 20 Jun 2019 21:06:20 GMT",
        "cache-control": "\r\n",
        "expires": "	",
        "user-agent": "localhost",
        "content-length": "%",
        "server": "max-age=604800",
        "accept": "_xsrf=2|8bea5404|5ef47a59a0516e67bbd5f86849e28a1c|1553532280",
        "accept-encoding": "text/html",
        "accept-language": "websocket",
    },
    body: (),
}
```

License: MIT/Apache-2.0
