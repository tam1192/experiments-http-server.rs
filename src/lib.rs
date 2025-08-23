use httparse::{EMPTY_HEADER, Header, Request};

#[test]
fn ex_request() {
    let buf = b"POST / HTTP/1.1\r\nHost: localhost\r\nSec-Fetch-Dest: document\r\nUser-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15\r\nUpgrade-Insecure-Requests: 1\r\nAccept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\nSec-Fetch-Site: none\r\nSec-Fetch-Mode: navigate\r\nAccept-Language: ja\r\nPriority: u=0, i\r\nAccept-Encoding: gzip, deflate\r\nConnection: keep-alive\r\nContent-Type: text/plain\r\n\r\nhello world";

    let mut headers = [EMPTY_HEADER; 64];
    let mut request = Request::new(&mut headers);

    let head_size = request.parse(buf).unwrap().unwrap();

    assert_eq!(request.method.unwrap(), "POST");
    assert_eq!(request.path.unwrap(), "/");

    // 1.0だと0, 1.1だと1
    // 1.0, 1.1以外はnoneになる?
    assert_eq!(request.version.unwrap(), 1);

    println!("{:?}", headers);

    assert_eq!(String::from_utf8_lossy(&buf[head_size..]), "hello world");
}

#[test]
fn ex_response() {
    let _buf = r#"HTTP/1.1 200 OK"#;
}
