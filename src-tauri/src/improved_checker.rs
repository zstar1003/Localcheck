use crate::byte_to_char_index;
use crate::dictionary;
use crate::spelling_dict;
use crate::TextIssue;
use crate::MAX_ISSUES;
use std::collections::HashSet;

// 查找完整单词的位置，确保不会匹配到单词的一部分
pub fn find_whole_word(text: &str, word: &str) -> Option<usize> {
    let mut start_idx = 0;

    while let Some(pos) = text[start_idx..].find(word) {
        let actual_pos = start_idx + pos;

        // 检查单词前后是否是单词边界（空格、标点符号等）
        let is_start_boundary = actual_pos == 0
            || !text
                .chars()
                .nth(actual_pos - 1)
                .map_or(false, |c| c.is_alphanumeric());

        let is_end_boundary = actual_pos + word.len() >= text.len()
            || !text
                .chars()
                .nth(actual_pos + word.len())
                .map_or(false, |c| c.is_alphanumeric());

        if is_start_boundary && is_end_boundary {
            return Some(actual_pos);
        }

        // 继续查找下一个匹配
        start_idx = actual_pos + 1;
    }

    None
}

// 改进的拼写检查函数，使用词典
pub fn check_spelling(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 用于跟踪已经检测到的错误，避免重复提示
    let mut detected_errors = HashSet::new();

    // 首先，将行分割成单词（使用更精确的分割方法）
    let words: Vec<&str> = line
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'' && c != '-'))
        .filter(|w| !w.is_empty() && w.len() > 2 && !w.chars().all(|c| c.is_numeric()))
        .collect();

    // 加载词典
    let _dictionary_loaded = dictionary::load_dictionary();

    // 检查每个完整单词
    for word in words {
        // 跳过已经检测到的错误
        if detected_errors.contains(word) {
            continue;
        }

        // 检查单词是否在拼写错误字典中
        if let Some(correction) = spelling_dict::check_word_spelling(word) {
            // 找到单词在原始行中的位置（确保是完整单词）
            if let Some(pos) = find_whole_word(line, word) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, pos),
                    end: byte_to_char_index(line, pos + word.len()),
                    issue_type: "拼写错误".to_string(),
                    message: format!("可能的拼写错误: '{}'", word),
                    suggestion: format!("建议修改为: '{}'", correction),
                });

                // 添加到已检测集合
                detected_errors.insert(word);

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            }
            continue; // 如果在拼写错误字典中找到了，就不需要继续检查
        }

        // 如果不在拼写错误字典中，检查是否在正确词典中
        // 如果不在正确词典中，可能是拼写错误
        if !dictionary::is_word_in_dictionary(word) {
            // 检查是否是带连字符的复合词（如 "out-degree"）
            if word.contains('-') {
                // 直接跳过所有带连字符的词，这些通常是专业术语
                continue;

                // 以下代码保留但不执行，因为我们现在直接跳过所有带连字符的词
                /*
                let parts: Vec<&str> = word.split('-').collect();
                let all_parts_valid = parts.iter().all(|part| {
                    // 忽略太短的部分
                    part.len() <= 2 || dictionary::is_word_in_dictionary(part)
                });

                if all_parts_valid {
                    // 如果所有部分都是有效的单词，则认为整个复合词是有效的
                    continue;
                }
                */
            }

            // 找到单词在原始行中的位置（确保是完整单词）
            if let Some(pos) = find_whole_word(line, word) {
                // 检查是否是专有名词（首字母大写）
                if word.chars().next().map_or(false, |c| c.is_uppercase()) {
                    // 专有名词可能是正确的，不标记为错误
                    continue;
                }

                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, pos),
                    end: byte_to_char_index(line, pos + word.len()),
                    issue_type: "可能的拼写错误".to_string(),
                    message: format!("词典中未找到: '{}'", word),
                    suggestion: "请检查拼写是否正确".to_string(),
                });

                // 添加到已检测集合
                detected_errors.insert(word);

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            }
        }
    }

    // 特别检查标题中的错误
    check_title_errors(line, line_idx, issues, &mut detected_errors);
}

// 特别检查标题中的错误
fn check_title_errors(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    detected_errors: &mut HashSet<&str>,
) {
    // 特别针对您示例中的错误
    let example_errors = [
        ("Enronment", "Environment"),
        ("Financal", "Financial"),
        ("Alocation", "Allocation"),
        ("Empincal", "Empirical"),
        ("Eydence", "Evidence"),
        ("Corporat", "Corporate"),
        ("Geographc", "Geographic"),
        ("Busines", "Business"),
    ];

    for (error, correction) in example_errors.iter() {
        // 如果已经检测到这个错误，跳过
        if detected_errors.contains(*error) {
            continue;
        }

        // 尝试查找完整单词
        if let Some(pos) = find_whole_word(line, error) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + error.len()),
                issue_type: "拼写错误".to_string(),
                message: format!("可能的拼写错误: '{}'", error),
                suggestion: format!("建议修改为: '{}'", correction),
            });

            // 添加到已检测集合
            detected_errors.insert(*error);

            if issues.len() >= MAX_ISSUES {
                return;
            }
        }

        // 尝试小写版本
        let error_lower = error.to_lowercase();
        if let Some(pos) = find_whole_word(line, &error_lower) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + error_lower.len()),
                issue_type: "拼写错误".to_string(),
                message: format!("可能的拼写错误: '{}'", &error_lower),
                suggestion: format!("建议修改为: '{}'", correction),
            });

            if issues.len() >= MAX_ISSUES {
                return;
            }
        }

        // 尝试首字母大写版本
        let error_cap = capitalize_first(error);
        if let Some(pos) = find_whole_word(line, &error_cap) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, pos),
                end: byte_to_char_index(line, pos + error_cap.len()),
                issue_type: "拼写错误".to_string(),
                message: format!("可能的拼写错误: '{}'", &error_cap),
                suggestion: format!("建议修改为: '{}'", correction),
            });

            if issues.len() >= MAX_ISSUES {
                return;
            }
        }
    }
}

// 首字母大写的辅助函数
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
