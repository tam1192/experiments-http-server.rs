#[test]
fn success_from_str() {
    assert_eq!(
        super::HttpVersion::from_str("HTTP/1.0"),
        Some(super::HttpVersion::Http10)
    );
    assert_eq!(
        super::HttpVersion::from_str("HTTP/1.1"),
        Some(super::HttpVersion::Http11)
    );
    assert_eq!(
        super::HttpVersion::from_str("HTTP/2.0"),
        Some(super::HttpVersion::Http20)
    );
    assert_eq!(
        super::HttpVersion::from_str("HTTP/3.0"),
        Some(super::HttpVersion::Http30)
    );
    assert_eq!(
        super::HttpVersion::from_str("http/1.0"),
        Some(super::HttpVersion::Http10)
    );
    assert_eq!(
        super::HttpVersion::from_str("http/1.1"),
        Some(super::HttpVersion::Http11)
    );
    assert_eq!(
        super::HttpVersion::from_str("http/2.0"),
        Some(super::HttpVersion::Http20)
    );
    assert_eq!(
        super::HttpVersion::from_str("http/3.0"),
        Some(super::HttpVersion::Http30)
    );
    assert_eq!(
        super::HttpVersion::from_str("Http/1.0"),
        Some(super::HttpVersion::Http10)
    );
    assert_eq!(
        super::HttpVersion::from_str("Http/1.1"),
        Some(super::HttpVersion::Http11)
    );
    assert_eq!(
        super::HttpVersion::from_str("Http/2.0"),
        Some(super::HttpVersion::Http20)
    );
    assert_eq!(
        super::HttpVersion::from_str("Http/3.0"),
        Some(super::HttpVersion::Http30)
    );
}

#[test]
fn failure_from_str() {
    assert!(super::HttpVersion::from_str("HTTP/1.2").is_none());
    assert!(super::HttpVersion::from_str("HTTP/2.1").is_none());
    assert!(super::HttpVersion::from_str("HTTP/3.1").is_none());
}
