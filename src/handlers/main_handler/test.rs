#[test]
fn test_version_greater() {
    use super::parsers::version_greater;
    assert_eq!(
        version_greater("1.0.0".to_string(), "2.0.1".to_string()),
        false
    );
    assert_eq!(
        version_greater("3.0.0".to_string(), "2.0.1".to_string()),
        true
    );
    assert_eq!(
        version_greater("0.0.0".to_string(), "0.0.0".to_string()),
        false
    );
    assert_eq!(
        version_greater("1.1.1".to_string(), "1.1.0".to_string()),
        true
    );
    assert_eq!(
        version_greater("2.2.2".to_string(), "2.1.4".to_string()),
        true
    );
    assert_eq!(
        version_greater("2.3.1".to_string(), "2.9.2".to_string()),
        false
    );
}
