use crate::byte_to_char_index;
use crate::TextIssue;
use regex::Regex;

// Check for word order issues in Chinese
pub fn check_word_order(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Common word order issues in Chinese
    let word_order_patterns = [
        (r"不仅没有.+也没有", "不仅没有...而且没有", "搭配不当"),
        (r"不仅.+而且没有", "不仅...也没有", "搭配不当"),
        (r"虽然.+但是", "虽然...但", "虽然和但是不应同时使用"),
        (r"因为.+所以", "因为...所以", "因为和所以不应同时使用"),
    ];

    for (pattern, correct_form, explanation) in word_order_patterns {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

        if let Some(mat) = regex.find(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "语序问题".to_string(),
                message: format!("语序结构: {}", mat.as_str()),
                suggestion: format!("建议使用: {}, {}", correct_form, explanation),
            });
        }
    }
}

// Check for Chinese punctuation issues
pub fn check_chinese_punctuation(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Check for consecutive identical punctuation
    let consecutive_punct_regex = match Regex::new(r"[，。！？；：、]{2,}") {
        Ok(re) => re,
        Err(_) => return, // Return early if regex creation fails
    };

    for mat in consecutive_punct_regex.find_iter(line) {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, mat.start()),
            end: byte_to_char_index(line, mat.end()),
            issue_type: "标点符号".to_string(),
            message: "连续使用相同的标点符号".to_string(),
            suggestion: "使用单个标点符号".to_string(),
        });
    }

    // Check for unpaired brackets
    if line.contains("（") && !line.contains("）") {
        if let Some(pos) = line.find("（") {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + "（".len()),
                issue_type: "标点符号".to_string(),
                message: "圆括号不配对".to_string(),
                suggestion: "添加右括号）".to_string(),
            });
        }
    }
}

// Check for tense consistency in English
pub fn check_tense_consistency(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Past tense markers
    let past_tense_markers = [
        "yesterday",
        "last week",
        "last month",
        "last year",
        "ago",
        "in the past",
    ];

    // Present tense verbs
    let present_verbs = [r"\b(is|are|am)\b", r"\b(do|does)\b", r"\b(have|has)\b"];

    // Check for past tense markers with present tense verbs
    for marker in past_tense_markers {
        if line.to_lowercase().contains(marker) {
            for verb_pattern in present_verbs {
                let regex = match Regex::new(verb_pattern) {
                    Ok(re) => re,
                    Err(_) => continue, // Skip this pattern if regex creation fails
                };

                if let Some(mat) = regex.find(line) {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, mat.start()),
                        end: byte_to_char_index(line, mat.end()),
                        issue_type: "时态一致性".to_string(),
                        message: "过去时间标记与现在时态动词".to_string(),
                        suggestion: "使用过去时态动词".to_string(),
                    });
                }
            }
        }
    }
}

// Check for preposition usage in English
pub fn check_preposition_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Common preposition errors
    let preposition_errors = [
        (r"\bdifferent to\b", "different from", "不正确的介词搭配"),
        (r"\bargue on\b", "argue about", "不正确的介词搭配"),
        (r"\barrive to\b", "arrive at/in", "不正确的介词搭配"),
    ];

    for (pattern, correct_form, explanation) in preposition_errors {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

        if let Some(mat) = regex.find(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "介词用法".to_string(),
                message: format!("介词用法不当: {}", mat.as_str()),
                suggestion: format!("建议使用: {}, {}", correct_form, explanation),
            });
        }
    }
}
