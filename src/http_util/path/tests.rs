use super::*;

#[test]
fn success_http_path_from_str() {
    assert_eq!(
        HttpPath::from_str("/path/to/resource").unwrap().to_string(),
        String::from("/path/to/resource")
    );
    assert_eq!(
        HttpPath::from_str("/path/to/resource?query=1")
            .unwrap()
            .to_string(),
        String::from("/path/to/resource?query=1")
    );
    assert_eq!(
        HttpPath::from_str("/").unwrap().to_string(),
        String::from("/")
    );
}

#[test]
fn failure_http_path_from_str() {
    assert!(HttpPath::from_str("").is_err());
    assert!(HttpPath::from_str("/path/to/resource/*").is_err());
    assert!(HttpPath::from_str("path/to/resource").is_err());
}
