use super::*;

#[test]
fn sanitize_accepts_clean_names() {
    assert_eq!(
        sanitize_saver_name("security"),
        Some("security".to_string())
    );
    assert_eq!(
        sanitize_saver_name("/path/to/beams.scr"),
        Some("beams".to_string())
    );
    assert_eq!(
        sanitize_saver_name("screensaver-storm"),
        Some("storm".to_string())
    );
}

#[test]
fn sanitize_rejects_bad_names() {
    assert!(sanitize_saver_name("evil;rm -rf /").is_none());
    assert_eq!(
        sanitize_saver_name("../../etc/passwd"),
        Some("passwd".to_string())
    );
    assert_eq!(
        sanitize_saver_name("not-a-real-saver"),
        Some("not-a-real-saver".to_string())
    );
    assert!(sanitize_saver_name("").is_none());
}

#[test]
fn allowlist_is_complete() {
    assert_eq!(ALLOWED_SAVERS.len(), 7);
    assert!(ALLOWED_SAVERS.contains(&"beams"));
}
