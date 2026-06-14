#[cfg(test)]
mod tests {
    use json_parser::{JsonParser, lex};

    use std::fs;

    fn run_test_file(path: &std::path::Path) -> bool {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return false,
        };

        let tokens = match lex(&content) {
            Ok(t) => t,
            Err(_) => return false,
        };

        let mut parser = JsonParser::new(&tokens);
        parser.parse().is_ok()
    }

    #[test]
    fn test_all_json_files() {
        let dir_path = "test/";

        if let Ok(paths) = fs::read_dir(dir_path) {
            for path in paths {
                let p = path.expect("Failed to read path").path();
                let p_str = p.to_str().unwrap();

                let is_expected_to_fail = p_str.contains("invalid") || p_str.contains("fail");

                let result = run_test_file(&p);

                if is_expected_to_fail {
                    assert!(!result, "File should have been invalid but passed: {:?}", p);
                } else {
                    assert!(result, "File should have been valid but failed: {:?}", p);
                }
            }
        }
    }
}
