#[cfg(test)]
mod e2e {
    use std::fs;

    use jq;

    #[test]
    fn test_e2e_valid() {
        let dirname = "tests/testdata/valid";
        let filenames = get_all_files(dirname);
        for filename in filenames {
            let path = format!("{}/{}", dirname, filename);
            let result = jq::process_file(&path);
            assert!(result.is_ok(), "Error processing file: {}", path);
        }
    }

    #[test]
    fn test_e2e_invalid() {
        let dirname = "tests/testdata/invalid";
        let filenames = get_all_files(dirname);
        for filename in filenames {
            let path = format!("{}/{}", dirname, filename);
            let result = jq::process_file(&path);
            assert!(result.is_err(), "Expected error on file: {}", path);
        }
    }

    fn get_all_files(dir: &str) -> Vec<String> {
        let entries = fs::read_dir(dir).unwrap();
        let mut files = Vec::new();

        for entry in entries {
            let entry = entry.unwrap();
            let file_name = entry.file_name().into_string().unwrap();

            if entry.file_type().unwrap().is_file() {
                files.push(file_name);
            }
        }

        files
    }
}
