use crate::byte_to_char_index;
use crate::TextIssue;
use regex::Regex;

// Check for idiom usage - moved from lib.rs to avoid duplication
pub fn check_idiom_usage(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Common incorrect idiom usages - simplified list
    let idiom_pairs = [
        ("一鸣惊动", "一鸣惊人", "错误用法，应为'一鸣惊人'"),
        ("不可思异", "不可思议", "错误用法，应为'不可思议'"),
        ("入木三寸", "入木三分", "错误用法，应为'入木三分'"),
        ("文不加笔", "文不加点", "错误用法，应为'文不加点'"),
        ("契而不舍", "锲而不舍", "错误用法，应为'锲而不舍'"),
        ("首当其中", "首当其冲", "错误用法，应为'首当其冲'"),
        ("无独有对", "无独有偶", "错误用法，应为'无独有偶'"),
        ("鞭长莫逮", "鞭长莫及", "错误用法，应为'鞭长莫及'"),
        ("本末颠倒", "本末倒置", "错误用法，应为'本末倒置'"),
        ("刻船求剑", "刻舟求剑", "错误用法，应为'刻舟求剑'"),
    ];

    for (wrong_idiom, correct_idiom, explanation) in idiom_pairs {
        if line.contains(wrong_idiom) {
            if let Some(pos) = line.find(wrong_idiom) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, pos),
                    end: byte_to_char_index(line, pos + wrong_idiom.len()),
                    issue_type: "成语用法".to_string(),
                    message: format!("成语使用错误: '{}'", wrong_idiom),
                    suggestion: format!("应使用: '{}'，{}", correct_idiom, explanation),
                });
            }
        }
    }
}

// Check for academic writing style issues
pub fn check_academic_style(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    language: &str,
) {
    if language == "en" {
        // Check for informal contractions in English academic writing
        let contractions = [
            (r"\bdon't\b", "do not"),
            (r"\bcan't\b", "cannot"),
            (r"\bwon't\b", "will not"),
            (r"\bisn't\b", "is not"),
            (r"\baren't\b", "are not"),
            (r"\bhaven't\b", "have not"),
            (r"\bi'm\b", "I am"),
            (r"\byou're\b", "you are"),
            (r"\bit's\b", "it is"),
        ];

        for (contraction, full_form) in contractions {
            let regex = match Regex::new(contraction) {
                Ok(re) => re,
                Err(_) => continue, // Skip this pattern if regex creation fails
            };

            for mat in regex.find_iter(line) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, mat.start()),
                    end: byte_to_char_index(line, mat.end()),
                    issue_type: "学术写作风格".to_string(),
                    message: "学术写作中应避免使用缩写形式".to_string(),
                    suggestion: format!("使用完整形式: '{}'", full_form),
                });
            }
        }

        // Check for first person pronouns in formal academic writing
        let first_person_pronouns = [r"\bI\b", r"\bme\b", r"\bmy\b", r"\bmine\b", r"\bmyself\b"];
        for pronoun in first_person_pronouns {
            let regex = match Regex::new(pronoun) {
                Ok(re) => re,
                Err(_) => continue, // Skip this pattern if regex creation fails
            };

            for mat in regex.find_iter(line) {
                issues.push(TextIssue {
                    line_number: line_idx + 1,
                    start: byte_to_char_index(line, mat.start()),
                    end: byte_to_char_index(line, mat.end()),
                    issue_type: "学术写作风格".to_string(),
                    message: "正式学术写作中应避免使用第一人称代词".to_string(),
                    suggestion: "考虑使用被动语态或更客观的表达方式".to_string(),
                });
            }
        }
    } else if language == "zh" {
        // Check for informal expressions in Chinese academic writing
        let informal_expressions = [
            ("很好", "良好"),
            ("很大", "巨大"),
            ("很小", "微小"),
            ("很多", "大量"),
            ("很少", "稀少"),
            ("弄", "进行"),
            ("搞", "开展"),
            ("东西", "物品"),
            ("事情", "事件"),
        ];

        for (informal, formal) in informal_expressions {
            if line.contains(informal) {
                if let Some(pos) = line.find(informal) {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, pos),
                        end: byte_to_char_index(line, pos + informal.len()),
                        issue_type: "学术写作风格".to_string(),
                        message: format!("非正式表达: '{}'", informal),
                        suggestion: format!("考虑使用更正式的表达: '{}'", formal),
                    });
                }
            }
        }

        // Check for first person pronouns in Chinese academic writing
        let first_person_pronouns = ["我", "我们", "咱们", "俺", "俺们"];
        for pronoun in first_person_pronouns {
            if line.contains(pronoun) {
                if let Some(pos) = line.find(pronoun) {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, pos),
                        end: byte_to_char_index(line, pos + pronoun.len()),
                        issue_type: "学术写作风格".to_string(),
                        message: "正式学术写作中应避免使用第一人称代词".to_string(),
                        suggestion: "考虑使用被动语态或更客观的表达方式".to_string(),
                    });
                }
            }
        }
    }
}

// Check for sentence length issues
pub fn check_sentence_length(
    line: &str,
    line_idx: usize,
    issues: &mut Vec<TextIssue>,
    language: &str,
) {
    // Define maximum recommended sentence length (in characters)
    let max_length = if language == "zh" { 100 } else { 200 };

    // Split the line into sentences
    // Use Vec instead of fixed-size arrays to avoid type mismatch
    let sentence_endings: Vec<char> = if language == "zh" {
        vec!['.', '。', '！', '!', '？', '?', ';', '；']
    } else {
        vec!['.', '!', '?', ';']
    };

    let mut start_pos = 0;
    let mut in_sentence = true;

    for (i, c) in line.char_indices() {
        if sentence_endings.contains(&c) {
            if in_sentence {
                let sentence = &line[start_pos..i + 1];
                let sentence_length = sentence.chars().count();

                if sentence_length > max_length {
                    issues.push(TextIssue {
                        line_number: line_idx + 1,
                        start: byte_to_char_index(line, start_pos),
                        end: byte_to_char_index(line, i + 1),
                        issue_type: "句子长度".to_string(),
                        message: format!("句子过长 ({} 字符)", sentence_length),
                        suggestion: "考虑将长句拆分为多个短句，以提高可读性".to_string(),
                    });
                }

                in_sentence = false;
            }
        } else if !c.is_whitespace() && !in_sentence {
            start_pos = i;
            in_sentence = true;
        }
    }

    // Check if the last part of the line is a long sentence without ending punctuation
    if in_sentence && line.len() - start_pos > max_length {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: byte_to_char_index(line, start_pos),
            end: byte_to_char_index(line, line.len()),
            issue_type: "句子长度".to_string(),
            message: format!("可能的长句 ({} 字符)", line.len() - start_pos),
            suggestion: "考虑将长句拆分为多个短句，以提高可读性".to_string(),
        });
    }
}

// Check for citation format consistency
pub fn check_citation_format(line: &str, line_idx: usize, issues: &mut Vec<TextIssue>) {
    // Check for different citation formats in the same line
    let apa_citation = match Regex::new(r"\([A-Za-z]+,\s+\d{4}\)") {
        Ok(re) => re,
        Err(_) => return, // Return early if regex creation fails
    };

    let mla_citation = match Regex::new(r"\([A-Za-z]+\s+\d{1,3}\)") {
        Ok(re) => re,
        Err(_) => return,
    };

    let chicago_citation = match Regex::new(r"\d+\.\s+[A-Za-z]+") {
        Ok(re) => re,
        Err(_) => return,
    };

    let ieee_citation = match Regex::new(r"\[\d+\]") {
        Ok(re) => re,
        Err(_) => return,
    };

    let has_apa = apa_citation.is_match(line);
    let has_mla = mla_citation.is_match(line);
    let has_chicago = chicago_citation.is_match(line);
    let has_ieee = ieee_citation.is_match(line);

    let citation_count = [has_apa, has_mla, has_chicago, has_ieee]
        .iter()
        .filter(|&&x| x)
        .count();

    if citation_count > 1 {
        issues.push(TextIssue {
            line_number: line_idx + 1,
            start: 0,
            end: line.len(),
            issue_type: "引用格式".to_string(),
            message: "同一行中存在不同的引用格式".to_string(),
            suggestion: "请统一使用一种引用格式（如APA、MLA、Chicago或IEEE）".to_string(),
        });
    }

    // Check for potential citation errors - simplified
    let potential_errors = [
        (
            r"\(\s*[A-Za-z]+\s*\d{4}\s*\)",
            "引用格式可能缺少逗号",
            "例如：(Smith, 2020)",
        ),
        (
            r"\(\s*[A-Za-z]+\s*\)",
            "引用格式可能缺少年份",
            "例如：(Smith, 2020)",
        ),
    ];

    for (pattern, message, suggestion) in potential_errors {
        let regex = match Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => continue, // Skip this pattern if regex creation fails
        };

        for mat in regex.find_iter(line) {
            issues.push(TextIssue {
                line_number: line_idx + 1,
                start: byte_to_char_index(line, mat.start()),
                end: byte_to_char_index(line, mat.end()),
                issue_type: "引用格式".to_string(),
                message: message.to_string(),
                suggestion: suggestion.to_string(),
            });
        }
    }
}
