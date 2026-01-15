/// Fuzzy string matching utilities for script suggestions
/// 
/// This module implements Levenshtein distance for finding similar strings,
/// useful for suggesting corrections when a user types an incorrect command.

/// Calculate the Levenshtein distance between two strings
/// 
/// This is a classic dynamic programming algorithm that measures the minimum
/// number of single-character edits (insertions, deletions, substitutions)
/// required to change one string into another.
/// 
/// # Rust Concepts Learned:
/// - Dynamic programming with 2D vectors
/// - String slicing with .chars()
/// - Iterators and enumerate()
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    let len_a = a_chars.len();
    let len_b = b_chars.len();
    
    // Early exit for empty strings
    if len_a == 0 { return len_b; }
    if len_b == 0 { return len_a; }
    
    // Create a 2D matrix for dynamic programming
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; len_b + 1]; len_a + 1];
    
    // Initialize first row and column
    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }
    
    // Fill in the rest of the matrix
    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            
            matrix[i][j] = (matrix[i - 1][j] + 1)           // deletion
                .min(matrix[i][j - 1] + 1)                   // insertion
                .min(matrix[i - 1][j - 1] + cost);           // substitution
        }
    }
    
    matrix[len_a][len_b]
}

/// Calculate similarity score between 0.0 and 1.0
/// Higher score means more similar
pub fn similarity_score(a: &str, b: &str) -> f64 {
    let distance = levenshtein_distance(a, b);
    let max_len = a.len().max(b.len());
    
    if max_len == 0 {
        return 1.0;
    }
    
    1.0 - (distance as f64 / max_len as f64)
}

/// Find the best matching scripts for a given input
/// Returns matches sorted by similarity (best first)
/// 
/// # Rust Concepts Learned:
/// - Sorting with sort_by() and custom comparators
/// - Closures with |a, b| syntax
/// - Partial ordering for floating point comparison
pub fn find_similar_scripts<'a>(
    input: &str,
    available_scripts: &'a [String],
    threshold: f64,
) -> Vec<(&'a str, f64)> {
    let input_lower = input.to_lowercase();
    
    let mut matches: Vec<(&str, f64)> = available_scripts
        .iter()
        .map(|script| {
            let script_lower = script.to_lowercase();
            let score = similarity_score(&input_lower, &script_lower);
            (script.as_str(), score)
        })
        .filter(|(_, score)| *score >= threshold)
        .collect();
    
    // Sort by score descending (best match first)
    matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    matches
}

/// Suggest the best matching script if one is similar enough
pub fn suggest_script(input: &str, available_scripts: &[String]) -> Option<String> {
    let matches = find_similar_scripts(input, available_scripts, 0.5);
    matches.first().map(|(script, _)| script.to_string())
}

/// Check if input is an exact match (case-insensitive)
pub fn is_exact_match(input: &str, available_scripts: &[String]) -> bool {
    let input_lower = input.to_lowercase();
    available_scripts.iter().any(|s| s.to_lowercase() == input_lower)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("abc", ""), 3);
        assert_eq!(levenshtein_distance("", "abc"), 3);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
        assert_eq!(levenshtein_distance("abc", "abd"), 1);
        assert_eq!(levenshtein_distance("test", "tset"), 2);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn test_similarity_score() {
        assert!((similarity_score("abc", "abc") - 1.0).abs() < 0.001);
        assert!((similarity_score("abc", "abd") - 0.666).abs() < 0.01);
        assert!((similarity_score("", "") - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_find_similar_scripts() {
        let scripts = vec![
            "dev".to_string(),
            "build".to_string(),
            "test".to_string(),
            "start".to_string(),
        ];

        let matches = find_similar_scripts("tets", &scripts, 0.5);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "test");

        let matches = find_similar_scripts("bild", &scripts, 0.5);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "build");
    }

    #[test]
    fn test_suggest_script() {
        let scripts = vec![
            "dev".to_string(),
            "build".to_string(),
            "test".to_string(),
        ];

        assert_eq!(suggest_script("tets", &scripts), Some("test".to_string()));
        assert_eq!(suggest_script("bld", &scripts), Some("build".to_string()));
        assert_eq!(suggest_script("xyz123", &scripts), None);
    }

    #[test]
    fn test_is_exact_match() {
        let scripts = vec!["dev".to_string(), "Build".to_string()];
        
        assert!(is_exact_match("dev", &scripts));
        assert!(is_exact_match("DEV", &scripts));
        assert!(is_exact_match("build", &scripts));
        assert!(!is_exact_match("test", &scripts));
    }
}
