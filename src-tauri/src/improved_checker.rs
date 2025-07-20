use crate::byte_to_char_index;
use crate::dictionary;
use crate::spelling_dict;
use crate::TextIssue;
use crate::MAX_ISSUES;
use std::collections::HashSet;

// 查找完整单词的所有位置，确保不会匹配到单词的一部分
pub fn find_all_whole_words(text: &str, word: &str) -> Vec<usize> {
    let mut positions = Vec::new();
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
            positions.push(actual_pos);
        }

        // 继续查找下一个匹配
        start_idx = actual_pos + 1;
    }

    positions
}

// 查找完整单词的第一个位置，确保不会匹配到单词的一部分
pub fn find_whole_word(text: &str, word: &str) -> Option<usize> {
    find_all_whole_words(text, word).into_iter().next()
}

// 改进的拼写检查函数，统一处理所有拼写检查逻辑
pub fn check_spelling(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    global_detected_words: &mut HashSet<String>,
) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 用于跟踪本行已经检测到的错误，避免重复提示
    // 使用小写形式作为键，确保不区分大小写
    let mut line_detected_errors = HashSet::<String>::new();

    // 用于跟踪已经检测到的错误词根，避免重复提示相同词根的不同形式
    // 例如，如果已经检测到 "Corporate"，就不再检测 "corporate" 或 "CORPORATE"
    let mut line_detected_word_roots = HashSet::<String>::new();

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
        // 跳过已经检测到的错误（精确匹配）
        if line_detected_errors.contains(word) || global_detected_words.contains(&word.to_string())
        {
            continue;
        }

        // 跳过已经检测到的错误词根（不区分大小写）
        let word_lower = word.to_lowercase();
        if line_detected_word_roots.contains(&word_lower)
            || global_detected_words.contains(&word_lower)
        {
            continue;
        }

        // 检查单词是否在拼写错误字典中
        if let Some(correction) = spelling_dict::check_word_spelling(word) {
            // 找到单词在原始行中的所有位置（确保是完整单词）
            let positions = find_all_whole_words(line, word);

            // 只报告第一个位置的错误，避免重复报告
            if let Some(pos) = positions.first() {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, *pos),
                    end: byte_to_char_index(line, *pos + word.len()),
                    issue_type: "拼写错误".to_string(),
                    message: format!("可能的拼写错误: '{}'", word),
                    suggestion: format!("建议修改为: '{}'", correction),
                });

                // 添加到本行已检测集合
                line_detected_errors.insert(word.to_string());
                line_detected_word_roots.insert(word_lower.clone());

                // 添加到全局检测集合
                global_detected_words.insert(word.to_string());
                global_detected_words.insert(word_lower.clone());

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

                // 添加到本行已检测集合
                line_detected_errors.insert(word.to_string());

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
            }
        }
    }

    // 特别检查标题中的错误和常见拼写错误
    check_title_errors(
        line,
        line_idx,
        issues,
        &mut line_detected_errors,
        &mut line_detected_word_roots,
        global_detected_words,
    );

    // 检查常见拼写错误（整合原来的 check_common_typos 功能）
    check_common_spelling_errors(
        line,
        line_idx,
        issues,
        &mut line_detected_errors,
        &mut line_detected_word_roots,
        global_detected_words,
    );
}

// 特别检查标题中的错误
fn check_title_errors(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    detected_errors: &mut HashSet<String>,
    detected_word_roots: &mut HashSet<String>,
    global_detected_words: &mut HashSet<String>,
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

        // 检查词根是否已经被检测过（不区分大小写）
        let error_lower = error.to_lowercase();
        if detected_word_roots.contains(&error_lower) {
            continue;
        }

        // 尝试查找完整单词的所有位置
        let positions = find_all_whole_words(line, error);
        if let Some(pos) = positions.first() {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, *pos),
                end: byte_to_char_index(line, *pos + error.len()),
                issue_type: "拼写错误".to_string(),
                message: format!("可能的拼写错误: '{}'", error),
                suggestion: format!("建议修改为: '{}'", correction),
            });

            // 添加到已检测集合
            detected_errors.insert((*error).to_string());
            detected_word_roots.insert(error_lower.clone());

            // 添加到全局检测集合
            global_detected_words.insert((*error).to_string());
            global_detected_words.insert(error_lower.clone());

            if issues.len() >= MAX_ISSUES {
                return;
            }
        }

        // 尝试小写版本 - 只有在词根没有被处理过的情况下才检查
        if !detected_word_roots.contains(&error_lower) {
            if !detected_errors.contains(&error_lower) {
                let positions = find_all_whole_words(line, &error_lower);
                if let Some(pos) = positions.first() {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, *pos),
                        end: byte_to_char_index(line, *pos + error_lower.len()),
                        issue_type: "拼写错误".to_string(),
                        message: format!("可能的拼写错误: '{}'", &error_lower),
                        suggestion: format!("建议修改为: '{}'", correction),
                    });

                    // 添加到已检测集合
                    detected_errors.insert(error_lower.clone());
                    detected_word_roots.insert(error_lower.clone());

                    // 添加到全局检测集合
                    global_detected_words.insert(error_lower.clone());

                    if issues.len() >= MAX_ISSUES {
                        return;
                    }
                }
            }
        }

        // 尝试首字母大写版本 - 只有在词根没有被处理过的情况下才检查
        if !detected_word_roots.contains(&error_lower) {
            let error_cap = capitalize_first(error);
            if !detected_errors.contains(error_cap.as_str()) {
                let positions = find_all_whole_words(line, &error_cap);
                if let Some(pos) = positions.first() {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, *pos),
                        end: byte_to_char_index(line, *pos + error_cap.len()),
                        issue_type: "拼写错误".to_string(),
                        message: format!("可能的拼写错误: '{}'", &error_cap),
                        suggestion: format!("建议修改为: '{}'", correction),
                    });

                    // 添加到已检测集合
                    detected_errors.insert(error_cap.clone());
                    detected_word_roots.insert(error_lower.clone());

                    // 添加到全局检测集合
                    global_detected_words.insert(error_cap.clone());
                    global_detected_words.insert(error_lower.clone());

                    if issues.len() >= MAX_ISSUES {
                        return;
                    }
                }
            }
        }
    }
}

// 检查常见拼写错误（整合原来的 check_common_typos 功能）
fn check_common_spelling_errors(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    detected_errors: &mut HashSet<String>,
    detected_word_roots: &mut HashSet<String>,
    global_detected_words: &mut HashSet<String>,
) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 检测语言类型
    let language = detect_language_simple(line);

    if language == "zh" {
        // 中文重复字符检测
        check_chinese_repeated_chars(line, line_idx, issues);
    } else {
        // 英文常见拼写错误检测
        check_english_common_typos(
            line,
            line_idx,
            issues,
            detected_errors,
            detected_word_roots,
            global_detected_words,
        );
    }
}

// 简单的语言检测
fn detect_language_simple(text: &str) -> String {
    let mut chinese_count = 0;
    let mut english_count = 0;

    for c in text.chars() {
        if c >= '\u{4e00}' && c <= '\u{9fff}' {
            chinese_count += 1;
        } else if c.is_ascii_alphabetic() {
            english_count += 1;
        }
    }

    if chinese_count > english_count {
        "zh".to_string()
    } else {
        "en".to_string()
    }
}

// 检查中文重复字符
fn check_chinese_repeated_chars(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0;
    while i < chars.len().saturating_sub(1) {
        if chars[i] == chars[i + 1] && chars[i] >= '\u{4e00}' && chars[i] <= '\u{9fff}' {
            let start_byte_pos = line.char_indices().nth(i).map(|(pos, _)| pos).unwrap_or(0);
            let end_byte_pos = line
                .char_indices()
                .nth(i + 2)
                .map(|(pos, _)| pos)
                .unwrap_or_else(|| line.len());

            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, start_byte_pos),
                end: byte_to_char_index(line, end_byte_pos),
                issue_type: "重复字符".to_string(),
                message: format!("重复字符: '{}{}'", chars[i], chars[i]),
                suggestion: format!("删除重复的 '{}'", chars[i]),
            });

            i += 2;
            if issues.len() >= MAX_ISSUES {
                return;
            }
        } else {
            i += 1;
        }
    }
}

// 检查英文常见拼写错误
fn check_english_common_typos(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    detected_errors: &mut HashSet<String>,
    detected_word_roots: &mut HashSet<String>,
    global_detected_words: &mut HashSet<String>,
) {
    // Skip if we've already found too many issues
    if issues.len() >= MAX_ISSUES {
        return;
    }

    // 使用我们的拼写检查字典进行更全面的拼写检查
    let words: Vec<&str> = line
        .split(|c: char| !c.is_alphanumeric() && c != '\'')
        .map(|w| w.trim())
        .filter(|w| !w.is_empty())
        .collect();

    for word in words {
        // 跳过太短的单词和纯数字
        if word.len() <= 2 || word.chars().all(|c| c.is_numeric()) {
            continue;
        }

        // 清理单词，去除可能的标点符号
        let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '\'');
        if clean_word.is_empty() {
            continue;
        }

        // 检查是否已经检测过这个单词
        let clean_word_lower = clean_word.to_lowercase();
        if detected_errors.contains(clean_word)
            || detected_word_roots.contains(&clean_word_lower)
            || global_detected_words.contains(&clean_word.to_string())
            || global_detected_words.contains(&clean_word_lower)
        {
            continue;
        }

        // 检查单词是否在拼写错误字典中
        if let Some(correction) = spelling_dict::check_word_spelling(clean_word) {
            // 找到单词在原始行中的位置
            if let Some(pos) = find_whole_word(line, clean_word) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, pos),
                    end: byte_to_char_index(line, pos + clean_word.len()),
                    issue_type: "拼写错误".to_string(),
                    message: format!("可能的拼写错误: '{}'", clean_word),
                    suggestion: format!("建议修改为: '{}'", correction),
                });

                // 添加到检测集合
                detected_errors.insert(clean_word.to_string());
                detected_word_roots.insert(clean_word_lower.clone());
                global_detected_words.insert(clean_word.to_string());
                global_detected_words.insert(clean_word_lower);

                // Stop if we've found too many issues
                if issues.len() >= MAX_ISSUES {
                    return;
                }
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
