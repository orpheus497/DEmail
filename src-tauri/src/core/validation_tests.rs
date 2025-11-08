// Additional integration tests for validation module
// The main validation tests are in validation.rs, these are additional integration tests

#[cfg(test)]
mod validation_integration_tests {
    use crate::core::validation::*;
    use crate::error::DEmailError;
    use std::path::PathBuf;

    #[test]
    fn test_email_validation_edge_cases() {
        // Valid edge cases
        assert!(validate_email("a@b.co").is_ok());
        assert!(validate_email("test+tag@example.com").is_ok());
        assert!(validate_email("user.name@example.co.uk").is_ok());

        // Invalid edge cases
        assert!(validate_email("@example.com").is_err());
        assert!(validate_email("user@").is_err());
        assert!(validate_email("user@@example.com").is_err());
        assert!(validate_email("user @example.com").is_err());
    }

    #[test]
    fn test_subject_validation_special_chars() {
        // Valid subjects with special characters
        assert!(validate_subject("Re: Meeting @ 3pm").is_ok());
        assert!(validate_subject("[URGENT] Project Update #42").is_ok());
        assert!(validate_subject("Question: What's next?").is_ok());

        // Unicode support
        assert!(validate_subject("Привет мир").is_ok());
        assert!(validate_subject("你好世界").is_ok());
        assert!(validate_subject("🎉 Celebration").is_ok());
    }

    #[test]
    fn test_body_validation_size_limits() {
        // Just under the limit
        let almost_max = "a".repeat(10 * 1024 * 1024 - 1);
        assert!(validate_body(&almost_max).is_ok());

        // At the limit
        let exactly_max = "a".repeat(10 * 1024 * 1024);
        assert!(validate_body(&exactly_max).is_ok());

        // Over the limit
        let over_max = "a".repeat(10 * 1024 * 1024 + 1);
        assert!(validate_body(&over_max).is_err());
    }

    #[test]
    fn test_filename_sanitization_comprehensive() {
        // Path traversal attempts
        assert!(sanitize_filename("../../etc/passwd").is_err());
        assert!(sanitize_filename("..\\..\\windows\\system32").is_err());
        assert!(sanitize_filename("./secret.txt").is_err());

        // Null bytes
        assert!(sanitize_filename("file\0.txt").is_err());

        // Valid filenames
        assert_eq!(sanitize_filename("report.pdf").unwrap(), "report.pdf");
        assert_eq!(sanitize_filename("my-file_2024.docx").unwrap(), "my-file_2024.docx");

        // Special characters that should be sanitized
        let result = sanitize_filename("file:name.txt");
        assert!(result.is_ok());
        assert!(!result.unwrap().contains(':'));
    }

    #[test]
    fn test_path_validation_comprehensive() {
        let base = PathBuf::from("/tmp/demail");

        // Valid paths within base
        assert!(validate_path(&base.join("attachments"), &base).is_ok());
        assert!(validate_path(&base.join("exports/user1"), &base).is_ok());

        // Path traversal attempts
        assert!(validate_path(&PathBuf::from("/tmp"), &base).is_err());
        assert!(validate_path(&PathBuf::from("/etc/passwd"), &base).is_err());
        assert!(validate_path(&base.join("../../../etc/passwd"), &base).is_err());
    }

    #[test]
    fn test_pagination_validation_comprehensive() {
        // Valid pagination
        assert!(validate_pagination(10, 0).is_ok());
        assert!(validate_pagination(100, 50).is_ok());
        assert!(validate_pagination(1, 999).is_ok());

        // Invalid limits
        assert!(validate_pagination(0, 0).is_err());
        assert!(validate_pagination(-1, 0).is_err());
        assert!(validate_pagination(1001, 0).is_err());

        // Invalid offsets
        assert!(validate_pagination(10, -1).is_err());
        assert!(validate_pagination(10, 1_000_001).is_err());
    }

    #[test]
    fn test_search_query_validation_comprehensive() {
        // Valid queries
        assert!(validate_search_query("meeting").is_ok());
        assert!(validate_search_query("project update 2024").is_ok());
        assert!(validate_search_query("a").is_ok());

        // Empty query
        assert!(validate_search_query("").is_err());

        // Too long query (over 500 chars)
        let long_query = "word ".repeat(101); // 505 chars
        assert!(validate_search_query(&long_query).is_err());

        // Unicode queries
        assert!(validate_search_query("поиск").is_ok());
        assert!(validate_search_query("搜索").is_ok());
    }

    #[test]
    fn test_validation_error_messages() {
        // Verify error messages are helpful
        match validate_email("invalid") {
            Err(DEmailError::Validation(msg)) => {
                assert!(msg.contains("email") || msg.contains("invalid"));
            }
            _ => panic!("Expected validation error"),
        }

        match validate_subject(&"x".repeat(1000)) {
            Err(DEmailError::Validation(msg)) => {
                assert!(msg.contains("subject") || msg.contains("long"));
            }
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_xss_protection_in_body() {
        // Script tags should be allowed (sanitization happens elsewhere)
        // Validation is just for size and basic structure
        let script_body = "<script>alert('xss')</script>";
        assert!(validate_body(script_body).is_ok());

        // Validation doesn't sanitize, just validates structure
        let html_body = "<html><body>Content</body></html>";
        assert!(validate_body(html_body).is_ok());
    }
}
