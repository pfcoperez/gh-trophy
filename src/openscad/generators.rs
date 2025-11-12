use std::fmt::Debug;

pub fn generate_data_source<T>(
    user_handle: String,
    date_str: String,
    activity_data: Vec<Vec<T>>,
) -> String
where
    T: ToString,
    T: Debug,
{
    let mut builder = String::new();

    builder.push_str("// To be used in other OpenScad source file with\n");
    builder.push_str("// include <activity-data.scad>\n");
    builder.push_str("rawActivity = [\n");
    for row in activity_data.iter() {
        builder.push_str(format!("    {:?},\n", row).as_str());
    }
    builder.push_str("];\n");

    builder.push_str("\n");
    builder.push_str("\n");

    builder.push_str(format!("ghHandleTxt = \"{}\";\n", user_handle).as_str());
    builder.push_str(format!("spanTxt = \"{}\";\n", date_str).as_str());

    return builder;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_matrix_source_empty() {
        let data: Vec<Vec<i32>> = vec![];
        let result = generate_data_source("test".to_string(), data);

        assert!(result.contains("// To be used in other OpenScad source file with"));
        assert!(result.contains("// include <test.scad>"));
        assert!(result.contains("test = ["));
        assert!(result.contains("];"));
    }

    #[test]
    fn test_generate_matrix_source_single_row() {
        let data = vec![vec![1, 2, 3]];
        let result = generate_data_source("myMatrix".to_string(), data);

        assert!(result.contains("// include <myMatrix.scad>"));
        assert!(result.contains("myMatrix = ["));
        assert!(result.contains("[1, 2, 3],"));
        assert!(result.contains("];"));
    }

    #[test]
    fn test_generate_matrix_source_multiple_rows() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = generate_data_source("matrix3x3".to_string(), data);

        assert!(result.contains("// include <matrix3x3.scad>"));
        assert!(result.contains("matrix3x3 = ["));
        assert!(result.contains("[1, 2, 3],"));
        assert!(result.contains("[4, 5, 6],"));
        assert!(result.contains("[7, 8, 9],"));
        assert!(result.contains("];"));
    }

    #[test]
    fn test_generate_matrix_source_with_zeros() {
        let data = vec![vec![0, 0, 0], vec![0, 1, 0]];
        let result = generate_data_source("sparse".to_string(), data);

        assert!(result.contains("[0, 0, 0],"));
        assert!(result.contains("[0, 1, 0],"));
    }

    #[test]
    fn test_generate_matrix_source_large_numbers() {
        let data = vec![vec![100, 999, 1234567]];
        let result = generate_data_source("large".to_string(), data);

        assert!(result.contains("[100, 999, 1234567],"));
    }

    #[test]
    fn test_generate_matrix_source_format() {
        let data = vec![vec![1]];
        let result = generate_data_source("format".to_string(), data);

        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
        assert_eq!(lines[0], "// To be used in other OpenScad source file with");
        assert_eq!(lines[1], "// include <format.scad>");
        assert_eq!(lines[2], "format = [");
        assert!(lines[3].starts_with("    [1]"));
        assert_eq!(lines[4], "];");
    }

    #[test]
    fn test_generate_matrix_source_different_types() {
        // Test with u32 (used in real code)
        let data: Vec<Vec<u32>> = vec![vec![10, 20], vec![30, 40]];
        let result = generate_data_source("u32_matrix".to_string(), data);

        assert!(result.contains("[10, 20],"));
        assert!(result.contains("[30, 40],"));
    }

    #[test]
    fn test_generate_matrix_source_strings() {
        // Test with strings to verify generic type parameter works
        let data = vec![vec!["a", "b"], vec!["c", "d"]];
        let result = generate_data_source("string_matrix".to_string(), data);

        // Strings will be formatted with quotes by Debug trait
        assert!(result.contains("\"a\""));
        assert!(result.contains("\"b\""));
        assert!(result.contains("\"c\""));
        assert!(result.contains("\"d\""));
    }

    #[test]
    fn test_generate_matrix_source_irregular_matrix() {
        // Test with rows of different lengths
        let data = vec![vec![1, 2, 3, 4], vec![5, 6], vec![7]];
        let result = generate_data_source("irregular".to_string(), data);

        assert!(result.contains("[1, 2, 3, 4],"));
        assert!(result.contains("[5, 6],"));
        assert!(result.contains("[7],"));
    }
}
