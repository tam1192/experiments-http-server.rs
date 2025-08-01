#[test]
fn success_from_str() {
    assert_eq!(
        super::HttpMethod::from_str("GET"),
        Some(super::HttpMethod::Get)
    );
    assert_eq!(
        super::HttpMethod::from_str("POST"),
        Some(super::HttpMethod::Post)
    );
    assert_eq!(
        super::HttpMethod::from_str("PUT"),
        Some(super::HttpMethod::Put)
    );
    assert_eq!(
        super::HttpMethod::from_str("DELETE"),
        Some(super::HttpMethod::Delete)
    );
    assert_eq!(
        super::HttpMethod::from_str("get"),
        Some(super::HttpMethod::Get)
    );
    assert_eq!(
        super::HttpMethod::from_str("post"),
        Some(super::HttpMethod::Post)
    );
    assert_eq!(
        super::HttpMethod::from_str("put"),
        Some(super::HttpMethod::Put)
    );
    assert_eq!(
        super::HttpMethod::from_str("delete"),
        Some(super::HttpMethod::Delete)
    );
}

#[test]
fn failure_from_str() {
    assert!(super::HttpMethod::from_str("").is_none());
    assert!(super::HttpMethod::from_str("hello").is_none());
    assert!(super::HttpMethod::from_str("HELLO").is_none());
}
