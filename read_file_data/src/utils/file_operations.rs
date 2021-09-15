use std::fs::File;
use std::io::{BufRead, BufReader};

static EMPTY_STRING: &str = "";

/// Get the last line of the file
///
/// # Arguments
///
/// * `path` - This is the path of file
///
/// # Return
///
/// This function returns last line or error in the form of Result
pub fn get_last_line(path: &str) -> Result<String, String> {
    match File::open(path) {
        Ok(file) => {
            let file: BufReader<File> = BufReader::new(file);
            let mut lines: Vec<String> = file.lines().map(|line| -> String {
                match line {
                    Ok(data) => data,
                    Err(_) => EMPTY_STRING.to_string()
                }
            }).collect();
            match lines.len() > 0 {
                true => {
                    lines.reverse();
                    match lines.get(0) {
                        Some(data) => Ok(data.to_string()),
                        None => Err("Unable to extract data".to_string())
                    }
                }
                false => Err("No data found".to_string())
            }
        }
        Err(error) => Err(error.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::utils::file_operations::get_last_line;

    #[test]
    fn test_get_last_line_success() {
        assert!(get_last_line("tests/resources/test.txt").is_ok());
    }

    #[test]
    fn test_get_last_line_data_failure() {
        assert!(get_last_line("tests/resources/test1.txt").is_err());
    }

    #[test]
    fn test_get_last_line_file_failure() {
        assert!(get_last_line("tests/resources/test2.txt").is_err());
    }
}
